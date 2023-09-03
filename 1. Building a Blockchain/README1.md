### Substrate Blockchain Setup Guide
This guide will walk you through setting up your own blockchain using Substrate, a powerful framework for building decentralized applications (dApps) and blockchains. By following these steps, you'll be able to create and interact with your custom blockchain and explore its features.

### Prerequisites
Before you start, ensure you have the following prerequisites installed on your system:

### Rust: Substrate is built using Rust, so make sure you have Rust installed. You can check if Rust is installed by running rustc --version. If not, you can install it from rustup.rs.

Node.js and Yarn: To interact with the blockchain through a front-end dashboard, you'll need Node.js and Yarn. Verify their installations by running node --version and yarn --version.

### Setting Up Your Substrate Blockchain
Clone the Substrate Node Template
### Open your terminal and clone the Substrate Node Template repository from GitHub:
git clone https://github.com/substrate-developer-hub/substrate-node-template
### Navigate to the cloned project directory:
cd substrate-node-template
####                                   Create a New Branch
It's a good practice to create a new branch for your project. This way, you can easily compare your work with the original project and isolate changes.

git switch -c my-new-branch

### Build and Start the Substrate Node
Build the Substrate node project with the --release flag for optimized artifacts:
cargo build --release
### Start the node in development mode using the following command:

./target/release/node-template --dev

### The --dev flag ensures that the node runs in development mode, which is useful during development since it clears active data (keys, blockchain database, and networking information) when you stop the node.

You should now see the node running in your terminal.

### Install and Start the Front-End Dashboard
Substrate provides a front-end template for a dashboard that allows you to interact with your blockchain through a user interface (UI). To set it up:

## Clone the Substrate Front-End Template repository:
git clone https://github.com/substrate-developer-hub/substrate-front-end-template
## Navigate to the cloned front-end project directory:
cd substrate-front-end-template
## Install front-end dependencies using Yarn:
yarn install
## Start the front end:
 yarn start
The front end should automatically open in your default web browser at http://localhost:8000/. If not, you can manually open it.

From the dashboard, you can interact with your Substrate blockchain.

### Transferring Funds
On the dashboard, you'll notice a "balances" section that displays the balances of various users, including some with zero balances.

To transfer funds:

Select a user (e.g., "Dave") from the list.
Transfer at least 1000000000000 units to Dave.
After transferring the funds, you can verify if the value was successfully transferred to Dave by checking the balances table and the events table for the transfer event.

## Stopping the Local Substrate Node
After successfully transferring funds and exploring the front-end template, you can stop the local Substrate node to erase any state changes you made. Since you specified the --dev option when starting the node, stopping it will purge all persistent block data, allowing you to start with a clean state the next time you run the node.

To stop the local Substrate node:

## Return to the terminal shell where the node output is displayed.
Press Control-C to terminate the running process.


Verify that your terminal returns to the prompt in the substrate-node-template directory.
Congratulations! You've set up and interacted with your custom Substrate blockchain. You can now explore further and build your own decentralized applications on this blockchain.

