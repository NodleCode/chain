# Nodle Chain

A Blockchain node for the Nodle Parachain to connect and secure the next trillion things.

> Built on [Substrate](https://substrate.dev).

[**Read the documentation**](https://nodlecode.github.io/chain/nodle_parachain/index.html)


# Live networks

## Eden
Syncing Nodle's Parachain (codename: `eden`) is done easily via:
```
cargo run --bin nodle-parachain --release -- --chain eden
```

There are a few more chains available, such as `eden-testing` or `dev`.


# Development

## Building
```
cargo build
```

## Testing
```
cargo test --all
```

## Installing
```
cargo install
```

## Run a local parachain and relay chain
Assuming that `polkadot` is in `/usr/local/bin` and that you installed [`polkadot-launch`](https://github.com/paritytech/polkadot-launch) you can simply use this command:
```
cargo build --release -p nodle-parachain && polkadot-launch launch.json
```

# Usage

## With docker

1. Build the image: `docker build -t nodle/chain -f ./Dockerfile .`.
2. Run it: 
        `docker run -v ~/.local/path_to_parachain_data_dir:/data -p 9944:9944 -p 9933:9933 -p30333:30333 -it nodle/chain --chain=eden-testing --base-path=/data --rpc-methods=safe --rpc-cors all --rpc-external -- --rpc-external`.
