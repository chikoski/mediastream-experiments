import { createRoot } from "react-dom/client";
import App from "./ui/app";

async function main() {
    console.log("Initializing UI");
    const root = document.createElement("div");
    document.body.appendChild(root);
    const appRoot = createRoot(root);
    const worker = new Worker(new URL("./worker.ts", import.meta.url), {
        type: "module",
        name: "ve-applier"
    });  
    appRoot.render(<App worker={worker} />);
}

document.addEventListener("DOMContentLoaded", main);