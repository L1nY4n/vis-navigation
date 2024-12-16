
// 托盘菜单
// 1. AI对话：弹出webview,访问 https://aichat3.raisound.com/web/#/chat
// 2. AIPPT：弹出webview,访问 https://aichat3.raisound.com/web/#/ppt
// 3. AI绘画：弹出webview,访问 https://aichat3.raisound.com/web/#/draw
// 4. AI阅读：弹出webview,访问 https://aichat3.raisound.com/web/#/extractorbak
// 5. 思维导图：弹出webview,访问 https://aichat3.raisound.com/web/#/minds
// 6. 智能体: 弹出webview,访问 https://aichat3.raisound.com/web/#/agent
// 7. 检查更新: 检查更新API，判断是否有最新版本程序，如果有，弹出下载按钮窗口；
// 8. 退出系统
pub const TRAY_MENU: [(&str, &str, &str, u8); 6] = [
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