#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![rocm_top::get_temperature, rocm_top::get_gpu_clock])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
