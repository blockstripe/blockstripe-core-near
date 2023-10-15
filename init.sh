export CONTRACT_ID=$1
export TRUSTED_ACCOUNT=$2

near call $CONTRACT_ID new '{"trusted_account": "$TRUSTED_ACCOUNT"}' --accountId $TRUSTED_ACCOUNT