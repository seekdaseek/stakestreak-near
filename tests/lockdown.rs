mod common;
use common::*;
use near_sdk::serde_json::{json, Value};

/// Scene: read the policy. One role, one member (our contract), three permissions, no bond, no
/// access keys on the DAO account, and members have no way to touch the treasury.
#[tokio::test]
async fn dao_policy_leaves_one_way_out() -> R<()> {
    let w = World::new().await?;
    let c = w.new_pot("lockpot", 3, 3, 300).await?;
    let policy: Value = w.view(&c.dao, "get_policy", json!({})).await?;
    let roles = policy["roles"].as_array().unwrap();
    assert_eq!(roles.len(), 1);
    assert_eq!(roles[0]["kind"], json!({"Group": [w.streak.to_string()]}));
    let mut perms: Vec<&str> = roles[0]["permissions"].as_array().unwrap().iter().map(|p| p.as_str().unwrap()).collect();
    perms.sort();
    assert_eq!(perms, vec!["transfer:AddProposal", "transfer:Finalize", "transfer:VoteApprove"]);
    assert_eq!(policy["proposal_bond"], "0");

    let keys = near_api::Account(c.dao.clone()).list_keys().fetch_from(&w.net).await?;
    assert_eq!(keys.data.len(), 0, "nobody holds a key to the DAO account");

    // members cannot file anything, of any kind
    let m = &c.members[0];
    for kind in [
        json!({"Transfer": {"token_id": "", "receiver_id": m, "amount": "1", "msg": null}}),
        json!("Vote"),
        json!({"AddMemberToRole": {"member_id": m, "role": "settler"}}),
    ] {
        let r = w.call(m, &c.dao, "add_proposal", json!({"proposal": {"description": "x", "kind": kind}}), near(0.0), 30).await?;
        assert!(fail_text(&r).contains("ERR_PERMISSION_DENIED"), "{}", fail_text(&r));
    }
    // the stake sits in the treasury: three stakes plus what the DAO was created with
    assert!(w.balance(&c.dao).await? >= 3 * STAKE + 5 * NEAR);
    Ok(())
}
