//! The one-command demo: a whole pot told step by step on a local near-sandbox chain.
//!
//!   cargo test --test demo -- --ignored --nocapture
//!
//! Four friends, 1 NEAR each, three 2-minute days (chain clock), 3% fee. It is `#[ignore]`d so a plain
//! `cargo test` stays about the same length; the story is the same code path the real tests use.
mod common;
use common::*;
use near_sdk::serde_json::{json, Value};

fn near_str(y: u128) -> String {
    format!("{:.4}", y as f64 / 1e24)
}

async fn board(w: &World, name: &str, title: &str) -> R<()> {
    let p = w.pot(name).await?;
    println!("\n-- {title}");
    for (i, m) in p["members"].as_array().unwrap().iter().enumerate() {
        let who = ["Ana", "Bo", "Cy", "Dee"][i];
        let days: u64 = m["days"].as_str().unwrap().parse().unwrap();
        let bits: String = (0..DAYS).map(|d| if days >> d & 1 == 1 { 'x' } else { '.' }).collect();
        println!("   {who:<4} days [{bits}]  checkins {}  {}", m["checkins"], if m["alive"].as_bool().unwrap() { "in" } else { "OUT" });
    }
    Ok(())
}

#[tokio::test]
#[ignore]
async fn story_of_one_pot() -> R<()> {
    let w = World::new().await?;
    println!("StakeStreak on NEAR: one pot, start to finish (local sandbox, real Sputnik factory and DAO code)");
    let c = w.new_pot("story", 4, 3, 300).await?;
    let (ana, bo, cy, dee) = (&c.members[0], &c.members[1], &c.members[2], &c.members[3]);
    let names = ["Ana", "Bo", "Cy", "Dee"];
    println!("pot DAO: {}   stake 1 NEAR each, {} members, DAO holds {} NEAR", c.dao, c.members.len(), near_str(w.balance(&c.dao).await?));
    let p: Value = w.view(&c.dao, "get_policy", json!({})).await?;
    println!("DAO policy: roles = {}, only role = '{}', permissions = {}", p["roles"].as_array().unwrap().len(), p["roles"][0]["name"].as_str().unwrap(), p["roles"][0]["permissions"]);

    let dao_before = w.balance(&c.dao).await?;
    let mut before = vec![];
    for m in &c.members { before.push(w.balance(m).await?); }
    let fee_before = w.balance(&c.fee).await?;

    for day in 0..DAYS {
        w.goto_day(&c.name, day).await?;
        ensure_ok(&w.check_in(ana, &c.name).await?, "ana");
        ensure_ok(&w.check_in(bo, &c.name).await?, "bo");
        if day != 1 { ensure_ok(&w.check_in(cy, &c.name).await?, "cy"); }
        if day == 1 { ensure_ok(&w.check_in(dee, &c.name).await?, "dee"); }
        board(&w, &c.name, &format!("end of day {day} check-ins (Cy skips day 1, Dee skips day 0)")).await?;
    }
    w.goto_day(&c.name, DAYS).await?;
    board(&w, &c.name, "pot ended").await?;

    println!("\n-- settle (anyone may call it; here a bystander)");
    w.settle_all(&c, 3).await?;
    for p in w.proposals(&c.dao).await? {
        let to = p["kind"]["Transfer"]["receiver_id"].as_str().unwrap().to_string();
        let who = c.members.iter().position(|m| m.to_string() == to).map(|i| names[i].to_string()).unwrap_or_else(|| "fee account".into());
        let amt: u128 = p["kind"]["Transfer"]["amount"].as_str().unwrap().parse().unwrap();
        println!("   proposal #{} {:<12} {} NEAR  {}  proposed by the contract", p["id"], who, near_str(amt), p["status"].as_str().unwrap());
    }
    println!("\n-- balances after (change)");
    for (i, m) in c.members.iter().enumerate() {
        let now = w.balance(m).await?;
        println!("   {:<4} {:+.4}", names[i], (now as f64 - before[i] as f64) / 1e24);
    }
    println!("   fee  {:+.4}", (w.balance(&c.fee).await? as f64 - fee_before as f64) / 1e24);
    println!("   DAO  paid out {} NEAR (the pot) and kept its creation storage", near_str(dao_before - w.balance(&c.dao).await?));
    Ok(())
}
