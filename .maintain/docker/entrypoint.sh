#! /bin/sh

./load_keys.sh &

if [ $# -eq 0 ]; then
    /usr/local/bin/nodle-chain
else
	/usr/local/bin/nodle-chain $@
fi