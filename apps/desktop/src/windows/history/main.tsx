import React from "react";
import ReactDOM from "react-dom/client";
import { HistoryWindow } from "../../features/history-panel/history-window";
import "../pet/styles.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <HistoryWindow />
  </React.StrictMode>
);
