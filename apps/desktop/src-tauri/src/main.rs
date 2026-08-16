// Prevents an extra console window from appearing on Windows in release builds.
// Safe to keep on macOS — it's a no-op.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    desktop_lib::run();
}
