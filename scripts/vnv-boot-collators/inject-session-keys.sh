#!/usr/bin/env bash

set -euxo pipefail

PROJECT_ROOT=$(git rev-parse --show-toplevel)

source "$(dirname "$0")"/init-env.sh

cd "$PROJECT_ROOT"

generate_account_id() {
    docker \
		run \
		--rm \
		$COLLATOR_IMAGE \
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

inject_keys() {
    curl \
		$2 \
		-H "Content-Type:application/json;charset=utf-8" \
		-d "$(generate_author_insertKey_with_account_id "$1" '--scheme sr25519' aura)"
}


inject_keys "$COLLATOR1_OVERALL_SECRET" $COLLATOR1_RPC_HTTP_ENDPOINT
inject_keys "$COLLATOR2_OVERALL_SECRET" $COLLATOR2_RPC_HTTP_ENDPOINT
inject_keys "$COLLATOR3_OVERALL_SECRET" $COLLATOR3_RPC_HTTP_ENDPOINT
inject_keys "$COLLATOR4_OVERALL_SECRET" $COLLATOR4_RPC_HTTP_ENDPOINT
inject_keys "$COLLATOR5_OVERALL_SECRET" $COLLATOR5_RPC_HTTP_ENDPOINT