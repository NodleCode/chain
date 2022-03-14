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
STAKER_IMAGE=${STAKER_IMAGE:-"us-docker.pkg.dev/staking-testnet/nodle-staking:latest"}
STAKER_PORT=${STAKER_PORT:-"30600"}
STAKER_RPC_PORT=${STAKER_RPC_PORT:-"8600"}
STAKER_WS_PORT=${STAKER_WS_PORT:-"9600"}
STAKER_FIRST_BOOTNODE_ADDR=${STAKER_FIRST_BOOTNODE_ADDR:-"/ip4/127.0.0.1/tcp/30601/p2p/12D3KooWK2UnsrZWbpKvHoEzzmZBMTBJ9bHD7ftYsUTGduABazDW"}
STAKER_SECOND_BOOTNODE_ADDR=${STAKER_SECOND_BOOTNODE_ADDR:-"/ip4/127.0.0.1/tcp/30602/p2p/12D3KooWHVxy2CxrySd2fPNuDd66CUTMjK4xxbA9c3QxGzuvtDWm"}

STAKER_TNET_SPEC_CONFIG=${STAKER_TNET_SPEC_CONFIG:-"node/res/staking.json"}

delete_container() {
    local container_name=$1

    docker stop $container_name
    docker rm $container_name
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

initial_container_configurations() {
    local container_name=$1

    mkdir -p $DATA_DIR/$container_name
}

# Init

docker \
	container \
	stop \
	$(docker container ls -aq --filter name="$STAKER_NODE_PREFIX*") \
	&> /dev/null || true

docker \
	container \
	rm \
	$(docker container ls -aq --filter name="$STAKER_NODE_PREFIX*") \
	&> /dev/null || true

mkdir -p $DATA_DIR

cp $PROJECT_ROOT/$STAKER_TNET_SPEC_CONFIG $DATA_DIR/staking-tnet-spec.json

# Staker Validators

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

launch_container() {
    local container_name=$1
    local validator_extra_params=$2

    docker run \
        -d \
        -v $DATA_DIR/staking-tnet-spec.json:/nodle/staking-tnet-spec.json \
        -v $DATA_DIR/$container_name:/data \
        --name=$container_name \
        --network=host \
        --restart=always \
        $STAKER_IMAGE \
        --allow-private-ipv4 \
        --base-path=/data \
        --chain=/nodle/staking-tnet-spec.json \
        --discover-local \
        --name=$container_name \
        $validator_extra_params
}

launch_configured_node() {
    local idx=$1

    local container_name="$STAKER_NODE_PREFIX-$idx"
    local relay_port=$(($STAKER_PORT + $idx))
    local relay_rpc_port=$(($STAKER_RPC_PORT + $idx))
    local relay_ws_port=$(($STAKER_WS_PORT + $idx))
    local relay_port_extra="--port=$relay_port --rpc-port=$relay_rpc_port --ws-port=$relay_ws_port"

    initial_container_configurations "$container_name"

    if (( $idx <= 0 ))
    then
        launch_container \
			"$container_name" \
			"--bootnodes=$STAKER_FIRST_BOOTNODE_ADDR --bootnodes=$STAKER_SECOND_BOOTNODE_ADDR --pruning archive --rpc-cors=all --rpc-external --ws-external $relay_port_extra"
    else
        launch_container \
			"$container_name" \
			"--rpc-cors=all --rpc-methods=Unsafe --unsafe-rpc-external --validator $relay_port_extra"

        sleep 10
        inject_keys $idx $relay_rpc_port
        delete_container "$container_name"

        if (( $idx == 1 ))
        then
            launch_container \
				"$container_name" \
				"--bootnodes=$STAKER_SECOND_BOOTNODE_ADDR --node-key=$STAKER_FIRST_BOOTNODE_NODE_KEY --validator $relay_port_extra"

        elif (( $idx == 2 ))
        then
            launch_container \
				"$container_name" \
				"--bootnodes=$STAKER_FIRST_BOOTNODE_ADDR --node-key=$STAKER_SECOND_BOOTNODE_NODE_KEY --validator $relay_port_extra"
        else
            launch_container \
				"$container_name" \
				"--bootnodes=$STAKER_FIRST_BOOTNODE_ADDR --bootnodes=$STAKER_SECOND_BOOTNODE_ADDR --validator $relay_port_extra"
        fi
    fi
}

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
