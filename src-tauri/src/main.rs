// DOCPAD for Windows: a transparent window without any frame around the app at docalvers.de/docpad - the window is the
// case, as in the Mac app. The page does the rest (web/winshell.js answers the Mac app's shell protocol: moving,
// scaling by the corners, widening by the sides, as tall as the screen); this shell opens the window with a mark the
// page asks for before its first script, lets those pages move, size and close it (capabilities/remote.json) and keeps
// its place and size (window-state).
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::WebviewWindowBuilder;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .setup(|app| {
            let config = app.config().app.windows[0].clone();   // "create": false - made here, with the mark
            WebviewWindowBuilder::from_config(app.handle(), &config)?
                .initialization_script(concat!("window.__DOCPAD_WIN__ = '", env!("CARGO_PKG_VERSION"), "';"))
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("DOCPAD could not start");
}
