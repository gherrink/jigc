#!/usr/bin/env python3
"""Aggregate the cross-doc matrix into the headline cumulative DANGLING-REF curve + rates.

Reads every runs/refint-matrix/<arm>-<model>-rep<r>/sequence.json and produces, per
(arm, model): the per-edit cumulative dangling-cross-doc-ref curve (mean dangling
`supersedes`/`cites` edges in the committed HEAD at each edit — THE HEADLINE), final-state
dangling, tickets landed, hook blocks / --no-verify, control-edge violations, oracle
(path-a vs path-b) disagreements, and cost. Emits JSON + markdown.

The predicted finding: arm A's curve stays flat (~0) while the static curves rise — steeper
at higher dilution (C40 <= C160 < C550) — and plain rises fastest. A flat A vs a rising
static IS the capability-gap result the study is built to expose.

Usage:  eval-sequence-refint.py <matrix-dir> [--md out.md]
"""
from __future__ import annotations
import argparse, json, statistics, sys
from collections import defaultdict
from pathlib import Path


def load(matrix: Path) -> list[dict]:
    seqs = []
    for p in sorted(matrix.glob("*/sequence.json")):
        try:
            seqs.append(json.loads(p.read_text()))
        except Exception as e:
            print(f"skip {p}: {e}", file=sys.stderr)
    return seqs


def mean(xs):
    xs = [x for x in xs if x is not None]
    return round(statistics.mean(xs), 2) if xs else None


def curve(seqs: list[dict], key: str, nedits: int) -> list:
    out = []
    for n in range(nedits):
        vals = []
        for s in seqs:
            tl = s.get("timeline", [])
            if n < len(tl):
                vals.append(tl[n].get(key))
        out.append(mean(vals))
    return out


def aggregate(seqs: list[dict]) -> dict:
    by = defaultdict(list)
    for s in seqs:
        by[(s.get("arm"), s.get("model"))].append(s)
    nedits = max((s.get("n_edits", 8) for s in seqs), default=8)
    cells = {}
    for (arm, model), group in sorted(by.items()):
        cells[f"{arm}|{model}"] = {
            "arm": arm, "model": model, "reps": len(group),
            "dangling_curve": curve(group, "n_dangling", nedits),
            "final_dangling": mean([s.get("final_dangling") for s in group]),
            "edits_with_drift": mean([s.get("edits_with_drift") for s in group]),
            "commits_total": mean([s.get("commits_total") for s in group]),
            "any_blocked": sum(1 for s in group if s.get("any_blocked")),
            "any_no_verify": sum(1 for s in group if s.get("any_no_verify")),
            "control_violations": sum((s.get("control_violations") or 0) for s in group),
            "oracle_disagreements": sum((s.get("oracle_disagreements") or 0) for s in group),
            "total_cost": mean([s.get("total_cost") for s in group]),
        }
    return {"nedits": nedits, "cells": cells}


def _ml(model: str) -> str:
    return "sonnet" if "sonnet" in (model or "") else "opus" if "opus" in (model or "") else (model or "?")


def _order(k):
    arm = k.split("|")[0]
    return ({"P": 0, "C40": 1, "C160": 2, "C550": 3, "A": 4}.get(arm, 9), k)


def markdown(agg: dict) -> str:
    L = ["# Cross-doc forward-ref integrity matrix — results\n"]
    L.append("> Headline: arm A (jigc hook) flat ~0 vs static curves rising "
             "(C40 ≤ C160 < C550), plain fastest = the capability-gap result.\n")
    L.append("## Final-state + behavioral rates (mean over reps)\n")
    L.append("| arm | model | reps | final dangling | edits w/ drift | tickets landed | "
             "blocked | --no-verify | control✗ | oracle≠ | cost |")
    L.append("|---|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|")
    for k in sorted(agg["cells"], key=_order):
        c = agg["cells"][k]
        L.append(f"| {c['arm']} | {_ml(c['model'])} | {c['reps']} | "
                 f"{c['final_dangling']} | {c['edits_with_drift']} | {c['commits_total']} | "
                 f"{c['any_blocked']}/{c['reps']} | {c['any_no_verify']}/{c['reps']} | "
                 f"{c['control_violations']} | {c['oracle_disagreements']} | "
                 f"${c['total_cost']} |")
    n = agg["nedits"]
    L.append("\n## Cumulative dangling cross-doc refs in HEAD after edit N (THE HEADLINE)\n")
    L.append("| arm | model | " + " | ".join(f"e{i+1}" for i in range(n)) + " |")
    L.append("|---|---|" + "--:|" * n)
    for k in sorted(agg["cells"], key=_order):
        c = agg["cells"][k]
        L.append(f"| {c['arm']} | {_ml(c['model'])} | " +
                 " | ".join(str(x) for x in c["dangling_curve"]) + " |")
    L.append("\n*control✗ = control-edge violations (must be 0); oracle≠ = path-a/path-b "
             "disagreements (must be 0 — an oracle-integrity flag, not a result).*")
    return "\n".join(L) + "\n"


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("matrix"); ap.add_argument("--md")
    a = ap.parse_args()
    seqs = load(Path(a.matrix))
    if not seqs:
        print("no sequences found", file=sys.stderr); return 1
    agg = aggregate(seqs)
    print(json.dumps(agg, indent=2))
    if a.md:
        Path(a.md).write_text(markdown(agg))
        print(f"\nwrote {a.md}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
