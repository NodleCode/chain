![Nodle Banner](https://user-images.githubusercontent.com/10683430/80538204-2a6bef00-895a-11ea-94eb-2203ef6fae09.jpg)

# Nodle Chain

A Blockchain node for the Nodle Chain to connect and secure the next trillion things.

> Built on [Substrate](https://substrate.dev).


# Live networks

## Arcadia
This repository come with Arcadia's chain spec JSON file included. You can connect to the chain easily
by running the following command.
```
cargo run -- --chain arcadia
```


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

# Usage
```
nodle-chain purge-chain --dev # Purge old chain data
nodle-chain --dev             # Run a single node testnet
```

## With docker

1. Build the image: `docker build -t nodle/chain -f .maintain/docker/Dockerfile .`.
2. Run it: `docker run -v /path/to/local/repertory:/data -p 9944:9944 -it nodle/chain`.
