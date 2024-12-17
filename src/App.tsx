import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

interface NetworkInterface {
  name: string;
  ip_address: string;
  netmask: string;
  cidr: string;
  network_address: string;
}

interface HostInfo {
    state: string,
    reason: string,
    ip_address: string,
    mac_address: string,
    vendor: string,
}

interface Port {
  protocol: string;
  portid: string;
  state: State;
  service?: Service;
  script?: Script[];
}

interface State {
  state: string;
  reason: string;
}

interface Service {
  name?: string;
  product?: string;
  version?: string;
  extrainfo?: string;
  ostype?: string;
  cpe?: Cpe[];
}

interface Script {
  table?: ScriptTable[];
}

interface ScriptTable {
  key: string;
  table: Table[];
}

interface Table {
  elem: Elem[];
}

interface Elem {
  key: string;
  "$value": string;
}

interface OSMatch {
  name: string;
  osclass: OSClass[];
}

interface OSClass {
  type?: string;
  vendor?: string;
  osfamily?: string;
  cpe?: Cpe[];
}

interface Cpe {
  "$value": string;
}


function App() {
  const [network_interfaces, setNetworkInterfaces] = useState<NetworkInterface[]>([]);
  const [selectedInterface, setSelectedInterface] = useState<string | null>(null);
  const [networkAddress, setNetworkAddress] = useState<string>("");

  const [hosts_info, setHostsInfo] = useState<HostInfo[]>([]);
  const [selectedHostInfo, setSelectedHostInfo] = useState<string | null>(null);
  const [ipAddress, setIpAddress] = useState<string>("");
  const [port, setPort] = useState<string>("");
  const [ports_info, setPortsInfo] = useState<Port[]>([]);
  const [oss_info, setOssInfo] = useState<OSMatch[]>([]);

  useEffect(() => {
    invoke<NetworkInterface[]>("network_interface_run")
      .then((result) => {
        setNetworkInterfaces(result);
      });
  }, [])

  useEffect(() => {
    const selected = network_interfaces.find(
      (network_interface) => network_interface.name === selectedInterface
    );

    if (selected) {
      setNetworkAddress(`${selected.network_address}/${selected.cidr}`);
    } else {
      setNetworkAddress("");
    }
  }, [selectedInterface, network_interfaces]);

  async function host_scan() {
    await invoke<HostInfo[]>("host_scan", {networkAddress})
      .then((result) => {
        setHostsInfo(result)
      })
  }

  async function port_scan() {
    await invoke<[Port[], OSMatch[]]>("port_scan", {ipAddress, port})
      .then(([ports, oss]) => {
        setPortsInfo(ports)
        setOssInfo(oss)
      })
  }

  useEffect(() => {
    const selected = hosts_info.find(
      (host_info) => host_info.ip_address === selectedHostInfo 
    );

    if (selected) {
      setIpAddress(`${selected.ip_address}`);
    } else {
      setIpAddress("");
    }
  }, [selectedHostInfo, hosts_info]);

  return (
    <main className="container">
      <h1>Nmap GUI</h1>

      <div id="main-box">
        <div id="left-box">
          <div id="network-interface">
            <h2>Network Interface</h2>

            <ul>
              {network_interfaces.map((network_interface) => (
                <li 
                  key={network_interface.name}
                  className={network_interface.name === selectedInterface ? "selected" : ""}
                  onClick={() => setSelectedInterface(network_interface.name)}
                >
                  <p>{network_interface.name}</p>
                  <p>{network_interface.network_address}/{network_interface.cidr}</p>
                  <p>{network_interface.ip_address}/{network_interface.cidr}</p>
                  <p>{network_interface.netmask}</p>
                </li>
              ))}
            </ul>
          </div>
        </div>

        <div id="right-box">
          <div id="host-scan">
            <h2>Host Scan</h2>

            <div className="nmap-description">
              <p>nmap -sn -oX - [NETWORK ADDRESS]/[CIDR]</p>
            </div>

            <label htmlFor="network-address">Network Address</label>
            <input
              id="network-address"
              name="network-address"
              value={networkAddress}
              onChange={(e) => setNetworkAddress(e.target.value)}
            />

            <a className="btn" onClick={host_scan}>Run</a>

            <table id="hosts-info">
              <thead>
                <tr>
                  <th>State</th>
                  <th>Reason</th>
                  <th>IP Address</th>
                  <th>MAC Address</th>
                  <th>Vendor</th>
                </tr>
              </thead>
              <tbody>
                {hosts_info.map((host_info) => (
                  <tr 
                    key={host_info.ip_address}
                    className={host_info.ip_address === selectedHostInfo ? "selected" : ""}
                    onClick={() => setSelectedHostInfo(host_info.ip_address)}
                  >
                    <td>{host_info.state}</td>
                    <td>{host_info.reason}</td>
                    <td>{host_info.ip_address}</td>
                    <td>{host_info.mac_address}</td>
                    <td>{host_info.vendor}</td>
                  </tr>
                ))}
              </tbody>              
            </table>
          </div>

          <div id="port-scan">
            <h2>Port Scan</h2>

            <div className="nmap-description">
              <p>nmap -A -Pn --script vuln -oX - [IP ADDRESS] [PORT]</p>
              <p>[PORT]について<br/>未入力: Nmapのデフォルト -p-: 全ポート -p [PORT]: ポート指定 -p [PORT],[PORT],...: ポート指定(複数) -p [PORT]-[PORT]: ポート指定(範囲)</p>
            </div>

            <label htmlFor="ip-address">IP Address</label>
            <input
              id="ip-address"
              name="ip-address"
              value={ipAddress}
              onChange={(e) => setIpAddress(e.target.value)}
            />
            <label htmlFor="port">Port</label>
            <input id="port" name="port" onChange={(e) => setPort(e.target.value)}/>

            <a className="btn" onClick={port_scan}>Run</a>

            <h3>OS Info</h3>
            {oss_info.map((os) => (
              <div>
                <h4>{os.name}</h4>
                <table id="os-info">
                  <thead>
                    <tr>
                      <th>Type</th>
                      <th>Vendor</th>
                      <th>OS Family</th>
                      <th>CPE</th>
                    </tr>
                  </thead>
                  <tbody>
                    {os.osclass.map((osclass) => (
                      <tr>
                        <td>{osclass.type}</td>
                        <td>{osclass.vendor}</td>
                        <td>{osclass.osfamily}</td>
                        <td>
                          {osclass.cpe?.map((cpe) => (
                            <p>{cpe["$value"]}</p>
                          ))}
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            ))}

            <h3>Port Info</h3>
            {ports_info.map((port) => (
              <div>
                <h4>{port.service?.product}</h4>
                <table id="port-info">
                  <thead>
                    <tr>
                      <th>Protocol</th>
                      <th>Port</th>
                      <th>State</th>
                      <th>Reason</th>
                      <th>Name</th>
                      <th>Version</th>
                      <th>Extra Info</th>
                      <th>OS Type</th>
                      <th>CPE</th>
                    </tr>
                  </thead>
                  <tbody>
                    <tr>
                      <td>{port.protocol}</td>
                      <td>{port.portid}</td>
                      <td>{port.state.state}</td>
                      <td>{port.state.reason}</td>
                      <td>{port.service?.name}</td>
                      <td>{port.service?.version}</td>
                      <td>{port.service?.extrainfo}</td>
                      <td>{port.service?.ostype}</td>
                      <td>
                        {port.service?.cpe?.map((cpe) => (
                          <p>{cpe["$value"]}</p>
                        ))}
                      </td>
                    </tr>
                  </tbody>
                </table>
              </div>
            ))}

            <h3>Exploit Info</h3>
            {ports_info.map((port) => (
              port.script?.map((script) => (
                script.table?.map((table) => (
                  <div>
                    <h4>{table.key}</h4>

                    <table id="exploit-info">
                      <thead>
                        <tr>
                          <th>ID</th>
                          <th>CVSS</th>
                          <th>Is Exploit</th>
                          <th>Type</th>
                        </tr>
                      </thead>
                      <tbody>
                        {table.table?.map((t) => (
                          <tr>
                            {t.elem?.map((elem) => (
                              <td>{elem["$value"]}</td>
                            ))}
                          </tr>
                        ))}
                      </tbody>
                    </table>
                  </div>
                ))
              ))
            ))}
          </div>
        </div>
      </div>
    </main>
  );
}

export default App;
