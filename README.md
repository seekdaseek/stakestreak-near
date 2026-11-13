# StakeStreak on NEAR

Friends put equal stakes into a shared pot, check in every day, and miss a day and you are out.
The survivors split the pot. On NEAR the pot is a Sputnik DAO treasury and the DAO's policy pays the
survivors: no vote, no server holding the money.

NEAR HACKATHON, track `social`. Testnet only: nothing here is deployed on mainnet.
Contract: `stakestreak-near.testnet`. Hosted web client: https://stakestreak-near.pages.dev

## The problem and what NEAR does

A habit pot needs someone to hold the money and decide who failed. A multisig cannot do the second
part: the people who lost have no reason to sign. Here the pot is a Sputnik DAO created per pot, its
policy has one role (the StakeStreak contract, allowed to file and approve `Transfer` proposals and
nothing else), and the contract's block clock decides who is out. After the last day anyone calls
`settle`; each payout is a `Transfer` proposal the policy approves.

## Try it (do X, see Y)

1. Open https://stakestreak-near.pages.dev. See four pots, `streak-demo-1` to `streak-demo-4`, all
   `settled`. Open `streak-demo-4`: the streak board (one member out after missing a day), the payouts
   (0.045 NEAR fee, 0.7275 NEAR to each of two survivors) and "What the DAO itself says", the policy
   and proposals read from the DAO account on chain.
2. Read the contract directly. Run the command below. See a JSON byte array of four pots with
   `"phase":"settled"` (the numbers are the UTF-8 bytes of the JSON; `"from"` and `"limit"` are in the base64 arguments).
   ```
   curl -s https://rpc.testnet.fastnear.com -H 'content-type: application/json' -d '{"jsonrpc":"2.0","id":1,"method":"query","params":{"request_type":"call_function","finality":"final","account_id":"stakestreak-near.testnet","method_name":"list_pots","args_base64":"eyJmcm9tIjowLCJsaW1pdCI6MjB9"}}'
   ```
3. Read a pot DAO's policy. Run the command below and decode `result.result`. See one role, `settler`,
   with `Group` = `stakestreak-near.testnet` and permissions `transfer:VoteApprove`,
   `transfer:AddProposal`, `transfer:Finalize`.
   ```
   curl -s https://rpc.testnet.fastnear.com -H 'content-type: application/json' -d '{"jsonrpc":"2.0","id":1,"method":"query","params":{"request_type":"call_function","finality":"final","account_id":"streak-demo-4.sputnikv2.testnet","method_name":"get_policy","args_base64":"e30="}}'
   ```
4. Run one whole pot locally, about a minute after the first build:
   `cargo test --test demo -- --ignored --nocapture`. See the output block under "One-command demo".
5. Run the whole suite: `RUSTUP_TOOLCHAIN=stable cargo test`. See 22 tests pass.
6. Check in with no wallet pop-up (needs a testnet wallet; starting a pot attaches 5.6 NEAR from the
   faucet, see "Cost to know about"): sign in with Meteor Wallet on the hosted page, start a pot or join
   an open one (one approval, the stake plus gas), then press "Check in". See no wallet window: the
   transaction is signed by a function-call key limited to `check_in`. The recorded run of this is in
   [docs/TESTNET.md](docs/TESTNET.md) (pot 4, signer keys read from the `tx` RPC).

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
- The control: the same pot as a plain 2-of-3 Sputnik multisig stalls when the losers will not vote
  (`streak-control-1` on testnet, and a sandbox test).

## What ran (testnet, our own test pots, no outside users)

Counted by `node scripts/results.mjs` from view calls on the contract, as of 2026-11-13T08:48:38Z
(block 277487647), written to [docs/RESULTS.md](docs/RESULTS.md): 4 pots created, 4 settled, 7.0 NEAR
staked and 7.0 NEAR paid out by 12 approved Transfer proposals. Five accounts were funded: four test
accounts and one separate wallet held by the developer. They are not outside users. Transaction links
for every pot are in [docs/TESTNET.md](docs/TESTNET.md). Pot 2 settled under an earlier rule (the
whole pot went to the fee account when nobody survived); that rule was changed and the later pots show the new one.

## Cost to know about

On the testnet factory one pot DAO needs 5.52 NEAR or more attached to `create_pot` (the web client
attaches 5.6): 5.58 NEAR goes into the DAO and about 5.4 NEAR of it is storage for the DAO's own
code, which the testnet factory stores inside every DAO and which cannot be recovered. This is the
number that looks bad. The mainnet factory uses a global contract and should cost far less, but we
measured that only on other people's DAOs, not on a pot of ours; see the section "What a pot costs,
honestly" in [docs/SUBMISSION.md](docs/SUBMISSION.md). The minimum deposit is a per-deployment init value.

## Run it

```
RUSTUP_TOOLCHAIN=stable cargo test
```

Needs Rust stable with the `wasm32-unknown-unknown` target, `cargo-near` 0.22 and network access the
first time (near-sandbox downloads its node). 22 tests (pot scenes for both payout paths, survivors
split and no survivors refunded, the create-deposit boundary, the per-deployment minimum and the
migration), 3 to 7 minutes depending on load. The tests deploy the real Sputnik factory and DAO code
(`tests/fixtures`, from `sputnikv2.testnet`).

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

## Hackathon rules, by item

- **Q-001, Q-002, Q-003: what is the submission.** StakeStreak already exists on Solana (live on the
  Solana dApp Store) in a separate codebase that is not part of this submission. This repository
  contains only the NEAR integration. It has no pre-hackathon history: its first commit,
  [1355c2c](https://github.com/seekdaseek/stakestreak-near/commit/1355c2c), is dated 2026-10-29 and
  already holds the contract and its sandbox tests, so everything in the repository was written
  inside the window that opened with the announcement on 2026-10-19. Full history:
  https://github.com/seekdaseek/stakestreak-near/commits/hackathon-submission . Diff from the first
  commit to the tag `hackathon-submission`:
  https://github.com/seekdaseek/stakestreak-near/compare/1355c2c...hackathon-submission .
- **Q-005: libraries and AI.** Written with AI coding assistance, as the rules allow (Q-005). Libraries:
  near-sdk 5.29.1; for the tests near-sandbox 0.3.12, near-api 0.8, cargo-near-build 0.11.0, tokio,
  testresult. The Sputnik DAO factory and DAO wasm in `tests/fixtures` are the code deployed at
  `sputnikv2.testnet`, not ours (hashes in `tests/fixtures/SHA256SUMS`). The web client is one static
  page (no build step) that loads `@near-wallet-selector` core, modal-ui and meteor-wallet 10.1.4 from
  jsDelivr, reads testnet through FastNEAR RPCs and links to NEAR Rocks. Hosting: Cloudflare Pages.
  Own libraries from before the window: none.
- **Q-010, Q-012: testnet and demo.** Testnet only. The hosted client and the testnet contract stay
  up through judging and, as far as we can keep them, to 2026-11-25; a testnet reset would erase the
  pots (the recorded runs and their transaction links are in [docs/TESTNET.md](docs/TESTNET.md)).
  The demo video is in the X post for this entry.
- **Q-014: license.** MIT, see [LICENSE](LICENSE). The repository stays public through judging.
- **Q-015: other entry.** The same author's other entry is
  [agentfeed-near](https://github.com/seekdaseek/agentfeed-near) (track `agents`, also `realworld`),
  an independent project: a paid data gateway for AI agents, live at https://near.ochinimus.app.
  StakeStreak on NEAR is not part of it and it is not part of this repository.

## Layout

`src/lib.rs` the contract; `tests/` sandbox scenes against the real Sputnik code; `web/` the static
client; `scripts/` testnet runners and `results.mjs`; `docs/` design, testnet log, results, runbook,
cost analysis, notes on NEAR tooling ([FEEDBACK.md](docs/FEEDBACK.md)); [CHECKLIST.md](CHECKLIST.md) the
status of each submission item.

MIT licensed.
