#! /bin/sh

./load_keys.sh &

if [ $# -eq 0 ]; then
    /usr/local/bin/nodle-chain --chain arcadia.json
else
	/usr/local/bin/nodle-chain $@
fi