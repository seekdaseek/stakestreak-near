# Feedback on Sputnik DAO and NEAR tooling

Things we hit building this, each with how it showed up. Written as we go; dates are 2026.

## Sputnik DAO

1. **Deployed factory differs from the source on `main`.** `sputnikv2.testnet` reports DAO code
   built from commit `7764926…` (metadata via `get_contracts_metadata`), which we could not find in
   `near-daos/sputnik-dao-contract`. On `main` the factory's `create` needs 0.01 NEAR; the deployed
   one needs 5.3968 NEAR (it stores the DAO code locally: `code_len + 32` bytes of storage). We only
   found out by running the deployed bytes in a sandbox. Suggestion: publish the commit that built
   each registered code hash, and state the real minimum deposit in the docs.
2. **A DAO costs about 5.4 NEAR in locked storage on the deployed factory**, which makes
   "one DAO per small group" expensive on mainnet. The `main` source uses global contracts, which
   would remove most of that cost.
3. **`create` needs more than 80 Tgas** when called from a contract (we use 240); the factory's own
   constants suggest far less.
4. **The policy format is a good fit for rule-based payouts.** One role whose only member is a
   contract, `transfer:AddProposal` + `transfer:VoteApprove` + `transfer:Finalize`, makes a treasury
   that only that contract can pay out of. `Finalize` also gave us a retry path for failed transfers
   without extra DAO code. A short doc example of "a contract as a council member" would have saved
   us reading `policy.rs`.
5. **`WeightOrRatio::Ratio(n, d)` reads as a fraction but rounds up to `n*total/d + 1`**, so 2/3
   of 3 is unanimity. For a fixed "2 of N" use `Weight`.
6. **No way to recover the creation deposit**: the DAO account cannot be deleted by design.

## Tooling

1. **near-api (Rust, 0.8) drops the node's reason text** for a transaction refused at validation
   (it prints `InvalidTransaction({})`), so a test of "this key may not call that method" can only
   assert that it was refused. The node does say why on the raw RPC.
2. **near-sandbox `store` of a 540 KB contract: the client sent the transaction twice** and the
   second run failed with the contract's own `ERR_ALREADY_EXISTS`; the first had succeeded.
3. **cargo-near build inside `cargo test`** needs `near-sdk` view return types to implement
   `JsonSchema` (use `#[near(serializers = [json])]`, not a bare `Serialize` derive).
4. **near-cli-rs prints everything to stderr**, including view results; a shell script that pipes
   stdout gets nothing.
5. **Wallet Selector 10.1.4 through esm.sh fails in the browser** (`js-sha256` takes its Node path
   and calls `require`); the jsDelivr `+esm` build of the same version works.
6. **Testnet faucet service rate limit**: the sixth account creation in a row got HTTP 429.
7. **Sandbox clock**: about 0.3 s of chain time per fast-forwarded block, so a day-based contract
   is testable by fast-forwarding, but the clock also runs with wall time, so a test must not rely
   on slow transactions finishing inside a short "day".
