# Row 2 · probe integrity · measurement / the invocation log — driver scope (rc.24)

What the Opus driver is told **beyond** the standing driver brief. Source of the row: M54's S18
axes *probe integrity* and *measurement / the invocation log*. **No numbered-axis predecessor —
first drive.**

## Standing for this run (every row)

- **Binary:** `~/.local/bin/jigc`; `jigc --version` must print `jigc 1.0.0-rc.24` — assert it first and
  STOP if it does not. Release posture. Never `target/debug/jigc`, never `cargo run`.
- **Rigs:** `rig=$(dev/jigc-rig <state> --binary ~/.local/bin/jigc) || exit; eval "$rig"` — two steps,
  stdout only (never `2>&1` into the capture). The rig's default binary is the **debug** one, so
  `--binary` is not optional. No teardown; never `rm -rf` a variable path.
- **`CLAUDECODE` is set in your session.** Every commit jigc makes in a rig therefore carries
  `Co-Authored-By: Claude <noreply@anthropic.com>` unless the cell clears it (`env -u CLAUDECODE …`
  or `CLAUDECODE= …`). On this row that is expected, not a finding — row 10 owns it.
- **Keys:** a baseline row keeps its `(axis, id)` key verbatim; a new finding is keyed `(R2, <id>)`.
- Under `.jigc/` use `command grep` with a before-control — **this row reads `.jigc/logs/` all the
  time, and the harness `grep` says *not found* there whether or not the bytes exist.**

## READ FIRST

- **Baseline: none — first drive.** `COMMITTING_DOORS` was the door set of numbered axes 2 and 4;
  their posture and rollback rows are *not* this row's.
- The contract: `implementation/module-layout.md` → *Probe boundary*; `design/validation.md` →
  *Pack-probe determinism contract* (the six rules, the wire contract, *Failure semantics —
  meta-findings*) and → *The `doc-code` probe*; `design/measurement.md` → *The in-repo invocation
  log* and → *Honest bounds*; `design/surface-contract.md` → the error-code namespace;
  `design/assistant-adapter.md` → the hook's cases; DECISIONS.md → *M54 settled* S1 and S4.
- What changed: `completions/artifacts/M54/VERDICT.md` (finding 3; scenarios 1–6; *Not run*).

## DERIVE THE DOOR SET FROM — and state the count you read

| registry | file | what the instrument's author read (compare, do not copy) |
|---|---|---|
| production callers of `doc_code_invoker` (there is **no named registry** of probe doors — this is a derivation; state it as one) | `crates/cli/src/cli.rs`, `task.rs`, `milestone.rs` | 3 call sites: the store sweep, the task gate, the milestone boundary |
| the doors those sites are reached from | header of `crates/cli/tests/probe_failure_doors.rs` | 6 doors over 5 paths: `validate` · `task validate` · `task finalize` · `milestone finalize` · bare `start` (orientation) · the pre-commit hook's `jigc validate` |
| `ProbeArgv` | `crates/cli/src/invoke.rs` | 3 variants: not-a-probe · run · refuse(reason) |
| the probe's own failure arms | `crates/cli/src/doc_code_probe/mod.rs` → `run` | M54's VERDICT names 5 (stdin read · request parse · snapshot read · snapshot parse · response serialize) — re-count |
| `PROBE_STDERR_BOUND` | `crates/engine/src/probe.rs` | 4096 bytes |
| `COMMITTING_DOORS` | `crates/cli/src/invocation_log.rs` | 11 rows over 9 verbs |
| `ERROR_CODE_REGISTRY` | `crates/cli/src/invocation_log.rs` | 12 (the 11 doors' identities + `migrate.review-pending`) |
| the log record's key set | `crates/cli/src/invocation_log.rs` module doc; `design/measurement.md` → *Record shape* | 8 keys: `timestamp, argv, exit_code, duration_ms, finding_codes, output_bytes, binary_version, error_code` |
| `STORE_EXIT_FLIPS` → `probe-unreliable` | `crates/cli/src/render.rs` | 1 of 7 members (its exit is row 3's; its producer is yours) |

## CELL SET

**Probe (× all six doors unless stated):**

1. **Self-exec, the control** — no override, no sibling file beside the binary: the probe runs and
   reports the drifted anchor (`doc-code.symbol-exists`), with no probe-failure finding.
2. **Could not start** — `JIGC_DOC_CODE_PROBE` naming a non-executable regular file · naming no file ·
   set to the empty string. The finding's message names the program.
3. **A skewed build** — drive the real child once (`jigc __probe doc-code --build 0.0.0-skew`), read
   its stderr and exit, then replay those through an override stub; the stub must not exec `jigc`.
4. **Malformed `__probe` argv** — missing `--build`, extra args, a wrong probe id, `--format json
   __probe …` (first-word-only by design); `jigc --help` carries no `__probe`.
5. **Each failure arm** — garbage on stdin, a valid request naming an unreadable or unparseable
   snapshot: one reason line on stderr, stdout empty, non-zero exit.
6. **The stderr bound** — a child writing far past the bound: the message is cut at it and the door
   returns promptly.
7. **Install shapes** — the binary reached through a symlink · resolved from `PATH` · copied to
   another directory · a stale file named `doc-code` beside it that answers "no findings".
8. **One key for a twice-probed task** — a task-scoped door whose gate probes the staged index and
   the base: one finding, not two.

**The invocation log (knob on unless stated):**

9. One record per invocation, the declared 8 keys and no other, on success · on a findings exit · on
   a clap usage error (exit 2) · on an operational failure.
10. **A probe spawn adds zero records** — at the CLI doors and at the hook door.
11. Knob **off** (the default) → nothing written; **outside a project layer** → nothing written and
    no error.
12. **Each `COMMITTING_DOORS` row under a rejecting `pre-commit` hook** → that row's own `error_code`
    in the record, exit 1, no commit; and the exit-4 review hold → `migrate.review-pending`.
13. A log that cannot be written (the logs directory read-only, or a file where the directory should
    be) → the invocation's exit and output are unchanged.
14. `binary_version` equals `jigc --version`'s version; `output_bytes` equals the bytes actually
    emitted on both streams.

## BASELINE ROWS TO RE-DRIVE

None. From M54's *Not run*: the **Linux replaced-binary-while-running** arm (`/proc/self/exe`) cannot
be driven on this host — mark it **NOT DRIVEN (needs Linux; Docker cell)**, and if Docker answers and
you have budget, say whether `dev/runner-faithful` gives you a way to reach it or not.

## RIG STATES

- **`vendored`** — the probe cells: it carries a spec and an arch-doc whose code anchors resolve, so
  renaming the anchored symbol makes the control arm speak. Mint the task-scoped doors' task with
  `--start` or through the binary.
- **`fresh`** — the log cells; turn the knob on with `jigc config set invocation-log true` (a recorded
  project-layer delta — say whether you committed it before the cells that follow).
- **`bare`** — *outside a project layer*.
- **`chatty-hooks`** — a non-blocking hook's stream beside the log record.
- **Non-rig fixtures, named at their cells:** a rejecting hook (behind `core.hooksPath` in a
  `mktemp -d`), the override stubs (a `chmod 644` file; a compiled or shell stub that only replays
  bytes), the relocated/symlinked binary copies under `mktemp -d`. For how each committing door is
  brought to its commit, read `crates/cli/tests/support/committing_doors.rs` and reproduce the
  construction with the binary.

## ENVIRONMENT NOTES

- **`JIGC_DOC_CODE_PROBE` must be unset for the control arm.** Check your own environment first
  (`[ -z "${JIGC_DOC_CODE_PROBE+x}" ]`), and print only that it is set or unset, never a value.
- A relocated copy of the binary is a **copy of the installed release build** — `cp` it into a
  `mktemp -d`; do not build one.
- The hook door prints nothing for a probe-integrity finding by design; its observable is the
  `finding_codes` of the record the hook's own `jigc validate` writes — so that cell needs the knob on.
- The record carries `argv`: when you quote a record in a repro block, quote it from a rig whose
  path is under your scratch root and elide the host prefix.
