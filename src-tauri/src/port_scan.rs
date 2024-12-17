use serde::{Deserialize, Serialize};
use tauri_plugin_shell::ShellExt;

#[derive(Debug, Deserialize)]
struct NmapRun {
    host: Vec<Host>,
}

#[derive(Debug, Deserialize)]
struct Host {
    ports: Option<Ports>,
    os: Option<OS>,
}

#[derive(Debug, Deserialize)]
struct Ports {
    port: Vec<Port>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Port {
    protocol: String,
    portid: String,
    state: State,
    service: Option<Service>,
    #[serde(default)]
    script: Vec<Script>,
}

#[derive(Debug, Deserialize, Serialize)]
struct State {
    state: String,
    reason: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Service {
    name: Option<String>,
    product: Option<String>,
    version: Option<String>,
    extrainfo: Option<String>,
    ostype: Option<String>,
    cpe: Option<Vec<Cpe>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Script {
    #[serde(rename = "table", default)]
    tables: Vec<ScriptTable>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ScriptTable {
    key: String,
    table: Vec<Table>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Table {
    elem: Vec<Elem>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Elem {
    key: String,
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Debug, Deserialize)]
struct OS {
    osmatch: Vec<OSMatch>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OSMatch {
    name: String,
    osclass: Vec<OSClass>,
}

#[derive(Debug, Deserialize, Serialize)]
struct OSClass {
    #[serde(rename = "type")]
    os_type: Option<String>,
    vendor: Option<String>,
    osfamily: Option<String>,
    cpe: Option<Vec<Cpe>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Cpe {
    #[serde(rename = "$value")]
    value: String,
}

fn process_port_scan(app: tauri::AppHandle, ip_address: String, port: String) -> Result<String, String> {
    let shell = app.shell();

    let args = if port.is_empty() {
        vec!["-A", "-Pn", "--script", "vuln", "-oX", "-", &ip_address]
    } else {
        vec!["-A", "-Pn", "--script", "vuln", "-oX", "-", &ip_address, &port]
    };

    let output = tauri::async_runtime::block_on(async move {
        shell
            .command("nmap")
            .args(args)
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
pub fn port_scan_run(app: tauri::AppHandle, ip_address: String, port: String) -> (Vec<Port>, Vec<OSMatch>) {
    let result = process_port_scan(app, ip_address, port);

    let (ports_info, oss_info) = match result {
        Ok(output) => {
            let parsed: NmapRun = serde_xml_rs::from_str(&output)
                .expect("XMLのパースに失敗しました");

            let mut ports = Vec::<Port>::new();
            let mut oss = Vec::<OSMatch>::new();

            for host in parsed.host {
                for port_list in host.ports {
                    for port in port_list.port {
                        ports.push(port);
                    }
                }

                for os in host.os {
                    for osmatch in os.osmatch {
                        oss.push(osmatch);
                    }
                }
            }

            (ports, oss)
        },
        _ => (Vec::<Port>::new(), Vec::<OSMatch>::new())
    };

    (ports_info, oss_info)
}
