pub mod cmds;
pub mod handle;
pub mod shortcuts;
pub mod tray;

use shortcuts::register_shortcuts;
use tray::create_tray;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            // #[cfg(target_os = "macos")]
            // app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            create_tray(app)?;
            #[cfg(desktop)]
            match register_shortcuts(app) {
                Ok(_) => {}
                Err(_) => {
                    println!("Failed to register shortcuts");
                }
            }
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![cmds::bring_window_to_top])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
