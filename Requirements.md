# 鼠标最简APP功能需求

## AI鼠标APP（windows，MacOS系统鼠标上位机程序）

### 开发TIPS

1. 选择Rust , Tarui开发；确保APP跨平台可运行，确保安装包尺寸不超过10M；
2. APP程序保持最精简的功能，接受鼠标特殊按键的触发，弹出AI聚合平台；
3. APP支持Windows，Mac操作系统。未来支持Linux及安卓操作系统；

### 功能

- [x] 1. 打开exe，弹出首屏功能
- [x] 2. 点击最小化按钮，窗口隐藏到系统栏，显示图标；
- [x] 3. 左键单击系统栏图标，后恢复到首屏
- [x] 4. 系统栏右键菜单（点击菜单项，弹出webview，并访问页面，不可弹出多个webview，新的页面仍然在老的webview显示）
    - 1. AI对话：弹出webview,访问 https://aichat3.raisound.com/web/#/chat
    - 2. AIPPT：弹出webview,访问 https://aichat3.raisound.com/web/#/ppt
    - 3. AI绘画：弹出webview,访问 https://aichat3.raisound.com/web/#/draw
    - 4. AI阅读：弹出webview,访问 https://aichat3.raisound.com/web/#/extractorbak
    - 5. 思维导图：弹出webview,访问 https://aichat3.raisound.com/web/#/minds
    - 6. 智能体: 弹出webview,访问 https://aichat3.raisound.com/web/#/agent
    - 7. 检查更新: 检查更新API，判断是否有最新版本程序，如果有，弹出下载按钮窗口；
    - 8. 退出系统

第一屏

上半部分是一个webview，地址是 [https://aichat3.raisound.com](https://aichat3.raisound.com/web/#/agent)，未来是一个广告位。

![image.png](https://prod-files-secure.s3.us-west-2.amazonaws.com/79c0215a-dec4-4d8c-a569-cd1dd277e8ba/69ad6ee7-d355-420c-af1d-79687ce64d4f/image.png)

**系统栏图标**

![image.png](https://prod-files-secure.s3.us-west-2.amazonaws.com/79c0215a-dec4-4d8c-a569-cd1dd277e8ba/0f692fe5-748a-4327-a006-28610a637d32/image.png)

**右键菜单**

![image.png](https://prod-files-secure.s3.us-west-2.amazonaws.com/79c0215a-dec4-4d8c-a569-cd1dd277e8ba/7c168a08-9419-4425-9c65-7b7b3579aebe/image.png)

弹出webview示意图，需要保留标题栏和状态栏，窗口默认最大化

![image.png](https://prod-files-secure.s3.us-west-2.amazonaws.com/79c0215a-dec4-4d8c-a569-cd1dd277e8ba/5b7cdf87-f765-4f77-988c-bd3c99e0aacc/image.png)

响应键盘组合快捷键，打开webview，访问[https://aichat3.raisound.com/web/](https://aichat3.raisound.com/web/#/chat)
快捷键设定：
windows：win+alt+F12 
Mac：command+option+F12