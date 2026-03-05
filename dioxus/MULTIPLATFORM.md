# BeSIM GUI - Multi-Platform Build Guide

This Dioxus application supports both **web** (WebAssembly) and **desktop** (native) targets from a single codebase.

## Architecture

- **`src/lib.rs`** – Shared library code (components, pages, routes, models, API)
- **`src/web.rs`** – Web entry point (WebAssembly/WASM)
- **`src/main.rs`** – Desktop entry point (native binary with GTK UI)
- **`src/api.rs`** – Platform-agnostic API client with conditional compilation

### Platform-Specific Code

The `api.rs` module provides a `sleep()` function that:
- **Web:** Uses `gloo_timers` for async delays
- **Desktop:** Uses `tokio::time` for async delays

Dependencies are configured in `Cargo.toml` with `[target.'cfg(...)']` sections to separate platform-only packages.

## Building

### Web Build

```bash
cd dioxus
dx build --release
```

Output: `target/dx/besim_gui_dioxus/release/web/public/`

Serve locally:
```bash
dx serve --open
```

### Desktop Build

```bash
cd dioxus
cargo build --bin besim_gui_dioxus_desktop --release
```

**Requirements:** GTK development libraries (Linux/Unix):
```bash
# Ubuntu/Debian
sudo apt-get install libgtk-4-dev libadwaita-1-dev

# Fedora
sudo dnf install gtk4-devel libadwaita-devel

# macOS
brew install gtk4 libadwaita
```

Run desktop app:
```bash
./target/release/besim_gui_dioxus_desktop
```

## Configuration

- **API URL (Web):** Set via `window.__API_URL` in HTML or `API_URL` env var at build time
- **API URL (Desktop):** Set via `API_URL` environment variable

Example:
```bash
# Web build with API URL
API_URL=https://api.example.com/ dx build --release

# Desktop run with API URL
API_URL=https://api.example.com/ ./target/release/besim_gui_dioxus_desktop
```

## Branching Logic

Use `#[cfg(...)]` attributes to conditionally compile platform-specific code:

```rust
#[cfg(target_arch = "wasm32")]
fn web_only_feature() { ... }

#[cfg(not(target_arch = "wasm32"))]
fn desktop_only_feature() { ... }
```

All platform-specific dependencies (web-sys, wasm-bindgen, dioxus-desktop, tokio) are isolated in `Cargo.toml` target-specific sections.

## Codebase Structure

```
src/
├── lib.rs                 ← Shared root (exports all modules)
├── web.rs                 ← Web-only entry point
├── main.rs                ← Desktop-only entry point
├── api.rs                 ← Cross-platform API + sleep helper
├── app.rs                 ← Main App component
├── routes.rs              ← Routing definitions
├── models.rs              ← Data structures
├── components/
│   ├── mod.rs
│   └── schedule_chart.rs ← Canvas-based chart (works on both)
└── pages/
    ├── mod.rs
    ├── devices.rs
    ├── rooms.rs
    ├── device_details.rs
    ├── room_details.rs
    ├── room_history.rs    ← Uses plotters_canvas (works on both)
    └── about.rs
```

## Notes

- Both web and desktop share identical UI logic and styling
- CSS is served at `/assets/styles.css` and `/static/schedule_chart.css` in web mode
- Charts (plotters + plotters_canvas) work on both platforms
- The `api::sleep()` abstraction handles timing differences
- Desktop builds currently require GTK installed; consider adding feature flags for alternative UI frameworks in future
