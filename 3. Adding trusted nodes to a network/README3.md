# Setting Up a Permissioned Blockchain Network with Substrate
This guide will walk you through the process of setting up a permissioned blockchain network using the Substrate framework. In this network, only authorized nodes are allowed to participate. This guide assumes you have already compiled the Substrate node and are familiar with basic Substrate concepts.

1. Table of Contents
2. Generate Account and Keys
3. Create a Custom Chain Specification
4. Prepare for Launch
Enable Other Participants to Join
1. #### Generate Account and Keys
Open a terminal and navigate to the project directory where you've compiled the Substrate node. Follow these steps to generate keys:

-Generate a seed phrase and keys:

./target/release/node-template key generate --scheme Sr25519 --password-interactive


-Use the seed phrase to derive keys with the Ed25519 signature scheme (replace <your-secret-seed> with your actual seed phrase):

./target/release/node-template key inspect --password-interactive --scheme Ed25519 "<your-secret-seed>"

2. #### Create a Custom Chain Specification
After generating keys, you'll need to create a custom chain specification. To do this, follow these steps:

---- Export the local chain specification to a file named customSpec.json:

./target/release/node-template build-spec --disable-default-bootnode --chain local > customSpec.json

-Open customSpec.json in a text editor and make the desired modifications:

--- Change the network name under "name".
Modify the "aura" and "grandpa" fields to specify authorities for block creation and finalization.
Convert the customSpec.json to a raw format specification:


./target/release/node-template build-spec --chain=customSpec.json --raw --disable-default-bootnode > customSpecRaw.json


3. #### Prepare for Launch
With the custom chain specification prepared, you can now launch your private blockchain node. To start the first node, run the following command:
 
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

-You will be prompted to enter the password you used earlier.

Add the aura and grandpa authority keys to enable block production and finalization.

Restart the node once the grandpa key is entered.
 

4. ####  Enable Other Participants to Join
To allow other validators to join the network, follow similar steps as for the first node:

Start the second node:


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


-Add aura and grandpa keys for the second node.

Restart the second node after entering the grandpa key.

Repeat these steps to add more nodes to your network as needed. You can automate this process using custom bash scripts.

Remember to verify that the keys are present in the respective keystores of each node:


ls /tmp/node01/chains/local_testnet/keystore
ls /tmp/node02/chains/local_testnet/keystore










