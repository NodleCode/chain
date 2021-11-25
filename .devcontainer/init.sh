#! /bin/bash

CHAIN="eden-dev"

# make sure cargo and rust are in our path
export PATH=$PATH:$HOME/.cargo/bin

# export our artefacts and cache dependencies in the process
echo "Building artefacts and preloading dependencies..."
./scripts/init.sh
cargo run -- export-genesis-state --chain $CHAIN > /tmp/head.hex
cargo run -- export-genesis-wasm --chain $CHAIN > /tmp/wasm.hex

# wait for relay chain node to be available
echo "Waiting for relay chain node..."
while ! nc -z node-polkadot-alice 9944; do   
  sleep 1
done

# register and onboard the parachain on the relay chain
echo "Registering parachain (should take about 2 minutes)..."
polkadot-js-api --ws ws://node-polkadot-alice:9944 --seed "//Alice" --sudo tx.parasSudoWrapper.sudoScheduleParaInitialize 2000 '["`cat /tmp/head.hex`", "`cat /tmp/wasm.hex`", true]'

# keep running for vscode
echo "Ready for battle"
while true; do sleep 1000; done