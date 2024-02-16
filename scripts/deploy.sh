#!/bin/bash

source .env

function az () { 
    F=$1;
    shift;
    cargo contract $F \
        --skip-confirm --suri "$MNEMONIC" --url "$WSS_URL" "$@"; 
}

CONTRACT="$1"

CODE_HASH=$(az upload "target/ink/$CONTRACT/$CONTRACT.contract" | grep "Code hash" | sed -n 's/.*Code hash "//p' | sed -n 's/"//p')

echo "$CODE_HASH"
