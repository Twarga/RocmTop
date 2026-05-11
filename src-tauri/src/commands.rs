use serde::{Deserialize, Serialize};

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

// Placeholder functions - will be implemented in TA-5 to TA-15
