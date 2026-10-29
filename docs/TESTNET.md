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

Not an outside user. The developer signed in with Meteor Wallet on testnet as the developer's wallet account;
sign-in added a function-call key limited to `check_in` on `stakestreak-near.testnet` (allowance
0.25 NEAR), read from the account's key list. The first join attempts failed in the page for two
reasons found and fixed (browser could not reach `rpc.testnet.near.org`; Meteor rejects the
internal action shape, see docs/FEEDBACK.md items 8 and 9). Join, read from chain
(`tx` RPC, final): [6YjBhFEX…](https://testnet.nearblocks.io/txns/6YjBhFEXyFQwiXTC9Bpkpimq99zx8Cst5H3Y7jM22m75),
signer: the developer's wallet account, (full-access key, wallet approval), method `join`, 0.5 NEAR, 50 Tgas,
`SuccessValue`. The pot then listed 3 funded members and the DAO held 7.08 NEAR (5.58 creation + 3 x 0.5).

### streak-demo-2 run (2026-10-29), result: nobody survived, the whole pot went to the fee account (old rule)

Pot clock: start 18:50:44 UTC, days of 10 minutes, end 19:20:44 UTC. Test accounts m1 and m2 were driven
by a script; the developer's wallet account the developer's wallet account by hand in the browser, through the web client.

| Window | Who | Result |
|---|---|---|
| 1 (18:50:44-19:00:44) | m1 | check-in ok ([9AsEMHT5…](https://testnet.nearblocks.io/txns/9AsEMHT5CCvNiVH48QWea48FCDMbobUUkkSLWcq8f3Yr)) |
| 1 | m2 | check-in ok ([2uNNKQXk…](https://testnet.nearblocks.io/txns/2uNNKQXkvojei1hjzEpC6ZAaFPycCD5JgXqgom2oE198)) |
| 1 | developer's wallet account | check-in ok at 18:51:41 ([3moLRnQz…](https://testnet.nearblocks.io/txns/3moLRnQzPgrotkimCddpfKubWRXx5iZQzPTXcmaSEGry)) |
| 2 (19:00:44-19:10:44) | m1 | **no transaction reached the chain** (script call at 19:01:14 returned no tx id; scan of every block from about 19:00:34 to 19:02:04 shows no m1 transaction) |
| 2 | m2 | skipped on purpose (out) |
| 2 | developer's wallet account | no check-in on chain |
| 3 (19:10:44-19:20:44) | m1 | check-in ok ([AnMm9Sx4…](https://testnet.nearblocks.io/txns/AnMm9Sx43NbceDRxrkLPHCLmyPAXi1Sh6ikPXcDzrVvp)), but too late: day 1 was already missed |
| 3 | developer's wallet account | no check-in on chain |

Read back from the contract after the end (`get_pot`): day bitmasks m1 `5` (days 0 and 2), m2 `1`, developer wallet `1`;
all three `alive: false`. The contract did what its rules say for "no survivors": one Transfer proposal,
id 0, 1.5 NEAR (the whole pot, no 3% cut) to `stakestreak-near-fee.testnet`, status `Approved`,
settled by m4 in [4L45icTR…](https://testnet.nearblocks.io/txns/4L45icTRNLwZgAfqeFXWYjpRNvddsbJhrtG26p2g3Roy).
Balances after: pot DAO 5.58 NEAR (the creation storage only), fee account 5.944 NEAR. Nobody was paid out as a winner. The
planned result (the developer's wallet account and m1 paid, m2 nothing, fee 0.045) did **not** happen.

**Why:** m1's window-2 check-in never reached the chain (cause unknown: the script did not keep the CLI's error text; likely the CLI's RPC). The
developer's account has one check-in on chain, for window 1.

**Does Meteor ask for approval on a check-in?** For the one check-in on chain, no. The transaction was signed by the
function-call key `CwaSRxWJwbcn…` added at sign-in (receiver `stakestreak-near.testnet`, method `check_in`, no deposit,
30 Tgas), not by the full-access key, read from the `tx` RPC (`public_key`). A silent check-in is therefore proven for window 1
only; windows 2 and 3 were not exercised through the wallet.

### Contract redeployed with the new no-survivor rule (2026-10-29)

The sweep above was the contract's rule at the time. It is replaced: when nobody survives, every funded
member gets the stake back minus the fee (3% of the stake), and the fees go to the fee account as one Transfer. Tests:
`no_survivors_refunds_stake_minus_fee` (refund path), `full_pot_pays_survivors_by_policy` (survivors split), unit test
`refund_split_adds_up_to_one_stake`; the full suite ran green (15 tests) after the change.
Redeploy to `stakestreak-near.testnet`: [GdUjQwvu…](https://testnet.nearblocks.io/txns/GdUjQwvuYmEqB5G2vjb2ybPqoJNgjbeFG4Lw5axBDZrm).
On-chain code sha256 read back from the node equals the local build (`97651042…b69d9`), and `get_pot` still reads
`streak-demo-1` and `streak-demo-2` (state layout unchanged). The new rule has **not yet run on testnet**; the recorded demo pot will show it.

## The control on testnet: `streak-control-1` (2026-10-29)

The same three-friend pot as a plain Sputnik multisig: council of three, 2 votes needed, **no companion contract**
([create, 5.6 NEAR attached](https://testnet.nearblocks.io/txns/Buju1HcxGVyxpn2yqCt9AQhXbg6zeK2CB4KybQa9vksv); DAO
[`streak-control-1.sputnikv2.testnet`](https://testnet.nearblocks.io/address/streak-control-1.sputnikv2.testnet)). Made by
`scripts/testnet-control.sh`. Each member sent 0.5 NEAR to the treasury:
[m1](https://testnet.nearblocks.io/txns/G714So4jCg3crFCNNU5fo7Cr2GQp6Cye3HHuUnZJcdkF),
[m2](https://testnet.nearblocks.io/txns/ByKikqeEUu4fsY5LCqrunsT1RD7Q2DR2GMQ6tG2MoXGW),
[m4](https://testnet.nearblocks.io/txns/BqDy3T86tixs6Dyd5DtuXV46BD6qt5h3QVn4GBLYXmr1).

| Step | Result |
|---|---|
| m1 is the "survivor": [add_proposal](https://testnet.nearblocks.io/txns/2cFomKwgjyxTgS4Y455Lar68Mm576HDpDhP6kfk9Yy84) (1.5 NEAR to m1) and [votes yes](https://testnet.nearblocks.io/txns/HVWH1zKTuxyurhaaiC4cLSSPhNhVhj2uKPf9tCrYyyZR) | proposal 0: 1 of 2 votes |
| m2 and m4 (the "losers") do nothing for 180 s | status `InProgress`, treasury still 7.1 NEAR (5.6 creation + 1.5 stakes), the survivor has nothing |
| Control check: [m2 votes yes](https://testnet.nearblocks.io/txns/FPPRvPLz7QgAoZMa278rdLP3yJHNLLz6vh58M8koQzhi) | status `Approved`, treasury 5.6 NEAR, m1 received the 1.5 NEAR (balance read back) |

So the multisig works only when a loser cooperates; the StakeStreak pot pays by rule, with nobody voting
(`streak-demo-1`). The first create attempt with 5.4 NEAR attached was refused by the node
("remaining balance will not be enough to cover storage", the DAO account itself needs about 5.41 NEAR for its
copy of the code); 5.6 worked. Both are in the runner log, not on chain (no transaction was sent).

### Contract redeployed again: create-deposit minimum (2026-10-29)

`MIN_DAO_DEPOSIT` raised from 5.4 to 5.5 NEAR (so `create_pot` needs at least 5.52 attached), derived in
[DESIGN.md](DESIGN.md#cost-of-one-pot-dao-the-number-that-looks-bad). Tests: `one_yocto_under_the_minimum_is_refused`,
`exactly_the_minimum_creates_and_settles`, `min_dao_deposit_covers_the_worst_case_dao`; the full suite ran once, 18 passed.
Redeploy: [6sPPjEtT…](https://testnet.nearblocks.io/txns/6sPPjEtTzMYvxGH821k5HpXEAMM2fZecUKPrCoRwugyD); on-chain code
sha256 `da36de71…23ab` equals the local build (read back from the node); `streak-demo-1` and `streak-demo-2` still read.

Also found while running the suite: one test run hung for 16 minutes at 0% CPU inside `cargo near build`'s ABI step
(macOS `dyld` loading the freshly built dylib). Nothing in the contract was involved. The test harness now skips the ABI step
and builds the contract once per test binary; the deployed wasm is still built with its ABI.

## Pot 3: `streak-demo-3`, a leave and a freeze (2026-10-29), run only by scripts on the server

No wallet, no laptop: `scripts/testnet-freeze-leave.sh` ran on a server with the retrying sender (`scripts/send-retry.sh`).
0.5 NEAR stakes, 3 days of 2 minutes, 3% fee, minimum 3 members, DAO
[`streak-demo-3.sputnikv2.testnet`](https://testnet.nearblocks.io/address/streak-demo-3.sputnikv2.testnet).
Created by m1 with 5.6 NEAR ([7jP47qDS…](https://testnet.nearblocks.io/txns/7jP47qDS7GmLhjFyEPE9vDm2aYt1kshy3HaXGDZgpvp1)).

| Step | Transaction |
|---|---|
| m1, m2, m3, m4 join | [m1](https://testnet.nearblocks.io/txns/6iS5qwwQWVaS5poANjB5TfXYUF1eunZ3MGgiZSxwdoiz), [m2](https://testnet.nearblocks.io/txns/2QEfGPuCj8K5VYjPmCrPQd2Zhzx7ByvULNereYStkR53), [m3](https://testnet.nearblocks.io/txns/8jbebmoJPjLZRzV3jzGx2p8FPvaqBTTim8eNJRrVGCUA), [m4](https://testnet.nearblocks.io/txns/3uutayW3NtB5yQco5kRaxpMxcVzNJDNC1hfa91Sxjrjc) |
| **m4 leaves before the start** (refund of the full stake, no fee) | [CakTium…](https://testnet.nearblocks.io/txns/CakTiumQxNv9AdxwqQivynrqAF7o8ejh2h3xmZTzh8pD) |
| Day 0: m1, m2, m3 check in | [m1](https://testnet.nearblocks.io/txns/4DzpqyLvaYk7b3B2MpZDHUWrk8cFAGuktK3ioHBh4N7X), [m2](https://testnet.nearblocks.io/txns/BCppiJ5JBUrY86rp8JoVHobuRr5WygLFiAnEfhQFKmHw), [m3](https://testnet.nearblocks.io/txns/BWNBZa31WyUMezLfYpzN3djy5DUBvVuL8WbFTGtJ2T1A) |
| Day 1: m1, m2 check in, **m3 does not** | [m1](https://testnet.nearblocks.io/txns/J7k68ELTcEhrxxkDRfjkwHoJjME9CVTbANyHiFZYzqmT), [m2](https://testnet.nearblocks.io/txns/6sGNzrpzZ8kb4eXr8qxjXdc6mF9WkUSp5K5m98EwZ1bm) |
| Day 2: **m3 buys a freeze** (0.05 NEAR, covers day 1) | [EmHQyfmt…](https://testnet.nearblocks.io/txns/EmHQyfmtniZWucMqjLrAMV1Hz3hQtdxYW3JHVtFMrKAU) |
| Day 2: m1, m2, m3 check in | [m1](https://testnet.nearblocks.io/txns/DeuiXBpThLqi397hdX5ZvWrqYDeVHXxcaxVJXwVSUeU9), [m2](https://testnet.nearblocks.io/txns/9zZES4pBcDN82zCSi32cU2sZP4YfaCdgYfAQyCd7XM9R), [m3](https://testnet.nearblocks.io/txns/99Ezp6DM1ABqtQLf1QWmnKr933n17bSHo7JZw8yuWvXt) |
| Settle (two calls, batch 3) | [AwgNR1c8…](https://testnet.nearblocks.io/txns/AwgNR1c81aqFG1p7kQgmPhgJscyz1tyRGtQz3bhwzFy8), [7kVLFv2Z…](https://testnet.nearblocks.io/txns/7kVLFv2Z9cbt6T4urw5G7DSEeXRaLdKNpZ8Dh2aUMoS) |

Read back by view calls after the end: `get_pot` says `settled`, 0 pending; m1, m2, m3 `alive` with day mask `7`, m3 with 1
freeze used. Five Transfer proposals, all `Approved`, all proposed by the contract: 0.5 NEAR to m4 (the leave refund), 0.045
to the fee account (3% of 1.5), 0.485 to each of m1, m2, m3 ((1.5 - 0.045) / 3). The fee account's balance rose by 0.095 NEAR
over the pot: 0.045 fee plus the 0.05 freeze fee, which goes straight to it. The DAO is left at its 5.58 NEAR creation deposit.

**What went wrong, kept in the record:** the script sent `leave` with 60 Tgas and the node refused it ("Exceeded the prepaid gas",
no transaction made; the retry helper tried three times, as designed, and gave up: "give-ups: 1"). `leave` files and approves
the refund proposal in the DAO and needs about 150 Tgas. I re-sent it by hand from the same server with 150 Tgas, 70 seconds
before the start, which is the transaction in the table; the script now uses 150 Tgas.

The server runner: `near-cli-rs` 0.30.1 (the project's release binary, sha256 checked against the published file), the four
test accounts' keys in a root-only folder of its own, nothing else on the server touched. `send_tx` proof on the server: a view
call and one real transaction ([7MrWbvaX…](https://testnet.nearblocks.io/txns/7MrWbvaBXR5SXzzaXKpuhiueF9LejLoJYDfseqAgSz8a),
an idempotent `settle` on the already settled `streak-demo-2`, `SuccessValue`).

**Deposit minimum, live:** a `create_pot` with 5.51 NEAR (one cent under the 5.52 minimum) from the server was refused by the node's
own simulation with `Smart contract panicked: ERR_DEPOSIT_TOO_SMALL`; no transaction was sent, nothing created.
