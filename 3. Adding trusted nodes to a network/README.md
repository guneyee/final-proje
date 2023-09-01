markdown
Copy code
# Simulating a Substrate Network with Multiple Nodes

In the previous section, we learned how to start a single Substrate node. However, in a real-world scenario, blockchain nodes don't operate in isolation; they are typically part of a network. In this section, we will simulate a Substrate network by adding multiple nodes to the network and observe how they interact.

## Prerequisites

Before getting started, ensure you have already completed the setup steps mentioned in the previous section, including installing Rust and the required dependencies.

## Starting the First Node

1. Open a terminal and change the directory to your Substrate project.

2. Purge old chain data for the first node using the following command:

   ```bash
   ./target/release/node-template purge-chain --base-path /tmp/alice --chain local
You will be prompted to confirm the operation; press 'Y' to proceed.

Start the local blockchain node using the "alice" account with the following command:

bash
Copy code
./target/release/node-template
--base-path /tmp/alice
--chain local
--alice
--port 30333
--ws-port 9945
--rpc-port 9933
--node-key 0000000000000000000000000000000000000000000000000000000000000001
--telemetry-url "wss://telemetry.polkadot.io/submit/0"
--validator
This command will start the first node, and you should monitor the terminal for any errors or issues.

Adding Another Node
Open a new terminal window to add a second node to the network.

Purge old chain data for the second node using the following command (notice the "-y" flag to bypass the confirmation prompt):

bash
Copy code
./target/release/node-template purge-chain --base-path /tmp/bob --chain local -y
Start the new node using the "bob" account with the following command:

bash
Copy code
./target/release/node-template
--base-path /tmp/bob
--chain local
--bob
--port 30334
--ws-port 9946
--rpc-port 9934
--telemetry-url "wss://telemetry.polkadot.io/submit/0"
--validator
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
Verifying Block Production
With both nodes running, you can observe the terminals to see if they display similar messages. You should notice the following:

Node identity discovery: The nodes will discover each other as peers.

Number of peers: You should see that each node has 1 peer, representing the other node in the network.

Block production: Both nodes will produce and finalize blocks, and you will see messages indicating this activity.

Stopping Nodes
To shut down one of the nodes, simply press Ctrl+C in the terminal where the node is running. After doing so, you will notice that the remaining node will have 0 peers, indicating that one node has been disconnected from the network.

Customization
You can customize this setup further by adjusting configurations and adding more nodes to simulate a larger network. Experiment with different configurations and network topologies to better understand how Substrate nodes interact in a networked environment.

License
This project is licensed under the MIT License.

Acknowledgments
Thanks to the Substrate development community for providing the tools and templates for building and simulating blockchain networks