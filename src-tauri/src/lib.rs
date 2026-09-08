use tauri::{Manager, WebviewWindow};

/// Desktop-only enhancement layer injected into the remote app:
/// smoother transitions, richer glass, custom scrollbars and a
/// `desktop` marker class so the web app can adapt.
const DESKTOP_LAYER: &str = r#"
(function () {
  if (window.__edunovaDesktop) return;
  window.__edunovaDesktop = true;
  document.documentElement.classList.add('is-desktop-app', 'dark');
  var css = document.createElement('style');
  css.textContent = [
    'html.is-desktop-app { --desktop: 1; }',
    'html.is-desktop-app * { transition-duration: .22s; transition-timing-function: cubic-bezier(.22,1,.36,1); }',
    'html.is-desktop-app ::-webkit-scrollbar { width: 10px; height: 10px; }',
    'html.is-desktop-app ::-webkit-scrollbar-track { background: transparent; }',
    'html.is-desktop-app ::-webkit-scrollbar-thumb { background: hsl(var(--primary) / .35); border-radius: 999px; border: 2px solid transparent; background-clip: content-box; }',
    'html.is-desktop-app ::-webkit-scrollbar-thumb:hover { background: hsl(var(--primary) / .6); background-clip: content-box; }',
    'html.is-desktop-app body { overscroll-behavior: none; }',
    'html.is-desktop-app img, html.is-desktop-app button { -webkit-user-drag: none; }',
    'html.is-desktop-app body::after { content: ""; position: fixed; inset: 0; pointer-events: none; z-index: 9999; background: radial-gradient(1200px 600px at 50% -10%, hsl(var(--primary) / .07), transparent 70%); }'
  ].join('\n');
  document.head.appendChild(css);
  document.addEventListener('contextmenu', function (e) {
    var t = e.target;
    var editable = t && (t.tagName === 'INPUT' || t.tagName === 'TEXTAREA' || t.isContentEditable);
    if (!editable) e.preventDefault();
  });
})();
"#;

#[tauri::command]
fn app_version(app: tauri::AppHandle) -> String {
    app.package_info().version.to_string()
}

fn reveal(main: &WebviewWindow, splash: Option<WebviewWindow>) {
    let _ = main.show();
    let _ = main.set_focus();
    if let Some(s) = splash {
        let _ = s.close();
    }
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![app_version])
        .on_page_load(|window, _payload| {
            let _ = window.eval(DESKTOP_LAYER);
        })
        .setup(|app| {
            let main = app.get_webview_window("main").expect("main window missing");
            let splash = app.get_webview_window("splash");

            let _ = main.eval(DESKTOP_LAYER);

            // Keep the splash visible until the remote app is usable, so
            // startup never shows a blank window.
            let main_for_thread = main.clone();
            let splash_for_thread = splash.clone();
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(1800));
                reveal(&main_for_thread, splash_for_thread);
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Edunova desktop");
}
