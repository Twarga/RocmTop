#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            rocm_top::get_temperature,
            rocm_top::get_gpu_clock,
            rocm_top::get_gpu_busy,
            rocm_top::get_vram_used,
            rocm_top::get_vram_total,
            rocm_top::get_power_mode,
            rocm_top::get_charger_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
