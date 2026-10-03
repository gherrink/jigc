<!-- Reconciled ROW 2 file (probe integrity · measurement / the invocation log), copied verbatim below this line. Driven on the installed registry build `~/.local/bin/jigc` -> `jigc 1.0.0-rc.24`, 2026-10-03. `axis2` in the body means ROW 2 of this run, not numbered axis 2. -->

> **Reconciled row file — row 2 · probe integrity · measurement / the invocation log (`jigc 1.0.0-rc.24`).**
> This row **has** a source pass (Codex, exit 0). Part A below is the Opus driver's record, unchanged except
> for the demotion marks written `[RECON: …]` (17 rows, all re-driven by the reconciler and reproducing);
> Part B, the *Reconciliation ledger*, follows it. **Outcome: 4 confirmed findings, all origin driver, all
> tier 3 · 0 tier 1 · 0 tier 2 · the one Codex defect claim (proposed tier 1) REFUTED by driving · 10 leads
> left open.** The reconciler did not author Part A and did not build the code; nothing in the repository
> was edited, committed or built.

---

# Row 2 · probe integrity · measurement / the invocation log — the driver's record (rc.24)

Partial per-axis re-review of the published `jigc 1.0.0-rc.24`. **First drive of this row — no baseline.**
Keys of this run are `(R2, <id>)`.

- **Binary:** the installed registry build `~/.local/bin/jigc`; `jigc --version` → `jigc 1.0.0-rc.24`
  (asserted first; exit 0). Release posture. No sibling `doc-code` file beside it.
- **Environment (set / unset only):** `JIGC_DOC_CODE_PROBE` unset · `JIGC_PACK_DIR` unset · `CLAUDECODE`
  set (held constant; every commit jigc made in a rig carries `Co-Authored-By: Claude
  <noreply@anthropic.com>`, expected on this row) · `GIT_DIR` unset.
- **Rigs:** `rig=$(dev/jigc-rig <state> --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit`
  — ten rigs (`vendored` ×4, `fresh` ×4, `bare` ×1, `chatty-hooks` ×1), every root from
  `mktemp -d`, no teardown. Non-rig fixtures under `<tmp>/fx.*` (a `mktemp -d`): the override stubs, the
  relocated / symlinked binary copies, the rejecting hook directory.
- **Source tree read for the registries:** the working repository at `bffa6667` (one tooling commit past
  the `aa6666cb` the instrument was read at; nothing under `crates/` differs for this row's symbols).
  Nothing in the repository was edited, committed or built.
- **Verdict in one line: no tier-1 row.** 174 rows driven, 4 defects recorded, all proposed **tier 3**.

---

## 1 · The door set, derived from the code — counts read beside the instrument's

| registry | where | instrument's count | **count read here** | note |
|---|---|---|---|---|
| production callers of `doc_code_invoker` (a derivation — there is no named registry of probe doors) | `cli.rs` (store sweep, `validate_store_in_repo`) · `task.rs` (`Task::validate`, reached by `run_validate`, `finalize`, `sweep_for_orientation`) · `milestone.rs` (`milestone_boundary_gate`'s `validate_task`) | 3 call sites | **3** | same |
| the doors those sites are reached from | header of `tests/probe_failure_doors.rs` + the callers above | 6 doors over 5 paths | **6 doors over 5 paths** — `validate` · `task validate` · `task finalize` · `milestone finalize` · bare `start` · the pre-commit hook's `jigc validate` | same. The header says *all five* and lists six; five clap leaves, the hook being a second route into `validate` |
| `ProbeArgv` | `invoke.rs` | 3 | **3** (`NotProbe` · `Run` · `Refuse(reason)`) | same |
| the probe's own failure arms | `doc_code_probe/mod.rs` → `run` | 5 | **5** `refuse(...)` call sites (stdin read · request parse · snapshot read · snapshot parse · response serialize) | same |
| `PROBE_STDERR_BOUND` | `engine/src/probe.rs` | 4096 | **4096** | same; driven: the message is 78 + 4096 chars |
| `COMMITTING_DOORS` | `invocation_log.rs` | 11 rows over 9 verbs | **11 rows over 9 verbs** | same (`task finalize` ×2, `milestone finalize` ×2) |
| `ERROR_CODE_REGISTRY` | `invocation_log.rs` | 12 | **12** | same; `design/surface-contract.md` names all 12 |
| the log record's key set | `invocation_log.rs` `Record` | 8 | **8**, emitted in the order `timestamp, argv, exit_code, duration_ms, finding_codes, output_bytes, binary_version, error_code` | same; read off emitted bytes |
| `STORE_EXIT_FLIPS` → `probe-unreliable` | `render.rs` | 1 of 7 | **present** (member read; the other six not counted — row 3's) | its producer driven here: every probe-failure arm at `validate` exits 1 |

**No count differs from the instrument's.** One datum the registries do not carry: M55's third commit
model (`DocOnly`, the path-scoped doc-only finalize) has **no `COMMITTING_DOORS` row of its own** — it is
the `jigc task finalize` leaf and logs `finalize.commit-rejected` (driven, §4 cell 12).

**Doors and their abbreviations below:** `V` = `jigc validate` · `TV` = `jigc task validate` · `TF` =
`jigc task finalize` · `MF` = `jigc milestone finalize` · `ST` = bare `jigc start` (orientation) · `HK` =
a real `git commit` through the pre-commit hook `jigc setup` installed · `PI` = the `__probe` intercept
(not a clap leaf; confers no leaf coverage).

Route kind column: the JSON envelope carries `route` as prose and no kind key, so the kind is read off the
route's content — `Human` (prose instruction), `Mechanical` (a runnable `jigc …`/`git …` line),
`none` (no finding).

---

## 2 · Fixtures (built once, named at the cells)

```text
# rig P1 — vendored, drifted, with a live task          (cells 1–3, 6–8, the extra stubs)
rig=$(dev/jigc-rig vendored --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
printf 'export function renamed(s: string): string {\n  return s;\n}\n' > src/pad.ts   # the anchored symbol `pad` renamed away
jigc start --workflow single-task "rename the pad"                                     # → task minted: rename-the-pad
jigc doc set-field commit:rename-the-pad#type --task rename-the-pad --value refactor
printf 'rename the pad\n' | jigc doc set-slot commit:rename-the-pad#summary --task rename-the-pad --from-file -
git add src/pad.ts

# rig P2 — vendored, a milestone whose one sub-task worktree stages the same drift   (door MF)
jigc milestone create "Pad rework"; jigc milestone add-task pad-rework "Rename the pad"; jigc milestone provision pad-rework
#   then, in $REPO/.jigc/worktrees/rename-the-pad: the same pad.ts rewrite, `git add src/pad.ts`

# rig P3 — vendored, invocation log ON and committed, drift in the working tree     (door HK, cell 10)
jigc config set invocation-log true      # "written to `.jigc/config/`, uncommitted — commit it with your next commit"
git add .jigc/config && git commit -q -m "chore: turn the invocation log on"          # committed before the cells that follow
#   then the pad.ts rewrite (unstaged), and per arm: a notes/<arm>.txt, `git add`, `git commit -q -m "arm <arm>"`

# override stubs under <tmp>/fx.*/  (none execs jigc)
doc-code-not-executable   'not a program\n', mode 644
no-such-probe             (a path that names no file)
doc-code-replay           #!/bin/sh · printf '%s\n' '<the real skewed child's stderr, verbatim>' >&2 · exit 1
                          (cmp against the real child's captured stderr: byte-identical)
doc-code-flood            #!/bin/sh · 4096 × 'a' · 'PAST-THE-BOUND' · 2,000,000 × 'b' on stderr · exit 1
doc-code-count            #!/bin/sh · appends a line to <tmp>/fx.*/spawn.count · one reason line on stderr · exit 1
doc-code-garbage0 / -empty0 / -signal / -silent1 / -clean0     (exit 0 + 'not json' · exit 0 + nothing · kill -9 $$ · exit 1 silently · exit 0 + {"findings":[],"schema_version":3})
doc-code-spin / -exec47 / -hang47 / -hang                        (while :; do :; done · exec sleep 47 · sleep 47 · sleep 120)

# install shapes under <tmp>/fx.*/  — copies of the INSTALLED release build, never a build of ours
link/jigc   → symlink to ~/.local/bin/jigc
copy/jigc   = cp ~/.local/bin/jigc      (cmp: byte-identical)
stale/jigc  = cp ~/.local/bin/jigc, beside an executable `doc-code` that touches a marker file and answers {"findings":[],"schema_version":1}

# the rejecting hook (cell 12): a directory from mktemp -d holding
#   pre-commit:  #!/bin/sh · echo "policy: COMMIT-REJECTED-R2-MARKER" 1>&2 · exit 1
# switched with `git config core.hooksPath <tmp>/hooks.*` / `git config --unset core.hooksPath`
```

---

## 3 · The probe cells

### Cell 1 — self-exec, the control (override unset, no sibling)

| door | argv driven | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|---|
| V | `jigc validate --format json` (P1) | 0 | `doc-code.symbol-exists` | Human | `findings[0].key.target = arch-doc:padding-layer#components/pad/implemented-by`; `blocking_probes: ["doc-code"]`; no `pack-probe-integrity` | matches |
| TV | `jigc --format json task validate rename-the-pad` | 3 | `doc-code.symbol-exists` (+ advisory `changelog-recording.gate-granted-unused`) | Human | message *"…absent from `src/pad.ts` in the staged index"* | matches |
| TF | `jigc --format json task finalize rename-the-pad` | 3 | `doc-code.symbol-exists` | Human | same; `HEAD` unmoved | matches |
| ST | `jigc --format json start` | 0 | `doc-code.symbol-exists` in `tasks[0].findings` | Human | `findings_unavailable: null` | matches |
| MF | `jigc --format json milestone finalize pad-rework` (P2) | 3 | `doc-code.symbol-exists` | Human | `HEAD` unmoved | matches |
| HK | `git commit -q -m "arm control"` (P3) | 0 (commit lands) | record: `finding_codes: ["doc-code.symbol-exists"]` | — | hook stderr: *"jigc: doc<->code drift detected in committed docs — run `jigc validate` for details (commit not blocked)."*; records +1 | matches (case (iv) control) |

### Cell 2 — could not start

**2a · the override names a non-executable regular file (mode 644)**

| door | argv (prefix `JIGC_DOC_CODE_PROBE=<tmp>/fx.*/doc-code-not-executable`) | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|---|
| V | `jigc validate --format json` | 1 | `pack-probe-integrity.probe-failure`, check `crash`, blocking | Human | message ``probe `doc-code` could not start `<tmp>/fx.*/doc-code-not-executable`: Permission denied (os error 13)``; `key.target: doc-code`; `blocking_probes: ["pack-probe-integrity"]`; no `symbol-exists` | matches |
| TV | `jigc --format json task validate rename-the-pad` | 3 | same, **one** finding | Human | same message | matches |
| TF | `jigc --format json task finalize rename-the-pad` | 3 | same | Human | `HEAD` unmoved | matches |
| ST | `jigc --format json start` | 0 | same, in the task row | Human | `findings_unavailable: null` | matches |
| MF | `jigc --format json milestone finalize pad-rework` | 3 | same | Human | `HEAD` unmoved | matches |
| HK | `git commit` | 0 | record `finding_codes: ["pack-probe-integrity.probe-failure"]`, `exit_code: 1` | — | hook stderr empty (case (v)); commit +1; records +1 | matches |

The text render's route, read once at V: *"the probe subprocess misbehaved (the message carries the reason)
— a shipped probe runs inside `jigc`, so reinstall `jigc` (or repair the probe binary an override names),
then re-run the sweep"* — the M54 rewrite, no `jigc setup` in it.

**2b · the override names no file**

| door | argv (prefix `JIGC_DOC_CODE_PROBE=<tmp>/fx.*/no-such-probe`) | exit | code | surface asserted | verdict |
|---|---|---|---|---|---|
| V | `jigc validate --format json` | 1 | none (`error` envelope on stderr, stdout empty) | ``"error": "`doc-code` probe not found at \"<tmp>/fx.*/no-such-probe\" — `JIGC_DOC_CODE_PROBE` names no file; point it at a probe executable, or unset it to run the probe built into `jigc`"`` | matches (`design/validation.md` → Distribution bound: the one-operational-error shape) |
| HK | `git commit` | 0 | record `exit_code: 1, finding_codes: [], error_code: null` | hook silent; commit +1; records +1 | matches (case (iv), override path) |
| TV | `jigc --format json task validate rename-the-pad` — task stages **no** anchored doc; its staged code is cited by a committed anchor | **3** | `pack-probe-integrity.probe-failure`, `crash` | ``probe `doc-code` could not start `<tmp>/fx.*/no-such-probe`: No such file or directory (os error 2)`` | **DEFECT (R2, P-1)** |
| TF | `… task finalize rename-the-pad` (same task state) | **3** | same | `HEAD` unmoved | **DEFECT (R2, P-1)** |
| MF | `… milestone finalize pad-rework` | **3** | same | `HEAD` unmoved | **DEFECT (R2, P-1)** |
| ST | `jigc --format json start` (same task state) | 0 | same, as a finding in the task row | `findings_unavailable: null` | part of (R2, P-1) |
| TV′ | the same argv after `jigc doc set-slot arch-doc:padding-layer#overview --task rename-the-pad --from-file -` (the task now stages an anchored doc) | **1** | none (`error` envelope, the *probe not found* text) | stdout empty | matches the pre-flight |
| TF′ | `… task finalize rename-the-pad` (same) | **1** | none | `HEAD` unmoved | matches the pre-flight |
| ST′ | `jigc --format json start` (same) | 0 | none | `tasks[0].findings_unavailable` = the *probe not found* text | matches |

**2c · the override set to the empty string** — treated as unset at every door (the control's result):

| door | argv (prefix `JIGC_DOC_CODE_PROBE=`) | exit | code | verdict |
|---|---|---|---|---|
| V · TV · TF · ST · MF | as cell 1 | 0 · 3 · 3 · 0 · 3 | `doc-code.symbol-exists`, no probe-failure | matches (M54 e2e scenario 5: *an empty override is unset*) |
| HK | `git commit` | 0 | record `["doc-code.symbol-exists"]`; the drift warning printed | matches |

### Cell 3 — a skewed build

```text
$ jigc __probe doc-code --build 0.0.0-skew </dev/null          # the real child, door PI
exit=1  stdout: 0 bytes  stderr (1 line):
jigc 1.0.0-rc.24: the `doc-code` probe was asked for build `0.0.0-skew`, but this jigc is build `1.0.0-rc.24` — the jigc binary changed while the run was in flight; rerun the command
```

Replayed through `doc-code-replay` (a shell stub of builtins only; its stderr `cmp`s equal to the capture):

| door | argv (prefix `JIGC_DOC_CODE_PROBE=<tmp>/fx.*/doc-code-replay`) | exit | code | surface asserted | verdict |
|---|---|---|---|---|---|
| V | `jigc validate --format json` | 1 | `pack-probe-integrity.probe-failure`, `crash` | ``probe `doc-code` exited non-zero (exit-code 1) with no usable output; stderr: jigc 1.0.0-rc.24: the `doc-code` probe was asked for build `0.0.0-skew`, but this jigc is build `1.0.0-rc.24` — …`` (260 chars); no `symbol-exists` | matches |
| TV · TF · MF | the three task/milestone argv | 3 · 3 · 3 | same, one finding each | `HEAD` unmoved at TF and MF | matches |
| ST | `jigc --format json start` | 0 | same in the task row | `findings_unavailable: null` | matches |
| HK | `git commit` | 0 | record `["pack-probe-integrity.probe-failure"]` | hook silent; commit +1; records +1 | matches |

### Cell 4 — malformed `__probe` argv (door PI; `</dev/null`)

| argv | exit | stdout | stderr (one line unless noted) | verdict |
|---|---|---|---|---|
| `jigc __probe` | 1 | 0 | ``jigc 1.0.0-rc.24: `__probe` takes exactly `doc-code --build 1.0.0-rc.24`, got `__probe` `` | matches |
| `jigc __probe doc-code` | 1 | 0 | same form, `got `__probe doc-code`` | matches |
| `jigc __probe doc-code --build` | 1 | 0 | same form | matches |
| `jigc __probe doc-code --build 1.0.0-rc.24 extra` | 1 | 0 | same form | matches |
| `jigc __probe other-probe --build 1.0.0-rc.24` | 1 | 0 | same form | matches |
| `jigc __probe doc-code --builds 1.0.0-rc.24` | 1 | 0 | same form | matches |
| `jigc __probe --help` | 1 | 0 | same form (no help text leaks) | matches |
| `jigc --format json __probe doc-code --build 1.0.0-rc.24` | 2 | 0 | clap: `error: unrecognized subcommand '__probe'` (5 lines) | matches (first-word-only by design) |
| `jigc __PROBE doc-code --build 1.0.0-rc.24` | 2 | 0 | clap: unrecognized subcommand | matches |
| `jigc --help` · `jigc help` · `jigc __prob` | 0 · 0 · 2 | — | `__probe` occurrences: **0 · 0 · 0** (no did-you-mean) | matches |

### Cell 5 — each failure arm (door PI; argv `jigc __probe doc-code --build 1.0.0-rc.24`, the request on stdin)

| arm | stdin | exit | stdout | stderr (exactly one line) | verdict |
|---|---|---|---|---|---|
| stdin read | three non-UTF-8 bytes | 1 | 0 | `doc-code probe: cannot read the request from stdin: stream did not contain valid UTF-8` | matches |
| request parse | `garbage` | 1 | 0 | `doc-code probe: cannot parse the request: expected value at line 1 column 1` | matches |
| request parse | empty · stdin closed (`<&-`) | 1 · 1 | 0 | `… cannot parse the request: EOF while parsing a value at line 1 column 0` | matches |
| snapshot read | valid request naming no file | 1 | 0 | ``doc-code probe: cannot read the snapshot `<tmp>/fx.*/no-such-snapshot.json`: No such file or directory (os error 2)`` | matches **[RECON: demoted — the request on stdin is not stated; re-driven with a stated request, ledger R-5: reproduces]** |
| snapshot read | … naming a mode-000 file · a directory | 1 · 1 | 0 | `… Permission denied (os error 13)` · `… Is a directory (os error 21)` | matches **[RECON: demoted — the request on stdin is not stated; re-driven with a stated request, ledger R-5: reproduces]** |
| snapshot parse | … naming a file holding `not json` | 1 | 0 | ``doc-code probe: cannot parse the snapshot `…`: expected ident at line 1 column 2`` | matches **[RECON: demoted — the request on stdin is not stated; re-driven with a stated request, ledger R-5: reproduces]** |
| snapshot parse | … an unknown `root_kind` | 1 | 0 | ``… unknown variant `sideways`, expected `staged-index` or `working-tree` …`` | matches **[RECON: demoted — the request on stdin is not stated; re-driven with a stated request, ledger R-5: reproduces]** |
| control | valid request, empty anchors | 0 | `{"findings":[],"schema_version":3}` | empty | matches **[RECON: demoted — the request on stdin is not stated; re-driven with a stated request, ledger R-5: reproduces]** |
| datum | the same with `"schema_version": 99` in the request | 0 | same response | empty | n/a — the request's version is not compared (the versioning policy is declared deferred, `design/validation.md` → Scope) **[RECON: demoted — the request on stdin is not stated; re-driven with a stated request, ledger R-5: reproduces]** |
| response serialize | — | — | — | — | **NOT DRIVEN** (no input reaches it) |

### Cell 6 — the stderr bound (`doc-code-flood`: 4096 × `a`, then `PAST-THE-BOUND`, then 2 MB)

| door | exit | wall | message | verdict |
|---|---|---|---|---|
| V | 1 | 0.27 s | 4174 chars = the 78-char prefix + exactly 4096 × `a`; `PAST` absent | matches |
| TV · TF · ST | 3 · 3 · 0 | 0.45 · 0.54 · 0.45 s | same, one finding each | matches |
| MF | 3 | (not timed) | 4174 chars | matches |
| HK | 0 (commit lands) | — | record `["pack-probe-integrity.probe-failure"]`; hook silent | matches |

### Cell 7 — install shapes (override unset; rig P1 for V/TV/TF/ST, P2 for MF)

| shape | door × argv | exit | code | verdict |
|---|---|---|---|---|
| symlink (`<tmp>/fx.*/link/jigc`) | V · TV · TF · ST · MF | 0 · 3 · 3 · 0 · 3 | `doc-code.symbol-exists`, no probe-failure | matches |
| resolved from `PATH` (bare `jigc`, the install directory first on `PATH`) | V · TV · TF · ST · MF | 0 · 3 · 3 · 0 · 3 | same | matches |
| relocated copy (`<tmp>/fx.*/copy/jigc`, byte-identical) | V · TV · TF · ST · MF | 0 · 3 · 3 · 0 · 3 | same | matches |
| copy beside a stale `doc-code` answering "no findings" | V · TV · TF · ST · MF | 0 · 3 · 3 · 0 · 3 | same; **the sibling's marker file was never written** — it is not consulted | matches |
| **the running image removed mid-run, macOS** (a copy started, then `mv`-ed away after 5–80 ms; 12 attempts) | V | 1 in 9 attempts · 0 in 3 (the `mv` lost the race) | `pack-probe-integrity.probe-failure`: ``probe `doc-code` could not start `<tmp>/fx.*/race/jigc`: No such file …`` — names `jigc`'s own image | matches (the could-not-start of the self-exec default; blocking, never a silent pass) **[RECON: demoted — the procedure is not stated as commands; re-driven, ledger R-6: reproduces (9 of 12)]** |
| **the Linux replaced-binary arm** — see below | V · TV | 0 · 3 | `doc-code.symbol-exists` | matches — driven on the registry rc.24 **Linux** build, not the host binary |
| any shape × HK | — | — | — | **NOT DRIVEN** — the hook pins the absolute path of the `jigc` that ran `setup`; which binary that is belongs to row 1 |

**The Linux `/proc/self/exe` arm — driven, with its bound stated.** `dev/runner-faithful` does **not** reach
it for this row: it runs one `cargo` command on a clone of the committed tree, so it can run the fence
(`probe_self_image.rs`, a debug build of the tree) and nothing else — a fence row, no coverage, and not the
published binary. What does reach it is the trial-harness image another driver of this run had already
built, `jigc-gate:registry-1.0.0-rc.24` (`completions/trial-harness/build-image.sh --registry 1.0.0-rc.24`:
`cargo install jigc --version 1.0.0-rc.24 --locked` from crates.io, Linux aarch64). `docker version`
answered. Rig P1 was shipped in as a tar on stdin (the daemon does not see the scratch root as a bind
mount), and inside the container:

```text
$ jigc --version                                   → jigc 1.0.0-rc.24      (Linux aarch64, /usr/local/bin/jigc)
L0  jigc validate --format json                    → exit 0, doc-code.symbol-exists            (control)
L1  cp $(command -v jigc) /tmp/x/jigc; exec 3</tmp/x/jigc; rm /tmp/x/jigc
    printf '#!/bin/sh\necho IMPOSTOR-RAN >&2\nexit 7\n' > /tmp/x/jigc; chmod 755 /tmp/x/jigc
    /proc/self/fd/3 validate --format json         → exit 0, doc-code.symbol-exists — the image's file is gone
                                                      for the whole run and an impostor sits at its path; the
                                                      probe still ran, the impostor never did
    /proc/self/fd/3 --format json task validate rename-the-pad   → exit 3, doc-code.symbol-exists
L2  the mid-run replacement race (copy started; after 2–40 ms `mv`-ed away and replaced by the impostor), 12 attempts
                                                   → 12 × exit 0, doc-code.symbol-exists; 0 × probe-failure
```

The same race on macOS produced *could not start* in 9 of 12 attempts (row above), so the window is real and
Linux closes it. **Bound:** this is the published `1.0.0-rc.24` built for Linux by `cargo install`, in a
container; the host's installed binary is a macOS image and cannot run this arm.

### Cell 8 — one key for a twice-probed task (`doc-code-count`: counts its spawns, refuses)

| door | spawns counted | probe-integrity findings | exit | verdict |
|---|---|---|---|---|
| V (the once-probed control) | 1 | 1 | 1 | matches |
| TV | **2** | **1** | 3 | matches |
| TF | **2** | **1** | 3 | matches |
| ST | **2** | **1** (task row) | 0 | matches |
| MF | **2** | **1** | 3 | matches |

### Extra probe cells (not in the scope's list; driven at V on rig P1)

| stub | exit | code / check | message | wall | verdict |
|---|---|---|---|---|---|
| `doc-code-garbage0` (exit 0, `not json`) | 1 | probe-failure / `malformed-output` | ``probe `doc-code` exited 0 but emitted unparseable output: expected ident at line 1 column 2`` | — | matches |
| `doc-code-empty0` (exit 0, nothing) | 1 | probe-failure / `malformed-output` | `… EOF while parsing a value at line 1 column 0` | — | matches (an empty stream is never "no findings") |
| `doc-code-signal` (`kill -9 $$`) | 1 | probe-failure / `crash` | `… exited non-zero (exit-code signal) with no usable output` | — | matches |
| `doc-code-silent1` (exit 1, silent) | 1 | probe-failure / `crash` | `… exited non-zero (exit-code 1) with no usable output` — no `; stderr:` suffix | — | matches |
| `doc-code-clean0` (exit 0, `{"findings":[]…}`) at V and at TV | 0 · 0 | none (V: `blocking_probes: []`; TV: only the changelog advisory) | — | — | n/a, datum — an override that answers clean passes the drifted anchor at exit 0, and no surface says an override adjudicated. The override is the declared trusted-probe arm; recorded, not proposed as a defect |
| `doc-code-spin` (`while :; do :; done`, one process) | 1 | probe-failure / `timeout` | ``probe `doc-code` exceeded its time budget and was killed`` | **30 s** | matches |
| `doc-code-exec47` (`exec sleep 47`, one process) | 1 | `timeout` | same | **30 s** | matches |
| `doc-code-hang47` (`sleep 47` as a child of the stub's shell) | 1 | `timeout` | same | **47 s** | **DEFECT (R2, T-1)** |
| `doc-code-hang` (`sleep 120` as a child) | 1 | `timeout` | same | **120 s** | **DEFECT (R2, T-1)** |

---

## 4 · The invocation-log cells

Rigs: **L1** = `fresh`, `jigc config set invocation-log true`, the delta **committed** before the cells
(`git add .jigc/config && git commit`) · **B** = `bare` · **C** = `chatty-hooks`, knob on and committed.
Records are read with `tail` / `wc -l` on `$REPO/.jigc/logs/invocations.jsonl` and `command grep` (the
harness `grep` reports *not found* under the gitignored `.jigc/`).

### Cell 11 — knob off, and outside a project layer

| door | argv · state | exit | observed | verdict |
|---|---|---|---|---|
| `config get` | `jigc config get invocation-log` (L1, before the set) | 0 | `invocation-log = false  (pack-default)` | matches |
| `validate` · clap · `task finalize` | `jigc validate` · `jigc bogus` · `jigc task finalize nope`, knob off | 0 · 2 · 1 | `.jigc/logs` does not exist; 0 records | matches |
| `config set` | `jigc config set invocation-log true` | 0 | *"written to `.jigc/config/`, uncommitted — commit it with your next commit"*; **its own invocation writes no record** (the knob is read once, before dispatch) | matches — datum |
| `config set` | `jigc config set invocation-log false` (knob on) | 0 | **recorded** (+1); the next `jigc validate` writes nothing; set back to `true`, the next run records again | matches — datum (the on-switch is unrecorded, the off-switch is) |
| `config set` | `jigc config set invocation-log maybe` | 1 | `blocking · config.value-rejected — … "maybe" is not a bool`; record `finding_codes: ["config.value-rejected"]` | matches |
| `validate` · `--version` · clap · `start` · `config set` · `config get` | rig B (git repo, no `jigc setup`) | 1 · 0 · 2 · 0 · 1 · 1 | the verbs' own outputs only (*"this project isn't set up — run `jigc setup` …"*); no `.jigc/` created, no log, `git status` clean | matches (nothing written and no error from the log path) |
| `validate` · `--version` · clap | a non-git `mktemp -d` | 1 · 0 · 2 | 0 files created | matches |
| `validate` | L1, cwd `$REPO/sub/deep` | 0 | +1 record at the repo root's log; no stray `.jigc` under `sub/` | matches |
| `validate` | L1, cwd a linked `git worktree` outside the repo | 0 | +1 record in the **main checkout's** log; none in the worktree | matches (`design/storage.md`: a worktree binds to the main checkout's `.jigc`) |
| `.gitignore` | `git check-ignore -v .jigc/logs/invocations.jsonl` | 0 | `.jigc/.gitignore:6:logs/` | matches |

### Cells 9 and 14 — one record, the 8 keys, `binary_version`, `output_bytes` (rig L1, knob on)

Every row: records **+1**, key list **exactly** `timestamp, argv, exit_code, duration_ms, finding_codes,
output_bytes, binary_version, error_code`, `binary_version = "1.0.0-rc.24"` (= `jigc --version`), record
`exit_code` = the process exit, and `output_bytes` = `wc -c` of the captured stdout **plus** stderr.

| door | argv | exit | stdout + stderr | `output_bytes` | `finding_codes` | `error_code` | verdict |
|---|---|---|---|---|---|---|---|
| `validate` | `jigc validate` | 0 | 125 + 0 | 125 | `[]` | null | matches |
| clap | `jigc --version` · `jigc --help` | 0 · 0 | 17 · 6605 | 17 · 6605 | `[]` | null | matches |
| `validate` | `jigc validate --format json` | 0 | 112 + 0 | 112 | `[]` | null | matches |
| clap (exit 2) | `jigc bogus-verb` · `jigc task finalize` (missing arg) · `jigc` | 2 · 2 · 2 | 0 + 114 · 0 + 135 · 0 + 6605 | 114 · 135 · 6605 | `[]` | null | matches |
| `task finalize` | `jigc task finalize no-such-task` | 1 | 0 + 200 | 200 | `["finalize.no-task"]` | null | matches |
| `doc show` | `jigc doc show adr:no-such` · `jigc --format json doc show 'adr:../../x'` | 1 · 1 | 0 + 453 · 0 + 312 | 453 · 312 | `["store.not-found"]` · `["store.malformed-slug"]` | null | matches |
| `start` | `jigc start` | 0 | 2377 + 0 | 2377 | `[]` | null | matches |
| `validate` (operational failure, no finding) | `JIGC_DOC_CODE_PROBE=<tmp>/fx.*/no-such-probe jigc validate` | 1 | 0 + 289 | 289 | `[]` | null | matches — the anonymous exit 1 (the registry is closed at the commit doors + the review hold) |
| `task finalize` (both streams) | the doc-only finalize under the rejecting hook (cell 12) | 1 | 233 + 362 | **595** | `[]` | `finalize.commit-rejected` | matches |
| findings exits | rig P3: `validate` (exit 0, `["doc-code.symbol-exists"]`, 798) · `task validate` (exit 3, 1663) · `task finalize` (exit 3, 842) · `milestone finalize` on P2 (exit 3, 842) | — | — | = stdout bytes | as surfaced | null | matches **[RECON: the `task validate` and `task finalize` members demoted — rig P3 as §2 builds it has no task and an unstaged drift; re-driven on a rig with the task stated, ledger R-2c / R-5: reproduces]** |
| clap (non-UTF-8 argv) | `jigc <2 invalid bytes>-not-utf8` · `jigc start --workflow single-task <intent with invalid bytes>` | 2 · 2 | 0 + 119 · 0 + 134 | 119 · 134 | `[]` | null | matches — no panic; `argv` recorded lossily (U+FFFD) |
| `start` (orientation, findings in the envelope) | rig P3: `jigc --format json start` while the task row carries `doc-code.symbol-exists` / `pack-probe-integrity.probe-failure` | 0 | 4597 / 4495 | = | **`[]`** | null | **DEFECT (R2, L-1)** **[RECON: demoted — rig P3 as §2 builds it has no task and an unstaged drift; re-driven on a rig with the task stated, ledger R-2c / R-5: reproduces]** |
| every stderr-only row above | — | — | — | = stderr bytes | — | — | **DEFECT (R2, L-2)** — a design-doc sentence, not the binary |

`jigc --help | head -1` (the reader closes early): status 0, +1 record, `output_bytes: 6605`. `jigc
validate >&-` (stdout closed): exit 0, +1 record. 20 concurrent `jigc validate`: +20 records, every line
valid JSON with 8 keys.

Rig C (`chatty-hooks`), `jigc task finalize add-a-note` with a staged file: exit 0, the commit lands,
stdout 543 bytes ending `--- hook output ---` / `chatty-hook: pre-commit spoke on success`, stderr 0;
record `exit_code: 0, finding_codes: ["changelog-recording.gate-granted-unused"], output_bytes: 543,
error_code: null` — the relayed hook stream is inside the count. **matches.**

### Cell 10 — a probe spawn adds zero records (rig P3 for the CLI doors and HK; P2 with the knob on for MF)

| door | arm | spawns (counting stub) | records added | the one record | verdict |
|---|---|---|---|---|---|
| V | control · counting stub | — · 1 | +1 · +1 | `["validate","--format","json"]`, exit 0 `["doc-code.symbol-exists"]` · exit 1 `["pack-probe-integrity.probe-failure"]` | matches |
| TV | control · counting stub | — · 2 | +1 · +1 | exit 3 | matches **[RECON: demoted — rig P3 as §2 builds it has no task and an unstaged drift; re-driven on a rig with the task stated, ledger R-2c / R-5: reproduces]** |
| TF | control · counting stub | — · 2 | +1 · +1 | exit 3 | matches **[RECON: demoted — rig P3 as §2 builds it has no task and an unstaged drift; re-driven on a rig with the task stated, ledger R-2c / R-5: reproduces]** |
| ST | control · counting stub | — · 2 | +1 · +1 | exit 0 | matches **[RECON: demoted — rig P3 as §2 builds it has no task and an unstaged drift; re-driven on a rig with the task stated, ledger R-2c / R-5: reproduces]** |
| MF | control · counting stub | — · 2 | +1 · +1 | exit 3 | matches |
| HK | six arms (cells 1, 2a, 2b, 2c, 3, 6) | ≥ 1 each | **+1 each** | `argv: ["validate","--format","json"]` — the hook's own sweep, never a `__probe` | matches |
| PI | `jigc __probe doc-code --build 1.0.0-rc.24` (garbage on stdin) · `--build 0.0.0-skew` · bare `__probe`, each run **inside** the log-on repo | — | **+0 · +0 · +0** | — | matches |
| clap | `jigc --format json __probe doc-code --build 1.0.0-rc.24` | — | +1 (exit 2) | the only log line carrying `__probe` (`command grep -c` → 1) | matches (not a probe invocation, so it is a logged usage error) |

A committing door driven with the rejecting hook **off** adds +2 records: its own, and the `validate` the
installed pre-commit hook runs — a real second invocation, not a probe child.

### Cell 12 — each `COMMITTING_DOORS` row under a rejecting `pre-commit` hook, and the review hold

Common to all eleven: exit **1**, `HEAD` unmoved, records +1, `finding_codes: []`, the hook's line
`policy: COMMIT-REJECTED-R2-MARKER` verbatim on stderr under *"`git commit` was rejected (no commit was
made):"*, and a closing frame that states what survived and prints the re-run. Rigs: **D-A** (`vendored`,
knob on and committed) · **D-B** (`fresh`, same) · **D-C** (`fresh`, squash default) · **D-D** (`fresh`,
`jigc config set finalize.fan-out.squash false` committed **before** the milestone was created).

| `COMMITTING_DOORS` row | rig · how the door was brought to its commit | argv driven | record `error_code` | state truth asserted | verdict |
|---|---|---|---|---|---|
| `jigc task finalize` | D-B · `start --workflow single-task`, commit doc filled, `git add code.txt` | `jigc task finalize survive-the-rejection` | `finalize.commit-rejected` | *"task … is intact — nothing was committed …"*; `git status` unchanged | matches |
| `jigc task finalize (amend)` | D-B · `jigc task amend "repair the install message"`, commit doc filled, empty index | `jigc task finalize repair-the-install-message` | `finalize.amend-rejected` | *"`HEAD` is unchanged …"*; HEAD's message hash unchanged | matches |
| `jigc milestone finalize (squash: true)` | D-C · `milestone create` · `add-task` · `provision`, a staged file in the sub-task worktree | `jigc milestone finalize cache-rework` | `milestone-finalize.commit-rejected` | *"milestone:cache-rework is intact … every provisioned sub-task worktree still holds its staged code"*; the worktree still shows `A  area.txt` | matches |
| `jigc milestone finalize (squash: false)` | D-D · the same, plus the sub-task entered by its launch line (`jigc workflow sub-task --task area-low`) and its commit doc filled | `jigc milestone finalize cache-rework` | `milestone-finalize.chain-commit-rejected` | *"… HEAD is at its pre-finalize commit …"*; no stray branch, worktree intact | matches |
| `jigc rename` | D-A · the rig's committed `spec:padding` | `jigc rename spec:padding --to "Padding helper"` | `rename.commit-rejected` | *"the rename was rolled back …"*; `docs/specs/padding.md` still there, status unchanged | matches |
| `jigc migrate-corpus` | D-A · an unstamped ADR hand-written at `docs/decisions/alpha-decision.md` and committed with plain git (the one fixture the binary cannot produce: it stamps what it writes) | `jigc migrate-corpus` | `migrate-corpus.commit-rejected` | *"the migrated bytes are written and staged, and the corpus is still recorded as unmigrated"* — observed `M  docs/decisions/alpha-decision.md`, exactly as the frame says | matches |
| `jigc milestone create` | D-A | `jigc milestone create "Cache rework"` | `milestone-create.commit-rejected` | *"… nothing of milestone:cache-rework survives"*; status unchanged | matches |
| `jigc milestone add-task` | D-A · after a hook-off `milestone create` | `jigc milestone add-task cache-rework "Area low"` | `milestone-add-task.commit-rejected` | *"… milestone:cache-rework is unchanged"* | matches |
| `jigc milestone add-from-spec` | D-A · `milestone create "Padding work"`, the rig's one-criterion `spec:padding` | `jigc milestone add-from-spec padding-work spec:padding` | `milestone-add-from-spec.commit-rejected` | two `note:` lines (*0 of 1 … landed*), then the frame | matches |
| `jigc milestone discard` | D-A | `jigc milestone discard cache-rework` | `milestone-discard.commit-rejected` | *"… record is still at its pre-discard state and its workbench is untouched"* | matches |
| `jigc task discard` | D-A · the sub-task `area-low` | `jigc task discard area-low --force` | `task-discard.commit-rejected` | *"… the task's working area is intact"*; the re-run keeps `--force` | matches |

The exit-4 review hold (rig D-B; `jigc migrate docs/direction.md --as vision`, `jigc doc author vision
--from-file - --task <id>`, commit doc filled, hook **off**):

| door | argv | exit | record | verdict |
|---|---|---|---|---|
| `task finalize` | `jigc task finalize <migrating-task>` | 4 | `exit_code: 4, finding_codes: [], error_code: "migrate.review-pending"`; *"migration review required — nothing committed …"*; `HEAD` unmoved | matches |
| `task finalize` | `jigc --format json task finalize <migrating-task>` | 4 | same `error_code`; a JSON document on stdout (`retires`, `rewrites`, …) | matches |
| `task finalize` | `jigc task finalize <migrating-task> --approve` under the rejecting hook | 1 | `error_code: "finalize.commit-rejected"`; the foreign original `docs/direction.md` present with unchanged bytes, no `VISION.md` written | matches — datum: the approved migration commit shares the ordinary arm's identity; nothing lost |
| `task finalize` (M55's doc-only model) | `jigc task finalize readme-names-a-verb` (minted by `jigc start --workflow report-inconsistency …`, an `inconsistency` doc authored) under the rejecting hook | 1 | `error_code: "finalize.commit-rejected"`; stdout carried the path-scoped narration, stderr the frame | matches — datum: `DocOnly` has no row of its own in `COMMITTING_DOORS` |

**Twelve of twelve `ERROR_CODE_REGISTRY` members were read back from a record written by the release
binary.**

### Cell 13 — a log that cannot be written (rig L1; baseline `jigc validate` → exit 0, 125 bytes; `jigc task finalize no-such-task` → exit 1)

| state | argv | exit | stdout / stderr vs the baseline (`cmp`) | records | verdict |
|---|---|---|---|---|---|
| `.jigc/logs` mode 555 and the file mode 444 | `jigc validate` · `jigc task finalize no-such-task` | 0 · 1 | identical / identical (both) | +0 | matches |
| a regular **file** at `.jigc/logs` | `jigc validate` | 0 | identical / identical; the file's bytes untouched | — | matches |
| a **directory** named `invocations.jsonl` | `jigc validate` | 0 | identical / identical | — | matches |
| `.jigc/logs` absent | `jigc validate` | 0 | identical | recreated, 1 record | matches |

---

## 5 · FINDINGS — every defect, with its repro and a proposed tier

The predicate: **tier 1** = exit-0 loss or repository harm through a committing, destroying or moving door ·
**tier 2** = a posture or route dead end · **tier 3** = a surface says something the binary does not do.

**No tier-1 row. No tier-2 row.** Four tier-3 proposals.

### (R2, T-1) — the probe's 30 s wall-clock budget is not enforced when the killed probe leaves a child holding its pipes

- **Door:** every probe door (driven at V). **Cell:** timeout (extra cell).
- **Contract:** `design/validation.md` → the six rules, 5 (*"A runaway probe is killed and surfaces as a
  `timeout` failure"*) and → Scope (*"a wall-clock budget the CLI invoker enforces"*); `invoke.rs`'s own
  statement of the invoker (*"On a budget overrun the child is killed and reaped (no leaked process, no
  hang)"*; `DOC_CODE_BUDGET` = 30 s).
- **Repro:**

```text
# rig P1 (vendored, drifted). Three override stubs, mode 755:
#   doc-code-spin    #!/bin/sh · while :; do :; done        (one process)
#   doc-code-exec47  #!/bin/sh · exec sleep 47              (one process)
#   doc-code-hang47  #!/bin/sh · sleep 47                   (the shell + a child that inherits stdout/stderr)
$ time JIGC_DOC_CODE_PROBE=<tmp>/fx.*/doc-code-spin    jigc validate --format json   → exit 1, check `timeout`, wall 30 s
$ time JIGC_DOC_CODE_PROBE=<tmp>/fx.*/doc-code-exec47  jigc validate --format json   → exit 1, check `timeout`, wall 30 s
$ time JIGC_DOC_CODE_PROBE=<tmp>/fx.*/doc-code-hang47  jigc validate --format json   → exit 1, check `timeout`, wall 47 s
$ time JIGC_DOC_CODE_PROBE=<tmp>/fx.*/doc-code-hang    jigc validate --format json   → exit 1, check `timeout`, wall 120 s   (sleep 120)
# every one reports: probe `doc-code` exceeded its time budget and was killed
```

- **Read:** the direct child is killed at 30 s and the finding is right, but the door returns only when
  the grandchild exits — the invoker joins its pipe readers, and the grandchild still holds the write
  ends. A grandchild that never exits would hold the door (and, through the hook, a `git commit`) open.
- **Bound, stated:** only the **override** path reaches it, and only with a probe that itself breaks rule 6
  (*no shelling out*). The bundled self-exec probe is one process and is bounded at 30 s (not driven to a
  hang — it cannot be made to hang from outside).
- **Proposed tier: 3** — the invoker says *killed, no hang* and the binary waits past its budget; nothing
  is committed, lost or mis-reported (exit 1, blocking `timeout`).

### (R2, P-1) — an override that names no file takes two different shapes at the task and milestone doors

- **Doors:** `task validate` · `task finalize` · `milestone finalize` (and orientation's row). **Cell:** 2b.
- **Contract:** `design/validation.md` → Distribution bound (*"The pre-flight … still fires on the override
  path, when `JIGC_DOC_CODE_PROBE` names an executable that is not there — the one-operational-error shape
  is unchanged for that case"*); `design/assistant-adapter.md` (*"Case (iv) survives only on the override
  path, a `JIGC_DOC_CODE_PROBE` naming no file"*); `implementation/module-layout.md` → Probe boundary
  (*"The task-scope absence path … probe-absent reports one operational error … not a floored per-anchor
  `pack-probe-integrity.crash`"* — in the M20 bullet the M54 bullet supersedes for the shipped probe, kept
  for the override).
- **Repro:**

```text
# rig P1: the task `rename-the-pad` stages NO doc with a code anchor; its staged src/pad.ts is cited by the committed arch-doc.
$ JIGC_DOC_CODE_PROBE=<tmp>/fx.*/no-such-probe jigc --format json task validate rename-the-pad
exit=3   stdout: findings envelope →
  pack-probe-integrity.probe-failure · check crash · blocking · key.target doc-code
  "probe `doc-code` could not start `<tmp>/fx.*/no-such-probe`: No such file or directory (os error 2)"
$ … task finalize rename-the-pad                → exit 3, the same finding, HEAD unmoved
$ … milestone finalize pad-rework   (rig P2)    → exit 3, the same finding, HEAD unmoved
$ … validate --format json                      → exit 1, stdout empty, stderr {"error": "`doc-code` probe not found at … — `JIGC_DOC_CODE_PROBE` names no file; …"}

# then make the task stage an anchored doc, and nothing else changes:
$ printf 'The padding layer owns string-width normalization.\n' | jigc doc set-slot arch-doc:padding-layer#overview --task rename-the-pad --from-file -
$ JIGC_DOC_CODE_PROBE=<tmp>/fx.*/no-such-probe jigc --format json task validate rename-the-pad
exit=1   stdout empty   stderr {"error": "`doc-code` probe not found at … — `JIGC_DOC_CODE_PROBE` names no file; …"}
$ … task finalize rename-the-pad                → exit 1, the same error envelope, HEAD unmoved
$ … start                                       → exit 0, tasks[0].findings_unavailable = that text
```

- **Read:** the task-scope pre-flight is keyed to the task's **own** anchor surface, while the gate also
  spawns the probe for the committed anchors a staged file dangles (the blast radius). With no anchor of
  its own the pre-flight is inert and the spawn runs into `ENOENT`. Same cause, same door: exit 1 +
  `error` envelope + `finding_codes: []` in one task state, exit 3 + a `crash` finding in the other.
- **Proposed tier: 3** — the stated one-operational-error shape is not what these doors do in the
  blast-radius-only state; both shapes block, both name the program, and it is one finding, never one per
  anchor. The contract sentence is strongest for the store sweep; the reconciler may read it narrower.

### (R2, L-1) — orientation surfaces findings and its log record says `finding_codes: []`

- **Door:** bare `start`. **Cells:** 9 / 10.
- **Contract:** `design/measurement.md` → The in-repo invocation log (*"every key is present on every
  record, `finding_codes` being `[]` on a run that raised none"*); the scope's own hook-door note makes
  `finding_codes` the observable for a probe-integrity failure.
- **Repro:**

```text
# rig P3 (log on), a live task whose staged file dangles a committed anchor
$ jigc --format json start                                   → exit 0; tasks[0].findings carries doc-code.symbol-exists (blocking)
record: {"argv":["--format","json","start"],"exit_code":0,"finding_codes":[], …,"error_code":null}
$ JIGC_DOC_CODE_PROBE=<tmp>/fx.*/doc-code-count jigc --format json start
                                                             → exit 0; tasks[0].findings carries pack-probe-integrity.probe-failure
record: {"argv":["--format","json","start"],"exit_code":0,"finding_codes":[], …}
```

- **Read:** of the six probe doors, orientation is the one where a probe that did not run leaves no trace
  in the log: exit 0, no code. `invocation_log.rs` scopes the field to *"the report-bearing verbs (validate
  / finalize)"*, which is narrower than the design sentence.
- **Proposed tier: 3** — the design doc's rule for `finding_codes` is not what this door records. Arguable:
  if the code-side scoping is the contract, this is a datum, not a defect.

### (R2, L-2) — `design/measurement.md` says `output_bytes` is the emitted **stdout** size; the binary records stdout **plus** stderr

- **Door:** every logged invocation (driven at `validate`, `task finalize`, `doc show`, clap).
  **Cell:** 14.
- **Contract:** `design/measurement.md` → The in-repo invocation log: *"the **`output_bytes`** field (M39)
  is the invocation's emitted stdout size, captured by an fd-level tee at the dispatch boundary"*. The
  scope's cell 14, `main.rs` and `invocation_log.rs` all say both streams.
- **Repro:**

```text
# rig L1 (log on)
$ jigc bogus-verb                       → exit 2; stdout 0 bytes, stderr 114 bytes;  record output_bytes: 114
$ jigc task finalize no-such-task       → exit 1; stdout 0, stderr 200;              record output_bytes: 200
$ jigc task finalize readme-names-a-verb   (rejecting hook) → stdout 233, stderr 362; record output_bytes: 595
```

- **Proposed tier: 3** — a design-doc sentence the binary does not do; the binary is self-consistent and
  matches its own module doc. A doc edit, not a code change.

---

## 6 · Observations that are not defects (recorded so a reconciler does not re-derive them)

1. `jigc config set invocation-log true` writes **no record of itself**; `… false` does. The knob is read
   once, before dispatch.
2. The override path's *probe not found* reaches the log as an **anonymous exit 1** (`finding_codes: []`,
   `error_code: null`) at `validate` and at the hook. The registry is closed at the committing doors plus
   the review hold, so this is as declared.
3. An override that answers `{"findings":[]}` passes a drifted anchor at exit 0 at `validate` and `task
   validate`; no surface says the adjudicator was an override. Declared: the override is a trusted
   standalone probe. `task finalize` was **not** driven under it (it would have committed in rig P1).
4. The probe does not compare the request's `schema_version` (99 is accepted). Declared deferred.
5. `DocOnly` (M55) and the approved migration commit both log `finalize.commit-rejected`; `COMMITTING_DOORS`
   has 11 rows and neither has its own.
6. A rejected `task finalize` prints no advisory and records `finding_codes: []`; a landed one records the
   advisory it printed (`changelog-recording.gate-granted-unused`). Consistent each way.
7. `argv` in a record carries the intent text and `--value` content verbatim (declared, `measurement.md`
   item 6) and lossy U+FFFD for non-UTF-8 arguments.

**Leads outside this row's subject, seen in passing and not driven further — for the rows that own them:**

- *Row 5 (finalize / the left-out narration):* in rig D-B the doc-only finalize printed `left-out (… a
  staged path stays staged for the task it belongs to): code.txt` while `git status` showed `?? code.txt`
  — the file was **untracked**, not staged. One observation, not re-driven.
- *Row 8 (composed surfaces):* in rig D-D, `jigc start --task area-low` run inside the sub-task's worktree
  exited 0 and printed the sub-task's text but did not provision its commit doc; the next `jigc doc
  set-field commit:area-low#type …` refused with *"enter it with `jigc workflow sub-task --task
  area-low`"*, and that launch line did provision it. One observation, not re-driven.

---

## 7 · NOT DRIVEN — every (door, cell) pair left un-driven, and why

| # | pair(s) | count | reason |
|---|---|---|---|
| 1 | HK × cell 7 (four install shapes) | 4 | the hook pins the absolute path of the `jigc` that ran `setup`; which binary the hook runs is row 1's subject |
| 2 | PI × cell 5, the *response serialize* arm | 1 | no input reaches it from outside the process |
| 3 | the `timeout` arm × TV, TF, MF, ST, HK | 5 | driven at V only (four stubs); each further door costs 30–60 s and the invoker is the one shared function |
| 4 | `malformed-output` (×2), `signal`, `silent exit 1` × TV, TF, MF, ST, HK | 20 | driven at V only; the classification is engine-side and shared |
| 5 | the Linux replaced-binary arm × TF, MF, ST, HK | 4 | driven at V and TV in the registry Linux image; the rest not repeated there |
| 6 | the Linux arm **through `dev/runner-faithful`** | 1 | it runs a `cargo` command on the committed tree — a debug-build fence, never the published binary; no coverage, so not run |
| 7 | cell 12, a **partially landed** `milestone add-from-spec` (a hook that passes the first commit) | 1 | the rig's spec has one criterion; the multi-criterion fixture was not built |
| 8 | cell 12, the chain arm's rejected per-sub-task commit **vs** rejected aggregate | 1 | declared bound in `COMMITTING_DOORS`' own doc: one code names the arm, not the commit |
| 9 | cell 13 × a committing door (the log unwritable while a door commits) | 1 | driven at `validate` and a failing `task finalize` only |
| 10 | cell 11, the knob set at the **team** layer | 1 | no team layer was built |
| 11 | the log silently disabled by a drifted frozen schema (`enabled_logs_dir`'s `make_pack().ok()?`) | 1 | needs a project-layer schema shadow; this row's scope grants no hand-written `.jigc/config/` file |
| 12 | *could not start* with **no** program named (the running image's own path unreadable) | 1 | no way to make `current_exe()` fail from outside |
| 13 | `task finalize` under an override that answers clean | — | not a cell; deliberately not driven (it would commit) — see §6.3 |

**41 pairs not driven.** No un-driven pair is presented above as driven. The Linux arm's two driven rows
ran on the registry `1.0.0-rc.24` **Linux** build in a container, and are marked so wherever they appear.

---

## 8 · Baseline rows: CLOSED / STILL-OPEN

**None — this row has no baseline (first drive).**

The one carried item, from M54's *Not run*: **the Linux replaced-binary-while-running arm
(`/proc/self/exe`)** — the scope asked for *NOT DRIVEN (needs Linux; Docker cell)* unless Docker answered.
It answered, and the arm was **DRIVEN** (§3, cell 7) on the published rc.24 as `cargo install` builds it for
Linux: the image file removed before the run and replaced by a failing impostor → the probe still ran
(`doc-code.symbol-exists` at `validate`, exit 0, and at `task validate`, exit 3); 12 of 12 mid-run
replacements likewise. `dev/runner-faithful` does not give a way to reach it for a driven row (§7, item 6).

---

## 9 · Counts

- **Rows driven: 174** — probe cells 103 (cell 1: 6 · 2a: 6 · 2b: 9 · 2c: 6 · 3: 7 · 4: 12 · 5: 11 · 6: 6 ·
  7: 25 · 8: 5 · extra stubs: 10) and log cells 71 (9 / 14: 15 · 10: 14 · 11: 18 · 12: 16 · 13: 8, the
  three closed-stream / concurrency drives counted under 13). A drive that serves two cells (the hook arms
  under cell 10, the findings-exit records under 9 / 14) is counted once.
- **Rows not driven: 41** (§7).
- **Defects: 4**, all proposed tier 3 — `(R2, T-1)`, `(R2, P-1)`, `(R2, L-1)`, `(R2, L-2)`.
- **Tier-1 rows: 0. Tier-2 rows: 0.**
- **Clap leaves that are the door of ≥ 1 driven row (`VERB_KINDS` spelling):** `start` · `validate` ·
  `task validate` · `task finalize` · `task discard` · `milestone finalize` · `milestone create` ·
  `milestone add-task` · `milestone add-from-spec` · `milestone discard` · `rename` · `migrate-corpus` ·
  `config set` · `config get` · `doc show` — **15**. The `__probe` intercept and the pre-commit hook are
  doors of driven rows and are not clap leaves. Leaves used only to build a fixture (`workflow`, `migrate`,
  `task amend`, `milestone provision`, `milestone execute`, `doc create`, `doc set-field`, `doc set-slot`,
  `doc add-item`, `doc author`) carry no verdict here and are **not** claimed as covered.

---

# Reconciliation ledger — row 2 (rc.24)

The rule applied (`completions/artifacts/M51/acceptance-design.md` → The reconciliation rule): *a claim by
one that the other cannot reproduce is a lead, not a finding.* Every Codex claim below was entered as
`lead(codex, …)` and then driven; every driver defect was re-driven once by the reconciler. Nothing here was
promoted on a source read, and nothing was dropped on silence.

## R-0 · What the reconciler ran on

- **Binary:** `~/.local/bin/jigc`; `jigc --version` → `jigc 1.0.0-rc.24` (asserted first, exit 0).
- **Environment (set / unset only):** `JIGC_DOC_CODE_PROBE` unset · `JIGC_PACK_DIR` unset · `GIT_DIR` unset ·
  `CLAUDECODE` set in the session, and **cleared or set per cell** (`env -u CLAUDECODE …` /
  `CLAUDECODE=1 …`) wherever the cell's subject is who made the commit.
- **Rigs (all `dev/jigc-rig <state> --binary ~/.local/bin/jigc`, two-step eval, stdout only, `[ -n "$REPO" ]`
  guard, every root from `mktemp -d`, no teardown):** `fresh` ×2 (**C**, **D**) · `vendored` ×2 (**V**, **V2**)
  · `bare` ×2. Non-rig fixtures under `<tmp>/fx` and `<tmp>/hooks` (both `mktemp -d`): the rejecting hook,
  the override stubs, the relocated binary copies.
- **The working repository** was read for the registries at `bffa6667` and left untouched: `HEAD` and
  `git status --short` (0 lines) were read again after the last drive.
- **The source pass:** `codex/axis2-codex.md` exists and its run recorded exit 0. A second, higher-effort
  Codex pass over the same prompt ended on the provider's usage limit (exit 1) and wrote **no** claims
  file; nothing from it is reconciled here.

## R-1 · Codex claims

| # | `lead(codex, …)` | status | datum |
|---|---|---|---|
| C-1 | *a rejected agent-authored commit permanently signs its message file, so a later human retry lands a false `Co-Authored-By` trailer at exit 0* (proposed tier 1, confidence high) | **REFUTED** | driven through 13 commit constructions — all 11 `COMMITTING_DOORS` rows, the doc-only finalize and the approved migration commit: agent run under a rejecting hook → exit 1, `HEAD` unmoved; the human retry → exit 0 and **no trailer on any landed commit**. Repro R-1a |
| K-1 | M54 code-review finding 3 remains fixed: the five probe-local failure arms emit one reason line; a spawn failure carries the program; non-zero stderr is bounded | **CONFIRMED clean** (4 of 5 arms) · **OPEN LEAD** (the *response serialize* arm) | R-5 (four arms, one line each, stdout empty, exit 1) · R-6 (*could not start* names the program; the message is 78 + 4096 chars). No input reaches the serialize arm from outside the process, so its line was never read |
| K-2 | the Linux `/proc/self/exe` arm is intact in source; *"this source pass did not execute it"* | **CONFIRMED clean** by driving | R-7: on the published `1.0.0-rc.24` Linux build, image file removed and an impostor at its path → the probe ran, the impostor did not; 12 of 12 mid-run replacements likewise |
| K-3 | `probe_intercept` runs before route fencing, clap, teeing and logging; classification is first-word-only | **CONFIRMED clean** | R-5: every `jigc __probe …` invocation inside a log-on repo added **0** records; `jigc --format json __probe …` is a clap exit 2 and the one log line carrying `__probe` (`command grep -c` → 1, before-control `"validate"` → 16) |
| K-4 | the override treats empty as unset and non-empty as the standalone program | **CONFIRMED clean** | R-6: `JIGC_DOC_CODE_PROBE=` → the control's result (exit 0, `doc-code.symbol-exists`); a non-empty value is spawned (`Permission denied`, the counting stub's count) |
| K-5 | three production invoker consumers; bare orientation reaches the task sweep; *no fourth production spawn was found* | positive half **CONFIRMED clean** · negative half **OPEN LEAD** | spawn counts read off the counting stub: store sweep 1, orientation 2 (R-2c); the driver's cell 8 has `task validate` / `task finalize` / `milestone finalize` at 2. *No fourth spawn* is an absence claim over source; driving cannot establish it and it is not promoted |
| K-6 | the invocation log is one best-effort post-dispatch append, written only under a project layer, append errors discarded, eight keys | **CONFIRMED clean** | R-2c (key list read off an emitted record, in the declared order) · R-6 (read-only log: exit and both streams byte-identical, +0 records) · R-8 (bare rig: six invocations, no `.jigc/` created, `git status` clean) |
| K-7a | `COMMITTING_DOORS` has 11 rows; `ERROR_CODE_REGISTRY` their 11 identities plus `migrate.review-pending` | **CONFIRMED clean** | read: 11 rows over 9 verbs, 12 members (R-4). Read back from records written by the release binary in the reconciler's own drives: 10 of 12 (R-1a, R-3); the other two (`finalize.amend-rejected`, `milestone-finalize.commit-rejected`) were driven by the reconciler in a rig with the log off and are the driver's cell 12 rows |
| K-7b | *no release-reachable `Outcome::error` / `coded_error` identity outside the registry, and no production hook-capable commit bypass* | **OPEN LEAD** | an absence claim over source. Every record read in this row carries `error_code: null` or a registry member, which is consistent with it and does not establish it |
| K-7c | `setup`'s `--no-verify` commit signs separately | **CONFIRMED clean** | R-8: `CLAUDECODE=1 jigc setup` → the install commit carries the trailer; `env -u CLAUDECODE jigc setup` → it carries none |
| K-8 | the named fences cover the probe and log cells and do **not** cover the rejected-agent → human-retry transition | **NOT A BEHAVIOUR CLAIM — not driven** | a statement about tests; a fence row confers no coverage. The transition itself is now driven (C-1) |
| K-9 | no schema-hash-boundary violation; the rc.24 `planning-record` edit changes only an erased `hint` | **OPEN LEAD** (outside this row's cells) | not driven here. Indirect only: every invocation in six rigs passed pack-load, whose freeze assertion blocks loudly on an un-migrated shape |

**Codex-origin findings confirmed: 0.** The one claim Codex raised as a defect is refuted; its clean reads
agree with the driven rows wherever a row exists.

### R-1a · C-1, driven (REFUTED)

```text
# the rejecting hook, behind core.hooksPath in a mktemp -d:
#   <tmp>/hooks/pre-commit:  #!/bin/sh · echo "policy: COMMIT-REJECTED-R2-MARKER" 1>&2 · exit 1
# the transition, identical at every door:
#   git config core.hooksPath <tmp>/hooks ;  CLAUDECODE=1 jigc <door argv>          # the agent's run
#   git config --unset core.hooksPath     ;  env -u CLAUDECODE jigc <door argv>     # the human's retry
#   git log -1 --format='%s · trailers: [%(trailers:separator=;)]'

# rig C — fresh. Fixture commands all run under `env -u CLAUDECODE`.
# positive control (the trailer does land when the agent's commit lands):
$ CLAUDECODE=1 jigc task finalize agent-control           → exit 0   feat: agent control · trailers: [Co-Authored-By: Claude <noreply@anthropic.com>]

# 1 · jigc task finalize            (start --workflow single-task "survive the rejection"; commit doc filled; git add code.txt)
$ CLAUDECODE=1 jigc task finalize survive-the-rejection   → exit 1   `git commit` was rejected (no commit was made): / policy: COMMIT-REJECTED-R2-MARKER   HEAD unmoved
$ env -u CLAUDECODE jigc task finalize survive-the-rejection → exit 0   feat: survive the rejection · trailers: []
#     second task, same door, the bytes looked for after the rejection:
$ command grep -rl "noreply@anthropic.com" "$REPO" --exclude-dir=objects
    $REPO/.git/COMMIT_EDITMSG          # git's own file, left by the control commit that LANDED — the before-control: the grep finds the bytes
                                       # nothing under $REPO/.jigc/ — no signed message file survives the rejection
$ env -u CLAUDECODE jigc task finalize second-rejection   → exit 0   fix: second rejection · trailers: []

# 2 · jigc task finalize (amend)    (jigc task amend "repair the install message"; commit doc filled; HEAD carries no trailer)
  agent + hook → exit 1, HEAD unmoved      · human retry → exit 0   fix: repair the install message · trailers: []
# 3 · the doc-only finalize (M55)   (start --workflow report-inconsistency …; an `inconsistency` doc created and authored)
  agent + hook → exit 1 (stdout 0, stderr 362) · human retry → exit 0   docs: file the readme inconsistency · trailers: []   (1 file: docs/inconsistencies/readme-names-a-verb.md)
# 4 · jigc milestone finalize (squash: true)   (milestone create · add-task · provision; a staged file in the sub-task worktree)
  agent + hook → exit 1, HEAD unmoved      · human retry → exit 0   Finalize milestone cache-rework (1 sub-task) · trailers: []

# rig D — fresh, invocation log on, `jigc config set finalize.fan-out.squash false` committed before the milestone
# 5 · jigc milestone finalize (squash: false)  (… plus `jigc workflow sub-task --task area-low` in its worktree, commit doc filled, `milestone join`)
  agent + hook → exit 1, record error_code "milestone-finalize.chain-commit-rejected", HEAD unmoved
  $ command grep -rl "noreply@anthropic.com" "$REPO" --exclude-dir=objects   → (none)
  human retry → exit 0, two commits:   feat: area low · trailers: []      Finalize milestone cache-rework (1 sub-task) · trailers: []
# 6 · the approved migration commit  (jigc migrate docs/direction.md --as vision; doc author vision; commit doc filled)
  $ CLAUDECODE=1 jigc task finalize <migrating-task> --approve   → exit 1, record error_code "finalize.commit-rejected"; docs/direction.md present, no VISION.md
  $ env -u CLAUDECODE jigc task finalize <migrating-task> --approve → exit 0   docs(vision): migrate the direction note · trailers: []

# rig V — vendored, invocation log on (the record's error_code read after each agent run)
# positive control at a record door:
$ CLAUDECODE=1 jigc milestone create "Agent control"      → exit 0   chore(milestone): open record for milestone:agent-control · trailers: [Co-Authored-By: Claude <noreply@anthropic.com>]
# 7  · jigc milestone create "Cache rework"                 → exit 1 (milestone-create.commit-rejected)        · retry exit 0 · trailers: []
# 8  · jigc milestone add-task cache-rework "Area low"      → exit 1 (milestone-add-task.commit-rejected)      · retry exit 0 · trailers: []
# 9  · jigc milestone add-from-spec padding-work spec:padding → exit 1 (milestone-add-from-spec.commit-rejected) · retry exit 0 · trailers: []
# 10 · jigc task discard area-low --force                   → exit 1 (task-discard.commit-rejected)            · retry exit 0 · trailers: []
# 11 · jigc milestone discard cache-rework                  → exit 1 (milestone-discard.commit-rejected)       · retry exit 0 · trailers: []

# rig V2 — vendored, clean tree, invocation log on
# 12 · jigc rename spec:padding --to "Padding helper"       → exit 1 (rename.commit-rejected), status clean    · retry exit 0 · trailers: []
# 13 · jigc migrate-corpus  (an unstamped ADR committed with plain git at docs/decisions/alpha-decision.md)
                                                            → exit 1 (migrate-corpus.commit-rejected), `M  docs/decisions/alpha-decision.md`
                                                            · retry exit 0 · chore(jigc): migrate the managed corpus to the current schema versions · trailers: []
```

**The falsifying datum:** 13 of 13 human retries landed at exit 0 with an empty trailer block, beside two
positive controls in which the same doors, run by the agent to a landed commit, did carry the trailer. The
claim's reproduction was followed as proposed (its own three commands at `task finalize`) and then widened
to every other commit construction; it reproduced at none. *Aside, a source read and labelled as one:* each
door writes its message file from the commit doc or its structural subject immediately before the seam on
every run, so a file signed by a rejected run is overwritten by the retry — the claim's step *"the
already-signed file is passed unchanged"* is the one that does not happen. Tier is moot.

## R-2 · Driver defects, re-driven once each

All four reproduce. **None is tier 1 and none is tier 2**; the tier-1 test was applied in both directions
(below, R-2e).

| key | origin | status | tier | door(s) | why this tier |
|---|---|---|---|---|---|
| `(R2, T-1)` | driver | **CONFIRMED** (R-2a) | **3** | `validate` (the one door driven; the invoker is shared by all six) | the invoker says *killed, no hang* and the door outlives its 30 s budget; exit is 1 and the finding is the right blocking `timeout`. Not tier 1: exit non-zero, nothing committed or lost. Not tier 2: the door does return and its route is intact |
| `(R2, P-1)` | driver | **CONFIRMED** (R-2b) | **3** | `task validate` · `task finalize` (re-driven) · `start` (re-driven) · `milestone finalize` (the driver's row, not re-driven) | the documented one-operational-error shape is not what these doors do in the blast-radius-only state. Not tier 1: exit 3 or 1, `HEAD` unmoved, nothing lost. Not tier 2: both shapes block and both name the program and the fix |
| `(R2, L-1)` | driver | **CONFIRMED** (R-2c) | **3** | `start` | `design/measurement.md` says `finding_codes` is `[]` *on a run that raised none*; orientation raises findings and records `[]`. Exit 0 is present, but `start` is not a committing, destroying or moving door and nothing is lost — the findings are in the envelope the invocation printed; only the log record omits them |
| `(R2, L-2)` | driver | **CONFIRMED** (R-2d) | **3** | every logged invocation (re-driven at a clap usage error, `task finalize`, `validate`) | a design-doc sentence (*"emitted stdout size"*) the binary does not do; the binary counts both streams and agrees with its own module doc. No loss, no route |

The source pass contradicts none of the four: it is silent on all of them (its clean read of the log and of
the invoker does not reach the timeout join, the task-scope pre-flight, orientation's `Outcome`, or the
`output_bytes` sentence). Silence is not refutation; they stand as driven.

### R-2a · `(R2, T-1)`

```text
# rig V (vendored; src/pad.ts rewritten so the anchored symbol `pad` is gone). Override stubs, mode 755:
#   <tmp>/fx/doc-code-spin    #!/bin/sh · while :; do :; done        (one process)
#   <tmp>/fx/doc-code-hang47  #!/bin/sh · sleep 47                   (the shell + a child holding its stdout/stderr)
$ JIGC_DOC_CODE_PROBE=<tmp>/fx/doc-code-spin    jigc validate --format json   → exit 1, wall 30.2 s
$ JIGC_DOC_CODE_PROBE=<tmp>/fx/doc-code-hang47  jigc validate --format json   → exit 1, wall 47.2 s
# both: pack-probe-integrity.probe-failure · check timeout · blocking
#       "probe `doc-code` exceeded its time budget and was killed"
# no stub process left running afterwards
```

Bound, as the driver stated it and re-read here: only the **override** path, and only a probe that itself
spawns a child. The driver's *"a grandchild that never exits would hold the door open"* was driven to 120 s,
never to an unbounded child — it is entered as an open lead (R-7), not as part of the finding.

### R-2b · `(R2, P-1)`

```text
# rig V. Task `rename-the-pad` (start --workflow single-task "rename the pad"; commit doc filled;
# git add src/pad.ts) stages NO doc carrying a code anchor; its staged file is cited by the committed arch-doc.
$ JIGC_DOC_CODE_PROBE=<tmp>/fx/no-such-probe jigc validate --format json
exit=1  stdout 0 bytes  stderr {"error": "`doc-code` probe not found at \"<tmp>/fx/no-such-probe\" — `JIGC_DOC_CODE_PROBE` names no file; point it at a probe executable, or unset it to run the probe built into `jigc`"}
$ JIGC_DOC_CODE_PROBE=<tmp>/fx/no-such-probe jigc --format json task validate rename-the-pad
exit=3  stderr 0 bytes  findings: pack-probe-integrity.probe-failure · crash · blocking · key.target doc-code
        "probe `doc-code` could not start `<tmp>/fx/no-such-probe`: No such file or directory (os error 2)"
        record: exit_code 3, finding_codes ["pack-probe-integrity.probe-failure","changelog-recording.gate-granted-unused"]
$ … task finalize rename-the-pad      → exit 3, the same finding, HEAD unmoved
$ … --format json start               → exit 0, tasks[0].findings carries the probe-failure, findings_unavailable null

# the task now stages an anchored doc; nothing else changes:
$ printf 'The padding layer owns string-width normalization.\n' | jigc doc set-slot 'arch-doc:padding-layer#overview' --task rename-the-pad --from-file -
$ JIGC_DOC_CODE_PROBE=<tmp>/fx/no-such-probe jigc --format json task validate rename-the-pad
exit=1  stdout 0 bytes  stderr {"error": "`doc-code` probe not found at … — `JIGC_DOC_CODE_PROBE` names no file; …"}
        record: exit_code 1, finding_codes [], error_code null
$ … task finalize rename-the-pad      → exit 1, the same error envelope, HEAD unmoved
$ … --format json start               → exit 0, tasks[0].findings [], findings_unavailable = that text
```

### R-2c · `(R2, L-1)`

```text
# rig V: `jigc config set invocation-log true`, the delta committed; then the drift and the task of R-2b's first state.
$ jigc --format json start
exit=0  stdout 4541  stderr 0   tasks[0].findings: doc-code.symbol-exists (blocking), changelog-recording.gate-granted-unused (advisory)
record keys, in order: timestamp, argv, exit_code, duration_ms, finding_codes, output_bytes, binary_version, error_code
record: {"argv":["--format","json","start"],"exit_code":0,"finding_codes":[],"output_bytes":4541,"binary_version":"1.0.0-rc.24","error_code":null}
$ JIGC_DOC_CODE_PROBE=<tmp>/fx/doc-code-count jigc --format json start         # the stub counts its spawns, writes one line on stderr, exits 1
exit=0  spawns 2  records +1   tasks[0].findings: pack-probe-integrity.probe-failure (blocking), …
record: {"argv":["--format","json","start"],"exit_code":0,"finding_codes":[], … ,"error_code":null}
# contrast, same repo, same finding:
$ jigc validate --format json                         → exit 0  record finding_codes ["doc-code.symbol-exists"]
$ jigc --format json task validate rename-the-pad     → exit 3  record finding_codes ["doc-code.symbol-exists","changelog-recording.gate-granted-unused"]
```

The driver's own caveat is kept: `invocation_log.rs` scopes the field to *"the report-bearing verbs
(validate / finalize)"*. The design sentence is the wider one and is what a log reader is told, so the row
stays a tier-3 finding rather than a datum; which text gives way is a triage decision.

### R-2d · `(R2, L-2)`

```text
# rig V (log on)
$ jigc bogus-verb                   → exit 2   stdout 0   stderr 114   record output_bytes 114
$ jigc task finalize no-such-task   → exit 1   stdout 0   stderr 200   record output_bytes 200, finding_codes ["finalize.no-task"]
$ jigc validate                     → exit 0   stdout 549 stderr 0     record output_bytes 549
# design/measurement.md → The in-repo invocation log: "the `output_bytes` field (M39) is the invocation's emitted stdout size"
```

### R-2e · The tier-1 test, both directions

- **Downward** (is a tier-1 claim missing a half?): the only tier-1 claim on this row was Codex's C-1. It
  had neither half once driven — the exit-0 commit exists, the false trailer does not.
- **Upward** (does a lower-tiered row show both halves?): no. T-1 and P-1 exit non-zero. L-1 exits 0 at a
  door that commits, destroys and moves nothing. L-2 is a sentence. Among the driver's *not defects*: an
  override that answers `{"findings":[]}` passes a drifted anchor at exit 0 at `validate` and `task
  validate` — the exit-0 half is there, but neither run went through a committing door (`task finalize`
  under it was deliberately not driven by the driver and was not driven here), no bytes were lost, and the
  override is the operator's own declared adjudicator. Not tier 1 on the evidence; not promoted.

## R-3 · Rows marked driven that carry no repro — demotions

**The standard applied.** The instrument's rule is that a row is driven iff its argv ran on the binary and
its verdict was recorded with a repro. In the driver's record only the four defects carry a fenced repro
block; every other row's repro is its table row (argv, exit, asserted surface) on a fixture §2 constructs. A
row is held to carry its repro when those two together are enough to run it again. **17 rows fail that
test** and are marked `[RECON: demoted …]` in the table above. The reconciler then drove each of the 17 with
a stated construction; all 17 reproduce and are restored as driven **on the reconciler's repro**, so the
row count and the leaf coverage are unchanged.

| rows | n | what the driver's record does not state | reconciler's repro | result |
|---|---|---|---|---|
| cell 5 — snapshot read ×3, snapshot parse ×2, control, datum | 7 | the request on stdin (the rows say *"valid request naming …"*) | R-5 | all 7 reproduce, byte-for-byte on the reason lines |
| cell 7 — the running image removed mid-run, macOS | 1 | the procedure as commands | R-6 | reproduces: 9 × *could not start*, 2 × the `mv` lost the race, 1 × the image was gone before it started |
| cell 9 / 14 — *findings exits* (`task validate`, `task finalize` on rig P3) and the orientation row | 3 | rig P3 as §2 builds it has **no task** and an **unstaged** drift, so no task-scoped door can produce these rows on it | R-2c, R-5 | reproduce on rig V (P1's task construction + P3's committed knob) |
| cell 10 — `task validate`, `task finalize`, `start` × (control, counting stub) on rig P3 | 6 | the same missing task | R-2c, R-5 | records +1 at every arm. Spawn count: 2 at `start` in the driver's task state; **1** at `task validate` / `task finalize` in the state the reconciler's rig was in by then (the task also stages an anchored doc) — a datum, still one finding |

`(R2, L-1)`'s own repro block names the same under-built rig P3; the finding stands on R-2c.

**Not demoted, and why:** cell 12's rows and the review-hold rows state their argv sequence and leave only
payload prose unstated (the commit-doc summary, the authored `vision` body); that is enough to run them and
the reconciler did (R-1a, R-5). The Linux rows carry a fenced block.

**One bound on the Linux rows, restated because it bears on the definition of *driven*:** they ran on the
published `1.0.0-rc.24` as `cargo install` builds it for Linux, in a container — not on the installed host
binary. They are counted as driven rows of the *published version*; they add no clap leaf that the host
binary's rows do not already cover (`validate`, `task validate`).

## R-4 · The driver's door-set counts against the registries

Read at `bffa6667`. **Every count the driver states matches the registry it names.**

| registry | driver | read by the reconciler |
|---|---|---|
| production callers of `doc_code_invoker` (a derivation, not a registry) | 3 | **3** — `cli.rs` (store sweep) · `task.rs` (the task gate) · `milestone.rs` (the boundary gate) |
| the doors they are reached from | 6 over 5 paths | **6 over 5** — the header of `tests/probe_failure_doors.rs` says *"all five"* and lists five bullets naming six doors; five clap leaves, the hook being a second route into `validate` |
| `ProbeArgv` | 3 | **3** — `NotProbe` · `Run` · `Refuse(String)` |
| the probe's failure arms | 5 | **5** `refuse(…)` call sites in `doc_code_probe::run` |
| `PROBE_STDERR_BOUND` | 4096 | **4096**; driven: message length 4174 = 78 + 4096 |
| `COMMITTING_DOORS` | 11 rows over 9 verbs | **11 rows over 9 verbs** (`task finalize` ×2, `milestone finalize` ×2) |
| `ERROR_CODE_REGISTRY` | 12 | **12** |
| the record's key set | 8 | **8**, read off an emitted line, in the declared order |
| `STORE_EXIT_FLIPS` | `probe-unreliable` present | **7 members**, `probe-unreliable` first |
| clap leaves that are the door of ≥ 1 driven row | 15 | **15** — each has a row with argv and exit in the tables; all are `VERB_KINDS` leaf paths |

Bookkeeping note, immaterial to coverage: the per-cell tallies in the driver's §9 sum to 174 as stated, but
counting table rows independently gives 19 for cell 11 (stated 18) and 15 for cell 12 (stated 16); the
de-duplication rule the driver states (*a drive that serves two cells is counted once*) is not enough to
re-derive each tally from the tables. Coverage is per leaf and does not depend on it.

## R-5 · Reconciler repro for the demoted rows, and the review hold

```text
# door PI — argv `jigc __probe doc-code --build 1.0.0-rc.24`, run inside rig V (log on); the request on stdin:
#   {"probe_id":"doc-code","target":"spec:padding","effective_state":{"snapshot_path":"<P>"},"config":{},"schema_version":3}
stdin = 3 non-UTF-8 bytes            → exit 1  stdout 0  1 line: doc-code probe: cannot read the request from stdin: stream did not contain valid UTF-8
stdin = `garbage`                    → exit 1  stdout 0  1 line: doc-code probe: cannot parse the request: expected value at line 1 column 1
stdin empty · stdin closed (<&-)     → exit 1  stdout 0  1 line: doc-code probe: cannot parse the request: EOF while parsing a value at line 1 column 0
<P> names no file                    → exit 1  stdout 0  1 line: doc-code probe: cannot read the snapshot `<tmp>/fx/no-such-snapshot.json`: No such file or directory (os error 2)
<P> a mode-000 file                  → exit 1  stdout 0  1 line: … cannot read the snapshot `<tmp>/fx/snap.000`: Permission denied (os error 13)
<P> a directory                      → exit 1  stdout 0  1 line: … cannot read the snapshot `<tmp>/fx/snapdir`: Is a directory (os error 21)
<P> holds `not json`                 → exit 1  stdout 0  1 line: doc-code probe: cannot parse the snapshot `<tmp>/fx/snap.notjson`: expected ident at line 1 column 2
<P> holds {"anchors":[],"working_tree_root":"$REPO","root_kind":"sideways"}
                                     → exit 1  stdout 0  1 line: … unknown variant `sideways`, expected `staged-index` or `working-tree` at line 1 column 137
<P> holds {"anchors":[],"working_tree_root":"$REPO","root_kind":"working-tree"}
                                     → exit 0  stdout {"findings":[],"schema_version":3}  stderr empty        (control)
the same, "schema_version":99 in the request → exit 0, the same response                                      (datum)
every one of the above: invocation-log records +0

$ jigc __probe doc-code --build 0.0.0-skew </dev/null   → exit 1  stdout 0  records +0
  jigc 1.0.0-rc.24: the `doc-code` probe was asked for build `0.0.0-skew`, but this jigc is build `1.0.0-rc.24` — the jigc binary changed while the run was in flight; rerun the command
$ jigc __probe </dev/null                               → exit 1  records +0   jigc 1.0.0-rc.24: `__probe` takes exactly `doc-code --build 1.0.0-rc.24`, got `__probe`
$ jigc --format json __probe doc-code --build 1.0.0-rc.24 </dev/null → exit 2  records +1   error: unrecognized subcommand '__probe'
$ jigc --help | command grep -c __probe                 → 0

# cells 9 / 14 and 10 at the task-scoped doors, rig V (log on; the task of R-2b's second state)
$ jigc --format json task validate rename-the-pad   → exit 3  records +1   codes: doc-code.symbol-exists, file-state.staged-copy, changelog-recording.gate-granted-unused
$ JIGC_DOC_CODE_PROBE=<tmp>/fx/doc-code-count jigc --format json task validate rename-the-pad → exit 3  spawns 1  records +1  one probe-failure finding
$ jigc --format json task finalize rename-the-pad   → exit 3  records +1   codes: doc-code.symbol-exists     HEAD unmoved
$ JIGC_DOC_CODE_PROBE=<tmp>/fx/doc-code-count jigc --format json task finalize rename-the-pad → exit 3  spawns 1  records +1  one probe-failure finding   HEAD unmoved

# the exit-4 review hold, rig D (fresh, log on)
$ printf '# Direction\n\nWe build a small padding library.\n\n## Why\n\nBecause strings need padding.\n' > docs/direction.md   # committed with plain git
$ jigc migrate docs/direction.md --as vision          → task minted: migrate-vision-docs-direction-<hash>
$ jigc doc author vision --from-file - --task <id>    (thesis / invariants / open-questions, one line each)
$ jigc doc set-field commit:<id>#type --task <id> --value docs ; … set-slot commit:<id>#summary …
$ jigc task finalize <id>                             → exit 4  stdout 981  stderr 0  HEAD unmoved
  "migration review required — nothing committed. Re-run `jigc task finalize <id> --approve` …"
  record: {"exit_code":4,"finding_codes":[],"output_bytes":981,"error_code":"migrate.review-pending"}
```

## R-6 · Spot re-drives of rows the driver recorded as matching (a sample, not the table)

```text
# rig V. Override stubs under <tmp>/fx: doc-code-not-executable ('not a program\n', mode 644) ·
# doc-code-flood (4096 × 'a', then PAST-THE-BOUND, then 2,000,000 × 'b' on stderr; exit 1) · doc-code-count
cell 2a  V   JIGC_DOC_CODE_PROBE=<tmp>/fx/doc-code-not-executable jigc validate --format json
             → exit 1  blocking_probes ["pack-probe-integrity"]  probe-failure · crash · "probe `doc-code` could not start `<tmp>/fx/doc-code-not-executable`: Permission denied (os error 13)"   no symbol-exists
cell 2c  V   JIGC_DOC_CODE_PROBE= jigc validate --format json     → exit 0  blocking_probes ["doc-code"]  doc-code.symbol-exists
cell 6   V   JIGC_DOC_CODE_PROBE=<tmp>/fx/doc-code-flood …        → exit 1  wall 0.56 s  message 4174 chars, 4096 × 'a', `PAST` absent
cell 7   V   <tmp>/fx/copy/jigc (cp of the installed build, cmp-identical) validate --format json → exit 0  doc-code.symbol-exists
cell 7   V   the removed-image race, macOS: cp the installed build to <tmp>/fx/race/jigc, run it once (`--version`),
             start `<tmp>/fx/race/jigc validate --format json` in the background, `mv` the file away after 0–30 ms; 12 attempts
             → 9 × exit 1, probe-failure · crash · "probe `doc-code` could not start `<tmp>/fx/race/jigc`: No such file or directory (os error 2)"
               2 × exit 0, doc-code.symbol-exists (the mv lost)   1 × the shell's 127 (the file was gone before exec — not a jigc run)
cell 13      chmod 444 the log, chmod 555 .jigc/logs; jigc validate → exit 0, stdout and stderr cmp-identical to the baseline, records +0; restored → records +1
door HK  rig V, a real `git commit -q -m "arm <arm>" -- notes/<arm>.txt` through the hook `jigc setup` installed (core.hooksPath unset):
  control                        → commit lands; hook stderr "jigc: doc<->code drift detected in committed docs — run `jigc validate` for details (commit not blocked)."; records +1 ["validate","--format","json"] exit 0 ["doc-code.symbol-exists"]
  override = the mode-644 file   → commit lands; hook silent; records +1, exit_code 1, finding_codes ["pack-probe-integrity.probe-failure"]
  override names no file         → commit lands; hook silent; records +1, exit_code 1, finding_codes [], error_code null
  override = the counting stub   → commit lands; hook silent; records +1, exit_code 1, finding_codes ["pack-probe-integrity.probe-failure"]
```

## R-7 · Baseline rows, the carried item, and the leads left open

**Baseline rows: none — this row is a first drive. Nothing to mark CLOSED or STILL-OPEN.**

**The carried item** (M54's *Not run*: the Linux replaced-binary-while-running arm, `/proc/self/exe`):
**DRIVEN by the driver and re-driven here — holds.** The M54 *Not run* entry is discharged for `validate`
and `task validate` on the published Linux build; `task finalize`, `milestone finalize`, bare `start` and
the hook door were not driven on Linux by either.

```text
# image jigc-gate:registry-1.0.0-rc.24 (completions/trial-harness/build-image.sh --registry 1.0.0-rc.24; Linux aarch64).
# rig V shipped in as a tar on stdin, built with COPYFILE_DISABLE=1. A first tar built without it made the
# control itself exit 1 with `schema-conformance.unadopted-instance` in the container, where the host sweep of
# the same tree exits 0; with the variable set the control is clean. Read as macOS tar's extended-attribute
# members landing as stray files beside the docs — an artefact of the transport, not investigated further.
$ jigc --version                                    → jigc 1.0.0-rc.24      (/usr/local/bin/jigc, Linux aarch64)
L0  jigc validate --format json                     → exit 0  doc-code.symbol-exists                       (control)
L1  cp "$(command -v jigc)" /tmp/x/jigc; exec 3</tmp/x/jigc; rm /tmp/x/jigc
    printf '#!/bin/sh\necho IMPOSTOR-RAN >&2\nexit 7\n' > /tmp/x/jigc; chmod 755 /tmp/x/jigc
    /proc/self/fd/3 validate --format json          → exit 0  doc-code.symbol-exists   IMPOSTOR-RAN lines: 0
    /proc/self/fd/3 --format json task validate rename-the-pad → exit 3  doc-code.symbol-exists (+ file-state.staged-copy, the changelog advisory)
L2  a copy started, `mv`-ed away after 2–40 ms and replaced by the impostor; 12 attempts
                                                    → 12 × exit 0 with doc-code.symbol-exists · 0 × probe-failure
```

**Open leads** (never promoted on a source read, never dropped):

| # | lead | origin | why it stays open |
|---|---|---|---|
| O-1 | the *response serialize* failure arm emits one reason line | codex (K-1) / driver *NOT DRIVEN* | no input reaches it from outside the process |
| O-2 | no fourth production probe spawn exists | codex (K-5) | an absence claim over source; not drivable |
| O-3 | no error identity outside `ERROR_CODE_REGISTRY` is release-reachable, and no hook-capable commit bypasses the seam | codex (K-7b) | an absence claim over source; every record read is consistent with it |
| O-4 | no schema-hash-boundary violation in rc.24 (the `planning-record` hint reword) | codex (K-9) | outside this row's cells; not driven here — it belongs to the row that owns the freeze |
| O-5 | a probe grandchild that **never** exits holds the door — and, through the hook, a `git commit` — open without bound | driver (the *Read* of T-1) | driven to 47 s here and 120 s by the driver, never to an unbounded child; `(R2, T-1)` is tiered on what was driven |
| O-6 | `(R2, T-1)` at `task validate`, `task finalize`, `milestone finalize`, `start` and the hook door | driver *NOT DRIVEN* | driven at `validate` only by both; the invoker is one shared function, which is a source read |
| O-7 | `(R2, P-1)` at `milestone finalize` | driver (driven) | the driver's row stands as driven; the reconciler did not re-drive that door |
| O-8 | *row 5:* the doc-only finalize narrated an **untracked** file as *left-out (… a staged path stays staged …)* | driver, in passing | one observation, not re-driven by either; in the reconciler's doc-only drive there was no such file and stdout was 0 bytes. For the row that owns finalize narration |
| O-9 | *row 8:* `jigc start --task <sub-task>` inside a sub-task worktree exits 0 without provisioning the commit doc; `jigc workflow sub-task --task …` does | driver, in passing | one observation, not re-driven here (the reconciler entered the sub-task by its launch line). For the row that owns composed surfaces |
| O-10 | the second, higher-effort Codex pass | — | it ended on a usage limit with no claims file; whatever it would have claimed is unreviewed |

The driver's remaining *NOT DRIVEN* pairs (§7 above, 41) are unchanged by this reconciliation except where
R-3 / R-5 / R-6 drove a pair again; none of them was promoted to driven on a source read.

## R-8 · The `bare` rig rows (K-6, K-7c)

```text
# rig `bare` (a git repo with one commit, no `jigc setup`); CLAUDECODE cleared
$ jigc validate                        → exit 1   "this project isn't set up — run `jigc setup` (no `.jigc/config/` cascade layer found)"
$ jigc --version · jigc bogus · jigc start · jigc config get invocation-log · jigc config set invocation-log true
                                       → exit 0 · 2 · 0 · 1 · 1
after each of the six: no `.jigc/` exists, `git status --short` is empty

$ CLAUDECODE=1 jigc setup              → exit 0, commits +1:  chore(jigc): install jigc workspace config · trailers: [Co-Authored-By: Claude <noreply@anthropic.com>]
# a second `bare` rig:
$ env -u CLAUDECODE jigc setup         → exit 0, commits +1:  chore(jigc): install jigc workspace config · trailers: []
```

## Doors covered

Every clap leaf that is the door of ≥ 1 driven row, in `VERB_KINDS` spelling:

- `start`
- `validate`
- `task validate`
- `task finalize`
- `task discard`
- `milestone create`
- `milestone add-task`
- `milestone add-from-spec`
- `milestone finalize`
- `milestone discard`
- `rename`
- `migrate-corpus`
- `config set`
- `config get`
- `doc show`
- `setup` — **the reconciler's row only** (K-7c, R-8): driven to settle a Codex claim; the trailer it
  asserts is row 10's subject, so a reader who counts only this row's subject reads 15

Not leaves, and doors of driven rows all the same: the `__probe` intercept and the pre-commit hook `jigc
setup` installs. Leaves used only to build a fixture — `workflow`, `migrate`, `task amend`, `milestone
provision`, `milestone execute`, `milestone join`, `doc create`, `doc set-field`, `doc set-slot`, `doc
add-item`, `doc author` — carry no verdict and are **not** claimed.
