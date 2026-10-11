"""q4_duplicates.py — duplicates among verifications, what a merge by code would fold, and triage's own
named restatements as the comparison. Inputs: model.py; the triage reports (kin notes)."""
import sys, os; sys.path.insert(0, os.path.dirname(__file__))
from model import *
OUT = {"confirmed": "confirmed", "nab": "reproduced, no break", "dnr": "did not reproduce", "halted": "halted", "": "not verified"}
def verdicts(i):
    return [(r["att"], r["out"]) for r in RUNS if r["key"] == i["key"]]
def vtxt(i):
    v = verdicts(i)
    return "; ".join("%s %s" % (a, OUT[o]) for a, o in v) if v else "not verified (%s)" % i["grade"]
def launch_order(r): return (0 if r["att"] == "a1" else 1, r["item"]["pass"])
def pairs(depth, with_door):
    def ks(i):
        if with_door: return {(c, d) for c in i[depth] for d in (i["cmd_door"] or ["-"])}
        return set(i[depth])
    out = []
    for a in range(len(RUNS)):
        for b in range(a + 1, len(RUNS)):
            ra, rb = RUNS[a], RUNS[b]
            if ra["key"] == rb["key"]: continue
            sh = ks(ra["item"]) & ks(rb["item"])
            if sh: out.append((ra, rb, sh))
    return out
for depth, dn in (("c1", "the key at row depth"), ("c2", "row + bullet")):
    for wd in (False, True):
        P = pairs(depth, wd)
        runs_in = set(id(r) for a, b, _ in P for r in (a, b))
        later = set()
        for a, b, _ in P:
            if launch_order(a) < launch_order(b): later.add(id(b))
            elif launch_order(b) < launch_order(a): later.add(id(a))
        same = sum(1 for a, b, _ in P if launch_order(a) == launch_order(b))
        vv = collections.Counter(tuple(sorted((OUT[a["out"]], OUT[b["out"]]))) for a, b, _ in P)
        print("== %s%s: %d pairs of verified rows share a key · %d of 80 runs are in a pair · %d runs were launched after a row of the same key had its verdict · %d pairs were launched in one batch" % (dn, " + door verb" if wd else "", len(P), len(runs_in), len(later), same))
        print("   verdict pairs:", dict(vv))
        if depth == "c2":
            for a, b, sh in P:
                print("   - %s `%s` [%s, hop %d] ~ %s `%s` [%s, hop %d] — shared %s — %s" % (a["agent"], a["key"], OUT[a["out"]], a["item"]["hop"], b["agent"], b["key"], OUT[b["out"]], b["item"]["hop"], sorted(sh), "one batch" if launch_order(a) == launch_order(b) else "the second after the first's verdict"))
    print()

# ---- a mechanical merge over all 372 rows ------------------------------------------------------
def merge_sim(depth, with_door, same_pass_only=False):
    seen = {}; merged = []; first = {}
    for i in sorted(items, key=lambda i: i["pass"]):
        ks = {(c, d) for c in i[depth] for d in (i["cmd_door"] or ["-"])} if with_door else set(i[depth])
        if same_pass_only: ks = {(i["pass"], k) for k in ks}
        hit = [seen[k] for k in ks if k in seen]
        if hit: merged.append((i, hit[0]))
        for k in ks: seen.setdefault(k, i)
    return merged
print("== what triage merged: 65 of 437 findings handed in (20+4+15+26), all inside one pass; across passes it could mark none `found again` (its reports say the ledger it was handed held one row)")
for depth, dn in (("c1", "row depth"), ("c2", "row + bullet")):
    for wd in (False, True):
        M = merge_sim(depth, wd); MS = merge_sim(depth, wd, True)
        tv = sum(1 for i, _ in M if i["grade"] in ("breaks", "unclear"))
        ver = [(i, j) for i, j in M if verdicts(i)]
        print("  merge by code%s (%s): %d rows fold into an earlier row (%d inside one pass, %d across passes) · %d of them were to be verified · %d were verified" % (" + door verb" if wd else "", dn, len(M), len(MS), len(M) - len(MS), tv, len(ver)))
        bypass = collections.Counter(i["pass"] for i, _ in M)
        print("     by pass:", dict(sorted(bypass.items())))
        diff = 0
        for i, j in ver:
            vi = {o for _, o in verdicts(i)}; vj = {o for _, o in verdicts(j)}
            print("     verified & folded: `%s` [%s] -> `%s` [%s]" % (i["key"], vtxt(i), j["key"], vtxt(j)))
print()
# ---- groups, for the reading -------------------------------------------------------------------
print("== groups of rows sharing a code (row + bullet), for the reading in the census")
occ = collections.defaultdict(list)
for i in items:
    for c in i["c2"]: occ[c].append(i)
for c, v in sorted(occ.items(), key=lambda kv: -len(kv[1])):
    if len(v) < 2: continue
    print("### %s — %d rows" % (c, len(v)))
    for i in v:
        print("   - p%d %s `%s` @ %s [%s] — %s" % (i["pass"], i["own"], i["key"], i["door"][:40], vtxt(i), i["why_return"][:230]))
# ---- triage's own kin notes ---------------------------------------------------------------------
def kin():
    out = []
    t2 = read(CAN + "/run/r1/reports/test/triage-p2.a1.md"); s = t2[t2.index("## Three things"):]; s = s[:s.index("\n2. **")]
    for b in re.split(r"\n   - ", s)[1:]:
        b = " ".join(b.split()); ks = re.findall(r"`(r1-[a-z0-9-]+)`", b)
        head = b.split(" — ")[0]; later = re.findall(r"`(r1-[a-z0-9-]+)`", head)
        for l in later:
            for e in ks:
                if e not in later: out.append(("p2", l, e))
    t3 = read(CAN + "/run/r1/reports/test/triage-p3.a1.md"); s = t3[t3.index("| entry of this pass |"):]; s = s[:s.index("\n\n")]
    for line in s.splitlines()[2:]:
        cells = [c.strip() for c in line.strip("|").split(" | ")]
        if len(cells) < 2: continue
        for l in re.findall(r"`(r1-[a-z0-9-]+)`", cells[0]):
            for e in re.findall(r"`(r1-[a-z0-9-]+)`", cells[1]): out.append(("p3", l, e))
    t4 = read(CAN + "/run/r1/reports/test/triage-p4.a1.md"); alias = {}
    for line in t4.splitlines():
        m = re.match(r"\| (r1-p4-[a-z0-9-]+) \|(?:[^|]*\|){4} ([A-Z0-9 ]+?) \|", line)
        if m:
            for a in m.group(2).split(): alias[a] = m.group(1)
    s = t4[t4.index("1. **Repeats of rows"):]; s = s[:s.index("\n2. **")]
    for m in re.finditer(r"((?:[A-Z]{1,2}\d+)(?:(?:\+| and )[A-Z]{1,2}\d+)*)[^;→]*→[^;]*?`(r1-[a-z0-9-]+)`", s):
        for a in re.findall(r"[A-Z]{1,2}\d+", m.group(1)):
            if a in alias: out.append(("p4", alias[a], m.group(2)))
    ta = read(CAN + "/run-pending-a2/triage-p1.a2.md"); rown = {}
    for line in ta.splitlines():
        m = re.match(r"\| *(\d+) *\| *`([a-z0-9-]+)` *\|", line)
        if m: rown[m.group(1)] = m.group(2) if m.group(2).startswith("r1-") else "r1-p4-" + m.group(2)
    s = ta[ta.index("### Kin rows"):]; s = s[:s.index("\n## ")]
    for m in re.finditer(r"row (\d+) (?:beside|is)(?:[^`\n]|`Repro[^`]*`)*`(r1-[a-z0-9-]+)`", s): out.append(("a2", rown[m.group(1)], m.group(2)))
    seen = set(); uniq = []
    for x in out:
        if (x[1], x[2]) not in seen and x[1] in BY and x[2] in BY and x[1] != x[2]: seen.add((x[1], x[2])); uniq.append(x)
    return uniq
KIN = kin()
print()
print("== triage's own notes of a row that restates an earlier row (its judgment; never folded): %d pairs %s" % (len(KIN), dict(collections.Counter(k[0] for k in KIN))))
sc = sd = sboth = 0
for src, l, e in KIN:
    a, b = BY[l], BY[e]
    c = bool(set(a["c2"]) & set(b["c2"])); d = bool(set(a["cmd_door"]) & set(b["cmd_door"]))
    sc += c; sd += d; sboth += (c and d)
    print("   %s `%s` [%s] ~ `%s` [%s] · share a code: %s · share a door verb: %s · codes %s | %s" % (src, l, vtxt(a), e, vtxt(b), "yes" if c else "no", "yes" if d else "no", a["c2"], b["c2"]))
print("   share a code (row + bullet): %d of %d · share a door verb: %d · both: %d · either row carries a code at all: %d" % (sc, len(KIN), sd, sboth, sum(1 for _, l, e in KIN if BY[l]["c2"] or BY[e]["c2"])))
vp = [(BY[l], BY[e]) for _, l, e in KIN if verdicts(BY[l]) and verdicts(BY[e])]
print("   both rows verified: %d pairs · verdict pairs: %s" % (len(vp), dict(collections.Counter(tuple(sorted((OUT[verdicts(a)[-1][1]], OUT[verdicts(b)[-1][1]]))) for a, b in vp))))
