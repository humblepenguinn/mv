# Setup Guide

## Prerequisites

Install the required dependencies:

- **Rust**
- **Node.js**
- **pnpm**

## Desktop App

1. Head over to [Tauri's site](https://v2.tauri.app/start/prerequisites/#system-dependencies) and install the platform-specific dependencies for your system

2. Install the required frontend dependencies:

   ```bash
   pnpm install
   ```

3. Run the desktop app:

   ```bash
   make dev-desktop
   ```

   Or, using pnpm:

   ```bash
   pnpm dev:tauri
   ```

   The Tauri app will compile and run locally

## Web App

1. Install WASM dependencies:
   ```bash
   make install-web-deps
   ```
   
2. Run the web app:
   ```bash
   make dev-web
   ```
   This will compile the WASM package and run the webapp in your browser
