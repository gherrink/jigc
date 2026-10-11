"""items.py — build the item table (one row per ledger entry) with the texts a key is read from.

Inputs (repository-relative, under canary-one/):
  data/entries.tsv, run/ledger.md, run/r1/reports/test/triage-p{1..4}.a1.md,
  data/verified-attempt-1.tsv, data/verified-attempt-2-not-on-record.tsv
Output: items.json beside this script.
No judgment: every field is a column of a committed table or a line located by the entry's key.
"""
import sys, os
sys.path.insert(0, os.path.dirname(__file__))
from common import *

def source_refs(cell):
    """ledger `source` cell -> list of (reporter, item-id). Mechanical split on ';' and ' and left open'."""
    refs = []
    cell = re.sub(r"\([^)]*\)", "", cell)          # parenthesised remarks carry no reference
    for part in cell.split(";"):
        part = part.strip()
        if not part: continue
        if part.startswith("seeded at the opening"):
            refs.append(("ledger", "seed")); continue
        m = re.match(r"(scope), left open (\d+)", part)
        if m: refs.append((m.group(1), "lo" + m.group(2))); continue
        m = re.match(r"((?:verify|advocate|proposal)-p\d-[a-z0-9-]+?)(?: -)? left open (\d+)(?: and left open (\d+))?", part)
        if m:
            refs.append((m.group(1), "lo" + m.group(2)))
            if m.group(3): refs.append((m.group(1), "lo" + m.group(3)))
            continue
        m = re.match(r"((?:row|arm|cross)-[a-z-]+) ([A-Z]+-?\d+)", part)
        if m: refs.append((m.group(1), m.group(2))); continue
        refs.append(("UNPARSED", part))
    return refs

def triage_texts(path):
    """key -> {'entry': text, 'redrive': text}: the lines of one triage report that are keyed to an entry.
    A keyed line is a table row whose first key-shaped cell is the key, a heading that names the key in
    backticks, or a paragraph opening with **`key`**. Section `To verify`/`To the verifier` is 're-drive'."""
    out = {}
    cur_h2 = ""; cur_key = None; mode = None
    def add(key, kind, line):
        out.setdefault(key, {"entry": [], "redrive": []})[kind].append(line)
    for line in read(path).splitlines():
        if line.startswith("## "):
            cur_h2 = line; cur_key = None; mode = None; continue
        kind = "redrive" if re.match(r"## To (verify|the verifier)", cur_h2) else "entry"
        m = re.match(r"#{3,4} +(?:\d+\. )?`((?:r1-|canary-)[a-z0-9-]+)`", line)
        if m: cur_key = m.group(1); mode = "section"; add(cur_key, kind, line); continue
        if line.startswith("#"):
            cur_key = None; mode = None; continue
        m = re.match(r"\*\*`((?:r1-|canary-)[a-z0-9-]+)`\*\*", line)
        if m: cur_key = m.group(1); mode = "para"; add(cur_key, kind, line); continue
        m = re.match(r"\|(?: *\d+ *\|)? *`?((?:r1-|canary-)[a-z0-9-]+)`? *\|", line)
        if m:
            add(m.group(1), kind, line); cur_key = None; mode = None; continue
        if mode == "para" and (line.startswith("- ") or line.startswith("  ")):
            add(cur_key, kind, line); continue
        if mode == "para" and line.strip() == "":
            continue
        if mode == "section":
            add(cur_key, kind, line); continue
        if mode == "para":
            cur_key = None; mode = None
    return {k: {a: "\n".join(b) for a, b in v.items()} for k, v in out.items()}

def main():
    E = tsv(CAN + "/data/entries.tsv")
    L = {r[0].strip("`"): r for r in ledger_rows()}
    T = {}
    for n in (1, 2, 3, 4):
        T["p%d" % n] = triage_texts(CAN + "/run/r1/reports/test/triage-p%d.a1.md" % n)
    v1 = {r["key"]: r for r in tsv(CAN + "/data/verified-attempt-1.tsv")}
    v2 = {r["key"]: r for r in tsv(CAN + "/data/verified-attempt-2-not-on-record.tsv")}
    items = []
    for e in E:
        k = e["key"]; l = L[k]; t = T[e["triage_pass"]].get(k, {"entry": "", "redrive": ""})
        items.append({
            "key": k, "pass": int(e["triage_pass"][1]), "source_kind": e["source_kind"],
            "door": e["door"], "clause": e["clause"], "grade": e["triage_grade"],
            "refs": source_refs(l[3]), "ledger_source": l[3], "ledger_pointer": l[9],
            "why_return": e["triage_why"], "triage_entry": t["entry"], "triage_redrive": t["redrive"],
            "v1": v1.get(k, {}).get("verdict", ""), "v1_regression": v1.get(k, {}).get("regression", ""),
            "v2_status": v2.get(k, {}).get("attempt_2_status", ""), "v2": v2.get(k, {}).get("attempt_2_verdict", ""),
            "key_shape": e["key_shape_heuristic"],
        })
    json.dump(items, open(os.path.join(os.path.dirname(__file__), "items.json"), "w"), indent=1)
    # self-checks, printed
    import collections
    print("entries", len(items))
    print("source refs", sum(len(i["refs"]) for i in items), "unparsed", [r for i in items for r in i["refs"] if r[0] == "UNPARSED"])
    print("refs by pass", collections.Counter(i["pass"] for i in items for r in i["refs"]))
    print("no triage entry text", [i["key"] for i in items if not i["triage_entry"].strip()][:20], sum(1 for i in items if not i["triage_entry"].strip()))
    print("with redrive text", collections.Counter((i["pass"], bool(i["triage_redrive"].strip())) for i in items if i["grade"] in ("breaks", "unclear")))
    print("verified a1", sum(1 for i in items if i["v1"]), "a2", sum(1 for i in items if i["v2_status"]))
    import statistics
    print("median chars: return why", statistics.median(len(i["why_return"]) for i in items), "triage entry", statistics.median(len(i["triage_entry"]) for i in items))
if __name__ == "__main__":
    main()
