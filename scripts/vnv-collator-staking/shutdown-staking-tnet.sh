#!/usr/bin/env bash

set -euxo pipefail

PROJECT_ROOT=$(git rev-parse --show-toplevel)

source "$(dirname "$0")"/init-env.sh

cd "$PROJECT_ROOT"

STAKER_NODE_PREFIX=${STAKER_NODE_PREFIX:-"nodle-staking-tnet"}

# Shutdown

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

rm -Rf $DATA_DIR
