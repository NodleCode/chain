CHAIN="eden-dev"

cargo build

./target/debug/nodle-chain export-genesis-state --chain $CHAIN --parachain-id 2000 > .maintain/head.hex
./target/debug/nodle-chain export-genesis-wasm --chain $CHAIN > .maintain/wasm.hex