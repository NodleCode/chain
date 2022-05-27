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