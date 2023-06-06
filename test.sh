#!/bin/bash

DATABASE_URL=`cat .env.test | grep DATABASE_URL | cut -d '=' -f 2,3`
if [[ -z $DATABASE_URL ]]; then
    echo "DATABASE_URL not found in .env.test"
    exit 1
fi
cargo test -- --nocapture