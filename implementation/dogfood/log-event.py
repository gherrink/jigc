#!/usr/bin/env python3
"""The dogfood capture hook — one PostToolUse observation in, one v1 JSONL event out.

Wired by `hooks.json` for two matchers (`design/measurement.md` -> The capture
substrate, item 1):

- `Bash`       -> records every **jigc invocation** (argv after the `jigc` token,
                  exit code, finding codes extracted from the captured output);
                  non-jigc commands are not logged.
- `Write|Edit` -> records the direct file operation (path only) — the OOB
                  denominator channel, identical on every comparison arm.

The log path is env-configured (`JIGC_DOGFOOD_LOG`, an absolute path OUTSIDE the
twin repo — measurement apparatus, not project content). Unset = apparatus off:
exit 0 silently. The hook ALWAYS exits 0 — a measurement hook never perturbs the
run it observes; extraction errors degrade to an empty findings list, never a
block. The schema is pinned in README.md; the tally is its consumer and test.
"""

import json
import os
import re
import sys
from datetime import datetime, timezone

# Agent-text finding line: `<severity> · <code> — <message>` (crates/cli/src/render.rs).
FINDING_LINE = re.compile(r"^(?:blocking|warning|advisory) · (\S+) — (.*)$")
# A jigc invocation inside a (possibly compound) shell command.
JIGC_CMD = re.compile(r"(?:^|[\s;&|(])jigc\s+(.+)", re.S)
BACKTICKED = re.compile(r"`([^`]+)`")


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


def exit_code(tool_response):
    """The invocation's exit code, when the harness payload carries one."""
    if isinstance(tool_response, dict):
        for key in ("exitCode", "exit_code"):
            if isinstance(tool_response.get(key), int):
                return tool_response[key]
    return None


def event_for(payload):
    """Map one PostToolUse payload to its v1 log event, or None (not observed)."""
    tool = payload.get("tool_name")
    tool_input = payload.get("tool_input") or {}
    tool_response = payload.get("tool_response")

    if tool == "Bash":
        command = tool_input.get("command") or ""
        m = JIGC_CMD.search(command)
        if not m:
            return None
        stdout, stderr = "", ""
        if isinstance(tool_response, dict):
            stdout = tool_response.get("stdout") or ""
            stderr = tool_response.get("stderr") or ""
        elif isinstance(tool_response, str):
            stdout = tool_response
        return {
            "event": "jigc",
            "cmd": m.group(1).strip(),
            "exit": exit_code(tool_response),
            "findings": findings_from_output(stdout, stderr),
        }

    if tool in ("Write", "Edit"):
        path = tool_input.get("file_path")
        if not path:
            return None
        return {"event": "file_op", "tool": tool, "path": path}

    return None


def main():
    log_path = os.environ.get("JIGC_DOGFOOD_LOG")
    if not log_path:
        return  # apparatus off — never block the run
    try:
        payload = json.load(sys.stdin)
        event = event_for(payload)
        if event is None:
            return
        line = {"v": 1, "ts": datetime.now(timezone.utc).isoformat(timespec="seconds")}
        line.update(event)
        os.makedirs(os.path.dirname(os.path.abspath(log_path)), exist_ok=True)
        with open(log_path, "a", encoding="utf-8") as log:
            log.write(json.dumps(line, ensure_ascii=False) + "\n")
    except Exception as err:  # noqa: BLE001 — observation must never perturb the run
        print(f"jigc dogfood hook: dropped one observation ({err})", file=sys.stderr)


if __name__ == "__main__":
    main()
    sys.exit(0)
