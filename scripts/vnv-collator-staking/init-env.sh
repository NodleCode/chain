#!/usr/bin/env bash

## Common
export DATA_DIR="$HOME/dscbox/sun/trash/vnv-01-eden-staking"

## Relay Chain
export RC_IMAGE="parity/polkadot:v0.9.17"
export RC_SPEC="scripts/vnv-collator-staking/rococo-local.json"

export RC_01_BOOTNODE_KEY=""
export RC_02_BOOTNODE_KEY=""

export RC_VALIDATOR_01_SECRET=""
export RC_VALIDATOR_02_SECRET=""

export RC_PREFIX="dot-tnet"

export RC_PORT="20520"
export RC_RPC_PORT="8600"
export RC_WS_PORT="9600"

export RC_01_P2P_ADDR="/ip4/127.0.0.1/tcp/20521/p2p/12D3KooWKhbBnk47SYoC9z4iB7UFh8uabpbsS46SghLMiaRZzcPB"
export RC_02_P2P_ADDR="/ip4/127.0.0.1/tcp/20522/p2p/12D3KooWHyHjXofp7ax5YVDfoEqffJhsmpyVPK6ySYGpFsUPZDGj"

## Parachain Chain
export PC_IMAGE="us-docker.pkg.dev/staking-testnet/nodle-parachain:latest"
export PC_SPEC="scripts/vnv-collator-staking/eden-staking-local-mock.json"

export PC_01_BOOTNODE_KEY=""
export PC_02_BOOTNODE_KEY=""
export PC_03_BOOTNODE_KEY=""
export PC_04_BOOTNODE_KEY=""
export PC_05_BOOTNODE_KEY=""

export PC_COLLATOR_01_SECRET=""
export PC_COLLATOR_02_SECRET=""
export PC_COLLATOR_03_SECRET=""
export PC_COLLATOR_04_SECRET=""
export PC_COLLATOR_05_SECRET=""

export PC_PREFIX="npc-tnet"

export PC_PORT="21520"
export PC_RPC_PORT="8620"
export PC_WS_PORT="9620"

export PC_RARGS_PORT="22520"
export PC_RARGS_RPC_PORT="8640"
export PC_RARGS_WS_PORT="9640"

export PC_01_P2P_ADDR="/ip4/127.0.0.1/tcp/21521/p2p/12D3KooWRg4uQj3vaxCAapoXKgBmQNTfV8oJwBsch53AZmijjwyH"
export PC_02_P2P_ADDR="/ip4/127.0.0.1/tcp/21522/p2p/12D3KooWEeGCxr9mizSMiELmj9ecKP1YYM1cRsZGYNiCwvoB9QJN"
export PC_03_P2P_ADDR="/ip4/127.0.0.1/tcp/21523/p2p/12D3KooWL9PCpynH2fPUwsCtxjrU8drsUjarTnHs38s2mCtcjKUF"
export PC_04_P2P_ADDR="/ip4/127.0.0.1/tcp/21524/p2p/12D3KooWD9V7nxFnxYbWTuiYZPvBQzRzN1UiNRgmno8sefYx6sPe"
export PC_05_P2P_ADDR="/ip4/127.0.0.1/tcp/21525/p2p/12D3KooWMUVmP8bjQKZoUt7VJH1yqrFiZDuG7cSgB4YbVWMvnDFx"
