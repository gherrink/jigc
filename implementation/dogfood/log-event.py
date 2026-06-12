#!/usr/bin/env python3
"""The dogfood capture hook — one PostToolUse observation in, one v1 JSONL event out.

Wired by `hooks.json` for two matchers (`design/measurement.md` -> The capture
substrate, item 1):

- `Bash`       -> records every **jigc invocation** (argv after the `jigc` token,
                  exit code, finding codes extracted from the captured output);
                  non-jigc commands are not logged.
- `Write|Edit` -> records the direct file operation (path only; repo-relative
                  when under `CLAUDE_PROJECT_DIR`) — the OOB denominator
                  channel, identical on every comparison arm.

The log path is env-configured (`JIGC_DOGFOOD_LOG`, an absolute path OUTSIDE the
twin repo — measurement apparatus, not project content). Unset = apparatus off:
exit 0 silently. The hook ALWAYS exits 0 — a measurement hook never perturbs the
run it observes; extraction errors degrade to an empty findings list, never a
block. The schema is pinned in README.md; the tally is its consumer and test.
"""

import json
import os
import re
import shlex
import sys
from datetime import datetime, timezone

# Agent-text finding line: `<severity> · <code> — <message>` (crates/cli/src/render.rs).
FINDING_LINE = re.compile(r"^(?:blocking|warning|advisory) · (\S+) — (.*)$")
BACKTICKED = re.compile(r"`([^`]+)`")
# The token characters shlex splits out as shell punctuation — an operator token
# ends the current invocation.
SHELL_PUNCT = frozenset("();<>|&")


def split_tokens(command):
    """Tokenize a Bash command — quotes respected, shell operators split out.

    A bounded shlex tokenization, NOT a shell parser: substitutions, subshells,
    heredocs and redirect targets are not interpreted (README -> Known bounds).
    A lexing error (e.g. an unbalanced quote) degrades to whitespace splitting.
    """
    lex = shlex.shlex(command, punctuation_chars=True)
    lex.whitespace_split = True
    lex.commenters = ""  # `#` is address syntax (`doc:id#field`), never a comment
    try:
        return list(lex)
    except ValueError:
        return command.split()


def jigc_invocations(command):
    """Every jigc invocation in a (possibly compound) Bash command.

    Each invocation is the argument string after its jigc token, truncated at
    the next shell operator. The token may be path-qualified — exactly `jigc`
    or ending in `/jigc` (`/usr/local/bin/jigc`, `./jigc`). Quoted prose that
    merely mentions jigc (`echo "run jigc ..."`) is a single token and never
    matches; an UNQUOTED prose mention still logs (README -> Known bounds).
    """
    invocations, current = [], None
    for token in split_tokens(command):
        if token and all(c in SHELL_PUNCT for c in token):
            if current:
                invocations.append(" ".join(current))
            current = None
            continue
        if current is not None:
            current.append(token)
        elif token == "jigc" or token.endswith("/jigc"):
            current = []
    if current:
        invocations.append(" ".join(current))
    return invocations


def findings_from_output(*chunks):
    """Extract (code, path) findings from captured output.

    Two channels: the `--format json` envelope (`findings[].code` +
    `findings[].location.address`) and the agent-text rendering (the finding line,
    path recovered from the first backticked token in the message). Deduplicated,
    order-preserving.
    """
    found = []
    for chunk in chunks:
        if not isinstance(chunk, str) or not chunk.strip():
            continue
        try:
            envelope = json.loads(chunk)
        except ValueError:
            envelope = None
        if isinstance(envelope, dict) and isinstance(envelope.get("findings"), list):
            for f in envelope["findings"]:
                if not isinstance(f, dict) or "code" not in f:
                    continue
                location = f.get("location") or {}
                path = location.get("address") if isinstance(location, dict) else None
                found.append({"code": str(f["code"]), "path": path})
            continue
        for line in chunk.splitlines():
            m = FINDING_LINE.match(line)
            if m:
                tick = BACKTICKED.search(m.group(2))
                found.append({"code": m.group(1), "path": tick.group(1) if tick else None})
    seen, unique = set(), []
    for f in found:
        key = (f["code"], f["path"])
        if key not in seen:
            seen.add(key)
            unique.append(f)
    return unique


def repo_relative(path):
    """Record a file_op path repo-relative when it sits under the repo root.

    Claude Code's Write|Edit tools supply ABSOLUTE paths, while the corroborating
    `reconciliation.absorb` channel carries repo-RELATIVE finding paths — and the
    tally's (path x window) OOB dedup key must be byte-identical across both
    channels (corroborate, never sum). The root is `CLAUDE_PROJECT_DIR`, the env
    Claude Code sets for every hook command. The payload `cwd` is NOT a fallback —
    the agent may have cd'd into a subdirectory, which would mint a wrong relative
    path that silently mismatches. No root known, or path outside it: verbatim.
    """
    root = os.environ.get("CLAUDE_PROJECT_DIR")
    if not root or not os.path.isabs(path):
        return path
    root = os.path.normpath(root)
    normalized = os.path.normpath(path)
    if normalized.startswith(root + os.sep):
        return normalized[len(root) + 1 :]
    return path


def exit_code(tool_response):
    """The invocation's exit code, when the harness payload carries one."""
    if isinstance(tool_response, dict):
        for key in ("exitCode", "exit_code"):
            if isinstance(tool_response.get(key), int):
                return tool_response[key]
    return None


def events_for(payload):
    """Map one PostToolUse payload to its v1 log events ([] when not observed).

    A Bash command yields one `jigc` event PER invocation it contains — a
    compound command (`a && b`) logs each. All invocations from one command
    share its single exit code and captured-output findings (README -> Known
    bounds): the harness observes the command, not the pipeline stages.
    """
    tool = payload.get("tool_name")
    tool_input = payload.get("tool_input") or {}
    tool_response = payload.get("tool_response")

    if tool == "Bash":
        invocations = jigc_invocations(tool_input.get("command") or "")
        if not invocations:
            return []
        stdout, stderr = "", ""
        if isinstance(tool_response, dict):
            stdout = tool_response.get("stdout") or ""
            stderr = tool_response.get("stderr") or ""
        elif isinstance(tool_response, str):
            stdout = tool_response
        exit_ = exit_code(tool_response)
        findings = findings_from_output(stdout, stderr)
        return [
            {"event": "jigc", "cmd": cmd, "exit": exit_, "findings": findings}
            for cmd in invocations
        ]

    if tool in ("Write", "Edit"):
        path = tool_input.get("file_path")
        if not path:
            return []
        return [{"event": "file_op", "tool": tool, "path": repo_relative(path)}]

    return []


def main():
    log_path = os.environ.get("JIGC_DOGFOOD_LOG")
    if not log_path:
        return  # apparatus off — never block the run
    try:
        payload = json.load(sys.stdin)
        events = events_for(payload)
        if not events:
            return
        ts = datetime.now(timezone.utc).isoformat(timespec="seconds")
        os.makedirs(os.path.dirname(os.path.abspath(log_path)), exist_ok=True)
        with open(log_path, "a", encoding="utf-8") as log:
            for event in events:
                line = {"v": 1, "ts": ts}
                line.update(event)
                log.write(json.dumps(line, ensure_ascii=False) + "\n")
    except Exception as err:  # noqa: BLE001 — observation must never perturb the run
        print(f"jigc dogfood hook: dropped one observation ({err})", file=sys.stderr)


if __name__ == "__main__":
    main()
    sys.exit(0)
