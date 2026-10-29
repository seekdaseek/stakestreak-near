# Testnet runs

All on NEAR testnet, our own test accounts (`stakestreak-near-*.testnet`), no outside users.
Contract: `stakestreak-near.testnet`, deployed in
[4J1JwVygDz1Roc4eQmQjgF3E3XezgHxT4Sfwhr4CrQ6A](https://explorer.testnet.near.org/transactions/4J1JwVygDz1Roc4eQmQjgF3E3XezgHxT4Sfwhr4CrQ6A).
Factory: `sputnikv2.testnet`. Produced by `scripts/testnet-pot.sh`.

## Pot 1: `streak-demo-1` (settled 2026-10-29)

0.5 NEAR stakes, 4 members, 3 days of 10 minutes, 3% fee. m1 and m2 checked in every day, m3 skipped
day 1, m4 skipped day 0. Treasury: `streak-demo-1.sputnikv2.testnet`
([account](https://explorer.testnet.near.org/accounts/streak-demo-1.sputnikv2.testnet)).

Result, read from the DAO: 3 Transfer proposals, all `Approved`, all proposed by the contract:
fee 0.06 NEAR, m1 0.97 NEAR, m2 0.97 NEAR (4 x 0.5 = 2.0 in; 0.06 + 0.97 + 0.97 = 2.0 out).
m3 and m4 received nothing. Balances read afterwards: m1 10.47 (10 - 0.5 + 0.97), m3 9.50, fee 10.06.

| step | transaction |
|---|---|
| create_pot | [6QcweaTL7P…](https://explorer.testnet.near.org/transactions/6QcweaTL7Pw317mvmEA3aAfy4A89X93gh3YFW5ES4Vdu) |
| join stakestreak-near-m1.testnet | [4atJp5tVjR…](https://explorer.testnet.near.org/transactions/4atJp5tVjR4Uy9BwSk9uQU3emGkcdCAXtu1mJBH4mGrZ) |
| join stakestreak-near-m2.testnet | [3QQaB4WnGd…](https://explorer.testnet.near.org/transactions/3QQaB4WnGdbHpoP8aL1riakBascmzx8ZsDy4ist5txs1) |
| join stakestreak-near-m3.testnet | [4UpH62xwhF…](https://explorer.testnet.near.org/transactions/4UpH62xwhFGSAdX95q2yF3tjAwLMQ4TfpF9Ed1mBYDDE) |
| join stakestreak-near-m4.testnet | [cKs2QpRaPw…](https://explorer.testnet.near.org/transactions/cKs2QpRaPwACH3ZnrzJdMpPjAFiVASY1BjGC6NMtXiz) |
| check_in d0 stakestreak-near-m1.testnet | [9H5vFVkUgu…](https://explorer.testnet.near.org/transactions/9H5vFVkUguTDQagrGJQa49VmZtRHnDMySCBDxenpAGJc) |
| check_in d0 stakestreak-near-m2.testnet | [AfPvuV8tcN…](https://explorer.testnet.near.org/transactions/AfPvuV8tcNX58NXeXk3wDKaDZsycfLQWUPu3zdUoz1G1) |
| check_in d0 stakestreak-near-m3.testnet | [685B66cQLH…](https://explorer.testnet.near.org/transactions/685B66cQLHLjeLjneAKXwcfo7KZbKMnkpbiaCSgKNVYu) |
| check_in d1 stakestreak-near-m1.testnet | [G1thAZdyKf…](https://explorer.testnet.near.org/transactions/G1thAZdyKfT8iHJo3YPR2ZPzs1av16NZgd9x2y2eWqiL) |
| check_in d1 stakestreak-near-m2.testnet | [5jF12YPTWx…](https://explorer.testnet.near.org/transactions/5jF12YPTWx1NYtWAXZijkNT71FrrTt9oLRU9JXov1qay) |
| check_in d1 stakestreak-near-m4.testnet | [8J5s5WZAxY…](https://explorer.testnet.near.org/transactions/8J5s5WZAxY24EQeGUrucUq2DdRn7NDsMGYGS3AXiZhjn) |
| check_in d2 stakestreak-near-m1.testnet | [AHxHQy5WUH…](https://explorer.testnet.near.org/transactions/AHxHQy5WUHNnrViZLkKY3x8KU23hXtFyeC63N5gtWopd) |
| check_in d2 stakestreak-near-m2.testnet | [55VEvFEPAU…](https://explorer.testnet.near.org/transactions/55VEvFEPAUuVRfsEUAe1aseaZkTEATT85WhWew65WUFW) |
| check_in d2 stakestreak-near-m3.testnet | [6fs5nFtgdF…](https://explorer.testnet.near.org/transactions/6fs5nFtgdFGAjGfPofpMSyxvJCWax695mjuSwe8YUsu3) |
| check_in d2 stakestreak-near-m4.testnet | [8CHaprh6xK…](https://explorer.testnet.near.org/transactions/8CHaprh6xKDY5wu8q1214hnGTaZ1dytNZADk8pQ1K5yS) |
| settle (batch 3) #0 | [CCufRYoSEM…](https://explorer.testnet.near.org/transactions/CCufRYoSEMwAJQVPRj9TWd8Gk16pQUdk9TGxwHgunFv6) |

The explorer pages were not opened and read; the transaction ids were printed by the CLI and the outcomes above were read from the contract and the DAO by view calls.
