use anyhow::Result;
use tauri::{AppHandle, Emitter, Manager as _};

pub fn register_shortcuts(app: &mut tauri::App) -> Result<()> {
    {
        use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

        let handle = app.app_handle().clone();
        app.handle().plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |_app, _shortcut, event| {
                    if event.state == ShortcutState::Released {
                        open_webview(&handle);
                    }
                })
                .build(),
        )?;

        #[cfg(target_os = "macos")]

        app.global_shortcut().register("Command+Option+F12")?;

        #[cfg(windows)]
        app.global_shortcut().register("Ctrl+Alt+F12")?;
        

        Ok(())
    }
}

fn open_webview(handle: &AppHandle) {
    let window = handle.get_webview_window("main").unwrap();

    // Check if the window is already open or visible
    if !window.is_visible().unwrap() {
        window.show().unwrap();
    }
    handle
        .emit(
            "WEBVIEW_PUSH",
            ["AI聚合平台", "https://aichat3.raisound.com/web/"],
        )
        .unwrap();
}
