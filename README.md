No liquidations synthetic assets (stable coins) designed for the (3, 3) model

The (3, 3) based rebase sToken can be used to achieve increasing value, so it is a good choice for achieving unsecured synthetic assets and have never liquidation.

The use of overcollateralization provides a degree of systemic safety, with prices gradually repairing over time when synthetic assets fall below the anchor price. At the same time, using lower prices to redeem assets will help the price of synthetic assets to recover.

So in essence, this is a way to go long (3,3) sToken.

In this example, the mock code is used to anchor the sLSRV to the UST.

The following is an explanation of the mint and redeem functions.

- Mint: When mint synthetic assets(LUST),  will first call the oracel machine to get the price of wsLSRV/UST. Then mint LUST for user with deposit ratio in 200%
- Redeem: Burn user's issued LUST and transfer deposited wsLSRV to user. Of course, it can be adjusted according to the actual circumstances, for example, when the deposited ratio is below 100%, a penalty is charged as a fee

> Note: This repo is written in cosmwasm and is for theoretical simulation only, not for production and normal operation. 