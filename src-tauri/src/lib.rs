mod network_interface;
mod host_scan;
mod port_scan;

#[tauri::command]
fn host_scan(app_handle: tauri::AppHandle, networkAddress: String) -> Vec<host_scan::HostInfo> {
    host_scan::host_scan_run(app_handle, networkAddress.to_string())
}

#[tauri::command]
fn port_scan(app_handle: tauri::AppHandle, ipAddress: String, port: String) -> (Vec<port_scan::Port>, Vec<port_scan::OSMatch>) {
    port_scan::port_scan_run(app_handle, ipAddress.to_string(), port.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // use asを使うとエラーになるので、[module_name]_runとした.
            network_interface::network_interface_run,
            host_scan,
            port_scan,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
