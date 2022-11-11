#!/bin/bash

export external="frame_system pallet  pallet_balances pallet_collator_selection pallet_contracts  pallet_membership pallet_multisig pallet_preimage  pallet_scheduler pallet_timestamp pallet_uniques pallet_utility"
export internal="pallet_allocations pallet_grants pallet_reserve"
cargo build --profile release \
    --features=runtime-benchmarks \
    --manifest-path=node/Cargo.toml 

install -d weights
for PALLET in $internal
do
./target/release/nodle-parachain    benchmark pallet \
    --chain=dev \
    --steps=50 \
    --repeat=20 \
    --pallet=$PALLET \
    '--extrinsic=*' \
    --execution=wasm \
    --wasm-execution=compiled \
    --template=./.maintain/internal_pallet_weights.hbs \
    --output=weights
done

mv weights/pallet_allocations.rs pallets/allocations/src/weights.rs
mv weights/pallet_grants.rs pallets/grants/src/weights.rs 
mv weights/pallet_reserve.rs pallets/reserve/src/weights.rs

exit

for PALLET in $external
do
echo ./target/release/nodle-parachain    benchmark pallet \
    benchmark pallet \
    --chain=dev \
    --steps=50 \
    --repeat=20 \
    --pallet=$PALLET \
    '--extrinsic=*' \
    --execution=wasm \
    --wasm-execution=compiled \
    --template=./.maintain/external_pallet_weights.hbs \
    --output=runtimes/eden/src/weights

done
exit



echo "Running on gcloud server? Run:"
echo "    git commit -v -a -m Benchmarks ; git format-patch HEAD~"
echo "And on dev machine:"
echo "    gcloud compute scp chain-bench-012bd056:chain/0001\* . --zone=us-central1-a --tunnel-through-iap "
echo "    git apply 0001*"

