#!/usr/bin/env python3
"""List every shell command in a session's transcripts that runs a git history verb.

    python3 raw-git-acts.py <out-dir>

`<out-dir>` is a `run-session.sh` out-dir; every `*.jsonl` under its
`.session-transcript/` is read, the main session and each subagent alike.

Why this exists beside `run.py observe`'s `git!` lines: that channel takes the first
non-flag word after `git` as the subcommand, so `git -C /work commit --amend` and
`git -c user.name=x commit` are read as the subcommands `/work` and `user.name=x`
and are not reported (driven 2026-10-03, five shapes, two caught). jigc's own
printed routes lead with `git -C <absolute path>`, so that is the shape a worker on
this binary is most likely to type. This lists the command text and scores nothing;
a `jigc …` command is never listed, whatever its arguments say.

A transcript resumed across turns is cumulative: the last turn's file repeats the
earlier turns' commands.
"""
from __future__ import annotations

import json
import pathlib
import re
import sys

VERBS = "commit|reset|revert|rebase|cherry-pick|am|merge|filter-branch|update-ref"
#: `git`, then anything that is not a statement separator, then a history verb as a
#: whole word. Deliberately loose: an over-match is read and dismissed by a human, an
#: under-match is a rewrite nobody saw.
PATTERN = re.compile(r"(?<![\w./-])git\b[^|;&\n]*?\s(" + VERBS + r")(?=\s|$)")


def commands(path: pathlib.Path):
    for raw in path.read_text(errors="replace").splitlines():
        raw = raw.strip()
        if not raw.startswith("{"):
            continue
        try:
            event = json.loads(raw)
        except json.JSONDecodeError:
            continue
        message = event.get("message")
        if not isinstance(message, dict) or not isinstance(message.get("content"), list):
            continue
        for block in message["content"]:
            if (isinstance(block, dict) and block.get("type") == "tool_use"
                    and block.get("name") == "Bash"):
                yield str((block.get("input") or {}).get("command") or "")


def main() -> int:
    if len(sys.argv) != 2:
        print(__doc__, file=sys.stderr)
        return 2
    root = pathlib.Path(sys.argv[1]).expanduser().resolve() / ".session-transcript"
    if not root.is_dir():
        print("refusing: no .session-transcript/ — the write channel is unread, "
              "not empty", file=sys.stderr)
        return 2
    files = sorted(root.rglob("*.jsonl"))
    hits = 0
    for path in files:
        who = f"subagent {path.stem}" if "subagents" in path.parts else "main"
        for n, command in enumerate(commands(path), 1):
            for statement in re.split(r"(?:\|\||&&|;|\n)", command):
                if statement.strip().startswith("jigc "):
                    continue
                got = PATTERN.search(statement)
                if got:
                    hits += 1
                    print(f"{who:28} bash#{n:<4} {got.group(1):11} {statement.strip()[:160]}")
    print(f"{hits} raw git history act(s) across {len(files)} transcript(s)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
