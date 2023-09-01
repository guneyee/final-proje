# Controlling Authorized Nodes in a Substrate Network

In the previous chapters, we learned how to start single and multiple Substrate nodes in a network. In this chapter, we will explore how to control which nodes are authorized to enter the network, essentially creating a permissioned blockchain network.

## Prerequisites

Before proceeding, make sure you have completed the setup steps mentioned in the previous sections, including installing Rust and the required dependencies.

## Generating Account and Keys

1. Open a terminal and change the directory to your Substrate project.

2. Generate a random secret phrase and keys by running the following command:

   ```bash
   ./target/release/node-template key generate --scheme Sr25519 --password-interactive
You will be prompted to enter a password. Save the generated seed phrase for later reference.

Use the seed phrase to derive keys using the Ed25519 signature scheme. For example:

bash
Copy code
./target/release/node-template key inspect --password-interactive --scheme Ed25519 "pig giraffe ceiling enter weird liar orange decline behind total despair fly"
Enter the same password used earlier to create the key, and you will obtain the key for finalizing blocks using Grandpa.

Creating a Custom Chain Specification
After generating keys for your blockchain, you can create a custom chain specification using these key pairs. This custom specification defines which nodes are allowed in the network.

Change to the directory where you compiled the Substrate node template.

Export the predefined local chain specification to a file named customSpec.json with this command:

bash
Copy code
./target/release/node-template build-spec --disable-default-bootnode --chain local > customSpec.json
Open customSpec.json in a text editor and make the following modifications:

Change the name field to a custom name, for example:

json
Copy code
"name": "My Custom Testnet",
Modify the aura and grandpa fields to include the addresses of validator nodes. These nodes will have the authority to create and finalize blocks. For example:

json
Copy code
"aura": {
  "authorities": [
    "5CfBuoHDvZ4fd8jkLQicNL8tgjnK8pVG9AiuJrsNrRAx6CNW", 
    "5CXGP4oPXC1Je3zf5wEDkYeAqGcGXyKWSRX2Jm14GdME5Xc5"
  ]
},
"grandpa": {
  "authorities": [
    [
      "5CuqCGfwqhjGzSqz5mnq36tMe651mU9Ji8xQ4JRuUTvPcjVN",
      1
    ],
    [
      "5DpdMN4bVTMy67TfMMtinQTcUmLhZBWoWarHvEYPM4jYziqm",
      1
    ]
  ]
}
These modifications specify which nodes have the authority to create and finalize blocks in the network.

Convert the custom chain specification to raw format:

bash
Copy code
./target/release/node-template build-spec --chain=customSpec.json --raw --disable-default-bootnode > customSpecRaw.json
Preparing for Launch
After creating the custom chain specification, distribute it to all network participants. Now you're ready to launch your private blockchain.

Start the first node using the custom specification:

bash
Copy code
./target/release/node-template
  --base-path /tmp/node01
  --chain ./customSpecRaw.json
  --port 30333
  --ws-port 9945
  --rpc-port 9933
  --telemetry-url "wss://telemetry.polkadot.io/submit/0"
  --validator
  --rpc-methods Unsafe
  --name MyNode01
  --password-interactive
You will be asked for a password; use the same password used to generate the keys.

Add the aura and grandpa authority keys to the keystore for the first node:

bash
Copy code
./target/release/node-template key insert --base-path /tmp/node01
  --chain customSpecRaw.json
  --scheme Sr25519
  --suri <your-secret-seed>
  --password-interactive
  --key-type aura
Replace <your-secret-seed> with the secret phrase or seed of the first key pair you generated. Enter the password when prompted.

bash
Copy code
./target/release/node-template key insert --base-path /tmp/node01
  --chain customSpecRaw.json
  --scheme Ed25519
  --suri <your-secret-key>
  --password-interactive
  --key-type gran
Replace <your-secret-key> with the secret key generated for Grandpa. Enter the password when prompted.

Verify that the keys are in the keystore for node01:

bash
Copy code
ls /tmp/node01/chains/local_testnet/keystore
Restart node01 as Substrate nodes require a restart after adding the Grandpa key:

bash
Copy code
./target/release/node-template
  --base-path /tmp/node01
  --chain ./customSpecRaw.json
  --port 30333
  --ws-port 9945
  --rpc-port 9933
  --telemetry-url 'wss://telemetry.polkadot.io/submit/0'
  --validator
  --rpc-methods Unsafe
  --name MyNode01
  --password-interactive
Repeat the same steps for additional nodes you want to add to the network.

Enabling Other Participants to Join
To allow other validators to join the network, follow these steps for each additional node:

Start the node with the custom specification:

bash
Copy code
./target/release/node-template
  --base-path /tmp/node02
  --chain ./customSpecRaw.json
  --port 30334
  --ws-port 9946
  --rpc-port 9934
  --telemetry-url "wss://telemetry.polkadot.io/submit/0"
  --validator
  --rpc-methods Unsafe
  --name MyNode02
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWLmrYDLoNTyTYtRdDyZLWDe1paxzxTw5RgjmHLfzW96SX
  --password-interactive
Add the aura and grandpa authority keys for the new node:

bash
Copy code
./target/release/node-template key insert --base-path /tmp/node02
  --chain customSpecRaw.json
  --scheme Sr25519
  --suri <second-participant-secret-seed>
  --password-interactive
  --key-type aura
bash
Copy code
./target/release/node-template key insert