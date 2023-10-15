# Account who we trust in order to execute money transfer tx.
export TRUSTED_ACCOUNT=$1
# Account of new tenant who want to create one.
export TENANT_ACCOUNT=$2
# Account who will receive a money.
export RECEIVER_ACCOUNT_EMAIL=$3
export RECEIVER_ACCOUNT=$4

near call $CONTRACT_ID add_tenant '{"email": ""}' --accountId $TENANT_ACCOUNT
export TENANT_EXECUTABLE_ID=$(near call $CONTRACT_ID add_tenant_executable '{"executable_counts": 2, "executable_amount": 2, "executable_recepient_account": "$RECEIVER_ACCOUNT", "executable_recepient_email": "$RECEIVER_ACCOUNT_EMAIL"}' --accountId $TENANT_ACCOUNT --amount 4)
near call $CONTRACT_ID trigger_tenant_executable '{"tenant_executable_id": "$TENANT_EXECUTABLE_ID"}' --accountId $TRUSTED_ACCOUNT