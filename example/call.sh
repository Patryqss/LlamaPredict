source .env

cargo contract call \
    --contract "5F3KVgLa6mXTLeYUFyQ9dgvBbFAEwqpnRSYjPVDLX1ZKJniR" \
    --message "add_market" --suri "$MNEMONIC" --url "$WSS_URL" target/ink/predictor/predictor.json \
    --args "5Fu7VErBYka2q8VEkGKxg7RU2RD1oaWddfMyw9EbzUrL5voF" \
    --args "0xdffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f" \
    --args 1708199002407 \
    --args 3600000 \
    --args 0
