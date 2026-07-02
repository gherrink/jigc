#!/usr/bin/env python3
"""Aggregate the M35 rename-cost matrix into the cost+completeness verdict.

The M35 analog of the cross-doc study's eval-sequence-refint.py. Where that study's
headline was the cumulative *dangling-ref curve* (a correctness story), THIS study's
headline is **turns/cost per rename** — the first study designed for jigc to beat plain
on *effort*, not just tie static on correctness ([ideas/cli-owned-rename.md](../../../ideas/cli-owned-rename.md)
-> Acceptance; [DECISIONS.md](../../../DECISIONS.md) -> 2026-06-28 M35 planning).

Reads every `<matrix>/*/sequence.json` (the per-rep summaries `run-sequence-refint.sh`
emits) and produces, per (arm, model):

  - **cost_per_rename** (THE HEADLINE) — `total_cost / renames-performed`, where a rename
    is performed when an edit landed a commit. jigc should be CHEAPER (one `jigc rename`
    command) where plain/static burn turns hand-editing + chasing referrers.
  - **turns_per_rename** — the same denominator over agent turns.
  - **completeness** — fraction of forward edges that RESOLVE at the final committed HEAD,
    scored over **STRUCTURED MANAGED REFS ONLY** (the determinism boundary: a prose or
    unmanaged mention is never a measured ref, so it can never count as (in)completeness).
    Derived from the measure-refint oracle's per-edit `edges_total`/`n_dangling`.
  - **rename_engagement** — `renames_via_verb / n_edits`: the M35 verb-engagement confound
    as a PRIMARY rate (availability != induced-usage). A hand-edit re-incurs the
    block->recovery cost, so a low engagement rate forfeits the cost win even if the verb
    exists. From the in-repo rename-aware extractor (analyze.py).
  - **rename_error_recoveries** — count of edits that hit a block->recovery cycle
    (`hook_blocked_seen`): the rename-error->recovery confound.
  - **reps** + **underpowered** — small-N effort noise: per-rename cost is noisy on short
    sequences, so the reps count is surfaced and a cell below `--min-reps` is flagged.

**Void-tripwires (NOT results — study-integrity gates).** Any cell with a control-edge
violation (`control_violations > 0` — the never-edited control edge dangled) or an oracle
disagreement (`oracle_disagreements > 0` — measure-refint's edge-walker path (a) and
`jigc validate` path (b) disagree) VOIDS the cell and the study: the metrics are not
trustworthy and no verdict is rendered. The tripwires fire loudly (stderr + `study_valid:
false`) rather than silently reporting a corrupted number.

**The pre-registered win** (rendered only when the study is valid): jigc strictly cheaper
per rename than plain AND completeness >= static.

Usage:
    eval-sequence-refint.py <matrix-dir> [--md out.md] [--min-reps N]
    eval-sequence-refint.py --selftest        # bundled synthetic fixtures, no API spend
"""
from __future__ import annotations
import argparse
import json
import statistics
import sys
from collections import defaultdict
from pathlib import Path

MIN_REPS = 5  # below this, per-rename cost is small-N noisy (a confound, flagged not fatal)


def load(matrix: Path) -> list[dict]:
    seqs = []
    for p in sorted(matrix.glob("*/sequence.json")):
        try:
            seqs.append(json.loads(p.read_text()))
        except Exception as e:  # noqa: BLE001 - a malformed rep is skipped, never fatal
            print(f"skip {p}: {e}", file=sys.stderr)
    return seqs


def mean(xs):
    """Order-invariant mean: filter None, SORT (so a reversed input order yields a
    byte-identical sum -> identical output), round. None for an all-empty input."""
    xs = sorted(x for x in xs if x is not None)
    return round(statistics.mean(xs), 4) if xs else None


def _renames_performed(s: dict) -> int:
    """Edits that actually landed a rename (a commit) — the cost-per-rename denominator."""
    return sum(1 for e in (s.get("timeline") or []) if (e.get("n_commits") or 0) > 0)


def _sum_turns(s: dict) -> int:
    return sum((e.get("turns") or 0) for e in (s.get("timeline") or []))


def _error_recoveries(s: dict) -> int:
    return sum(1 for e in (s.get("timeline") or []) if e.get("hook_blocked_seen"))


def _final_completeness(s: dict):
    """Resolved / total over structured managed refs at the final committed HEAD."""
    tl = s.get("timeline") or []
    if not tl:
        return None
    et = tl[-1].get("edges_total")
    nd = s.get("final_dangling")
    if nd is None:
        nd = tl[-1].get("n_dangling")
    if not et or nd is None:
        return None
    return (et - nd) / et


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


def _ml(model: str) -> str:
    # Display label: strip the vendor prefix but keep the version — the matrix can carry
    # more than one Sonnet generation (the 2026-07-02 Sonnet-5 supplementary cell).
    m = model or ""
    return (m[len("claude-"):] if m.startswith("claude-") else m) or "?"


def verdict(cells: dict) -> dict:
    """The pre-registered win, per model: jigc strictly cheaper per rename than plain AND
    completeness >= static. Withheld (null) when an arm is missing or any arm is void."""
    out = {}
    for model in sorted({c["model"] for c in cells.values()}):
        byarm = {c["arm"]: c for c in cells.values() if c["model"] == model}
        jigc, plain, static = byarm.get("jigc"), byarm.get("plain"), byarm.get("static")
        if not (jigc and plain and static):
            out[model] = {"win": None, "void": False, "note": "missing arm(s)"}
            continue
        if jigc["void"] or plain["void"] or static["void"]:
            out[model] = {"win": None, "void": True,
                          "note": "tripwire void — verdict withheld"}
            continue
        cheaper = (jigc["cost_per_rename"] is not None
                   and plain["cost_per_rename"] is not None
                   and jigc["cost_per_rename"] < plain["cost_per_rename"])
        complete_ge = (jigc["completeness"] is not None
                       and static["completeness"] is not None
                       and jigc["completeness"] >= static["completeness"])
        out[model] = {
            "void": False,
            "jigc_cheaper_than_plain": cheaper,
            "jigc_completeness_ge_static": complete_ge,
            "win": bool(cheaper and complete_ge),
        }
    return out


def aggregate(seqs: list[dict], min_reps: int = MIN_REPS) -> dict:
    by = defaultdict(list)
    for s in seqs:
        by[(s.get("arm"), s.get("model"))].append(s)
    nedits = max((s.get("n_edits", 8) for s in seqs), default=8)
    cells: dict = {}
    tripwires: list[str] = []
    for (arm, model), group in sorted(by.items()):
        cpr, tpr, comp, eng = [], [], [], []
        cviol = sum((s.get("control_violations") or 0) for s in group)
        odis = sum((s.get("oracle_disagreements") or 0) for s in group)
        for s in group:
            rp = _renames_performed(s)
            tc = s.get("total_cost")
            if rp and tc is not None:
                cpr.append(tc / rp)
            if rp:
                tpr.append(_sum_turns(s) / rp)
            c = _final_completeness(s)
            if c is not None:
                comp.append(c)
            ne = s.get("n_edits") or nedits
            rv = s.get("renames_via_verb")
            if rv is not None and ne:
                eng.append(rv / ne)
        void = (cviol > 0) or (odis > 0)
        label = f"{arm}|{_ml(model)}"
        if cviol > 0:
            tripwires.append(f"{label}: {cviol} control-edge violation(s) — STUDY VOID")
        if odis > 0:
            tripwires.append(f"{label}: {odis} oracle disagreement(s) — STUDY VOID")
        cells[f"{arm}|{model}"] = {
            "arm": arm, "model": model, "reps": len(group),
            "cost_per_rename": mean(cpr),
            "turns_per_rename": mean(tpr),
            "completeness": mean(comp),
            "rename_engagement": mean(eng),
            "rename_error_recoveries": sum(_error_recoveries(s) for s in group),
            "final_dangling": mean([s.get("final_dangling") for s in group]),
            "control_violations": cviol,
            "oracle_disagreements": odis,
            "total_cost": mean([s.get("total_cost") for s in group]),
            "dangling_curve": curve(group, "n_dangling", nedits),
            "underpowered": len(group) < min_reps,
            "void": void,
        }
    return {"nedits": nedits, "min_reps": min_reps,
            "study_valid": not tripwires, "tripwires": tripwires,
            "cells": cells, "verdict": verdict(cells)}


def _order(k: str):
    arm = k.split("|")[0]
    return ({"plain": 0, "static": 1, "jigc": 2}.get(arm, 9), k)


def _fmt(x):
    return "—" if x is None else str(x)


def markdown(agg: dict) -> str:
    L = ["# M35 rename-cost matrix — cost + completeness\n"]
    L.append("> Headline: **cost per rename** (the first study where jigc can beat plain on "
             "*effort*). Win = jigc strictly cheaper per rename than plain AND completeness "
             "≥ static. Completeness scored over **structured managed refs only**.\n")
    if not agg["study_valid"]:
        L.append("> **STUDY VOID — tripwire(s) fired:**\n")
        for t in agg["tripwires"]:
            L.append(f"> - {t}")
        L.append("")
    L.append("| arm | model | reps | cost/rename | turns/rename | completeness | "
             "rename-engagement | err→recov | final dangling | control✗ | oracle≠ | void |")
    L.append("|---|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:--:|")
    for k in sorted(agg["cells"], key=_order):
        c = agg["cells"][k]
        flag = "✗ VOID" if c["void"] else ("⚠ low-N" if c["underpowered"] else "")
        L.append(f"| {c['arm']} | {_ml(c['model'])} | {c['reps']} | "
                 f"{_fmt(c['cost_per_rename'])} | {_fmt(c['turns_per_rename'])} | "
                 f"{_fmt(c['completeness'])} | {_fmt(c['rename_engagement'])} | "
                 f"{c['rename_error_recoveries']} | {_fmt(c['final_dangling'])} | "
                 f"{c['control_violations']} | {c['oracle_disagreements']} | {flag} |")
    L.append("\n## Pre-registered win (per model)\n")
    for model, v in sorted(agg["verdict"].items()):
        if v.get("void"):
            L.append(f"- **{_ml(model)}: VOID** — {v['note']}")
        elif v.get("win") is None:
            L.append(f"- **{_ml(model)}: indeterminate** — {v['note']}")
        else:
            L.append(f"- **{_ml(model)}: {'WIN' if v['win'] else 'NO WIN'}** "
                     f"(cheaper than plain={v['jigc_cheaper_than_plain']}, "
                     f"completeness ≥ static={v['jigc_completeness_ge_static']})")
    L.append("\n*control✗ = control-edge violations (must be 0); oracle≠ = path-a/path-b "
             "disagreements (must be 0). Either > 0 voids the cell — an integrity tripwire, "
             "not a result.*")
    return "\n".join(L) + "\n"


# --------------------------------------------------------------------------- selftest
def _selftest() -> int:
    fx = Path(__file__).parent / "eval-fixtures"
    seqs = load(fx)
    if not seqs:
        print(f"FAIL: no fixtures under {fx}")
        return 1
    agg = aggregate(seqs)
    fails = 0

    def check(cond, msg):
        nonlocal fails
        if not cond:
            print(f"FAIL: {msg}")
            fails += 1

    cells = agg["cells"]
    jigc = cells.get("jigc|claude-opus-4-8")
    static = cells.get("static|claude-opus-4-8")
    plain = cells.get("plain|claude-opus-4-8")
    check(jigc is not None and static is not None and plain is not None,
          f"three arm cells present: {sorted(cells)}")

    # Headline column — cost per rename: jigc rep1 0.15/3=0.05, rep2 0.18/3=0.06 -> 0.055.
    check(jigc["reps"] == 2, f"jigc 2 reps: {jigc['reps']}")
    check(jigc["cost_per_rename"] == 0.055, f"jigc cost/rename 0.055: {jigc['cost_per_rename']}")
    check(jigc["turns_per_rename"] == 3.5, f"jigc turns/rename 3.5: {jigc['turns_per_rename']}")
    # Completeness column — over structured managed refs only.
    check(jigc["completeness"] == 1.0, f"jigc complete: {jigc['completeness']}")
    check(static["completeness"] == 0.8333, f"static 5/6: {static['completeness']}")
    check(plain["completeness"] is not None, "plain completeness computed")
    # Rename-engagement column — the verb-engagement confound as a primary rate.
    check(jigc["rename_engagement"] == 1.0, f"jigc engagement 1.0: {jigc['rename_engagement']}")
    check(static["rename_engagement"] == 0.0, f"static engagement 0.0: {static['rename_engagement']}")
    # Headline ordering the study predicts (within the valid cells).
    check(jigc["cost_per_rename"] < static["cost_per_rename"] < plain["cost_per_rename"],
          f"cost ordering jigc<static<plain: {jigc['cost_per_rename']}/"
          f"{static['cost_per_rename']}/{plain['cost_per_rename']}")
    # Small-N confound — every fixture cell is under MIN_REPS, so flagged underpowered.
    check(jigc["underpowered"] and static["underpowered"], "low-N flagged")

    # Void-tripwire — the seeded control violation (plain rep2) must fire and void.
    check(plain["control_violations"] == 1, f"plain control violation seen: {plain['control_violations']}")
    check(plain["void"] is True, "plain cell voided by the tripwire")
    check(jigc["void"] is False, "the clean jigc cell is not voided")
    check(agg["study_valid"] is False, "study marked void")
    check(any("control-edge violation" in t for t in agg["tripwires"]),
          f"tripwire fired: {agg['tripwires']}")
    # Verdict is withheld when a tripwire fired.
    check(agg["verdict"]["claude-opus-4-8"]["void"] is True, "verdict void on tripwire")

    # The pre-registered win logic itself, on a hand-built CLEAN cells dict (no tripwire).
    clean = {
        "jigc|m": {"arm": "jigc", "model": "m", "void": False,
                   "cost_per_rename": 0.05, "completeness": 1.0},
        "plain|m": {"arm": "plain", "model": "m", "void": False,
                    "cost_per_rename": 0.20, "completeness": 0.6},
        "static|m": {"arm": "static", "model": "m", "void": False,
                     "cost_per_rename": 0.10, "completeness": 0.9},
    }
    vc = verdict(clean)["m"]
    check(vc["win"] is True, f"clean win condition holds: {vc}")
    # No-win when jigc is not cheaper than plain.
    clean["jigc|m"]["cost_per_rename"] = 0.25
    check(verdict(clean)["m"]["win"] is False, "no win when jigc not cheaper than plain")

    # Order-invariance (the merge is reproducible): aggregating the SAME fixtures under a
    # reversed file order must be byte-identical (mean sorts its inputs; cells/tripwires
    # are built in sorted (arm,model) order).
    a_fwd = json.dumps(aggregate(seqs), sort_keys=True)
    a_rev = json.dumps(aggregate(list(reversed(seqs))), sort_keys=True)
    check(a_fwd == a_rev, "aggregate is order-invariant (forward == reversed)")
    # Markdown renders without error and surfaces the void banner.
    md = markdown(agg)
    check("STUDY VOID" in md, "markdown surfaces the void banner")

    if fails:
        print(f"\n{fails} assertion(s) failed")
        return 1
    print(f"eval-sequence-refint.py selftest OK — {len(seqs)} fixtures, "
          f"{len(cells)} cells, tripwire fired, order-invariant")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("matrix", nargs="?", help="dir of <rep>/sequence.json summaries")
    ap.add_argument("--md", help="also write the markdown table here")
    ap.add_argument("--min-reps", type=int, default=MIN_REPS,
                    help=f"flag cells below N reps as underpowered (default {MIN_REPS})")
    ap.add_argument("--selftest", action="store_true",
                    help="run the bundled synthetic-fixture assertions (no API spend)")
    a = ap.parse_args()
    if a.selftest:
        return _selftest()
    if not a.matrix:
        ap.error("need a matrix dir (or --selftest)")
    seqs = load(Path(a.matrix))
    if not seqs:
        print("no sequences found", file=sys.stderr)
        return 1
    agg = aggregate(seqs, a.min_reps)
    json.dump(agg, sys.stdout, indent=2)
    sys.stdout.write("\n")
    if not agg["study_valid"]:
        print("\nSTUDY VOID — tripwire(s) fired:", file=sys.stderr)
        for t in agg["tripwires"]:
            print(f"  {t}", file=sys.stderr)
    if a.md:
        Path(a.md).write_text(markdown(agg))
        print(f"wrote {a.md}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
