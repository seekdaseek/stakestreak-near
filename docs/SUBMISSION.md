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
> Try it: `cargo test --test demo -- --ignored --nocapture` tells one whole pot in about a minute; the hosted web client at https://stakestreak-near.pages.dev joins a testnet pot with Meteor Wallet.

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
The 5.5 NEAR per pot is a property of the testnet factory, not of the design; see the next section. No mainnet pot has been created.

## What a pot costs, honestly (the number that looks bad)

**Who paid for `streak-demo-4`'s DAO, and where is the money?** The creator account of the run (a test account, `create_pot` with 5.6 NEAR
attached, [6zjjGiXz…](https://testnet.nearblocks.io/txns/6zjjGiXze43ugvJ3xto93XhGq6iNvZcAWHjmPLjuW9ME)). The contract keeps 0.02 NEAR and
forwards 5.58 NEAR to the Sputnik factory, which creates `streak-demo-4.sputnikv2.testnet` with that balance. After the full 1.5 NEAR payout the DAO
still holds exactly 5.58 NEAR (read from the node). Of that, **5.411 NEAR is storage staking**: the DAO account's storage is 541,136 bytes
(0.00001 NEAR per byte), almost all of it a copy of the DAO's own wasm (about 540 KB) that the testnet factory stores inside every DAO.
The other 0.169 NEAR is spare. All testnet NEAR here came from the faucet, so this run cost nothing real.

**Can any of it be recovered?** No, not by anything in this project:
- the DAO account has no access keys (checked: empty key list) and its policy allows one thing, Transfer proposals filed by the StakeStreak
  contract from its payout list, so nobody can sign for it or delete it;
- storage staking is released only if the account or its code goes away, and neither can happen;
- the 0.169 NEAR spare could in principle be paid out by a Transfer, but the contract never files one, and it is about 3% of the cost;
- the only party that can change a DAO's code is the Sputnik factory owner (`factory.update`), which is not us.

Five DAOs exist on testnet from our runs (pots 1 to 4 and the control), about 27.9 NEAR locked, all faucet NEAR.

**What would a pot cost on mainnet?** Probably far less. The storage numbers below were **measured on other people's DAOs, not on a pot of ours**: no DAO of ours exists on mainnet and nothing was sent there.
The mainnet factory `sputnik-dao.near` (read-only RPC calls, nothing sent) now creates DAOs that run a **global contract**
(`global_contract_hash` `DtHZA1mU…`, local code hash empty). The three newest DAOs on it (2,040 in total) use 2,310 to 5,927 bytes of storage
(0.023 to 0.059 NEAR) and hold about 0.09 NEAR. The testnet factory we tested against predates that and stores the 540 KB code in each DAO.
So on mainnet one pot DAO should cost about 0.1 to 0.2 NEAR (our policy and up to 21 proposals add a little), not 5.5.
**Not verified:** nobody has created a DAO through the mainnet factory with our policy; our sandbox tests run the testnet DAO code, which is
not the mainnet code; and the contract's minimum deposit was tuned for the testnet factory (5.5 NEAR). It is now a per-deployment init value (`new(factory, min_dao_deposit)`, tested), so a mainnet
deployment would set its own; it still needs a test run against the mainnet DAO code first.

**Fixes that exist, with their price:**
1. *Use the global-contract factory (mainnet as it is today).* Cuts the cost about 30 to 50 times with no design change. Needs a mainnet minimum
   deposit and a re-run of the tests on the mainnet DAO code. Not done; mainnet needs the developer's go.
2. *One shared DAO for many pots.* The contract already keeps a per-pot payout list, so one DAO could hold every pot's stakes and pay each pot's
   survivors; the 5.5 NEAR is then paid once. Cost: pots stop being separate accounts (one pool, one policy, a bug in the ledger touches all pots),
   and the contract, tests and web client change. Roughly a day of work; worth it only if we stay on a factory that stores the code per DAO.
3. *Creator pays and gets it back.* Does not work: storage staking cannot be released while the DAO exists. At most the 0.169 NEAR spare (3%)
   could be returned at settlement, which is not worth a contract change.

Recommendation: leave the submission as it is (testnet, one DAO per pot, 5.5 NEAR stated up front) and say plainly that mainnet cost is expected to
be about a fiftieth of that, unmeasured.
