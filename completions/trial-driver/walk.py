#!/usr/bin/env python3
"""Drive an operator walk as scripted arms, so an unrun arm is visibly blank.

`cue-card-postmortem.md` §6 step 4, which it calls the cheapest item on its list:

    The operator walk carries T10 and T11 explicitly, as a checklist with recorded
    verbatim output per arm. §5 arm 4 already chartered a committed-identity
    `doc rename` and the walk record does not carry it — **a narrative walk lost a
    chartered probe**. Make each arm a line with a paste command and a captured
    output block, so an unrun arm is visibly blank rather than quietly absent.
    This is a process fix, not a design fix.

So: one script per arm, each driven through `run-session.sh --exec` — the same
copy-in / copy-out / provenance path a blind session rides — and one markdown
record with a block per arm. An arm that did not run renders as `NOT RUN` with its
command beside it, which is the whole point: a narrative walk can lose a probe
silently, a table cannot.

Arm 0 is the positive control and is treated specially: **if it does not fire, no
blind session may be read at all** (protocol §3.4). The runner refuses to report
the rest as meaningful when arm 0 fails, rather than printing a tidy table with a
dead instrument at the top of it.

    python3 completions/trial-driver/walk.py <corpus> <out-dir> [--tag T] [--only N]
"""
from __future__ import annotations

import argparse
import pathlib
import subprocess
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))

from driver import session
from driver.observe import observe

HERE = pathlib.Path(__file__).resolve().parent
REPO = HERE.parents[1]
RUN_SESSION = REPO / "completions" / "trial-harness" / "run-session.sh"
ARMS = HERE / "arms" / "walk"
#: The arm's captured stdout/stderr, written into its out-dir so a later pass can
#: re-render the block instead of pointing at a directory (see `run_arm`).
ARM_OUTPUT = "ARM-OUTPUT.txt"
ARM_STDERR = "ARM-STDERR.txt"


def discover() -> list[pathlib.Path]:
    """Arms are files named `NN-<slug>.sh`; the number is the order and the label."""
    if not ARMS.is_dir():
        return []
    return sorted(ARMS.glob("[0-9][0-9]-*.sh"))


def run_arm(arm: pathlib.Path, corpus: pathlib.Path, out: pathlib.Path,
            tag: str) -> dict:
    """One arm, through the same chain a blind session uses."""
    argv = [str(RUN_SESSION), "--exec", str(arm), str(corpus), str(out), tag]
    got = subprocess.run(argv, cwd=str(REPO), capture_output=True, text=True)
    rc = session.arm_exit_code(out) or 0
    # Persist the arm's own output beside its evidence. Before this, the bar lines
    # lived only in the pass that ran the arm: a record assembled from several
    # `--only` passes (the M50 trial's first walk) kept every exit code and lost
    # every PASS/FAIL line — an "earlier pass" block that could say the arm ran and
    # not what it saw. The out-dir must not pre-exist, so this cannot clobber.
    if out.is_dir():
        (out / ARM_OUTPUT).write_text(got.stdout)
        if got.stderr.strip():
            (out / ARM_STDERR).write_text(got.stderr)
    return {"arm": arm.name, "rc": rc, "driver_rc": got.returncode,
            "stdout": got.stdout, "stderr": got.stderr, "out": out}


def render(results: list[dict], every: list[pathlib.Path], out: pathlib.Path) -> str:
    """One block per arm — including the ones that did not run."""
    ran = {r["arm"]: r for r in results}

    def earlier(arm: pathlib.Path) -> "dict | None":
        """An arm run in a PREVIOUS pass, recovered from the evidence it left.

        Calling it NOT RUN while its out-dir sits beside the record would be a
        plain lie, and the record exists so an unrun arm is visible — not so a run
        one is hidden.
        """
        d = out / arm.stem
        if not (d / "PROVENANCE.txt").is_file():
            return None
        rc = session.arm_exit_code(d) or 0
        stdout = (d / ARM_OUTPUT).read_text() if (d / ARM_OUTPUT).is_file() else ""
        stderr = (d / ARM_STDERR).read_text() if (d / ARM_STDERR).is_file() else ""
        return {"arm": arm.name, "rc": rc, "out": d, "stdout": stdout,
                "stderr": stderr, "earlier": True}
    lines = ["# Operator walk — recorded per arm", "",
             "One block per arm. An arm that did not run says so, with the command that",
             "would run it: a narrative walk loses a chartered probe silently, a table",
             "cannot.", "",
             "| arm | ran | exit | evidence |", "|---|---|---|---|"]
    for arm in every:
        got = ran.get(arm.name) or earlier(arm)
        if got is None:
            lines.append(f"| `{arm.name}` | **NOT RUN** | — | — |")
        else:
            when = "earlier pass" if got.get("earlier") else "this pass"
            lines.append(f"| `{arm.name}` | {when} | {got['rc']} | `{got['out'].name}/` |")
    lines.append("")
    for arm in every:
        got = ran.get(arm.name) or earlier(arm)
        lines += [f"## `{arm.name}`", ""]
        if got is None:
            lines += ["**NOT RUN.** To run it:", "",
                      "```sh",
                      f"python3 completions/trial-driver/walk.py <corpus> <out> "
                      f"--only {arm.name[:2]}",
                      "```", ""]
            continue
        if got.get("earlier"):
            lines += [f"Run in an **earlier pass**, exit **{got['rc']}**. Its evidence is "
                      f"in `{got['out'].name}/`; delete that directory to re-run.", ""]
            if got["stdout"]:
                lines += ["```", got["stdout"].rstrip(), "```", ""]
            else:
                lines += ["*(its output was not persisted — a pass before "
                          f"`{ARM_OUTPUT}` existed; only the exit code survives)*", ""]
        else:
            lines += [f"exit **{got['rc']}**", "", "```"]
            lines += [got["stdout"].rstrip() or "(no output)"]
            lines += ["```", ""]
        if got.get("stderr", "").strip():
            lines += ["stderr:", "", "```", got["stderr"].rstrip(), "```", ""]
        log = got["out"] / ".jigc" / "logs" / "invocations.jsonl"
        if log.is_file():
            o = observe(arm.name, log, None)
            lines += [f"invocation log: {o.records} records · "
                      f"`doc show … --task` **{o.verb}** · §3.3 adjacent {o.adjacent}", ""]
    return "\n".join(lines) + "\n"


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(prog="walk.py", description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("corpus")
    ap.add_argument("out")
    ap.add_argument("--tag", default="jigc-gate:rc11")
    ap.add_argument("--only", help="run just this arm, by its two-digit prefix")
    args = ap.parse_args(argv)

    every = discover()
    if not every:
        print(f"no arms found under {ARMS}", file=sys.stderr)
        return 2

    corpus = pathlib.Path(args.corpus).expanduser().resolve()
    root = pathlib.Path(args.out).expanduser().resolve()
    root.mkdir(parents=True, exist_ok=True)

    chosen = [a for a in every if not args.only or a.name.startswith(args.only)]
    results = []
    for arm in chosen:
        out = root / arm.stem
        if out.exists():
            print(f"skipping {arm.name}: {out} exists (delete it to re-run)")
            continue
        print(f"→ {arm.name}")
        got = run_arm(arm, corpus, out, args.tag)
        results.append(got)
        print(f"  exit {got['rc']}")
        # Arm 0 is the positive control: if it does not fire, nothing else is
        # readable, so stop rather than print a tidy table with a dead instrument
        # at the top of it.
        if arm.name.startswith("00") and got["rc"] != 0:
            print("\nARM 0 FAILED — the positive control did not fire.\n"
                  "No blind session may be read against this rig until it does "
                  "(protocol.md §3.4). Fix the instrument and re-run.", file=sys.stderr)
            (root / "walk-record.md").write_text(render(results, every, root))
            return 1

    record = root / "walk-record.md"
    record.write_text(render(results, every, root))
    print(f"\nrecord: {record}")
    missing = [a.name for a in every if a.name not in {r['arm'] for r in results}]
    if missing:
        print(f"arms not run this pass: {', '.join(missing)} — the record says so")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
