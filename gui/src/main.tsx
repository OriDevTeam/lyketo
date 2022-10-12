// Local Imports
import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./assets/style.css";
import Providers from "./Providers";

// External Imports


ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <Providers>
            <App />
        </Providers>
    </React.StrictMode>
)
