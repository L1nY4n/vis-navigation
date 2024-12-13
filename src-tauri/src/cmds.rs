use tauri::{Manager as _, Window};

#[tauri::command]
pub fn bring_window_to_top(window: Window) {
    window.set_focus().unwrap(); // This will bring the window to the top
}

#[tauri::command]
pub fn check_update(app: tauri::AppHandle) {
    let webview = app.get_webview_window("update").unwrap();
    let _ = webview.show();
    let _ = webview.set_focus();
}
