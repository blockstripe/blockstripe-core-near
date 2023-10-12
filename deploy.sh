#!/bin/sh

./build.sh

if [ $? -ne 0 ]; then
  echo ">> Error building contract"
  exit 1
fi

echo ">> Deploying contract"

export CONTRACT_ID=$1
export TRUSTED_ACCOUNT=$2

# https://docs.near.org/tools/near-cli#near-dev-deploy
near dev-deploy --wasmFile ./target/wasm32-unknown-unknown/release/blockstripe.wasm
near call $CONTRACT_ID new '{"trusted_account": "$TRUSTED_ACCOUNT"}' --accountId $TRUSTED_ACCOUNT