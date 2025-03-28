use tauri::Manager;
use window_vibrancy::{apply_mica};

mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app|{
            let window = app.get_webview_window("main").unwrap();

            // #[cfg(target_os="windows")]
            // apply_mica(&window, Some(true)).expect("apply_blur only is supported on windows");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![commands::mouselink::return_mouse_pos])
        .invoke_handler(tauri::generate_handler![commands::mouselink::mouse_position])
        .invoke_handler(tauri::generate_handler![commands::mouselink::return_mouse_state])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
