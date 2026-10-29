#!/bin/sh
# The control on testnet: the same 3-friend pot as a plain Sputnik multisig (2 votes of 3, no
# companion contract). One member "survives" and proposes the payout, the two others do nothing:
# the proposal stays InProgress and the money stays in the treasury. Then one more member votes
# and it pays, which shows the control itself works. Needs `near` and the legacy keychain.
#
#   NAME=streak-control-1 CREATOR=a.testnet WINNER=b.testnet L1=c.testnet L2=d.testnet \
#   ./scripts/testnet-control.sh
set -eu
: "${NAME:?}" "${CREATOR:?}" "${WINNER:?}" "${L1:?}" "${L2:?}"
here=$(cd "$(dirname "$0")" && pwd)
FACTORY="${FACTORY:-sputnikv2.testnet}"
STAKE_NEAR="${STAKE_NEAR:-0.5}"
WAIT_SECS="${WAIT_SECS:-180}"
export LOG="${LOG:-./runner-$NAME.log}"
. "$here/send-retry.sh"
DAO="$NAME.$FACTORY"
view() { near contract call-function as-read-only "$DAO" "$1" json-args "$2" network-config testnet now 2>&1 |
  sed -n '/printed to stdout/,$p' | sed '1d' | sed '/Here is your console/,$d'; }

CREATE_ARGS=$(python3 - <<PY
import json,base64
members=["$WINNER","$L1","$L2"]
policy={"roles":[{"name":"council","kind":{"Group":members},
  "permissions":["*:AddProposal","*:VoteApprove","*:VoteReject","*:VoteRemove","*:Finalize"],"vote_policy":{}}],
  "default_vote_policy":{"weight_kind":"RoleWeight","quorum":"0","threshold":"2"},
  "proposal_bond":"0","proposal_period":"86400000000000","bounty_bond":"0","bounty_forgiveness_period":"0"}
cfg={"config":{"name":"$NAME","purpose":"control: plain 2-of-3 multisig","metadata":""},"policy":policy}
print(json.dumps({"name":"$NAME","args":base64.b64encode(json.dumps(cfg).encode()).decode()}))
PY
)
echo "== create control DAO $DAO (2 of 3 council)"
CONTRACT=$FACTORY send_tx "create $NAME" "$CREATOR" create "$CREATE_ARGS" "${DEPOSIT:-5.6} NEAR" "300 Tgas"
sleep 12
echo "== each member sends $STAKE_NEAR NEAR to the treasury"
for m in "$WINNER" "$L1" "$L2"; do
  out=$(near tokens "$m" send-near "$DAO" "$STAKE_NEAR NEAR" network-config testnet sign-with-legacy-keychain send 2>&1) || true
  id=$(printf '%s\n' "$out" | sed -n 's/.*Transaction ID: *\([A-Za-z0-9]*\).*/\1/p' | head -1)
  printf '%s stake %-30s %s\n' "$(date -u +%H:%M:%S)" "$m" "${id:+$EXPLORER/$id}"
  printf '%s\n' "$out" >> "$LOG"
done
TOTAL=$(python3 -c "print(int(round(3*float('$STAKE_NEAR')*10**6))*10**18)")
KIND="{\"Transfer\":{\"token_id\":\"\",\"receiver_id\":\"$WINNER\",\"amount\":\"$TOTAL\",\"msg\":null}}"
echo "== survivor proposes the payout of the whole pot to itself, and votes yes"
CONTRACT=$DAO send_tx "add_proposal" "$WINNER" add_proposal "{\"proposal\":{\"description\":\"pay the survivor\",\"kind\":$KIND}}" "0 NEAR" "30 Tgas"
CONTRACT=$DAO send_tx "vote approve (survivor)" "$WINNER" act_proposal "{\"id\":0,\"action\":\"VoteApprove\",\"proposal\":$KIND,\"memo\":null}" "0 NEAR" "100 Tgas"
echo "== the two losers do nothing for ${WAIT_SECS}s"
sleep "$WAIT_SECS"
echo "-- proposal 0 now:"; view get_proposal '{"id":0}' | grep -E '"status"|"vote_counts"|"votes"|"amount"|"receiver_id"' || true
echo "-- treasury balance (NEAR, yocto): $(near account view-account-summary "$DAO" network-config testnet now 2>&1 | grep -i 'native account balance' | head -1)"
echo "== control check: one loser votes, it pays"
CONTRACT=$DAO send_tx "vote approve ($L1)" "$L1" act_proposal "{\"id\":0,\"action\":\"VoteApprove\",\"proposal\":$KIND,\"memo\":null}" "0 NEAR" "100 Tgas"
sleep 20
echo "-- proposal 0 after the second vote:"; view get_proposal '{"id":0}' | grep -E '"status"' || true
echo FINISHED
