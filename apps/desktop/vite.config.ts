import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],

  // Tauri dev server settings
  // The port must match `devUrl` in tauri.conf.json
  server: {
    port: 1420,
    strictPort: true,
  },

  // Vite uses posix paths even on Windows
  clearScreen: false,
});
