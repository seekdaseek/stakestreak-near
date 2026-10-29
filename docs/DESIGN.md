# Design decisions

Each decision has its reason. Facts about Sputnik were checked against the code the sandbox
tests run (the bytes `sputnikv2.testnet` serves, `tests/fixtures/`), not only against docs.

## Pot rules (copied from the Solana product, not its code)

| Rule | Solana product | Here |
|---|---|---|
| Equal stakes into a shared pot | yes | yes, exact stake required at `join` |
| Daily check-in, miss a day = out | server cron sweeps | the contract's block clock: a member is in while every completed day has its bit set (`alive()`); no sweep, nobody to trust |
| Survivors split the pot | server sends transfers | settlement fixes a payout list from the contract's books, then each payout is a Transfer proposal in the pot's DAO |
| 3% fee | taken at lock | `fee_bps` per pot, capped at 300, taken at settlement as one more Transfer |
| No survivors | pot swept to the revenue wallet | changed: every funded member gets the stake back minus the fee; the fees go to the fee account as one Transfer (`no_survivors_refunds_stake_minus_fee`). Reason: sweeping the whole pot to the operator looked like a trap when nobody could win |
| Full refund before the pot starts | refund tx from treasury | `leave` before `start_ns`: a refund Transfer proposal, full stake, no fee |
| Under 3 members at lock = refund everyone | cron `killunfilled` | pot starts below `min_members`: `settle` refunds every member in full, no fee |
| Streak freeze (0.015 SOL, max 2, covers yesterday) | server | `use_freeze`, fee to the fee account, max 2, covers yesterday only |
| Check-in slots, late join up to 24 h, invite links | yes | not copied. Joining closes at `start_ns`; day 0 begins there. Reason: fewer moving parts, and a fixed start removes the creator's discretion. Invite link = a URL with the pot id (web client) |

Deviation worth stating: on Solana a freeze re-activates an eliminated member even if older days
were missed. Here a freeze only fills yesterday, so it cannot revive an older miss
(`freeze_covers_yesterday_only`).

## Why one DAO per pot, and what the policy says

`create_pot` makes the Sputnik factory create `<pot>.sputnikv2.testnet` and passes the policy
itself, so every pot has the same policy and a judge reads it with `get_policy`:

- one role, `settler`, a `Group` whose only member is the StakeStreak contract;
- permissions `transfer:AddProposal`, `transfer:VoteApprove`, `transfer:Finalize` and nothing else;
- vote policy for `transfer`: `RoleWeight`, threshold 1 (the single settler approves);
- `proposal_bond` 0, `proposal_period` one year;
- no other role. Nobody can file or vote any other kind: no `ChangePolicy`, no `FunctionCall`, no upgrade.
- the DAO account has no access keys (`lockdown` test), so nobody can sign for it.

Consequence: money leaves the pot's treasury only as a `Transfer` that the contract filed and
approved, and the contract only files transfers from its own payout list (survivors, fee,
refunds). A loser, the creator or a stranger has no call that moves it
(`settler_crash_mid_settlement_pays_only_survivors_once`, `dao_policy_leaves_one_way_out`).

Trust assumptions, stated plainly:
1. The Sputnik factory owner can update DAO code (`factory.update`: "factory owner or the DAO
   itself"). On testnet that is the owner of `sputnikv2.testnet`. Not mitigated here.
2. We keep a full-access key on the StakeStreak contract account during the hackathon (testnet), so
   we could redeploy it. A mainnet version would delete it.
3. Stakes are native NEAR (testnet). Sputnik also pays fungible tokens (`token_id`); not used.

## Stakes go into the DAO immediately

`join` is payable with exactly the stake. The contract forwards it to the DAO account and only marks
the member `funded` once that transfer succeeded (a failed transfer removes the member and
refunds). So the contract never holds pot money; the DAO treasury does.

## Check-ins and function-call keys

`check_in(pot_id)` takes no deposit, so a function-call access key restricted to
`receiver = the contract, methods = ["check_in"]` can sign it. The web client adds that key in the
same wallet transaction as `join`; later daily check-ins need no wallet pop-up. The key cannot call
`leave`, attach a deposit or move money (`check_in_key_can_only_check_in`). `use_freeze` needs a
deposit, so it needs a wallet signature.

## Settlement

`settle(pot_id, batch)` is callable by anyone once the pot ended (or started below minimum). First
call fixes the payout list (stored on chain, with amounts); every call then pushes up to `batch`
payouts: `add_proposal` on the DAO, a callback records the proposal id, `act_proposal(VoteApprove)`,
a callback marks it approved. It is resumable and idempotent: state per payout is
`Unpaid -> Filed(id) -> Approved(id)`, only the first filing counts, so a settler that dies midway
or two callers racing cannot pay anyone twice (the race produced one extra proposal that was never
approved, see the drill test). Equal split is `floor(pool / survivors)`; the few yoctoNEAR of dust
stay in the DAO.

If a payee's account was deleted, the DAO's transfer fails and Sputnik marks the proposal `Failed`; `retry(pot_id, index)` (anyone) sends `Finalize` to run it again. The funds only ever go to the same receiver, so they wait in the treasury until that account exists again (`payee_with_deleted_account_is_paid_after_retry`).

## Cost of one pot DAO (the number that looks bad)

The deployed factory's `create` refuses less than `code_len + 32` bytes of storage:
**5.3968 NEAR** (measured in the sandbox against the deployed code, 29 Oct 2026). The upstream
source on `main` asks for 0.01 NEAR, so the live factory differs from that source; the sandbox
tests use the live bytes. `create_pot` therefore needs about 5.6 NEAR attached (5.4 for the DAO
plus a 0.02 storage reserve kept by the contract). That NEAR is storage staking for the DAO's copy
of the code: it stays in the DAO and is not recoverable, so it is a per-pot cost of the
one-DAO-per-pot design, on testnet free, on mainnet real. Gas for `create_pot`: about 93 Tgas.

## Named accounts

The pot id is the DAO's sub-account label, so a pot is `<name>.sputnikv2.testnet` and the explorer
shows its proposals and transfers under that name.

## The control

`tests/control.rs` builds the same pot as a plain Sputnik multisig (council = the three members,
2 votes needed, no companion contract). With one survivor and two losers who do not vote, the
payout proposal sits at 1 of 2 votes forever and the money stays locked
(`control_multisig_stalls_when_losers_refuse`). The same multisig pays when a second member does
vote (`control_multisig_pays_when_everyone_votes`), so the control is a fair one.
