#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, WindowEvent,
};

fn main() {
    // WebKitGTK 2.48+ with the DMA-BUF renderer blanks / flickers the window
    // on many AMD and NVIDIA setups (Arch, CachyOS, Fedora 40+, Nobara).
    // Forcing the legacy compositor path is the upstream-recommended
    // workaround until Mesa + WebKit ship a fix together.
    // See: https://bugs.webkit.org/show_bug.cgi?id=278453
    //
    // These must be set before the webview is created, so do it first thing.
    #[cfg(target_os = "linux")]
    {
        if std::env::var_os("WEBKIT_DISABLE_DMABUF_RENDERER").is_none() {
            std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
        }
        if std::env::var_os("WEBKIT_DISABLE_COMPOSITING_MODE").is_none() {
            std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
        }
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // --- Tray menu --------------------------------------------------
            let show_hide =
                MenuItem::with_id(app, "toggle", "Show / Hide RocmTop", true, None::<&str>)?;
            let separator = PredefinedMenuItem::separator(app)?;
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_hide, &separator, &quit])?;

            // --- Tray icon --------------------------------------------------
            let icon = app
                .default_window_icon()
                .cloned()
                .ok_or("missing default window icon")?;

            TrayIconBuilder::with_id("main")
                .tooltip("RocmTop — AMD GPU Monitor")
                .icon(icon)
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "toggle" => toggle_main_window(app),
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        toggle_main_window(tray.app_handle());
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            // Close button → hide to tray instead of quitting.
            if let WindowEvent::CloseRequested { api, .. } = event {
                if window.label() == "main" {
                    let _ = window.hide();
                    api.prevent_close();
                }
            }
        })
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

/// Show the main window if hidden, hide it if visible.
fn toggle_main_window<R: tauri::Runtime>(app: &tauri::AppHandle<R>) {
    if let Some(window) = app.get_webview_window("main") {
        match window.is_visible() {
            Ok(true) => {
                let _ = window.hide();
            }
            _ => {
                let _ = window.show();
                let _ = window.unminimize();
                let _ = window.set_focus();
            }
        }
    }
}
