#!/bin/sh
# The recorded demo pot: creates the pot, joins the scripted members, then hands the day clock to
# run-actors.sh. A human wallet joins and checks in through the web client meanwhile.
#
#   POT=streak-demo-4 CREATOR=a.testnet FEE=f.testnet SETTLER=s.testnet \
#   MEMBERS_ALL="m1.testnet m2.testnet" PLAN="m1.testnet:0,1,2 m2.testnet:0" ./scripts/testnet-demo-pot.sh
#
# Defaults: 0.5 NEAR stake, 3 days of 120 s, 8 minute join window, minimum 3 members, 3% fee.
set -eu
: "${POT:?}" "${CREATOR:?}" "${FEE:?}" "${SETTLER:?}" "${MEMBERS_ALL:?}" "${PLAN:?}"
here=$(cd "$(dirname "$0")" && pwd)
STAKE_YOCTO="${STAKE_YOCTO:-500000000000000000000000}"
DAY_SECS="${DAY_SECS:-120}"
JOIN_SECS="${JOIN_SECS:-480}"
DAYS="${DAYS:-3}"
export LOG="${LOG:-./runner-$POT.log}"
. "$here/send-retry.sh"
ARG="{\"pot_id\":\"$POT\"}"
echo "== create $POT"
send_tx "create_pot" "$CREATOR" create_pot "{\"args\":{\"name\":\"$POT\",\"rule\":\"walk 20 minutes\",\"stake\":\"$STAKE_YOCTO\",\"duration_days\":$DAYS,\"day_secs\":$DAY_SECS,\"join_secs\":$JOIN_SECS,\"min_members\":3,\"fee_account\":\"$FEE\",\"fee_bps\":300,\"freeze_fee\":\"50000000000000000000000\"}}" "5.6 NEAR" "300 Tgas" || { echo "GAVE UP: create_pot"; exit 1; }
sleep 15
echo "== scripted members join"
for m in $MEMBERS_ALL; do send_tx "join $m" "$m" join "$ARG" "0.5 NEAR" "50 Tgas" || echo "GAVE UP: join $m"; done
START_NS=$(near contract call-function as-read-only "$CONTRACT" get_pot json-args "$ARG" network-config testnet now 2>&1 | sed 's/\x1b\[[0-9;]*m//g' | sed -n 's/.*"start_ns": *"\{0,1\}\([0-9]*\).*/\1/p' | head -1)
START=$(( START_NS / 1000000000 ))
echo "START_EPOCH=$START"
echo "JOIN_DEADLINE_UTC=$(date -u -d "@$START" +%H:%M:%S)"
d=0; while [ "$d" -lt "$DAYS" ]; do echo "DAY_${d}_UTC=$(date -u -d "@$(( START + d * DAY_SECS ))" +%H:%M:%S)"; d=$(( d + 1 )); done
echo "END_UTC=$(date -u -d "@$(( START + DAYS * DAY_SECS ))" +%H:%M:%S)"
exec env POT="$POT" START="$START" DAY_SECS="$DAY_SECS" DAYS="$DAYS" SETTLER="$SETTLER" PLAN="$PLAN" LOG="$LOG" "$here/run-actors.sh"
