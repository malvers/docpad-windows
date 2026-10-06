# DOCPAD for Windows

A window without any frame around the DOCPAD app at https://docalvers.de/docpad/ - Doc Alvers Mathe-Labor.

- The page is the app: every push to docalvers.de reaches this window too, the shell itself rarely changes.
- `src-tauri/` is the Tauri 2 shell: the window (`tauri.conf.json`), what the pages may do with it
  (`capabilities/remote.json`: move, maximize, minimize, close), its size and place kept (window-state plugin).
- GitHub Actions builds the installer on Windows (`.github/workflows/build.yml`) and puts it on the release:
  https://github.com/malvers/docpad-windows/releases/latest - it installs for the current user, no admin rights.
- Icon: `app-icon.svg` (the deck D, as the Mac app's, without the macOS margin) -> `app-icon.png` -> `tauri icon`.
