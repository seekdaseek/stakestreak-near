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
