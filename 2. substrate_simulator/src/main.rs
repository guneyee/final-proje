use std::process::Command;
use std::thread;

fn main() {
    // Start the first node (Alice)
    let alice_handle = start_node("alice", 30333, 9945, 9933, "0000000000000000000000000000000000000000000000000000000000000001");

    // Wait for a moment to allow Alice's node to start
    thread::sleep(std::time::Duration::from_secs(5));

    // Start the second node (Bob)
    let bob_handle = start_node("bob", 30334, 9946, 9934, "");

    // Wait for user input to exit
    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");

    // Terminate both nodes
    terminate_node(alice_handle);
    terminate_node(bob_handle);
}

fn start_node(name: &str, port: u16, ws_port: u16, rpc_port: u16, node_key: &str) -> std::process::Child {
    let base_path = format!("/tmp/{}", name);

    // Purge old chain data
    Command::new("./target/release/node-template")
        .arg("purge-chain")
        .arg("--base-path")
        .arg(&base_path)
        .arg("--chain")
        .arg("local")
        .arg("-y")
        .spawn()
        .expect("Failed to execute command")
        .wait()
        .expect("Failed to wait for command");

    // Start the local blockchain node
    let mut command = Command::new("./target/release/node-template");
    command
        .arg("--base-path")
        .arg(&base_path)
        .arg("--chain")
        .arg("local")
        .arg(format!("--{}", name))
        .arg("--port")
        .arg(port.to_string())
        .arg("--ws-port")
        .arg(ws_port.to_string())
        .arg("--rpc-port")
        .arg(rpc_port.to_string())
        .arg("--telemetry-url")
        .arg("wss://telemetry.polkadot.io/submit/0")
        .arg("--validator");

    if !node_key.is_empty() {
        command
            .arg("--node-key")
            .arg(node_key);
    }

    command
        .spawn()
        .expect("Failed to execute command")
}

fn terminate_node(handle: std::process::Child) {
    // Terminate the node
    handle
        .kill()
        .expect("Failed to terminate node process");
}
