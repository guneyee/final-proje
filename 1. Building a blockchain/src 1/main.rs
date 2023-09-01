    // Update Rust environment
    println!("Updating Rust environment...");
    std::process::Command::new("rustup")
        .arg("component")
        .arg("add")
        .arg("rust-src")
        .status()
        .expect("Failed to update Rust environment");

    // Check for WebAssembly target
    println!("Checking WebAssembly target...");
    std::process::Command::new("rustup")
        .arg("target")
        .arg("add")
        .arg("wasm32-unknown-unknown")
        .arg("--toolchain")
        .arg("nightly")
        .status()
        .expect("Failed to add WebAssembly target");

    // Install CLI tool
    println!("Installing cargo-contract CLI tool...");
    std::process::Command::new("cargo")
        .arg("install")
        .arg("--force")
        .arg("--locked")
        .arg("cargo-contract")
        .arg("--version")
        .arg("2.0.0-rc")
        .status()
        .expect("Failed to install cargo-contract CLI tool");

    // Verify installation
    println!("Verifying cargo-contract installation...");
    std::process::Command::new("cargo")
        .arg("contract")
        .arg("--help")
        .status()
        .expect("Failed to verify cargo-contract installation");

    // Create a new smart contract project
    println!("Creating a new smart contract project...");
    std::process::Command::new("cargo")
        .arg("contract")
        .arg("new")
        .arg("flipper")
        .status()
        .expect("Failed to create a new smart contract project");

    // Test the smart contract
    println!("Testing the smart contract...");
    std::process::Command::new("cargo")
        .arg("test")
        .status()
        .expect("Failed to test the smart contract");

    // Build the smart contract
    println!("Building the smart contract...");
    std::process::Command::new("cargo")
        .arg("build")
        .status()
        .expect("Failed to build the smart contract");

    // Deploy the contract
    println!("Deploying the contract...");
    std::process::Command::new("substrate-contracts-node")
        .arg("--log")
        .arg("info,runtime::contracts=debug")
        .arg("2>&1")
        .status()
        .expect("Failed to start the Substrate node");

    // Navigate to the project folder
    println!("Navigating to the project folder...");
    std::env::set_current_dir("flipper").expect("Failed to navigate to the project folder");

    // Build the contract
    println!("Building the contract...");
    std::process::Command::new("cargo")
        .arg("contract")
        .arg("build")
        .status()
        .expect("Failed to build the contract");

    // Instantiate the contract
    println!("Instantiating the contract...");
    let salt = chrono::Utc::now().timestamp();
    std::process::Command::new("cargo")
        .arg("contract")
        .arg("instantiate")
        .arg("--constructor")
        .arg("new")
        .arg("--args")
        .arg("false")
        .arg("--suri")
        .arg("//Alice")
        .arg("--salt")
        .arg(format!("{}", salt))
        .status()
        .expect("Failed to instantiate the contract");

    // Interact with the contract
    println!("Interacting with the contract...");
    std::process::Command::new("cargo")
        .arg("contract")
        .arg("call")
        .arg("--contract")
        .arg("$INSTANTIATED_CONTRACT_ADDRESS")
        .arg("--message")
        .arg("get")
        .arg("--suri")
        .arg("//Alice")
        .arg("--dry-run")
        .status()
        .expect("Failed to interact with the contract");

    std::process::Command::new("cargo")
        .arg("contract")
        .arg("call")
        .arg("--contract")
        .arg("$INSTANTIATED_CONTRACT_ADDRESS")
        .arg("--message")
        .arg("flip")
        .arg("--suri")
        .arg("//Alice")
        .status()
        .expect("Failed to interact with the contract");
}