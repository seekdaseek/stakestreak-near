import test from "node:test";
import assert from "node:assert/strict";
import { actionCreators, najActionToInternal } from "@near-wallet-selector/core";
import { functionCallAction } from "../web/actions.js";

test("builder output goes through core's najActionToInternal as a FunctionCall", () => {
  const a = functionCallAction(actionCreators, { method: "join", args: { pot_id: "streak-demo-2" }, gasTgas: 50, depositYocto: "500000000000000000000000" });
  const i = najActionToInternal(a);
  assert.equal(i.type, "FunctionCall");
  assert.equal(i.params.methodName, "join");
  assert.deepEqual(i.params.args, { pot_id: "streak-demo-2" });
  assert.equal(i.params.gas, String(50n * 10n ** 12n));
  assert.equal(i.params.deposit, "500000000000000000000000");
});

test("check_in has a zero deposit", () => {
  const i = najActionToInternal(functionCallAction(actionCreators, { method: "check_in", args: { pot_id: "p" }, gasTgas: 30 }));
  assert.equal(i.params.methodName, "check_in");
  assert.equal(i.params.deposit, "0");
  assert.equal(i.params.gas, String(30n * 10n ** 12n));
});

test("the old internal shape is rejected by core, which is the bug this guards against", () => {
  assert.throws(() => najActionToInternal({ type: "FunctionCall", params: { methodName: "join", args: {}, gas: "1", deposit: "0" } }), /Unsupported NAJ action/);
});
