import { check, Update } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";

export const UpdateButton = () => {
  const [showDialog, setShowDialog] = useState(false);
  const [update, setUpdate] = useState<Update | null>(null);
  const [contentLength, setContentLength] = useState(0);
  const [downloaded, setDownloaded] = useState(0);
  const [updateState, setupdateState] = useState<
    null | "Started" | "Progress" | "Finished"
  >(null);

  let avoidExtraCall = false;
  useEffect(() => {
    if (!avoidExtraCall) {
      avoidExtraCall = true;
      console.log("run");
      listen("CHECK_UPDATE", () => {
        checkUpdate();
      });
    }
  }, []);

  async function checkUpdate() {
    setShowDialog(true);
    const up = await check();
    console.log(up);
    setUpdate(up);
  }

  function downloadAndInstall() {
    if (update) {
      update.downloadAndInstall((event) => {
        switch (event.event) {
          case "Started":
            setupdateState("Started");
            setContentLength(event.data.contentLength as number);

            break;
          case "Progress":
            setupdateState("Progress");
            setDownloaded((v) => v + event.data.chunkLength);

            break;
          case "Finished":
            setupdateState("Finished");
            break;
        }
      });
    }
  }

  function cancelUpdate() {
    update?.close();
    setShowDialog(false);
    setUpdate(null);
    setupdateState(null);
    setContentLength(0);
    setDownloaded(0);
  }

  return (
    <>
      {showDialog && (
        <div className="updata-dialog">
          <div className="updata-box">
            <div className="updata-title">软件更新</div>
            <div className="updata-body">
              {update && update.available ? (
                <div>
                  <span>当前版本: </span>
                  <span>
                    新版本: {update.currentVersion} ➤ {update.version}
                  </span>
                  <code>--{update.body}</code>
                  {(updateState === "Progress" ||
                    updateState === "Finished") && (
                    <div>
                      {" "}
                      {(downloaded / 1024).toFixed(2)} /{" "}
                      {(contentLength / 1024).toFixed(2)} KB (
                      {Math.floor((downloaded / contentLength) * 100)}%)
                    </div>
                  )}
                </div>
              ) : (
                <p className="">无可用更新</p>
              )}
            </div>
            <div className="updata-footer">
              {updateState == null && (
                <button onClick={() => downloadAndInstall()}>下载并安装</button>
              )}
              {updateState == "Finished" && (
                <button onClick={() => relaunch()}>重启</button>
              )}
              {updateState == null && (
                <button onClick={() => cancelUpdate()}>取消</button>
              )}
            </div>
          </div>
        </div>
      )}
      <button onClick={checkUpdate}>检查更新</button>
    </>
  );
};
