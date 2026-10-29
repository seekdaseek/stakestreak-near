#!/bin/sh
# Test-account runner for one pot on the real clock: each actor checks in on the days listed in
# PLAN, then SETTLER calls settle until nothing is pending. Meant to run on a server, not a laptop
# (a laptop that freezes mid-pot loses a day; see docs/RUNBOOK.md). Uses send-retry.sh.
#
#   POT=streak-demo-3 START=<epoch seconds of the pot start> DAY_SECS=600 DAYS=3 \
#   SETTLER=a.testnet PLAN="m1.testnet:0,1,2 m2.testnet:0" ./scripts/run-actors.sh
#
# PLAN is a list of account:days. Days are 0-based. Check-ins go out 30 s into each day. The wallet
# account of a human tester is simply left out of PLAN.
set -eu
here=$(cd "$(dirname "$0")" && pwd)
: "${POT:?}" "${START:?}" "${SETTLER:?}" "${PLAN:?}"
DAY_SECS="${DAY_SECS:-600}"
DAYS="${DAYS:-3}"
export LOG="${LOG:-./runner-$POT.log}"
. "$here/send-retry.sh"

sleep_until() { now=$(date +%s); [ "$1" -gt "$now" ] && sleep $(( $1 - now )); return 0; }
view_pot() {
  near contract call-function as-read-only "$CONTRACT" get_pot json-args "{\"pot_id\":\"$POT\"}" \
    network-config testnet now 2>&1
}
failed=0
d=0
while [ "$d" -lt "$DAYS" ]; do
  sleep_until $(( START + d * DAY_SECS + 30 ))
  echo "== day $d"
  for item in $PLAN; do
    acct=${item%%:*}; days=${item#*:}
    case ",$days," in
      *",$d,"*) send_tx "check_in d$d $acct" "$acct" check_in "{\"pot_id\":\"$POT\"}" "0 NEAR" "30 Tgas" || {
        echo "GAVE UP: check_in d$d $acct (see $LOG)"; failed=$(( failed + 1 )); } ;;
    esac
  done
  d=$(( d + 1 ))
done

sleep_until $(( START + DAYS * DAY_SECS + 25 ))
echo "== settle"
i=0
while [ "$i" -lt 8 ]; do
  send_tx "settle" "$SETTLER" settle "{\"pot_id\":\"$POT\",\"batch\":3}" "0 NEAR" "300 Tgas" || failed=$(( failed + 1 ))
  sleep 25
  pend=$(view_pot | sed -n 's/.*"payouts_pending": *\([0-9]*\).*/\1/p' | head -1)
  echo "payouts_pending ${pend:-?}"
  [ "${pend:-}" = "0" ] && break
  i=$(( i + 1 ))
done
echo "FINISHED, give-ups: $failed"
[ "$failed" -eq 0 ]
