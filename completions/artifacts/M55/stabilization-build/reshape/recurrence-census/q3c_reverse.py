"""q3c_reverse.py — the reverse view: each confirmed finding, and each that did not reproduce, against how often
its code had been named before its verifier was launched. Inputs: model.py.
A finding's codes = the codes its row carried before launch (row text, key, re-drive line, reporter's bullet)
plus the codes its verifier's basis names (known only after the verdict; marked *)."""
import sys, os; sys.path.insert(0, os.path.dirname(__file__))
from model import *
NAME = {"confirmed": "confirmed", "dnr": "did not reproduce", "nab": "reproduced, no break", "halted": "halted"}
def view(r):
    i = r["item"]; pre = set(i["c2"]) | set(r["ch"]); post = set(r["basis_codes"]) - pre
    cells = []; best_other = 0; best_rep = 0
    for c in sorted(pre) + sorted(post):
        rows = [j for j in items if j["pass"] <= r["horizon"] and c in j["c2"] and j is not i]
        ags = set(a for j in rows for a in j["agents"]) - set(i["agents"])
        reps = [a for a in report_agents(c, r["horizon"])]
        cells.append((c, c in post, len(rows), len(ags), len(reps)))
        best_other = max(best_other, len(ags)); best_rep = max(best_rep, len(reps))
    return pre, post, cells, best_other, best_rep
summ = collections.defaultdict(list)
for o in ("confirmed", "dnr"):
    print("| %s: run | key | hop | code (* = named only by the verifier's basis) | other rows naming it at launch | other agents | whole reports naming it at launch |" % NAME[o])
    print("|---|---|---:|---|---:|---:|---:|")
    for r in RUNS:
        if r["out"] != o: continue
        pre, post, cells, bo, br = view(r)
        summ[o].append((bool(pre), bool(pre or post), bo, br))
        if not cells: print("| %s | `%s` | %d | none | - | - | - |" % (r["agent"], r["key"], r["item"]["hop"])); continue
        for n, (c, isp, nr, na, nrep) in enumerate(cells):
            print("| %s | %s | %s | `%s`%s | %d | %d | %d |" % (r["agent"] if n == 0 else "", ("`%s`" % r["key"]) if n == 0 else "", r["item"]["hop"] if n == 0 else "", c, "*" if isp else "", nr, na, nrep))
    print()
for r in RUNS:
    if r["out"] in ("nab", "halted"):
        pre, post, cells, bo, br = view(r); summ[r["out"]].append((bool(pre), bool(pre or post), bo, br))
print("| verdict | runs | row carried a code before launch | a code by row or basis | another agent's row had named one of its codes before launch: none | 1 agent | 2 | 3+ | an earlier whole report had named one |")
print("|---|---:|---:|---:|---:|---:|---:|---:|---:|")
for o in ("confirmed", "dnr", "nab", "halted"):
    v = summ[o]
    print("| %s | %d | %d | %d | %d | %d | %d | %d | %d |" % (NAME[o], len(v), sum(a for a, *_ in v), sum(b for _, b, *_ in v), sum(1 for *_, n, _ in v if n == 0), sum(1 for *_, n, _ in v if n == 1), sum(1 for *_, n, _ in v if n == 2), sum(1 for *_, n, _ in v if n >= 3), sum(1 for *_, m in v if m > 0)))
print()
print("rows carrying a code by hop (row text / row + bullet):", {h: "%d / %d of %d" % (sum(1 for i in items if i["hop"] == h and i["c1"]), sum(1 for i in items if i["hop"] == h and i["c2"]), sum(1 for i in items if i["hop"] == h)) for h in range(4)})
print("rows carrying a code by grade (row + bullet):", {g: "%d of %d" % (sum(1 for i in items if i["grade"] == g and i["c2"]), sum(1 for i in items if i["grade"] == g)) for g in ("breaks", "unclear", "needs-bound", "no-break")})
print("codes: row depth %d (in sites.tsv %d) · row + bullet %d (in sites.tsv %d)" % (len(set(c for i in items for c in i["c1"])), len(set(c for i in items for c in i["c1"]) & SITES), len(set(c for i in items for c in i["c2"])), len(set(c for i in items for c in i["c2"]) & SITES)))
print("refs-post-hoc named in verifier reports: %d of 80; store.not-found named in %d of those and %d of the rest" % (sum(1 for r in RUNS if "refs-post-hoc" in REPORT[r["agent"]][1]), sum(1 for r in RUNS if "refs-post-hoc" in REPORT[r["agent"]][1] and "store.not-found" in REPORT_CODES[r["agent"]]), sum(1 for r in RUNS if "refs-post-hoc" not in REPORT[r["agent"]][1] and "store.not-found" in REPORT_CODES[r["agent"]])))
