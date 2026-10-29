#!/bin/sh
# Fetch the Sputnik DAO factory and the default DAO code that sputnikv2.testnet serves today,
# then check them against tests/fixtures/SHA256SUMS. The sandbox tests run against these exact bytes.
set -eu
cd "$(dirname "$0")/../tests/fixtures"
RPC="${NEAR_TESTNET_RPC:-https://rpc.testnet.near.org}"
python3 - "$RPC" <<'PY'
import base64, json, sys, urllib.request
rpc = sys.argv[1]
def q(params):
    req = urllib.request.Request(rpc, data=json.dumps({"jsonrpc": "2.0", "id": 1, "method": "query", "params": params}).encode(), headers={"Content-Type": "application/json"})
    return json.load(urllib.request.urlopen(req, timeout=120))["result"]
open("factory.wasm", "wb").write(base64.b64decode(q({"request_type": "view_code", "finality": "final", "account_id": "sputnikv2.testnet"})["code_base64"]))
args = base64.b64encode(json.dumps({"code_hash": "AA4nA45vauiac3jdVwJTzSdUP3oj2r1fDaPAb2mHdgcH"}).encode()).decode()
open("dao.wasm", "wb").write(bytes(q({"request_type": "call_function", "finality": "final", "account_id": "sputnikv2.testnet", "method_name": "get_code", "args_base64": args})["result"]))
PY
shasum -a 256 -c SHA256SUMS
