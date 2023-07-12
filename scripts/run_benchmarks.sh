#!/bin/bash

# Steps and Repats are optional command line argument in $1 and $2
STEPS="${1:-50}"
REPEAT="${2:-20}"

export external="frame_system pallet_balances pallet_collator_selection pallet_contracts pallet_membership\
 pallet_multisig pallet_preimage pallet_scheduler pallet_timestamp pallet_uniques pallet_utility pallet_xcm"
export internal="pallet_allocations pallet_grants pallet_reserve pallet_nodle_uniques"
export xcm_generic_extrinsic="report_holding, buy_execution, query_response, transact, refund_surplus,\
 set_error_handler, set_appendix, clear_error, descend_origin, clear_origin, report_error, claim_asset, trap, \
 subscribe_version, unsubscribe_version, initiate_reserve_withdraw, burn_asset, expect_asset, expect_origin,\
 expect_error, expect_transact_status, query_pallet, expect_pallet, report_transact_status,\
 clear_transact_status, set_topic, clear_topic, set_fees_mode, unpaid_execution"

cargo build --profile release \
    --features=runtime-benchmarks \
    --manifest-path=node/Cargo.toml 

install -d temp_weights
for PALLET in $internal
do
./target/release/nodle-parachain benchmark pallet \
    --chain=dev \
    --steps=$STEPS \
    --repeat=$REPEAT \
    --pallet=$PALLET \
    '--extrinsic=*' \
    --execution=wasm \
    --wasm-execution=compiled \
    --template=./.maintain/internal_pallet_weights.hbs \
    --output=temp_weights
done

for PALLET in $external
do
./target/release/nodle-parachain benchmark pallet \
    --chain=dev \
    --steps=$STEPS \
    --repeat=$REPEAT \
    --pallet=$PALLET \
    '--extrinsic=*' \
    --execution=wasm \
    --wasm-execution=compiled \
    --template=./.maintain/external_pallet_weights.hbs \
    --output=runtimes/eden/src/weights

done

./target/release/nodle-parachain benchmark pallet \
    --chain=dev \
    --steps=$STEPS \
    --repeat=$REPEAT \
    --pallet=pallet_xcm_benchmarks::fungible \
    '--extrinsic=*' \
    --execution=wasm \
    --wasm-execution=compiled \
    --template=./.maintain/xcm.hbs \
    --output=runtimes/eden/src/weights

./target/release/nodle-parachain benchmark pallet \
    --chain=dev \
    --steps=$STEPS \
    --repeat=$REPEAT \
    --pallet=pallet_xcm_benchmarks::generic \
    --extrinsic="$xcm_generic_extrinsic" \
    --execution=wasm \
    --wasm-execution=compiled \
    --template=./.maintain/xcm.hbs \
    --output=runtimes/eden/src/weights
sed -s 's/pallet_contracts::WeightInfo/pallet_contracts::weights::WeightInfo/' -i runtimes/eden/src/weights/pallet_contracts.rs

mv temp_weights/pallet_grants.rs pallets/grants/src/weights.rs
mv temp_weights/pallet_allocations.rs pallets/allocations/src/weights.rs
mv temp_weights/pallet_reserve.rs pallets/reserve/src/weights.rs
mv temp_weights/pallet_nodle_uniques.rs pallets/uniques/src/weights.rs

cargo clippy --fix --allow-dirty
cargo fmt

echo "Running on gcloud server? Run:"
echo " git config --global user.email \$USER ; git config --global user.name \$USER  git commit -v -a -m Benchmarks ; git format-patch HEAD~ ; find $PWD/*patch"
echo "Download to dev machine and apply with:"
echo "    git apply 0001*"

