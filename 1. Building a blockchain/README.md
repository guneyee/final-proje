markdown
Copy code
# Substrate Blockchain Project README

## Overview

This is a Substrate-based blockchain project built using the Substrate Node Template and Substrate Front-end Template. This README provides instructions for setting up and running the blockchain node and interacting with it through a web-based front-end.

## Prerequisites

Before you get started, make sure you have the following software installed:

- [Rust](https://www.rust-lang.org/)
- [Node.js](https://nodejs.org/)
- [Yarn](https://yarnpkg.com/)

## Getting Started

### Clone the Substrate Node Template

```bash
git clone https://github.com/substrate-developer-hub/substrate-node-template
cd substrate-node-template
Create a New Branch
bash
Copy code
git switch -c my-new-branch
Build and Run the Node
To build the node with optimized artifacts, run:

bash
Copy code
cargo build --release
Start the node in development mode:

bash
Copy code
./target/release/node-template --dev
The --dev flag is used to run the node in development mode, which resets all active data when the node is stopped.

Install and Start the Front-end Template
Clone the Substrate Front-end Template:

bash
Copy code
git clone https://github.com/substrate-developer-hub/substrate-front-end-template
cd substrate-front-end-template
Install the project dependencies using Yarn:

bash
Copy code
yarn install
Start the front end:

bash
Copy code
yarn start
The front end will be accessible at http://localhost:8000/ in your web browser.

Transfer Funds
Access the front-end dashboard at http://localhost:8000/.
Navigate to the "balances" section to view user balances.
Select the user (e.g., Dave) and transfer at least 1000000000000 funds to that user.
After the transfer, you can check the "balances" table and the "events" table in the dashboard to confirm the transfer was successful.

Stopping the Node
To stop the local Substrate node:

Return to the terminal where the node is running.
Press Ctrl-C to terminate the running process.
The node will stop, and all persistent block data will be purged, allowing you to start with a clean state next time you run the node.

Customization
Feel free to customize and extend this project to suit your specific blockchain application needs. You can modify the runtime logic, add custom modules, or integrate other features as required.

License
This project is licensed under the MIT License.

Acknowledgments
Thanks to the Substrate development community for providing the tools and templates to kickstart blockchain development.