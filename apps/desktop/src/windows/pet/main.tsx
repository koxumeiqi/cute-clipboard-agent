import React from "react";
import ReactDOM from "react-dom/client";
import { PetWindow } from "../../features/pet/pet-window";
import "./styles.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <PetWindow />
  </React.StrictMode>
);
