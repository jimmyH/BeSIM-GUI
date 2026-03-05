## BeSMART GUI - Dioxus Version

This is a port of the Yew-based BeSMART GUI to the Dioxus framework.

### Prerequisites

- Rust (latest stable)
- Dioxus CLI: `cargo install dioxus-cli`

### Development

Run the development server with hot reloading:

```bash
API_URL=https://<api endpoint>/api/v1.0/ dx serve --hot-reload
```

Or without an API URL (will use window.__API_URL or build-time API_URL):

```bash
dx serve --hot-reload
```

Format using:

```bash
dx fmt
```

### Building for Production

Build for production:

```bash
dx build --release
```
