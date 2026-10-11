"""q6_confound.py — is a much-noticed code simply on the path? Inputs: model.py (whole reports, rows, runs)."""
import sys, os; sys.path.insert(0, os.path.dirname(__file__))
from model import *
VER = [r for r in RUNS]                                  # 80 verifier runs, each with a report
def rep_codes(r): return REPORT_CODES[r["agent"]]
def rep_text(r): return REPORT[r["agent"]][1]
def sect_of(text, code):
    """the classes of the sections the code is named in, by the section's heading words"""
    out = collections.Counter(); cur = "(top)"; fence = False
    for line in text.splitlines():
        if line.startswith("```"): fence = not fence
        m = re.match(r"#{1,6} +(.*)", line)
        if m and not fence: cur = m.group(1)
        if code in line:
            h = cur.lower()
            cls = "left open" if h.startswith("left open") or h.startswith("leads") else "repro block" if h.startswith("repro") else "verdict" if "verdict" in h else "other section"
            out[cls] += 1
    return out
rc = collections.Counter(c for r in VER for c in rep_codes(r))
top = [c for c, _ in rc.most_common(14)]
sc = collections.defaultdict(set)
for r in K.sites(): sc[r[3]].add(r[0])
print("verifier reports: 80 (61 + 19). Sent for = the row the verifier was handed carries the code (row + re-drive, or row + bullet).")
print()
print("| code | sites.tsv command | verifier reports naming it | of them sent for it | not sent for it | … of those, handed a row at a different door verb than any row carrying the code | rows naming it (row + bullet) | reports naming it per row naming it | where the not-sent reports name it (lines: left open · repro block · verdict · other) |")
print("|---|---|---:|---:|---:|---:|---:|---:|---|")
for c in top:
    named = [r for r in VER if c in rep_codes(r)]
    sent = [r for r in named if c in r["ch"] or c in r["item"]["c2"]]
    notsent = [r for r in named if r not in sent]
    doors_c = set(d for i in items if c in i["c2"] for d in i["cmd_door"])
    other_door = [r for r in notsent if not (set(r["item"]["cmd_door"]) & doors_c)]
    rows = [i for i in items if c in i["c2"]]
    w = collections.Counter()
    for r in notsent: w.update(sect_of(rep_text(r), c))
    print("| `%s` | %s | %d | %d | %d | %d | %d | %s | %d · %d · %d · %d |" % (c, ", ".join(sorted(sc.get(c, ["(not listed)"]))), len(named), len(sent), len(notsent), len(other_door), len(rows), ("%.1f" % (len(named) / len(rows))) if rows else "-", w["left open"], w["repro block"], w["verdict"], w["other section"]))
print()
# per door: share of verifiers at that door whose report names the code, whatever they were sent for
print("share of the verifiers handed a row at a door whose report names the code (runs with one door verb; doors with >= 4 runs):")
bydoor = collections.defaultdict(list)
for r in VER:
    if len(r["item"]["cmd_door"]) == 1: bydoor[r["item"]["cmd_door"][0]].append(r)
doors = [d for d, v in sorted(bydoor.items(), key=lambda kv: -len(kv[1])) if len(v) >= 4]
print("| code | " + " | ".join("%s (%d)" % (d, len(bydoor[d])) for d in doors) + " | all 80 |")
print("|---|" + "---:|" * (len(doors) + 1))
for c in top:
    print("| `%s` | " % c + " | ".join("%d" % sum(1 for r in bydoor[d] if c in rep_codes(r)) for d in doors) + " | %d |" % sum(1 for r in VER if c in rep_codes(r)))
print()
# inheritance: was the agent that left a row naming code c itself handed a row naming c?
print("inheritance — each row naming a code (row + bullet), by whether its own reporter had been handed that code:")
fresh = echo = root_ = 0; per = collections.defaultdict(lambda: [0, 0, 0])
for i in items:
    for c in i["c2"]:
        par = i["parent"]
        if par is None or par not in BY: root_ += 1; per[c][2] += 1; continue
        pr = BY[par]
        prun = [r for r in RUNS if r["key"] == par]
        pcodes = set(pr["c2"]) | set(x for r in prun for x in r["ch"])
        if c in pcodes: echo += 1; per[c][0] += 1
        else: fresh += 1; per[c][1] += 1
print("   row-occurrences: %d · the reporter had been handed the code: %d · had not: %d · from an instrument (no parent): %d" % (echo + fresh + root_, echo, fresh, root_))
print("   per code with >= 3 distinct agents (handed · not handed · instrument):", {c: tuple(v) for c, v in per.items() if len(set(a for i in items if c in i["c2"] for a in i["agents"])) >= 3})
print()
# independence of the 3+ band
print("the six runs in band 3+ (row + bullet, distinct agents):")
for r in RUNS:
    i = r["item"]
    if not i["c2"]: continue
    best = max(len(set(a for j in items if j["pass"] <= r["horizon"] and c in j["c2"] for a in j["agents"])) for c in i["c2"])
    if best >= 3:
        named = {c: len(report_agents(c, r["horizon"])) for c in i["c2"]}
        print("   %s `%s` · door %s · root `%s` · %s · reports naming its codes at launch: %s · parent's verdict: %s" % (r["agent"], r["key"], i["cmd_door"], i["root"], r["out"], named, r["pverd"]))
print()
# how many verifier reports name a code at all, and how many codes a report names
import statistics
n = [len(rep_codes(r)) for r in VER]
print("codes named per verifier report: median %s · mean %.1f · max %d · reports naming none: %d" % (statistics.median(n), sum(n) / len(n), max(n), sum(1 for x in n if x == 0)))
print("codes a verifier report names that its handed row did not carry: median %s" % statistics.median(len(rep_codes(r) - set(r["ch"]) - set(r["item"]["c2"])) for r in VER))
# confirmed vs not: number of codes in the report
for o in ("confirmed", "nab", "dnr", "halted"):
    v = [len(rep_codes(r)) for r in VER if r["out"] == o]
    print("   %s: median codes in the report %s (n %d)" % (o, statistics.median(v), len(v)))
# rig states named in reports that name the top codes
print()
print("fixture named in the report (`refs-post-hoc`, `fresh`, `bare`, ...) against naming the code:")
for c in top[:8]:
    row = []
    for fx in ("refs-post-hoc", "fresh", "bare", "committed-singletons"):
        a = [r for r in VER if re.search(r"(?<![\w-])%s(?![\w-])" % fx, rep_text(r))]
        row.append("%s %d/%d" % (fx, sum(1 for r in a if c in rep_codes(r)), len(a)))
    print("   %-40s %s" % (c, " · ".join(row)))
