mod common;
use common::*;
use near_api::NearToken;
use near_sdk::serde_json::{json, Value};

/// The control: the same pot as a plain Sputnik multisig (2 votes of 3 members). Members have to
/// vote the payout by hand, which is exactly what a loser has no reason to do.
async fn control(w: &World) -> R<(near_api::AccountId, Vec<near_api::AccountId>)> {
    let creator = w.user("ctl-creator").await?;
    let mut members = vec![];
    for i in 0..3 {
        members.push(w.user(&format!("ctl-m{i}")).await?);
    }
    let policy = json!({
        "roles": [{
            "name": "council", "kind": {"Group": members},
            "permissions": ["*:AddProposal", "*:VoteApprove", "*:VoteReject", "*:VoteRemove", "*:Finalize"],
            "vote_policy": {}
        }],
        "default_vote_policy": {"weight_kind": "RoleWeight", "quorum": "0", "threshold": "2"},
        "proposal_bond": "0", "proposal_period": "86400000000000",
        "bounty_bond": "0", "bounty_forgiveness_period": "0"
    });
    let dao_args = json!({"config": {"name": "control", "purpose": "plain multisig", "metadata": ""}, "policy": policy});
    let b64 = near_sdk::json_types::Base64VecU8::from(near_sdk::serde_json::to_vec(&dao_args)?);
    let r = w.call(&creator, &w.factory, "create", json!({"name": "control", "args": b64}), near(5.6), 300).await?;
    ensure_ok(&r, "factory.create control");
    w.wait_blocks(10).await?;
    let dao: near_api::AccountId = format!("control.{}", w.factory).parse()?;
    // each member stakes 1 NEAR into the treasury
    for m in &members {
        near_api::Tokens::account(m.clone())
            .send_to(dao.clone())
            .near(NearToken::from_yoctonear(STAKE))
            .with_signer(w.signer.clone())
            .send_to(&w.net)
            .await?
            .assert_success();
    }
    Ok((dao, members))
}

async fn propose_payout(w: &World, dao: &near_api::AccountId, winner: &near_api::AccountId) -> R<Value> {
    let kind = json!({"Transfer": {"token_id": "", "receiver_id": winner, "amount": (3 * STAKE).to_string(), "msg": null}});
    ensure_ok(&w.call(winner, dao, "add_proposal", json!({"proposal": {"description": "pay the survivor", "kind": kind}}), near(0.0), 30).await?, "add_proposal");
    // the proposer votes yes
    ensure_ok(&w.call(winner, dao, "act_proposal", json!({"id": 0, "action": "VoteApprove", "proposal": kind, "memo": null}), near(0.0), 100).await?, "winner votes");
    Ok(kind)
}

/// Scene: the control stalls. One survivor, two losers who do not vote: the payout never happens.
#[tokio::test]
async fn control_multisig_stalls_when_losers_refuse() -> R<()> {
    let w = World::new().await?;
    let (dao, members) = control(&w).await?;
    let winner = &members[0];
    let before = w.balance(winner).await?;
    propose_payout(&w, &dao, winner).await?;
    w.wait_blocks(600).await?; // about three minutes of chain time; the losers do nothing
    let p: Value = w.view(&dao, "get_proposal", json!({"id": 0})).await?;
    assert_eq!(p["status"], "InProgress", "one vote of the two needed: stalled");
    assert!(w.balance(winner).await? <= before, "the survivor has not been paid");
    assert!(w.balance(&dao).await? >= 3 * STAKE, "the money is still locked in the treasury");
    println!("control: after 600 blocks status={} votes={}", p["status"], p["vote_counts"]);
    Ok(())
}

/// A named test that the control itself works: when a second member does vote, it pays.
#[tokio::test]
async fn control_multisig_pays_when_everyone_votes() -> R<()> {
    let w = World::new().await?;
    let (dao, members) = control(&w).await?;
    let winner = &members[0];
    let before = w.balance(winner).await?;
    let kind = propose_payout(&w, &dao, winner).await?;
    ensure_ok(&w.call(&members[1], &dao, "act_proposal", json!({"id": 0, "action": "VoteApprove", "proposal": kind, "memo": null}), near(0.0), 100).await?, "second vote");
    w.wait_blocks(15).await?;
    let p: Value = w.view(&dao, "get_proposal", json!({"id": 0})).await?;
    assert_eq!(p["status"], "Approved");
    let gained = w.balance(winner).await? - before;
    assert!(gained > 3 * STAKE - NEAR / 20 && gained <= 3 * STAKE, "winner got the pot: {gained}");
    Ok(())
}
