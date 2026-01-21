mod vpn;
mod privileges;
mod service_manager;
mod network_monitor;

use std::sync::{Arc, Mutex};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .manage(Arc::new(Mutex::new(network_monitor::MonitorState::new())))
        .invoke_handler(tauri::generate_handler![
            greet,
            vpn::connect_korea,
            vpn::disconnect_vpn,
            network_monitor::start_monitoring,
            network_monitor::get_detected_servers,
            network_monitor::stop_monitoring,
            network_monitor::add_detected_ip_to_routes,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
