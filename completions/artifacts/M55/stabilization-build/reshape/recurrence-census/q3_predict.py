"""q3_predict.py — recurrence against verdict, both views. Inputs: model.py.

A run's *horizon* is the last triage pass whose rows were on record when its verifier was launched:
the row's own pass in the first invocation, pass 4 in the second. Only rows up to the horizon count.
"""
import sys, os; sys.path.insert(0, os.path.dirname(__file__))
from model import *
OUT = {"confirmed": "confirmed", "nab": "reproduced, no break", "dnr": "did not reproduce", "halted": "halted"}
def n_for(r, own_depth, count_depth, what, strict=False):
    cs = r["ch"] if own_depth == "handed" else r["item"][own_depth]
    if not cs: return None, {}
    h = r["horizon"] - (1 if strict else 0)
    per = {}
    for c in cs:
        rows = [i for i in items if i["pass"] <= h and c in i[count_depth]]
        if strict: pass
        elif r["item"] not in rows and c in cs: rows = rows + [r["item"]]     # the item itself is one sighting
        ags = set(a for i in rows for a in i["agents"])
        per[c] = len(rows) if what == "rows" else len(ags)
    return max(per.values()), per
def bands(own_depth, count_depth, what, strict=False):
    g = collections.defaultdict(list)
    for r in RUNS:
        n, _ = n_for(r, own_depth, count_depth, what, strict)
        g["no code" if n is None else ("0" if n == 0 else "1" if n == 1 else "2" if n == 2 else "3+")].append(r)
    return g
def show(title, g, order=("no code", "0", "1", "2", "3+")):
    print(title)
    print("| band | runs | confirmed | reproduced, no break | did not reproduce | halted |")
    print("|---|---:|---:|---:|---:|---:|")
    for b in order:
        v = g.get(b, [])
        if not v and b == "0": continue
        c = collections.Counter(r["out"] for r in v)
        print("| %s | %d | %d | %d | %d | %d |" % (b, len(v), c["confirmed"], c["nab"], c["dnr"], c["halted"]))
    print()
show("A. key = codes of the row text; count = distinct agents with a row naming the code, up to the horizon (the item's own reporter is 1)", bands("c1", "c1", "agents"))
show("A'. the same, counting rows instead of agents", bands("c1", "c1", "rows"))
show("B. key = codes of row text + the reporter's bullet; count over the same depth, distinct agents", bands("c2", "c2", "agents"))
show("B'. the same, counting rows", bands("c2", "c2", "rows"))
show("C. key = codes of row text + re-drive line (what the verifier was handed); count over row text + bullet, distinct agents", bands("handed", "c2", "agents"))
show("D. strict: distinct agents with a row naming the code in an EARLIER pass than the horizon (0 = nobody before this pass); key = row text + bullet", bands("c2", "c2", "agents", strict=True))
# report level
g = collections.defaultdict(list); rawrep = {}
for r in RUNS:
    cs = r["item"]["c2"]
    if not cs: g["no code"].append(r); continue
    n = max(len(report_agents(c, r["horizon"])) for c in cs); rawrep[id(r)] = n
    g["0" if n == 0 else "1" if n == 1 else "2" if n == 2 else "3+"].append(r)
show("E. report level: distinct agents whose WHOLE report (returned before the horizon's triage) names the code; key = row text + bullet", g)
g2 = collections.defaultdict(list)
for r in RUNS:
    if r["item"]["c2"]:
        n = rawrep[id(r)]; g2["1-2" if n <= 2 else "3-5" if n <= 5 else "6+"].append(r)
print("E, finer bands:", {b: rate(v) for b, v in sorted(g2.items())})
print()
print("RAW — every run whose row carries a code (row text + bullet):")
print("| run | key | hop | horizon | codes (row text; +bullet marked ^) | rows / agents naming it at launch (per code) | reports naming it at launch | verdict |")
print("|---|---|---:|---:|---|---|---|---|")
for r in RUNS:
    i = r["item"]
    if not i["c2"]: continue
    _, pr = n_for(r, "c2", "c2", "rows"); _, pa = n_for(r, "c2", "c2", "agents")
    print("| %s | `%s` | %d | %d | %s | %s | %s | %s |" % (r["agent"], r["key"], i["hop"], r["horizon"],
        ", ".join("`%s`%s" % (c, "" if c in i["c1"] else "^") for c in i["c2"]),
        ", ".join("%d / %d" % (pr[c], pa[c]) for c in i["c2"]),
        ", ".join(str(len(report_agents(c, r["horizon"]))) for c in i["c2"]), OUT[r["out"]]))
print()
print("REVERSE — the 18 confirmed and the 10 that did not reproduce: the codes the verifier's basis names (known only after the verdict),")
print("and how often each had been named before launch")
print("| verdict | run | key | hop | row's own codes before launch (row / +re-drive / +bullet) | basis codes | per basis code: rows / distinct agents (row+bullet) at launch — excluding this row | per basis code: whole reports at launch |")
print("|---|---|---|---:|---|---|---|---|")
summary = collections.defaultdict(list)
for o in ("confirmed", "dnr"):
    for r in RUNS:
        if r["out"] != o: continue
        i = r["item"]; bc = r["basis_codes"]; cells = []; reps = []; best = 0; bestrep = 0
        for c in bc:
            rows = [j for j in items if j["pass"] <= r["horizon"] and c in j["c2"] and j is not i]
            ags = set(a for j in rows for a in j["agents"])
            cells.append("%d / %d" % (len(rows), len(ags))); best = max(best, len(ags))
            ra = report_agents(c, r["horizon"]); reps.append(str(len(ra))); bestrep = max(bestrep, len(ra))
        summary[o].append((bool(bc), bool(i["c2"]), best, bestrep))
        print("| %s | %s | `%s` | %d | %s / %s / %s | %s | %s | %s |" % (OUT[o], r["agent"], r["key"], i["hop"],
            len(i["c1"]), len(r["ch"]), len(i["c2"]), ", ".join("`%s`" % c for c in bc) or "none", ", ".join(cells) or "-", ", ".join(reps) or "-"))
print()
for o, v in summary.items():
    print(OUT[o], "· n", len(v), "· basis names a code:", sum(a for a, _, _, _ in v), "· the row itself carried a code before launch:", sum(b for _, b, _, _ in v),
          "· some OTHER row had named a basis code before launch:", sum(1 for a, _, n, _ in v if n > 0), "(by 1 agent: %d, by 2: %d, by 3+: %d)" % (sum(1 for *_, n, _ in v if n == 1), sum(1 for *_, n, _ in v if n == 2), sum(1 for *_, n, _ in v if n >= 3)),
          "· some earlier report had named it:", sum(1 for *_, m in v if m > 0), "(median reports %s)" % sorted(m for *_, m in v)[len(v) // 2])
# the same reverse summary for no-break
v = []
for r in RUNS:
    if r["out"] != "nab": continue
    i = r["item"]; best = 0; bestrep = 0
    for c in r["basis_codes"]:
        rows = [j for j in items if j["pass"] <= r["horizon"] and c in j["c2"] and j is not i]
        best = max(best, len(set(a for j in rows for a in j["agents"]))); bestrep = max(bestrep, len(report_agents(c, r["horizon"])))
    v.append((bool(r["basis_codes"]), bool(i["c2"]), best, bestrep))
print("reproduced, no break · n", len(v), "· basis names a code:", sum(a for a, *_ in v), "· the row itself carried a code:", sum(b for _, b, _, _ in v), "· some OTHER row had named a basis code:", sum(1 for *_, n, _ in v if n > 0), "· some earlier report:", sum(1 for *_, m in v if m > 0))
