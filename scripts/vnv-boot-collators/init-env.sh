#!/usr/bin/env bash

# Avoid committing the secrets
export COLLATOR1_OVERALL_SECRET=""
export COLLATOR1_RPC_HTTP_ENDPOINT="https://node-6902949049423048704.lh.onfinality.io/rpc?apikey=3fd2b397-22f6-4f69-9933-374e5b997a72"

export COLLATOR2_OVERALL_SECRET=""
export COLLATOR2_RPC_HTTP_ENDPOINT="https://node-6902949221541761024.lh.onfinality.io/rpc?apikey=47cbd5c4-2fb0-4949-a05a-79976716ef4e"

export COLLATOR3_OVERALL_SECRET=""
export COLLATOR3_RPC_HTTP_ENDPOINT="https://node-6902966549867081728.lh.onfinality.io/rpc?apikey=b7d76ee1-bfb9-42ac-8269-06665e7ac8c8"

export COLLATOR4_OVERALL_SECRET=""
export COLLATOR4_RPC_HTTP_ENDPOINT="https://node-6902966717992816640.lh.onfinality.io/rpc?apikey=be2f713d-d521-49ed-a59c-5289d2043c70"

export COLLATOR5_OVERALL_SECRET=""
export COLLATOR5_RPC_HTTP_ENDPOINT="https://node-6902966878802432000.lh.onfinality.io/rpc?apikey=8ea0334e-3706-47e7-a208-64288510d8b4"

export COLLATOR_IMAGE="nodlecode/chain:2.0.16"