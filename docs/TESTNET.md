# Testnet runs

All on NEAR testnet, our own test accounts (`stakestreak-near-*.testnet`), no outside users.
Contract: `stakestreak-near.testnet`, deployed in
[4J1JwVygDz1Roc4eQmQjgF3E3XezgHxT4Sfwhr4CrQ6A](https://testnet.nearblocks.io/txns/4J1JwVygDz1Roc4eQmQjgF3E3XezgHxT4Sfwhr4CrQ6A).
Factory: `sputnikv2.testnet`. Produced by `scripts/testnet-pot.sh`.

## Pot 1: `streak-demo-1` (settled 2026-10-29)

0.5 NEAR stakes, 4 members, 3 days of 10 minutes, 3% fee. m1 and m2 checked in every day, m3 skipped
day 1, m4 skipped day 0. Treasury: `streak-demo-1.sputnikv2.testnet`
([account](https://testnet.nearblocks.io/address/streak-demo-1.sputnikv2.testnet)).

Result, read from the DAO: 3 Transfer proposals, all `Approved`, all proposed by the contract:
fee 0.06 NEAR, m1 0.97 NEAR, m2 0.97 NEAR (4 x 0.5 = 2.0 in; 0.06 + 0.97 + 0.97 = 2.0 out).
m3 and m4 received nothing. Balances read afterwards: m1 10.47 (10 - 0.5 + 0.97), m3 9.50, fee 10.06.

| step | transaction |
|---|---|
| create_pot | [6QcweaTL7P…](https://testnet.nearblocks.io/txns/6QcweaTL7Pw317mvmEA3aAfy4A89X93gh3YFW5ES4Vdu) |
| join stakestreak-near-m1.testnet | [4atJp5tVjR…](https://testnet.nearblocks.io/txns/4atJp5tVjR4Uy9BwSk9uQU3emGkcdCAXtu1mJBH4mGrZ) |
| join stakestreak-near-m2.testnet | [3QQaB4WnGd…](https://testnet.nearblocks.io/txns/3QQaB4WnGdbHpoP8aL1riakBascmzx8ZsDy4ist5txs1) |
| join stakestreak-near-m3.testnet | [4UpH62xwhF…](https://testnet.nearblocks.io/txns/4UpH62xwhFGSAdX95q2yF3tjAwLMQ4TfpF9Ed1mBYDDE) |
| join stakestreak-near-m4.testnet | [cKs2QpRaPw…](https://testnet.nearblocks.io/txns/cKs2QpRaPwACH3ZnrzJdMpPjAFiVASY1BjGC6NMtXiz) |
| check_in d0 stakestreak-near-m1.testnet | [9H5vFVkUgu…](https://testnet.nearblocks.io/txns/9H5vFVkUguTDQagrGJQa49VmZtRHnDMySCBDxenpAGJc) |
| check_in d0 stakestreak-near-m2.testnet | [AfPvuV8tcN…](https://testnet.nearblocks.io/txns/AfPvuV8tcNX58NXeXk3wDKaDZsycfLQWUPu3zdUoz1G1) |
| check_in d0 stakestreak-near-m3.testnet | [685B66cQLH…](https://testnet.nearblocks.io/txns/685B66cQLHLjeLjneAKXwcfo7KZbKMnkpbiaCSgKNVYu) |
| check_in d1 stakestreak-near-m1.testnet | [G1thAZdyKf…](https://testnet.nearblocks.io/txns/G1thAZdyKfT8iHJo3YPR2ZPzs1av16NZgd9x2y2eWqiL) |
| check_in d1 stakestreak-near-m2.testnet | [5jF12YPTWx…](https://testnet.nearblocks.io/txns/5jF12YPTWx1NYtWAXZijkNT71FrrTt9oLRU9JXov1qay) |
| check_in d1 stakestreak-near-m4.testnet | [8J5s5WZAxY…](https://testnet.nearblocks.io/txns/8J5s5WZAxY24EQeGUrucUq2DdRn7NDsMGYGS3AXiZhjn) |
| check_in d2 stakestreak-near-m1.testnet | [AHxHQy5WUH…](https://testnet.nearblocks.io/txns/AHxHQy5WUHNnrViZLkKY3x8KU23hXtFyeC63N5gtWopd) |
| check_in d2 stakestreak-near-m2.testnet | [55VEvFEPAU…](https://testnet.nearblocks.io/txns/55VEvFEPAUuVRfsEUAe1aseaZkTEATT85WhWew65WUFW) |
| check_in d2 stakestreak-near-m3.testnet | [6fs5nFtgdF…](https://testnet.nearblocks.io/txns/6fs5nFtgdFGAjGfPofpMSyxvJCWax695mjuSwe8YUsu3) |
| check_in d2 stakestreak-near-m4.testnet | [8CHaprh6xK…](https://testnet.nearblocks.io/txns/8CHaprh6xKDY5wu8q1214hnGTaZ1dytNZADk8pQ1K5yS) |
| settle (batch 3) #0 | [CCufRYoSEM…](https://testnet.nearblocks.io/txns/CCufRYoSEMwAJQVPRj9TWd8Gk16pQUdk9TGxwHgunFv6) |

The transaction pages were checked on nearblocks (see the report), not all of them; the transaction ids were printed by the CLI and the outcomes above were read from the contract and the DAO by view calls.

## Pot 2: `streak-demo-2` (wallet test, in progress)

0.5 NEAR stakes, 3 days of 10 minutes, 3% fee, joining open until 2026-10-29 18:50:44 UTC, minimum 3
members. Created by `stakestreak-near-fee.testnet`
([74nGZpqW…](https://testnet.nearblocks.io/txns/74nGZpqWdZqhspCV25p9BBFWt4nrKh5tRHSvDc6KuySf)); test accounts m1
([H7MpXDwY…](https://testnet.nearblocks.io/txns/H7MpXDwYwupSvMruF1y7MXmXnfiEhCNoAdwYpj1YYw2H)) and m2
([j4Dmryu1…](https://testnet.nearblocks.io/txns/j4Dmryu12NVpHsBkKKBNmJUrVKEf51ti32jxJZZ9XYu)) joined by CLI.
The remaining transactions are made through the web client with a separate wallet held by the
developer, through the web client. That is not an outside user.

The developer's separate wallet account was funded with 2 NEAR from test account m4
([GrJoSv4Y…](https://testnet.nearblocks.io/txns/GrJoSv4YWPeDpbciPcUy4ndLVE9u7SCgoKbKmDYPY44Q)).
Note: `rpc.testnet.near.org` started answering HTTP 429 "deprecated" on 2026-10-29; the client and the
fetch script use FastNEAR's testnet RPC (`rpc.testnet.fastnear.com`, then `test.rpc.fastnear.com`, then the archival RPC) instead.

### Wallet test (a separate wallet held by the developer, through the web client)

Not an outside user. The developer signed in with Meteor Wallet on testnet as `ochinimus.testnet`;
sign-in added a function-call key limited to `check_in` on `stakestreak-near.testnet` (allowance
0.25 NEAR), read from the account's key list. The first join attempts failed in the page for two
reasons found and fixed (browser could not reach `rpc.testnet.near.org`; Meteor rejects the
internal action shape, see docs/FEEDBACK.md items 8 and 9). Join, read from chain
(`tx` RPC, final): [6YjBhFEX…](https://testnet.nearblocks.io/txns/6YjBhFEXyFQwiXTC9Bpkpimq99zx8Cst5H3Y7jM22m75),
signer `ochinimus.testnet` (full-access key, wallet approval), method `join`, 0.5 NEAR, 50 Tgas,
`SuccessValue`. The pot then listed 3 funded members and the DAO held 7.08 NEAR (5.58 creation + 3 x 0.5).
