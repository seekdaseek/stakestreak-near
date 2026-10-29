# Runbook: running a testnet pot

Rules learned from `streak-demo-2` (see [TESTNET.md](TESTNET.md)), where a scripted check-in never
reached the chain and nobody survived.

1. **Scripted accounts run on a server, never on a laptop.** A pot is on the real clock: a laptop
   that freezes or sleeps for a minute loses a day for every account it drives. The runner
   (`scripts/run-actors.sh`) goes on a server with `near` (near-cli-rs) installed and the test
   accounts' keys in that server's own legacy keychain, readable by one user only. These are
   testnet keys; they never go in the repo.
2. **Every transaction goes through `send_tx`** (`scripts/send-retry.sh`). It keeps the CLI's full
   output in the runner log, retries up to 3 times when no transaction id came back, and a contract
   panic (which does have an id) is final and shown, not retried. `run-actors.sh` ends with
   `FINISHED, give-ups: N` and exits non-zero when anything gave up.
3. **Humans click in the page; everything else is in `PLAN`.** A human tester's account is left out
   of `PLAN`. Give them the exact UTC times: each day opens at `start + d x day length`.
4. **After the end, check by view calls, not by the log:** `get_pot` (members, `days`, `alive`,
   `payouts`, `payouts_pending`), `get_proposals` on the pot's DAO, and the `view_account`
   balances. Record transaction links as `testnet.nearblocks.io/txns/<hash>`.
5. **Which key signed?** `tx` RPC, `transaction.public_key`: the function-call key added at sign-in
   means no wallet pop-up was needed; the full-access key means the wallet asked.

Run (one pot, on the server):

```
POT=streak-demo-3 START=<epoch seconds of start_ns> DAY_SECS=600 DAYS=3 \
SETTLER=<an account with gas> PLAN="<m1>:0,1,2 <m2>:0" ./scripts/run-actors.sh
```

The server runner is a folder of its own holding the released `near` binary (sha256 checked), the test accounts' keys
(root-only, own HOME so the CLI never touches anything else) and these scripts. Tested: `send_tx` against a stub CLI (flaky,
dead, refused and panicking cases), `run-actors.sh` end to end against a stub CLI, and on the server with the real CLI: a view
call, a real transaction, and a whole pot (`streak-demo-3` in [TESTNET.md](TESTNET.md)). A refusal that the CLI's own
simulation reports (`ExecutionError`, for example not enough gas or `ERR_...`) is final and is not retried; the log keeps its text.
