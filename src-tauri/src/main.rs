// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod packet_sniffer;
use packet_sniffer::get_network_interfaces;
use packet_sniffer::log_packets;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello {} from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_network_interfaces,
            log_packets
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
