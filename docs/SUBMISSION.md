# Submission draft (not submitted; the submission waits for the developer's yes)

Track: `social`. Repo: github.com/seekdaseek/stakestreak-near (private until the public flip).
Refresh the numbers from [RESULTS.md](RESULTS.md) (`node scripts/results.mjs`) right before submitting.

## Idea text (limit 20-2000 characters; headline first)

> Friends stake NEAR into one Sputnik DAO, check in daily, and a DAO policy pays the survivors, with no vote.
>
> StakeStreak (live on the Solana dApp Store) is a habit game: equal stakes go into a shared pot, you check in every day, miss one and you are out, the survivors split the pot. On NEAR the pot is a Sputnik DAO treasury made by the factory for each pot (`<pot>.sputnikv2.testnet`). Its policy has one role, our contract, allowed to file and approve Transfer proposals and nothing else. The contract's block clock decides who is out. After the last day anyone calls settle; each payout is a Transfer proposal the policy approves. Nobody holds the money and nobody has to vote, which is what a multisig cannot do: we ran the same pot as a plain 2-of-3 Sputnik multisig on testnet and it sat at 1 of 2 votes, because the losers have no reason to sign.
>
> Check-ins use a function-call key limited to `check_in`, so after joining there is no wallet pop-up.
>
> Evidence, testnet, our own test pots (no outside users): see docs/RESULTS.md for the counts. We also say plainly what looks bad: a pot DAO locks about 5.4 NEAR for the DAO's copy of its code, and the first full pot we ran ended with nobody alive, which led us to change the rule so losers are refunded their stake minus the fee instead of the pot going to the operator.
>
> Disclosures: the product exists on Solana; only the NEAR integration (written from 2026-10-19) is the submission. Written with AI coding assistance, as the rules allow (Q-005). Same author's other entry: agentfeed-near (track `agents`).
>
> Try it: `cargo test --test demo -- --ignored --nocapture` tells one whole pot in about a minute; the web client in `web/` joins a testnet pot with Meteor Wallet.

Quoted text: 1,724 characters when drafted (counted by script); recount after any edit.

## Field checklist

| Item | State |
|---|---|
| README + MIT LICENSE | in repo |
| Repo public by Nov 14 23:59Z | OPEN, the developer flips it on Nov 13 |
| Demo video | OPEN, plan in [VIDEO.md](VIDEO.md) |
| X post with the video | OPEN, the developer posts |
| Liveness to Nov 25 | contract and pot DAOs stay on testnet; testnet resets are a risk, stated in the README |
| Evidence line | from RESULTS.md, own test pots only |

## Claims we do not make

No outside users. Not "first" on NEAR (search was weak). Not audited. Testnet only, no mainnet.
The 5.4 NEAR per pot is real money on mainnet; a mainnet version would need the global-contract factory path.
