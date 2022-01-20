#!/usr/bin/env bash
set -euxo pipefail

PROJECT_ROOT=$(git rev-parse --show-toplevel)

source "$(dirname "$0")"/init-env.sh

cd "$PROJECT_ROOT"

# Avoid committing the secrets
OVERALL_SECRET=${OVERALL_SECRET:-""}
STAKER_IMAGE=${STAKER_IMAGE:-"us-docker.pkg.dev/staking-testnet/nodle-chain:latest"}

if [ "$#" -ne 1 ]; then
	echo "Please provide the number of initial validators!"
	exit 1
fi

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

generate_address() {
    docker \
		run \
		--rm \
		$STAKER_IMAGE \
		key \
		inspect \
		-n nodle \
		${3:-} \
		"$OVERALL_SECRET//$1//$2" \
		| grep "SS58 Address" \
		| awk '{ print $3 }'
}

generate_address_and_account_id() {
	ACCOUNT=$(generate_account_id $1 $2 "$3")
	ADDRESS=$(generate_address $1 $2 "$3")
	if ${4:-false}; then
		INTO="unchecked_into"
	else
		INTO="into"
	fi

	printf "//$ADDRESS\nhex![\"${ACCOUNT#'0x'}\"].$INTO(),"
}

V_NUM=$1

AUTHORITIES=""

for i in $(seq 1 $V_NUM); do
	AUTHORITIES+="(\n"
	AUTHORITIES+="$(generate_address_and_account_id $i stash '--scheme sr25519')\n"
	AUTHORITIES+="$(generate_address_and_account_id $i controller '--scheme sr25519')\n"
	AUTHORITIES+="$(generate_address_and_account_id $i grandpa '--scheme ed25519' true)\n"
	AUTHORITIES+="$(generate_address_and_account_id $i babe '--scheme sr25519' true)\n"
	AUTHORITIES+="$(generate_address_and_account_id $i im_online '--scheme sr25519' true)\n"
	AUTHORITIES+="$(generate_address_and_account_id $i authority_discovery '--scheme sr25519' true)\n"
	AUTHORITIES+="),\n"
done

printf "$AUTHORITIES"
