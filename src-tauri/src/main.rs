// DOCPAD for Windows: a window without any frame around the app at docalvers.de/docpad. The page brings the case,
// the keys and the band above the display that moves the window (docpad.html, `tauri`); this shell only opens the
// window, lets those pages move, maximize, minimize and close it (capabilities/remote.json) and keeps its size and place.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("DOCPAD could not start");
}
