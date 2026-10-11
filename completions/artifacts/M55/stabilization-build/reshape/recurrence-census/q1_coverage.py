"""q1_coverage.py — the mechanical key and its coverage. Inputs: model.py (which names its own)."""
import sys, os; sys.path.insert(0, os.path.dirname(__file__))
from model import *

def cov(texts):
    n = len(texts); anytok = nofile = vocab = st = so = 0
    for t in texts:
        toks = K.code_tokens(t); cl = [classify(x) for x in toks]
        anytok += bool(toks); nofile += any(c != "file-name" for c in cl)
        vocab += any(c in ("sites", "source-only") for c in cl); st += "sites" in cl; so += ("source-only" in cl and "sites" not in cl)
    return n, anytok, nofile, vocab, st, so
def line(name, texts):
    n, a, b, v, s, o = cov(texts)
    print("| %s | %d | %d | %d | **%d** (%.0f%%) | %d | %d |" % (name, n, a, b, v, 100.0 * v / n if n else 0, s, o))

def hunt_sections():
    """the 47 instrument findings, each by its id: the heading section, the `- **ID …` bullet, or the `| ID |` row"""
    out = {}
    for i in items:
        for rep, fid in i["refs"]:
            if rep not in INSTR: continue
            txt = read(CAN + "/run/r1/reports/test/%s.a1.md" % rep); got = []
            lines = txt.splitlines()
            for n, l in enumerate(lines):
                m = re.match(r"(#{2,4}) ", l)
                if m and re.search(r"(?<![\w-])%s(?![\w-])" % re.escape(fid), l):
                    lvl = len(m.group(1)); j = n + 1
                    while j < len(lines) and not (re.match(r"(#{1,6}) ", lines[j]) and len(re.match(r"(#{1,6}) ", lines[j]).group(1)) <= lvl): j += 1
                    got.append("\n".join(lines[n:j]))
                elif re.match(r"- \*\*%s[ (]" % re.escape(fid), l):
                    j = n + 1
                    while j < len(lines) and (lines[j].startswith("  ") or lines[j].strip() == "") and not lines[j].startswith("- "): j += 1
                    got.append("\n".join(lines[n:j]))
                elif re.match(r"\| *%s *\|" % re.escape(fid), l): got.append(l)
            out[(rep, fid)] = "\n".join(got)
    return out

print("## vocabularies")
print("sites.tsv rows", len(K.sites()), "· distinct codes", len(SITES), "· rows whose code is `dynamic`", sum(1 for r in K.sites() if r[3] == "dynamic"))
print("code-shaped string literals in crates/cli/src + crates/engine/src:", len(SRCLIT), "· minus file-name shapes:", sum(1 for x in SRCLIT if not is_ext(x)), "· of those not in sites.tsv:", sum(1 for x in SRCLIT if not is_ext(x) and x not in SITES))
print()
print("| item class (text) | items | any code-shaped token | … not a file name | a code of the vocabulary | of sites.tsv | source-only |")
print("|---|---:|---:|---:|---:|---:|---:|")
H = hunt_sections()
print("hunt ids resolved", sum(1 for v in H.values() if v), "of", len(H), [k for k, v in H.items() if not v])
line("hunt findings, the reporter's own section (47 ids)", [v for v in H.values()])
line("triage rows, pass 1 (hunt, scope, seed) — row text", [d1(i) for i in items if i["pass"] == 1])
line("triage rows, all — row text, dotted tokens only", [d1(i) for i in items])
print("| triage rows, all — THE KEY (dotted tokens, or the code spelled with hyphens in the row's key) | %d | | | **%d** (%.0f%%) | %d | %d |" % (len(items), sum(bool(i["c1"]) for i in items), 100.0*sum(bool(i["c1"]) for i in items)/len(items), sum(any(c in SITES for c in i["c1"]) for i in items), sum(bool(i["c1"]) and not any(c in SITES for c in i["c1"]) for i in items)))
print("| triage rows, all — the key + the reporter's bullet | %d | | | **%d** (%.0f%%) | %d | %d |" % (len(items), sum(bool(i["c2"]) for i in items), 100.0*sum(bool(i["c2"]) for i in items)/len(items), sum(any(c in SITES for c in i["c2"]) for i in items), sum(bool(i["c2"]) and not any(c in SITES for c in i["c2"]) for i in items)))
print("rows the key-slug form adds:", [(i["key"], i["slug"]) for i in items if i["slug"] and not set(i["slug"]) <= set(i["c1d"])])
fu = [i for i in items if i["source_kind"] in ("verifier left-open", "advocate left-open", "independent-drive left-open")]
line("follow-up rows — row text", [d1(i) for i in fu])
line("follow-up rows — row text + the reporter's bullet (where resolved)", [d2(i) for i in fu])
line("follow-up rows whose bullet resolved — the bullet alone", [resolved[i["key"]] for i in fu if resolved.get(i["key"])])
line("left-open bullets as written, attempt 1", [b for k, v in bullets.items() if k[1] == "a1" for b in v])
line("left-open bullets as written, attempt 2", [b for k, v in bullets.items() if k[1] == "a2" for b in v])
line("the 80 verifier runs — row text", [d1(r["item"]) for r in RUNS])
line("the 80 verifier runs — row text + re-drive line", [handed(r["item"], r["att"]) for r in RUNS])
line("the 80 verifier runs — the verifier's basis (after the verdict)", [r["basis"] for r in RUNS])
line("whole reports (90 agents)", [t for _, t, _ in REPORT.values()])
print()
# distinct tokens
tok = collections.Counter(x for i in items for x in i["tok1"])
by = collections.defaultdict(list)
for t, n in tok.items(): by[classify(t)].append((n, t))
print("distinct code-shaped tokens in the 372 rows' text:", len(tok))
for c in ("sites", "source-only", "file-name", "other"):
    print("  %s: %d distinct, %d row-occurrences — %s" % (c, len(by[c]), sum(n for n, _ in by[c]), ", ".join("%s×%d" % (t, n) for n, t in sorted(by[c], reverse=True)[:40])))
tokr = collections.Counter(x for _, t, _ in REPORT.values() for x in set(K.code_tokens(t)))
byr = collections.Counter(classify(t) for t in tokr)
print("distinct code-shaped tokens in the 90 reports:", len(tokr), dict(byr))
print()
# commands
print("rows whose door cell spells >=1 leaf verb:", sum(bool(i["cmd_door"]) for i in items), "· exactly one:", sum(len(i["cmd_door"]) == 1 for i in items), "· several:", sum(len(i["cmd_door"]) > 1 for i in items))
print("rows whose text spells >=1 `jigc <leaf verb>`:", sum(bool(i["cmd_text"]) for i in items), "· distinct verbs named:", len(set(v for i in items for v in i["cmd_text"])))
print("rows with a code AND a door verb:", sum(bool(i["c1"]) and bool(i["cmd_door"]) for i in items), "of", sum(bool(i["c1"]) for i in items))
print("door cell agrees with sites.tsv's command for the code (row has one door verb and a sites code):")
sc = collections.defaultdict(set)
for r in K.sites(): sc[r[3]].add(r[0])
ag = dis = 0; ex = []
for i in items:
    if len(i["cmd_door"]) == 1:
        for c in i["c1"]:
            if c in sc:
                fam = i["cmd_door"][0]
                ok = any(fam == s or fam.startswith(s + " ") or (s == "doc read" and fam in ("doc show", "doc list", "doc schema")) or (s == "doc write" and fam.startswith("doc ") and fam not in ("doc show", "doc list", "doc schema")) for s in sc[c])
                ag += ok; dis += (not ok)
                if not ok: ex.append((c, sorted(sc[c]), fam))
print("  agree", ag, "differ", dis, ex[:12])
print()
# route-named
R = [r for r in RUNS if r["route"]]; NR = [r for r in RUNS if not r["route"]]
def sh(rs, f): return "%d of %d" % (sum(bool(f(r)) for r in rs), len(rs))
print("route-named runs (the earlier census's sort; made after the verdicts were read):")
for nm, f in (("row text", lambda r: r["item"]["c1"]), ("row + re-drive", lambda r: r["ch"]), ("row + bullet", lambda r: r["item"]["c2"])):
    print("  %-16s names a route: %s carry a code · no route: %s" % (nm, sh(R, f), sh(NR, f)))
print("  by its two groups (row + re-drive):", {g: sh([r for r in RUNS if r["grp"] == g], lambda r: r["ch"]) for g in sorted(set(r["grp"] for r in RUNS))})
print("blind proxies over all 372 rows: key contains `route` -> %s carry a code; does not -> %s" % (sh([i for i in items if "route" in i["key"]], lambda i: i["c1"]), sh([i for i in items if "route" not in i["key"]], lambda i: i["c1"])))
print("   triage's line contains `route` -> %s ; does not -> %s" % (sh([i for i in items if "route" in i["why_return"].lower()], lambda i: i["c1"]), sh([i for i in items if "route" not in i["why_return"].lower()], lambda i: i["c1"])))
print("carries a code vs verdict (row + re-drive):", {o: sh([r for r in RUNS if r["out"] == o], lambda r: r["ch"]) for o in ("confirmed", "nab", "dnr", "halted")})
print("carries a code vs verdict (row text):", {o: sh([r for r in RUNS if r["out"] == o], lambda r: r["item"]["c1"]) for o in ("confirmed", "nab", "dnr", "halted")})
print("carries a code vs verdict (row + bullet):", {o: sh([r for r in RUNS if r["out"] == o], lambda r: r["item"]["c2"]) for o in ("confirmed", "nab", "dnr", "halted")})
print("confirm rate: code (row text) %s · no code %s" % (rate([r for r in RUNS if r["item"]["c1"]]), rate([r for r in RUNS if not r["item"]["c1"]])))
print("confirm rate: code (row+redrive) %s · no code %s" % (rate([r for r in RUNS if r["ch"]]), rate([r for r in RUNS if not r["ch"]])))
print("confirm rate: code (row+bullet) %s · no code %s" % (rate([r for r in RUNS if r["item"]["c2"]]), rate([r for r in RUNS if not r["item"]["c2"]])))
print("route x code (row+redrive):", {(a, b): rate([r for r in RUNS if r["route"] == a and bool(r["ch"]) == b]) for a in (True, False) for b in (True, False)})
