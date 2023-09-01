// Starting the first node (Alice)
// Purge old chain data
./target/release/node-template purge-chain --base-path /tmp/alice --chain local
// Confirm the operation
// Press 'Y' when prompted

// Start the local blockchain node using the "alice" account
./target/release/node-template \
  --base-path /tmp/alice \
  --chain local \
  --alice \
  --port 30333 \
  --ws-port 9945 \
  --rpc-port 9933 \
  --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
  --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
  --validator

// Adding another node (Bob)
// Purge old chain data with the -y flag to confirm
./target/release/node-template purge-chain --base-path /tmp/bob --chain local -y

// Start the new node using the "bob" account
./target/release/node-template \
  --base-path /tmp/bob \
  --chain local \
  --bob \
  --port 30334 \
  --ws-port 9946 \
  --rpc-port 9934 \
  --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
  --validator \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp

// Verify Block production
// Observe both terminals for similar messages
// You will see node identity discovered and 1 peer, which is the other node
// Both nodes will produce and finalize blocks

// Shut down one of the nodes by pressing control+c
// The first node will have 0 peers remaining

