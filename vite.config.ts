import path from 'path';
import { defineConfig } from 'vite';
import { tanstackRouter } from '@tanstack/router-plugin/vite';
import react from '@vitejs/plugin-react';
import tailwindcss from '@tailwindcss/vite';
import wasmPack from 'vite-plugin-wasm-pack';
import pkg from './package.json';

const host = process.env.TAURI_DEV_HOST;

export default defineConfig(async () => ({
  base:
    process.env.TAURI_ENV_PLATFORM || process.env.NODE_ENV === 'development'
      ? '/'
      : '/mv/',
  plugins: [
    tanstackRouter({
      routesDirectory: './src-web/routes',
      generatedRouteTree: './src-web/routeTree.gen.ts',
      target: 'react',
      autoCodeSplitting: true,
    }),
    react(),
    tailwindcss(),
    wasmPack({
      crates: [
        {
          path: './src-wasm',
          outName: 'mv-wasm',
        },
      ],
      runPlugin: process.env.TAURI_ENV_PLATFORM ? false : true,
    }),
  ],
  resolve: {
    alias: {
      '@mv/wasm': path.resolve(__dirname, './node_modules/mv-wasm/mv_wasm.js'),
      '@': path.resolve(__dirname, './src-web'),
    },
  },
  build: {
    outDir: './dist',
  },
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: 'ws',
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      ignored: ['**/src-tauri/**'],
    },
  },
  define: {
    __APP_VERSION__: JSON.stringify(pkg.version),
    __APP_NAME__: JSON.stringify(pkg.name),
  },
}));
