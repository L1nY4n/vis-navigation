import { useEffect, useState } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

import "./App.css";
import { UpdateButton } from "./component/update-button";

function App() {
  const [fullscreen, setFullscreen] = useState(false);
  const [url, setUrl] = useState("https://aichat3.raisound.com/web/#/agent");

  let avoidExtraCall = false;

  useEffect(() => {
    if (!avoidExtraCall) {
      avoidExtraCall = true;
      console.log("run");
      listen("WEBVIEW_PUSH", ({ payload }) => {
        const [name, url] = payload as [string, string];

        getCurrentWindow().setTitle(name);
        getCurrentWindow().show();
        setUrl(url);
        setFullscreen(true);

        bring_window_to_top();
      });
    }
  }, []);

  async function bring_window_to_top() {
    await invoke("bring_window_to_top");
  }

  async function minimize() {
    await getCurrentWindow().minimize();
  }

  async function quit() {
    await getCurrentWindow().destroy();
  }


  return (
    <main id="container">
      <div id="frame-container">
        <iframe src={url} width="100%" height="100%" key={url} />
      </div>
      <div id="menu-container" className={fullscreen ? "fullscreen" : ""}>
        <div id="button-container">
          <UpdateButton />
          <button onClick={quit}>关闭程序</button>
          <button onClick={minimize}>最小化</button>
        </div>
      </div>
    </main>
  );
}

export default App;
