//! Sandbox harness: the real Sputnik factory + DAO code (bytes served by sputnikv2.testnet,
//! see scripts/fetch-sputnik.sh), our contract, and helpers.
#![allow(dead_code)]
use near_api::{AccountId, NearGas, NearToken, NetworkConfig, Signer};
use near_sdk::serde::de::DeserializeOwned;
use near_sdk::serde_json::{json, Value};
use std::sync::Arc;

/// base58(sha256(tests/fixtures/dao.wasm)); checked in scripts/fetch-sputnik.sh via SHA256SUMS.
pub const DAO_CODE_HASH: &str = "AA4nA45vauiac3jdVwJTzSdUP3oj2r1fDaPAb2mHdgcH";
pub type R<T> = testresult::TestResult<T>;

pub const NEAR: u128 = 1_000_000_000_000_000_000_000_000;
/// Build the contract once per test binary. The ABI step is skipped: cargo-near's ABI generation
/// dlopen()s a freshly built dylib, which hung for 16 minutes with 0% CPU on macOS (dyld, 2026-10-29),
/// and no test reads the ABI. The ABI is still built and embedded for the deployed wasm (`cargo near build`).
fn contract_wasm() -> R<Vec<u8>> {
    static WASM: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
    if let Some(w) = WASM.get() {
        return Ok(w.clone());
    }
    let opts = cargo_near_build::BuildOpts { no_abi: true, no_embed_abi: true, ..Default::default() };
    let path = cargo_near_build::build_with_cli(opts)?;
    let bytes = std::fs::read(path)?;
    Ok(WASM.get_or_init(|| bytes).clone())
}

pub fn near(n: f64) -> NearToken {
    NearToken::from_yoctonear((n * 1e6) as u128 * (NEAR / 1_000_000))
}

pub struct World {
    pub sandbox: near_sandbox::Sandbox,
    pub net: NetworkConfig,
    pub signer: Arc<Signer>,
    pub factory: AccountId,
    pub streak: AccountId,
}

impl World {
    pub async fn new() -> R<World> {
        let sandbox = near_sandbox::Sandbox::start_sandbox().await?;
        let net = NetworkConfig::from_rpc_url("sandbox", sandbox.rpc_addr.parse()?);
        let signer = Signer::from_secret_key(
            near_sandbox::config::DEFAULT_GENESIS_ACCOUNT_PRIVATE_KEY.parse().unwrap(),
        )?;
        let w = World {
            sandbox,
            net,
            signer,
            factory: "sputnikv2.sandbox".parse()?,
            streak: "streak.sandbox".parse()?,
        };

        // Sputnik factory: deploy, store the DAO code (a global contract), make it the default.
        w.create_account(&w.factory.clone(), 400).await?;
        let factory_wasm = std::fs::read("tests/fixtures/factory.wasm")?;
        let dao_wasm = std::fs::read("tests/fixtures/dao.wasm")?;
        near_api::Contract::deploy(w.factory.clone())
            .use_code(factory_wasm)
            .with_init_call("new", json!({}))?
            .with_signer(w.signer.clone())
            .send_to(&w.net)
            .await?
            .assert_success();
        let factory = w.factory.clone();
        let stored = w
            .call_raw(&factory, &factory, "store", dao_wasm, near(200.0), 300)
            .await?;
        // A 540 KB transaction can be sent twice by the client; the repeat then says ERR_ALREADY_EXISTS.
        if !stored.is_success() {
            let text = format!("{:?}", stored.failures().into_iter().map(|f| f.clone().into_result().err()).collect::<Vec<_>>());
            assert!(text.contains("ERR_ALREADY_EXISTS"), "factory.store failed: {}", &text[..text.len().min(1500)]);
        }
        let hash = DAO_CODE_HASH.to_string();
        w.call(
            &factory,
            &factory,
            "store_contract_metadata",
            json!({"code_hash": hash, "metadata": {"version": [3, 31], "commit_id": "test", "changelog_url": null}, "set_default": true}),
            near(0.0),
            30,
        )
        .await?
        .assert_success();

        // Our contract.
        let wasm = contract_wasm()?;
        w.create_account(&w.streak.clone(), 20).await?;
        near_api::Contract::deploy(w.streak.clone())
            .use_code(wasm)
            .with_init_call("new", json!({"factory": w.factory}))?
            .with_signer(w.signer.clone())
            .send_to(&w.net)
            .await?
            .assert_success();
        Ok(w)
    }

    pub async fn create_account(&self, id: &AccountId, near_amount: u32) -> R<()> {
        self.sandbox
            .create_account(id.clone())
            .initial_balance(NearToken::from_near(near_amount as u128))
            .send()
            .await?;
        // The account is a state patch; wait until the node serves its access key at FINAL finality
        // (the signer's nonce lookup uses Final; Optimistic is a few blocks ahead), or the next transaction fails with "access key does not exist while viewing".
        for _ in 0..60 {
            let keys = near_api::Account(id.clone()).list_keys().at(near_api::types::Reference::Final).fetch_from(&self.net).await;
            if matches!(&keys, Ok(k) if !k.data.is_empty()) {
                return Ok(());
            }
            self.sandbox.fast_forward(3).await?;
        }
        panic!("account {id} never became visible");
    }

    pub async fn user(&self, name: &str) -> R<AccountId> {
        let id: AccountId = format!("{name}.sandbox").parse()?;
        self.create_account(&id, 100).await?;
        Ok(id)
    }

    pub async fn call(
        &self,
        who: &AccountId,
        to: &AccountId,
        method: &str,
        args: Value,
        deposit: NearToken,
        tgas: u64,
    ) -> R<near_api::types::transaction::result::ExecutionFinalResult> {
        Ok(near_api::Contract(to.clone())
            .call_function(method, args)
            .transaction()
            .deposit(deposit)
            .gas(NearGas::from_tgas(tgas))
            .with_signer(who.clone(), self.signer.clone())
            .send_to(&self.net)
            .await?
            .into_full()
            .ok_or("transaction still pending")?)
    }

    pub async fn call_raw(
        &self,
        who: &AccountId,
        to: &AccountId,
        method: &str,
        args: Vec<u8>,
        deposit: NearToken,
        tgas: u64,
    ) -> R<near_api::types::transaction::result::ExecutionFinalResult> {
        Ok(near_api::Contract(to.clone())
            .call_function_raw(method, args)
            .transaction()
            .deposit(deposit)
            .gas(NearGas::from_tgas(tgas))
            .with_signer(who.clone(), self.signer.clone())
            .send_to(&self.net)
            .await?
            .into_full()
            .ok_or("transaction still pending")?)
    }

    pub async fn view<T: DeserializeOwned + Send + Sync>(&self, to: &AccountId, method: &str, args: Value) -> R<T> {
        Ok(near_api::Contract(to.clone())
            .call_function(method, args)
            .read_only()
            .fetch_from(&self.net)
            .await?
            .data)
    }

    pub async fn balance(&self, id: &AccountId) -> R<u128> {
        Ok(near_api::Account(id.clone())
            .tokens()
            .near_balance()
            .fetch_from(&self.net)
            .await?
            .total
            .as_yoctonear())
    }

    /// Move the chain clock: blocks are about a second apart in the sandbox.
    pub async fn wait_blocks(&self, n: u64) -> R<()> {
        self.sandbox.fast_forward(n).await?;
        Ok(())
    }

    /// Chain clock in nanoseconds.
    pub async fn chain_ns(&self) -> R<u64> {
        let b = near_api::Chain::block().fetch_from(&self.net).await?;
        Ok(b.header.timestamp)
    }

    pub async fn pot(&self, name: &str) -> R<Value> {
        self.view(&self.streak, "get_pot", json!({"pot_id": name})).await
    }
}

/// Panic with a short reason (never the whole transaction, which can hold a wasm blob).
pub fn ensure_ok(r: &near_api::types::transaction::result::ExecutionFinalResult, what: &str) {
    if !r.is_success() {
        let reasons: Vec<String> = r
            .failures()
            .into_iter()
            .map(|f| format!("{:?}", f.clone().into_result().err()))
            .collect();
        let mut msg = format!("{what} failed: {}", reasons.join(" | "));
        msg.truncate(3000);
        panic!("{msg}");
    }
}

pub fn pot_args(name: &str, fee_account: &AccountId) -> Value {
    json!({"args": {
        "name": name, "rule": "walk 20 minutes", "stake": (NEAR).to_string(),
        "duration_days": 3, "day_secs": 30, "join_secs": 40, "min_members": 3,
        "fee_account": fee_account, "fee_bps": 300, "freeze_fee": (NEAR / 10).to_string(),
    }})
}

pub const STAKE: u128 = NEAR;
pub const DAY_SECS: u64 = 120;
pub const DAYS: u64 = 3;

pub fn fail_text(r: &near_api::types::transaction::result::ExecutionFinalResult) -> String {
    r.failures()
        .into_iter()
        .map(|f| format!("{:?}", f.clone().into_result().err()))
        .collect::<Vec<_>>()
        .join(" | ")
}

pub struct Ctx {
    pub name: String,
    pub dao: AccountId,
    pub fee: AccountId,
    pub members: Vec<AccountId>,
    pub rando: AccountId,
}

impl World {
    /// Advance the chain clock until it reaches `ns`.
    pub async fn advance_to(&self, ns: u64) -> R<()> {
        loop {
            let now = self.chain_ns().await?;
            if now >= ns {
                return Ok(());
            }
            let blocks = ((ns - now) / 300_000_000 + 2).min(300);
            self.sandbox.fast_forward(blocks).await?;
        }
    }

    /// Land 3 seconds into day `d` of the pot.
    pub async fn goto_day(&self, name: &str, d: u64) -> R<()> {
        let p = self.pot(name).await?;
        let start: u64 = p["start_ns"].as_u64().unwrap();
        let day_ns: u64 = p["day_ns"].as_u64().unwrap();
        self.advance_to(start + d * day_ns + 3_000_000_000).await
    }

    /// Create a pot and let `n` members join. Joining closes `join_secs` after creation.
    pub async fn new_pot(&self, name: &str, n: usize, min_members: u32, fee_bps: u32) -> R<Ctx> {
        self.new_pot_with(name, n, min_members, fee_bps, near(5.6)).await
    }

    /// Same, with the deposit attached to `create_pot` chosen by the caller.
    pub async fn new_pot_with(&self, name: &str, n: usize, min_members: u32, fee_bps: u32, deposit: NearToken) -> R<Ctx> {
        let creator = self.user(&format!("{name}-creator")).await?;
        let fee = self.user(&format!("{name}-fee")).await?;
        let rando = self.user(&format!("{name}-rando")).await?;
        let mut args = pot_args(name, &fee);
        args["args"]["min_members"] = min_members.into();
        args["args"]["fee_bps"] = fee_bps.into();
        args["args"]["day_secs"] = DAY_SECS.into();
        args["args"]["duration_days"] = DAYS.into();
        args["args"]["join_secs"] = 150.into();
        let r = self.call(&creator, &self.streak, "create_pot", args, deposit, 300).await?;
        ensure_ok(&r, "create_pot");
        let mut members = vec![];
        for i in 0..n {
            let m = self.user(&format!("{name}-m{i}")).await?;
            let r = self.call(&m, &self.streak, "join", json!({"pot_id": name}), NearToken::from_yoctonear(STAKE), 60).await?;
            ensure_ok(&r, "join");
            members.push(m);
        }
        self.wait_blocks(10).await?;
        Ok(Ctx {
            name: name.to_string(),
            dao: format!("{name}.{}", self.factory).parse()?,
            fee,
            members,
            rando,
        })
    }

    pub async fn check_in(&self, who: &AccountId, name: &str) -> R<near_api::types::transaction::result::ExecutionFinalResult> {
        self.call(who, &self.streak, "check_in", json!({"pot_id": name}), near(0.0), 30).await
    }

    /// Settle in batches until nothing is pending, then give the DAO callbacks a moment.
    pub async fn settle_all(&self, c: &Ctx, batch: u32) -> R<()> {
        for _ in 0..12 {
            let r = self.call(&c.rando, &self.streak, "settle", json!({"pot_id": c.name, "batch": batch}), near(0.0), 300).await?;
            ensure_ok(&r, "settle");
            eprintln!("settle(batch={batch}) burnt {} Tgas", r.total_gas_burnt.as_gas() / 1_000_000_000_000);
            self.wait_blocks(12).await?;
            let p = self.pot(&c.name).await?;
            if p["payouts_pending"] == 0 {
                return Ok(());
            }
        }
        panic!("settlement did not finish: {}", self.pot(&c.name).await?);
    }

    pub async fn proposals(&self, dao: &AccountId) -> R<Vec<Value>> {
        self.view(dao, "get_proposals", json!({"from_index": 0, "limit": 100})).await
    }
}
