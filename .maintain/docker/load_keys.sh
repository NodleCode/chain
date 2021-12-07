#! /bin/sh

insert_key() {
  curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d \
    '{
      "jsonrpc":"2.0",
      "id":1,
      "method":"author_insertKey",
      "params": [
        "'$1'",
        "'$2'",
        "'$3'"
      ]
    }'
}

while ! nc -z localhost 9933; do   
  sleep 0.1 # wait for 1/10 of the second before check again
done

insert_key "aura" $AURA_SK $AURA_PK || echo "no aura keys"
insert_key "babe" $BABE_SK $BABE_PK || echo "no babe keys"
insert_key "gran" $GRANDPA_SK $GRANDPA_PK || echo "no grandpa keys"
insert_key "imon" $IM_ONLINE_SK $IM_ONLINE_PK || echo "no im_online keys"
insert_key "auth" $AUTHORITY_DISCOVERY_SK $AUTHORITY_DISCOVERY_PK || echo "no authority_discovery keys"