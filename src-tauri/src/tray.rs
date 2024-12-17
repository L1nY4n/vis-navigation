use anyhow::Result;
use tauri::menu::Menu;
use tauri::menu::{MenuItem, PredefinedMenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::tray::{MouseButton, MouseButtonState, TrayIconEvent};
use tauri::{Emitter, Manager};

use crate::const_data::TRAY_MENU;

pub fn create_tray(app: &mut tauri::App) -> Result<()> {
    let menu = Menu::new(app.app_handle())?;

    for (id, name, _, _) in TRAY_MENU.iter() {
        let item = MenuItem::with_id(app, id, name, true, None::<&str>).unwrap();
        menu.append(&item)?;
    }

    let clean_data = &MenuItem::with_id(app, "clean_data", "清除缓存", true, None::<&str>).unwrap();
    let update = &MenuItem::with_id(app, "update", "检查更新", true, None::<&str>).unwrap();
    let quit = &MenuItem::with_id(app, "quit", "退出", true, Some("CmdOrControl+Q")).unwrap();
    let separator = &PredefinedMenuItem::separator(app).unwrap();

    menu.append(separator)?;
    menu.append(clean_data)?;
    menu.append(update)?;
    menu.append(quit)?;

    let tray_menu = TrayIconBuilder::with_id("tray")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .icon(app.default_window_icon().unwrap().clone())
        .build(app)?;

    let app_clone = app.app_handle().clone();
    tray_menu.on_tray_icon_event(move |_tray, event| {
        if let TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
            ..
        } = event
        {
            let webview = app_clone.get_webview_window("main").unwrap();
            let _ = webview.emit("FULLSCREEN", [0]);
            if webview.is_visible().unwrap() {
                #[cfg(target_os = "macos")]
                {
                    let webview_clone = webview.clone();
                    tauri::async_runtime::spawn(async move {
                        if !webview_clone.is_minimized().unwrap() {
                            webview_clone.minimize().unwrap();
                            tokio::time::sleep(std::time::Duration::from_millis(400)).await;
                        }
                        let _ = webview.hide();
                    });
                }
                #[cfg(not(target_os = "macos"))]
                {
                    let _ = webview.minimize();
                    let _ = webview.hide();
                }
            } else {
                #[cfg(not(target_os = "macos"))]
                {
                    let _ = webview.show();
                }

                if webview.is_minimized().unwrap() {
                    let _ = webview.unminimize();
                } else {
                    let _ = webview.show();
                }

                let _ = webview.set_focus();
            }
        }
    });

    tray_menu.on_menu_event(move |h, event| match event.id.as_ref() {
        "quit" => {
            let _ = h.get_webview_window("quit").unwrap().show();
         
        }
        "update" => {
            let webview = h.get_webview_window("update").unwrap();
            let _ = webview .show();
            let _ = webview .set_focus();
        },
        "clean_data" => {
            let webview = h.get_webview_window("main").unwrap();
            let _ = webview .clear_all_browsing_data();
        },
        m => {
            #[cfg(not(target_os = "macos"))]
            {
                let _ = h.get_webview_window("main").unwrap().show();
            }
       
            if let Some((_, _name, url,index)) = TRAY_MENU
                .iter()
                .find(|(id, _, _,_)| *id == m) { 
                    
                      let wv = h .get_webview_window("main")
                        .unwrap();
                       let _ =   wv.eval(&format!("
                        window.location.replace('{}'); 
                        var target =  document.querySelectorAll('.sidebar-container .module-list .module-item')[{}];
                       target && target.click();
                       ", url,index)); 

                        if wv.is_minimized().unwrap_or(true) {
                            let _ = wv.unminimize();
                        }

                        if !wv.is_focused().unwrap_or(false) {
                            let _ = wv.set_focus();
                        }
                    }
        }
    });

    Ok(())
}
