"""keys.py — the mechanical key: code-shaped tokens and commands, by regular expression only.

Vocabularies (all read from the tree, none typed here):
  SITES   = the `code` column of crates/cli/tests/route_catalog/sites.tsv (minus `dynamic`)
  SRC     = every string literal of code shape in crates/cli/src and crates/engine/src (*.rs)
  VERBS   = the leaf verbs of VERB_KINDS in crates/cli/src/cli.rs
"""
import sys, os, glob
sys.path.insert(0, os.path.dirname(__file__))
from common import *

SEG = r"[a-z][a-z0-9]*(?:-[a-z0-9]+)*"
# a code-shaped token: two or three lowercase hyphenated words joined by dots, not part of a
# longer word, path, method call or dotted name
CODE = re.compile(r"(?<![\w./<$-])(" + SEG + r"(?:\." + SEG + r"){1,2})(?![\w/(-]|\.[A-Za-z0-9])")

def sites():
    rows = [l.split("\t") for l in read("crates/cli/tests/route_catalog/sites.tsv").splitlines() if l and not l.startswith("#")]
    return rows

def site_codes():
    return {r[3] for r in sites() if r[3] != "dynamic"}

def src_literals():
    lit = re.compile(r'"(' + SEG + r"(?:\." + SEG + r'){1,2})"')
    out = {}
    for root in ("crates/cli/src", "crates/engine/src"):
        for fn in glob.glob(p(root) + "/**/*.rs", recursive=True):
            for m in lit.finditer(open(fn).read()):
                out.setdefault(m.group(1), set()).add(os.path.relpath(fn, p("")))
    return out

def verbs():
    txt = read("crates/cli/src/cli.rs")
    m = re.search(r"pub const VERB_KINDS[^=]*=\s*&\[(.*?)\n\];", txt, re.S)
    out = []
    for mm in re.finditer(r"\(&\[([^\]]*)\], VerbKind::", m.group(1)):
        out.append(" ".join(re.findall(r'"([a-z-]+)"', mm.group(1))))
    return out

_V = None
def commands(text):
    """every `jigc <leaf verb>` the text spells, longest match first"""
    global _V
    if _V is None: _V = sorted(verbs(), key=len, reverse=True)
    found = set()
    for m in re.finditer(r"\bjigc ((?:[a-z-]+)(?: [a-z-]+)?)", text):
        s = m.group(1)
        for v in _V:
            if s == v or s.startswith(v + " ") :
                found.add(v); break
    return sorted(found)

def code_tokens(text):
    return sorted(set(CODE.findall(text)))

if __name__ == "__main__":
    S = site_codes(); L = src_literals(); V = verbs()
    print("sites rows", len(sites()), "distinct codes", len(S), "dynamic rows", sum(1 for r in sites() if r[3] == "dynamic"))
    print("source code-shaped literals", len(L), "of them in sites", len(S & set(L)), "sites codes not a literal", sorted(S - set(L)))
    print("leaf verbs", len(V))
