use serde::Serialize;
use std::fs::File;
use std::io::Write;

// Define a struct representing configuration data.
#[derive(Serialize)]
struct NodeConfig {
    name: String,
    base_path: String,
    chain: String,
    port: u16,
    ws_port: u16,
    rpc_port: u16,
    telemetry_url: String,
    // Add other configuration items here.
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the node configuration using the NodeConfig struct.
    let config = NodeConfig {
        name: "MyNode01".to_string(),
        base_path: "/tmp/node01".to_string(),
        chain: "./customSpecRaw.json".to_string(),
        port: 30333,
        ws_port: 9945,
        rpc_port: 9933,
        telemetry_url: "wss://telemetry.polkadot.io/submit/0".to_string(),
        // Add other configuration items here.
    };

    // Convert it to JSON format.
    let json_config = serde_json::to_string_pretty(&config)?;

    // Write to a JSON file.
    let mut file = File::create("node_config.json")?;
    file.write_all(json_config.as_bytes())?;

    Ok(())
}
