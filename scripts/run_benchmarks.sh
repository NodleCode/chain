#!/bin/bash

cargo run --profile release \
    --features=runtime-benchmarks \
    --manifest-path=node/Cargo.toml \
    -- \
    benchmark pallet \
    --chain=dev \
    --steps=50 \
    --repeat=20 \
    '--pallet=*' \
    '--extrinsic=*' \
    --execution=wasm \
    --wasm-execution=compiled \
    --template=./.maintain/frame-weight-template.hbs \
    --output=runtimes/eden/src/weights


mv runtimes/eden/src/weights/pallet_allocations.rs pallets/allocations/src/weights.rs
mv runtimes/eden/src/weights/pallet_grants.rs pallets/grants/src/weights.rs 
mv runtimes/eden/src/weights/pallet_reserve.rs pallets/reserve/src/weights.rs
