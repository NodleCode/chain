#! /bin/bash

NODE_POLKA="node-polkadot:9944"
NODE_NODLE="node-nodle:9944"

CHAIN="eden-dev"

# make sure cargo and rust are in our path
export PATH=$PATH:$HOME/.cargo/bin

# export our artefacts and cache dependencies in the process
echo "Building artefacts and preloading dependencies..."
cargo run -- export-genesis-head --chain $CHAIN > /tmp/head.hex 2>> /tmp/init.log
cargo run -- export-genesis-wasm --chain $CHAIN > /tmp/wasm.hex 2>> /tmp/init.log

# wait for relay chain node to be available
echo "Waiting for relay chain node..."

# register the parachain on the relay chain
echo "Registering parachain..."

# keep running for vscode
echo "Ready for battle"
while true; do sleep 1000; done
