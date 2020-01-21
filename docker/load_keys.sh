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

insert_key "aura" $1 $2
insert_key "gran" $3 $4
