# StakeStreak on NEAR

Friends put equal stakes into a shared pot, check in every day, and miss a day and you are out.
The survivors split the pot. On NEAR the pot is a Sputnik DAO treasury and the DAO's policy pays the
survivors: no vote, no server holding the money.

Hackathon track: `social` (NEAR HACKATHON). Status: work in progress, testnet only.

**Disclosures.** StakeStreak already exists on Solana (live on the Solana dApp Store); only the NEAR
integration in this repo, written from 2026-10-19 on, is the submission. The code was written with
AI assistance (Claude Code). The same author's other entry is
[agentfeed-near](https://near.ochinimus.app) (track `agents`).

## How it works

- `create_pot` makes the Sputnik factory create `<pot>.sputnikv2.testnet` with a fixed policy: one
  role, the StakeStreak contract, allowed to file and approve `Transfer` proposals only. See
  [docs/DESIGN.md](docs/DESIGN.md).
- `join` forwards the exact stake to the DAO. `check_in` takes no deposit, so a function-call key
  limited to it signs the daily check-in without a wallet pop-up.
- The contract's block clock decides who is out. After the last day anyone calls `settle`; each
  payout is a Transfer proposal the policy approves.
- The control: the same pot as a plain 2-of-3 Sputnik multisig stalls when the losers will not vote.

## Run it

```
RUSTUP_TOOLCHAIN=stable cargo test
```

Needs Rust stable with the `wasm32-unknown-unknown` target, `cargo-near` 0.22 and network access
the first time (near-sandbox downloads its node). 13 tests, about 2.5 minutes. The tests deploy the
real Sputnik factory and DAO code (`tests/fixtures`, from `sputnikv2.testnet`).

## Cost to know about

One pot DAO needs about 5.6 NEAR attached on `create_pot`, locked in the DAO (storage for its code).

## Not done yet

Testnet deployment, web client, results script, video. See [CHECKLIST.md](CHECKLIST.md).

MIT licensed.
