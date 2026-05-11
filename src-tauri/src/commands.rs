use serde::{Deserialize, Serialize};
use std::fs;

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
}

/// Read temperature from sysfs hwmon
/// Tries all hwmon* directories to find temp1_input
#[tauri::command]
pub fn get_temperature() -> u32 {
    if let Ok(entries) = fs::read_dir("/sys/class/hwmon") {
        for entry in entries.flatten() {
            let temp_path = entry.path().join("temp1_input");
            if temp_path.exists() {
                if let Ok(content) = fs::read_to_string(&temp_path) {
                    if let Ok(temp_millidegrees) = content.trim().parse::<u32>() {
                        return temp_millidegrees / 1000;
                    }
                }
            }
        }
    }
    0
}

/// Read GPU clock from sysfs
/// Parses pp_dpm_sclk to find the line with * marker
#[tauri::command]
pub fn get_gpu_clock() -> u32 {
    let path = "/sys/class/drm/card1/device/pp_dpm_sclk";
    if let Ok(content) = fs::read_to_string(path) {
        for line in content.lines() {
            if line.contains('*') {
                // Parse "0: 400Mhz *" or similar
                if let Some(mhz_str) = line.split_whitespace().nth(1) {
                    let mhz_clean: String = mhz_str.chars().filter(|c| c.is_digit(10)).collect();
                    if let Ok(mhz) = mhz_clean.parse::<u32>() {
                        return mhz;
                    }
                }
            }
        }
    }
    0
}

/// Read GPU busy percent from sysfs
#[tauri::command]
pub fn get_gpu_busy() -> u32 {
    let path = "/sys/class/drm/card1/device/gpu_busy_percent";
    fs::read_to_string(path)
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0)
}

/// Read VRAM used (bytes)
#[tauri::command]
pub fn get_vram_used() -> u64 {
    let path = "/sys/class/drm/card1/device/mem_info_vram_used";
    fs::read_to_string(path)
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0)
}

/// Read VRAM total (bytes)
#[tauri::command]
pub fn get_vram_total() -> u64 {
    let path = "/sys/class/drm/card1/device/mem_info_vram_total";
    fs::read_to_string(path)
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0)
}

/// Read power mode from sysfs
#[tauri::command]
pub fn get_power_mode() -> String {
    let path = "/sys/class/drm/card1/device/power_dpm_force_performance_level";
    fs::read_to_string(path)
        .ok()
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

/// Read charger status from sysfs
#[tauri::command]
pub fn get_charger_status() -> bool {
    let path = "/sys/class/power_supply/AC/online";
    let fallback = "/sys/class/power_supply/ACAD/online";
    
    fs::read_to_string(path)
        .or_else(|_| fs::read_to_string(fallback))
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_temperature() {
        let temp = get_temperature();
        assert!(temp < 150);
    }

    #[test]
    fn test_get_gpu_clock() {
        let clock = get_gpu_clock();
        assert!(clock < 5000);
    }

    #[test]
    fn test_get_gpu_busy() {
        let busy = get_gpu_busy();
        assert!(busy <= 100);
    }

    #[test]
    fn test_get_vram() {
        let used = get_vram_used();
        let total = get_vram_total();
        assert!(used <= total || total == 0);
    }

    #[test]
    fn test_get_power_mode() {
        let mode = get_power_mode();
        assert!(mode == "high" || mode == "auto" || mode == "manual" || mode == "unknown");
    }

    #[test]
    fn test_get_charger_status() {
        let _status = get_charger_status();
        // Just verify it doesn't crash
    }
}
