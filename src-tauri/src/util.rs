use std::env;
use std::path::PathBuf;

use anyhow::Ok;
use tauri::{webview::DownloadEvent, Manager, WebviewBuilder, WebviewUrl, WindowBuilder};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};

pub enum MessageType {
    Start,
    Success,
    Failure,
}

pub fn create_window(app_handle: &tauri::AppHandle) -> anyhow::Result<()> {
    #[cfg(target_os = "windows")]
    {
        let handle = app_handle.clone();
        std::thread::spawn(move || {
            let window = tauri::window::WindowBuilder::new(&handle, "main")
                .title("AI聚合平台")
                .inner_size(1400.0, 1000.0)
                .maximizable(true)
                .center()
                .transparent(false)
                .skip_taskbar(true)
                .build()
                .unwrap();
            let webview_builder = tauri::webview::WebviewBuilder::new(
                "main",
                tauri::WebviewUrl::App("index.html".into()),
            )
            // disable devtools
            .devtools(false)
            .auto_resize();
            let _ = window.add_child(
                webview_builder,
                tauri::LogicalPosition::new(0, 0),
                window.inner_size().unwrap(),
            );
        });
    }

    #[cfg(target_os = "macos")]
    {
        let handle_clone = app_handle.clone();
        let window = WindowBuilder::new(app_handle, "main")
            .title("AI聚合平台")
            .inner_size(1400.0, 1000.0)
            .maximizable(true)
            .resizable(true)
            .center()
            .build()?;

        let webview_builder = WebviewBuilder::new("main", WebviewUrl::App("index.html".into()))
            .auto_resize()
            // disable devtools
            .devtools(false)
            .on_download(move |_webview, event| {
                match event {
                    DownloadEvent::Requested { url, destination } => {
                        let file_name = url.path().split('/').last().unwrap_or("未命名");
                        let file_name =
                            &urlencoding::decode(file_name).expect("UTF-8").into_owned();
                        let download_dir = handle_clone.path().download_dir().unwrap();
                        println!(
                            "downloading {:#?} to {:#?}",
                            url.as_str(),
                            download_dir.join(PathBuf::from(file_name))
                        );
                        *destination = download_dir.join(PathBuf::from(file_name));
                        return true;
                    }
                    DownloadEvent::Finished {
                        url,
                        //  **macOS**: The second parameter indicating the path the file was saved to is always empty, due to API  limitations.
                        path: _,
                        success,
                    } => {
                        if success {
                            let handle_clone_for_dialog = handle_clone.clone();
                            let download_dir =
                                handle_clone_for_dialog.path().download_dir().unwrap();
                            std::thread::spawn(move || {
                                let p = url.path().split('/').last().unwrap_or("未命名");
                                let file_name =
                                    &urlencoding::decode(p).expect("UTF-8").into_owned();
                                let file_path = download_dir.join(PathBuf::from(file_name));
                                let _ = handle_clone_for_dialog
                                    .dialog()
                                    .message(format!(
                                        "文件下载完成: {}",
                                        &file_path.to_string_lossy()
                                    ))
                                    .title("下载成功")
                                    .buttons(MessageDialogButtons::Ok)
                                    .blocking_show();
                            });
                        }
                    }
                    _ => (),
                }
                // let the download start
                true
            });

        let _ = window.add_child(
            webview_builder,
            tauri::LogicalPosition::new(0, 0),
            window.inner_size().unwrap(),
        )?;
    }

    #[cfg(target_os = "linux")]
    {
        let handle = app_handle.clone();
        let window = tauri::window::WindowBuilder::new(&handle, "main")
            .title("AI聚合平台")
            .inner_size(1400.0, 1000.0)
            .maximizable(true)
            .center()
            .transparent(false)
            .skip_taskbar(true)
            .build()
            .unwrap();
        let webview_builder = tauri::webview::WebviewBuilder::new(
            "main",
            tauri::WebviewUrl::App("index.html".into()),
        )
        .devtools(false)
        .auto_resize();
        let _ = window.add_child(
            webview_builder,
            tauri::LogicalPosition::new(0, 0),
            window.inner_size().unwrap(),
        );
    }

    Ok(())
}

pub fn get_download_message(message_type: MessageType) -> String {
    let default_start_message = "Start downloading~";
    let chinese_start_message = "开始下载中~";

    let default_success_message = "Download successful, saved to download directory~";
    let chinese_success_message = "下载成功，已保存到下载目录~";

    let default_failure_message = "Download failed, please check your network connection~";
    let chinese_failure_message = "下载失败，请检查你的网络连接~";

    env::var("LANG")
        .map(|lang| {
            if lang.starts_with("zh") {
                match message_type {
                    MessageType::Start => chinese_start_message,
                    MessageType::Success => chinese_success_message,
                    MessageType::Failure => chinese_failure_message,
                }
            } else {
                match message_type {
                    MessageType::Start => default_start_message,
                    MessageType::Success => default_success_message,
                    MessageType::Failure => default_failure_message,
                }
            }
        })
        .unwrap_or_else(|_| match message_type {
            MessageType::Start => default_start_message,
            MessageType::Success => default_success_message,
            MessageType::Failure => default_failure_message,
        })
        .to_string()
}

// Check if the file exists, if it exists, add a number to file name
pub fn check_file_or_append(file_path: &str) -> String {
    let mut new_path = PathBuf::from(file_path);
    let mut counter = 0;

    while new_path.exists() {
        let file_stem = new_path.file_stem().unwrap().to_string_lossy().to_string();
        let extension = new_path.extension().unwrap().to_string_lossy().to_string();
        let parent_dir = new_path.parent().unwrap();

        let new_file_stem = match file_stem.rfind('-') {
            Some(index) if file_stem[index + 1..].parse::<u32>().is_ok() => {
                let base_name = &file_stem[..index];
                counter = file_stem[index + 1..].parse::<u32>().unwrap() + 1;
                format!("{}-{}", base_name, counter)
            }
            _ => {
                counter += 1;
                format!("{}-{}", file_stem, counter)
            }
        };

        new_path = parent_dir.join(format!("{}.{}", new_file_stem, extension));
    }

    new_path.to_string_lossy().into_owned()
}
