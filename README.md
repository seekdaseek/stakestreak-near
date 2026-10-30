# StakeStreak on NEAR

Friends put equal stakes into a shared pot, check in every day, and miss a day and you are out.
The survivors split the pot. On NEAR the pot is a Sputnik DAO treasury and the DAO's policy pays the
survivors: no vote, no server holding the money.

Hackathon track: `social` (NEAR HACKATHON). Status: work in progress, testnet only.

**Disclosures.** StakeStreak already exists on Solana (live on the Solana dApp Store); only the NEAR
integration in this repo, written from 2026-10-19 on, is the submission.
Written with AI coding assistance, as the rules allow (Q-005).
The same author's other entry is [agentfeed-near](https://near.ochinimus.app) (track `agents`).

## How it works

- `create_pot` makes the Sputnik factory create `<pot>.sputnikv2.testnet` with a fixed policy: one
  role, the StakeStreak contract, allowed to file and approve `Transfer` proposals only. See
  [docs/DESIGN.md](docs/DESIGN.md).
- `join` forwards the exact stake to the DAO. `check_in` takes no deposit, so a function-call key
  limited to it signs the daily check-in without a wallet pop-up.
- The contract's block clock decides who is out. After the last day anyone calls `settle`; each
  payout is a Transfer proposal the policy approves.
- If nobody survives, each member gets the stake back minus the fee (3%), and the fees go to the
  fee account. The pot is never swept whole to the operator.
- The control: the same pot as a plain 2-of-3 Sputnik multisig stalls when the losers will not vote.

## Run it

```
RUSTUP_TOOLCHAIN=stable cargo test
```

Needs Rust stable with the `wasm32-unknown-unknown` target, `cargo-near` 0.22 and network access
the first time (near-sandbox downloads its node). 18 tests (pot scenes for both payout paths, survivors split and no survivors refunded, and the create-deposit boundary), 3 to 7 minutes depending on load (the last 10 plain `cargo test` runs in a row, after the final contract and harness change, all passed: 18 tests each, 197 to 414 s). The tests deploy the
real Sputnik factory and DAO code (`tests/fixtures`, from `sputnikv2.testnet`).

## One-command demo

```
cargo test --test demo -- --ignored --nocapture
```

Tells one whole pot on a local chain in about a minute (after the first build): four friends, 1 NEAR
each, Cy and Dee miss a day, the policy pays the two survivors. Output from a real run (also in
[docs/demo-output.txt](docs/demo-output.txt)); the small negative balances are the members' own gas:

```
StakeStreak on NEAR: one pot, start to finish (local sandbox, real Sputnik factory and DAO code)
pot DAO: story.sputnikv2.sandbox   stake 1 NEAR each, 4 members, DAO holds 9.5800 NEAR
DAO policy: roles = 1, only role = 'settler', permissions = ["transfer:VoteApprove","transfer:AddProposal","transfer:Finalize"]

-- end of day 0 check-ins (Cy skips day 1, Dee skips day 0)
   Ana  days [x..]  checkins 1  in
   Bo   days [x..]  checkins 1  in
   Cy   days [x..]  checkins 1  in
   Dee  days [...]  checkins 0  in

-- end of day 1 check-ins (Cy skips day 1, Dee skips day 0)
   Ana  days [xx.]  checkins 2  in
   Bo   days [xx.]  checkins 2  in
   Cy   days [x..]  checkins 1  in
   Dee  days [.x.]  checkins 1  OUT

-- end of day 2 check-ins (Cy skips day 1, Dee skips day 0)
   Ana  days [xxx]  checkins 3  in
   Bo   days [xxx]  checkins 3  in
   Cy   days [x.x]  checkins 2  OUT
   Dee  days [.x.]  checkins 1  OUT

-- pot ended
   Ana  days [xxx]  checkins 3  in
   Bo   days [xxx]  checkins 3  in
   Cy   days [x.x]  checkins 2  OUT
   Dee  days [.x.]  checkins 1  OUT

-- settle (anyone may call it; here a bystander)
   proposal #0 fee account  0.1200 NEAR  Approved  proposed by the contract
   proposal #1 Ana          1.9400 NEAR  Approved  proposed by the contract
   proposal #2 Bo           1.9400 NEAR  Approved  proposed by the contract

-- balances after (change)
   Ana  +1.9394
   Bo   +1.9394
   Cy   -0.0004
   Dee  -0.0002
   fee  +0.1200
   DAO  paid out 3.9996 NEAR (the pot) and kept its creation storage
```

## Cost to know about

One pot DAO needs about 5.6 NEAR attached on `create_pot`, locked in the DAO (storage for its code).

## Not done yet

A freeze/leave pot on testnet, video. The control ran on testnet (`streak-control-1`). Testnet contract: `stakestreak-near.testnet`; pots and links in [docs/TESTNET.md](docs/TESTNET.md). Web client: `web/` (static), hosted at https://stakestreak-near.pages.dev (Cloudflare Pages, no backend: it reads testnet through FastNEAR RPCs and signs with Meteor Wallet). Counts of what ran on testnet: [docs/RESULTS.md](docs/RESULTS.md), written by `node scripts/results.mjs`. See [CHECKLIST.md](CHECKLIST.md).

MIT licensed.
