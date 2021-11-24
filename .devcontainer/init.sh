#! /bin/bash

CHAIN="eden-dev"

# make sure cargo and rust are in our path
export PATH=$PATH:$HOME/.cargo/bin

# export our artefacts and cache dependencies in the process
echo "Building artefacts and preloading dependencies..."
cargo run -- export-genesis-head --chain $CHAIN > /tmp/head.hex 2>> /tmp/init.log
cargo run -- export-genesis-wasm --chain $CHAIN > /tmp/wasm.hex 2>> /tmp/init.log

# wait for relay chain node to be available
echo "Waiting for relay chain node..."
while ! nc -z node-polkadot 9944; do   
  sleep 0.1 # wait for 1/10 of the second before check again
done

# register the parachain on the relay chain
echo "Registering parachain..."
polkadot-js-api --ws ws://node-polkadot:9944 --seed "//Alice" tx

# keep running for vscode
echo "Ready for battle"
while true; do sleep 1000; done