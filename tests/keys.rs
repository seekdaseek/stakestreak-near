mod common;
use common::*;
use near_api::types::{AccessKeyPermission, transaction::actions::FunctionCallPermission};
use near_api::NearToken;
use near_sdk::serde_json::json;

/// Scene: check in without a wallet pop-up. A function-call key limited to `check_in` on our
/// contract does the daily check-in; it can do nothing else.
#[tokio::test]
async fn check_in_key_can_only_check_in() -> R<()> {
    let w = World::new().await?;
    let c = w.new_pot("keypot", 3, 3, 300).await?;
    let me = &c.members[0];
    let sk = near_api::signer::generate_secret_key()?;
    let key_signer = near_api::Signer::from_secret_key(sk.clone())?;
    let perm = AccessKeyPermission::FunctionCall(FunctionCallPermission {
        allowance: Some(NearToken::from_near(1)),
        receiver_id: w.streak.to_string(),
        method_names: vec!["check_in".to_string()],
    });
    near_api::Account(me.clone())
        .add_key(perm, sk.public_key())
        .with_signer(w.signer.clone())
        .send_to(&w.net)
        .await?
        .assert_success();

    w.goto_day(&c.name, 0).await?;
    let call = |method: &str, deposit: NearToken| {
        let net = w.net.clone();
        let (me, streak, key_signer, name) = (me.clone(), w.streak.clone(), key_signer.clone(), c.name.clone());
        let method = method.to_string();
        async move {
            near_api::Contract(streak)
                .call_function(&method, json!({"pot_id": name}))
                .transaction()
                .deposit(deposit)
                .gas(near_api::NearGas::from_tgas(30))
                .with_signer(me, key_signer)
                .send_to(&net)
                .await
        }
    };
    // the limited key checks in
    let ok = call("check_in", NearToken::from_yoctonear(0)).await?;
    assert!(ok.assert_success().logs().iter().any(|l| l.contains("checkin pot=keypot")));
    let p = w.pot(&c.name).await?;
    assert_eq!(p["members"][0]["checkins"], 1);

    // it cannot call any other method, attach a deposit, or move money
    let e = call("leave", NearToken::from_yoctonear(0)).await.err().map(|e| e.to_string()).unwrap_or_default();
    // near-api drops the node's reason text; the same call with the method swapped back succeeded above,
    // so the method is the only difference. Verbatim node messages are in docs/RESULTS.md (testnet).
    assert!(e.contains("InvalidTransaction"), "leave: {e}");
    let e = call("check_in", NearToken::from_yoctonear(1)).await.err().map(|e| e.to_string()).unwrap_or_default();
    assert!(e.contains("InvalidTransaction"), "deposit: {e}");
    let e = near_api::Tokens::account(me.clone())
        .send_to(c.fee.clone())
        .near(NearToken::from_yoctonear(1))
        .with_signer(key_signer.clone())
        .send_to(&w.net)
        .await
        .err()
        .map(|e| e.to_string())
        .unwrap_or_default();
    assert!(e.contains("InvalidTransaction"), "a transfer signed with the check-in key must be refused: {e}");
    println!("refusals: leave / deposit / transfer all refused at validation");
    Ok(())
}
