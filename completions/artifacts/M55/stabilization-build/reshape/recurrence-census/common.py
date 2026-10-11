"""Shared readers for the recurrence census. Read-only on the repository."""
import csv, re, os, json, sys
# the repository root is passed in the environment variable CENSUS_REPO; every path below is repository-relative
CAN = "completions/artifacts/M55/stabilization-build/canary-one"
def p(rel): return os.path.join(os.environ["CENSUS_REPO"], rel)
def tsv(rel):
    with open(p(rel), newline="") as f:
        return list(csv.DictReader(f, delimiter="\t"))
def read(rel):
    with open(p(rel)) as f: return f.read()
def ledger_rows():
    rows = []
    for line in read(CAN + "/run/ledger.md").splitlines():
        if not line.startswith("| `"): continue
        cells = [c.strip() for c in line.strip().strip("|").split(" | ")]
        rows.append(cells)
    return rows
