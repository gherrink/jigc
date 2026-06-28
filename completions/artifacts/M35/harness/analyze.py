#!/usr/bin/env python3
"""Behavioral analyzer for one agent-in-the-loop workflow-eval run.

The pilot (`completions/artifacts/differentiator-pilot-study1/`) proved the gap this
harness closes: **"the CLI composes a workflow correctly" ≠ "a real agent selects,
engages, and completes it correctly."** The pilot was the first behavioral test of the
workflow layer and it failed all three (mis-selection, 0/16 bare-task engagement,
fumbled completion). This analyzer is the reusable core of that test.

It reads ONE run's transcript and reports the behavioral signals plus the
outcome:

- **engaged** — did the agent invoke `jigc` at all? (`jigc` ran in 0/32 bare-task
  pilot runs — the agent edited files directly.)
- **rename_engaged** — did the agent use the `jigc rename` **verb**, or rename by
  hand (`git mv`/`sd`/a file edit)? The M35 verb-engagement confound, measured as a
  PRIMARY quantity: availability ≠ induced-usage — a hand-edit re-incurs the
  block→recovery cost, so the per-rename cost win evaporates. Distinct from
  `engaged`: an agent that runs `jigc validate` but hand-edits the rename scores
  `rename_engaged: False` with `engaged: True`.
- **selected_workflow** — which workflow did `jigc start --workflow X` pick? (the
  weaker model picked `quick-fix` 6/6 when nudged — the wrong safety choice.)
- **finalized** — did the agent reach `jigc task finalize`? (completion.)
- **outcome** — clean / drift / blocked, from an optional `jigc validate` capture
  (the Phase-2 floor is the natural oracle) or an old-symbol grep over doc files.

Behavioral signals need the **tool-call stream** (`claude -p --output-format
stream-json --verbose`); the final-result `--output-format json` carries no tool
trace, so this analyzer reports `format: result-only` and `engaged: null` for it
(still extracting turns/cost). `run-eval.sh` captures stream-json for this reason.

Usage:
    analyze.py TRANSCRIPT [--validate FILE] [--old-symbol NAME --doc DOC ...]
    analyze.py --selftest        # run the fixture assertions (no API spend)

Pure-stdlib (json, re, sys, argparse) so it runs anywhere `python3` does.
"""
from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

# A `jigc` invocation inside a (possibly compound) shell command. Word-bounded so
# `jigcfoo` / a path like `/x/jigc-notes` never matches; captures the rest of the
# command up to a shell separator so the verb + flags are recoverable.
_JIGC = re.compile(r"(?:^|[\s;&|(])jigc\s+([^\n;&|]+)")
_WORKFLOW = re.compile(r"\bstart\b[^\n;&|]*--workflow\s+(\S+)")
_FINALIZE = re.compile(r"\btask\s+finalize\b")


def _iter_events(text: str):
    """Yield event dicts from a transcript, tolerating all three shapes the Claude
    CLI emits: a JSON array of events, newline-delimited JSON (stream-json), or a
    single final-result object (`--output-format json`)."""
    stripped = text.strip()
    if not stripped:
        return
    # Whole-document JSON: either a list of events or one result object.
    try:
        whole = json.loads(stripped)
    except json.JSONDecodeError:
        whole = None
    if isinstance(whole, list):
        yield from (e for e in whole if isinstance(e, dict))
        return
    if isinstance(whole, dict):
        yield whole
        return
    # Newline-delimited JSON (stream-json): one event per line.
    for line in stripped.splitlines():
        line = line.strip()
        if not line:
            continue
        try:
            obj = json.loads(line)
        except json.JSONDecodeError:
            continue
        if isinstance(obj, dict):
            yield obj


def _tool_commands(event: dict):
    """Every shell command string in an assistant event's `tool_use` blocks. Reads
    `input.command` (the Bash tool) and falls back to stringifying the input, so a
    differently-named shell tool still surfaces its `jigc` calls.

    Tolerates the content living under `event["message"]["content"]` (the documented
    stream-json wrapping of the raw API message) **or** a top-level `event["content"]`,
    so a CLI-version shape shift doesn't silently zero the behavioral signal."""
    content = None
    msg = event.get("message")
    if isinstance(msg, dict) and isinstance(msg.get("content"), list):
        content = msg["content"]
    elif isinstance(event.get("content"), list):
        content = event["content"]
    if content is None:
        return
    for block in content:
        if not isinstance(block, dict) or block.get("type") != "tool_use":
            continue
        inp = block.get("input")
        if isinstance(inp, dict) and isinstance(inp.get("command"), str):
            yield inp["command"]
        elif inp is not None:
            yield json.dumps(inp)


def analyze_transcript(text: str) -> dict:
    """Extract the behavioral record from one transcript's text."""
    has_tool_stream = False
    result_text = None
    num_turns = None
    cost_usd = None
    jigc_commands: list[str] = []

    for event in _iter_events(text):
        etype = event.get("type")
        if etype == "assistant":
            for cmd in _tool_commands(event):
                has_tool_stream = True
                for m in _JIGC.finditer(cmd):
                    jigc_commands.append(m.group(1).strip())
        elif etype in ("user", "tool_result"):
            # A tool-bearing stream (even if the agent ran no jigc) — distinguishes
            # "result-only json" from "stream-json where the agent simply bypassed".
            has_tool_stream = True
        elif etype == "result":
            result_text = event.get("result")
            num_turns = event.get("num_turns", num_turns)
            cost_usd = event.get("total_cost_usd", cost_usd)

    # First verb of each jigc invocation (`start`, `doc`, `task`, `validate`, …).
    verbs = [c.split()[0] for c in jigc_commands if c.split()]
    selected = None
    for c in jigc_commands:
        m = _WORKFLOW.search(c)
        if m:
            selected = m.group(1).strip().strip("\"'")
            break
    finalized = any(_FINALIZE.search(c) for c in jigc_commands)
    # Rename-engagement (M35 verb-engagement confound, a PRIMARY measured quantity):
    # did the agent use `jigc rename` (the verb), or rename by hand (`git mv`/`sd`/
    # a file edit)? Keyed on the `rename` verb specifically — NOT on `engaged` (any
    # jigc ran), so an agent that runs `jigc validate` but hand-edits the rename
    # scores `rename_engaged: False` while `engaged: True`. Availability ≠
    # induced-usage: a hand-edit re-incurs the block→recovery cost, so the cost win
    # evaporates and the study must measure verb-use, not verb-presence.
    rename_engaged = ("rename" in verbs) if has_tool_stream else None

    return {
        "format": "stream" if has_tool_stream else "result-only",
        # `engaged`/`finalized`/`rename_engaged` are unknowable without a tool
        # stream → null, not false.
        "engaged": bool(jigc_commands) if has_tool_stream else None,
        "jigc_commands": jigc_commands,
        "jigc_verbs": sorted(set(verbs)),
        "selected_workflow": selected,
        "finalized": finalized if has_tool_stream else None,
        "rename_engaged": rename_engaged,
        "num_turns": num_turns,
        "cost_usd": cost_usd,
        "result_text": result_text,
    }


def classify_outcome(validate_text: str | None = None,
                     old_symbol: str | None = None,
                     doc_texts: list[str] | None = None) -> str:
    """Classify a run's doc↔code outcome.

    Preference order:
    1. A `jigc validate` capture (the Phase-2 floor's store sweep — the precise
       oracle): any `doc-code.*` finding → `drift`; "validates clean" → `clean`.
    2. An old-symbol grep over the provided managed-doc texts (works for non-jigc
       arms too): the old name still present in a doc → `drift`, else `clean`.
    Otherwise `unknown`.
    """
    if validate_text is not None:
        if re.search(r"\bdoc-code\.", validate_text):
            return "drift"
        if "validates clean" in validate_text or "no findings" in validate_text:
            return "clean"
    if old_symbol and doc_texts is not None:
        hit = any(old_symbol in t for t in doc_texts)
        return "drift" if hit else "clean"
    return "unknown"


# --------------------------------------------------------------------------- CLI

def _read(path: str) -> str:
    return Path(path).read_text(encoding="utf-8", errors="replace")


def _selftest() -> int:
    here = Path(__file__).parent
    fx = here / "fixtures"
    cases = {
        "engaged-single-task.stream.json": dict(
            engaged=True, selected_workflow="single-task", finalized=True,
            rename_engaged=False, verbs={"start", "doc", "task"}),
        "bypassed-quick-fix.stream.json": dict(
            engaged=True, selected_workflow="quick-fix", finalized=False,
            rename_engaged=False, verbs={"start"}),
        "no-engagement.stream.json": dict(
            engaged=False, selected_workflow=None, finalized=False,
            rename_engaged=False, verbs=set()),
        # The verb-engagement confound (M35): used `jigc rename` …
        "rename-engaged.stream.json": dict(
            engaged=True, selected_workflow=None, finalized=False,
            rename_engaged=True, verbs={"rename"}),
        # … vs hand-edited the rename (`git mv`+`sd`) while still running some
        # other `jigc` verb — so `engaged` is True but `rename_engaged` is False.
        # This is the RED: a classifier keying only on "any jigc ran" would
        # wrongly mark this bypass engaged.
        "rename-by-hand.stream.json": dict(
            engaged=True, selected_workflow=None, finalized=False,
            rename_engaged=False, verbs={"validate"}),
    }
    failures = 0
    for name, want in cases.items():
        got = analyze_transcript(_read(str(fx / name)))
        for key in ("engaged", "selected_workflow", "finalized", "rename_engaged"):
            if got.get(key) != want[key]:
                print(f"FAIL {name}: {key} = {got.get(key)!r}, want {want[key]!r}")
                failures += 1
        if set(got["jigc_verbs"]) != want["verbs"]:
            print(f"FAIL {name}: verbs = {got['jigc_verbs']}, want {sorted(want['verbs'])}")
            failures += 1
    # Outcome classifier.
    assert classify_outcome(validate_text="blocking · doc-code.symbol-exists — …") == "drift"
    assert classify_outcome(validate_text="no findings — the committed store validates clean") == "clean"
    assert classify_outcome(old_symbol="OldName", doc_texts=["uses OldName here"]) == "drift"
    assert classify_outcome(old_symbol="OldName", doc_texts=["all renamed"]) == "clean"
    assert classify_outcome() == "unknown"
    if failures:
        print(f"\n{failures} assertion(s) failed")
        return 1
    print(f"selftest OK — {len(cases)} transcript fixtures + outcome classifier")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("transcript", nargs="?", help="path to the run transcript")
    ap.add_argument("--validate", help="path to a `jigc validate` capture (outcome oracle)")
    ap.add_argument("--old-symbol", help="pre-rename symbol name to grep for drift")
    ap.add_argument("--doc", action="append", default=[], help="managed-doc file to grep (repeatable)")
    ap.add_argument("--selftest", action="store_true", help="run fixture assertions and exit")
    args = ap.parse_args()

    if args.selftest:
        return _selftest()
    if not args.transcript:
        ap.error("a transcript path is required (or pass --selftest)")

    record = analyze_transcript(_read(args.transcript))
    record["outcome"] = classify_outcome(
        validate_text=_read(args.validate) if args.validate else None,
        old_symbol=args.old_symbol,
        doc_texts=[_read(d) for d in args.doc] if args.doc else None,
    )
    json.dump(record, sys.stdout, indent=2)
    sys.stdout.write("\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
