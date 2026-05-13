use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

// ---------------------------------------------------------------------------
// Device detection (cached once at first access)
// ---------------------------------------------------------------------------

const AMD_VENDOR_ID: &str = "0x1002";

static CARD_DEVICE_PATH: OnceLock<Option<PathBuf>> = OnceLock::new();
static PCI_POWER_CONTROL_PATH: OnceLock<Option<PathBuf>> = OnceLock::new();
static HWMON_TEMP_PATH: OnceLock<Option<PathBuf>> = OnceLock::new();
static MAX_CLOCK_MHZ: OnceLock<u32> = OnceLock::new();

/// Scan `/sys/class/drm/cardN` entries for one whose `device/vendor` is AMD (0x1002).
/// Returns the `device/` subdir path — the common parent of all sysfs metrics.
fn detect_card_device_path() -> Option<PathBuf> {
    let entries = fs::read_dir("/sys/class/drm").ok()?;
    for entry in entries.flatten() {
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();
        // Match cardN only (skip card1-eDP-1, renderD128, etc.)
        if !name.starts_with("card") || name.len() < 5 {
            continue;
        }
        if !name[4..].chars().all(|c| c.is_ascii_digit()) {
            continue;
        }
        let device_path = entry.path().join("device");
        let vendor_path = device_path.join("vendor");
        if let Ok(content) = fs::read_to_string(&vendor_path) {
            if content.trim() == AMD_VENDOR_ID {
                return Some(device_path);
            }
        }
    }
    None
}

fn card_device_path() -> Option<&'static Path> {
    CARD_DEVICE_PATH
        .get_or_init(detect_card_device_path)
        .as_deref()
}

/// Resolve the card's device symlink to its real PCI path and compute
/// `/sys/bus/pci/devices/<addr>/power/control` (which on modern kernels is
/// just `<real-path>/power/control` since the real path *is* the PCI device).
fn detect_pci_power_control_path() -> Option<PathBuf> {
    let card = card_device_path()?;
    let canonical = fs::canonicalize(card).ok()?;
    Some(canonical.join("power").join("control"))
}

fn pci_power_control_path() -> Option<&'static Path> {
    PCI_POWER_CONTROL_PATH
        .get_or_init(detect_pci_power_control_path)
        .as_deref()
}

/// Find the hwmon directory that belongs to the AMD GPU (name == "amdgpu").
/// Falls back to first hwmon with `temp1_input` if no name matches.
fn detect_hwmon_temp_path() -> Option<PathBuf> {
    // Preferred: iterate hwmon, pick one whose `name` is "amdgpu"
    if let Ok(entries) = fs::read_dir("/sys/class/hwmon") {
        let mut fallback: Option<PathBuf> = None;
        for entry in entries.flatten() {
            let dir = entry.path();
            let temp_path = dir.join("temp1_input");
            if !temp_path.exists() {
                continue;
            }
            if let Ok(name) = fs::read_to_string(dir.join("name")) {
                if name.trim() == "amdgpu" {
                    return Some(temp_path);
                }
            }
            if fallback.is_none() {
                fallback = Some(temp_path);
            }
        }
        return fallback;
    }
    None
}

fn hwmon_temp_path() -> Option<&'static Path> {
    HWMON_TEMP_PATH
        .get_or_init(detect_hwmon_temp_path)
        .as_deref()
}

/// Parse the highest clock value (MHz) from `pp_dpm_sclk`. Each line looks
/// like `"1: 673Mhz *"` or `"2: 2900Mhz"`. Returns the max of all lines.
fn detect_max_clock_mhz() -> u32 {
    let Some(card) = card_device_path() else {
        return 0;
    };
    let path = card.join("pp_dpm_sclk");
    let Ok(content) = fs::read_to_string(&path) else {
        return 0;
    };
    content
        .lines()
        .filter_map(parse_sclk_line_mhz)
        .max()
        .unwrap_or(0)
}

/// Extract MHz value from a single `pp_dpm_sclk` line.
/// Format: `"<idx>: <mhz>Mhz[ *]"`.
fn parse_sclk_line_mhz(line: &str) -> Option<u32> {
    let mhz_part = line.split_whitespace().nth(1)?;
    let digits: String = mhz_part
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    digits.parse::<u32>().ok()
}

fn max_clock_mhz() -> u32 {
    *MAX_CLOCK_MHZ.get_or_init(detect_max_clock_mhz)
}

// ---------------------------------------------------------------------------
// Stats struct
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct GpuStats {
    pub temperature: u32,
    pub gpu_clock: u32,
    pub gpu_busy: u32,
    pub vram_used: u64,
    pub vram_total: u64,
    pub power_mode: String,
    pub charger_status: bool,
    pub runtime_pm: String,
    /// Max sclk DPM state in MHz — used by the UI for progress bar scaling.
    pub max_clock: u32,
}

// ---------------------------------------------------------------------------
// Read commands
// ---------------------------------------------------------------------------

/// Read GPU temperature in °C from the amdgpu hwmon node.
#[tauri::command]
pub fn get_temperature() -> u32 {
    let Some(path) = hwmon_temp_path() else {
        return 0;
    };
    fs::read_to_string(path)
        .ok()
        .and_then(|s| s.trim().parse::<u32>().ok())
        .map(|millideg| millideg / 1000)
        .unwrap_or(0)
}

/// Read current GPU core clock (MHz). Parses the `*`-marked line of `pp_dpm_sclk`.
#[tauri::command]
pub fn get_gpu_clock() -> u32 {
    let Some(card) = card_device_path() else {
        return 0;
    };
    let Ok(content) = fs::read_to_string(card.join("pp_dpm_sclk")) else {
        return 0;
    };
    content
        .lines()
        .find(|l| l.contains('*'))
        .and_then(parse_sclk_line_mhz)
        .unwrap_or(0)
}

/// Read GPU busy percentage (0-100).
#[tauri::command]
pub fn get_gpu_busy() -> u32 {
    let Some(card) = card_device_path() else {
        return 0;
    };
    fs::read_to_string(card.join("gpu_busy_percent"))
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0)
}

/// VRAM used in bytes.
#[tauri::command]
pub fn get_vram_used() -> u64 {
    let Some(card) = card_device_path() else {
        return 0;
    };
    fs::read_to_string(card.join("mem_info_vram_used"))
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0)
}

/// VRAM total in bytes.
#[tauri::command]
pub fn get_vram_total() -> u64 {
    let Some(card) = card_device_path() else {
        return 0;
    };
    fs::read_to_string(card.join("mem_info_vram_total"))
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0)
}

/// Power mode: `"high"`, `"auto"`, `"manual"`, or `"unknown"`.
#[tauri::command]
pub fn get_power_mode() -> String {
    let Some(card) = card_device_path() else {
        return "unknown".into();
    };
    fs::read_to_string(card.join("power_dpm_force_performance_level"))
        .ok()
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".into())
}

/// Charger plugged in?
#[tauri::command]
pub fn get_charger_status() -> bool {
    // Try known mainline name, then laptop ACAD variant
    for candidate in [
        "/sys/class/power_supply/AC/online",
        "/sys/class/power_supply/ACAD/online",
        "/sys/class/power_supply/ADP0/online",
        "/sys/class/power_supply/ADP1/online",
    ] {
        if let Ok(content) = fs::read_to_string(candidate) {
            if let Ok(v) = content.trim().parse::<u8>() {
                return v == 1;
            }
        }
    }
    false
}

/// Runtime PM state: `"on"`, `"auto"`, or `"unknown"`.
#[tauri::command]
pub fn get_runtime_pm() -> String {
    let Some(path) = pci_power_control_path() else {
        return "unknown".into();
    };
    fs::read_to_string(path)
        .ok()
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".into())
}

/// Max GPU core clock (MHz), as reported by `pp_dpm_sclk`.
#[tauri::command]
pub fn get_max_clock() -> u32 {
    max_clock_mhz()
}

/// Aggregated single-shot query for the UI.
#[tauri::command]
pub fn get_all_stats() -> GpuStats {
    GpuStats {
        temperature: get_temperature(),
        gpu_clock: get_gpu_clock(),
        gpu_busy: get_gpu_busy(),
        vram_used: get_vram_used(),
        vram_total: get_vram_total(),
        power_mode: get_power_mode(),
        charger_status: get_charger_status(),
        runtime_pm: get_runtime_pm(),
        max_clock: max_clock_mhz(),
    }
}

// ---------------------------------------------------------------------------
// Write commands
// ---------------------------------------------------------------------------

/// Set the GPU performance level. Valid values: `"high"` or `"auto"`.
#[tauri::command]
pub fn set_power_mode(mode: String) -> Result<(), String> {
    if mode != "high" && mode != "auto" {
        return Err("Invalid mode. Use 'high' or 'auto'".into());
    }
    let Some(card) = card_device_path() else {
        return Err("AMD GPU not detected".into());
    };
    let target = card.join("power_dpm_force_performance_level");
    fs::write(&target, &mode).map_err(|e| {
        format!(
            "Failed to write {} (need root? run `sudo chmod a+w {}` or use pkexec): {}",
            target.display(),
            target.display(),
            e
        )
    })
}

/// Set PCI runtime PM. Valid values: `"on"` or `"auto"`.
#[tauri::command]
pub fn set_runtime_pm(mode: String) -> Result<(), String> {
    if mode != "on" && mode != "auto" {
        return Err("Invalid mode. Use 'on' or 'auto'".into());
    }
    let Some(target) = pci_power_control_path() else {
        return Err("PCI power control path not detected".into());
    };
    fs::write(target, &mode).map_err(|e| {
        format!(
            "Failed to write {}: {} (requires root)",
            target.display(),
            e
        )
    })
}

/// Start AI session: force high perf + keep PCI powered.
#[tauri::command]
pub fn start_ai_session() -> Result<(), String> {
    set_power_mode("high".into())?;
    set_runtime_pm("on".into())?;
    Ok(())
}

/// End AI session: restore auto everything.
#[tauri::command]
pub fn end_ai_session() -> Result<(), String> {
    set_power_mode("auto".into())?;
    set_runtime_pm("auto".into())?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sclk_line_mhz_current() {
        assert_eq!(parse_sclk_line_mhz("1: 673Mhz *"), Some(673));
    }

    #[test]
    fn test_parse_sclk_line_mhz_plain() {
        assert_eq!(parse_sclk_line_mhz("2: 2900Mhz"), Some(2900));
    }

    #[test]
    fn test_parse_sclk_line_mhz_trailing_space() {
        assert_eq!(parse_sclk_line_mhz("0: 600Mhz "), Some(600));
    }

    #[test]
    fn test_parse_sclk_line_mhz_garbage() {
        assert_eq!(parse_sclk_line_mhz(""), None);
        assert_eq!(parse_sclk_line_mhz("nonsense"), None);
    }

    #[test]
    fn test_get_temperature() {
        // Sanity: under 150 °C or 0 if no AMD hwmon.
        assert!(get_temperature() < 150);
    }

    #[test]
    fn test_get_gpu_clock() {
        assert!(get_gpu_clock() < 10_000);
    }

    #[test]
    fn test_get_gpu_busy() {
        assert!(get_gpu_busy() <= 100);
    }

    #[test]
    fn test_get_vram_consistency() {
        let used = get_vram_used();
        let total = get_vram_total();
        assert!(used <= total || total == 0);
    }

    #[test]
    fn test_get_power_mode_valid() {
        let mode = get_power_mode();
        assert!(["high", "auto", "manual", "low", "unknown"].contains(&mode.as_str()));
    }

    #[test]
    fn test_get_runtime_pm_valid() {
        let pm = get_runtime_pm();
        assert!(["on", "auto", "unknown"].contains(&pm.as_str()));
    }

    #[test]
    fn test_get_max_clock_sanity() {
        // Either 0 (no AMD GPU on CI) or between 100 MHz and 10 GHz.
        let m = get_max_clock();
        assert!(m == 0 || (100..=10_000).contains(&m));
    }

    #[test]
    fn test_get_all_stats_structure() {
        let stats = get_all_stats();
        assert!(stats.temperature < 150);
        assert!(stats.gpu_busy <= 100);
    }

    #[test]
    fn test_set_power_mode_rejects_invalid() {
        assert!(set_power_mode("turbo".into()).is_err());
    }

    #[test]
    fn test_set_runtime_pm_rejects_invalid() {
        assert!(set_runtime_pm("forever".into()).is_err());
    }
}
