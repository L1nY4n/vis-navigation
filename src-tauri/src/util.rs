
use std::env;
use std::path::PathBuf;

pub enum MessageType {
    Start,
    Success,
    Failure,
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