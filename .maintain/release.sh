# this script can be used to prepare a new release for this repo

VERSION_STRING="version = \"$1\""

find . -name Cargo.toml -not -path "./target/*" -exec sed -i '' -e "s/^version = .*/${VERSION_STRING}/" {} \;

git add .
git commit -m "bump version for release"
git push

cargo build --release

gh release create $1 --title $1 --target master 'target/release/wbuild/nodle-chain-runtime/nodle_chain_runtime.compact.wasm#Runtime Blob'