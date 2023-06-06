#!/bin/bash

DATABASE_URL=`cat .env.test | grep DATABASE_URL | cut -d '=' -f 2,3`
if [[ -z $DATABASE_URL ]]; then
    echo "DATABASE_URL not found in .env.test"
    exit 1
fi
MIGRATION_PENDING=`diesel migration pending --database-url $DATABASE_URL`
PENDING="true"
if [[ "$MIGRATION_PENDING" == "$PENDING" ]]; then
    diesel migration run --database-url $DATABASE_URL
else
    diesel migration redo --database-url $DATABASE_URL --all
fi
cargo test -- --nocapture