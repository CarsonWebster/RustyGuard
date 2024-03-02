// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello {} from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let matches = app.get_cli_matches().expect("Failed to get CLI matches");
            // Check if 'echo' argument is present
            if let Some(echo_arg) = matches.args.get("echo") {
                if let serde_json::Value::Bool(true) = echo_arg.value {
                    println!("echo is present");
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
