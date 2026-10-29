#!/bin/sh
# One testnet pot that shows two things the other pots do not: a member who LEAVES before the start
# (full refund, no fee) and a member who buys a streak FREEZE (fee to the fee account, covers
# yesterday only) and so stays alive. Everything is scripted, no wallet. Run it on a server (RUNBOOK.md).
#
#   POT=streak-demo-3 CREATOR=a.testnet FEE=f.testnet M1=.. M2=.. M3=.. M4=.. ./scripts/testnet-freeze-leave.sh
#
# Plan (4 members join, 3 needed): M4 leaves before the start. Day 0: M1 M2 M3 check in. Day 1: M1 and
# M2 check in, M3 does not. Day 2: M3 buys a freeze (covers day 1) and checks in, M1 and M2 check in.
# All three end alive; expected: fee 3% of 1.5 NEAR to the fee account (plus the freeze fee), 0.485 NEAR each.
set -eu
: "${POT:?}" "${CREATOR:?}" "${FEE:?}" "${M1:?}" "${M2:?}" "${M3:?}" "${M4:?}"
here=$(cd "$(dirname "$0")" && pwd)
STAKE_YOCTO="${STAKE_YOCTO:-500000000000000000000000}"
FREEZE_YOCTO="${FREEZE_YOCTO:-50000000000000000000000}"
DAY_SECS="${DAY_SECS:-120}"
JOIN_SECS="${JOIN_SECS:-150}"
export LOG="${LOG:-./runner-$POT.log}"
. "$here/send-retry.sh"
sleep_until() { now=$(date +%s); [ "$1" -gt "$now" ] && sleep $(( $1 - now )); return 0; }
pot() { near contract call-function as-read-only "$CONTRACT" get_pot json-args "{\"pot_id\":\"$POT\"}" network-config testnet now 2>&1 | sed 's/\x1b\[[0-9;]*m//g'; }
step() { send_tx "$@" || { echo "GAVE UP: $1 (see $LOG)"; GAVE=$(( ${GAVE:-0} + 1 )); }; }
ARG="{\"pot_id\":\"$POT\"}"

echo "== create $POT"
step "create_pot" "$CREATOR" create_pot "{\"args\":{\"name\":\"$POT\",\"rule\":\"walk 20 minutes\",\"stake\":\"$STAKE_YOCTO\",\"duration_days\":3,\"day_secs\":$DAY_SECS,\"join_secs\":$JOIN_SECS,\"min_members\":3,\"fee_account\":\"$FEE\",\"fee_bps\":300,\"freeze_fee\":\"$FREEZE_YOCTO\"}}" "5.6 NEAR" "300 Tgas"
sleep 15
echo "== join"
for m in "$M1" "$M2" "$M3" "$M4"; do step "join $m" "$m" join "$ARG" "0.5 NEAR" "50 Tgas"; done
START=$(( $(pot | sed -n 's/.*"start_ns": *"\{0,1\}\([0-9]*\).*/\1/p' | head -1) / 1000000000 ))
echo "== pot starts at epoch $START ($(date -u -r "$START" +%H:%M:%S 2>/dev/null || date -u -d "@$START" +%H:%M:%S) UTC)"
echo "== M4 leaves before the start"
[ "$(date +%s)" -lt "$START" ] || echo "WARNING: start already passed, leave will be refused"
step "leave $M4" "$M4" leave "$ARG" "0 NEAR" "150 Tgas"

d=0
while [ "$d" -lt 3 ]; do
  sleep_until $(( START + d * DAY_SECS + 25 ))
  echo "== day $d"
  [ "$d" -eq 2 ] && step "use_freeze $M3 (covers day 1)" "$M3" use_freeze "$ARG" "0.05 NEAR" "60 Tgas"
  step "check_in d$d $M1" "$M1" check_in "$ARG" "0 NEAR" "30 Tgas"
  step "check_in d$d $M2" "$M2" check_in "$ARG" "0 NEAR" "30 Tgas"
  [ "$d" -ne 1 ] && step "check_in d$d $M3" "$M3" check_in "$ARG" "0 NEAR" "30 Tgas"
  d=$(( d + 1 ))
done

sleep_until $(( START + 3 * DAY_SECS + 20 ))
echo "== members before settle"; pot | grep -E '"account"|"alive"|"checkins"|"days"' | paste - - - - | sed 's/  */ /g'
echo "== settle"
i=0
while [ "$i" -lt 8 ]; do
  step "settle" "$M2" settle "{\"pot_id\":\"$POT\",\"batch\":3}" "0 NEAR" "300 Tgas"
  sleep 25
  pend=$(pot | sed -n 's/.*"payouts_pending": *\([0-9]*\).*/\1/p' | head -1); echo "payouts_pending ${pend:-?}"
  [ "${pend:-}" = "0" ] && break
  i=$(( i + 1 ))
done
echo "FINISHED, give-ups: ${GAVE:-0}"
