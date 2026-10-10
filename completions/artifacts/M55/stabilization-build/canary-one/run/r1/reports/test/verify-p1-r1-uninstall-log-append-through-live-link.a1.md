# verify-real — `r1-uninstall-log-append-through-live-link`

Run `canary-one`, round 1, stage `test`, attempt 1. One finding, one verdict.

- **Ledger key:** `r1-uninstall-log-append-through-live-link`
- **Door:** `jigc uninstall`
- **Clause it is said to break:** `no-lost-files`
- **Triage's grade:** unclear
- **Its source:** `completions/artifacts/canary-one/r1/reports/test/cross-cutting-review.a1.md`, heading `Lead L3` — prose, read at the source, no repro block. The reporter's own clause field there is `none`.

## Verdict

**`refuted` as a blocker — basis `breaks-no-clause`.** Not `does-not-reproduce`: the mechanism the lead describes is real and was driven. It stays a row of the ledger.

- **What is real.** With the `invocation-log` knob on and `.jigc/logs/invocations.jsonl` a link to an existing file outside the repository, a `jigc uninstall` invocation appends its one JSONL record to that outside file. Driven on the plain form (exit 1, a refusal) and on `--help` (exit 0).
- **Why it breaks no clause inside its scope.** The first clause's scope (`DECISIONS.md` → *2026-10-04 — The exit rule, revised*, sharpening 1) is: *no jigc command at exit 0 destroys bytes no git object holds or commits content the user did not ask for*, in a healthy repository used as documented, with deliberately planted states a declared bound. In every arm driven, **no byte of the target was destroyed** — its prior 48 bytes are an intact prefix afterwards, sha256 equal — and nothing was committed. The plain teardown is exit 1, not exit 0. The one arm that lands at exit 0 with an effect on the tree, `jigc uninstall --force`, removes the link without following it and leaves the target **byte-identical**. The exit-0 arm that does write, `--help`, adds one record after the existing bytes and removes none.
- **The precondition is not a documented use.** jigc only ever creates the log as a regular file; a link at that gitignored path is placed by the operator. No guide and no design section describes relocating the log through a link (`design/measurement.md` → *The in-repo invocation log*, items 1 to 7, read whole; `crates/cli/guides/QUICKSTART.md` names the knob and the teardown order only).
- **Not `intended`.** No settled decision says the log is to be written through a link. The source's doc comment (`crates/cli/src/invocation_log.rs`, on `append_record`) rules on the dangling link only; item 7 of the design section says the teardown *appends only to a log that is already there and creates nothing*, and is silent on what *there* means for a link whose target exists. So the behaviour is unruled, and the verdict rests on the clause's scope, never on a decision.
- **`contested`:** false. The finding argues against no decision.
- **Regression fact:** not established and not owed — step 4 runs with `confirmed` only. The previous release's hash was asserted (below) and nothing was driven on it.

## The binaries

Both asserted through `dev/stabilize-step hash` before the first command, the output read whole:

```text
{"act": "hash", "status": "hashed", "path": "bin/c1.a1/jigc", "bytes": 15795744, "content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc", "sha256": "2d69497a3ae508f3f6ad6737913308a8ddea060d89735de71357d037cf4ca575"}
{"act": "hash", "status": "hashed", "path": "bin/previous-91834b5e011d/jigc", "bytes": 15240064, "content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d", "sha256": "07a25005d6362085774a346fd39e2ca3524f4c2aa1f0d959a273e33dcb7efe4f"}
```

- Candidate `content_sha256` `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` — equal to the one the prompt hands (commit `eeffe347`, label c1).
- Previous release `content_sha256` `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` — equal to the one the prompt hands (`1.0.0-rc.24`). Asserted, not driven.
- The candidate's directory went first on `PATH` in every call, and `command -v jigc` printed the candidate's path each time. Both rigs were built with `dev/jigc-rig fresh --binary <candidate>`. Nothing under `target/` was driven and nothing was built.
- The candidate prints `binary_version` `1.0.0-rc.24` in its log records, as the contract says it will; the evidence here is keyed by the commit and the hash.

## What was driven

Two fresh rigs of my own, each minted under a `mktemp -d` directory beneath the scratch root the prompt names. The lead carries no block, so the setup is reconstructed from triage's sentence, and this is the whole of what I added:

1. `jigc config set invocation-log true` — exit 0.
2. `git add .jigc/config/manifest.yaml`, then `git commit` — exit 0. The commit keeps the knob's own config delta from being the teardown's refusal. The installed `pre-commit` hook ran `jigc validate` and minted the log as a regular file, 193 bytes, one record.
3. The minted log moved out of the repository (`mv`), to a sibling directory of the repository inside the rig root.
4. A second file written beside it, `keep.txt`: 48 bytes, two lines of text that are not log records, sha256 `b98409e96f093172bc34049d1fc63a1727231d903313abcbcf2e3ba85c8f724b`.
5. `ln -s <rig>/outside/keep.txt .jigc/logs/invocations.jsonl`.
6. `git status --short` — empty, exit 0: the tree is clean, the link sits under a gitignored directory.

### Cell A — `jigc uninstall`, plain (rig 1)

```text
$ jigc uninstall
exit: 1
stdout: (0 bytes)
stderr:
blocking · uninstall.foreign-bytes — `.jigc/` holds 1 path(s) jigc did not write — the tree is gitignored, so removing it would destroy bytes nothing else has a copy of:
  .jigc/logs/invocations.jsonl
  route: move what you need out of the paths above, or delete the ones you do not (`rm -r` takes them — jigc has no verb that clears `.jigc/displaced/`), then re-run `jigc uninstall`; or, once you have confirmed they hold nothing you need, `jigc uninstall --force` deletes them with the install
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

The target, before and after:

| | bytes | lines | sha256 |
|---|---|---|---|
| before | 48 | 2 | `b98409e9…5c8f724b` |
| after | 249 | 3 | `13adc5fa…0a0b81bf` |

`diff` of the saved copy against the target names one added line and nothing else:

```text
2a3
> {"timestamp":"2026-10-08T09:07:46Z","argv":["uninstall"],"exit_code":1,"duration_ms":14,"finding_codes":["uninstall.foreign-bytes"],"output_bytes":576,"binary_version":"1.0.0-rc.24","error_code":null}
```

The first 48 bytes of the target after the run hash to `b98409e9…5c8f724b`, the value before it. `.jigc/` stands whole, the link still a link, the inode of the target unchanged.

### Cell B — `jigc uninstall --help` (rig 1, straight after cell A)

```text
$ jigc uninstall --help
exit: 0
stdout: 4624 bytes (the help)
stderr: 0 bytes
```

| | bytes | lines | sha256 |
|---|---|---|---|
| before | 249 | 3 | `13adc5fa…0a0b81bf` |
| after | 435 | 4 | `f7b00a18…4d95a144` |

The added line:

```text
{"timestamp":"2026-10-08T09:08:07Z","argv":["uninstall","--help"],"exit_code":0,"duration_ms":14,"finding_codes":[],"output_bytes":4624,"binary_version":"1.0.0-rc.24","error_code":null}
```

### Cell C — `jigc uninstall --force` (rig 2, a fresh rig, the same six setup steps)

```text
$ jigc uninstall --force
exit: 0
stdout:
jigc uninstall — repo-local install removed

  - removed .jigc/
  - unwired bootstrap reference ← CLAUDE.md
  - removed jigc allowlist permit ← .claude/settings.json
  - removed SessionStart hook ← .claude/settings.json
  - removed deny safety floor ← .claude/settings.json
  - removed pre-commit hook
  - removed jigc guide artifact
— jigc · run `jigc start` for orientation; all writes through `jigc`.
stderr:
warning: removing the workbench directory .jigc/logs discards work that is not in git:
    .jigc/logs/invocations.jsonl
  note: the workbench directory is the only copy of these bytes — they are not recoverable.
warning: removing `.jigc/` also removes 7 tracked file(s) under it:
    .jigc/.gitignore
    .jigc/AGENT.md
    .jigc/config/.gitkeep
    .jigc/config/manifest.yaml
    .jigc/config/packs.yaml
    .jigc/settings-entries.json
    .jigc/version
  note: each is in the index, so `git -C <rig>/repo checkout -- <path>` brings it back.
```

| | bytes | sha256 |
|---|---|---|
| before | 48 | `b98409e9…5c8f724b` |
| after | 48 | `b98409e9…5c8f724b` |

`.jigc/` is gone (`ls` exit 1), the target and the moved-out log both stand in the outside directory, untouched. The link was removed as a link and never followed; no record was appended, because the log's path went with the tree before the record is written.

One thing the forced run says that is not so in this state: it names `.jigc/logs/invocations.jsonl` as *the only copy of these bytes — they are not recoverable*, where the entry was a link and the bytes it pointed at are still there. Nothing is lost by it; it is in `left_open`.

### Control — a minting verb against the same link (rig 1, after cell B)

```text
$ jigc doc list
exit: 0
stdout: jigc doc list — no committed docs
```

The target went from 435 to 611 bytes, one more record (`"argv":["doc","list"]`). So the write through a live link is the log writer's behaviour for every verb with the knob on, and is not particular to the teardown's append-only arm: both arms open with `append(true)` and no no-follow flag. Driven once, as a control; not pursued.

## Against the claim, and against the design

- **The lead's claim** — *a link whose target exists is appended through* — **holds** on the candidate, for the plain teardown and for its `--help`.
- **Against the clause's scope:** exit 1 on the plain form; no destroyed byte and no commit on any form; the forced, landing form leaves the target byte-identical. Nothing inside *at exit 0 destroys bytes no git object holds or commits content the user did not ask for*.
- **Against the design section that owns the log** (`design/measurement.md` → *The in-repo invocation log*, item 7): the teardown *appends only to a log that is already there and creates nothing*. Driven, it created nothing: no file, no directory, in either rig. Whether a live link's target counts as *a log that is already there* is unruled.
- **Against the teardown's own help**, which the candidate prints: *Touches nothing outside the repository*. Cells A and B each added 186 to 201 bytes to a file outside the repository. That sentence is about the door's removals, and the write is the log wrapper's, made after the door returns — but the sentence is unqualified, and a reader would take it at its word. This is the residue the ledger row keeps; it is a help-truth matter and breaks no clause of the closing condition as scoped.

## The class

**Instance, unbounded.** I drove one door, `jigc uninstall`, in three forms (plain, `--help`, `--force`), over one shape (a link at the log's file path whose target is a regular file outside the repository), plus one control verb. I did not enumerate the log writer's consumers, and I give no count. Not driven: a link at `.jigc/logs` itself (the directory), a link whose target is inside the repository or tracked, a hard link, a target that is not writable, a usage error at the teardown (clap's exit 2), and any platform but this one (macOS, APFS).

## Coverage

The lead makes no coverage claim, so step 5 has nothing to verify. For the block below, read at the tests and not at the diff:

- `crates/cli/src/invocation_log.rs`, unit test `an_append_only_write_creates_nothing_and_still_appends_to_a_log_that_is_there` — plants a **dangling** link at the log's path and asserts nothing is created through it. No live link.
- `crates/cli/tests/uninstall_workbench_subject.rs`, `a_symlink_inside_a_cache_directory_blocks_and_its_target_is_untouched` — a live link at `.jigc/state/link`, knob off, asserts the refusal and that the target's bytes are unchanged. Not the log's path, and no knob, so the log writer never runs against it.
- A search of both test trees for a `symlink(` call at the log's path finds the unit test above and no other.

No test drives a live link at the log's path with the knob on.

## The repro block

#### Repro V1

```yaml
claim: "with the invocation-log knob on and .jigc/logs/invocations.jsonl a link to an existing file outside the repository, `jigc uninstall` at exit 0 destroys bytes of the link's target (clause no-lost-files)"
verdict: REFUTED
basis: breaks-no-clause
binary: "candidate c1, commit eeffe347, content_sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"
setup:
  - fixture: fresh                      # dev/jigc-rig fresh --binary <binary>
  - ["jigc", "config", "set", "invocation-log", "true"]            # exit 0
  - ["git", "add", ".jigc/config/manifest.yaml"]
  - ["git", "commit", "-q", "-m", "chore: turn the invocation log on"]   # the hook's `jigc validate` mints the log
  - "mkdir <rig>/outside; mv .jigc/logs/invocations.jsonl <rig>/outside/moved-log.jsonl"
  - "write <rig>/outside/keep.txt: 48 bytes, 'line one of a file that is not the log\\nline two\\n'"
  - "ln -s <rig>/outside/keep.txt .jigc/logs/invocations.jsonl"
  - "git status --short is empty"
repro:
  - cell: plain
    argv: ["jigc", "uninstall"]
    expect:
      exit: 1
      stdout_bytes: 0
      stderr_contains:
        - "blocking · uninstall.foreign-bytes"
        - ".jigc/logs/invocations.jsonl"
      target_first_48_bytes: "identical to before (sha256 b98409e96f093172bc34049d1fc63a1727231d903313abcbcf2e3ba85c8f724b)"
      link: "still a link, .jigc/ still whole"
      observed_not_asserted: "the target grows by one JSONL record, argv [uninstall], exit_code 1, finding_codes [uninstall.foreign-bytes] — 48 -> 249 bytes"
  - cell: forced                        # a fresh fixture, the same setup
    argv: ["jigc", "uninstall", "--force"]
    expect:
      exit: 0
      stdout_contains: "removed .jigc/"
      target: "byte-identical to before: 48 bytes, sha256 b98409e96f093172bc34049d1fc63a1727231d903313abcbcf2e3ba85c8f724b"
      moved_log: "byte-identical to before: 193 bytes"
      jigc_dir: "absent"
  - cell: help                          # the plain cell's fixture, after it
    argv: ["jigc", "uninstall", "--help"]
    expect:
      exit: 0
      target_first_48_bytes: "identical to before"
      observed_not_asserted: "the target grows by one JSONL record, argv [uninstall, --help], exit_code 0"
pinned-by: "UNPINNED: no test plants a live link at the log's path with the knob on — the unit test in invocation_log.rs covers the dangling link only, and uninstall_workbench_subject's symlink cell is at .jigc/state/link with the knob off"
```

**Pinnable as it stands: yes**, with two notes for whoever converts it.

- It needs a unix link, so it is a `#[cfg(unix)]` test like its neighbour in `uninstall_workbench_subject.rs`, whose `Installed` fixture already builds an installed repository with an outside file.
- The two `observed_not_asserted` lines are deliberate. The block pins what refutes the claim — the exits, the refusal, the intact prefix, the untouched target under `--force`. It does **not** pin the appended record as wanted behaviour: whether the log writer should follow a live link is unruled, and a standing test asserting the append would turn an unruled behaviour into a contract.

## Left open

Not pursued; each is for triage like any finding.

1. **The teardown's help says *Touches nothing outside the repository*, and with the knob on and a live link at the log's path a plain `jigc uninstall` and `jigc uninstall --help` each add a record to a file outside it.** Driven (cells A and B). A help-truth matter; no clause graded here.
2. **Every other verb's minting arm writes through the same live link.** One control drive (`jigc doc list`, exit 0, the target 435 to 611 bytes). The class is the log writer, not the teardown; unbounded here.
3. **`jigc uninstall --force` names a link at the log's path as *the only copy of these bytes — they are not recoverable*,** where the entry is a link and its target outside the repository survives the run byte-identical (cell C). The warning overstates; nothing is lost.

## Tree state

Branch `fix/canary-one` at `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`. `git status --short` at the start: `?? completions/artifacts/canary-one/r1/` and nothing else. I edited, staged and committed nothing in the repository; this report is the one file I wrote, and it reaches the run's directory through the record script only.

<!-- end of report -->
