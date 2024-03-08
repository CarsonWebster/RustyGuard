use pnet;

#[tauri::command]
pub fn get_network_interfaces() -> Vec<String> {
    pnet::datalink::interfaces()
        .into_iter()
        .map(|interface| interface.name)
        .collect()
}
