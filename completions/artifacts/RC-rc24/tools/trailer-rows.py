#!/usr/bin/env python3
"""Print the rows the trailer check is read from. It scores nothing.

    python3 trailer-rows.py <out-dir> [--since YYYY-MM-DDTHH:MM:SSZ]

`<out-dir>` is a `run-session.sh` out-dir: a corpus with `.git`, and
`.jigc/logs/invocations.jsonl` inside it. `--since` is the instant the agent
session began; it defaults to `session-start` in `<out-dir>/PROVENANCE.txt`. For a
seeded conversation pass turn 1's `session-start` and point at the LAST turn's
out-dir, whose log and history are cumulative.

What it prints, and nothing more:

  * every committing-door record in the log at or after `--since` (exit 0);
  * every commit object created at or after `--since` — reachable from any ref OR
    from the reflog, so a commit a later amend superseded is still listed — with
    its `Co-Authored-By` trailer lines as git's own trailer parser reads them;
  * the join between the two, by time: a commit is attributed to a door when its
    committer time falls inside that record's run.

The join is an aid, not a verdict. A raw `git commit` that lands inside a door's
window is attributed to the door, so every `jigc` row is checked against the
`git!` lines `run.py observe` prints for the same session before it is believed.
"""
from __future__ import annotations

import argparse
import datetime as dt
import json
import math
import pathlib
import subprocess
import sys

EXACT = "Co-Authored-By: Claude <noreply@anthropic.com>"
ADDRESS = "<noreply@anthropic.com>"

#: `cli::invocation_log::COMMITTING_DOORS`, as verb prefixes, plus `setup`'s install
#: commit — the axis design/assistant-adapter.md -> The co-author trailer names.
DOORS = (
    ("task", "finalize"), ("task", "discard"),
    ("milestone", "finalize"), ("milestone", "create"), ("milestone", "add-task"),
    ("milestone", "add-from-spec"), ("milestone", "discard"),
    ("rename",), ("migrate-corpus",), ("setup",),
)
NOT_A_COMMIT = ("--dry-run", "--help", "-h", "--preview")


def epoch(stamp: str) -> int:
    return int(dt.datetime.strptime(stamp, "%Y-%m-%dT%H:%M:%SZ")
               .replace(tzinfo=dt.timezone.utc).timestamp())


def verb(argv: list[str]) -> list[str]:
    """argv with leading global options dropped (`--format json doc show …`)."""
    i = 0
    while i < len(argv) and argv[i].startswith("-"):
        i += 2 if (argv[i] == "--format") else 1
    return argv[i:]


def git(out: pathlib.Path, *args: str) -> str:
    got = subprocess.run(["git", "-C", str(out), *args], capture_output=True, text=True)
    if got.returncode != 0:
        raise SystemExit(f"git {' '.join(args)} failed: {got.stderr.strip()[:300]}")
    return got.stdout


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("out")
    ap.add_argument("--since")
    args = ap.parse_args()
    out = pathlib.Path(args.out).expanduser().resolve()
    log = out / ".jigc" / "logs" / "invocations.jsonl"
    if not (out / ".git").exists() or not log.is_file():
        print(f"refusing: {out.name} has no .git or no invocation log", file=sys.stderr)
        return 2

    since = args.since
    if not since:
        prov = out / "PROVENANCE.txt"
        for line in prov.read_text().splitlines() if prov.is_file() else ():
            if line.startswith("session-start"):
                since = line.split(None, 1)[1].strip()
    if not since:
        print("refusing: no --since and no session-start in PROVENANCE.txt — without "
              "it the adoption's own commits would be checked as the agent's",
              file=sys.stderr)
        return 2
    t0 = epoch(since)

    doors = []
    for raw in log.read_text().splitlines():
        if not raw.strip():
            continue
        r = json.loads(raw)
        v = verb(list(r["argv"]))
        if r["exit_code"] != 0 or any(a in NOT_A_COMMIT for a in v):
            continue
        if not any(tuple(v[:len(d)]) == d for d in DOORS):
            continue
        end = epoch(r["timestamp"])
        if end < t0:
            continue
        span = math.ceil(int(r.get("duration_ms", 0)) / 1000) + 1
        doors.append({"end": end, "start": end - span, "stamp": r["timestamp"],
                      "argv": " ".join(v)[:70], "hits": 0})

    head_set = set(git(out, "rev-list", "HEAD").split())
    found = []
    for sha in git(out, "rev-list", "--all", "--reflog").split():
        fields = git(out, "show", "-s",
                     "--format=%ct%x00%s%x00%(trailers:key=Co-Authored-By,separator=%x01)",
                     sha).rstrip("\n").split("\x00")
        if int(fields[0]) < t0:
            continue
        # Depth breaks a same-second tie in history order, so two doors run back to
        # back are each given their own commit rather than both given the first.
        depth = int(git(out, "rev-list", "--count", sha).strip())
        found.append((int(fields[0]), depth, sha, fields[1], fields[2]))

    rows = []
    for ct, _, sha, subject, trailers in sorted(found):
        lines = [ln.strip() for ln in trailers.split("\x01") if ln.strip()]
        ours = [ln for ln in lines if ADDRESS in ln.lower()]
        fits = [d for d in doors if d["start"] <= ct <= d["end"] + 1]
        # Log order, an unclaimed door first: a boundary that makes several commits
        # still collects all of them, because it is then the only door that fits.
        door = next((d for d in fits if d["hits"] == 0), fits[0] if fits else None)
        if door is None:
            verdict = "not-jigc"
        else:
            door["hits"] += 1
            if len(ours) > 1:
                verdict = "DUPLICATE"
            elif not ours:
                verdict = "NONE"
            elif ours[0] == EXACT:
                verdict = "exact"
            else:
                verdict = "variant"
        rows.append((ct, sha[:8], verdict, "HEAD" if sha in head_set else "reflog",
                     door["argv"] if door else "-", subject[:48], " | ".join(lines)))

    print(f"session since {since} · {len(doors)} committing-door record(s) at exit 0 · "
          f"{len(rows)} commit(s) created since")
    print(f"{'commit':8}  {'row':9}  {'at':6}  door  ·  subject  ·  Co-Authored-By lines")
    for ct, sha, verdict, where, door, subject, lines in sorted(rows):
        print(f"{sha}  {verdict:9}  {where:6}  {door}  ·  {subject}  ·  {lines or '(none)'}")
    for d in doors:
        if d["hits"] == 0:
            print(f"door with NO commit matched: {d['stamp']}  {d['argv']}")
    tally = {}
    for r in rows:
        tally[r[2]] = tally.get(r[2], 0) + 1
    print("tally: " + ", ".join(f"{k} {v}" for k, v in sorted(tally.items())))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
