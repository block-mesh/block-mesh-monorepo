import {defineConfig} from "vite";
// import { svelte } from "@sveltejs/vite-plugin-svelte";
import {internalIpV4} from 'internal-ip'

// https://vitejs.dev/config/
export default defineConfig(async () => ({
    // plugins: [svelte()],
    clearScreen: false,
    server: {
        host: "0.0.0.0",
        port: 1420,
        strictPort: true,
        hmr: {
            protocol: 'ws',
            host: await internalIpV4(),
            port: 5183,
        },
    },
    envPrefix: ["VITE_", "TAURI_"],
}));
