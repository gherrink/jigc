#!/usr/bin/env python3
"""The dogfood tally — derive the mechanized facts from a v1 hook log.

Usage: tally.py <hook-log.jsonl> [--managed-prefix <dir/>]...

Reads the append-only v1 log `log-event.py` produces (schema pinned in README.md)
and emits one JSON report on stdout: totals + per-event grouped detail. The rules
(`design/measurement.md` -> The three facts):

- **adapter-writes** — logical mutations: successful jigc write-verb invocations
  (`doc create` / `set-field` / `set-slot` / `add-item`) grouped per
  (doc address x finalize window). A window is the span up to and including each
  LANDED finalize (exit 0); raw invocation counts ride as telemetry, never the
  headline.
- **oob-edits** — post-dedup: ONE event per (path x window), assembled from two
  corroborating channels — hook-observed Write|Edit on a managed-doc path, and
  `reconciliation.absorb` findings in captured jigc output. The channels
  corroborate, never sum. Managed-doc paths = the `--managed-prefix` dirs (the
  active schemas' `location:` dirs) plus the built-in task working-area rule
  (`.jigc/tasks/<id>/docs/`).
- **drift-caught** — a `finalize` invocation exiting 3 (validation-blocked);
  **validate-blocks** — a `validate` invocation exiting 3 (the paired,
  non-headline count). Finding codes ride each entry (the qualitative record).

The tally counts raw capture only: **seeded attribution is transcription
protocol, never tally-inferred** — the per-event detail exists so the
transcribing agent can attribute seeded events against the run protocol.
"""

import json
import re
import shlex
import sys

WRITE_VERBS = {"create", "set-field", "set-slot", "add-item"}
SHELL_OPERATORS = {"&&", "||", ";", "|", "&"}
WORKING_AREA = re.compile(r"(?:^|/)\.jigc/tasks/[^/]+/docs/")


def parse_args(argv):
    if len(argv) < 2:
        sys.stderr.write(__doc__)
        sys.exit(1)
    log_path, prefixes = argv[1], []
    rest = argv[2:]
    while rest:
        flag = rest.pop(0)
        if flag == "--managed-prefix" and rest:
            prefixes.append(rest.pop(0))
        else:
            sys.stderr.write(f"tally.py: unknown argument {flag!r}\n")
            sys.exit(1)
    return log_path, prefixes


def jigc_tokens(cmd):
    """The jigc argument tokens, truncated at the first shell operator."""
    try:
        tokens = shlex.split(cmd)
    except ValueError:
        tokens = cmd.split()
    out = []
    for token in tokens:
        if token in SHELL_OPERATORS:
            break
        out.append(token)
    return out


def verb_class(tokens):
    """`finalize` / `validate` when the invocation is one, else None."""
    nonflag = [t for t in tokens if not t.startswith("-")][:2]
    for verb in ("finalize", "validate"):
        if verb in nonflag:
            return verb
    return None


def slug(title):
    """The id-from-title slug a `doc create` mints (lowercase, non-alnum runs -> -)."""
    return re.sub(r"[^a-z0-9]+", "-", title.lower()).strip("-")


def doc_address(tokens):
    """The doc address a write-verb invocation targets, or None when not one."""
    if len(tokens) < 3 or tokens[0] != "doc" or tokens[1] not in WRITE_VERBS:
        return None
    positional = [t for t in tokens[2:] if not t.startswith("-")]
    if not positional:
        return None
    if tokens[1] == "create":
        if len(positional) < 2:
            return None
        return f"{positional[0]}:{slug(positional[1])}"
    return positional[0].split("#", 1)[0]


def is_managed(path, prefixes):
    normalized = path[2:] if path.startswith("./") else path
    if WORKING_AREA.search(normalized):
        return True
    return any(normalized.startswith(p) for p in prefixes)


def tally(events, prefixes):
    window = 1
    jigc_invocations = 0
    write_invocations = 0
    mutations = {}  # (address, window) -> raw invocation count
    oob = {}  # (path, window) -> set of channels
    drift, blocks = [], []

    for event in events:
        if event.get("v") != 1:
            continue
        kind = event.get("event")

        if kind == "file_op":
            path = event.get("path") or ""
            if is_managed(path, prefixes):
                oob.setdefault((path, window), set()).add("write-edit")
            continue

        if kind != "jigc":
            continue
        jigc_invocations += 1
        tokens = jigc_tokens(event.get("cmd") or "")
        exit_code = event.get("exit")
        codes = [f["code"] for f in event.get("findings") or [] if "code" in f]

        for finding in event.get("findings") or []:
            if finding.get("code") == "reconciliation.absorb" and finding.get("path"):
                oob.setdefault((finding["path"], window), set()).add("absorb")

        address = doc_address(tokens)
        if address is not None and exit_code == 0:
            write_invocations += 1
            key = (address, window)
            mutations[key] = mutations.get(key, 0) + 1

        verb = verb_class(tokens)
        if verb == "finalize" and exit_code == 3:
            drift.append({"window": window, "cmd": event.get("cmd"), "findings": codes})
        if verb == "validate" and exit_code == 3:
            blocks.append({"window": window, "cmd": event.get("cmd"), "findings": codes})
        if verb == "finalize" and exit_code == 0:
            window += 1  # a landed finalize closes the window

    return {
        "v": 1,
        "note": (
            "raw capture only — seeded attribution is transcription protocol, "
            "never tally-inferred"
        ),
        "totals": {
            "adapter-writes": len(mutations),
            "oob-edits": len(oob),
            "drift-caught": len(drift),
            "validate-blocks": len(blocks),
            "jigc-invocations": jigc_invocations,
            "write-verb-invocations": write_invocations,
        },
        "detail": {
            "logical-mutations": [
                {"address": address, "window": w, "invocations": count}
                for (address, w), count in sorted(mutations.items(), key=lambda i: (i[0][1], i[0][0]))
            ],
            "oob-edits": [
                {"path": path, "window": w, "channels": sorted(channels)}
                for (path, w), channels in sorted(oob.items(), key=lambda i: (i[0][1], i[0][0]))
            ],
            "drift-caught": drift,
            "validate-blocks": blocks,
        },
    }


def main():
    log_path, prefixes = parse_args(sys.argv)
    events = []
    with open(log_path, encoding="utf-8") as log:
        for line in log:
            if not line.strip():
                continue
            try:
                events.append(json.loads(line))
            except ValueError:
                sys.stderr.write("tally.py: skipped one unparseable line\n")
    report = tally(events, prefixes)
    json.dump(report, sys.stdout, indent=2, sort_keys=True, ensure_ascii=False)
    sys.stdout.write("\n")


if __name__ == "__main__":
    main()
