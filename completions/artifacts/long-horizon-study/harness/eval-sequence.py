#!/usr/bin/env python3
"""Aggregate the long-horizon matrix into the headline cumulative-drift curve + rates.

Reads every runs/matrix/<arm>-<model>-rep<r>/sequence.json and produces, per
(arm, model): the per-edit cumulative drift curve (mean dangling anchors and mean
stale-prose names in the committed HEAD at each edit), final-state drift, tickets
landed, jigc engagement / hook blocks / --no-verify, and cost. Emits JSON + markdown.

Usage:  eval-sequence.py <matrix-dir> [--md out.md]
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
    """Mean of timeline[n][key] across the reps, per edit."""
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
            "stale_curve": curve(group, "n_stale", nedits),
            "final_dangling": mean([s.get("final_dangling") for s in group]),
            "final_stale": mean([s.get("final_stale") for s in group]),
            "edits_with_drift": mean([s.get("edits_with_drift") for s in group]),
            "commits_total": mean([s.get("commits_total") for s in group]),
            "any_engaged": sum(1 for s in group if s.get("any_engaged")),
            "any_blocked": sum(1 for s in group if s.get("any_blocked")),
            "any_no_verify": sum(1 for s in group if s.get("any_no_verify")),
            "total_cost": mean([s.get("total_cost") for s in group]),
            "final_drift_reps": sum(1 for s in group if s.get("final_drift")),
        }
    return {"nedits": nedits, "cells": cells}


def markdown(agg: dict) -> str:
    L = ["# Long-horizon matrix — results\n"]
    L.append("## Final-state drift + behavioral rates (mean over reps)\n")
    L.append("| arm | model | reps | final dangling | final stale | edits w/ drift | tickets landed | engaged | blocked | --no-verify | cost |")
    L.append("|---|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|")
    def order(k):
        arm = k.split("|")[0]
        return ({"P":0,"C40":1,"C160":2,"C550":3,"A":4}.get(arm,9), k)
    for k in sorted(agg["cells"], key=order):
        c = agg["cells"][k]
        L.append(f"| {c['arm']} | {c['model'].split('-')[-1]} | {c['reps']} | "
                 f"{c['final_dangling']} | {c['final_stale']} | {c['edits_with_drift']} | "
                 f"{c['commits_total']} | {c['any_engaged']}/{c['reps']} | "
                 f"{c['any_blocked']}/{c['reps']} | {c['any_no_verify']}/{c['reps']} | "
                 f"${c['total_cost']} |")
    L.append("\n## Cumulative dangling-anchor curve (mean dangling anchors in HEAD after edit N)\n")
    n = agg["nedits"]
    L.append("| arm | model | " + " | ".join(f"e{i+1}" for i in range(n)) + " |")
    L.append("|---|---|" + "--:|"*n)
    for k in sorted(agg["cells"], key=order):
        c = agg["cells"][k]
        L.append(f"| {c['arm']} | {c['model'].split('-')[-1]} | " +
                 " | ".join(str(x) for x in c["dangling_curve"]) + " |")
    L.append("\n## Cumulative stale-prose curve (mean dead names in HEAD doc after edit N)\n")
    L.append("| arm | model | " + " | ".join(f"e{i+1}" for i in range(n)) + " |")
    L.append("|---|---|" + "--:|"*n)
    for k in sorted(agg["cells"], key=order):
        c = agg["cells"][k]
        L.append(f"| {c['arm']} | {c['model'].split('-')[-1]} | " +
                 " | ".join(str(x) for x in c["stale_curve"]) + " |")
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
