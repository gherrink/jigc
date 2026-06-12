# The dogfood measurement apparatus

The versioned capture substrate for M17's measured runs ([design/measurement.md](../../design/measurement.md) → The capture substrate, item 1): a Claude Code PostToolUse hook set + the pinned append-only log format + the tally script that derives the mechanized facts. **Home note:** this directory holds *runnable apparatus* in a directory of process markdown — a conscious placement, same bucket as the external build harness, not an erosion of the routing convention.

The scripts run on stock `python3` (the no-non-stock-interpreter bound: they must run under `cargo test` on a dev machine — `crates/cli/tests/dogfood_apparatus.rs` executes both for real). Fact *definitions* live in measurement.md; this README documents only the apparatus mechanics. Seeded attribution is transcription protocol, never tally-inferred — the tally emits per-event grouped detail + totals so the transcribing agent can attribute seeded events against the run protocol.

## Install (per measured run)

1. Export the two env vars in the session that runs the twin:
   - `JIGC_DOGFOOD_HOME` — the absolute path of this directory (a jigc checkout).
   - `JIGC_DOGFOOD_LOG` — the absolute log path, **outside the twin repo** (the log is measurement apparatus, not project content), e.g. `/tmp/jigc-dogfood/<run>/hook-log.jsonl`.
2. Merge `hooks.json` into the twin's `.claude/settings.json` (both `PostToolUse` matchers — `Bash` and `Write|Edit` — route to `log-event.py`).
3. **Pre-pilot smoke check** (before burning a pilot session): in the hooked session, run one Bash command containing a jigc invocation (e.g. `jigc --help`), then confirm the logged event's `exit` is **non-null** (`tail -1 "$JIGC_DOGFOOD_LOG"`). The hook reads `tool_response.exitCode`/`exit_code` and never invents a value — if the harness payload carries neither, every jigc event logs `exit: null` and the tally will refuse the whole log (see below).
4. After the run: `python3 tally.py "$JIGC_DOGFOOD_LOG" --managed-prefix <dir/> ...` — one `--managed-prefix` per active schema `location:` dir (e.g. `decisions/`, `dogfood/`); the task working-area rule (`.jigc/tasks/<id>/docs/`) is built in.

The raw log + the tally output are exported **unchanged** into the run's owner-artifact (measurement.md → the per-case two-half shape).

## The v1 log schema (pinned, append-only JSONL)

One JSON object per line; the hook only appends, never rewrites. Common fields: `"v": 1` (schema version — bump on any shape change; the tally must keep reading committed v1 capture, pinned by the fixture test) and `"ts"` (ISO-8601 UTC, informational).

**`"event": "jigc"`** — one per **jigc invocation** in a Bash command (non-jigc commands are not logged; a compound command — `&&` / `||` / `;` / `|` — logs one event per invocation it contains):

```json
{"v":1,"ts":"2026-06-12T10:00:00+00:00","event":"jigc","cmd":"task finalize t1 --format json","exit":0,"findings":[{"code":"reconciliation.absorb","path":"decisions/0001-x.md"}]}
```

- `cmd` — the argument string after the invocation's `jigc` token (the token may be path-qualified: exactly `jigc` or ending in `/jigc`), truncated at the next shell operator.
- `exit` — the exit code when the harness payload carries one (`tool_response.exitCode`/`exit_code`), else `null`. Every invocation extracted from one compound command shares that command's single exit code (see Known bounds).
- `findings` — `{code, path}` pairs extracted from the captured output, via the `--format json` envelope (`findings[].code` + `findings[].location.address`) or the agent-text finding line (path from the first backticked token); deduplicated.

**`"event": "file_op"`** — one per Write/Edit tool operation (every path, unclassified — classification is the tally's job). Write/Edit supply **absolute** paths; the hook records the path **repo-relative** when it sits under `CLAUDE_PROJECT_DIR` (the repo root Claude Code sets for every hook command), else verbatim — so the tally's (path × window) OOB dedup key is byte-identical with the `reconciliation.absorb` channel's repo-relative finding paths (the channels corroborate, never sum):

```json
{"v":1,"ts":"2026-06-12T10:03:00+00:00","event":"file_op","tool":"Write","path":"decisions/0001-x.md"}
```

The hook always exits 0 (a measurement hook never perturbs the run); with `JIGC_DOGFOOD_LOG` unset the apparatus is off.

## The tally rules

`tally.py` emits one JSON report (totals + per-event detail) applying measurement.md's units:

- **Finalize window** — windows are 1-based; a `finalize` invocation with `exit: 0` *closes* the current window (the landed finalize belongs to the window it closes).
- **adapter-writes** — successful write-verb invocations (`doc create|set-field|set-slot|add-item`) grouped one logical mutation per (doc address × finalize window); the address is the positional arg with any `#fragment` stripped, and a `create`'s minted `type:slug(title)`. Raw counts ride as `jigc-invocations` / `write-verb-invocations` telemetry, never the headline.
- **oob-edits** — one per (path × window), post-dedup across the two corroborating channels (`write-edit` hook observation on a managed path · `reconciliation.absorb` finding); the channels are named per event, never summed.
- **drift-caught** vs **validate-blocks** — exit-3 keying: a `finalize` exit 3 keys the drift bucket, a `validate` exit 3 keys the paired validate-blocks count; each entry carries its finding codes (the qualitative what-it-caught record).
- **Null-exit refusal** — every fact above except oob-edits keys on exits, so a log whose `jigc` events carry `exit: null` would silently tally as all zeros while telemetry still accrues. The tally **refuses** such a log (exit 2, message naming the null-exit count and the smoke check) rather than emit a plausible-looking report; `file_op` events carry no exit and are unaffected.

## Known bounds (the hook is a tokenizer, not a shell parser)

The hook extracts jigc invocations with a bounded `shlex` tokenization (quotes respected, shell operators split out) — deliberately **not** a shell parser:

- **Substitutions, subshells, heredocs and redirect targets are not interpreted** — `$(jigc ...)` is not recognized as an invocation; a redirect target after an invocation is simply not part of its `cmd`. An unbalanced quote degrades the whole command to plain whitespace splitting.
- **Quoted prose is sound, unquoted prose is noise** — `echo "run jigc task validate next"` logs nothing (the quoted span is one token), but the *unquoted* `echo run jigc task validate next` still logs one event whose `cmd` is the prose tail. Such a noise event carries the compound command's exit code (echo exits 0), so the tally's exit-3 keying ignores it; it only inflates `jigc-invocations` telemetry.
- **One exit code per compound command** — the harness reports a single exit for the whole Bash command, so every invocation extracted from a compound shares it. Consequences for the tally's per-event exit keying: in a `&&` chain that exits 0 every stage genuinely landed (each landed `finalize` closes its own window — two finalizes in one compound close two windows); but in a `;`-joined compound the shared exit is the *last* command's, and on a non-zero shared exit the tally keys **every** finalize/validate invocation in the compound (it cannot tell which stage failed) — possible over-attribution of `drift-caught`/`validate-blocks`. The transcription protocol should avoid `;`-compounded jigc invocations during a measured run.
