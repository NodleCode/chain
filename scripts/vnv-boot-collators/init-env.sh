#!/usr/bin/env bash

# Avoid committing the secrets
export COLLATOR1_OVERALL_SECRET=""
export COLLATOR1_RPC_HTTP_ENDPOINT="https://node-6907886836151037952.lh.onfinality.io/rpc?apikey=266ee86b-d5c6-420b-a2e0-6c33ce00442f"

export COLLATOR2_OVERALL_SECRET=""
export COLLATOR2_RPC_HTTP_ENDPOINT="https://node-6907886965943775232.lh.onfinality.io/rpc?apikey=eb3f18f1-2b23-4747-92fa-ea65db73c43e"

export COLLATOR3_OVERALL_SECRET=""
export COLLATOR3_RPC_HTTP_ENDPOINT="https://node-6907887090221178880.lh.onfinality.io/rpc?apikey=407e444f-aa56-4897-9dc7-6c9941d00de4"

export COLLATOR4_OVERALL_SECRET=""
export COLLATOR4_RPC_HTTP_ENDPOINT="https://node-6907887240700223488.lh.onfinality.io/rpc?apikey=be137b79-1706-4194-bcc0-cf9b0932e9bc"

export COLLATOR5_OVERALL_SECRET=""
export COLLATOR5_RPC_HTTP_ENDPOINT="https://node-6907887383360909312.lh.onfinality.io/rpc?apikey=b4a00cf0-62e6-46cc-8d3d-1d420a07b1cb"

export COLLATOR_IMAGE="ghcr.io/nodlecode/chain:polkadot-0.9.17"