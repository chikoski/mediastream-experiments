import { createRoot } from "react-dom/client";
import App from "./ui/app";

async function main() {
    console.log("Initializing UI");
    //const lib = await import("./ui/media-screen");
    //const screen = lib.default();
    //document.body.appendChild(screen);
    const root = document.createElement("div");
    document.body.appendChild(root);
    const appRoot = createRoot(root);
    appRoot.render(App());
}

document.addEventListener("DOMContentLoaded", main);