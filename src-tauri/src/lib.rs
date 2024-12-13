pub mod cmds;
pub mod handle;
pub mod shortcuts;
pub mod tray;
pub mod util;

use shortcuts::register_shortcuts;
use tauri::Manager;
use tray::create_tray;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args,_cwd|{
            let _ = app.get_webview_window("main")
            .expect("no main window")
            .set_focus();

        }))
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            create_tray(app)?;
            #[cfg(desktop)]
            match register_shortcuts(app) {
                Ok(_) => {}
                Err(err) => {
                    println!("Failed to register shortcuts {}", err);
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
                    #[cfg(not(target_os = "macos"))]
                    if window_handle.is_minimized().unwrap_or(false) {
                        let _ = window_handle.hide();
                    }
                }

                tauri::WindowEvent::CloseRequested { api, .. } => {
                    api.prevent_close();
                    #[cfg(target_os = "macos")]
                    {
                        window_handle.minimize().unwrap();
                    }

                    let _ = window_handle.hide();
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            cmds::bring_window_to_top,
            cmds::check_update
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
