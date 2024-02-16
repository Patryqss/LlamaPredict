#!/bin/bash

source .env

function az () { 
    F=$1;
    shift;
    cargo contract $F \
        --skip-confirm --suri "$MNEMONIC" --url "$WSS_URL" "$@"; 
}

CONTRACT="$1"
shift

az instantiate "target/ink/$CONTRACT/$CONTRACT.contract" $@
