/** @type {import('vite').UserConfig} */
import react from "@vitejs/plugin-react";

export default {
    build: {
        target: "esnext"
    },
    plugins: [
        react()
    ],
}
