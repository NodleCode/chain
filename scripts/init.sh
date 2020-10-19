#!/usr/bin/env bash

set -e

echo "*** Initializing WASM build environment"

if [ -z $CI_PROJECT_NAME ] ; then
   rustup update nightly
   rustup update stable
fi

rustup toolchain install `cat rust-toolchain`
rustup target add wasm32-unknown-unknown --toolchain `cat rust-toolchain`