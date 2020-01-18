# Nodle Chain

A Blockchain node for the Nodle Chain to connect and secure the next trillion things.

> Built on [Substrate](substrate.dev).


# Development

## Building
```
cargo build
```

## Testing
```
cargo test -p nodle-chain-runtime
```

## Installing
```
cargo install
```

# Usage
```
nodle-chain purge-chain --dev # Purge old chain data
nodle-chain --dev             # Run a single node testnet
```

## With docker

1. Build the image: `docker build -t nodle/chain -f docker/Dockerfile .`.
2. Run it: `docker run -v /path/to/local/repertory:/data -p 9944:9944 -it nodle/chain`.