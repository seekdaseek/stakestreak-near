# Sourced by the testnet scripts. send_tx retries a transaction that never reached the chain.
#
#   send_tx <label> <signer> <method> <json> <deposit> <gas>
#
# A refusal the CLI's own simulation reports (ExecutionError) is final and not retried.
# Up to TRIES attempts (default 3), RETRY_WAIT seconds apart (default 6). An attempt counts as
# having reached the chain when the CLI printed a Transaction ID; a contract panic after that is
# final and is not retried. Every attempt's full CLI output goes to LOG (default ./runner.log),
# so a failure leaves its error text behind. Sets TX_ID (empty if all attempts failed) and
# returns 0 when the transaction reached the chain, 1 otherwise.
CONTRACT="${CONTRACT:-stakestreak-near.testnet}"
EXPLORER="${EXPLORER:-https://testnet.near.rocks/tx}"
TRIES="${TRIES:-3}"
RETRY_WAIT="${RETRY_WAIT:-6}"
LOG="${LOG:-./runner.log}"

send_tx() {
  _label=$1 _signer=$2 _method=$3 _json=$4 _dep=$5 _gas=$6
  TX_ID=""
  _n=1
  while [ "$_n" -le "$TRIES" ]; do
    _out=$(near contract call-function as-transaction "$CONTRACT" "$_method" json-args "$_json" \
      prepaid-gas "$_gas" attached-deposit "$_dep" sign-as "$_signer" network-config testnet \
      sign-with-legacy-keychain send 2>&1) || true
    TX_ID=$(printf '%s\n' "$_out" | sed -n 's/.*Transaction ID: *\([A-Za-z0-9]*\).*/\1/p' | head -1)
    {
      printf -- '--- %s %s attempt %s/%s tx=%s\n' "$(date -u +%H:%M:%S)" "$_label" "$_n" "$TRIES" "${TX_ID:-none}"
      printf '%s\n' "$_out"
    } >> "$LOG"
    if [ -n "$TX_ID" ]; then
      printf '%s %-34s %s\n' "$(date -u +%H:%M:%S)" "$_label" "$EXPLORER/$TX_ID"
      printf '%s\n' "$_out" | grep -E "Smart contract panicked|ERR_|Failure" | head -3 || true
      return 0
    fi
    # The CLI simulates first: a contract refusal ("ExecutionError") is final, retrying cannot change it.
    if printf '%s\n' "$_out" | grep -q 'ExecutionError'; then
      printf '%s %-34s REFUSED by the contract: %s\n' "$(date -u +%H:%M:%S)" "$_label" \
        "$(printf '%s\n' "$_out" | sed -n 's/.*ExecutionError("\(.*\)").*/\1/p' | head -1 | cut -c1-120)"
      return 1
    fi
    printf '%s %-34s no tx id, attempt %s/%s: %s\n' "$(date -u +%H:%M:%S)" "$_label" "$_n" "$TRIES" \
      "$(printf '%s\n' "$_out" | grep -v '^[[:space:]]*$' | tail -1 | cut -c1-160)"
    _n=$(( _n + 1 ))
    [ "$_n" -le "$TRIES" ] && sleep "$RETRY_WAIT"
  done
  return 1
}
