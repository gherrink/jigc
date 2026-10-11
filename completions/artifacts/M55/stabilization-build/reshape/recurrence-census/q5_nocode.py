"""q5_nocode.py — rows without a code: which other mechanical keys exist, and what each gives. Inputs: model.py.
Every key below is a regular expression over the row text or a committed column; none is a reading."""
import sys, os, statistics; sys.path.insert(0, os.path.dirname(__file__))
from model import *
NC = [i for i in items if not i["c1"]]
print("rows without a code (the key at row depth):", len(NC), "of", len(items), "· verified runs among them:", sum(1 for r in RUNS if not r["item"]["c1"]))
def k_door(i): return set(i["cmd_door"])
def k_cmd(i): return set(i["cmd_text"])
def k_src(i): return set(re.findall(r"(?:crates/[\w/-]+/)?([a-z_]+\.rs)\b", d1(i)))
def k_srcpath(i): return set(re.findall(r"crates/[\w/-]+/[a-z_]+\.rs", d1(i)))
def k_ident(i): return set(x for x in re.findall(r"`([a-z]+(?:_[a-z0-9]+)+)(?:\(\))?`", d1(i)))
def k_gitcfg(i): return set(x.lower() for x in re.findall(r"\b(core\.[A-Za-z]+|commit\.[A-Za-z]+|i18n\.[A-Za-z]+|protocol\.[a-z.]+)\b", d1(i)))
def k_design(i): return set(re.findall(r"(?:design|implementation)/[a-z-]+\.md", d1(i)))
def k_repro(i): return set((m.group(1), m.group(2)) for m in [re.search(r"([a-z0-9-]+)\.a1\.md.*?[Rr]epro ([A-Z][A-Z0-9-]*-?\d+[a-z]?)", i["ledger_pointer"])] if m)
STOPW = set("r1 p2 p3 p4 not driven undriven and of the a on in to by no at is for other route".split())
def k_slug3(i):
    w = [x for x in i["key"].split("-") if x not in ("r1", "p2", "p3", "p4")]
    return set("-".join(w[n:n + 3]) for n in range(len(w) - 2))
def k_slug2(i):
    w = [x for x in i["key"].split("-") if x not in ("r1", "p2", "p3", "p4")]
    return set("-".join(w[n:n + 2]) for n in range(len(w) - 1) if w[n] not in STOPW and w[n + 1] not in STOPW)
KEYS = [("door verb (the ledger's door cell)", k_door), ("`jigc <verb>` spelled in the row text", k_cmd), ("source file named (`x.rs`)", k_src),
        ("backticked snake_case identifier", k_ident), ("git config name", k_gitcfg), ("design/implementation doc named", k_design),
        ("the repro block the pointer names (report + block id)", k_repro), ("three consecutive words of the key", k_slug3), ("two consecutive words of the key", k_slug2)]
print()
print("| key tried | no-code rows that have it | distinct values | values on one row only | median rows per value | largest value (rows) | rows sharing a value with another row |")
print("|---|---:|---:|---:|---:|---|---:|")
for name, f in KEYS:
    occ = collections.defaultdict(list)
    for i in NC:
        for v in f(i): occ[v].append(i)
    have = sum(1 for i in NC if f(i))
    if not occ: print("| %s | 0 | 0 | | | | |" % name); continue
    big = max(occ.items(), key=lambda kv: len(kv[1]))
    shared = sum(1 for i in NC if any(len(occ[v]) > 1 for v in f(i)))
    print("| %s | %d (%.0f%%) | %d | %d | %s | `%s` (%d) | %d |" % (name, have, 100.0 * have / len(NC), len(occ), sum(1 for v in occ.values() if len(v) == 1), statistics.median(len(v) for v in occ.values()), big[0] if isinstance(big[0], str) else " ".join(big[0]), len(big[1]), shared))
print()
# does recurrence under each key say anything about the verdict, for the verified rows without a code
print("verified runs without a code, by how many distinct agents had a row with the same key value up to the horizon (max over the row's values; the row's own reporter counts)")
print("| key | no value | 1 | 2 | 3-5 | 6+ |")
print("|---|---|---|---|---|---|")
for name, f in KEYS:
    g = collections.defaultdict(list)
    for r in RUNS:
        i = r["item"]
        if i["c1"]: continue
        vs = f(i)
        if not vs: g["none"].append(r); continue
        n = 0
        for v in vs:
            ags = set(a for j in items if j["pass"] <= r["horizon"] and v in f(j) for a in j["agents"])
            n = max(n, len(ags))
        g["1" if n <= 1 else "2" if n == 2 else "3-5" if n <= 5 else "6+"].append(r)
    print("| %s | %s | %s | %s | %s | %s |" % (name, *[rate(g[b]) if g[b] else "-" for b in ("none", "1", "2", "3-5", "6+")]))
print()
# the two-word key: the values that recur most, as examples of what it groups
occ = collections.defaultdict(list)
for i in items:
    for v in k_slug2(i): occ[v].append(i)
print("two consecutive key words, all 372 rows — the 25 most frequent values (rows · distinct agents · verified runs confirmed/all):")
for v, rows in sorted(occ.items(), key=lambda kv: -len(kv[1]))[:25]:
    rs = [r for r in RUNS if r["item"] in rows]
    print("   %-28s %2d · %2d · %s" % (v, len(rows), len(set(a for i in rows for a in i["agents"])), rate(rs)))
occ3 = collections.defaultdict(list)
for i in items:
    for v in k_slug3(i): occ3[v].append(i)
print("three consecutive key words — the 15 most frequent:")
for v, rows in sorted(occ3.items(), key=lambda kv: -len(kv[1]))[:15]:
    rs = [r for r in RUNS if r["item"] in rows]
    print("   %-40s %2d · %2d · %s" % (v, len(rows), len(set(a for i in rows for a in i["agents"])), rate(rs)))
# door: rows per door, verified confirm rate per door
print()
print("door verb over all 372 rows (rows · distinct agents · confirmed/verified runs):")
occd = collections.defaultdict(list)
for i in items:
    for v in k_door(i): occd[v].append(i)
for v, rows in sorted(occd.items(), key=lambda kv: -len(kv[1])):
    rs = [r for r in RUNS if r["item"] in rows]
    print("   %-22s %3d · %2d · %s" % (v, len(rows), len(set(a for i in rows for a in i["agents"])), rate(rs)))
# does the key-word key find triage's kin pairs?
