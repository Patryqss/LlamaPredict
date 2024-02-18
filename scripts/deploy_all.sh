#!/bin/bash

source .env.testnet
SALT=$1
BASE=$PWD

cat > $BASE/.env.next <<EOF
WSS_URL="$WSS_URL"
MNEMONIC="$MNEMONIC"
ADMIN_ADDRESS="$ADMIN_ADDRESS"

EOF

function az () { 
    F=$1;
    shift;
    cargo contract $F \
        --skip-confirm --suri "$MNEMONIC" --url "$WSS_URL" "$@"; 
}

function gethash() {
    tail -n1 | grep  -o '0x[^"]*'
}
function getcontract() {
    tail -n1 | grep -o '[^ ]*$'
}

cd amm/contracts

cd pair
cargo contract build --release --offline
PAIR_HASH=`az upload -x 2>&1 | gethash`
cat >> $BASE/.env.next <<EOF
PAIR_HASH="$PAIR_HASH"
EOF

cd ../factory
cargo contract build --release --offline
AMM_FACTORY_ADDRESS=`az instantiate -x --salt $SALT --args $ADMIN_ADDRESS $PAIR_HASH | getcontract`
cat >> $BASE/.env.next <<EOF
AMM_FACTORY_ADDRESS="$AMM_FACTORY_ADDRESS"
EOF

cd ../router
cargo contract build --release --offline
ROUTER_ADDRESS=`az instantiate -x --salt $SALT --args $AMM_FACTORY_ADDRESS $ADMIN_ADDRESS | getcontract`
cat >> $BASE/.env.next <<EOF
ROUTER_ADDRESS="$ROUTER_ADDRESS"
EOF

cd ../../../contracts/minty_psp22
cargo contract build --release --offline
USD_ADDRESS=`az instantiate -x --salt $SALT --args "1000000000" --args "\"USD\"" --args "\"USD\"" --args "6" | getcontract`
cat >> $BASE/.env.next <<EOF
USD_ADDRESS="$USD_ADDRESS"
EOF

cd ../conditional_psp22
cargo contract build --release --offline
PSP22C_HASH=`az upload -x 2>&1 | gethash`
cat >> $BASE/.env.next <<EOF
PSP22C_HASH="$PSP22C_HASH"
EOF

cd ../predictor
cargo contract build --release --offline
PREDICTOR_ADDRESS=`az instantiate -x --args $PSP22C_HASH $ADMIN_ADDRESS | getcontract`
cat >> $BASE/.env.next <<EOF
PREDICTOR_ADDRESS="$PREDICTOR_ADDRESS"
EOF
