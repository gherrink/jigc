#!/usr/bin/env python3
"""Aggregate a directory of workflow-eval runs into the behavioral report.

Given a results dir whose immediate subdirectories are each one run (a
`transcript.jsonl` from run-eval.sh, optionally a `jigc-validate.after.txt`), this
computes the four headline metrics the pilot showed the workflow layer had never been
tested on:

  - engage-rate    — fraction of runs that invoked `jigc` at all
  - select-rate    — fraction that picked --expected-workflow (of those that selected)
  - complete-rate  — fraction that reached `jigc task finalize`
  - outcome        — clean / drift / blocked / unknown distribution

"The CLI composes it correctly" is a `cargo test`; THIS is the missing test that a real
agent *uses* it correctly. Re-run it per workflow on a pinned model to track the gap.

Usage:
    eval.py RESULTS_DIR [--expected-workflow NAME] [--old-symbol NAME --doc REL ...]
                        [--md REPORT.md]

`--doc REL` names a managed-doc path *relative to each run dir* to grep for the old
symbol (the non-jigc outcome path); when a run has a `jigc-validate.after.txt` that
takes precedence (the Phase-2 floor oracle). Pure-stdlib.
"""
from __future__ import annotations

import argparse
import json
from pathlib import Path

from analyze import analyze_transcript, classify_outcome


def _run_dirs(results: Path):
    for child in sorted(results.iterdir()):
        if child.is_dir() and any((child / n).exists()
                                  for n in ("transcript.jsonl", "transcript.json")):
            yield child


def _transcript(run: Path) -> str:
    for name in ("transcript.jsonl", "transcript.json"):
        p = run / name
        if p.exists():
            return p.read_text(encoding="utf-8", errors="replace")
    return ""


def evaluate(results: Path, expected_workflow=None, old_symbol=None, docs=None) -> dict:
    rows = []
    for run in _run_dirs(results):
        rec = analyze_transcript(_transcript(run))
        vfile = run / "jigc-validate.after.txt"
        doc_texts = None
        if docs:
            doc_texts = [(run / d).read_text(encoding="utf-8", errors="replace")
                         for d in docs if (run / d).exists()]
        rec["outcome"] = classify_outcome(
            validate_text=vfile.read_text(encoding="utf-8", errors="replace") if vfile.exists() else None,
            old_symbol=old_symbol, doc_texts=doc_texts)
        rec["run"] = run.name
        rows.append(rec)

    n = len(rows)
    streamed = [r for r in rows if r["format"] == "stream"]
    ns = len(streamed)
    engaged = [r for r in streamed if r["engaged"]]
    selected = [r for r in streamed if r["selected_workflow"]]
    finalized = [r for r in streamed if r["finalized"]]

    def rate(num, den):
        return None if den == 0 else round(num / den, 3)

    outcomes: dict[str, int] = {}
    for r in rows:
        outcomes[r["outcome"]] = outcomes.get(r["outcome"], 0) + 1

    on_expected = None
    if expected_workflow is not None and selected:
        on_expected = rate(sum(1 for r in selected
                               if r["selected_workflow"] == expected_workflow), len(selected))

    return {
        "n_runs": n,
        "n_with_tool_stream": ns,
        "engage_rate": rate(len(engaged), ns),
        "select_count": len(selected),
        "selected_workflows": _tally(r["selected_workflow"] for r in selected),
        "expected_workflow": expected_workflow,
        "select_rate_on_expected": on_expected,
        "complete_rate": rate(len(finalized), ns),
        "outcomes": outcomes,
        "mean_cost_usd": _mean(r["cost_usd"] for r in rows),
        "mean_turns": _mean(r["num_turns"] for r in rows),
        "rows": rows,
    }


def _tally(values):
    out: dict[str, int] = {}
    for v in values:
        if v is None:
            continue
        out[v] = out.get(v, 0) + 1
    return dict(sorted(out.items(), key=lambda kv: (-kv[1], kv[0])))


def _mean(values):
    nums = [v for v in values if isinstance(v, (int, float))]
    return round(sum(nums) / len(nums), 4) if nums else None


def to_markdown(report: dict) -> str:
    lines = ["# Workflow-eval report", ""]
    lines.append(f"- runs: **{report['n_runs']}** ({report['n_with_tool_stream']} with a tool stream)")
    lines.append(f"- engage-rate: **{_pct(report['engage_rate'])}**")
    if report["expected_workflow"]:
        lines.append(f"- select-rate on `{report['expected_workflow']}`: "
                     f"**{_pct(report['select_rate_on_expected'])}** "
                     f"({report['select_count']} runs selected a workflow)")
    lines.append(f"- selected workflows: {report['selected_workflows'] or '—'}")
    lines.append(f"- complete-rate (reached finalize): **{_pct(report['complete_rate'])}**")
    lines.append(f"- outcomes: {report['outcomes']}")
    lines.append(f"- mean cost: ${report['mean_cost_usd']} · mean turns: {report['mean_turns']}")
    lines.append("")
    lines.append("| run | engaged | selected | finalized | outcome | turns | cost |")
    lines.append("|---|---|---|---|---|---|---|")
    for r in report["rows"]:
        lines.append(f"| {r['run']} | {_b(r['engaged'])} | {r['selected_workflow'] or '—'} | "
                     f"{_b(r['finalized'])} | {r['outcome']} | {r['num_turns']} | "
                     f"{r['cost_usd']} |")
    return "\n".join(lines) + "\n"


def _pct(x):
    return "n/a" if x is None else f"{round(x * 100)}%"


def _b(x):
    return "—" if x is None else ("yes" if x else "no")


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("results_dir")
    ap.add_argument("--expected-workflow")
    ap.add_argument("--old-symbol")
    ap.add_argument("--doc", action="append", default=[])
    ap.add_argument("--md", help="also write the markdown report here")
    args = ap.parse_args()

    report = evaluate(Path(args.results_dir), args.expected_workflow,
                      args.old_symbol, args.doc or None)
    printable = {k: v for k, v in report.items() if k != "rows"}
    print(json.dumps(printable, indent=2))
    if args.md:
        Path(args.md).write_text(to_markdown(report), encoding="utf-8")
        print(f"\nwrote {args.md}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
