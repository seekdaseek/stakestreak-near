# Video plan (not recorded yet)

Goal: under 3 minutes, one take of a real pot plus two short cutaways, no music needed. The point to land:
**the DAO policy pays the survivors; nobody votes, nobody holds the money.**

## What gets recorded

A dedicated demo pot with **2-minute days** (3 days = 6 minutes, joining open 2 minutes; the contract allows
days down to 10 seconds), so the whole pot fits one screen recording. Scripted accounts run on a server
([RUNBOOK.md](RUNBOOK.md)); the developer's wallet account clicks in the page. Recorded only on the developer's yes.

## Shots and lines (narration is the developer's voice, in his words; these are the beats)

| Time | On screen | Beat |
|---|---|---|
| 0:00 | Web client, pot list | "Friends stake NEAR, check in daily, miss one and you are out. The survivors split the pot." |
| 0:15 | Pot page, DAO link on nearblocks, policy | "The pot is a Sputnik DAO. Its policy has one role, our contract, allowed to file and approve Transfers. Nothing else." |
| 0:35 | Join with Meteor, the page then shows members | "Joining sends the stake straight into the DAO." |
| 0:50 | Check-in click, no wallet window | "Check-ins use a key limited to check_in, so no pop-up." |
| 1:05 | Streak board across the three days (cut to the end of each day) | one member misses a day and turns OUT by the contract clock |
| 1:40 | Control cutaway: `streak-control-1` proposal stuck at 1 of 2 | "The same pot as a plain multisig: the losers will not sign, the money sits there." |
| 2:00 | Click settle; DAO proposals appear Approved, proposer = the contract | "Anyone calls settle. Each payout is a Transfer the policy approves." |
| 2:20 | Balances / nearblocks transfers | numbers read from the screen, not rounded up |
| 2:35 | RESULTS.md and the cost line | "Own test pots only, no outside users. A pot DAO locks about 5.4 NEAR; that is the price of one DAO per pot." |

## Checklist before recording

1. Fresh pot created (about 5.6 NEAR from a creator account), at least 3 funded members, a member who will miss.
2. Browser: wallet signed in with the check-in key present; hard-reload the page; nearblocks tabs pre-opened
   (the site sits behind a human check, so open them once by hand first).
3. Runner started on the server with START read from the pot's `start_ns`.
4. Do not show the developer's other accounts, local paths or keys. Do not show any mainnet wallet.
5. After the take: run `node scripts/results.mjs`, commit RESULTS.md and TESTNET.md with the real transaction links.

## What the video will not claim

No outside users. No mainnet. No claim that it is the first of its kind.
