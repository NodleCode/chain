#!/bin/bash

# Steps and Repats are optional command line argument in $1 and $2
STEPS="${1:-50}"
REPEAT="${2:-20}"

export xcm_generic_extrinsic="report_holding, buy_execution, query_response, transact, refund_surplus,\
 set_error_handler, set_appendix, clear_error, descend_origin, clear_origin, report_error, claim_asset, trap, \
 subscribe_version, unsubscribe_version, initiate_reserve_withdraw, burn_asset, expect_asset, expect_origin,\
 expect_error, expect_transact_status, query_pallet, expect_pallet, report_transact_status,\
 clear_transact_status, set_topic, clear_topic, set_fees_mode, unpaid_execution"

cargo build --profile release \
    --features=runtime-benchmarks \
    --manifest-path=node/Cargo.toml || exit -1


for PALLET in `./target/release/nodle-parachain benchmark pallet --list| sed s/,.*//|sort|uniq|grep -v ::`
do
 echo $PALLET
 $DRY_RUN ./target/release/nodle-parachain benchmark pallet \
    --pallet=$PALLET \
    '--extrinsic=*' \
    --steps=$STEPS \
    --repeat=$REPEAT \
    --genesis-builder=runtime \
    --runtime="./target/release/wbuild/runtime-eden/runtime_eden.wasm" \
    --wasm-execution=compiled \
    --template=./.maintain/external_pallet_weights.hbs \
    --output=runtimes/eden/src/weights
done

$DRY_RUN ./target/release/nodle-parachain benchmark pallet \
    --pallet=pallet_xcm_benchmarks::fungible \
    '--extrinsic=*' \
    --steps=$STEPS \
    --repeat=$REPEAT \
    --genesis-builder=runtime \
    --runtime="./target/release/wbuild/runtime-eden/runtime_eden.wasm" \
    --wasm-execution=compiled \
    --template=./.maintain/xcm.hbs \
    --output=runtimes/eden/src/weights

$DRY_RUN ./target/release/nodle-parachain benchmark pallet \
    --pallet=pallet_xcm_benchmarks::generic \
    --extrinsic="$xcm_generic_extrinsic" \
    --steps=$STEPS \
    --repeat=$REPEAT \
    --genesis-builder=runtime \
    --runtime="./target/release/wbuild/runtime-eden/runtime_eden.wasm" \
    --wasm-execution=compiled \
    --template=./.maintain/xcm.hbs \
    --output=runtimes/eden/src/weights
$DRY_RUN sed -s 's/pallet_contracts::WeightInfo/pallet_contracts::weights::WeightInfo/' -i runtimes/eden/src/weights/pallet_contracts.rs

$DRY_RUN cargo clippy --fix --allow-dirty
$DRY_RUN cargo fmt

echo "Running on gcloud server? Run:"
echo "git commit -v -a -m Benchmarks ; git format-patch HEAD~ ; find $PWD/*patch"
echo "Download to dev machine and apply with:"
echo "    git apply " 0001*

