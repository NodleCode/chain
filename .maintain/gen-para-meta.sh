CHAIN="eden-dev"

cargo run -- export-genesis-state --chain $CHAIN > .maintain/head.hex
cargo run -- export-genesis-wasm --chain $CHAIN > .maintain/wasm.hex