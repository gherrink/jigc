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
from driver import cascade, gate as gate_mod, plants as plants_mod, session as session_mod
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


def do_gate(args: argparse.Namespace) -> int:
    """Read the isolation record and refuse a round it does not cover."""
    ident = gate_mod.gate(pathlib.Path(args.record).expanduser().resolve(), args.tag)
    print(f"gated: {ident.tag}")
    print(f"  image  {ident.image_id}")
    print(f"  jigc   {ident.jigc_version}  ({ident.jigc_sha[:12]})")
    print(f"  cli    {ident.cli_version}")
    return 0


def do_record_gate(args: argparse.Namespace) -> int:
    """Write an isolation record for the image as it is right now.

    Deliberately requires the checks to be passed in: this writes down what a
    verifier found, and inventing a `{"isolation": true}` here would be the exact
    shape the gate exists to refuse — a record that certifies nothing.
    """
    checks = {}
    for pair in args.check or ():
        name, _, value = pair.partition("=")
        checks[name] = value.lower() in ("1", "true", "yes", "pass")
    if not checks:
        print("refusing: --check NAME=pass is required — a gate that passes on "
              "nothing is not a gate", file=sys.stderr)
        return 2
    ident = gate_mod.identity(args.tag)
    out = pathlib.Path(args.record).expanduser().resolve()
    gate_mod.write_record(out, ident, checks=checks, note=args.note or "")
    print(f"wrote {out} for image {ident.image_id[:19]}…")
    return 0


def do_seed(args: argparse.Namespace) -> int:
    """Drive a turns file as one conversation and freeze it."""
    turns = [t for t in pathlib.Path(args.turns).read_text().splitlines() if t.strip()]
    fixture = session_mod.seed(
        pathlib.Path(args.corpus).expanduser().resolve(), turns,
        pathlib.Path(args.frozen).expanduser().resolve(),
        tag=args.tag, force=args.force)
    print(f"frozen at {fixture.root}")
    print(f"  session {fixture.session_id}")
    print(f"  sha     {fixture.manifest['session_sha']}")
    print(f"  turns   {fixture.manifest['turns']}")
    return 0


def do_fork(args: argparse.Namespace) -> int:
    """Resume a frozen conversation for one measured turn."""
    fixture = session_mod.Frozen(pathlib.Path(args.frozen).expanduser().resolve())
    problems = fixture.verify()
    if problems:
        print("refusing to fork an unverified fixture:\n  " + "\n  ".join(problems),
              file=sys.stderr)
        return 2
    out = pathlib.Path(args.out).expanduser().resolve()
    session_mod.fork(fixture, pathlib.Path(args.prompt).read_text(), out,
                     corpus=pathlib.Path(args.corpus).expanduser().resolve(),
                     tag=args.tag, strict=args.strict)
    stream = out / "stream.jsonl"
    new = session_mod.forked_id(stream)
    log = out / ".jigc" / "logs" / "invocations.jsonl"
    transcript = session_mod._find_transcript(out, new) if new else None
    halted, why = halted_awaiting_human(stream)
    o = observe(out.name, log, transcript, seed_expected=True,
                seed_marker=fixture.seed_marker, halted_for_human=halted)
    _header()
    print(_row(o))
    if not o.seed_inherited:
        print("  ! the fork did NOT inherit the seed — this run is void, not a result",
              file=sys.stderr)
    if halted:
        print(f"  HALTED awaiting the operator — {why}")
    return 0


def do_plant(args: argparse.Namespace) -> int:
    """Wait for a plant's state in a live session, then fire it once."""
    plant = plants_mod.Plant(name=pathlib.Path(args.script).stem, when=args.when,
                             script=pathlib.Path(args.script).expanduser().resolve())
    got = plants_mod.watch_and_fire(pathlib.Path(args.cid_file).expanduser().resolve(),
                                    plant, poll_s=args.poll, timeout_s=args.timeout)
    print(f"plant  : {got.plant}")
    print(f"fired  : {got.fired}")
    print(f"reason : {got.reason}")
    print(f"waited : {got.waited_s:.0f}s")
    if got.stdout.strip():
        print("--- stdout ---\n" + got.stdout.rstrip())
    if got.stderr.strip():
        print("--- stderr ---\n" + got.stderr.rstrip())
    return 0 if got.fired else 1


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

    p_gate = sub.add_parser("gate", help="refuse a round the isolation record misses")
    p_gate.add_argument("record")
    p_gate.add_argument("--tag", default="jigc-gate:rc11")
    p_gate.set_defaults(fn=do_gate)

    p_rec = sub.add_parser("record-gate", help="write an isolation record for an image")
    p_rec.add_argument("record")
    p_rec.add_argument("--tag", default="jigc-gate:rc11")
    p_rec.add_argument("--check", action="append", metavar="NAME=pass",
                       help="a check the verifier ran and its result; repeatable")
    p_rec.add_argument("--note", default="")
    p_rec.set_defaults(fn=do_record_gate)

    p_seed = sub.add_parser("seed", help="drive a turns file and freeze the conversation")
    p_seed.add_argument("corpus")
    p_seed.add_argument("turns")
    p_seed.add_argument("frozen")
    p_seed.add_argument("--tag", default="jigc-gate:rc11")
    p_seed.add_argument("--force", action="store_true")
    p_seed.set_defaults(fn=do_seed)

    p_fork = sub.add_parser("fork", help="resume a frozen conversation for one turn")
    p_fork.add_argument("frozen")
    p_fork.add_argument("corpus")
    p_fork.add_argument("prompt")
    p_fork.add_argument("out")
    p_fork.add_argument("--tag", default="jigc-gate:rc11")
    p_fork.add_argument("--strict", action="store_true",
                        help="--strict-permissions: the adopter's real condition")
    p_fork.set_defaults(fn=do_fork)

    p_plant = sub.add_parser("plant", help="wait for a plant's state, then fire it")
    p_plant.add_argument("cid_file")
    p_plant.add_argument("script")
    p_plant.add_argument("--when", required=True,
                         help="shell predicate run inside the container against /work")
    p_plant.add_argument("--poll", type=float, default=5.0)
    p_plant.add_argument("--timeout", type=float, default=3600.0)
    p_plant.set_defaults(fn=do_plant)

    p_test = sub.add_parser("test", help="run every suite")
    p_test.set_defaults(fn=do_test)

    args = parser.parse_args(argv)
    return args.fn(args)


if __name__ == "__main__":
    raise SystemExit(main())
