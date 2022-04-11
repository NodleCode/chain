# this script can be used to prepare a new release for this repo

SED_OPT=""
case "$(uname -s)" in
   Darwin)
        SED_OPT="-i ''"
     ;;
    Linux)
        SED_OPT="-i"
     ;;
esac

git pull

VERSION_STRING="version = \"$1\""
find . -name Cargo.toml -not -path "./target/*" -exec sed ${SED_OPT} -e "s/^version = .*/${VERSION_STRING}/" {} \;
cargo test --all --all-features

echo "Please verify script execution and press enter or ctrl-c"
read sanity_check

git add .
git commit -m "bump version for release"
git push

echo "Please wait for srtool github action to complete and enter the file name: "
read runtime_file

gh run download -n $runtime_file
gh release create $1 --title $1 --target master "$runtime_file/runtimes/eden/target/srtool/release/wbuild/runtime-eden/runtime_eden.compact.wasm" "$runtime_file/runtimes/eden/target/srtool/release/wbuild/runtime-eden/runtime_eden.compact.compressed.wasm" "$runtime_file/eden-srtool-digest.json"