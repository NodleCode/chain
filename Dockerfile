FROM nodlecode/chain:2.2.2

WORKDIR /custom-wasm
ADD ./runtime_eden.compact.compressed.wasm /custom-wasm/runtime_eden.compact.compressed.wasm

ENTRYPOINT ["nodle-parachain"]
