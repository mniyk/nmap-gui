use if_addrs::get_if_addrs;
use serde::Serialize;
use std::net::Ipv4Addr;

#[derive(Debug, Serialize)]
pub struct NetworkInterface {
    name: String,
    ip_address: String,
    netmask: String,
    cidr: String,
    network_address: String,
}

fn calculate_network_address(ip: &str, cidr: u8) -> Option<String> {
    let ip_addr: Ipv4Addr = ip.parse().ok()?;

    let mask = !((1 << (32 - cidr)) - 1);
    let mask_addr = Ipv4Addr::from(mask);

    let network = u32::from(ip_addr) & u32::from(mask_addr);
    let network_addr = Ipv4Addr::from(network);

    Some(network_addr.to_string())
}

#[tauri::command]
pub fn network_interface_run() -> Vec<NetworkInterface> {
    let ifaces = match get_if_addrs() {
        Ok(ifaces) => ifaces,
        Err(_) => return Vec::<NetworkInterface>::new(),
    };

    let mut network_interfaces = Vec::<NetworkInterface>::new();
    for iface in ifaces {
        let mut ip_address = String::new();
        let mut netmask = String::new();
        let mut cidr = 0;

        if let if_addrs::IfAddr::V4(v4_addr) = iface.addr {
            ip_address = v4_addr.ip.to_string();
            netmask = v4_addr.netmask.to_string();
            cidr = v4_addr.prefixlen;
        } else {
            continue;
        }

        let result = calculate_network_address(&ip_address, cidr);
        let network_address = match result {
            Some(value) => value,
            None => "".to_string(),
        };

        network_interfaces.push(NetworkInterface {
            name: iface.name.clone(),
            ip_address,
            netmask,
            cidr: cidr.to_string(),
            network_address,
        });
    }

    network_interfaces
}
