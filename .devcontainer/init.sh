#! /bin/bash

NODE_POLKA="node-polkadot:9944"
NODE_NODLE="node-nodle:9944"

# make sure cargo and rust are in our path
export PATH=$PATH:$HOME/.cargo/bin

# export our artefacts and cache dependencies in the process
echo "Building artefacts and preloading dependencies..."
cargo run -- export-genesis-head --chain dev > /tmp/head.hex 2>> /tmp/init.log
cargo run -- export-genesis-wasm --chain dev > /tmp/wasm.hex 2>> /tmp/init.log

# wait for relay chain node to be available
echo "Waiting for relay chain node..."

# register the parachain on the relay chain
echo "Registering parachain..."

# run whatever command was passed to us
echo "Ready for battle"
$@