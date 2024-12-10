use anyhow::Result;
use tauri::menu::Menu;
use tauri::menu::{MenuItem, PredefinedMenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::tray::{MouseButton, MouseButtonState, TrayIconEvent};
use tauri::{window, Emitter, Manager};

// 1. AI对话：弹出webview,访问 https://aichat3.raisound.com/web/#/chat
// 2. AIPPT：弹出webview,访问 https://aichat3.raisound.com/web/#/ppt
// 3. AI绘画：弹出webview,访问 https://aichat3.raisound.com/web/#/draw
// 4. AI阅读：弹出webview,访问 https://aichat3.raisound.com/web/#/extractorbak
// 5. 思维导图：弹出webview,访问 https://aichat3.raisound.com/web/#/minds
// 6. 智能体: 弹出webview,访问 https://aichat3.raisound.com/web/#/agent
// 7. 检查更新: 检查更新API，判断是否有最新版本程序，如果有，弹出下载按钮窗口；
// 8. 退出系统
const TRAY_MENU: [(&str, &str, &str, u8); 6] = [
    (
        "chat",
        "AI对话",
        "https://aichat3.raisound.com/web/#/chat",
        1,
    ),
    ("ppt", "AIPPT", "https://aichat3.raisound.com/web/#/ppt", 3),
    (
        "draw",
        "AI绘画",
        "https://aichat3.raisound.com/web/#/draw",
        4,
    ),
    (
        "extractorbak",
        "AI阅读",
        "https://aichat3.raisound.com/web/#/extractorbak",
        5,
    ),
    (
        "minds",
        "思维导图",
        "https://aichat3.raisound.com/web/#/minds",
        6,
    ),
    (
        "agent",
        "智能体",
        "https://aichat3.raisound.com/web/#/agent",
        7,
    ),
];

pub fn create_tray(app: &mut tauri::App) -> Result<()> {
    let menu = Menu::new(app.app_handle())?;

    for (id, name, _, _) in TRAY_MENU.iter() {
        let item = MenuItem::with_id(app, id, name, true, None::<&str>).unwrap();
        menu.append(&item)?;
    }

    let update = &MenuItem::with_id(app, "update", "检查更新", true, None::<&str>).unwrap();
    let quit = &MenuItem::with_id(app, "quit", "退出", true, Some("CmdOrControl+Q")).unwrap();
    let separator = &PredefinedMenuItem::separator(app).unwrap();

    menu.append(separator)?;
    menu.append(update)?;
    menu.append(quit)?;

    let tray_menu = TrayIconBuilder::with_id("tray")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .icon(app.default_window_icon().unwrap().clone())
        .build(app)?;

    let app_clone = app.app_handle().clone();
    tray_menu.on_tray_icon_event(move |_tray, event| match event {
        TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
            ..
        } => {
            println!("{:?}", event);
            let webview = app_clone.get_webview_window("main").unwrap();

            // println!(
            //     "is_visible: {}  is_minimized: {} , is_maximized: {}",
            //     webview.is_visible().unwrap(),
            //     webview.is_minimized().unwrap(),
            //     webview.is_maximized().unwrap(),

            // );

            if webview.is_visible().unwrap_or(false) {
                let _ = webview.hide();
            } else {
                if webview.is_minimized().unwrap_or(true) {
                    let _ = webview.unminimize();
                }
                let _ = webview.set_focus();
                let _ = webview.show();
            }
        }
        _ => {}
    });

    tray_menu.on_menu_event(move |h, event| match event.id.as_ref() {
        "quit" => {
            h.exit(0);
        }
        "update" => {
            let _ = h.get_webview_window("main").unwrap().show();
            h.app_handle().emit("CHECK_UPDATE", ()).unwrap();
        }
        m => {
            let _ = h.get_webview_window("main").unwrap().show();
            TRAY_MENU
                .iter()
                .find(|(id, _, _,_)| *id == m)
                .map(|(_, name, url,index)| {
                    let _ = h
                        .get_webview_window("main")
                        .unwrap()
                        .eval(&format!("
                        window.location.replace('{}'); 
                        var target =  document.querySelectorAll('.sidebar-container .module-list .module-item')[{}];
                        console.log('target--',target);
                        target.click();", url,index));
                    //   h.app_handle().emit("WEBVIEW_PUSH", [name, url]).unwrap();
                });
        }
    });

    Ok(())
}
