import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import { fileURLToPath, URL } from "url";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [wasm(), react(), topLevelAwait()],
  resolve: {
    alias: [
      {
        find: "@soundcore-lib",
        replacement: fileURLToPath(
          new URL("./wasm/pkg/soundcore_lib_wasm.js", import.meta.url)
        ),
      },
      {
        find: "@bluetooth",
        replacement: fileURLToPath(new URL("./src/bluetooth", import.meta.url)),
      },
      {
        find: "@hooks",
        replacement: fileURLToPath(new URL("./src/hooks", import.meta.url)),
      },
      {
        find: "@components",
        replacement: fileURLToPath(
          new URL("./src/components", import.meta.url)
        ),
      },
      {
        find: "@assets",
        replacement: fileURLToPath(new URL("./src/assets", import.meta.url)),
      },
    ],
  },
});
