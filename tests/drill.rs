mod common;
use common::*;
use near_sdk::serde_json::{json, Value};
use std::collections::HashMap;

fn tr(p: &Value) -> (String, u128) {
    (
        p["kind"]["Transfer"]["receiver_id"].as_str().unwrap().to_string(),
        p["kind"]["Transfer"]["amount"].as_str().unwrap().parse().unwrap(),
    )
}

/// Destructive drill: the settler dies mid-settlement and two callers race on the same payout.
/// Money still goes only to survivors and the fee account, once each, exactly the pot.
#[tokio::test]
async fn settler_crash_mid_settlement_pays_only_survivors_once() -> R<()> {
    let w = World::new().await?;
    let c = w.new_pot("drillpot", 5, 3, 300).await?;
    let loser = &c.members[4];
    for day in 0..DAYS {
        w.goto_day(&c.name, day).await?;
        for m in &c.members[..4] {
            ensure_ok(&w.check_in(m, &c.name).await?, "check_in");
        }
        if day == 0 {
            ensure_ok(&w.check_in(loser, &c.name).await?, "loser day 0");
        }
    }
    w.goto_day(&c.name, DAYS).await?;
    let before: Vec<u128> = {
        let mut v = vec![];
        for m in &c.members { v.push(w.balance(m).await?); }
        v
    };
    let fee_before = w.balance(&c.fee).await?;

    // Settler one files and approves one payout, then is gone.
    let one = w.user("settler-one").await?;
    ensure_ok(&w.call(&one, &w.streak, "settle", json!({"pot_id": c.name, "batch": 1}), near(0.0), 300).await?, "first settle");
    w.wait_blocks(12).await?;
    let p = w.pot(&c.name).await?;
    assert_eq!(p["payouts"].as_array().unwrap().len(), 5, "fee + four winners are fixed");
    assert_eq!(p["payouts_pending"], 4, "one of five done, the settler stops here");

    // Two callers race on the same unpaid payout (index 2).
    let (a, b) = (w.user("racer-a").await?, w.user("racer-b").await?);
    let (r1, r2) = tokio::join!(
        w.call(&a, &w.streak, "pay", json!({"pot_id": c.name, "index": 2}), near(0.0), 200),
        w.call(&b, &w.streak, "pay", json!({"pot_id": c.name, "index": 2}), near(0.0), 200),
    );
    println!("race: a={} b={}", r1?.is_success(), r2?.is_success());
    w.wait_blocks(15).await?;

    // Somebody else finishes the job.
    w.settle_all(&c, 2).await?;
    // Settling again changes nothing.
    ensure_ok(&w.call(&one, &w.streak, "settle", json!({"pot_id": c.name}), near(0.0), 300).await?, "settle again");
    w.wait_blocks(10).await?;

    // Invariants, read from the DAO.
    let props = w.proposals(&c.dao).await?;
    let approved: Vec<&Value> = props.iter().filter(|p| p["status"] == "Approved").collect();
    let unapproved = props.len() - approved.len();
    println!("proposals filed: {}, approved: {}, left unapproved (never paid): {}", props.len(), approved.len(), unapproved);
    let mut per: HashMap<String, u128> = HashMap::new();
    for p in &approved {
        let (to, amt) = tr(p);
        *per.entry(to).or_default() += amt;
    }
    assert_eq!(approved.len(), 5, "exactly one approved transfer per payout");
    assert_eq!(per.values().sum::<u128>(), 5 * STAKE, "the whole pot, nothing more");
    assert_eq!(per.get(&loser.to_string()), None, "the loser is never a receiver");
    let share = (5 * STAKE - 5 * STAKE * 3 / 100) / 4;
    for m in &c.members[..4] {
        assert_eq!(per[&m.to_string()], share);
        assert_eq!(w.balance(m).await? - before[c.members.iter().position(|x| x == m).unwrap()], share);
    }
    assert_eq!(w.balance(loser).await?, before[4]);
    assert_eq!(w.balance(&c.fee).await? - fee_before, 5 * STAKE * 3 / 100);

    // The loser cannot push anything through by hand.
    let r = w
        .call(loser, &c.dao, "add_proposal", json!({"proposal": {"description": "mine", "kind": {"Transfer": {"token_id": "", "receiver_id": loser, "amount": STAKE.to_string(), "msg": null}}}}), near(0.0), 30)
        .await?;
    assert!(fail_text(&r).contains("ERR_PERMISSION_DENIED"), "{}", fail_text(&r));
    // ... nor approve what exists, nor poke the companion with a bad index.
    let r = w
        .call(loser, &c.dao, "act_proposal", json!({"id": 0, "action": "VoteReject", "proposal": props[0]["kind"], "memo": null}), near(0.0), 30)
        .await?;
    assert!(fail_text(&r).contains("ERR_PERMISSION_DENIED"), "{}", fail_text(&r));
    let r = w.call(loser, &w.streak, "pay", json!({"pot_id": c.name, "index": 99}), near(0.0), 100).await?;
    assert!(fail_text(&r).contains("ERR_NO_PAYOUT"), "{}", fail_text(&r));
    Ok(())
}

/// A payee deletes their account before settlement. Their transfer fails in the DAO (proposal
/// `Failed`), the others are paid anyway, and once the account exists again `retry` pays it.
#[tokio::test]
async fn payee_with_deleted_account_is_paid_after_retry() -> R<()> {
    let w = World::new().await?;
    let c = w.new_pot("ghostpot", 3, 3, 0).await?;
    for day in 0..DAYS {
        w.goto_day(&c.name, day).await?;
        for m in &c.members {
            ensure_ok(&w.check_in(m, &c.name).await?, "check_in");
        }
    }
    let ghost = c.members[2].clone();
    near_api::Account(ghost.clone())
        .delete_account_with_beneficiary(c.rando.clone())
        .with_signer(w.signer.clone())
        .send_to(&w.net)
        .await?
        .assert_success();
    w.goto_day(&c.name, DAYS).await?;
    let before = [w.balance(&c.members[0]).await?, w.balance(&c.members[1]).await?];
    w.settle_all(&c, 3).await?;
    assert_eq!(w.balance(&c.members[0]).await? - before[0], STAKE);
    assert_eq!(w.balance(&c.members[1]).await? - before[1], STAKE);
    let props = w.proposals(&c.dao).await?;
    let failed: Vec<&Value> = props.iter().filter(|p| p["status"] == "Failed").collect();
    assert_eq!(failed.len(), 1, "only the ghost's transfer failed: {props:?}");
    assert_eq!(tr(failed[0]).0, ghost.to_string());

    // while the account is gone, retry changes nothing and nobody else can take the money
    let idx = w.pot(&c.name).await?["payouts"].as_array().unwrap().iter().position(|p| p["to"] == ghost.to_string()).unwrap();
    let r = w.call(&c.rando, &w.streak, "retry", json!({"pot_id": c.name, "index": idx}), near(0.0), 100).await?;
    w.wait_blocks(12).await?;
    println!("retry while gone: success={}", r.is_success());
    assert!(w.balance(&c.dao).await? >= STAKE, "the ghost's share is still in the treasury");

    // the name is claimed again; retry now pays
    w.create_account(&ghost, 1).await?;
    let before_ghost = w.balance(&ghost).await?;
    ensure_ok(&w.call(&c.rando, &w.streak, "retry", json!({"pot_id": c.name, "index": idx}), near(0.0), 100).await?, "retry");
    w.wait_blocks(12).await?;
    assert_eq!(w.balance(&ghost).await? - before_ghost, STAKE);
    let props = w.proposals(&c.dao).await?;
    assert!(props.iter().all(|p| p["status"] == "Approved"));
    // a healthy payout cannot be paid twice by retry
    let healthy = if idx == 0 { 1 } else { 0 };
    let _ = w.call(&c.rando, &w.streak, "retry", json!({"pot_id": c.name, "index": healthy}), near(0.0), 100).await?;
    w.wait_blocks(12).await?;
    assert_eq!(w.balance(&c.members[0]).await? - before[0], STAKE);
    Ok(())
}
