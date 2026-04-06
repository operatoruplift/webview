// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
    Listener, Manager, RunEvent, WindowEvent,
};
use url::Url;

/// Offline fallback HTML shown when operatoruplift.com is unreachable.
const OFFLINE_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Operator Uplift – Offline</title>
<style>
  * { margin: 0; padding: 0; box-sizing: border-box; }
  body {
    background: #050508;
    color: #e0e0e0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100vh;
    text-align: center;
  }
  .container { max-width: 420px; padding: 2rem; }
  h1 { font-size: 1.5rem; margin-bottom: 0.75rem; color: #ff8c00; }
  p { font-size: 1rem; line-height: 1.6; color: #999; margin-bottom: 1.5rem; }
  button {
    background: #ff8c00;
    color: #050508;
    border: none;
    padding: 0.75rem 2rem;
    border-radius: 8px;
    font-size: 1rem;
    font-weight: 600;
    cursor: pointer;
    transition: opacity 0.2s;
  }
  button:hover { opacity: 0.85; }
</style>
</head>
<body>
<div class="container">
  <h1>No Connection</h1>
  <p>Unable to reach operatoruplift.com. Check your internet connection and try again.</p>
  <button onclick="window.location.href='https://www.operatoruplift.com'">Retry</button>
</div>
</body>
</html>"#;

fn main() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_deep_link::init())
        .setup(|app| {
            // --- System tray: Open / Quit ---
            let open_i = MenuItem::with_id(app, "open", "Open", true, None::<&str>)?;
            let separator = PredefinedMenuItem::separator(app)?;
            let quit_i =
                MenuItem::with_id(app, "quit", "Quit Operator Uplift", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&open_i, &separator, &quit_i])?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("Operator Uplift")
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "open" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let tauri::tray::TrayIconEvent::Click { .. } = event {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            // --- Deep link handler: operatoruplift://path → /path on the site ---
            let deep_handle = app.handle().clone();
            app.listen("deep-link://new-url", move |event| {
                let payload = event.payload();
                if let Ok(urls) = serde_json::from_str::<Vec<String>>(payload) {
                    for raw in urls {
                        if let Ok(parsed) = Url::parse(&raw) {
                            let path = parsed.path().trim_start_matches('/');
                            let host = parsed.host_str().unwrap_or("");
                            let route = if !host.is_empty() && path.is_empty() {
                                format!("/{}", host)
                            } else if !path.is_empty() {
                                format!("/{}", path)
                            } else {
                                "/".to_string()
                            };
                            let nav_url = format!("https://www.operatoruplift.com{}", route);
                            if let Some(window) = deep_handle.get_webview_window("main") {
                                let _ = window.eval(&format!(
                                    "window.location.href = '{}';",
                                    nav_url
                                ));
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                }
            });

            // --- Connectivity check & no-white-flash logic ---
            // Window starts hidden (visible: false in config).
            // We check connectivity, inject __TAURI__, then show the window.
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let online = check_connectivity().await;
                if let Some(window) = handle.get_webview_window("main") {
                    if !online {
                        // Show offline page via JS injection
                        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                        let escaped = OFFLINE_HTML.replace('\\', "\\\\").replace('\'', "\\'").replace('\n', "\\n");
                        let _ = window.eval(&format!(
                            "document.open(); document.write('{}'); document.close();",
                            escaped
                        ));
                    }

                    // Inject __TAURI__ flag for desktop detection
                    let _ = window.eval("window.__TAURI__ = true;");

                    // Brief delay to let content render, preventing white flash
                    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            });

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|app_handle, event| {
        match &event {
            // macOS: Cmd+W / close button hides window instead of quitting
            RunEvent::WindowEvent {
                label,
                event: WindowEvent::CloseRequested { api, .. },
                ..
            } if label == "main" => {
                #[cfg(target_os = "macos")]
                {
                    api.prevent_close();
                    if let Some(window) = app_handle.get_webview_window("main") {
                        let _ = window.hide();
                    }
                }
            }
            _ => {}
        }
    });
}

/// Quick connectivity check — try to reach operatoruplift.com
async fn check_connectivity() -> bool {
    match tokio::time::timeout(
        std::time::Duration::from_secs(5),
        tokio::net::TcpStream::connect("www.operatoruplift.com:443"),
    )
    .await
    {
        Ok(Ok(_)) => true,
        _ => false,
    }
}
