"""q3b_extras.py — the per-code view, the route control, the selection of the second invocation's 19. Inputs: model.py."""
import sys, os; sys.path.insert(0, os.path.dirname(__file__))
from model import *
OUT = {"confirmed": "C", "nab": "n", "dnr": "d", "halted": "h"}
def nag(r, depth="c2"):
    i = r["item"]
    if not i[depth]: return None
    return max(len(set(a for j in items if j["pass"] <= r["horizon"] and c in j[depth] for a in j["agents"]) | set(i["agents"])) for c in i[depth])
print("per code (row + bullet): distinct agents over the whole run · verified rows carrying it, in launch order (C confirmed, n reproduced/no break, d did not reproduce) with the agent count at each launch")
percode = collections.defaultdict(list)
for r in sorted(RUNS, key=lambda r: (r["att"], r["item"]["pass"])):
    for c in r["item"]["c2"]:
        n = len(set(a for j in items if j["pass"] <= r["horizon"] and c in j["c2"] for a in j["agents"]))
        percode[c].append((n, OUT[r["out"]], r["agent"]))
tot = lambda c: len(set(a for j in items if c in j["c2"] for a in j["agents"]))
print("| code | distinct agents, whole run | verified rows: agents-at-launch → verdict |")
print("|---|---:|---|")
for c, v in sorted(percode.items(), key=lambda kv: -tot(kv[0])):
    print("| `%s` | %d | %s |" % (c, tot(c), " · ".join("%d→%s (%s)" % x for x in v)))
g = collections.defaultdict(lambda: [0, 0])
for c, v in percode.items():
    b = "1" if tot(c) == 1 else "2" if tot(c) == 2 else "3+"
    g[b][1] += 1; g[b][0] += any(x[1] == "C" for x in v)
print("codes with at least one verified row: %d · with a confirmed row, by distinct agents over the whole run: %s" % (len(percode), {b: "%d of %d" % tuple(v) for b, v in sorted(g.items())}))
codes_never_verified = [c for c in set(c for i in items for c in i["c2"]) if c not in percode]
print("codes no verified row carried:", len(codes_never_verified), "· of them noticed by 3+ agents:", [c for c in codes_never_verified if tot(c) >= 3])
print()
print("route control (the earlier census's sort): confirm rate by band (row + bullet, distinct agents at launch)")
for rt in (True, False):
    g = collections.defaultdict(list)
    for r in RUNS:
        if r["route"] != rt: continue
        n = nag(r); g["no code" if n is None else "1" if n == 1 else "2" if n == 2 else "3+"].append(r)
    print("   names a route: %s — %s" % (rt, {b: rate(g[b]) for b in ("no code", "1", "2", "3+") if g[b]}))
print()
print("by invocation: band x verdict")
for att in ("a1", "a2"):
    g = collections.defaultdict(list)
    for r in RUNS:
        if r["att"] != att: continue
        n = nag(r); g["no code" if n is None else "1" if n == 1 else "2" if n == 2 else "3+"].append(r)
    print("   %s — %s" % (att, {b: rate(g[b]) for b in ("no code", "1", "2", "3+") if g[b]}))
print()
# the second invocation's 19: where they sit in the ledger's order of the 118
order = [i["key"] for i in items if (i["grade"] in ("breaks", "unclear") and not i["v1"]) or i["v1"] == "halted"]
a2 = [r["key"] for r in RUNS if r["att"] == "a2"]
pos = sorted(order.index(k) + 1 for k in a2 if k in order)
print("the 118 unverified rows in ledger order; positions of the 19 that returned in the second invocation:", pos, "of", len(order))
roots = collections.Counter(BY[k]["root"] for k in a2); allroots = collections.Counter(BY[k]["root"] for k in order)
print("   by root: the 19 —", dict(roots)); print("   by root: all 118 —", dict(allroots))
print("   carrying a code (row + bullet): the 19: %d · the other 99: %d" % (sum(bool(BY[k]["c2"]) for k in a2), sum(bool(BY[k]["c2"]) for k in order if k not in a2)))
unv = [BY[k] for k in order if k not in a2]
def nag_i(i):
    if not i["c2"]: return None
    return max(len(set(a for j in items if c in j["c2"] for a in j["agents"])) for c in i["c2"])
c = collections.Counter("no code" if nag_i(i) is None else "1" if nag_i(i) == 1 else "2" if nag_i(i) == 2 else "3+" for i in unv)
print("   the 99 never verified, by band at pass 4:", dict(c))
print()
# left-open counts: returns against report sections
fo = json.load(open(p(CAN + "/data/fan-out.json")))
ret = {}
for ps in fo["passes"]:
    if ps["attempt"] == 1:
        for r in ps["findings_in_by_reporter"]: ret[r["reporter"]] = r["count"]
sect = {k[0]: len(v) for k, v in bullets.items() if k[1] == "a1"}
ver = [k for k in ret if k.startswith("verify-")]
print("first-invocation verifiers whose left-open items were handed on: %d · items in their returns: %d · bullets in their reports' `Left open` sections: %d · reporters where the two counts agree: %d" % (len(ver), sum(ret[k] for k in ver), sum(sect.get(k, 0) for k in ver), sum(1 for k in ver if ret[k] == sect.get(k))))
print("all first-invocation reports with a `Left open` section: %d reports, %d bullets; second invocation: %d reports, %d bullets (their returns: 99)" % (len(sect), sum(sect.values()), sum(1 for k in bullets if k[1] == "a2"), sum(len(v) for k, v in bullets.items() if k[1] == "a2")))
print("source references in the ledger:", sum(len(i["refs"]) for i in items), "· rows:", len(items), "· findings handed to triage:", sum(ps["findings_in"] for ps in fo["passes"] if ps["attempt"] == 1))

print()
print("second invocation, by root and band (row + bullet, distinct agents at launch):")
g = collections.defaultdict(list)
for r in RUNS:
    if r["att"] != "a2": continue
    n = nag(r); g[(r["item"]["root"], "no code" if n is None else "1" if n == 1 else "2" if n == 2 else "3+")].append(r)
for k, v in sorted(g.items()): print("   %s · band %s · %s" % (k[0], k[1], rate(v)))
print("first invocation, band 2 and 3+ runs:", [(r["agent"], r["key"], r["out"]) for r in RUNS if r["att"] == "a1" and (nag(r) or 0) >= 2])
