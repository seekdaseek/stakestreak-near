# CHECKLIST (no file, no submit)

Status as of 2026-10-29. DONE / OPEN / UNTESTED are literal.

1. **Rubric.** No formal rubric. Track `social` text mapped to scenes and named tests:
   - "Shared budgets": the pot's stakes sit in one Sputnik DAO treasury per pot. `full_pot_pays_survivors_by_policy`
   - "Programmable rewards": survivors paid by rule, no vote. `full_pot_pays_survivors_by_policy`, `settler_crash_mid_settlement_pays_only_survivors_once`
   - "Collective ownership": the group's money is in a DAO none of them can drain alone. `dao_policy_leaves_one_way_out`
   - "Reputation-based access": the streak board (check-in bits, `alive`) decides who is paid. `missed_day_is_out_by_contract_clock`
   - "Don't rebuild the multisig": the control shows why a multisig alone is not the product. `control_multisig_stalls_when_losers_refuse`
2. **Sponsor primitives by name.** Sputnik DAO factory (`sputnikv2.testnet` `create`), policy roles and per-kind vote policy (`settler` role, `transfer` vote policy), Transfer proposals (`add_proposal`, `act_proposal`), named accounts (`<pot>.sputnikv2.testnet`), function-call access keys (`check_in` only), near-sdk 5.29.1. Trezu: not used (it is a treasury UI; the pot DAO is readable in it, UNTESTED). Why NEAR: the pot's money sits in a DAO whose policy pays survivors and nothing else.
3. **Action.** OPEN: a judge with a testnet wallet joins a demo pot and checks in within five minutes (needs testnet deploy + web client, Nov 1-4). DONE: one command runs whole pots in near-sandbox: `scripts/fetch-sputnik.sh` is not needed (fixtures are committed), `cargo test` runs 13 tests. Expected output in README: OPEN (a one-command demo script that prints a pot's story is planned for Nov 5-9).
4. **Evidence.** Headline: pots settled on testnet: 0 so far; sandbox only. Own test pots only, no outside users, which lowers the odds; this will be written in the idea text. Counted by script into docs/RESULTS.md: OPEN.
5. **Field twins.** Searched 2026-10-29 (three web searches: Sputnik + savings circle/accountability/commitment; NEAR habit/check-in stake apps; Trezu). Found nothing on NEAR that is a stake-and-streak accountability pot or savings circle on Sputnik. Weak evidence only (search tool covers Devpost/NEAR forum poorly); we will not claim "first". Closest relatives are generic treasury tools (Trezu, Sputnik UIs). Our angle: a streak game whose payout needs no vote.
6. **Format.** README (draft in repo, final OPEN), LICENSE MIT DONE, video OPEN, X post OPEN (he posts), idea text 20-2000 characters with the headline first OPEN.
7. **Liveness to Nov 25.** OPEN: demo pot DAOs and the contract stay on testnet; testnet resets are a risk (state it in README).
8. **Social.** X post required, he posts. OPEN.
9. **Fit.** Social track, Sputnik DAO is named in the track text. DONE.
10. **Control built and shown failing.** DONE in sandbox (`control_multisig_stalls_when_losers_refuse`, `control_multisig_pays_when_everyone_votes`). On testnet with explorer links: OPEN.
11. **Gates.** Testnet NEAR from the faucet: OPEN. The DAO create deposit is 5.3968 NEAR per pot (see DESIGN.md), so N testnet pots need about 5.6 x N NEAR in creator accounts: OPEN, plan the faucet accounts first.
12. **Artifacts.** One-command demo with expected output: OPEN. docs/RESULTS.md written by script: OPEN. The number that makes us look bad: DONE, 5.3968 NEAR locked per pot DAO (DESIGN.md). Destructive drill: DONE in sandbox (`settler_crash_mid_settlement_pays_only_survivors_once`). docs/FEEDBACK.md: OPEN (notes so far: deployed factory differs from upstream source; get_proposals shape; near-api drops node error text). docs/SUBMISSION.md, docs/VIDEO.md: OPEN.

Disclosures to put in the README: existing product on another chain (Solana StakeStreak), only the NEAR integration is claimed; AI-assisted code, listed (README line: "Written with AI coding assistance, as the rules allow (Q-005)."); names the other project, agentfeed-near (track `agents`).
