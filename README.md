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
the first time (near-sandbox downloads its node). 15 tests (2 of them pot scenes for both payout paths: survivors split, no survivors refunded), 3 to 8 minutes depending on load (10 plain `cargo test` runs in a row all passed). The tests deploy the
real Sputnik factory and DAO code (`tests/fixtures`, from `sputnikv2.testnet`).

## Cost to know about

One pot DAO needs about 5.6 NEAR attached on `create_pot`, locked in the DAO (storage for its code).

## Not done yet

The control on testnet, results script, one-command demo, video. Testnet contract: `stakestreak-near.testnet`; first settled pot and links in [docs/TESTNET.md](docs/TESTNET.md). Web client: `web/index.html` (static; serve the folder). See [CHECKLIST.md](CHECKLIST.md).

MIT licensed.
