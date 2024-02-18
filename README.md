# LlamaPredict
Prediction Market built on AlephZero

# What kind of problem we try to solve?

People likes to speculate. They have no such tool on Aleph Zero yet.

# Why prediction market? You can use lending protocol for shorting and DEX for longs.

There are a couple of problems with this approach:
1. Lending protocol relies very much on oracle. Right now, on Aleph Zero, we don't have a reliable oracle.
2. There are no tokens you could trade.
3. Buying tokens on DEX is very good strategy for longs, but you cannot leverage your position. You can only buy tokens for the amount of money you have.

# Good oracle, tokens and leverage will be available soon. Why do we need prediction market?

Prediction market can be deployed and be usable right now. If it is popular, it will create demand for tokens, what can help whole ecosystem to grow.

# Prediction market requires stablecoins. We don't have stablecoins on Aleph Zero.

Prediction market can actually be used to create stablecoins. If you have a market for AZERO/USD you can buy SHORT token from this market. If price of AZERO drops, your SHORT will be worth more and compensate the loss. If price of AZERO raises, your short is worth less.

# If it is so easy, why such stablecoins are not popular?

Prediction market based stablecoins are problematic, because they are stable just for a specific amount of time. This is perfectly fine for prediction market usecase, but in other cases you have to actively switch from token to token. You cannot just hold stablecoin and forget about it.

# What about leverage?

Leverage is also possible with prediction market. You can buy LONG token for market AZERO/USD. If the price of AZERO raises, your LONG token will be worth more and also AZERO tokens in which that market is back are worth more. 

# What about liquidity?

Each prediction market issues two tokens. Those are just regular tokens and can be traded on any DEX. If someone thinks actual prediction is true, she can mint tokens from the market, add liquidity and earn fees. If someone thinks actual prediction is false, she can also add liquidity, but also hold SHORT or LONG token, depending on the direction of prediction. 

# What do you mean by "mint tokens"?

For each market you can mint tokens. You deposit N underlying tokens (which can be anything) and you get N/2 LONG and N/2 SHORT tokens. Before market expires you can burn those tokens in M:M ratio and receive M+M underlying tokens. After oracle sets outcome of the market, you can also burn these tokens in ratio OUTCOME_A:OUTCOME_B and receive OUTCOME_A+OUTCOME_B underlying tokens.

# Oracle? You said we don't have reliable oracle.

Prediction market creator is also its oracle. However current prediction is determined by market. Users also have incentive not to wait for the market to expire, because they are loosing their collateral each second they are waiting for outcome after market is resolved.

# What is the risk?

You can easily verify how oracle voted in the past. Even if oracle is going to cheat, but the rest of market is honest, burning all the tokens before market is resolved is always safe. In other case, you can loose the most, if you disagree with the initial prediction.

# How prediction market creator is incentivized?

Prediction market creator provides initial liquidity and earns fees on DEXs. Also users waiting for the market to expire are loosing their collateral (few %). This is the main source of income for the creator.

# What is so exciting about prediction market?

You can use prediction market's tokens as underlying tokens for other prediction markets. This creates a lot of possibilities. You can introduce conditional probability, multi choice markets, options, futures, headging, etc. You can also use existing DEXs to trade those tokens, place limit orders, lend/borrow them, etc. You can also create your own centralized prediction market and use decentralized prediction markets for hedging.

# How it aligns with Aleph Zero's vision?

Each prediction market stores just hash of input. For users not knowing the input, it is just a random token. You can create private prediction markets, what aligns perfectly with Aleph Zero's vision of privacy. There are also a lot of math to do off chain.

# Is it production ready?

Unfortunately not yet. Collateral melting is not yet correct. There are also some things of burning by outcome, we have noticed during the hackathon and it was too late to change it.

# Any more questions?

We are prediction market enthusiasts and we are happy to answer any questions. 