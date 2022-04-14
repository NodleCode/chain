#!/usr/bin/env bash

set -euxo pipefail

PROJECT_ROOT=$(git rev-parse --show-toplevel)

source "$(dirname "$0")"/init-env.sh

cd "$PROJECT_ROOT"

delete_container() {
    local container_name=$1

    docker stop $container_name
    docker rm $container_name
}

generate_account_id() {
    docker \
		run \
		--rm \
		$RC_IMAGE \
		key \
		inspect \
		-n nodle \
		${2:-} \
		"$1" \
		| grep "Account ID" \
		| awk '{ print $3 }'
}

generate_author_insertKey_with_account_id() {
    ACCOUNT=$(generate_account_id "$1" "$2")
    SEED="$1"

    printf '{"jsonrpc":"2.0","id":1,"method":"author_insertKey","params":["'"$3"'","'"$SEED"'","'"$ACCOUNT"'"]}'
}

initial_container_configurations() {
    local container_name=$1

    mkdir -p $DATA_DIR/$container_name
}

# Init

docker \
	container \
	stop \
	$(docker container ls -aq --filter name="$RC_PREFIX*") \
	&> /dev/null || true

docker \
	container \
	rm \
	$(docker container ls -aq --filter name="$RC_PREFIX*") \
	&> /dev/null || true

docker \
	container \
	stop \
	$(docker container ls -aq --filter name="$PC_PREFIX*") \
	&> /dev/null || true

docker \
	container \
	rm \
	$(docker container ls -aq --filter name="$PC_PREFIX*") \
	&> /dev/null || true

mkdir -p $DATA_DIR

cp $PROJECT_ROOT/$RC_SPEC $DATA_DIR/rc-spec.json
cp $PROJECT_ROOT/$PC_SPEC $DATA_DIR/pc-spec.json

# Staker Validators

inject_keys() {
    local rpc_port=$2

    curl \
		http://localhost:$rpc_port \
		-H "Content-Type:application/json;charset=utf-8" \
		-d "$(generate_author_insertKey_with_account_id "$1" '--scheme sr25519' aura)"
}

launch_rc_container() {
    local container_name=$1
    local validator_extra_params=$2

    docker run \
        -d \
        -v $DATA_DIR/rc-spec.json:/nodle/rc-spec.json \
        -v $DATA_DIR/$container_name:/data \
        --name=$container_name \
        --network=host \
        --restart=always \
        $RC_IMAGE \
        --allow-private-ipv4 \
        --base-path=/data \
        --chain=/nodle/rc-spec.json \
        --discover-local \
        --name=$container_name \
        $validator_extra_params
}

launch_pc_container() {
    local container_name=$1
    local collator_extra_params=$2
	local rc_extra_params=$3

    docker run \
        -d \
        -v $DATA_DIR/pc-spec.json:/nodle/pc-spec.json \
		-v $DATA_DIR/rc-spec.json:/nodle/rc-spec.json \
        -v $DATA_DIR/$container_name:/data \
        --name=$container_name \
        --network=host \
        --restart=always \
        $PC_IMAGE \
        --allow-private-ipv4 \
        --base-path=/data \
        --chain=/nodle/pc-spec.json \
        --name=$container_name \
        $collator_extra_params \
		-- \
		--chain=/nodle/rc-spec.json \
		$rc_extra_params
}

launch_rc_configured_node() {
    local idx=$1

    local container_name="$RC_PREFIX-$idx"
    local relay_port=$(($RC_PORT + $idx))
    local relay_rpc_port=$(($RC_RPC_PORT + $idx))
    local relay_ws_port=$(($RC_WS_PORT + $idx))
    local relay_port_extra="--port=$relay_port --rpc-port=$relay_rpc_port --ws-port=$relay_ws_port"
	local launch_extra_params=""

    initial_container_configurations "$container_name"

	launch_extra_params+="--bootnodes=$RC_01_P2P_ADDR "
	launch_extra_params+="--bootnodes=$RC_02_P2P_ADDR "

    if (( $idx <= 1 ))
    then
		launch_extra_params+="--rpc-cors=all "
		launch_extra_params+="--rpc-external "
		launch_extra_params+="--rpc-methods=Unsafe "
		launch_extra_params+="--unsafe-rpc-external "
		launch_extra_params+="--ws-external "
		launch_extra_params+="$relay_port_extra "

        launch_rc_container "$container_name" "--alice --validator $launch_extra_params"

    else
		launch_extra_params+="--rpc-cors=all "
		launch_extra_params+="--rpc-external "
		launch_extra_params+="--rpc-methods=Unsafe "
		launch_extra_params+="--unsafe-rpc-external "
		launch_extra_params+="--ws-external "
		launch_extra_params+="$relay_port_extra "

        if (( $idx == 2 ))
        then
            launch_rc_container \
				"$container_name" \
				"--bob --validator $launch_extra_params"

        else
            launch_rc_container \
				"$container_name" \
				"$launch_extra_params"
        fi
    fi
}

launch_pc_configured_node() {
    local idx=$1
	local collator_secrets=${2:-}
	local node_key=${3:-}

    local container_name="$PC_PREFIX-$idx"

    local pc_port=$(($PC_PORT + $idx))
    local pc_rpc_port=$(($PC_RPC_PORT + $idx))
    local pc_ws_port=$(($PC_WS_PORT + $idx))
    local pc_port_extra="--port=$pc_port --rpc-port=$pc_rpc_port --ws-port=$pc_ws_port"

    local relay_args_port=$(($PC_RARGS_PORT + $idx))
    local relay_args_rpc_port=$(($PC_RARGS_RPC_PORT + $idx))
    local relay_args_ws_port=$(($PC_RARGS_WS_PORT + $idx))

	local relay_args_port_extra=""
	relay_args_port_extra+="--port=$relay_args_port "
	relay_args_port_extra+="--rpc-port=$relay_args_rpc_port "
	relay_args_port_extra+="--ws-port=$relay_args_ws_port "

	local launch_pc_extra_params=""
	local launch_rc_extra_params=""

    initial_container_configurations "$container_name"

	launch_pc_extra_params+="--bootnodes=$PC_01_P2P_ADDR "
	launch_pc_extra_params+="--bootnodes=$PC_02_P2P_ADDR "
	launch_pc_extra_params+="--bootnodes=$PC_03_P2P_ADDR "
	launch_pc_extra_params+="--bootnodes=$PC_04_P2P_ADDR "
	launch_pc_extra_params+="--bootnodes=$PC_05_P2P_ADDR "

	launch_rc_extra_params+="--bootnodes=$RC_01_P2P_ADDR "
	launch_rc_extra_params+="--bootnodes=$RC_02_P2P_ADDR "

    if (( $idx <= 0 ))
    then

		launch_pc_extra_params+="--pruning archive "
		launch_pc_extra_params+="--rpc-cors=all "
		launch_pc_extra_params+="--rpc-external "
		launch_pc_extra_params+="--ws-external "
		launch_pc_extra_params+="$pc_port_extra "

		launch_rc_extra_params+="--rpc-cors=all "
		launch_rc_extra_params+="--rpc-external "
		launch_rc_extra_params+="--ws-external "
		launch_rc_extra_params+="--no-telemetry "
		launch_rc_extra_params+="$relay_args_port_extra "

        launch_pc_container \
			"$container_name" \
			"$launch_pc_extra_params" \
			"$launch_rc_extra_params"

    else
		launch_pc_extra_params+="--rpc-cors=all "
		launch_pc_extra_params+="--unsafe-pruning "
		launch_pc_extra_params+="--collator "
		launch_pc_extra_params+="--force-authoring "
		launch_pc_extra_params+="--node-key=$node_key "
		launch_pc_extra_params+="$pc_port_extra "

		launch_rc_extra_params+="--rpc-cors=all "
		launch_rc_extra_params+="--rpc-external "
		launch_rc_extra_params+="--ws-external "
		launch_rc_extra_params+="--unsafe-pruning "
		launch_rc_extra_params+="--no-telemetry "
		launch_rc_extra_params+="--node-key=$node_key "
		launch_rc_extra_params+="$relay_args_port_extra "

        launch_pc_container \
			"$container_name" \
			"--rpc-external --ws-external --rpc-methods=Unsafe --unsafe-rpc-external $launch_pc_extra_params" \
			"$launch_rc_extra_params"

        sleep 180
        inject_keys "$collator_secrets" $pc_rpc_port
        delete_container "$container_name"

        launch_pc_container \
			"$container_name" \
			"$launch_pc_extra_params" \
			"$launch_rc_extra_params"

    fi
}

## Launch the RC nodes
launch_rc_configured_node 1
launch_rc_configured_node 2

## Launch the PC nodes
launch_pc_configured_node 0
launch_pc_configured_node 1 "$PC_COLLATOR_01_SECRET" "$PC_01_BOOTNODE_KEY"
launch_pc_configured_node 2 "$PC_COLLATOR_02_SECRET" "$PC_02_BOOTNODE_KEY"
launch_pc_configured_node 3 "$PC_COLLATOR_03_SECRET" "$PC_03_BOOTNODE_KEY"
launch_pc_configured_node 4 "$PC_COLLATOR_04_SECRET" "$PC_04_BOOTNODE_KEY"
launch_pc_configured_node 5 "$PC_COLLATOR_05_SECRET" "$PC_05_BOOTNODE_KEY"
