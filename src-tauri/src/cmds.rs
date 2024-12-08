use tauri::Window;

#[tauri::command]
pub fn bring_window_to_top(window: Window) {
    window.set_focus().unwrap(); // This will bring the window to the top
}
