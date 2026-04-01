# Operator Uplift Desktop

A Tauri 2.0 desktop application that wraps [operatoruplift.com](https://www.operatoruplift.com) in a native window.

The website includes 38 pages, a full dashboard, real LLM chat, swarm orchestration, and a memory engine. This Tauri shell provides a native desktop experience around the production site.

## Features

- **Native window** — 1280x800 default, 900x600 minimum, dark theme (#050508)
- **System tray** — Open / Quit menu, click to restore
- **macOS integration** — Cmd+W hides to tray, Cmd+Q quits
- **Desktop detection** — Injects `window.__TAURI__ = true`
- **No white flash** — Window starts hidden, shown after content loads
- **Offline fallback** — Shows "No Connection" page when offline
- **Cross-platform bundles** — macOS .dmg, Windows .exe (NSIS), Linux .AppImage

## Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Tauri CLI](https://v2.tauri.app/start/prerequisites/) v2

```bash
cargo install tauri-cli --version "^2"
```

## Development

```bash
cargo tauri dev
```

## Build

```bash
cargo tauri build
```

Output bundles are in `src-tauri/target/release/bundle/`.

## Roadmap

- **Phase 1** (current): Load production URL in webview
- **Phase 2**: Connect to CORE runtime at `localhost:3001`

## License

[MIT](LICENSE) — Copyright (c) 2026 Operator Uplift
