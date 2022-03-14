#!/usr/bin/env bash

set -euxo pipefail

PROJECT_ROOT=$(git rev-parse --show-toplevel)

source "$(dirname "$0")"/init-env.sh

cd "$PROJECT_ROOT"

DATA_DIR=${DATA_DIR:-"$PWD/nodle-staking-tnet"}

# Avoid committing the secrets
OVERALL_SECRET=${OVERALL_SECRET:-""}
STAKER_FIRST_BOOTNODE_NODE_KEY=${STAKER_FIRST_BOOTNODE_NODE_KEY:-""}
STAKER_SECOND_BOOTNODE_NODE_KEY=${STAKER_SECOND_BOOTNODE_NODE_KEY:-""}

STAKER_NODE_PREFIX=${STAKER_NODE_PREFIX:-"nodle-staking-tnet"}
STAKER_BIN=${STAKER_BIN:-"target/release/nodle-staking"}
STAKER_IMAGE=${STAKER_IMAGE:-"us-docker.pkg.dev/staking-testnet/nodle-staking:latest"}
STAKER_PORT=${STAKER_PORT:-"30600"}
STAKER_RPC_PORT=${STAKER_RPC_PORT:-"8600"}
STAKER_WS_PORT=${STAKER_WS_PORT:-"9600"}
STAKER_FIRST_BOOTNODE_ADDR=${STAKER_FIRST_BOOTNODE_ADDR:-"/ip4/127.0.0.1/tcp/30601/p2p/12D3KooWK2UnsrZWbpKvHoEzzmZBMTBJ9bHD7ftYsUTGduABazDW"}
STAKER_SECOND_BOOTNODE_ADDR=${STAKER_SECOND_BOOTNODE_ADDR:-"/ip4/127.0.0.1/tcp/30602/p2p/12D3KooWHVxy2CxrySd2fPNuDd66CUTMjK4xxbA9c3QxGzuvtDWm"}

STAKER_TNET_SPEC_CONFIG=${STAKER_TNET_SPEC_CONFIG:-"staking"}

mkdir -p $DATA_DIR

# setup variables
declare g_node_pid=""
declare -a node_pids
declare -a node_pipes

# create a sed expression which injects the node name and stream type into each line
function make_sed_expr() {
  name="$1"
  type="$2"

  printf "s/^/%8s %s: /" "$name" "$type"
}

initial_node_configurations() {
    local node_name=$1

    mkdir -p $DATA_DIR/$node_name
}

generate_account_id() {
    docker \
		run \
		--rm \
		$STAKER_IMAGE \
		key \
		inspect \
		-n nodle \
		${3:-} \
		"$OVERALL_SECRET//$1//$2" \
		| grep "Account ID" \
		| awk '{ print $3 }'
}

generate_author_insertKey_with_account_id() {
    ACCOUNT=$(generate_account_id $1 $2 "$3")
    SEED=$OVERALL_SECRET//$1//$2

    printf '{"jsonrpc":"2.0","id":1,"method":"author_insertKey","params":["'"$4"'","'"$SEED"'","'"$ACCOUNT"'"]}'
}

inject_keys() {
    local idx=$1
    local rpc_port=$2

    curl \
		http://localhost:$rpc_port \
		-H "Content-Type:application/json;charset=utf-8" \
		-d "$(generate_author_insertKey_with_account_id $idx babe '--scheme sr25519' babe)"

    curl \
		http://localhost:$rpc_port \
		-H "Content-Type:application/json;charset=utf-8" \
		-d "$(generate_author_insertKey_with_account_id $idx grandpa '--scheme ed25519' gran)"

    curl \
		http://localhost:$rpc_port \
		-H "Content-Type:application/json;charset=utf-8" \
		-d "$(generate_author_insertKey_with_account_id $idx im_online '--scheme sr25519' imon)"

    curl \
		http://localhost:$rpc_port \
		-H "Content-Type:application/json;charset=utf-8" \
		-d "$(generate_author_insertKey_with_account_id $idx authority_discovery '--scheme sr25519' audi)"
}

delete_node() {
    local node_pid=$1

	kill -9 "$node_pid"
}

# start a node and label its output
#
# This function takes a single argument, the node name.
launch_node() {
    local node_name=$1
    local validator_extra_params=$2

	# create a named pipe so we can get the node's PID while also sedding its output
	local stdout
	local stderr
	stdout=$(mktemp --dry-run --tmpdir)
	stderr=$(mktemp --dry-run --tmpdir)
	mkfifo "$stdout"
	mkfifo "$stderr"
	node_pipes+=("$stdout")
	node_pipes+=("$stderr")

	$STAKER_BIN \
		--name="$node_name" \
        --chain="$STAKER_TNET_SPEC_CONFIG" \
        --base-path="$DATA_DIR/$node_name" \
		--allow-private-ipv4 \
        --discover-local \
        $validator_extra_params \
	>  "$stdout" \
	2> "$stderr" \
	&

	g_node_pid=$!

	# send output from the stdout pipe to stdout, prepending the node name
	sed -e "$(make_sed_expr "$node_name" "OUT")" "$stdout" >&1 &
	# send output from the stderr pipe to stderr, prepending the node name
	sed -e "$(make_sed_expr "$node_name" "INFO")" "$stderr" >&2 &
}

launch_configured_node() {
    local idx=$1
	local local_node_pid=""
    local node_name="$STAKER_NODE_PREFIX-$idx"
    local relay_port=$(($STAKER_PORT + $idx))
    local relay_rpc_port=$(($STAKER_RPC_PORT + $idx))
    local relay_ws_port=$(($STAKER_WS_PORT + $idx))
    local relay_port_extra="--port=$relay_port --rpc-port=$relay_rpc_port --ws-port=$relay_ws_port"

    initial_node_configurations "$node_name"

    if (( $idx <= 0 ))
    then
        launch_node \
			"$node_name" \
			"--bootnodes=$STAKER_FIRST_BOOTNODE_ADDR --bootnodes=$STAKER_SECOND_BOOTNODE_ADDR --pruning archive --rpc-cors=all --rpc-external --ws-external $relay_port_extra"
		node_pids+=("$g_node_pid")
    else
        launch_node \
			"$node_name" \
			"--rpc-cors=all --rpc-methods=Unsafe --unsafe-rpc-external --validator $relay_port_extra"

        sleep 10
        inject_keys $idx $relay_rpc_port
        delete_node "$g_node_pid"

        if (( $idx == 1 ))
        then
            launch_node \
				"$node_name" \
				"--bootnodes=$STAKER_SECOND_BOOTNODE_ADDR --node-key=$STAKER_FIRST_BOOTNODE_NODE_KEY --validator $relay_port_extra"
			node_pids+=("$g_node_pid")
        elif (( $idx == 2 ))
        then
            launch_node \
				"$node_name" \
				"--bootnodes=$STAKER_FIRST_BOOTNODE_ADDR --node-key=$STAKER_SECOND_BOOTNODE_NODE_KEY --validator $relay_port_extra"
			node_pids+=("$g_node_pid")
        else
            launch_node \
				"$node_name" \
				"--bootnodes=$STAKER_FIRST_BOOTNODE_ADDR --bootnodes=$STAKER_SECOND_BOOTNODE_ADDR --validator $relay_port_extra"
			node_pids+=("$g_node_pid")
        fi
    fi
}

# clean up the nodes when this script exits
function finish {
  for node_pid in "${node_pids[@]}"; do
    kill -9 "$node_pid"
  done
  for node_pipe in "${node_pipes[@]}"; do
    rm "$node_pipe"
  done
}
trap finish EXIT

# start the nodes
launch_configured_node 0
launch_configured_node 1
launch_configured_node 2
launch_configured_node 3
# launch_configured_node 4
# launch_configured_node 5
# launch_configured_node 6
# launch_configured_node 7
# launch_configured_node 8
# launch_configured_node 9
# launch_configured_node 10


# now wait; this will exit on its own only if both subprocesses exit
# the practical implication, as both subprocesses are supposed to run forever, is that
# this script will also run forever, until killed, at which point the exit trap should kill
# the subprocesses
wait
