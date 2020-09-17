#!/usr/bin/env bash

LAST_KNOWN_WORKING_RUST_VERSION=nightly-2020-06-01

echo "*** Initializing WASM build environment"

rustup default stable
rustup uninstall nightly
rustup toolchain install $LAST_KNOWN_WORKING_RUST_VERSION
rustup target add wasm32-unknown-unknown --toolchain $LAST_KNOWN_WORKING_RUST_VERSION