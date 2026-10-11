"""bullets.py — the reporters' own left-open text.

(1) Every top-level bullet of a report's `Left open…` section (verifier, advocate, drive, scope; both attempts).
(2) For a ledger entry whose pointer says `item N` of that section: bullet N, accepted only if it passes a
    mechanical check — of all bullets of the section, bullet N shares the most content words with triage's
    line for the entry (ties accepted). No reading.
Output: bullets.json (all bullets), resolved.json (key -> bullet text or null, with the reason).
"""
import sys, os, glob
sys.path.insert(0, os.path.dirname(__file__))
from common import *

def left_open(text):
    m = re.search(r"^(#+) +Left open[^\n]*\n(.*?)(?=^#{1,6} |\Z)", text, re.S | re.M)
    if not m: return None
    parts = re.split(r"(?m)^(?:[-*] |\d+\. )", m.group(2))
    return [x.strip() for x in parts[1:]]

STOP = set("the a an of to in is it and or that this with for on at as by its not no was were be are from which what one two".split())
def words(t): return {w for w in re.findall(r"[a-z][a-z0-9-]{2,}", t.lower()) if w not in STOP}

def main():
    here = os.path.dirname(__file__)
    allb = {}
    for d, suf in ((CAN + "/run/r1/reports/test", ".a1.md"), (CAN + "/run-pending-a2", ".a2.md")):
        for fn in sorted(glob.glob(p(d) + "/*" + suf)):
            name = os.path.basename(fn)[:-len(suf)]
            if name.startswith(("triage", "attempt", "preflight")): continue
            b = left_open(open(fn).read())
            if b is not None: allb[(name, suf[1:3])] = b
    json.dump({"%s|%s" % k: v for k, v in allb.items()}, open(here + "/bullets.json", "w"), indent=1)
    items = json.load(open(here + "/items.json"))
    res = {}; why = {}
    import collections
    c = collections.Counter()
    for i in items:
        ptr = i["ledger_pointer"]
        m = re.search(r"([a-z0-9-]+)\.a1\.md", ptr)
        nm = re.search(r"\bitem (\d+)\b", ptr)
        if "left-open" not in i["source_kind"] or not m: res[i["key"]] = None; why[i["key"]] = "not a left-open entry"; c["n/a"] += 1; continue
        if not nm or "the return's left-open item" in ptr: res[i["key"]] = None; why[i["key"]] = "pointer names no item of the section"; c["no item index"] += 1; continue
        b = allb.get((m.group(1), "a1")); n = int(nm.group(1))
        if not b or n > len(b): res[i["key"]] = None; why[i["key"]] = "no such bullet"; c["no such bullet"] += 1; continue
        tw = words(i["why_return"] + " " + i["triage_entry"] + " " + i["key"].replace("-", " "))
        sc = [len(tw & words(x)) for x in b]
        if sc[n - 1] == max(sc) and max(sc) > 0:
            res[i["key"]] = b[n - 1]; why[i["key"]] = "bullet %d, best overlap" % n; c["resolved"] += 1
        else:
            res[i["key"]] = None; why[i["key"]] = "bullet %d is not the best overlap" % n; c["failed the overlap check"] += 1
    json.dump({"text": res, "why": why}, open(here + "/resolved.json", "w"), indent=1)
    print("reports with a Left open section:", len(allb), "bullets:", sum(len(v) for v in allb.values()))
    print(" attempt 1:", sum(len(v) for k, v in allb.items() if k[1] == "a1"), "in", sum(1 for k in allb if k[1] == "a1"), "reports; attempt 2:", sum(len(v) for k, v in allb.items() if k[1] == "a2"), "in", sum(1 for k in allb if k[1] == "a2"))
    print(dict(c))
if __name__ == "__main__": main()
