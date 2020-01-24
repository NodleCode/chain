#! /bin/sh

./load_keys.sh $AURA_SK $AURA_PK $GRANDPA_SK $GRANDPA_PK &

/usr/local/bin/nodle-chain --chain ./spec_raw.json $@