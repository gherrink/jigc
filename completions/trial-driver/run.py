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
import json
import pathlib
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))

from driver import channels
from driver import cascade, gate as gate_mod, plants as plants_mod, session as session_mod
from driver.observe import Observation, halted_awaiting_human, ended_asking, observe

HERE = pathlib.Path(__file__).resolve().parent
EVIDENCE = HERE.parent / "artifacts" / "RC-1.0-gate" / "evidence"

#: `trial-record.md` §3, after its logged correction to B1: (records, VERB, §3.3 adjacent).
RECORD = {
    "harborlight": (26, 3, 2),
    "pinegrove": (102, 4, 9),
    "stonefly": (94, 6, 5),
    "rosewater": (99, 7, 6),
}


def _find(out: pathlib.Path) -> tuple[pathlib.Path, list[pathlib.Path]]:
    """Locate a session's channels inside a `run-session.sh` out-dir.

    Returns every transcript the session produced, **main first**: the main session,
    then one file per subagent. `observe` reads the whole list for the FILESYSTEM
    channel and the first element alone for the seed marker.

    This used to return one path — the largest `.jsonl`, on the reasoning that
    *"the largest is the session itself; sidecars are small"*. That reasoning was
    the defect: a subagent transcript IS that small sidecar, so a worker that
    delegated its orientation moved its reads into a file the reader never opened
    and the cell scored clean for the wrong reason. B2's two subagent transcripts
    (58 KB and 105 KB against a 586 KB main) are exactly the shape that hid.
    """
    log = out / ".jigc" / "logs" / "invocations.jsonl"
    root = out / ".session-transcript"
    if not root.is_dir():
        return log, []
    every = sorted(root.rglob("*.jsonl"))
    # A subagent file lives under a `subagents/` directory and is named by its
    # agentId, never by the session id — which is also why `session._find_transcript`
    # cannot find one by globbing the session id.
    delegated = [p for p in every if "subagents" in p.parts]
    own = [p for p in every if p not in delegated]
    main = max(own, key=lambda p: p.stat().st_size) if own else None
    return log, ([main] if main else []) + delegated


def _row(o: Observation) -> str:
    verdict = cascade.grade(o)
    mark = "VOID " if verdict.void else "     "
    return (f"{o.session:16} {o.records:5} {o.authoring_writes:5} {o.verb:5} "
            f"{o.adjacent:4} {o.filesystem:3}  {mark}{verdict.outcome}")


def _header() -> None:
    print(f"{'session':16} {'recs':>5} {'wrote':>5} {'VERB':>5} {'adj':>4} {'fs':>3}  outcome")
    print("-" * 84)


def _provenance_verdict(out: pathlib.Path,
                        record: "pathlib.Path | None") -> tuple[bool, str]:
    """Whose evidence is this, and is it this round's? (I-2)

    `run-session.sh` refuses to write into a pre-existing out-dir, which is right —
    but the reader then scored whatever was already there **without a word**, and
    `~/out/<name>` is shared across trials, so a rerun under a reused name silently
    graded the previous trial's sessions. Refusing here rather than warning, on the
    same reasoning as `interact.screen()`: a warning at the bottom of a table is a
    thing a reader compensates for by hand, every time, until once they do not.

    The comparison key is `jigc-sha`. The image *tag* is mutable — the whole reason
    `gate.py` keys on `image_id` — and `image-id` is the one strong key the evidence
    did not carry until this trial started recording it, so a dir written before
    then is checked on the sha and said to be.
    """
    got = session_mod.provenance(out)
    if not got:
        return False, ("no PROVENANCE.txt — this directory was not produced by "
                       "run-session.sh, so nothing says which binary or which "
                       "trial it belongs to")
    stamped = got.get("jigc-sha", "")
    if record is None:
        return True, (f"ungated: {got.get('image', '?')} / {stamped[:12] or '?'} "
                      f"(pass --gate <record.json> to check it)")
    if not record.is_file():
        return False, f"no gate record at {record}"
    want = json.loads(record.read_text()).get("identity", {})
    want_sha = want.get("jigc_sha", "")
    if stamped and want_sha and stamped != want_sha:
        return False, (f"this evidence is from jigc {stamped[:12]}, the round's "
                       f"record covers {want_sha[:12]} — a different binary, so "
                       f"scoring it here would attribute one trial's sessions to "
                       f"another")
    if got.get("image-id") and want.get("image_id") \
            and got["image-id"] != want["image_id"]:
        return False, (f"same sha, different image ({got['image-id'][:19]}… vs "
                       f"{want['image_id'][:19]}…) — a rebuilt tag is a different "
                       f"image and the record no longer covers it")
    return True, f"{got.get('image', '?')} / jigc {stamped[:12] or '?'}"


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
        record = pathlib.Path(args.gate).expanduser().resolve() if args.gate else None
        covered, whose = _provenance_verdict(out, record)
        if not covered:
            print(f"{out.name:16} REFUSED — {whose}", file=sys.stderr)
            rc = 1
            continue
        print(f"  provenance: {whose}")
        log, transcripts = _find(out)
        stream = out / "stream.jsonl"
        halted, why = halted_awaiting_human(stream)
        asked, question = ended_asking(stream)
        # The CLI's own exit code, not run-session.sh's — it exits 0 on a failed arm
        # by design, so without this a dead session scores as a product result.
        arm_rc = session_mod.arm_exit_code(out)
        o = observe(out.name, log, transcripts, halted_for_human=halted,
                    rc_failed=bool(arm_rc),
                    session_start=session_mod.session_start(out))
        print(_row(o))
        if halted:
            # The turn ended at exit 0, `subtype: success`, `is_error: false`.
            # Only the denials and the result text say it stopped mid-task, which
            # is why the process exit code cannot be the completion signal.
            print(f"  HALTED awaiting the operator — {why}")
        if asked and not halted:
            # The other halt shape: nothing was DENIED, the product blocked the
            # arc and the worker asked. `permission_denials` is empty, so the
            # exact detector cannot see it. Heuristic, and said so here.
            print(f"  ENDED ASKING the operator (heuristic — the turn's last "
                  f"message is a question, and nothing was denied):")
            print(f"    …{question}")
        if o.pre_session_records:
            # The invocation log lives in the corpus, so a plant or the adoption
            # arm writes into the channel the session is scored on. Printed rather
            # than silently corrected: the size of the error is the useful part.
            print(f"  note: {o.pre_session_records} record(s) predate this session "
                  f"(plant/adoption) and are NOT scored — they would have added "
                  f"{o.pre_session_verb} VERB and {o.pre_session_adjacent} adjacent")
        if o.log_missing:
            # The cascade has already voided this row and the table says so; this
            # line adds the path, which the outcome name cannot carry. It must not
            # restate the verdict — that was the duplicate-scoring defect.
            print(f"  ! no invocation log at {log}", file=sys.stderr)
            rc = 1
        if o.transcript_missing:
            print("  ! no transcript — the FILESYSTEM channel is unmeasured "
                  "for this session", file=sys.stderr)
            rc = 1
        for r in o.filesystem_reads:
            kind = "DOC " if r.is_document else "wkbn"
            who = f" ({r.agent})" if r.agent else ""
            print(f"  fs? {kind} [{r.tool}]{who} {r.detail[:104]}")
        if o.filesystem_reads:
            docs = sum(1 for r in o.filesystem_reads if r.is_document)
            print(f"  fs split: {docs} managed-document read(s), "
                  f"{len(o.filesystem_reads) - docs} workbench-bookkeeping read(s) "
                  f"— only the first is the adapter bypass the invariant is about")
            print("  (heuristic: check each against the corpus — an unregistered "
                  "foreign doc is not a managed one)")
        # The write channel. Reads got a fence after a debrief found what the reader
        # missed; writes got the same lesson one trial later (RC-rc14 F-9/F-10).
        for w in o.commit_writes:
            who = f" ({w.agent})" if w.agent else ""
            print(f"  git! {w.verb:<11}[{w.tool}]{who} {w.detail[:96]}")
        for line in o.history_surgery:
            print(f"  HISTORY REWRITTEN — {line}")
        if o.commit_writes or o.history_surgery:
            print("  ^ a commit produced or rewritten outside jigc. The CLI owns the commit "
                  "boundary; git is a blessed HUMAN channel, so this is evidence for review, "
                  "not a verdict — check whether the commit carried a managed doc.")
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


def do_carry(args: argparse.Namespace) -> int:
    """Turn a finished run's out-dir into the corpus for the next step.

    The chain is instantiate -> gate -> adopt -> plant -> session, and each step's
    OUT is the next step's corpus. But `run-session.sh` writes its own evidence
    INTO that directory (`PROVENANCE.txt`, `stream.jsonl`, `stderr.txt`,
    `.session-transcript/`), so handing it straight on plants the rig's droppings
    in the corpus a worker will read.

    `session.carry_forward` has always done this for `seed`'s turn-to-turn
    hand-off; nothing exposed it to the chain, so it was done by hand — three
    times, in one afternoon, which is the signal.
    """
    out = pathlib.Path(args.out).expanduser().resolve()
    dest = pathlib.Path(args.dest).expanduser().resolve()
    if not out.is_dir():
        print(f"refusing: no such run directory: {out}", file=sys.stderr)
        return 2
    if not (out / ".git").is_dir():
        print(f"refusing: {out} has no .git — that is not a corpus, and copying "
              f"it would hand the next step a tree with no history",
              file=sys.stderr)
        return 2
    session_mod.carry_forward(out, dest)
    left = sorted(p.name for p in out.iterdir()
                  if p.name in session_mod._EVIDENCE_NAMES)
    print(f"carried {out} -> {dest}")
    print(f"  left behind: {', '.join(left) if left else '(no rig evidence found)'}")
    return 0


def do_seed(args: argparse.Namespace) -> int:
    """Drive a turns file as one conversation and freeze it."""
    turns = [t for t in pathlib.Path(args.turns).read_text().splitlines() if t.strip()]
    fixture = session_mod.seed(
        pathlib.Path(args.corpus).expanduser().resolve(), turns,
        pathlib.Path(args.frozen).expanduser().resolve(),
        tag=args.tag, force=args.force,
        gate_record=pathlib.Path(args.gate).expanduser().resolve() if args.gate else None)
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
    _, arm_rc = session_mod.fork(
        fixture, pathlib.Path(args.prompt).read_text(), out,
        corpus=pathlib.Path(args.corpus).expanduser().resolve(),
        tag=args.tag, strict=args.strict,
        gate_record=pathlib.Path(args.gate).expanduser().resolve() if args.gate else None)
    stream = out / "stream.jsonl"
    new = session_mod.forked_id(stream)
    log = out / ".jigc" / "logs" / "invocations.jsonl"
    transcripts = session_mod._find_transcript(out, new) if new else []
    halted, why = halted_awaiting_human(stream)
    o = observe(out.name, log, transcripts, seed_expected=True,
                seed_marker=fixture.seed_marker, halted_for_human=halted,
                rc_failed=arm_rc != 0)
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
    p_obs.add_argument("--gate",
                       help="the round's isolation record; evidence from another "
                            "binary is refused rather than silently scored")
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

    p_carry = sub.add_parser(
        "carry", help="make a finished run's out-dir the next step's corpus")
    p_carry.add_argument("out")
    p_carry.add_argument("dest")
    p_carry.set_defaults(fn=do_carry)

    p_seed = sub.add_parser("seed", help="drive a turns file and freeze the conversation")
    p_seed.add_argument("corpus")
    p_seed.add_argument("turns")
    p_seed.add_argument("frozen")
    p_seed.add_argument("--tag", default="jigc-gate:rc11")
    p_seed.add_argument("--force", action="store_true")
    p_seed.add_argument("--gate", help="an isolation record this round must be covered by")
    p_seed.set_defaults(fn=do_seed)

    p_fork = sub.add_parser("fork", help="resume a frozen conversation for one turn")
    p_fork.add_argument("frozen")
    p_fork.add_argument("corpus")
    p_fork.add_argument("prompt")
    p_fork.add_argument("out")
    p_fork.add_argument("--tag", default="jigc-gate:rc11")
    p_fork.add_argument("--strict", action="store_true",
                        help="--strict-permissions: the adopter's real condition")
    p_fork.add_argument("--gate", help="an isolation record this round must be covered by")
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
