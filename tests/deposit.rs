mod common;
use common::*;
use near_api::NearToken;
use near_sdk::serde_json::json;

/// The least `create_pot` accepts: MIN_DAO_DEPOSIT (5.5) + the 0.02 storage reserve.
const LEAST: u128 = 5_520_000_000_000_000_000_000_000;

/// Boundary: one yoctoNEAR under the minimum is refused up front and creates nothing.
#[tokio::test]
async fn one_yocto_under_the_minimum_is_refused() -> R<()> {
    let w = World::new().await?;
    let creator = w.user("dep-creator").await?;
    let fee = w.user("dep-fee").await?;
    let r = w
        .call(&creator, &w.streak, "create_pot", pot_args("dep-low", &fee), NearToken::from_yoctonear(LEAST - 1), 300)
        .await?;
    assert!(fail_text(&r).contains("ERR_DEPOSIT_TOO_SMALL"), "{}", fail_text(&r));
    let p: Option<near_sdk::serde_json::Value> = w.view(&w.streak, "get_pot", json!({"pot_id": "dep-low"})).await?;
    assert!(p.is_none(), "nothing was created");
    Ok(())
}

/// Boundary: exactly the minimum is accepted, the DAO is created, a full pot settles, and after the
/// payouts the DAO still holds at least what its own storage needs (so nothing it owes can fail).
#[tokio::test]
async fn exactly_the_minimum_creates_and_settles() -> R<()> {
    let w = World::new().await?;
    let c = w.new_pot_with("dep-min", 3, 3, 300, NearToken::from_yoctonear(LEAST)).await?;
    let p = w.pot(&c.name).await?;
    assert_eq!(p["created"], true, "{p}");
    for day in 0..DAYS {
        w.goto_day(&c.name, day).await?;
        for m in &c.members {
            ensure_ok(&w.check_in(m, &c.name).await?, "check_in");
        }
    }
    w.goto_day(&c.name, DAYS).await?;
    w.settle_all(&c, 3).await?;
    let props = w.proposals(&c.dao).await?;
    assert_eq!(props.len(), 4, "fee + three winners");
    assert!(props.iter().all(|p| p["status"] == "Approved"));
    let acct = near_api::Account(c.dao.clone()).view().fetch_from(&w.net).await?.data;
    let storage_cost = acct.storage_usage as u128 * 10u128.pow(19);
    let balance = w.balance(&c.dao).await?;
    assert!(balance >= storage_cost, "DAO balance {balance} must cover its storage {storage_cost}");
    println!("DAO left {} yocto over its storage cost", balance - storage_cost);
    Ok(())
}

/// The minimum is a per-deployment value: a second deployment on the same factory with a higher minimum
/// refuses what the default one accepts, and a minimum under the floor cannot be deployed at all.
#[tokio::test]
async fn minimum_is_set_per_deployment() -> R<()> {
    let w = World::new().await?;
    let wasm = contract_wasm()?;
    let six = "6000000000000000000000000";

    // default deployment (the one World::new made): 5.5
    let v: String = w.view(&w.streak, "get_min_dao_deposit", json!({})).await?;
    assert_eq!(v, "5500000000000000000000000");

    // a deployment with a minimum under the floor is refused at init
    let low = w.user("low-streak").await?;
    let r = w.deploy_at(&low, wasm.clone(), "new", json!({"factory": w.factory, "min_dao_deposit": "1000"})).await?;
    assert!(fail_text(&r).contains("ERR_MIN_DEPOSIT_TOO_SMALL"), "{}", fail_text(&r));

    // a deployment with 6 NEAR
    let alt = w.user("alt-streak").await?;
    ensure_ok(&w.deploy_at(&alt, wasm, "new", json!({"factory": w.factory, "min_dao_deposit": six})).await?, "deploy alt");
    let v: String = w.view(&alt, "get_min_dao_deposit", json!({})).await?;
    assert_eq!(v, six);

    let creator = w.user("alt-creator").await?;
    let fee = w.user("alt-fee").await?;
    // 5.6 NEAR is plenty for the default deployment but under 6 + 0.02 for this one
    let r = w.call(&creator, &alt, "create_pot", pot_args("alt-low", &fee), near(5.6), 300).await?;
    assert!(fail_text(&r).contains("ERR_DEPOSIT_TOO_SMALL"), "{}", fail_text(&r));
    // exactly 6.02 is accepted and the DAO is created
    let r = w.call(&creator, &alt, "create_pot", pot_args("alt-ok", &fee), NearToken::from_yoctonear(6_020_000_000_000_000_000_000_000), 300).await?;
    ensure_ok(&r, "create_pot at the alt minimum");
    w.wait_blocks(10).await?;
    let p: near_sdk::serde_json::Value = w.view(&alt, "get_pot", json!({"pot_id": "alt-ok"})).await?;
    assert_eq!(p["created"], true, "{p}");
    Ok(())
}

/// Redeploying over the previous state layout keeps every pot and adds the minimum (the testnet deployment's path).
#[tokio::test]
async fn migrate_keeps_pots_and_adds_the_minimum() -> R<()> {
    let w = World::new().await?;
    let old_wasm = include_bytes!("fixtures/stakestreak_v1.wasm").to_vec();
    let acct = w.user("migrating").await?;
    ensure_ok(&w.deploy_at(&acct, old_wasm, "new", json!({"factory": w.factory})).await?, "deploy v1");
    let creator = w.user("mig-creator").await?;
    let fee = w.user("mig-fee").await?;
    ensure_ok(&w.call(&creator, &acct, "create_pot", pot_args("mig-pot", &fee), near(5.6), 300).await?, "v1 create_pot");
    w.wait_blocks(10).await?;
    let before: near_sdk::serde_json::Value = w.view(&acct, "get_pot", json!({"pot_id": "mig-pot"})).await?;
    assert_eq!(before["created"], true, "{before}");

    // the new code over the old state; migrate is private, the deploy transaction is signed by the account itself
    ensure_ok(&w.deploy_at(&acct, contract_wasm()?, "migrate", json!({})).await?, "migrate");
    let after: near_sdk::serde_json::Value = w.view(&acct, "get_pot", json!({"pot_id": "mig-pot"})).await?;
    assert_eq!(before, after, "pot unchanged by the migration");
    let v: String = w.view(&acct, "get_min_dao_deposit", json!({})).await?;
    assert_eq!(v, "5500000000000000000000000");
    // and the new rule is live: one yocto under is refused, the least accepted creates a pot
    let r = w.call(&creator, &acct, "create_pot", pot_args("mig-low", &fee), NearToken::from_yoctonear(LEAST - 1), 300).await?;
    assert!(fail_text(&r).contains("ERR_DEPOSIT_TOO_SMALL"), "{}", fail_text(&r));
    ensure_ok(&w.call(&creator, &acct, "create_pot", pot_args("mig-ok", &fee), NearToken::from_yoctonear(LEAST), 300).await?, "create_pot after migrate");
    Ok(())
}
