"""model.py — the joined model every count script reads. Mechanical throughout.

Reads: items.json, bullets.json, resolved.json (made by items.py, bullets.py), the two verified tables,
       the triage reports of pass 2 and of attempt 2 (re-drive lines), the reports themselves,
       earlier-census-scratch/runs.json and groups.json (the earlier census's per-run rows; not committed).
"""
import sys, os, glob, collections
sys.path.insert(0, os.path.dirname(__file__))
from common import *
import keys as K

HERE = os.path.dirname(__file__)
EXT = set("md rs json jsonl yaml yml toml txt sh py lock log out diff tsv js git".split())
SITES = K.site_codes()
SRCLIT = set(K.src_literals())
def is_ext(tok): return tok.rsplit(".", 1)[-1] in EXT
def classify(tok):
    if tok in SITES: return "sites"
    if tok in SRCLIT and not is_ext(tok): return "source-only"
    if is_ext(tok): return "file-name"
    return "other"
def codes(text):
    """the mechanical key: code-shaped tokens that are a code of sites.tsv or a code-shaped literal of source"""
    return sorted(t for t in K.code_tokens(text) if classify(t) in ("sites", "source-only"))

VOCAB = SITES | {x for x in SRCLIT if not is_ext(x)}
def slug_codes(key):
    """a vocabulary code spelled as a run of the key's hyphen-joined words: setup.install-hook -> -setup-install-hook-"""
    k = "-" + key + "-"
    return sorted(c for c in VOCAB if "-" + c.replace(".", "-") + "-" in k)
items = json.load(open(HERE + "/items.json"))
BY = {i["key"]: i for i in items}
resolved = json.load(open(HERE + "/resolved.json"))["text"]
bullets = {tuple(k.split("|")): v for k, v in json.load(open(HERE + "/bullets.json")).items()}

# --- agents -------------------------------------------------------------------------------------
INSTR = ["row-doc-list-source", "row-doc-list-driver", "row-doc-list-reconciler", "cross-cutting-review",
         "arm-control-rehearse", "arm-control-run", "arm-control-score"]
AG = {}
for n, r in enumerate(INSTR, 1): AG[r] = "h%02d" % n
AG["scope"] = "s01"; AG["ledger"] = "seed"
V1 = tsv(CAN + "/data/verified-attempt-1.tsv"); V2 = tsv(CAN + "/data/verified-attempt-2-not-on-record.tsv")
for n, r in enumerate(V1, 1): AG["verify-%s-%s" % (r["triage_pass"], r["key"])] = "v%02d" % n
AG["advocate-p1-r1-setup-installs-outside-work-tree"] = "a01"
AG["proposal-p1-r1-setup-installs-outside-work-tree"] = "d01"
A2 = {}
for n, r in enumerate(V2, 1): A2[r["key"]] = "w%02d" % n
def handed_key(reporter):
    m = re.match(r"(?:verify|advocate|proposal)-p\d-(.*)", reporter)
    return m.group(1) if m else None

# --- re-drive lines the item parser does not key (pass 2: a numbered list; attempt 2: its own table) ----
def redrive_p2():
    out = {}
    txt = read(CAN + "/run/r1/reports/test/triage-p2.a1.md")
    sec = txt[txt.index("## To verify"):]
    sec = sec[:sec.index("\n## ", 5)]
    for m in re.finditer(r"(?m)^\d+\. `((?:r1-)[a-z0-9-]+)` — (.*)$", sec): out[m.group(1)] = m.group(2)
    return out
def triage_a2():
    out = {}
    for line in read(CAN + "/run-pending-a2/triage-p1.a2.md").splitlines():
        m = re.match(r"\| *\d+ *\| *`([a-z0-9-]+)` *\|(.*)$", line)
        if m:
            k = m.group(1)
            if not k.startswith("r1-"): k = "r1-p4-" + k
            out[k] = m.group(2)
    return out
RD2 = redrive_p2(); TA2 = triage_a2()

# --- texts ----------------------------------------------------------------------------------------
def d1(i): return " ".join([i["why_return"], i["triage_entry"], i["door"]])
def d2(i): return d1(i) + " " + (resolved.get(i["key"]) or "")
def handed(i, att):
    t = d1(i) + " " + i["triage_redrive"] + " " + RD2.get(i["key"], "")
    if att == "a2": t += " " + TA2.get(i["key"], "")
    return t
for i in items:
    i["agents"] = [AG[r] for r, _ in i["refs"]]
    i["own"] = AG[i["refs"][0][0]]
    i["parent"] = handed_key(i["refs"][0][0])
    i["c1d"] = codes(d1(i))                                   # dotted tokens only
    i["slug"] = slug_codes(i["key"])                          # the code spelled with hyphens inside the key
    i["c1"] = sorted(set(i["c1d"]) | set(i["slug"]))          # THE KEY at row depth
    i["c2"] = sorted(set(i["c1"]) | set(codes(resolved.get(i["key"]) or "")))   # … plus the reporter's own bullet
    i["tok1"] = K.code_tokens(d1(i))
    i["cmd_text"] = K.commands(d1(i)); i["cmd_door"] = K.commands(i["door"])
    i["hop"] = i["pass"] - 1
def root(k):
    seen = []
    while BY[k]["parent"] and BY[k]["parent"] in BY: seen.append(k); k = BY[k]["parent"]
    return k
for i in items: i["root"] = root(i["key"])

# --- verification runs ------------------------------------------------------------------------------
RUNS = json.load(open(HERE + "/earlier-census-scratch/runs.json"))
GRP = json.load(open(HERE + "/earlier-census-scratch/groups.json"))
for r in RUNS:
    i = BY[r["key"]]
    r["item"] = i; r["grp"] = GRP[r["att"] + ":" + r["key"]]
    r["route"] = r["grp"].startswith("route")
    r["horizon"] = i["pass"] if r["att"] == "a1" else 4       # the triage passes whose rows were on record at launch
    r["agent"] = (AG["verify-p%d-%s" % (i["pass"], r["key"])] if r["att"] == "a1" else A2[r["key"]])
    r["basis_codes"] = codes(r["basis"])
    r["ch"] = sorted(set(codes(handed(i, r["att"]))) | set(i["slug"]))
assert len(RUNS) == 80 and sum(r["out"] == "confirmed" for r in RUNS) == 18

# --- reports, whole text --------------------------------------------------------------------------
REPORT = {}      # agent id -> (available at triage pass, text, reporter name)
def _load():
    d = p(CAN + "/run/r1/reports/test")
    for r in INSTR + ["scope"]: REPORT[AG[r]] = (1, open("%s/%s.a1.md" % (d, r)).read(), r)
    for name, a in AG.items():
        m = re.match(r"(verify|advocate|proposal)-p(\d)-", name)
        if m: REPORT[a] = (int(m.group(2)) + 1, open("%s/%s.a1.md" % (d, name)).read(), name)
    for k, a in A2.items():
        REPORT[a] = (9, open(p(CAN + "/run-pending-a2/verify-p1-%s.a2.md" % k)).read(), "verify-p1-%s (attempt 2)" % k)
_load()
REPORT_CODES = {a: set(codes(t)) for a, (_, t, _) in REPORT.items()}

def rows_known(horizon): return [i for i in items if i["pass"] <= horizon]
def code_stats(c, horizon, depth="c1"):
    rs = [i for i in rows_known(horizon) if c in i[depth]]
    ags = set(a for i in rs for a in i["agents"])
    return len(rs), len(ags)
def report_agents(c, horizon):
    return sorted(a for a, (avail, _, _) in REPORT.items() if avail <= horizon and c in REPORT_CODES[a])
def band(n): return "none" if n == 0 else ("1" if n == 1 else ("2" if n == 2 else "3+"))
def rate(rs): return "%d / %d" % (sum(r["out"] == "confirmed" for r in rs), len(rs))
