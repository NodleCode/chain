#!/usr/bin/env bash
set -euxo pipefail

PROJECT_ROOT=$(git rev-parse --show-toplevel)

source "$(dirname "$0")"/init-env.sh

cd "$PROJECT_ROOT"

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

generate_address() {
    docker \
		run \
		--rm \
		$RC_IMAGE \
		key \
		inspect \
		-n nodle \
		${2:-} \
		"$1" \
		| grep "SS58 Address" \
		| awk '{ print $3 }'
}

generate_address_and_account_id() {
	ACCOUNT=$(generate_account_id "$1" "$2")
	ADDRESS=$(generate_address "$1" "$2")
	if ${3:-false}; then
		INTO="unchecked_into"
	else
		INTO="into"
	fi

	printf "//$ADDRESS\nhex![\"${ACCOUNT#'0x'}\"].$INTO(),"
}

AUTHORITIES=""

update_authorities() {
	AUTHORITIES+="(\n"
	AUTHORITIES+="$(generate_address_and_account_id "$1" '--scheme sr25519')\n"
	AUTHORITIES+="$(generate_address_and_account_id "$1" '--scheme sr25519' true)\n"
	AUTHORITIES+="),\n"
}

update_authorities "$PC_COLLATOR_01_SECRET"
update_authorities "$PC_COLLATOR_02_SECRET"
update_authorities "$PC_COLLATOR_03_SECRET"
update_authorities "$PC_COLLATOR_04_SECRET"
update_authorities "$PC_COLLATOR_05_SECRET"

printf "$AUTHORITIES"
