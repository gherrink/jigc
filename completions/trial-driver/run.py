#!/usr/bin/env python3
"""The trial driver's one entry point.

    python3 completions/trial-driver/run.py observe <session-out-dir>...
    python3 completions/trial-driver/run.py observe --archive
    python3 completions/trial-driver/run.py test

`observe` reads the primary channels off a finished session — a `run-session.sh`
out-dir, which carries `.jigc/logs/invocations.jsonl` and
`.session-transcript/**/<uuid>.jsonl`. It replaces counting by eye, which is how
the 1.0.0-gate record produced one wrong headline figure and had to correct it
from the logs a day later.

`--archive` runs it over the committed 1.0.0-gate evidence and prints the table
beside the record's own hand-derived figures. That is the reader's own positive
control: if it cannot reproduce numbers this repo already settled by hand, the
reader is wrong and nothing it says about a fresh session should be believed.
"""
from __future__ import annotations

import argparse
import pathlib
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))

from driver import channels
from driver import cascade
from driver.observe import Observation, halted_awaiting_human, observe

HERE = pathlib.Path(__file__).resolve().parent
EVIDENCE = HERE.parent / "artifacts" / "RC-1.0-gate" / "evidence"

#: `trial-record.md` §3, after its logged correction to B1: (records, VERB, §3.3 adjacent).
RECORD = {
    "harborlight": (26, 3, 2),
    "pinegrove": (102, 4, 9),
    "stonefly": (94, 6, 5),
    "rosewater": (99, 7, 6),
}


def _find(out: pathlib.Path) -> tuple[pathlib.Path, pathlib.Path | None]:
    """Locate a session's two channels inside a `run-session.sh` out-dir."""
    log = out / ".jigc" / "logs" / "invocations.jsonl"
    transcripts = sorted((out / ".session-transcript").rglob("*.jsonl")) \
        if (out / ".session-transcript").is_dir() else []
    # The largest is the session itself; sidecars are small.
    transcript = max(transcripts, key=lambda p: p.stat().st_size) if transcripts else None
    return log, transcript


def _row(o: Observation) -> str:
    verdict = cascade.grade(o)
    mark = "VOID " if verdict.void else "     "
    return (f"{o.session:16} {o.records:5} {o.authoring_writes:5} {o.verb:5} "
            f"{o.adjacent:4} {o.filesystem:3}  {mark}{verdict.outcome}")


def _header() -> None:
    print(f"{'session':16} {'recs':>5} {'wrote':>5} {'VERB':>5} {'adj':>4} {'fs':>3}  outcome")
    print("-" * 84)


def do_observe(args: argparse.Namespace) -> int:
    if args.archive:
        if not EVIDENCE.is_dir():
            print(f"refusing: no archived evidence at {EVIDENCE}", file=sys.stderr)
            return 2
        _header()
        drift = []
        for name, (recs, verb, adj) in RECORD.items():
            o = observe(name, EVIDENCE / f"{name}-invocations.jsonl",
                        EVIDENCE / f"{name}-transcript.jsonl")
            print(_row(o))
            if (o.records, o.verb, o.adjacent) != (recs, verb, adj):
                drift.append(f"{name}: reader {(o.records, o.verb, o.adjacent)} "
                             f"vs record {(recs, verb, adj)}")
        print()
        if drift:
            print("READER DISAGREES WITH THE RECORD — the reader is the suspect:",
                  file=sys.stderr)
            for d in drift:
                print(f"  {d}", file=sys.stderr)
            return 1
        print("reproduces the 1.0.0-gate record's channel table exactly.")
        return 0

    if not args.out:
        print("usage: run.py observe <session-out-dir>... | --archive", file=sys.stderr)
        return 2

    _header()
    rc = 0
    for raw in args.out:
        out = pathlib.Path(raw).expanduser().resolve()
        log, transcript = _find(out)
        stream = out / "stream.jsonl"
        halted, why = halted_awaiting_human(stream)
        o = observe(out.name, log, transcript, halted_for_human=halted)
        print(_row(o))
        if halted:
            # The turn ended at exit 0, `subtype: success`, `is_error: false`.
            # Only the denials and the result text say it stopped mid-task, which
            # is why the process exit code cannot be the completion signal.
            print(f"  HALTED awaiting the operator — {why}")
        if o.log_missing:
            print(f"  ! no invocation log at {log} — outcome is "
                  f"{channels.UNMEASURED}, not {channels.NEITHER}", file=sys.stderr)
            rc = 1
        if o.transcript_missing:
            print("  ! no transcript — the FILESYSTEM channel is unmeasured "
                  "for this session", file=sys.stderr)
            rc = 1
        for r in o.filesystem_reads:
            print(f"  fs? [{r.tool}] {r.detail[:110]}")
        if o.filesystem_reads:
            print("  (heuristic: check each against the corpus — an unregistered "
                  "foreign doc is not a managed one)")
        if o.adjacent_counter_gap:
            print(f"  note: run-session.sh's own counter would report "
                  f"{o.shipped_adjacent} adjacent, not {o.adjacent} (§3.3)")
    return rc


def do_test(_: argparse.Namespace) -> int:
    """Every suite, each in its own interpreter.

    Separate processes because the suites patch module-level state; a shared
    interpreter would let one suite's fixtures decide another's result.
    """
    import subprocess
    rc = 0
    for suite in sorted(HERE.glob("test_*.py")):
        print(f"=== {suite.name} ===")
        rc |= subprocess.run([sys.executable, str(suite)]).returncode
    return rc


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(prog="run.py", description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = parser.add_subparsers(dest="verb", required=True)

    p_obs = sub.add_parser("observe", help="read a finished session's channels")
    p_obs.add_argument("out", nargs="*", help="a run-session.sh out-dir")
    p_obs.add_argument("--archive", action="store_true",
                       help="score the committed 1.0.0-gate evidence instead")
    p_obs.set_defaults(fn=do_observe)

    p_test = sub.add_parser("test", help="run the reader's own suite")
    p_test.set_defaults(fn=do_test)

    args = parser.parse_args(argv)
    return args.fn(args)


if __name__ == "__main__":
    raise SystemExit(main())
