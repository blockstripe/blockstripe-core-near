#!/bin/sh

CONTRACT_DIRECTORY=../smart-contracts
DEV_ACCOUNT_FILE=".env"

start () {
  echo The app is starting!
  npx parcel index.html --no-hmr --open
}

alert () {
  GREEN='\033[1;32m'
  NC='\033[0m' # No Color

  echo "======================================================"
  echo "It looks like you didn't deploy your contract"
  echo ">> Run ${GREEN}'npm run deploy'${NC} from the your project's root directory"
  echo "This frontend template works with contracts deployed to NEAR TestNet"
  echo "======================================================"
}

if [ -f "$DEV_ACCOUNT_FILE" ]; then
  start
else
  alert
fi
