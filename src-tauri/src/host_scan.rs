use serde::{Deserialize, Serialize};
use tauri_plugin_shell::ShellExt;

#[derive(Debug, Deserialize)]
struct NmapRun {
    host: Vec<Host>,
}

#[derive(Debug, Deserialize)]
struct Host {
    status: Option<Status>,
    address: Vec<Address>,
}

#[derive(Debug, Deserialize)]
struct Status {
    state: String,
    reason: String,
}

#[derive(Debug, Deserialize, Clone)]
struct Address {
    addr: String,
    addrtype: String,
    vendor: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct HostInfo {
    state: String,
    reason: String,
    ip_address: String,
    mac_address: String,
    vendor: String,
}

fn process_host_scan(app: tauri::AppHandle, network_address: String) -> Result<String, String> {
    let shell = app.shell();
    let output = tauri::async_runtime::block_on(async move {
        shell
            .command("nmap")
            .args(["-sn", "-oX", "-", &network_address])
            .output()
            .await
            .unwrap()
    });

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(format!(
            "Command exited with code {}: {}",
            output.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

#[tauri::command]
pub fn host_scan_run(app: tauri::AppHandle, network_address: String) -> Vec<HostInfo> {
    let result = process_host_scan(app, network_address);

    let hosts_info = match result {
        Ok(output) => {
            let parsed: NmapRun = serde_xml_rs::from_str(&output)
                .expect("XMLのパースに失敗しました");

            let mut hosts = Vec::<HostInfo>::new();
                
            for host in parsed.host {
                let (state, reason) = match &host.status {
                    Some(status) => (status.state.clone(), status.reason.clone()),
                    None => ("".to_string(), "".to_string()),
                };

                let mut ip_address = String::new();
                let mut mac_address = String::new();
                let mut vendor = String::new();
                for address in &host.address {
                    if address.addrtype == "mac" {
                        mac_address = address.addr.clone();
                        vendor = address.vendor.clone().unwrap_or("".to_string())
                    } else {
                        ip_address = address.addr.clone();
                    }
                }

                hosts.push(
                    HostInfo {
                        state,
                        reason,
                        ip_address,
                        mac_address,
                        vendor
                    }
                );
            }

            hosts
        },
        Err(_) => Vec::<HostInfo>::new()
    };

    hosts_info
}
