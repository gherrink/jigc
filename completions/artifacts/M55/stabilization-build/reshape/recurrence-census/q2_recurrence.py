"""q2_recurrence.py — recurrence of a code over the 372 rows. Inputs: model.py."""
import sys, os; sys.path.insert(0, os.path.dirname(__file__))
from model import *

def table(depth):
    occ = collections.defaultdict(list)
    for i in items:
        for c in i[depth]: occ[c].append(i)
    return occ
def dist(vals):
    c = collections.Counter(1 if v == 1 else 2 if v == 2 else 3 if v == 3 else 4 for v in vals)
    return "once %d · twice %d · three times %d · more %d" % (c[1], c[2], c[3], c[4])
for depth, name in (("c1", "row text"), ("c2", "row text + the reporter's bullet")):
    occ = table(depth)
    print("== %s: %d codes over %d rows (%d row-occurrences)" % (name, len(occ), sum(1 for i in items if i[depth]), sum(len(v) for v in occ.values())))
    print("  codes by rows naming them:      ", dist([len(v) for v in occ.values()]))
    print("  codes by distinct agents:       ", dist([len(set(a for i in v for a in i["agents"])) for v in occ.values()]))
    print("  codes by distinct own reporters:", dist([len(set(i["own"] for i in v)) for v in occ.values()]))
    print("  codes by distinct roots (the instrument finding the chain starts from):", dist([len(set(i["root"] for i in v)) for v in occ.values()]))
    rep = sum(1 for v in occ.values() if len(v) > len(set(i["own"] for i in v)))
    print("  codes where one agent names it in two or more of its own rows:", rep)
    print("  occurrences by hop (pass-1):", dict(sorted(collections.Counter(i["hop"] for v in occ.values() for i in v).items())), "· rows per hop:", dict(sorted(collections.Counter(i["hop"] for i in items).items())))
    print("  rows with one code:", sum(len(i[depth]) == 1 for i in items), "· two:", sum(len(i[depth]) == 2 for i in items), "· three or more:", sum(len(i[depth]) >= 3 for i in items))
    print()
    print("| code | in sites.tsv | rows | distinct agents | agents | hops of the rows | distinct roots | whole reports naming it (of 90) |")
    print("|---|---|---:|---:|---|---|---:|---:|")
    for c, v in sorted(occ.items(), key=lambda kv: (-len(set(a for i in kv[1] for a in i["agents"])), -len(kv[1]), kv[0])):
        ags = sorted(set(a for i in v for a in i["agents"]))
        print("| `%s` | %s | %d | %d | %s | %s | %d | %d |" % (c, "yes" if c in SITES else "no", len(v), len(ags), " ".join(ags), " ".join(str(i["hop"]) for i in sorted(v, key=lambda i: i["hop"])), len(set(i["root"] for i in v)), sum(1 for a in REPORT if c in REPORT_CODES[a])))
    print()
    # code + command (the door cell's leaf verbs)
    occ2 = collections.defaultdict(list)
    for i in items:
        for c in i[depth]:
            for d in (i["cmd_door"] or ["(no leaf verb in the door cell)"]): occ2[(c, d)].append(i)
    print("  code + command: %d pairs · by rows: %s · by distinct agents: %s" % (len(occ2), dist([len(v) for v in occ2.values()]), dist([len(set(a for i in v for a in i["agents"])) for v in occ2.values()])))
    for (c, d), v in sorted(occ2.items(), key=lambda kv: -len(kv[1])):
        if len(v) > 1: print("     %s @ %s: rows %d, agents %s, hops %s" % (c, d, len(v), sorted(set(a for i in v for a in i["agents"])), [i["hop"] for i in v]))
    print()
# report level
rc = collections.Counter(c for a in REPORT for c in REPORT_CODES[a])
print("== whole reports: %d codes named by at least one of 90 reports; by number of reports: %s" % (len(rc), dist(rc.values())))
print("   top:", ", ".join("%s×%d" % (c, n) for c, n in rc.most_common(20)))
print("   codes named in reports but in no row (row text):", len(set(rc) - set(table("c1"))), "· (row + bullet):", len(set(rc) - set(table("c2"))))
# merged rows
print("rows with more than one source reference:", sum(len(i["refs"]) > 1 for i in items), "· of them carrying a code (row text):", sum(len(i["refs"]) > 1 and bool(i["c1"]) for i in items))
