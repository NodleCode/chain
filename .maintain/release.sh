# this script can be used to prepare a new release for this repo

VERSION_STRING="version = \"$1\""

find . -name Cargo.toml -not -path "./target/*" -exec sed -i '' -e "s/^version = .*/${VERSION_STRING}/" {} \;

git pull
git add .
git commit -m "bump version for release"
git push

echo -n "Please wait for srtool github action to complete and enter the file name: "
read runtime_file

gh run download -n $runtime_file -n nodle-chain-srtool-digest.json
gh release create $1 --title $1 --target master 'nodle_chain_runtime.compact.wasm' 'nodle-chain-srtool-digest.json'