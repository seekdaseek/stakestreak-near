#!/bin/sh
# Run one whole pot on NEAR testnet on the real clock and print every transaction with its
# explorer link. Needs near-cli-rs (`near`) and testnet accounts whose keys are in the legacy
# keychain (~/.near-credentials). Safe to read: it only spends testnet NEAR.
#
#   POT=streak-demo-1 CREATOR=a.testnet FEE=f.testnet M1=.. M2=.. M3=.. M4=.. ./scripts/testnet-pot.sh
#
# Plan: M1 and M2 check in every day, M3 skips day 1, M4 skips day 0, so M1 and M2 survive.
set -eu
: "${POT:?}" "${CREATOR:?}" "${FEE:?}" "${M1:?}" "${M2:?}" "${M3:?}" "${M4:?}"
CONTRACT="${CONTRACT:-stakestreak-near.testnet}"
FACTORY="${FACTORY:-sputnikv2.testnet}"
STAKE_NEAR="${STAKE_NEAR:-0.5}"
STAKE_YOCTO="${STAKE_YOCTO:-500000000000000000000000}"
DAY_SECS="${DAY_SECS:-600}"
JOIN_SECS="${JOIN_SECS:-420}"
DAYS="${DAYS:-3}"
EXPLORER="https://testnet.nearblocks.io/txns"

here=$(cd "$(dirname "$0")" && pwd)
export LOG="${LOG:-./runner-$POT.log}"
. "$here/send-retry.sh"
send() { send_tx "$@" || echo "GAVE UP: $1 (see $LOG)"; }
view() { # method json
  near contract call-function as-read-only "$CONTRACT" "$1" json-args "$2" network-config testnet now 2>&1 |
    sed -n '/printed to stdout/,$p' | sed '1d' | sed '/Here is your console/,$d'
}
start_ns() { view get_pot "{\"pot_id\":\"$POT\"}" | sed -n 's/.*"start_ns": *"\{0,1\}\([0-9]*\).*/\1/p' | head -1; }
sleep_until() { # epoch seconds
  now=$(date +%s); [ "$1" -gt "$now" ] && sleep $(( $1 - now )); return 0
}

if [ -z "${SKIP_SETUP:-}" ]; then
echo "== create pot $POT ($FACTORY sub-account $POT.$FACTORY)"
send "create_pot" "$CREATOR" create_pot "{\"args\":{\"name\":\"$POT\",\"rule\":\"walk 20 minutes\",\"stake\":\"$STAKE_YOCTO\",\"duration_days\":$DAYS,\"day_secs\":$DAY_SECS,\"join_secs\":$JOIN_SECS,\"min_members\":3,\"fee_account\":\"$FEE\",\"fee_bps\":300,\"freeze_fee\":\"50000000000000000000000\"}}" "5.6 NEAR" "300 Tgas"
sleep 15
for m in "$M1" "$M2" "$M3" "$M4"; do
  send "join $m" "$m" join "{\"pot_id\":\"$POT\"}" "$STAKE_NEAR NEAR" "50 Tgas"
done
fi

START=$(( $(start_ns) / 1000000000 ))
echo "== pot starts at epoch $START"
d=0
while [ "$d" -lt "$DAYS" ]; do
  sleep_until $(( START + d * DAY_SECS + 20 ))
  echo "== day $d"
  for m in "$M1" "$M2"; do send "check_in d$d $m" "$m" check_in "{\"pot_id\":\"$POT\"}" "0 NEAR" "30 Tgas"; done
  [ "$d" -ne 1 ] && send "check_in d$d $M3" "$M3" check_in "{\"pot_id\":\"$POT\"}" "0 NEAR" "30 Tgas"
  [ "$d" -ne 0 ] && send "check_in d$d $M4" "$M4" check_in "{\"pot_id\":\"$POT\"}" "0 NEAR" "30 Tgas"
  d=$(( d + 1 ))
done

sleep_until $(( START + DAYS * DAY_SECS + 20 ))
echo "== pot over, settling"
view get_pot "{\"pot_id\":\"$POT\"}" | grep -E '"alive"|"account"' | paste - - | sed 's/  */ /g'
i=0
while [ "$i" -lt 6 ]; do
  send "settle (batch 3) #$i" "$CREATOR" settle "{\"pot_id\":\"$POT\",\"batch\":3}" "0 NEAR" "300 Tgas"
  sleep 20
  pend=$(view get_pot "{\"pot_id\":\"$POT\"}" | sed -n 's/.*"payouts_pending": *\([0-9]*\).*/\1/p' | head -1)
  echo "payouts pending: $pend"
  [ "$pend" = "0" ] && break
  i=$(( i + 1 ))
done
echo "== DAO proposals"
near contract call-function as-read-only "$POT.$FACTORY" get_proposals json-args '{"from_index":0,"limit":20}' network-config testnet now 2>&1 | grep -E '"id"|"status"|receiver_id|"amount"' | paste - - - -
echo "DAO: https://testnet.nearblocks.io/address/$POT.$FACTORY"
