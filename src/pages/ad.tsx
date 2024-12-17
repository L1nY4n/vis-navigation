import { useEffect, useState } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export const Ad = () => {
  const [fullscreen, setFullscreen] = useState(false);
  // 实际是个广告图片
  const AD_URL = "https://aichat3.raisound.com/web/#/agent";
  let avoidExtraCall = false;

  useEffect(() => {
    if (!avoidExtraCall) {
      avoidExtraCall = true;
      getCurrentWindow().center;
      getCurrentWindow().setFocus();
      listen("FULLSCREEN", () => {
        setFullscreen(true);
      });
    }
  }, []);


  async function checkUpdate() {
    await invoke("check_update");
  }

  async function minimize() {
    await quit();
  }

  async function quit() {
    setFullscreen(true);
    const w = getCurrentWindow()
    await w.minimize();
    await w.hide();
  }
  return (
    <main id="container">
      <div id="frame-container">
        <iframe src={AD_URL} />
      </div>
      <div id="menu-container" className={fullscreen ? "fullscreen" : ""}>
        <div id="button-container">
          <button onClick={checkUpdate}>更新</button>
          <button onClick={quit}>关闭程序</button>
          <button onClick={minimize}>最小化</button>
        </div>
      </div>
    </main>
  );
};
