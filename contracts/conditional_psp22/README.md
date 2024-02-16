# Conditional Token (PSP22)
This is a simple PSP22 token with 2 modifications and roles.
1. The creator (manager) can use `mint_to` and `burn_from` contract methods.
2. The AMM router can freely call `transfer_from` on any account owning these tokens.
