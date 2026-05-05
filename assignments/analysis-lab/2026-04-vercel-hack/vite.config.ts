import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig(async () => ({
  plugins: [react()],

  // Prevent Vite from obscuring rust errors
  clearScreen: false,

  // ─── Dev server (Tauri) ───────────────────────────────────────────────────
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? { protocol: "ws", host, port: 1421 }
      : undefined,
    watch: {
      // Don't watch the Rust source — cargo handles that
      ignored: ["**/src-tauri/**"],
    },
  },

  // ─── Build optimization (Phase 10.3) ─────────────────────────────────────
  build: {
    // Target modern browsers shipped with Tauri webview — enables smaller output
    target: ["es2021", "chrome105", "safari15"],
    // Disable sourcemaps in production (security + size)
    sourcemap: false,
    // Warn if any single chunk exceeds 500 KB gzipped
    chunkSizeWarningLimit: 500,

    rollupOptions: {
      output: {
        // Manual chunk splitting: vendor code changes rarely → better cache hits
        manualChunks: {
          "vendor-react": ["react", "react-dom"],
          "vendor-tauri":  ["@tauri-apps/api"],
        },
      },
      // Tree-shake unused exports aggressively
      treeshake: {
        moduleSideEffects: false,
        propertyReadSideEffects: false,
        unknownGlobalSideEffects: false,
      },
    },

    // Minification: esbuild is default and ~20x faster than terser
    minify: "esbuild",
  },

  // ─── esbuild transform options ────────────────────────────────────────────
  esbuild: {
    // Drop console.* and debugger statements from production bundles
    drop: ["console", "debugger"],
    // Inline pure annotations so tree-shaker can eliminate dead branches
    pure: ["console.log", "console.warn", "console.info"],
  },
}));
