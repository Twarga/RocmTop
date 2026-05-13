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
            rocm_top::get_charger_status,
            rocm_top::get_runtime_pm,
            rocm_top::get_max_clock,
            rocm_top::get_all_stats,
            rocm_top::set_power_mode,
            rocm_top::set_runtime_pm,
            rocm_top::start_ai_session,
            rocm_top::end_ai_session
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
