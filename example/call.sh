source .env

cargo contract call \
    --contract "5DBmUjxg5S7qTi9b6ck4eubWQfnA4YfpDDiN2MnDWVA3WjJw" \
    --message "add_market" --suri "$MNEMONIC" --url "$WSS_URL" target/ink/predictor/predictor.json \
    --args "5Fu7VErBYka2q8VEkGKxg7RU2RD1oaWddfMyw9EbzUrL5voF" \
    --args "0xdffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f" \
    --args 1708299002407 \
    --args 3600000 \
    --args 0 \

# cargo contract call \
#     --contract "5GMgVZrKNrHpDvSP5EiQZZnVXHsXtP345wSt3aideLLMqWwc" \
#     --message "mint" --suri "$MNEMONIC" --url "$WSS_URL" target/ink/predictor/predictor.json \
#     --args 0 \
#     --args 0 \
#     --args 9
