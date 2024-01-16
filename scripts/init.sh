#!/usr/bin/env bash

set -e

echo "*** Initializing WASM build environment"

if [ -z $CI_PROJECT_NAME ] ; then
   rustup update stable
fi

rustup component add rust-src
rustup target add wasm32-unknown-unknown --toolchain stable
