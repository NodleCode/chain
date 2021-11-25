#! /bin/bash

# make sure cargo and rust are in our
export PATH=$PATH:$HOME/.cargo/bin

# wait for relay chain node to be available
echo "Waiting for relay chain node..."
while ! nc -z node-polkadot-alice 9944; do   
  sleep 1
done

# register and onboard the parachain on the relay chain
echo "Registering parachain (should take about 2 minutes)..."
polkadot-js-api --ws ws://node-polkadot-alice:9944 --seed "//Alice" --sudo tx.parasSudoWrapper.sudoScheduleParaInitialize 2000 '["`cat .maintain/head.hex`", "`cat .maintain/wasm.hex`", true]'

# keep running for vscode
echo "Ready for battle"
while true; do sleep 1000; done
