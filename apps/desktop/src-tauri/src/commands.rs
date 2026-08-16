// IPC bridge between the React frontend and the Rust backend.
//
// How Tauri commands work:
//   1. Define a function here with #[tauri::command]
//   2. Register it in main.rs inside generate_handler![]
//   3. Call it from the frontend: invoke("function_name", { arg: value })
//
// Commands can be async and can return Results for error handling.

/// Placeholder command — demonstrates the round-trip between frontend and backend.
/// Replace with real commands as features are built.
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! The Rust backend is working.", name)
}
