mod common;
use common::*;
use near_api::NearToken;
use near_sdk::serde_json::{json, Value};

fn amount(p: &Value) -> u128 {
    p["kind"]["Transfer"]["amount"].as_str().unwrap().parse().unwrap()
}
fn receiver(p: &Value) -> String {
    p["kind"]["Transfer"]["receiver_id"].as_str().unwrap().to_string()
}

/// Scene: a whole pot. Two survivors split the pot minus the 3% fee; the DAO policy approves
/// every Transfer; the losers get nothing.
#[tokio::test]
async fn full_pot_pays_survivors_by_policy() -> R<()> {
    let w = World::new().await?;
    let c = w.new_pot("fullpot", 4, 3, 300).await?;
    let (a, b, cc, d) = (&c.members[0], &c.members[1], &c.members[2], &c.members[3]);
    let dao_before = w.balance(&c.dao).await?;

    for day in 0..DAYS {
        w.goto_day(&c.name, day).await?;
        ensure_ok(&w.check_in(a, &c.name).await?, "a check_in");
        ensure_ok(&w.check_in(b, &c.name).await?, "b check_in");
        if day != 1 {
            ensure_ok(&w.check_in(cc, &c.name).await?, "c check_in");
        }
        if day == 0 {
            // d misses day 0 entirely: out from day 1 on
        } else if day == 1 {
            // d would check in now, but is already out
            ensure_ok(&w.check_in(d, &c.name).await?, "d check_in");
        }
    }
    w.goto_day(&c.name, DAYS).await?;
    let p = w.pot(&c.name).await?;
    let alive: Vec<bool> = p["members"].as_array().unwrap().iter().map(|m| m["alive"].as_bool().unwrap()).collect();
    assert_eq!(alive, vec![true, true, false, false], "a,b alive; c missed day 1; d missed day 0");

    let before: Vec<u128> = {
        let mut v = vec![];
        for m in &c.members { v.push(w.balance(m).await?); }
        v
    };
    let fee_before = w.balance(&c.fee).await?;
    w.settle_all(&c, 3).await?;

    // 4 NEAR in, 3% fee = 0.12, 3.88 split between two survivors = 1.94 each
    assert_eq!(w.balance(a).await? - before[0], 1_940_000_000_000_000_000_000_000);
    assert_eq!(w.balance(b).await? - before[1], 1_940_000_000_000_000_000_000_000);
    assert_eq!(w.balance(cc).await?, before[2]);
    assert_eq!(w.balance(d).await?, before[3]);
    assert_eq!(w.balance(&c.fee).await? - fee_before, 120_000_000_000_000_000_000_000);
    // dao_before already held the 4 stakes. The DAO pays 4 NEAR out and burns a little gas on its own callbacks.
    let left = dao_before - w.balance(&c.dao).await?;
    assert!(left <= 4 * STAKE && 4 * STAKE - left < NEAR / 100, "DAO paid out the pot: {left}");

    let props = w.proposals(&c.dao).await?;
    assert_eq!(props.len(), 3, "fee + two winners");
    for p in &props {
        assert_eq!(p["status"], "Approved");
        assert_eq!(p["proposer"], w.streak.to_string());
    }
    let paid: u128 = props.iter().map(amount).sum();
    assert_eq!(paid, 4 * STAKE - 0, "everything the pot held leaves, nothing else");
    let mut to: Vec<String> = props.iter().map(receiver).collect();
    to.sort();
    let mut expect = vec![a.to_string(), b.to_string(), c.fee.to_string()];
    expect.sort();
    assert_eq!(to, expect);
    println!("settled: {} proposals, gas-tight batch 3", props.len());
    Ok(())
}

/// Scene: the clock is the judge. Missing a day puts you out even if you check in afterwards.
#[tokio::test]
async fn missed_day_is_out_by_contract_clock() -> R<()> {
    let w = World::new().await?;
    let c = w.new_pot("clockpot", 3, 3, 300).await?;
    let (a, b, cc) = (&c.members[0], &c.members[1], &c.members[2]);

    // before the start nobody can check in
    let r = w.check_in(a, &c.name).await?;
    assert!(!r.is_success() && fail_text(&r).contains("ERR_NOT_STARTED"), "{}", fail_text(&r));
    // a stranger cannot
    w.goto_day(&c.name, 0).await?;
    let r = w.check_in(&c.rando, &c.name).await?;
    assert!(fail_text(&r).contains("ERR_NOT_MEMBER"), "{}", fail_text(&r));
    // a and b check in on day 0, twice is harmless
    for who in [a, b] {
        ensure_ok(&w.check_in(who, &c.name).await?, "day 0");
    }
    ensure_ok(&w.check_in(a, &c.name).await?, "day 0 again");

    // day 1: c never checked in on day 0, so c is out as soon as day 1 begins
    w.goto_day(&c.name, 1).await?;
    let alive = |p: &Value| -> Vec<bool> { p["members"].as_array().unwrap().iter().map(|m| m["alive"].as_bool().unwrap()).collect() };
    let p = w.pot(&c.name).await?;
    assert_eq!(alive(&p), vec![true, true, false]);
    // c checking in late does not bring c back
    ensure_ok(&w.check_in(cc, &c.name).await?, "late c");
    ensure_ok(&w.check_in(a, &c.name).await?, "a day 1");
    let p = w.pot(&c.name).await?;
    assert_eq!(alive(&p), vec![true, true, false]);

    // day 2: b skipped day 1, b is out; a is the only one left
    w.goto_day(&c.name, 2).await?;
    let p = w.pot(&c.name).await?;
    assert_eq!(alive(&p), vec![true, false, false]);
    assert_eq!(p["day"], 2);

    // after the last day nobody can check in, and joining is long closed
    w.goto_day(&c.name, DAYS).await?;
    let r = w.check_in(a, &c.name).await?;
    assert!(fail_text(&r).contains("ERR_POT_ENDED"), "{}", fail_text(&r));
    let late = w.user("late").await?;
    let r = w.call(&late, &w.streak, "join", json!({"pot_id": c.name}), NearToken::from_yoctonear(STAKE), 60).await?;
    assert!(fail_text(&r).contains("ERR_JOIN_CLOSED"), "{}", fail_text(&r));
    Ok(())
}

/// Scene: change your mind before the start. The refund is a Transfer proposal the policy approves.
#[tokio::test]
async fn leave_before_start_refunds_in_full() -> R<()> {
    let w = World::new().await?;
    let c = w.new_pot("leavepot", 3, 3, 300).await?;
    let quitter = &c.members[2];
    let before = w.balance(quitter).await?;
    let r = w.call(quitter, &w.streak, "leave", json!({"pot_id": c.name}), near(0.0), 300).await?;
    ensure_ok(&r, "leave");
    w.wait_blocks(15).await?;
    let gained = w.balance(quitter).await? + 0 - before;
    // the stake is back, minus the gas the quitter paid for the leave call
    assert!(gained <= STAKE && STAKE - gained < NEAR / 50, "refund {gained}");
    let props = w.proposals(&c.dao).await?;
    assert_eq!(props.len(), 1);
    assert_eq!(props[0]["status"], "Approved");
    assert_eq!(amount(&props[0]), STAKE, "full refund, no fee");
    let p = w.pot(&c.name).await?;
    assert_eq!(p["members"].as_array().unwrap().len(), 2);

    // once the pot has started nobody can leave
    w.goto_day(&c.name, 0).await?;
    let r = w.call(&c.members[0], &w.streak, "leave", json!({"pot_id": c.name}), near(0.0), 300).await?;
    assert!(fail_text(&r).contains("ERR_ALREADY_STARTED"), "{}", fail_text(&r));
    Ok(())
}

/// Scene: not enough friends. A pot that starts below its minimum refunds everyone, no fee.
#[tokio::test]
async fn unfilled_pot_refunds_everyone_without_fee() -> R<()> {
    let w = World::new().await?;
    let c = w.new_pot("emptypot", 2, 3, 300).await?;
    // too early to settle
    let r = w.call(&c.rando, &w.streak, "settle", json!({"pot_id": c.name}), near(0.0), 300).await?;
    assert!(fail_text(&r).contains("ERR_NOT_ENDED"), "{}", fail_text(&r));
    w.goto_day(&c.name, 0).await?;
    let before = [w.balance(&c.members[0]).await?, w.balance(&c.members[1]).await?];
    w.settle_all(&c, 3).await?;
    assert_eq!(w.balance(&c.members[0]).await? - before[0], STAKE);
    assert_eq!(w.balance(&c.members[1]).await? - before[1], STAKE);
    let props = w.proposals(&c.dao).await?;
    assert_eq!(props.len(), 2);
    assert!(props.iter().all(|p| p["status"] == "Approved" && amount(p) == STAKE));
    Ok(())
}

/// Scene: everybody dropped out. Each funded member gets the stake back minus the fee, and the
/// fees go to the fee account as one Transfer (no sweep of the whole pot).
#[tokio::test]
async fn no_survivors_refunds_stake_minus_fee() -> R<()> {
    let w = World::new().await?;
    let c = w.new_pot("losers", 3, 3, 300).await?;
    w.goto_day(&c.name, DAYS).await?;
    let fee_before = w.balance(&c.fee).await?;
    let before: Vec<u128> = {
        let mut v = vec![];
        for m in &c.members {
            v.push(w.balance(m).await?);
        }
        v
    };
    w.settle_all(&c, 3).await?;
    let fee_each = STAKE * 3 / 100;
    assert_eq!(w.balance(&c.fee).await? - fee_before, 3 * fee_each, "fees to the fee account");
    for (i, m) in c.members.iter().enumerate() {
        assert_eq!(w.balance(m).await? - before[i], STAKE - fee_each, "refund of stake minus fee");
    }
    let props = w.proposals(&c.dao).await?;
    assert_eq!(props.len(), 4, "one fee transfer and three refunds");
    assert_eq!(receiver(&props[0]), c.fee.to_string());
    Ok(())
}

/// Scene: streak freeze. Buy one, it covers yesterday only, at most two, and it cannot revive an
/// older miss.
#[tokio::test]
async fn freeze_covers_yesterday_only() -> R<()> {
    let w = World::new().await?;
    let c = w.new_pot("freezepot", 3, 3, 300).await?;
    let (a, b) = (&c.members[0], &c.members[1]);
    let fee = NearToken::from_yoctonear(NEAR / 10);
    w.goto_day(&c.name, 0).await?;
    for m in &c.members {
        ensure_ok(&w.check_in(m, &c.name).await?, "day 0");
    }
    // nothing to freeze on day 0
    let r = w.call(a, &w.streak, "use_freeze", json!({"pot_id": c.name}), fee, 60).await?;
    assert!(fail_text(&r).contains("ERR_NOTHING_TO_FREEZE"), "{}", fail_text(&r));
    // day 1: a and c check in, b does not
    w.goto_day(&c.name, 1).await?;
    ensure_ok(&w.check_in(a, &c.name).await?, "a day 1");
    ensure_ok(&w.check_in(&c.members[2], &c.name).await?, "c day 1");
    // day 2: b buys a freeze for day 1 and is alive again
    w.goto_day(&c.name, 2).await?;
    let p = w.pot(&c.name).await?;
    assert_eq!(p["members"][1]["alive"], false);
    let fee_before = w.balance(&c.fee).await?;
    let r = w.call(b, &w.streak, "use_freeze", json!({"pot_id": c.name}), fee, 60).await?;
    ensure_ok(&r, "freeze");
    w.wait_blocks(5).await?;
    let p = w.pot(&c.name).await?;
    assert_eq!(p["members"][1]["alive"], true);
    assert_eq!(w.balance(&c.fee).await? - fee_before, NEAR / 10, "freeze fee goes to the fee account");
    // a was not missing yesterday: nothing to freeze
    let r = w.call(a, &w.streak, "use_freeze", json!({"pot_id": c.name}), fee, 60).await?;
    assert!(fail_text(&r).contains("ERR_YESTERDAY_NOT_MISSED"), "{}", fail_text(&r));
    // wrong fee is refused
    let r = w.call(b, &w.streak, "use_freeze", json!({"pot_id": c.name}), near(0.01), 60).await?;
    assert!(!r.is_success());
    Ok(())
}
