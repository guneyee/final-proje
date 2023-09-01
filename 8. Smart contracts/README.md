
# Creating and Deploying a Basic Smart Contract with ink!

This tutorial demonstrates how to build a basic smart contract to run on a Substrate-based chain using ink! as a programming language. ink! is a Rust-based framework for writing smart contracts.

## Prerequisites

Before you begin, make sure you have the following prerequisites set up:

1. **Rust Environment:** Update your Rust environment to include necessary components and targets.

   ```bash
   rustup component add rust-src
   rustup target add wasm32-unknown-unknown --toolchain nightly

#   CLI Tool: Install the latest version of the cargo-contract CLI tool.
cargo install --force --locked cargo-contract --version 2.0.0-rc
# Verify the installation and explore available commands:
cargo contract --help
Creating a New Smart Contract Project
Assuming you have a precompiled Substrate node installed, follow these steps to create a new ink! smart contract project:

1. Create a new project folder named "flipper":

cargo contract new flipper
This command generates a project with a default contract in the lib.rs file.

2. Test the smart contract:

cargo test
3. Build the smart contract:
cargo build
# Deploying the Contract
Once you have compiled the contract, deploy it on your Substrate-based chain:

1. Start your Substrate node with the following command:

substrate-contracts-node --log info,runtime::contracts=debug 2>&1
2. Navigate to the "flipper" project folder.

3. Build the contract using:

cargo contract build
4. Instantiate the contract using a constructor:

cargo contract instantiate --constructor new --args "false" --suri //Alice --salt $(date +%s)
# Replace //Alice with your desired account and set the contract's initial state as needed.

# Interacting with the Contract
Now that the smart contract is deployed, you can interact with it:

1. Use the get() function to retrieve the contract's current state:

cargo contract call --contract $INSTANTIATED_CONTRACT_ADDRESS --message get --suri //Alice --dry-run
2. Use the flip() function to change the contract's state:


cargo contract call --contract $INSTANTIATED_CONTRACT_ADDRESS --message flip --suri //Alice

# Replace $INSTANTIATED_CONTRACT_ADDRESS with the actual contract address.

You have successfully created, deployed, and interacted with a basic ink! smart contract on your Substrate-based chain.

# Customization and Further Development
Feel free to customize the smart contract logic in the lib.rs file to suit your specific needs. You can add more functions and features to your contract and explore the full capabilities of ink! for Substrate smart contract development.

# License
This project is licensed under the MIT License.