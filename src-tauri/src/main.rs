// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod echo_packet_sniffer;
use echo_packet_sniffer::PacketSniffer;
use std::sync::Mutex;
use tauri::{State, Window};

struct AppState {
    packet_sniffer: Mutex<PacketSniffer>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello {} from Rust!", name)
}

#[tauri::command]
async fn start_counting(window: Window) -> Result<(), String> {
    use tauri::api::process::{Command, CommandEvent};
    use tauri::async_runtime::spawn;

    let (mut rx, _child) = Command::new("pkexec")
        .args(["src/count-x86_64-unknown-linux-gnu"])
        .spawn()
        .expect("failed to create `count` binary command");

    spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(line) => {
                    window
                        .emit("count", line)
                        .expect("failed to emit event");
                },
                CommandEvent::Stderr(line) => {
                    eprintln!("Error: {}", line);
                },
                CommandEvent::Error(code) => {
                    println!("Exited with code: {}", code);
                },
                CommandEvent::Terminated(payload) => {
                    println!("Terminated by signal: {}", payload.signal.unwrap_or(0));
                },
                _ => (),
            }
        }
    });

    Ok(())
}

#[tauri::command]
fn get_network_interfaces() -> Vec<String> {
    pnet::datalink::interfaces()
        .into_iter()
        .map(|interface| interface.name)
        .collect()
}

#[tauri::command]
async fn start_packet_sniffing(
    interface_name: String,
    window: Window,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut sniffer = state
        .packet_sniffer
        .lock()
        .map_err(|_| "Lock failed on start sniffing")?;
    sniffer.start(interface_name, window);
    Ok(())
}

#[tauri::command]
async fn stop_packet_sniffing(state: State<'_, AppState>) -> Result<(), String> {
    let mut sniffer = state
        .packet_sniffer
        .lock()
        .map_err(|_| "Lock failed on stop sniffing")?;
    sniffer.stop();
    Ok(())
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
        .manage(AppState {
            packet_sniffer: Mutex::new(PacketSniffer::new()),
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            start_counting,
            get_network_interfaces,
            start_packet_sniffing,
            stop_packet_sniffing
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
