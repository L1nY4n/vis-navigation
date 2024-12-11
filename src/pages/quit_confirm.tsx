import { getCurrentWindow } from "@tauri-apps/api/window";
import { exit } from "@tauri-apps/plugin-process";
export const QuitConfirm = () => {
  return (
    <div className="quit_confirm">
      <h5>确定要<strong >退出</strong>吗?</h5>
      <div className="btn-group">
      <button onClick={() => getCurrentWindow().hide()}>取消</button>
      <button autoFocus onClick={() => exit(0)}>确定</button>
      </div>
    </div>
  );
};
