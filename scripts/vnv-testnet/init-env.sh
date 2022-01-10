#!/usr/bin/env bash

# Avoid committing the secrets
export OVERALL_SECRET=""
export STAKER_FIRST_BOOTNODE_NODE_KEY=""
export STAKER_SECOND_BOOTNODE_NODE_KEY=""

export DATA_DIR="$HOME/nodle-staking-tnet"

export STAKER_NODE_PREFIX="nodle-staking-tnet"
export STAKER_IMAGE="us-docker.pkg.dev/staking-testnet/nodle-chain:latest"
export STAKER_BIN="target/release/nodle-chain"
export STAKER_PORT="30600"
export STAKER_RPC_PORT="8600"
export STAKER_WS_PORT="9600"

export STAKER_FIRST_BOOTNODE_ADDR="/ip4/127.0.0.1/tcp/30601/p2p/12D3KooWK2UnsrZWbpKvHoEzzmZBMTBJ9bHD7ftYsUTGduABazDW"
export STAKER_SECOND_BOOTNODE_ADDR="/ip4/127.0.0.1/tcp/30602/p2p/12D3KooWHVxy2CxrySd2fPNuDd66CUTMjK4xxbA9c3QxGzuvtDWm"

# export STAKER_TNET_SPEC_CONFIG="node/res/tnet-staking.json"
export STAKER_TNET_SPEC_CONFIG="staking-tnet-templ"
# export STAKER_TNET_SPEC_CONFIG="staking-tnet"
