use std::process::Command;
use std::thread;
use std::io;

fn main() -> io::Result<()> {
    // Clone the substrate-node-template repository
    let _ = Command::new("git")
        .arg("clone")
        .arg("https://github.com/substrate-developer-hub/substrate-node-template")
        .status()?;

    // Change directory to the cloned project
    let _ = Command::new("cd")
        .arg("substrate-node-template")
        .status()?;

    // Create a new branch
    let _ = Command::new("git")
        .arg("switch")
        .arg("-c")
        .arg("my-new-branch")
        .status()?;

    // Build the substrate-node-template project in release mode
    let _ = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .status()?;

    // Start the Substrate node in development mode
    let _ = Command::new("./target/release/node-template")
        .arg("--dev")
        .spawn()?;

    // Clone the substrate-front-end-template repository
    let _ = Command::new("git")
        .arg("clone")
        .arg("https://github.com/substrate-developer-hub/substrate-front-end-template")
        .status()?;

    // Change directory to the cloned front-end project
    let _ = Command::new("cd")
        .arg("substrate-front-end-template")
        .status()?;

    // Install front-end dependencies using yarn
    let _ = Command::new("yarn")
        .arg("install")
        .status()?;

    // Start the front end
    let _ = Command::new("yarn")
        .arg("start")
        .spawn()?;

    // Display instructions to the user
    println!("Front end is running at http://localhost:8000/");
    println!("Interact with the Substrate node through the front end.");

    // Wait for user input to stop the node
    println!("Press Enter to stop the Substrate node...");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // Terminate the Substrate node
    let _ = Command::new("pkill")
        .arg("node-template")
        .status()?;

    Ok(())
}