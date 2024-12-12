use anyhow::Result;
use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use std::sync::Arc;
use tauri::{AppHandle, Manager, WebviewWindow};

#[derive(Debug, Default, Clone)]
pub struct Handle {
    pub app_handle: Arc<RwLock<Option<AppHandle>>>,
    pub is_exiting: Arc<RwLock<bool>>,
}

impl Handle {
    pub fn global() -> &'static Handle {
        static HANDLE: OnceCell<Handle> = OnceCell::new();

        HANDLE.get_or_init(|| Handle {
            app_handle: Arc::new(RwLock::new(None)),
            is_exiting: Arc::new(RwLock::new(false)),
        })
    }

    pub fn init(&self, app_handle: &AppHandle) {
        let mut handle = self.app_handle.write();
        *handle = Some(app_handle.clone());
    }

    pub fn app_handle(&self) -> Option<AppHandle> {
        self.app_handle.read().clone()
    }

    pub fn get_window(&self) -> Option<WebviewWindow> {
        let app_handle = self.app_handle().unwrap();
        let window: Option<WebviewWindow> = app_handle.get_webview_window("main");
        if window.is_none() {
            println!("window not found");
        }
        window
    }

    pub fn refresh() {
        if let Some(_window) = Self::global().get_window() {
            println!("refresh");
        }
    }

    #[allow(unused)]
    pub fn refresh_profiles() {
        if let Some(window) = Self::global().get_window() {
            println!("refresh_profiles");
        }
    }

    pub fn notice_message<S: Into<String>, M: Into<String>>(_status: S, _msg: M) {
        if let Some(_window) = Self::global().get_window() {
            println!("notice_message");
        }
    }

    /// update the system tray state
    pub fn update_systray_part() -> Result<()> {
        Ok(())
    }

    pub fn set_is_exiting(&self) {
        let mut is_exiting = self.is_exiting.write();
        *is_exiting = true;
    }

    pub fn is_exiting(&self) -> bool {
        *self.is_exiting.read()
    }
}
