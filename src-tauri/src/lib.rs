pub mod cmds;
pub mod handle;
pub mod shortcuts;
pub mod tray;
pub mod util;

use shortcuts::register_shortcuts;
use tray::create_tray;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
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

            // let config = app.config();
            // let data_dir = app
            //     .path()
            //     .app_data_dir()
            //     .unwrap()
            //     .join(config.product_name.clone().unwrap());
            // if !data_dir.exists() {
            //     std::fs::create_dir_all(&data_dir).unwrap_or_else(|e| {
            //         println!("Can't create dir {} {:?}", data_dir.display(), e)
            //     });
            // }

            // let window = tauri::window::WindowBuilder::new(app, "label")
            //     .resizable(true)
            //     .build()?;
            // let webview_builder = tauri::webview::WebviewBuilder::new(
            //     "label",
            //     tauri::WebviewUrl::External(
            //         Url::parse("https://aichat3.raisound.com/web/#/chat").unwrap(),
            //     ),
            // )
            // .data_directory(data_dir);

            // let webview = window.add_child(
            //     webview_builder,
            //     tauri::LogicalPosition::new(0, 0),
            //     window.inner_size().unwrap(),
            // );

            Ok(())
        })
        .on_window_event(|window_handle, event| {
            match event {
                tauri::WindowEvent::Resized(_) => {
                    // when minimized window, hide
                    if window_handle.is_minimized().unwrap_or(false) {
                        let _ = window_handle.hide();
                    }
                }

                tauri::WindowEvent::CloseRequested { api, .. } => {
                    {
                        let window_handle_clone = window_handle.clone();
                        tauri::async_runtime::spawn(async move {
                            if window_handle_clone.is_fullscreen().unwrap_or(false) {
                                window_handle_clone.set_fullscreen(false).unwrap();
                                // Give a small delay to ensure the full-screen exit operation is completed.
                                tokio::time::sleep(std::time::Duration::from_millis(900)).await;
                            }
                            window_handle_clone.minimize().unwrap();
                            window_handle_clone.hide().unwrap();
                        });
                    }
                    api.prevent_close();
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![cmds::bring_window_to_top, cmds::check_update])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
