# this script can be used to prepare a new release for this repo

git pull

VERSION_STRING="version = \"$1\""
find . -name Cargo.toml -not -path "./target/*" -exec sed -i '' -e "s/^version = .*/${VERSION_STRING}/" {} \;
cargo test --all --all-features

git add .
git commit -m "bump version for release"
git push

echo "Please wait for srtool github action to complete and enter the file name: "
read runtime_file

gh run download -n $runtime_file -n nodle-chain-srtool-digest.json
gh release create $1 --title $1 --target master "$runtime_file/nodle_chain_runtime.compact.wasm" "nodle-chain-srtool-digest.json/nodle-chain-srtool-digest.json"
