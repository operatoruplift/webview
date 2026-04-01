Build a Tauri 2.0 desktop app wrapping https://www.operatoruplift.com. Check existing repo first.
The website has 38 pages, full dashboard, real LLM chat, swarm orchestration, memory engine. The Tauri shell just wraps it in a native window.
Phase 1 (now): Load production URL in webview
Phase 2 (later): Connect to CORE runtime at localhost:3001
Requirements:
- Tauri 2.0, window 1280x800, min 900x600
- Title: "Operator Uplift", bg color: #050508
- System tray: Open, Quit
- Cmd+Q quit, Cmd+W hide
- Inject window.__TAURI__ = true for desktop detection
- No white flash on startup
- Offline: show "No connection" page
- macOS .dmg, Windows .exe, Linux .AppImage
After every change, run `cargo tauri dev` to verify it works.
