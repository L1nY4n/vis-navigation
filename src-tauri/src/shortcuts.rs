use anyhow::Result;
use tauri::{AppHandle, Manager as _};
 
 const URL: &str = "https://aichat3.raisound.com/web/#/agent";
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
        app.global_shortcut().register("Shift+Option+0")?;

        #[cfg(windows)]
        app.global_shortcut().register("Ctrl+Alt+0")?;

        Ok(())
    }
}

fn open_webview(handle: &AppHandle) {
    let wv = handle.get_webview_window("main").unwrap();
    let _ = wv.show();
    let home = URL;
    let _ =   wv.eval(&format!("
                        (!window.location.href === '{}') && window.location.replace('{}'); 
                                var target =  document.querySelectorAll('.sidebar-container .module-list .module-item')[{}];
                                 target.click();
                        
                      ",home,home,0));

    if wv.is_minimized().unwrap_or(true) {
        let _ = wv.unminimize();
    }

    if !wv.is_focused().unwrap_or(false) {
        let _ = wv.set_focus();
    }
}
