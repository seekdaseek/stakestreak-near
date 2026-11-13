# CHECKLIST

Status as of 2026-11-13, written before the repository was made public. DONE / OPEN / UNTESTED are literal.
The rules read live from https://nearhackathon.com/HACKATHON.md on 2026-11-13 (sha256 prefix `6dd0c1ff2f98a2fc`).

1. **Fit.** Track `social` (Sputnik DAO, named accounts, shared budgets, programmable rewards). DONE.
   Named tests: `full_pot_pays_survivors_by_policy`, `settler_crash_mid_settlement_pays_only_survivors_once`,
   `dao_policy_leaves_one_way_out`, `missed_day_is_out_by_contract_clock`, `control_multisig_stalls_when_losers_refuse`.
2. **Sponsor primitives.** Sputnik DAO factory `create`, policy roles and per-kind vote policy, `Transfer` proposals
   (`add_proposal`, `act_proposal`), named accounts (`<pot>.sputnikv2.testnet`), function-call access keys (`check_in` only),
   near-sdk 5.29.1. Trezu: not used. DONE.
3. **Action a judge can take.** README "Try it" has six steps with commands and expected results. The hosted client
   lists the four settled pots (checked headless on 2026-11-13); the two `curl` view calls were run on 2026-11-13;
   `cargo test --test demo -- --ignored --nocapture` is run from a fresh public clone before submitting (see the final report).
4. **Evidence.** [docs/RESULTS.md](docs/RESULTS.md), written by `node scripts/results.mjs` (10 s timeout per call, 3 tries,
   10 minute limit) from view calls at block 277487647, 2026-11-13T08:48:38Z: 4 pots, 4 settled, 7.0 NEAR staked, 7.0 NEAR paid
   by 12 approved Transfer proposals. Own test pots, no outside users. DONE.
5. **The number that looks bad.** 5.5 NEAR locked per pot DAO on the testnet factory, stated in README and
   [docs/SUBMISSION.md](docs/SUBMISSION.md); mainnet cost is expected to be far lower but is UNTESTED (no DAO of ours exists on mainnet).
6. **Control built and shown failing.** DONE in the sandbox and on testnet (`streak-control-1`, see docs/TESTNET.md).
7. **Destructive drill.** DONE in the sandbox (`settler_crash_mid_settlement_pays_only_survivors_once`).
8. **Field twins.** Searched 2026-10-29, nothing found on NEAR that is a stake-and-streak pot on Sputnik; weak evidence, so we do not claim "first".
9. **Liveness to 2026-11-25.** OPEN by nature: the contract, the pot DAOs and the hosted client are on testnet and a testnet reset would erase them.
10. **Demo video.** DONE: 90 s, narrated, captions in a sidecar file; it is attached to the X post (the developer posts it).
11. **X post.** OPEN until the developer posts it.
12. **Submission.** OPEN until the developer reads the full payload and says yes.

## Rules by item (HACKATHON.md, read live 2026-11-13)

| Item | Where it is met |
|---|---|
| Q-001 window from 2026-10-19 | the repository's first commit is 2026-10-29; no earlier history (README, "Hackathon rules") |
| Q-002 existing product on another chain | disclosed in README and in the idea text: Solana codebase is separate and not submitted |
| Q-003 tags plus README section with the diff | tag `hackathon-submission`; README links the history and the compare view from `1355c2c` |
| Q-005 libraries, AI code | README lists them; the AI line is in README and in the idea text |
| Q-010, Q-012 testnet only, demo | testnet only; public repo, reproducible README, hosted client, X post with the video |
| Q-014 license | MIT, `LICENSE` |
| Q-015 other entry | README names `agentfeed-near` with its repository URL; its README names this one (checked 2026-11-13) |
| Q-013, Q-016, Q-017 | nothing to do on our side; the README is written for a machine reader as well |
