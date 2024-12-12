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
      listen("FULLSCREEN", () => {
        setFullscreen(true);
      });
    }
  }, []);

  // async function bring_window_to_top() {
  //   await invoke("bring_window_to_top");
  // }

  async function checkUpdate() {
    await invoke("check_update");
  }

  async function minimize() {
    setFullscreen(true);
    await getCurrentWindow().hide();
  }

  async function quit() {
    setFullscreen(true);
    await getCurrentWindow().hide();
  }
  return (
    <main id="container">
      <div id="frame-container">
        <iframe src={AD_URL} key={AD_URL} />
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
