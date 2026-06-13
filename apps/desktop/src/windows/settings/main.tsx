import React from "react";
import ReactDOM from "react-dom/client";
import { SettingsWindow } from "../../features/settings/settings-window";
import "../pet/styles.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <SettingsWindow />
  </React.StrictMode>
);
