#!/bin/bash

collections=("boards" "roles" "sessions" "users")
dbname="bridge_scorecard_api"

for collection in ${collections[@]}; do
  
  keys=`mongosh $dbname --eval "var keys = []; for(var key in db.$collection.findOne()) { keys.push(key); }; keys;" --quiet`;
  echo "collection: $collection - keys: $keys"
  echo "command: mongoexport --host localhost --db $collection --collection $collection --type=csv --out $collection.csv --fields \"$keys\";"
  mongoexport --host localhost --db $collection --collection $collection --type=csv --out $collection.csv --fields "$keys";
done;

