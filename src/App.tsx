import { BrowserRouter, Route, Routes } from "react-router-dom";
import "./App.css";

import { Ad } from "./pages/ad";

import { Updater } from "./pages/updater";
import { QuitConfirm } from "./pages/quit_confirm";

function App() {
  return (
    <BrowserRouter>
    <Routes>
      <Route path="/" element={<Ad />} />
      <Route path="/updater" element={<Updater />} />
      <Route path="/quit" element={<QuitConfirm />} />
    </Routes>
    </BrowserRouter>
  );
}

export default App;
