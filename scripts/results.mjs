#!/usr/bin/env node
// Counts the pots on the deployed contract and writes docs/RESULTS.md. Everything comes from view
// calls on testnet; nothing is typed in by hand.   node scripts/results.mjs [--check]
//   --check  print the file instead of writing it
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const CONTRACT = process.env.CONTRACT || "stakestreak-near.testnet";
const RPCS = ["https://rpc.testnet.fastnear.com", "https://test.rpc.fastnear.com", "https://archival-rpc.testnet.near.org"];
const OWN = new RegExp(process.env.OWN_PATTERN || "^stakestreak-near-");
const EXPL = "https://testnet.near.rocks";
const NEAR = 10n ** 24n;
const fmt = (y) => { const n = BigInt(y); const w = n / NEAR, f = (n % NEAR) / 10n ** 18n; return `${w}.${String(f).padStart(6, "0")}`.replace(/0+$/, "").replace(/\.$/, ".0"); };

const DEADLINE = Date.now() + 10 * 60 * 1000; // give up after 10 minutes and keep the last snapshot
const TRIES = 3;
async function rpc(method, params) {
  let last;
  for (let attempt = 1; attempt <= TRIES; attempt++) {
    for (const url of RPCS) {
      if (Date.now() > DEADLINE) throw new Error(`10 minute limit reached, last error: ${last}`);
      try {
        const r = await fetch(url, { method: "POST", headers: { "content-type": "application/json" }, body: JSON.stringify({ jsonrpc: "2.0", id: 1, method, params }), signal: AbortSignal.timeout(10000) });
        const j = await r.json();
        if (j.error) throw new Error(JSON.stringify(j.error));
        return j.result;
      } catch (e) { last = e; }
    }
    await new Promise((res) => setTimeout(res, 1500 * attempt));
  }
  throw last;
}
async function view(account, method, args) {
  const r = await rpc("query", { request_type: "call_function", finality: "final", account_id: account, method_name: method, args_base64: Buffer.from(JSON.stringify(args)).toString("base64") });
  return JSON.parse(Buffer.from(r.result).toString());
}

const pots = [];
for (let from = 0; ; from += 20) {
  const page = await view(CONTRACT, "list_pots", { from, limit: 20 });
  pots.push(...page);
  if (page.length < 20) break;
}
const ph = (p) => p.phase;
const settled = pots.filter((p) => ph(p) === "settled");
const accounts = new Set(pots.flatMap((p) => p.members.filter((m) => m.funded).map((m) => m.account)));
const own = [...accounts].filter((a) => OWN.test(a)), other = [...accounts].filter((a) => !OWN.test(a));
let totalIn = 0n, totalOut = 0n, win = 0n, refund = 0n, fee = 0n, proposals = 0;
const rows = [];
for (const p of pots) {
  const funded = p.members.filter((m) => m.funded);
  // A member who left before the start is no longer in the member list, but their stake went in and came back as a Refund payout.
  const memberSet = new Set(p.members.map((m) => m.account));
  const leftRefunds = p.payouts.filter((x) => x.kind === "Refund" && !memberSet.has(x.to) && "Approved" in x.state).reduce((n, x) => n + BigInt(x.amount), 0n);
  const stakeIn = BigInt(p.stake) * BigInt(funded.length) + leftRefunds;
  let w = 0n, r = 0n, f = 0n, n = 0;
  for (const x of p.payouts) {
    if (!("Approved" in x.state)) continue;
    n++;
    const a = BigInt(x.amount);
    if (x.kind === "Win") w += a; else if (x.kind === "Refund") r += a; else f += a;
  }
  if (ph(p) === "settled") { totalIn += stakeIn; win += w; refund += r; fee += f; totalOut += w + r + f; proposals += n; }
  const alive = funded.filter((m) => m.alive).length;
  rows.push(`| [${p.name}](${EXPL}/account/${p.dao}) | ${ph(p)} | ${funded.length} | ${p.duration_days} x ${Number(p.day_ns) / 6e10} min | ${fmt(stakeIn)} | ${ph(p) === "settled" ? `${fmt(w)} / ${fmt(r)} / ${fmt(f)}` : "-"} | ${ph(p) === "settled" ? alive : "-"} |`);
}
const head = await rpc("block", { finality: "final" });
const when = new Date(head.header.timestamp / 1e6).toISOString().replace(/\.\d+Z$/, "Z");
const md = `# Results (written by scripts/results.mjs, do not edit by hand)

Read from view calls on \`${CONTRACT}\` (testnet) at block ${head.header.height}, ${when}.

| | |
|---|---|
| Pots created | ${pots.length} |
| Pots settled | ${settled.length} |
| Funded member accounts | ${accounts.size} |
| of which test accounts (\`${OWN.source}\`) | ${own.length} |
| of which other accounts | ${other.length} |
| NEAR staked in settled pots | ${fmt(totalIn)} |
| NEAR paid out by approved Transfer proposals | ${fmt(totalOut)} (wins ${fmt(win)}, refunds ${fmt(refund)}, fees ${fmt(fee)}) |
| Approved Transfer proposals | ${proposals} |

**Own test pots only.** There are no outside users. The accounts that do not match the test-account
pattern are a separate wallet held by the developer, used through the web client (see
[TESTNET.md](TESTNET.md)); they are not outside users.

Some pots settled under an earlier no-survivor rule (the whole pot went to the fee account); see
[TESTNET.md](TESTNET.md). The numbers above are whatever the chain says.

| Pot (DAO) | Phase | Funded | Length | Staked | Paid: wins / refunds / fees | Alive at end |
|---|---|---|---|---|---|---|
${rows.join("\n")}
`;
if (process.argv.includes("--check")) process.stdout.write(md);
else { const out = join(dirname(fileURLToPath(import.meta.url)), "..", "docs", "RESULTS.md"); writeFileSync(out, md); console.log(`wrote ${out}: ${pots.length} pots, ${settled.length} settled`); }
