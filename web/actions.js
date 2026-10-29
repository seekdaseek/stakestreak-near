// Builds the action list the wallet modules expect. Meteor Wallet's signAndSendTransaction maps every
// action through core's najActionToInternal, which only accepts near-js Action objects
// (action.functionCall ...) and throws "Unsupported NAJ action" for the internal
// { type: "FunctionCall", params } shape. `actionCreators` comes from @near-wallet-selector/core.
export function functionCallAction(actionCreators, { method, args, gasTgas, depositYocto = "0" }) {
  return actionCreators.functionCall(method, args, BigInt(gasTgas) * 10n ** 12n, BigInt(depositYocto));
}
