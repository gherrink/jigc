<!-- Reconciled ROW 10 file (co-author trailer), copied verbatim below this line. Driven on the installed registry build `~/.local/bin/jigc` -> `jigc 1.0.0-rc.24`, 2026-10-03. `axis10` in the body means ROW 10 of this run, not numbered axis 10. -->

> **Row 10 · co-author trailer — RECONCILED (rc.24).** The driver record follows **unchanged** (zero demotions — see the ledger's *Rows checked for a repro block*); the *Reconciliation ledger* and *Doors covered* sections after it are the reconciler's. This row **has** a source pass (Codex, exit 0, one numbered claim plus per-sentence dispositions). Reconciler: independent of the driver file and of the build; nothing in the repository was edited, committed or fixed.

# Row 10 · co-author trailer — driver record (rc.24)

Partial per-axis re-review of the published `jigc 1.0.0-rc.24`. Row 10 of this run (a NEW brief, no
baseline, first drive). Driver: Opus, independent of the build. Nothing in the repository was edited,
committed or fixed.

- **Binary:** `~/.local/bin/jigc`; `jigc --version` printed `jigc 1.0.0-rc.24` (asserted first). Release
  posture. Never `target/debug/jigc`, never `cargo run`.
- **git:** `git version 2.54.0 (Apple Git-157)`, darwin.
- **Environment at the top (state only, never a value):** `CLAUDECODE` set, non-empty · `AI_AGENT` set,
  non-empty · `JIGC_ADAPTERS_DIR` unset · `JIGC_PACK_DIR` unset · `GIT_CONFIG_GLOBAL` unset ·
  `GIT_CONFIG_SYSTEM` unset. Every cell below sets the variable explicitly; none inherits it.
- **Rigs:** `rig=$(dev/jigc-rig <state> --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit`
  — stdout only. Every scratch root from `mktemp -d`; no teardown; no `rm -r`.
- **Counting:** `git log -1 --format='%(trailers:key=Co-Authored-By,valueonly)' <sha>` (git's parser, one
  line per trailer) **and** `git log -1 --format=%B <sha>` (placement). "co-authored-by(git)=N" below is
  the first; the `|`-prefixed blocks are the second.

**Verdict on the exit rule: no tier-1 row.** Six defects, all proposed tier 3. 237 rows driven.

---

## 1. The door set, derived from the code

| registry | file | count I read | the instrument's count |
|---|---|---|---|
| `COMMITTING_DOORS` | `crates/cli/src/invocation_log.rs` | **11 rows over 9 clap leaves** (`task finalize` ×2 arms, `milestone finalize` ×2 arms, `rename`, `migrate-corpus`, `milestone create`, `milestone add-task`, `milestone add-from-spec`, `milestone discard`, `task discard`) | 11 over 9 — same |
| the one exclusion | `crates/cli/src/setup.rs` → `commit_install` | 1 (`git commit --no-verify -m <message> -- <paths>`, signs its own message via `session_co_author()`) | 1 — same |
| **doors** | | **12** | 12 — same |
| `CoAuthor` | `crates/cli/src/adapter.rs` | 3 fields (`name`, `email`, `when-env`); `RawCoAuthor` is `deny_unknown_fields` | 3 — same |
| `CoAuthorBasis` | `crates/cli/src/task.rs` | 2 (`Session`, `SessionOrHead`) | 2 — same |

**Data the instrument's count does not carry (read off the code, then driven):**

- `task finalize` has **five** commit constructions behind one leaf, not the three the scope names
  (ordinary · amend · doc-only): `StagePolicy::{MigrationFixed, IndexHonoring, Amend, DocOnly, Combine}`.
  The **migration** finalize (`jigc migrate <path> --as <type>` → `task finalize --approve`) is a shape of
  its own; it is driven below (§A, rows M1–M3).
- A census of the literal `"commit"` handed to git in production code finds exactly one site outside
  the seam (`setup.rs`, the install commit). The fan-out boundary moves `HEAD` with `git merge --ff-only`
  (no new commit object). So no production commit bypasses the seam or the install commit — **a source
  read, recorded as a classification, not as a driven row**; the driven half is the "HEAD does not move"
  rows in §A.

---

## 2. The row table

Schema: `(door, cell) → {argv driven, exit, code|none, route kind, surface asserted, verdict}`.
Arms: **set** = `CLAUDECODE=1 jigc …` · **unset** = `env -u CLAUDECODE jigc …` · **empty** = `CLAUDECODE= jigc …`.
"1 / 0 / 0" = co-authored-by(git) on every commit landed since the pre-act `HEAD`, per arm.

### A · Cell 1 (door sweep) and cell 8 (record-only doors) — 12 doors × {set · unset · empty}

| # | door | argv driven | exit (s/u/e) | code | route kind | surface asserted | observed (s/u/e) | verdict |
|---|---|---|---|---|---|---|---|---|
| A1–3 | `setup` (install commit) | `jigc setup` in a `bare` rig | 0/0/0 | none | none | `%(trailers)` + `%B` of the one landed commit | 1 / 0 / 0 | matches |
| A4–6 | `task finalize` | `jigc task finalize land-the-work` | 0/0/0 | none | none | same | 1 / 0 / 0 | matches |
| (C) | `task finalize (amend)` | see §C rows C4–C6 (a `HEAD` that does not carry it) | 0/0/0 | none | none | same | 1 / 0 / 0 | matches |
| A7–9 | `milestone finalize (squash: true)` | `jigc milestone finalize cache-rework` | 0/0/0 | none | none | the one combine commit | 1 / 0 / 0 | matches |
| A10–12 | `milestone finalize (squash: false)` | same, knob `false` | 0/0/0 | none | none | **both** landed commits (per-sub-task + aggregate) | 1,1 / 0,0 / 0,0 | matches |
| A13–15 | `rename` | `jigc rename adr:alpha-decision --to "Beta decision"` | 0/0/0 | none | none | landed commit | 1 / 0 / 0 | matches |
| A16–18 | `migrate-corpus` | `jigc migrate-corpus` | 0/0/0 | none | none | landed commit | 1 / 0 / 0 | matches |
| A19–21 | `milestone create` | `jigc milestone create "Cache rework"` | 0/0/0 | none | none | record commit | 1 / 0 / 0 | matches |
| A22–24 | `milestone add-task` | `jigc milestone add-task cache-rework "Area low"` (and `"Area docs"`) | 0/0/0 | none | none | each record commit | 1 / 0 / 0 | matches |
| A25–27 | `milestone add-from-spec` | `jigc milestone add-from-spec rate-limit spec:rate-limit` | 0/0/0 | none | none | **both** seeded record commits | 1,1 / 0,0 / 0,0 | matches |
| A28–30 | `milestone discard` | `jigc milestone discard cache-rework` | 0/0/0 | none | none | record commit | 1 / 0 / 0 | matches |
| A31–33 | `task discard` (sub-task) | `jigc task discard area-low --force` | 0/0/0 | none | none | record commit | 1 / 0 / 0 | matches |
| M1–3 | `task finalize`, **migration** shape | `jigc task finalize <migrate-task> --approve` | 0/0/0 | none | none | landed commit (`A VISION.md; D docs/direction.md`) | 1 / 0 / 0 | matches |
| V1–3 | `task finalize`, other non-empty values | `CLAUDECODE=0` · `=false` · `=' '` | 0 each | none | none | landed commit | 1 · 1 · 1 | matches the contract (*set and non-empty*) — datum O-3 |
| T1–3 | `task finalize` · `rename` · `migrate-corpus` — tree equality | each under set/unset/empty | 0 | none | none | `git rev-parse HEAD^{tree}` | EQUAL across the three arms, each door | matches (signing moves no tree) |
| R1–4 | `setup` re-run | `jigc setup` again, nothing changed (set, unset); then after a tracked install file was removed by hand (set, unset) | 0 | none | none | `HEAD` / the second install commit | no commit · no commit · 1 · 0 | matches |
| N1–11 | leaves **not** on the axis, under set | `ingest` · `validate` · `upgrade` · `describe` · `config set docs-root documentation` · `milestone list-tasks nope` (exit 1) · `task discard <top-level> --force` · `milestone provision` · `milestone join` · `milestone execute` · `uninstall --force` | 0 (one 1) | none | none | `git rev-parse HEAD` before/after | HEAD did not move at any | matches (thin rows — see §6) |

### B · Cell 7 — the doc-only path-scoped commit

| # | door | argv | exit | code | surface asserted | observed | verdict |
|---|---|---|---|---|---|---|---|
| B1–3 | `task finalize` (doc-only, `report-inconsistency`) beside two staged code files | `jigc task finalize guide-names-a-flag` × set/unset/empty | 0/0/0 | none (one advisory `file-state.staged-copy`, Informational) | trailer on the commit; commit's file list; `git diff --cached --name-status` and the digest of `git ls-files -s -- other.txt src/other.rs` before vs after | 1 / 0 / 0 · commit holds only `docs/inconsistencies/guide-flag-drift.md` · staged set and entries identical before/after in all three arms | matches |

### C · Cell 2 — amend

Tree and parent unchanged, sha moved, in **every** row below.

| # | `HEAD` before | amender | before → after | verdict |
|---|---|---|---|---|
| C1 | jigc commit, agent session (carries it) | set | 1 → 1 | matches (lands once) |
| C2 | same | unset | 1 → 1 | matches (carried over) |
| C3 | same | empty | 1 → 1 | matches (carried over) |
| C4 | jigc commit, human session (does not) | set | 0 → 1 | matches |
| C5 | same | unset | 0 → 0 | matches |
| C6 | same | empty | 0 → 0 | matches |
| C7/C8 | plain git, hand-typed `Co-Authored-By: Claude Opus <noreply@anthropic.com>` | unset / set | 1 → 1 / 1 → 1; **the value becomes `Claude <noreply@anthropic.com>`** | matches (identity is the address) — datum O-1 |
| C9/C10 | plain git, `co-authored-by: Claude <NOREPLY@anthropic.com>` (key and address in another case) | unset / set | 1 → 1 / 1 → 1 (canonical spelling lands) | matches |
| C11/C12 | plain git, the address only in body prose | unset / set | 0 → 0 / 0 → 1 | matches (prose is not a claim) |
| C13/C14 | plain git, a **different** co-author (`Pat Example <pat@example.com>`) | unset / set | 1 → 0 / 1 → 1 (Claude only) | matches the amend arm's own contract (re-authored from scratch; not this row's) |
| C15/C16 | plain git, the trailer in a paragraph that is **not** the last | unset / set | 0 → 0 / 0 → 1 | matches (git reads none before) |
| C17 | plain git, git-recognized **mixed** last paragraph carrying the profile's trailer | unset | **1 → 0** | **DEFECT (R10, F2)** |
| C18 | same | set | 1 → 1 | matches |
| C19 | jigc commit, agent session; amender authors **no** trailer item | unset | 1 → 1, and the composed step printed *"the amended message carries only the trailer items you add here"* | **DEFECT (R10, F5)** |
| C20 | same; amender authors only `Pat Example <pat@example.com>` | unset | 1 → 2 (Pat, then Claude) | part of (R10, F5) |

### D · Cell 3 — dedupe by address (`task finalize`, the doc's own `#trailers`)

| # | the commit doc already carries | set | unset | verdict |
|---|---|---|---|---|
| D1 | `Co-Authored-By: Claude <noreply@anthropic.com>` | 1 | 1 | matches |
| D2 | same address, `Claude Opus` | 1 (doc's value stands, byte-identical) | 1 | matches |
| D3 | key typed `co-authored-by` | 1 (line stays lower-case) | 1 | matches |
| D4 | a different co-author (`Pat Example <pat@example.com>`) | 2 (Pat, then Claude) | 1 (Pat) | matches (both survive) |
| D5 | the address only in body prose | 1 (appended) | 0 | matches |
| D6 | address in upper case | 1 | — | matches |
| D7 | **bare address, no angle brackets** (`Co-Authored-By: noreply@anthropic.com`) | **2** | — | **DEFECT (R10, F6)**, marginal |
| D8 | the agent's trailer first, a `Refs` item after it | 1 | — | matches |
| D9 | the agent's address under `Signed-off-by` | 1 (`Co-Authored-By` appended) | — | matches (another key is not a co-author claim) |
| D10 | a different co-author, then `Refs` | 2 | — | matches |
| D11 | key `CO-AUTHORED-BY`, value `claude  < noreply@anthropic.com >` | 1 | — | matches |

### E · Cell 4 — message shapes (`task finalize`)

| # | shape | set: where it lands · git's count | unset | verdict |
|---|---|---|---|---|
| S1 | subject only | own block after one blank line · 1 | 0 | matches |
| S2 | subject + body | own block · 1 | 0 | matches |
| S3 | body + a `Refs` trailer item | appended into the block (`Refs`, then it) · 1; git reads both | 0; git reads `Refs` | matches |
| S4 | last body paragraph mixes prose and a trailer-shaped line | own block · 1 | 0 | matches |
| S5 | last body paragraph all trailer-shaped, no doc items | appended into that paragraph · 1; git reads both | 0 | matches |
| S6 | git-recognized mixed paragraph that itself carries the agent line | own block · 1 (the raw text carries the string twice) | 1 (git reads the mixed block) | matches by git's count — datum O-2 |
| S7 | last body paragraph a `See: https://…` line | appended into it · 1 | 0 | matches |
| S8 | `BREAKING CHANGE: …` + `Refs:` paragraph | own block · 1 | 0 | matches |
| S9 | the agent trailer typed into body prose as the last paragraph | not doubled · 1 | 1 | matches |
| S10 | the same, then a doc `Refs` item | appended after `Refs` · 1 (raw text twice) | 0 | matches by git's count — O-2 |
| S11 | **git-recognized mixed last paragraph: `Signed-off-by:` + one prose line** | own block · 1 — **and git no longer reads the `Signed-off-by`** | git reads `Signed-off-by` | **DEFECT (R10, F1)** |
| S12 | ambient `trailer.ifexists/ifmissing=doNothing`, `trailer.where=start`, `trailer.co-authored-by.ifexists=replace` in the repo | after `Refs`, last line · 1 | — | matches (jigc renders it itself) |

### F · Cell 5 — the profile through `JIGC_ADAPTERS_DIR` (all under `CLAUDECODE` set)

| # | profile copy | `jigc setup --format json` in a `bare` rig | `jigc task finalize` in a `fresh` rig | verdict |
|---|---|---|---|---|
| P1 | unmodified (control) | exit 0 · install commit 1 | exit 0 · 1 | matches |
| P2 | `co-author:` absent | exit 0 · 0 | exit 0 · 0 | matches (*no key, no trailer*) |
| P3 | `name: ""` | exit 1 · `setup.profile-load` blocking · no commit, clean worktree | exit 0 · **commit lands, 0** | matches the contract; route sentence → (R10, F3) |
| P4 | `name: "Cl<aude"` | exit 1 · `setup.profile-load` | exit 0 · 0 | same |
| P5 | `email: "noreply @anthropic.com"` | exit 1 · `setup.profile-load` | exit 0 · 0 | same |
| P6 | `when-env: 1CLAUDE` | exit 1 · `setup.profile-load` | exit 0 · 0 | same |
| P7 | `when-env: CLAUDE-CODE` | exit 1 · `setup.profile-load` | exit 0 · 0 | same |
| P8 | `when-env: ""` | exit 1 · `setup.profile-load` | exit 0 · 0 | same |
| P9 | unknown key `extra: x` in the block | exit 1 · `setup.profile-load` (*unknown field `extra`, expected one of `name`, `email`, `when-env`*) | exit 0 · 0 | same |
| P10 | `email` field missing | exit 1 · `setup.profile-load` (*missing field `email`*) | exit 0 · 0 | same |
| P11 | directory set, no `claude-code.yaml` in it | exit 1 · `setup.profile-load` (*No such file or directory*) | exit 0 · 0 | same |
| P12 | file present, mode 000 | exit 1 · `setup.profile-load` (*Permission denied*) | exit 0 · 0 | same |
| P13 | `name: Robo Example`, `email: robo@example.com` | exit 0 · 1 (`Robo Example <robo@example.com>`) | exit 0 · 1 (same value) | matches (the trailer is the profile's) |
| P14–17 | `when-env: JIGC_R10_AGENT` × {that variable set/unset} × {`CLAUDECODE` set/unset} | set·set 1 · set·unset 1 · unset·set **0** · unset·unset 0 | same four: 1 · 1 · **0** · 0 | matches (follows the named variable, not `CLAUDECODE`) |
| P18 | same profile, `JIGC_R10_AGENT=` (set, empty), `CLAUDECODE` set | exit 0 · 0 | — | matches |
| P19/20 | `JIGC_ADAPTERS_DIR=` (set, empty) → embedded profile | set 1 · unset 0 | — | matches |
| P21/22 | `co-author:` absent **at amend time**, `HEAD` carries the shipped trailer | — | amend finalize: unset 1 → 0 · set 1 → 0 | matches (*no key, no trailer*) |

### G · Cell 6 — fan-out, both squash modes (1 code-carrying + 1 docs-only sub-task)

The three-arm sweep is rows A7–A12. Additional rows:

| # | cell | argv | exit | observed | verdict |
|---|---|---|---|---|---|
| G1 | **the declared bound**, `squash: false`: sub-tasks built under set, boundary run under **unset** | `env -u CLAUDECODE jigc milestone finalize cache-rework` | 0 | per-sub-task commit 0 · aggregate 0 | HOLDS AS DECLARED (an omission) |
| G2 | the declared bound, `squash: true` | same | 0 | combine commit 0 | HOLDS AS DECLARED |
| G3 | reverse, `squash: false`: built under unset, boundary under **set** | `CLAUDECODE=1 jigc milestone finalize cache-rework` | 0 | 1 · 1 | matches |
| G4 | reverse, `squash: true` | same | 0 | 1 | matches |
| G5 | the bound's **route**, `squash: false`, code-carrying sub-task: `commit:area-low#trailers` records the agent; boundary under unset | same as G1 | 0 | per-sub-task commit **1** (the doc's) · aggregate 0 | matches (the route works here) |
| G6 | the route, `squash: false`, **docs-only** sub-task: `commit:area-docs#trailers` records the agent; boundary under unset | same | 0 | per-sub-task commit 0 · aggregate **0** | part of **DEFECT (R10, F4)** |
| G7 | the route, **`squash: true` (the pack default)**: `commit:area-low#trailers` records the agent; boundary under unset | same | 0 | combine commit **0**; nothing narrated | **DEFECT (R10, F4)** |
| G8 | `squash: false`, **two** code-carrying sub-tasks + one docs-only; `commit:area-high#trailers` already names the agent as `Claude Opus`; boundary under set | `CLAUDECODE=1 jigc milestone finalize cache-rework` | 0 | three commits: 1 (`Claude Opus …`, not doubled) · 1 · 1 | matches |
| G9 | same, boundary under unset | | 0 | 1 (the doc's own) · 0 · 0 | matches |

### H · Cell 9 — rejection, then re-run (a rejecting `pre-commit` behind `core.hooksPath`)

Every rejecting run: exit 1, blocking, `HEAD` unchanged, the hook's marker relayed. Route kind of each
rejection: Mechanical (re-run the door). Codes (driven with `--format json`, set arm):

| door | code |
|---|---|
| `task finalize` | `finalize.commit-rejected` |
| `task finalize (amend)` | `finalize.amend-rejected` |
| `milestone finalize (squash: true)` | `milestone-finalize.commit-rejected` |
| `milestone finalize (squash: false)` | `milestone-finalize.chain-commit-rejected` |
| `rename` | `rename.commit-rejected` |
| `migrate-corpus` | `migrate-corpus.commit-rejected` |
| `milestone create` | `milestone-create.commit-rejected` |
| `milestone add-task` | `milestone-add-task.commit-rejected` |
| `milestone add-from-spec` | `milestone-add-from-spec.commit-rejected` |
| `milestone discard` | `milestone-discard.commit-rejected` |
| `task discard` | `task-discard.commit-rejected` |

| # | door | rejected under → re-run under | landed by the re-run | verdict |
|---|---|---|---|---|
| H1–33 | each of the eleven rows above | set → set · set → unset · unset → set | 1 · 0 · 1 on every landed commit (both commits for `squash: false` and for `add-from-spec`) | matches at all 33 (the trailer is the re-run's session, never a leftover of the refused run) |
| H34 | `milestone finalize (squash: false)`, **partial chain**: hook passes commit 1, rejects commit 2, under set; re-run under unset | exit 1 `milestone-finalize.chain-commit-rejected`; **nothing landed on `HEAD`**; then exit 0 | 0 · 0 | matches |
| H35 | same, rejected under unset, re-run under set | same | 1 · 1 | matches |

### I · Cell 10 — pinned keys untouched (`--format json`, set vs unset, sha-normalized)

| # | door | exit s/u | `git log -1 --format=%s` s vs u | stdout | stderr | verdict |
|---|---|---|---|---|---|---|
| I1 | `setup` | 0/0 | identical | byte-equal | byte-equal (empty) | matches |
| I2 | `task finalize` (`committed.subject`) | 0/0 | identical | byte-equal | byte-equal | matches |
| I3 | `task finalize (amend)` | 0/0 | identical | byte-equal | byte-equal | matches |
| I4 | `rename` | 0/0 | identical | byte-equal | byte-equal | matches |
| I5 | `migrate-corpus` | 0/0 | identical | byte-equal | byte-equal | matches |
| I6 | `milestone create` | 0/0 | identical | byte-equal | byte-equal | matches |
| I7 | `milestone add-task` | 0/0 | identical | byte-equal | byte-equal | matches |
| I8 | `task discard` | 0/0 | identical | byte-equal | byte-equal | matches |
| I9 | `milestone discard` | 0/0 | identical | byte-equal | byte-equal | matches |
| I10 | `milestone add-from-spec` | 0/0 | identical | byte-equal | byte-equal | matches |
| I11 | `milestone finalize (squash: true)` | 0/0 | identical | byte-equal | byte-equal | matches |
| I12 | `milestone finalize (squash: false)` | 0/0 | identical | byte-equal | byte-equal | matches |

No envelope of the 24 mentions the trailer string. (With a hook that *echoes* its message file the
`hook_output` key differs between the arms — the hook's doing; datum O-4.)

### J · Cell 11 — the hook stream

| # | door | cell | observed | verdict |
|---|---|---|---|---|
| J1–3 | `task finalize` | a `commit-msg` hook echoing the file it is handed × set/unset/empty | the message the hook is handed **already carries** the trailer under set (last line, after `Refs: TKT-1`), and does not under unset/empty; `committed.hook_output` and the stderr relay carry the same bytes | matches |
| J4/5 | `task finalize (amend)` | same × set/unset | handed message carries it / does not | matches |
| J6/7 | `milestone create` | same × set/unset | carries it / does not | matches |
| J8/9 | `rename` | same × set/unset | carries it / does not | matches |
| J10 | `task finalize` | a `commit-msg` hook that appends `Change-Id:` through `git interpret-trailers --in-place`, set | one block: `Co-Authored-By`, then `Change-Id`; git reads both; 1 | matches |
| J11/12 | `task finalize`, `chatty-hooks` rig | the success relay × set/unset | `committed.hook_output` = `chatty-hook: pre-commit spoke on success` and the stderr relay — byte-equal between the arms | matches (relay unchanged) |

### K · Cell 12 — run through cargo

| # | row | what it is | result |
|---|---|---|---|
| K1 | **fence row — classification only, confers no coverage** | `cargo test -p jigc --test g_finalize agent_co_author::` from the repository root (debug target, cargo's forced-empty variable) | exit 0 · `8 passed; 0 failed; 0 ignored; 465 filtered out` |
| K2 | the **equivalent-environment drive** | the *set, empty* arm on the installed binary — rows A3, A6, A9, A12, A15, A18, A21, A24, A27, A30, A33, B3, C3, C6, M3, J3, P18 | 0 trailers on every commit at every door |

---

## 3. Findings

Predicate: tier 1 = exit-0 loss or repository harm through a committing, destroying or moving door ·
tier 2 = a posture or route dead end · tier 3 = a surface says something the binary does not do.

### (R10, F1) — signing demotes a `Signed-off-by` git was reading, in a git-recognized mixed last paragraph

- **Door / cell:** `task finalize` · cell 4, row S11. **Exit 0, code none.**
- **Contract:** `design/assistant-adapter.md` → *Placement and de-duplication*: the trailer is placed *"appended
  to the message's trailer block when its last paragraph is one, else as a new block after one blank line —
  the place `git interpret-trailers` gives a trailer added at the end"*.
- **Observed:** when the message's last paragraph is one git itself reads as the trailer block although
  not every line is trailer-shaped (a `Signed-off-by:` line plus prose — git's recognized-prefix rule),
  jigc opens a **new** block. git then reads only the new block, so the `Signed-off-by` that was a trailer
  under the human arm is prose under the agent arm. `git interpret-trailers` on the same message appends
  into the existing block and keeps both.
- **Proposed tier: 3** — the placement sentence names git's placement and the binary differs from it in
  this shape; no byte is lost (the line is still in the message) and nothing dead-ends. Narrow: it needs a
  sign-off typed into the body slot with prose in the same paragraph; a sign-off authored as a `#trailers`
  item is unaffected (row S3 shape).

```sh
# setup
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
jigc start --workflow single-task "land the work"
jigc doc set-field commit:land-the-work#type --value feat
jigc doc set-field commit:land-the-work#scope --value cache
printf 'land the work\n' | jigc doc set-slot commit:land-the-work#summary --from-file -
printf 'A driven change.\n\nSigned-off-by: Pat Example <pat@example.com>\nwith one line of prose under it\n' \
  | jigc doc set-slot commit:land-the-work#body --from-file -
echo code > code.txt; git add code.txt
# argv, arm unset (control)            -> exit 0
env -u CLAUDECODE jigc task finalize land-the-work
git log -1 --format='%(trailers:only,unfold)'      # Signed-off-by: Pat Example <pat@example.com>
# what git itself does adding the trailer to THAT message (0 ambient trailer.* keys):
git log -1 --format=%B | git interpret-trailers --trailer "Co-Authored-By: Claude <noreply@anthropic.com>"
#   Signed-off-by: Pat Example <pat@example.com>
#   with one line of prose under it
#   Co-Authored-By: Claude <noreply@anthropic.com>        <- same block; git reads both
# argv, arm set (a second, identical rig)  -> exit 0
CLAUDECODE=1 jigc task finalize land-the-work
git log -1 --format=%B
#   feat(cache): land the work
#
#   A driven change.
#
#   Signed-off-by: Pat Example <pat@example.com>
#   with one line of prose under it
#
#   Co-Authored-By: Claude <noreply@anthropic.com>
git log -1 --format='%(trailers:only,unfold)'               # Co-Authored-By: Claude <noreply@anthropic.com>
git log -1 --format='%(trailers:key=Signed-off-by,valueonly)' | grep -c .    # 0
```

### (R10, F2) — the amend carry misses a `HEAD` trailer git reads inside a mixed block

- **Door / cell:** `task finalize (amend)` · cell 2, row C17. **Exit 0, code none.**
- **Contract:** *The amend arm*: *"carried over when `HEAD`'s message already carries the profile's trailer"*;
  the suite's own counting rule is git's parser.
- **Observed:** `HEAD`'s last paragraph is `Signed-off-by:` + the profile's `Co-Authored-By:` + one prose
  line. git counts 1 co-author before. A human's amend lands 0 — jigc's block reader accepts only an
  all-trailer paragraph, so it does not see the claim git sees. Tree and parent unchanged.
- **Proposed tier: 3** — the surface says *carried over*, the binary drops it in this shape; the result is
  an omission (never a false claim), and only for a hand-built `HEAD` message, since jigc's own renderer
  never emits a mixed block.

```sh
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
echo code > code.txt; git add code.txt
printf 'feat: hand commit\n\nBody.\n\nSigned-off-by: Pat Example <pat@example.com>\nCo-Authored-By: Claude <noreply@anthropic.com>\nand one prose line\n' > "$RIG/m.txt"
git commit -q -F "$RIG/m.txt"
git log -1 --format='%(trailers:key=Co-Authored-By,valueonly)' | grep -c .     # 1  (before-control)
env -u CLAUDECODE jigc task amend "repair the message"                         # exit 0
jigc doc set-field commit:repair-the-message#type --value fix
jigc doc set-field commit:repair-the-message#scope --value cache
printf 'the repaired summary\n' | jigc doc set-slot commit:repair-the-message#summary --from-file -
printf 'The repaired body.\n'   | jigc doc set-slot commit:repair-the-message#body --from-file -
env -u CLAUDECODE jigc task finalize repair-the-message                        # exit 0
git log -1 --format='%(trailers:key=Co-Authored-By,valueonly)' | grep -c .     # 0
# the same rig with CLAUDECODE=1 on both jigc calls: 1 -> 1
```

### (R10, F3) — `setup.profile-load`'s route blames the embedded profile for a directory-selected one

- **Door / cell:** `setup` · cell 5, rows P3–P12. **Exit 1, code `setup.profile-load`, blocking, route kind Human.**
- **Observed (the answer to "what does the install say, and under which code"):** the message names the
  fault exactly (*"the `co-author` `name` must be a non-empty single line without `<` or `>`"*, *"unknown
  field `extra`, expected one of `name`, `email`, `when-env` at line 59 column 3"*, *"Permission denied (os
  error 13)"*); nothing is written and no commit lands. The **route** is the same sentence in all ten:
  *"reinstall jigc — the embedded adapter profile is missing or malformed"* — but the profile that failed
  was read from `JIGC_ADAPTERS_DIR`, and the embedded one is intact (row P19 installs with it).
- **Proposed tier: 3** — a route that names a cause the binary did not have; reachable only through
  `JIGC_ADAPTERS_DIR`, which no adopter guide names (a test seam), and the route text predates the trailer
  (it is row 1's surface; the trailer adds six new ways to reach it). Recorded here because cell 5 asks.

```sh
rig=$(dev/jigc-rig bare --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
P=$(mktemp -d); cp <repo>/crates/cli/adapters/claude-code.yaml "$P/"
sed -i '' 's/^  name: Claude$/  name: ""/' "$P/claude-code.yaml"
JIGC_ADAPTERS_DIR="$P" CLAUDECODE=1 jigc setup --format json      # exit 1, stdout empty, stderr:
# { "schema_version": 3, "findings": [ { "severity": "blocking", "probe": "setup", "check": "profile-load",
#   "code": "setup.profile-load", "key": { "code": "setup.profile-load", "target": null },
#   "message": "cannot load the `claude-code` adapter profile: malformed adapter profile for `claude-code`: the `co-author` `name` must be a non-empty single line without `<` or `>`",
#   "location": null, "route": "reinstall jigc — the embedded adapter profile is missing or malformed" } ] }
git rev-list --count HEAD; git status --short                     # 1 ; (clean) — no install commit, nothing written
```

### (R10, F4) — the declared bound's recording route does not exist under `squash: true`, nor for a docs-only sub-task

- **Door / cell:** `milestone finalize` (both arms) · cell 6, rows G6 and G7. **Exit 0, code none.**
- **Contract:** `design/assistant-adapter.md` → *Declared bounds*: *"A human who finalizes work agents did
  … lands no trailer: an omission, never a false claim, and the commit doc's `#trailers` is the route to
  record it."*
- **Observed:** the omission holds as declared (G1, G2). The route holds for exactly one shape — a
  **code-carrying** sub-task under `squash: false` (G5: the per-sub-task commit carries the doc's
  trailer). Under **`squash: true`, the pack default**, the sub-task's commit doc exists
  (`jigc doc list --task area-low` lists `commit:area-low`), accepts the trailer item and its value at
  exit 0, and the boundary commit lands without it — the boundary message is CLI-synthesized and nothing
  says the item was not used. Under `squash: false` a **docs-only** sub-task's commit doc is likewise
  not rendered anywhere (its docs ride the aggregate).
- **Proposed tier: 3** — a design sentence names a route the binary honours in one of three shapes. Not
  tier 1: what is dropped is a transient sub-task commit doc that `design/finalize.md` already says has
  no place behind a `squash: true` boundary (*"there is no authored commit doc behind the boundary
  message in the first place"*) — the two design sentences contradict each other; no committed byte or
  repository state is harmed.

```sh
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
jigc config get finalize.fan-out.squash                 # finalize.fan-out.squash = true  (pack-default)
CLAUDECODE=1 jigc milestone create "Cache rework"
CLAUDECODE=1 jigc milestone add-task cache-rework "Area low"
CLAUDECODE=1 jigc milestone provision cache-rework
( cd .jigc/worktrees/area-low
  CLAUDECODE=1 jigc workflow sub-task --task area-low
  mkdir -p src; echo 'pub fn low() {}' > src/low.rs; git add src/low.rs
  jigc doc add-item commit:area-low#trailers --title Co-Authored-By --task area-low                       # exit 0
  jigc doc set-field commit:area-low#trailers/co-authored-by/value \
       --value "Claude <noreply@anthropic.com>" --task area-low                                            # exit 0
  jigc doc show commit:area-low --task area-low | tail -4 )
#   ### Co-Authored-By  {#co-authored-by}
#   <!-- fields -->
#   - value: Claude <noreply@anthropic.com>
env -u CLAUDECODE jigc milestone finalize cache-rework   # exit 0
#   finalized 8deaf90 — Finalize milestone cache-rework (2 sub-tasks)   (driven with a second, docs-only sub-task)
git log -1 --format=%B
#   Finalize milestone cache-rework (2 sub-tasks)
#
#   - area-docs
#   - area-low
git log -1 --format='%(trailers:key=Co-Authored-By,valueonly)' | grep -c .    # 0
```

### (R10, F5) — the composed amend step says the amended message carries *only* the trailers you add; it carries one more

- **Door / cell:** `task amend` (its composed surface) and `task finalize (amend)` · cell 2, rows C19, C20.
  **Exit 0, code none.**
- **Surface (printed by the installed binary on `jigc task amend`, line 58 of its output):** *"Trailers are
  re-authored too — the amended message carries only the trailer items you add here, so re-add any the
  old message had that still apply"*.
- **Observed:** over a `HEAD` that carries the profile's trailer, an amend that adds **no** trailer item
  lands the trailer (C19, by design — *The amend arm*), and an amend that adds only a different
  co-author lands both (C20). Neither `task validate` nor `task finalize` says a trailer was carried.
  Consequence worth the triager's eye: through jigc there is no way to land an amended message
  **without** the profile's trailer once `HEAD` has it — which is also the only jigc repair for the
  declared bound *"a command a human types into an agent's session … is signed as the agent's"*.
- **Proposed tier: 3** — a printed sentence the binary contradicts. (A tier-2 reading exists — the amend
  door cannot remove a co-author claim — but the design states the carry as intended and names no route
  for removal, so nothing printed dead-ends.)

```sh
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
CLAUDECODE=1 jigc start --workflow single-task "land the work"
# fill type/scope/summary/body as in F1 (body: "A driven change.")
echo code > code.txt; git add code.txt
CLAUDECODE=1 jigc task finalize land-the-work
git log -1 --format='%(trailers:key=Co-Authored-By,valueonly)' | grep -c .       # 1
env -u CLAUDECODE jigc task amend "repair the message" | grep -n "Trailers are re-authored"
#   58:Trailers are re-authored too — the amended message carries only the trailer items
# fill commit:repair-the-message type=fix scope=cache summary/body; add NO trailer item
env -u CLAUDECODE jigc task finalize repair-the-message                           # exit 0
git log -1 --format=%B
#   fix(cache): the repaired summary
#
#   The repaired body.
#
#   Co-Authored-By: Claude <noreply@anthropic.com>
# variant: before the finalize, add only
#   jigc doc add-item commit:repair-the-message#trailers --title Co-Authored-By
#   jigc doc set-field commit:repair-the-message#trailers/co-authored-by/value --value "Pat Example <pat@example.com>"
# -> Co-Authored-By: Pat Example <pat@example.com> / Co-Authored-By: Claude <noreply@anthropic.com>   (2)
```

### (R10, F6) — a bracket-less `Co-Authored-By: <address>` in the doc is not read as the same address *(marginal)*

- **Door / cell:** `task finalize` · cell 3, row D7. **Exit 0, code none.**
- **Contract:** *"A trailer block that already names the **same address** — whatever name it spells … —
  is left byte-identical"*; the scope's expectation: *one trailer per identity, by git's count*.
- **Observed:** a doc trailer `Co-Authored-By: noreply@anthropic.com` (the address with no name and no
  angle brackets) is followed by a second `Co-Authored-By: Claude <noreply@anthropic.com>`; git counts 2
  for the one address.
- **Proposed tier: 3**, and marginal — the design sentence does not say the address must be in angle
  brackets; a bracket-less value is not a `Name <address>` identity to git or to a forge either, so the
  added line is arguably the correct one. Filed so the reconciler can strike it.

```sh
# rig + fill as F1 (body "A driven change."), then:
jigc doc add-item commit:land-the-work#trailers --title Co-Authored-By
jigc doc set-field commit:land-the-work#trailers/co-authored-by/value --value "noreply@anthropic.com"
echo code > code.txt; git add code.txt
CLAUDECODE=1 jigc task finalize land-the-work                                  # exit 0
git log -1 --format='%(trailers:key=Co-Authored-By,valueonly)'
#   noreply@anthropic.com
#   Claude <noreply@anthropic.com>
```

---

## 4. Repro blocks for the rows that match (one per cell; argv differs only by door or arm)

### The helpers every block uses

```sh
J=~/.local/bin/jigc
arm() { a=$1; shift; case $a in set) CLAUDECODE=1 "$@";; unset) env -u CLAUDECODE "$@";; empty) CLAUDECODE= "$@";; esac; }
ntr() { git log -1 --format='%(trailers:key=Co-Authored-By,valueonly)' "${1:-HEAD}" | grep -c . ; }
rep() { for s in $(git rev-list --reverse "$1..HEAD"); do echo "$(git log -1 --format='%h %s' $s) co-authored-by(git)=$(ntr $s)"; done; }
fill() { $J doc set-field "commit:$1#type" --value "${2:-feat}"; $J doc set-field "commit:$1#scope" --value "${3:-cache}"
         printf '%s\n' "${4:-land the work}" | $J doc set-slot "commit:$1#summary" --from-file -
         printf '%s\n' "${5:-A driven change.}" | $J doc set-slot "commit:$1#body" --from-file - ; }   # add --task <id> in a sub-task
```

### Cell 1 / 8 — the door sweep

```sh
# setup — a `bare` rig per arm
rig=$(dev/jigc-rig bare --binary $J) || exit; eval "$rig"; [ -n "$REPO" ] || exit
pre=$(git rev-parse HEAD); ntr                           # 0  (the rig's own commit is plain git)
arm set $J setup; rep $pre; git log -1 --format=%B
#  [set]   exit 0  2be1b31 chore(jigc): install jigc workspace config co-authored-by(git)=1
#          chore(jigc): install jigc workspace config
#
#          Co-Authored-By: Claude <noreply@anthropic.com>
#  [unset] exit 0  537204f … co-authored-by(git)=0     %B = the subject alone
#  [empty] exit 0  5846843 … co-authored-by(git)=0

# task finalize — a `fresh` rig per arm (the rig's own install commit is agent-signed: it was built under
# the ambient variable; read only commits after `pre`)
rig=$(dev/jigc-rig fresh --binary $J) || exit; eval "$rig"; [ -n "$REPO" ] || exit
arm $A $J start --workflow single-task "land the work"; fill land-the-work
echo code > code.txt; git add code.txt; pre=$(git rev-parse HEAD)
arm $A $J task finalize land-the-work; rep $pre
#  [set]   exit 0  071fbb5 feat(cache): land the work co-authored-by(git)=1
#          %B: subject / blank / "A driven change." / blank / "Co-Authored-By: Claude <noreply@anthropic.com>"
#  [unset] exit 0  80b4c3f … =0        [empty] exit 0  4373b7c … =0

# rename — fresh rig; a committed ADR written by hand under docs/decisions/ (front-matter status/date/
# schema-version: 2; Context/Options/Decision/Consequences) and committed with plain git ("seed adr", 0 trailers)
arm $A $J rename adr:alpha-decision --to "Beta decision"
#  [set]   exit 0  3bf0a21 rename docs/decisions/alpha-decision.md -> docs/decisions/beta-decision.md =1
#          %B: subject / blank / "Repoint every persisted referrer in lockstep." / blank / the trailer
#  [unset] 9d03ff4 =0      [empty] 5af6774 =0

# migrate-corpus — the same ADR WITHOUT its schema-version line (the v0 state), committed with plain git
arm $A $J migrate-corpus
#  [set]   exit 0  c2e3602 chore(jigc): migrate the managed corpus to the current schema versions =1
#  [unset] 2447c00 =0      [empty] 5ec660c =0

# the record-only doors — fresh rig
arm $A $J milestone create "Cache rework"                       # chore(milestone): open record for milestone:cache-rework            1 / 0 / 0
arm $A $J milestone add-task cache-rework "Area low"            # chore(milestone): record task:area-low on milestone:cache-rework   1 / 0 / 0
arm $A $J task discard area-low --force                         # chore(milestone): discard task:area-low on milestone:cache-rework  1 / 0 / 0
arm $A $J milestone discard cache-rework                        # chore(milestone): discard record for milestone:cache-rework        1 / 0 / 0
# add-from-spec — a committed two-criteria spec at docs/specs/rate-limit.md (plain git), then
arm $A $J milestone create "Rate limit"; pre=$(git rev-parse HEAD)
arm $A $J milestone add-from-spec rate-limit spec:rate-limit; rep $pre
#  [set]  73d3222 chore(milestone): record task:rejects-the-101st-request on milestone:rate-limit =1
#         a72bc94 chore(milestone): record task:admits-within-the-window on milestone:rate-limit =1
#  [unset] 934fef4 =0, fd75542 =0     [empty] ab014ac =0, ee70ac5 =0

# migration shape — fresh rig; docs/direction.md written and committed with plain git
t=$(arm $A $J migrate docs/direction.md --as vision | sed -n 's/^task minted: //p')
$J doc author vision --from-file - --task "$t" <<'P'
title: Vision
sections:
  - id: thesis
    set: { thesis: "<<We build a deterministic context compiler.>>" }
  - id: invariants
    set: { invariants: "<<Structure belongs to the CLI; prose belongs to the model.>>" }
  - id: open-questions
    set: { open-questions: "<<Which domains earn a pack of their own.>>" }
P
# fill commit:$t (type docs, scope vision) with --task "$t"
arm $A $J task finalize "$t" --approve
#  [set] 7d1b072 docs(vision): migrate the direction doc into the managed vision =1 (A VISION.md; D docs/direction.md)
#  [unset] 292e621 =0     [empty] a21316d =0

# non-empty values: CLAUDECODE=0 / =false / =' ' jigc task finalize land-the-work  -> exit 0, =1 each
# tree equality: HEAD^{tree} after task finalize e18ad42b24 (set = unset = empty); rename 2cedbc2bd2; migrate-corpus e22086ab9b
# setup re-run: env -u CLAUDECODE jigc setup (=0); arm $A jigc setup again -> exit 0, HEAD unchanged;
#   git rm .jigc/AGENT.md && git commit (plain); arm $A jigc setup -> set: ed08948 chore(jigc): install jigc workspace config =1 ; unset: abda9b3 =0
# leaves not on the axis, CLAUDECODE=1, `committed-singletons` / `fresh` rigs: HEAD identical before and after each of
#   ingest(0) validate(0) upgrade(0) describe(0) "config set docs-root documentation"(0) "milestone list-tasks nope"(1)
#   "task discard throwaway-work --force"(0) "milestone provision cache-rework"(0) "milestone join cache-rework"(0)
#   "milestone execute cache-rework"(0) "uninstall --force"(0)
```

### Cell 6 — the fan-out fixture (built with the binary; used by rows A7–A12, G1–G9, H, I11–I12)

```sh
rig=$(dev/jigc-rig fresh --binary $J) || exit; eval "$rig"; [ -n "$REPO" ] || exit
arm $A $J config set finalize.fan-out.squash $SQ          # written to .jigc/config/manifest.yaml, uncommitted
arm $A $J milestone create "Cache rework"
arm $A $J milestone add-task cache-rework "Area low"
arm $A $J milestone add-task cache-rework "Area docs"
arm $A $J milestone provision cache-rework
( cd .jigc/worktrees/area-low;  arm $A $J workflow sub-task --task area-low
  mkdir -p src; echo 'pub fn low() {}' > src/low.rs; git add src/low.rs
  [ $SQ = false ] && fill area-low feat cache "rework the low cache path" "Low path body."      # with --task area-low
)
( cd .jigc/worktrees/area-docs; arm $A $J workflow sub-task --task area-docs
  $J doc create adr --title "Docs policy" --task area-docs
  # set-slot adr:docs-policy#context / #decision / #consequences --task area-docs
  [ $SQ = false ] && fill area-docs docs cache "record the docs policy" "Docs policy body."      # with --task area-docs
)
pre=$(git rev-parse HEAD); arm $B $J milestone finalize cache-rework; rep $pre
```

Observed (A = B, the sweep):

```text
squash=false set   exit 0   4eba2a8 feat(cache): rework the low cache path              =1   (src/low.rs)
                            a1201c7 Finalize milestone cache-rework (2 sub-tasks)       =1   (.jigc/config/manifest.yaml docs/decisions/docs-policy.md docs/milestone-records/cache-rework.md)
   %B of a1201c7:  Finalize milestone cache-rework (2 sub-tasks) / blank / "- area-docs" / "- area-low" / blank / Co-Authored-By: Claude <noreply@anthropic.com>
squash=false unset exit 0   82c52d6 =0   0c20c27 =0
squash=false empty exit 0   b5a2460 =0   ddeed0e =0
squash=true  set   exit 0   ddae8f0 Finalize milestone cache-rework (2 sub-tasks)       =1   (all four paths in one commit)
squash=true  unset exit 0   4d26537 =0
squash=true  empty exit 0   620dfd8 =0
```

Observed (A ≠ B):

```text
G1 squash=false  built set,   boundary unset  exit 0  1670166 feat(cache): rework the low cache path =0 ; 6e20a29 Finalize … =0
G3 squash=false  built unset, boundary set    exit 0  3c14d0b =1 ; aec0dc6 =1
G2 squash=true   built set,   boundary unset  exit 0  e60aaa8 =0
G4 squash=true   built unset, boundary set    exit 0  d48368a =1
G5 squash=false  built set, commit:area-low#trailers/co-authored-by = "Claude <noreply@anthropic.com>", boundary unset
                 exit 0  2b1fb13 feat(cache): rework the low cache path =1 ; 5a2a18c Finalize … =0
G6 squash=false  built set, commit:area-docs#trailers/co-authored-by = the same, boundary unset
                 exit 0  f615ae1 feat(cache): rework the low cache path =0 ; 607cb32 Finalize … =0     (F4)
G7 squash=true   built set, commit:area-low#trailers/co-authored-by = the same, boundary unset
                 exit 0  8deaf90 Finalize … =0                                                          (F4)
G8 squash=false  three sub-tasks (low, high code-carrying; docs), commit:area-high names "Claude Opus <noreply@anthropic.com>", boundary set
                 exit 0  38e99a4 feat(cache): rework the high cache path =1 [Claude Opus <noreply@anthropic.com>]
                         457b786 feat(cache): rework the low cache path  =1 [Claude <noreply@anthropic.com>]
                         e5cb8ea Finalize milestone cache-rework (3 sub-tasks) =1
G9 same, boundary unset  exit 0  5e87402 =1 [Claude Opus …] ; 20164d5 =0 ; 498f8ed =0
```

### Cell 7 — the doc-only path-scoped commit

```sh
rig=$(dev/jigc-rig fresh --binary $J) || exit; eval "$rig"; [ -n "$REPO" ] || exit
echo "someone else's work" > other.txt; mkdir -p src; echo 'fn other() {}' > src/other.rs; git add other.txt src/other.rs
T=guide-names-a-flag
arm $A $J start --workflow report-inconsistency "the guide names a flag the binary lacks"
$J doc create inconsistency --title "Guide flag drift" --task $T
$J doc set-field inconsistency:guide-flag-drift#meta/kind --value code-doc --task $T
for s in guide binary; do
  $J doc add-item inconsistency:guide-flag-drift#sides --title "side $s" --slug $s --task $T
  printf 'The %s says one thing.\n' $s | $J doc set-slot inconsistency:guide-flag-drift#sides/$s/says --from-file - --task $T
done
printf 'The guide names a flag the binary lacks.\n' | $J doc set-slot inconsistency:guide-flag-drift#description --from-file - --task $T
# fill commit:$T (type docs, scope findings, summary "file the guide flag drift") with --task $T
git diff --cached --name-status; git ls-files -s -- other.txt src/other.rs | shasum      # before
arm $A $J task finalize $T
git diff --cached --name-status; git ls-files -s -- other.txt src/other.rs | shasum      # after
git show --name-only --pretty=format: HEAD
```

```text
[set]   exit 0   finalized 21948ab — docs(findings): file the guide flag drift ; left-out: other.txt, src/other.rs
        staged before = after = [A other.txt; A src/other.rs]  entries digest 2347a2c8eab6 = 2347a2c8eab6
        commit files: docs/inconsistencies/guide-flag-drift.md      co-authored-by(git)=1
        %B: docs(findings): file the guide flag drift / blank / One finding, filed while other work is open. / blank / Co-Authored-By: Claude <noreply@anthropic.com>
[unset] exit 0   624d6a0  same staged set and digest, same one file   co-authored-by(git)=0
[empty] exit 0   ddebcff  same                                         co-authored-by(git)=0
```

### Cell 2 — amend

```sh
rig=$(dev/jigc-rig fresh --binary $J) || exit; eval "$rig"; [ -n "$REPO" ] || exit
# predecessor, one of:
#   (jigc)  arm $H $J start --workflow single-task "land the work"; fill land-the-work; echo code > code.txt; git add code.txt; arm $H $J task finalize land-the-work
#   (git)   echo code > code.txt; git add code.txt; git commit -q -F <message file>
t0=$(git rev-parse 'HEAD^{tree}'); p0=$(git rev-parse HEAD^); ntr           # BEFORE
arm $A $J task amend "repair the message"
fill repair-the-message fix cache "the repaired summary" "The repaired body."
arm $A $J task finalize repair-the-message
[ "$t0" = "$(git rev-parse 'HEAD^{tree}')" ] && [ "$p0" = "$(git rev-parse HEAD^)" ]; ntr      # AFTER
```

```text
C1  HEAD jigc/agent  x set    exit 0/0   1 -> 1   tree, parent unchanged   %B ends "Co-Authored-By: Claude <noreply@anthropic.com>"
C2  HEAD jigc/agent  x unset  exit 0/0   1 -> 1
C3  HEAD jigc/agent  x empty  exit 0/0   1 -> 1
C4  HEAD jigc/human  x set    exit 0/0   0 -> 1
C5  HEAD jigc/human  x unset  exit 0/0   0 -> 0   %B: fix(cache): the repaired summary / blank / The repaired body.
C6  HEAD jigc/human  x empty  exit 0/0   0 -> 0
C7  HEAD git "Co-Authored-By: Claude Opus <noreply@anthropic.com>"       x unset   1 -> 1  value now "Claude <noreply@anthropic.com>"
C8  same                                                                  x set     1 -> 1  same
C9  HEAD git "co-authored-by: Claude <NOREPLY@anthropic.com>"            x unset   1 -> 1  canonical line lands
C10 same                                                                  x set     1 -> 1
C11 HEAD git, address only in a prose paragraph                           x unset   0 -> 0
C12 same                                                                  x set     0 -> 1
C13 HEAD git "Co-Authored-By: Pat Example <pat@example.com>"             x unset   1 -> 0
C14 same                                                                  x set     1 -> 1  (Claude)
C15 HEAD git, the trailer in a non-last paragraph                         x unset   0 -> 0
C16 same                                                                  x set     0 -> 1
```

### Cell 3 / 4 — dedupe and shapes (`task finalize`, fresh rig per row)

```sh
rig=$(dev/jigc-rig fresh --binary $J) || exit; eval "$rig"; [ -n "$REPO" ] || exit
$J start --workflow single-task "land the work"
# type feat, scope cache, summary "land the work"; then the row's body and/or trailer items:
printf '%s\n' "$BODY" | $J doc set-slot commit:land-the-work#body --from-file -
$J doc add-item commit:land-the-work#trailers --title "$KEY" --format json      # ack .target.item = the slug of the key
$J doc set-field commit:land-the-work#trailers/<item>/value --value "$VALUE"
echo code > code.txt; git add code.txt
arm $A $J task finalize land-the-work
git log -1 --format='%(trailers:key=Co-Authored-By,valueonly)'; git log -1 --format=%B; git log -1 --format='%(trailers:only,unfold)'
```

```text
D1  set   exit 0  =1  Claude <noreply@anthropic.com>                         unset =1 (the doc's own)
D2  set   exit 0  =1  Claude Opus <noreply@anthropic.com>                    unset =1
D3  set   exit 0  =1  line "co-authored-by: Claude <noreply@anthropic.com>"  unset =1
D4  set   exit 0  =2  Pat Example <pat@example.com> / Claude <noreply@anthropic.com>     unset =1 (Pat)
D5  set   exit 0  =1  body "Pairing credit: Claude <noreply@anthropic.com> helped." then a block with the trailer    unset =0
D6  set   exit 0  =1  Claude <NOREPLY@ANTHROPIC.COM>
D7  set   exit 0  =2  noreply@anthropic.com / Claude <noreply@anthropic.com>                                   (F6)
D8  set   exit 0  =1  "Co-Authored-By: Claude <…>" / "Refs: TKT-1"
D9  set   exit 0  =1  "Signed-off-by: Claude <noreply@anthropic.com>" / "Co-Authored-By: Claude <noreply@anthropic.com>"
D10 set   exit 0  =2  "Co-Authored-By: Pat Example <pat@example.com>" / "Refs: TKT-2" / "Co-Authored-By: Claude <noreply@anthropic.com>"
D11 set   exit 0  =1  "CO-AUTHORED-BY: claude  < noreply@anthropic.com >"

S1  set   feat(cache): land the work / blank / Co-Authored-By: Claude <noreply@anthropic.com>                 =1   unset: subject only, =0
S2  set   … / A driven change. / blank / the trailer                                                          =1   unset =0
S3  set   … / A driven change. / blank / Refs: TKT-1 / the trailer   git reads [Refs; Co-Authored-By]         =1   unset git reads [Refs]
S4  set   … / See the ticket for the rationale. / Refs: TKT-1 / blank / the trailer   git reads [Co-Authored-By]   =1   unset git reads []
S5  set   … / Reviewed-by: Pat Example <pat@example.com> / the trailer   git reads both                       =1   unset git reads [Reviewed-by]
S6  set   … / Signed-off-by: Pat … / Co-Authored-By: Claude … / and one prose line / blank / the trailer      =1   unset =1 (git reads the mixed block)
S7  set   … / See: https://example.com/why / the trailer   git reads both                                     =1   unset git reads [See]
S8  set   … / BREAKING CHANGE: the cache key moved / Refs: TKT-9 / blank / the trailer                        =1   unset =0
S9  set   … / Co-Authored-By: Claude <noreply@anthropic.com>   (the body's own; not doubled)                  =1   unset =1
S10 set   … / Co-Authored-By: Claude … / blank / Refs: TKT-3 / Co-Authored-By: Claude …                        =1   unset =0, git reads [Refs]
S12 set   repo config trailer.ifexists=doNothing trailer.ifmissing=doNothing trailer.where=start trailer.co-authored-by.ifexists=replace
          … / A driven change. / blank / Refs: TKT-1 / Co-Authored-By: Claude <noreply@anthropic.com>         =1
```

### Cell 5 — the profile

```sh
P=$(mktemp -d); cp <repo>/crates/cli/adapters/claude-code.yaml "$P/"       # then edit the COPY (one variant per row)
rig=$(dev/jigc-rig bare --binary $J) || exit; eval "$rig"; [ -n "$REPO" ] || exit
JIGC_ADAPTERS_DIR="$P" CLAUDECODE=1 $J setup --format json
rig=$(dev/jigc-rig fresh --binary $J) || exit; eval "$rig"; [ -n "$REPO" ] || exit       # set up on the embedded profile
$J start --workflow single-task "land the work"; fill land-the-work; echo code > code.txt; git add code.txt
JIGC_ADAPTERS_DIR="$P" CLAUDECODE=1 $J task finalize land-the-work
```

```text
P1  control        setup exit 0, f2b7269 =1                         task finalize exit 0, 523aaf4 =1
P2  key absent     setup exit 0, 075246e =0                         task finalize exit 0, 8e93141 =0
P3  name ""        setup exit 1 setup.profile-load, no commit       task finalize exit 0, 4d8d1e7 =0
P4  name "Cl<aude" setup exit 1 setup.profile-load                  task finalize exit 0, 1e92843 =0
P5  email ws       setup exit 1 setup.profile-load ("… `email` must be a non-empty address without `<`, `>` or whitespace")   exit 0, e2ff11a =0
P6  1CLAUDE        setup exit 1 setup.profile-load ("… `when-env` must name an environment variable (`[A-Za-z_][A-Za-z0-9_]*`)")  exit 0, e5ed076 =0
P7  CLAUDE-CODE    setup exit 1 setup.profile-load (same message)   exit 0, 1cd5a3f =0
P8  when-env ""    setup exit 1 setup.profile-load (same message)   exit 0, 2108912 =0
P9  extra: x       setup exit 1 setup.profile-load ("co-author: unknown field `extra`, expected one of `name`, `email`, `when-env` at line 59 column 3")  exit 0, 31bef8b =0
P10 no email       setup exit 1 setup.profile-load ("co-author: missing field `email` at line 56 column 3")   exit 0, c010674 =0
P11 no file        setup exit 1 setup.profile-load ("cannot read adapter profile for `claude-code`: No such file or directory (os error 2)")  exit 0, 04b9166 =0
P12 mode 000       setup exit 1 setup.profile-load ("… Permission denied (os error 13)")   exit 0, 3390c7f =0
P13 Robo Example   setup exit 0, 96ba722 =1 [Robo Example <robo@example.com>]   task finalize exit 0, a2c315e =1 [same]
P14 when-env JIGC_R10_AGENT: that var set,   CLAUDECODE set     setup 4d1eac6 =1 ; task finalize ef92a77 =1
P15                          that var set,   CLAUDECODE unset   setup 4b9a53d =1 ; task finalize 01c23d8 =1
P16                          that var unset, CLAUDECODE set     setup c878e9f =0 ; task finalize 55dd98b =0
P17                          that var unset, CLAUDECODE unset   setup 8812b86 =0 ; task finalize 68c44db =0
P18                          that var set EMPTY, CLAUDECODE set setup 0c9a927 =0
P19 JIGC_ADAPTERS_DIR= (empty), CLAUDECODE set    setup e09f44f =1
P20 JIGC_ADAPTERS_DIR= (empty), CLAUDECODE unset  setup 0756354 =0
P21 amend over an agent-signed HEAD, profile-without-key at the amend finalize, unset   exit 0   1 -> 0
P22 same, set                                                                          exit 0   1 -> 0
```

In every malformed row (P3–P12) the install wrote nothing: `git rev-list --count HEAD` stayed 1 and
`git status --short` was empty.

### Cell 9 — rejection, then re-run

```sh
# <door fixture as in cell 1 / cell 6>, then:
H=$(mktemp -d); printf '#!/bin/sh\necho "policy: R10-REJECT" 1>&2\nexit 1\n' > "$H/pre-commit"; chmod 755 "$H/pre-commit"
git config core.hooksPath "$H"; pre=$(git rev-parse HEAD)
arm $A1 $J <door argv>          # exit 1; stderr carries "policy: R10-REJECT"; git rev-parse HEAD = $pre
git config --unset core.hooksPath
arm $A2 $J <door argv>; rep $pre
```

```text
task finalize            set->set 52780f6 =1    set->unset 2e71732 =0    unset->set fa0cab7 =1
task finalize (amend)    set->set 061733f =1    set->unset f4f34a9 =0    unset->set 81b74ba =1
rename                   set->set e7817c2 =1    set->unset dbb1712 =0    unset->set 48e1e19 =1
migrate-corpus           set->set 55f561d =1    set->unset 3c5262b =0    unset->set 50e5cab =1
milestone create         set->set db60b25 =1    set->unset 6b4c7a6 =0    unset->set df8b6d8 =1
milestone add-task       set->set df1d764 =1    set->unset 62beed8 =0    unset->set 24cc1fd =1
milestone add-from-spec  set->set 61d8d2f,b71c803 =1,1   set->unset 78fbb68,6006caf =0,0   unset->set 086c5ba,b166bac =1,1
milestone discard        set->set 39bd79c =1    set->unset 040a125 =0    unset->set 06aed80 =1
task discard             set->set 57e0884 =1    set->unset 25c3dbe =0    unset->set c78756d =1
milestone finalize true  set->set 6658496 =1    set->unset 4fbcde6 =0    unset->set f579754 =1
milestone finalize false set->set 2a0ebf7,3fa3b52 =1,1   set->unset 951fe45,7980fdc =0,0   unset->set fcd6f5f,a5ed4db =1,1
partial chain (hook: first commit passes, second rejected), squash=false:
   rejected under set   -> exit 1 milestone-finalize.chain-commit-rejected, no commit on HEAD ; re-run unset -> f7cae6c =0, e81abd6 =0
   rejected under unset -> exit 1 same                                                        ; re-run set   -> 8e989f4 =1, 87f0584 =1
```

### Cell 10 — pinned keys

```sh
# per door, the cell-1 / cell-6 fixture built twice; the act run with --format json under set and under unset;
# stdout and stderr normalized with  sed -E 's/[0-9a-f]{40}/<SHA40>/g; s/([^0-9a-f])[0-9a-f]{7}([^0-9a-f])/\1<SHA7>\2/g'
# and compared with cmp; `git log -1 --format=%s` compared as text.
```

```text
setup 239 B EQUAL · task-finalize 1015 B EQUAL · amend 282 B EQUAL · rename 295 B EQUAL · migrate-corpus 205 B EQUAL ·
ms-create 390 B EQUAL · ms-add-task 216 B EQUAL · task-discard 106 B EQUAL · ms-discard 105 B EQUAL ·
ms-add-from-spec 457 B EQUAL · ms-finalize-true 1317 B EQUAL · ms-finalize-false 1453 B EQUAL ; stderr empty and EQUAL in all twelve.
task finalize envelope keys: committed{displaced, files, hash, hook_output, left_out, manifest, promoted, subject}, findings, schema_version
  committed.subject = "feat(cache): land the work" in both arms ; %s identical in all twelve pairs.
```

### Cell 11 — the hook stream

```sh
H=$(mktemp -d); printf '#!/bin/sh\necho "HOOK-SAW-BEGIN" 1>&2\ncat "$1" 1>&2\necho "HOOK-SAW-END" 1>&2\nexit 0\n' > "$H/commit-msg"
chmod 755 "$H/commit-msg"; git config core.hooksPath "$H"
arm $A $J task finalize land-the-work --format json        # the doc carries a `Refs: TKT-1` trailer item
```

```text
[set]   exit 0   committed.hook_output and stderr ("--- hook output ---"):
        HOOK-SAW-BEGIN / feat(cache): land the work / (blank) / A driven change. / (blank) / Refs: TKT-1 /
        Co-Authored-By: Claude <noreply@anthropic.com> / HOOK-SAW-END                         landed =1
[unset] exit 0   the same without the Co-Authored-By line                                     landed =0
[empty] exit 0   the same without it                                                          landed =0
amend            set: handed message's last line = the trailer, landed =1 ; unset: last line "The repaired body.", =0
milestone create set: last line = the trailer, =1 ; unset: last line = the subject, =0
rename           set: last line = the trailer, =1 ; unset: last line "Repoint every persisted referrer in lockstep.", =0
commit-msg hook `git interpret-trailers --in-place --trailer "Change-Id: I0123456789abcdef" "$1"`, set:
        exit 0   … / A driven change. / blank / Co-Authored-By: Claude <noreply@anthropic.com> / Change-Id: I0123456789abcdef   =1, git reads both
chatty-hooks rig, task finalize --format json:
        set and unset: committed.hook_output = "chatty-hook: pre-commit spoke on success"; stderr "--- hook output ---" + that line;
        both byte-equal between the arms (set landed =1, unset =0)
```

---

## 5. Not driven — stated, never presented as driven

1. **A human typing into an agent's session** (the design's own *not driven*): a driver cannot be a
   human. NOT DRIVEN.
2. **Run through cargo on the installed binary** (cell 12): no such drive exists — `cargo run` is
   forbidden and cargo does not run the installed binary. Row K1 is a fence (debug target, no coverage);
   K2 cites the *set, empty* arm.
3. **Cell 5 at the other ten doors.** The profile variants were driven at `setup` and `task finalize`
   (and *absent* at the amend). `rename`, `migrate-corpus`, both `milestone finalize` arms and the five
   record-only doors were **not** driven against an edited profile; they share the seam's one profile
   load by a source read only.
4. **Cell 9 on the doc-only finalize and on `setup`.** `setup`'s install commit is `--no-verify`, so no
   hook can reject it (a source read; not driven). The doc-only finalize was not driven under a rejecting
   hook.
5. **Cell 10 for the doc-only finalize and the migration finalize envelopes** — not compared.
6. **Cell 11's echoing hook on `migrate-corpus`, both `milestone finalize` arms, `milestone add-task`,
   `add-from-spec`, `milestone discard`, `task discard` and the doc-only finalize** — not driven (driven
   on `task finalize`, the amend, `milestone create`, `rename`).
7. **Cells 3 and 4 on the doc-only commit, the amend and the per-sub-task chain** beyond rows C20, G5,
   G8, G9 — the dedupe and shape matrices were driven on the ordinary `task finalize` only.
8. **The *set, empty* arm of cells 3, 4, 5, 9 and 10** — those cells were driven set/unset; the empty arm
   was driven at every door in cell 1 and at cells 2, 6, 7 and 11's first row.
9. **Any other git or platform** — git 2.54.0 (Apple Git-157) on darwin only; no Linux container, no
   `dev/runner-faithful` drive. The trailer-block findings (F1, F2) rest on this git's recognized-prefix
   rule.
10. **A non-ASCII or CRLF message, a multi-line trailer value, `commit.cleanup`/`core.commentChar`
    ambient config** — not driven.

`relocate`, `unmanage`, `doc rename` and `migrate` (the mint) are not doors of this row: by a source
read none of them commits (the in-task rename lands at `task finalize`). `migrate` ran as fixture
construction only.

---

## 6. Observations that are not defects

- **O-1** An amend over a hand-typed `Co-Authored-By: Claude Opus <noreply@anthropic.com>` lands
  `Claude <noreply@anthropic.com>` — the claim is carried by address and re-rendered with the profile's
  name (C7–C10). Every other trailer `HEAD` had is dropped by the amend arm's own from-scratch contract
  (C13).
- **O-2** In shapes S6 and S10 the raw message carries the string `Co-Authored-By: Claude <…>` twice
  (once in body prose, once as the trailer) while git's parser counts one. The repository's own suite
  also asserts a raw-substring count of one; these two shapes are outside its fixtures.
- **O-3** `CLAUDECODE=0`, `=false` and a single space all sign as the agent — exactly *set and
  non-empty*. A human who exports a falsy-looking value is signed as the agent.
- **O-4** `committed.hook_output` carries whatever a hook prints, so a hook that echoes its message file
  makes that key differ between the arms. No pinned key carries the message by itself (cell 10).
- **O-5** A rig is born with an agent-signed install commit (`dev/jigc-rig fresh` under the ambient
  variable: `chore(jigc): install jigc workspace config | Claude <noreply@anthropic.com>`); every count
  above reads only commits after a recorded pre-act `HEAD`.
- **O-6** Under `squash: false` a docs-only sub-task gets no commit of its own (`sub-tasks: area-docs: 2
  docs · area-low: 1 doc, 1 code file, committed as …`), and the uncommitted
  `.jigc/config/manifest.yaml` the `config set` wrote rode the boundary commit. Both are row 5's and row
  8's subjects, not this row's; named so they are not re-filed here.
- **Neighbour, not re-filed:** `(2, DEFECT C)` lives on the same seam and was not driven.

## 7. Driver errors, recorded

- One `env` argv-order mistake (`env VAR=1 -u OTHER …`) produced exit 127 from `env(1)` for the
  *JIGC_R10_AGENT unset × CLAUDECODE set* cell; jigc never ran. Re-driven correctly (row P16).
- One script lost the rig's `cd` inside a pipeline subshell, so `git rev-parse HEAD`,
  `jigc milestone finalize cache-rework` (×6), `jigc doc list` (×2) and `jigc doc add-item` ran with the
  **working repository** as cwd. Every one of those jigc calls exited 1 and landed nothing (the messages
  captured: *this project isn't set up — run `jigc setup`* and *no task `area-low`*). Checked immediately afterwards: `git status --short` empty, `HEAD` unchanged, no `.jigc/`
  directory. A cwd guard (refuse unless `git rev-parse --show-toplevel` is a rig) was added to every
  helper and the cells were re-driven; the rows in this record are from the re-drive.
- The first squash-true route drive added the trailer item without its value; it is not counted. Row G7
  is the complete one.

---

## 8. Baseline rows: CLOSED / STILL-OPEN

**None — this row has no baseline (first drive).** Named neighbour `(2, DEFECT C)`: NOT RE-DRIVEN (not
this row's; lives on numbered axis 2).

## 9. Counts

- Doors: 12 (11 `COMMITTING_DOORS` rows over 9 clap leaves + the install commit) — equal to the
  instrument's count; plus one commit shape it does not name (the migration finalize).
- Rows driven: **237** (A 33 · A-extras 24 · C 20 · D 16 · E 23 · F 39 · G 9 · B 3 · H 46 · I 12 · J 12).
- Rows not driven: 10 groups (§5) + 1 fence row (K1).
- Defects: 6, all proposed tier 3. **Tier-1 rows: 0.**

---

# Reconciliation ledger

- **Binary:** `~/.local/bin/jigc`, `jigc --version` printed `jigc 1.0.0-rc.24` (asserted at the top of every script). git 2.54.0 (Apple Git-157), darwin.
- **Environment (state only):** `CLAUDECODE` set, non-empty (ambient; every cell below sets or removes it explicitly) · `JIGC_ADAPTERS_DIR` unset (removed in every script, set only where a row names it) · `JIGC_PACK_DIR` unset.
- **Rigs:** `rig=$(dev/jigc-rig <state> --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit`, stdout only, plus a cwd guard (refuse when `git rev-parse --show-toplevel` is the working repository) before every git command. Scratch roots from `mktemp -d`; no teardown. The working repository was checked after the last drive: `HEAD` unchanged, `git status --short` empty, no `.jigc/`.
- **The rule applied:** a claim by one that the other cannot reproduce is a lead, not a finding. Every Codex claim was entered as `lead(codex, …)` and driven; every driver defect was re-driven once.
- **Inputs:** the source pass is `axis10-codex.md` (exit 0). A second, higher-effort Codex run for this row exited 1 on a usage limit and produced **no report** — it contributes no claim and nothing here rests on it.

**Verdict on the exit rule after reconciliation: no tier-1 row.** Seven confirmed findings, all tier 3: the driver's six (all reproduce) and one of Codex origin. Two of Codex's *carried* dispositions are refuted by driven repros (F1, F2). Zero driver rows demoted.

Helpers used by the reconciler's blocks (same as the driver's §4):

```sh
J=~/.local/bin/jigc
arm() { a=$1; shift; case $a in set) CLAUDECODE=1 "$@";; unset) env -u CLAUDECODE "$@";; empty) CLAUDECODE= "$@";; esac; }
ntr() { git log -1 --format='%(trailers:key=Co-Authored-By,valueonly)' "${1:-HEAD}" | grep -c . ; }
rep() { for s in $(git rev-list --reverse "$1..HEAD"); do echo "$(git log -1 --format='%h %s' $s) co-authored-by(git)=$(ntr $s)"; done; }
fill() { $J doc set-field "commit:$1#type" --value "${2:-feat}"; $J doc set-field "commit:$1#scope" --value "${3:-cache}"
         printf '%s\n' "${4:-land the work}" | $J doc set-slot "commit:$1#summary" --from-file -
         printf '%s\n' "${5:-A driven change.}" | $J doc set-slot "commit:$1#body" --from-file - ; }
```

## 1. Registry check — the driver's door-set count against the code

| registry | what the code carries (read by the reconciler) | driver's count | verdict |
|---|---|---|---|
| `COMMITTING_DOORS` (`crates/cli/src/invocation_log.rs`) | 11 `CommittingDoor` rows, `verb:` = `jigc task finalize` · `jigc task finalize (amend)` · `jigc milestone finalize (squash: true)` · `jigc milestone finalize (squash: false)` · `jigc rename` · `jigc migrate-corpus` · `jigc milestone create` · `jigc milestone add-task` · `jigc milestone add-from-spec` · `jigc milestone discard` · `jigc task discard` — 9 distinct `VERB_KINDS` leaves | 11 over 9 | **equal** |
| the exclusion (`setup.rs` → `commit_install`) | 1 | 1 | equal |
| doors | 12 over 10 clap leaves (`setup` is the tenth) | 12 | equal |
| `StagePolicy` (`task.rs`) | 5 variants: `MigrationFixed`, `IndexHonoring`, `Amend`, `DocOnly`, `Combine` | 5 (the driver's addition over the instrument's three shapes) | equal |
| `CoAuthorBasis` (`task.rs`) | 2: `Session`, `SessionOrHead` | 2 | equal |
| `RawCoAuthor` (`adapter.rs`) | 3 fields, `deny_unknown_fields` | 3 | equal |
| row count | A 33 + A-extras 24 (M 3 · V 3 · T 3 · R 4 · N 11) + C 20 + D 16 + E 23 + F 39 + G 9 + B 3 + H 46 (33 + 2 + the 11 code rows) + I 12 + J 12 | 237 | **equal** (re-added) |

## 2. Rows checked for a repro block (the demotion check)

Every row the driver marks driven carries a repro: a full block (§3 defects; §4 cells 1/8, 6, 7, 2, 3/4, 5, 9, 10, 11) or a compressed line inside a block (argv, exit, asserted surface). **None is demoted.** The compressed ones were the candidates, so the reconciler re-drove them; each reproduced:

| driver rows | what was compressed | reconciler re-drive | result |
|---|---|---|---|
| N1–11 (leaves not on the axis) | fixture named only as "`committed-singletons` / `fresh` rigs" | block R-N below | all 11 reproduce: `HEAD` unchanged, exits 0 except `milestone list-tasks nope` = 1 |
| V1–3 | one comment line | block R-V | 1 · 1 · 1 |
| R1–4 | one comment line | block R-R | no commit · no commit · 1 · 0 |
| T1–3 (the `task finalize` third) | tree hashes only | block R-V | tree `e18ad42b24` in all three arms — the driver's own hash |
| H34/H35 (partial chain) | the hook that passes commit 1 and rejects commit 2 was not printed | block R-H | exit 1 `milestone-finalize.chain-commit-rejected`, 2 hook invocations, `HEAD` unchanged; re-run 0,0 / 1,1 |
| I2, I6 (sample of I1–12) | block described in comments | block R-I | stdout byte-equal sha-normalized (1015 B, 390 B); `%s` identical; no envelope line names the trailer |
| J1/J2 (sample of J1–12) | — | block R-I | the message the `commit-msg` hook is handed ends in the trailer under set, in `A driven change.` under unset |
| G2, G5, G6, G7 | observed lines | block F4 below | reproduce |
| K1 | a fence row — the driver itself classes it as conferring no coverage | not re-run | stays a classification |

```sh
# R-N — fresh rig, every act under CLAUDECODE=1; pre=$(git rev-parse HEAD) before, compared after
jigc ingest                                   # exit 0  HEAD unchanged
jigc validate                                 # exit 0  HEAD unchanged
jigc upgrade                                  # exit 0  HEAD unchanged
jigc describe                                 # exit 0  HEAD unchanged
jigc config set docs-root documentation       # exit 0  HEAD unchanged
jigc milestone list-tasks nope                # exit 1  HEAD unchanged
jigc start --workflow single-task "throwaway work"; jigc task discard throwaway-work --force   # exit 0  HEAD unchanged
jigc milestone create "Cache rework"; jigc milestone add-task cache-rework "Area low"          # (fixture; these two commit)
jigc milestone provision cache-rework         # exit 0  HEAD unchanged
jigc milestone execute cache-rework           # exit 0  HEAD unchanged
( cd .jigc/worktrees/area-low; jigc workflow sub-task --task area-low; mkdir -p src; echo 'pub fn low() {}' > src/low.rs; git add src/low.rs )
jigc milestone join cache-rework              # exit 0  HEAD unchanged
# a second fresh rig:
jigc uninstall --force                        # exit 0  HEAD unchanged
# reconciler additions, `committed-singletons` rig, CLAUDECODE=1 (leaves the driver classed by a source read only):
jigc unmanage VISION.md                       # exit 0  HEAD unchanged
jigc relocate vision --from docs/vision.md    # exit 1 (relocate.frozen-doctype)  HEAD unchanged — refusal path only
jigc migrate docs/direction.md --as vision    # exit 0  HEAD unchanged  (docs/direction.md seeded with plain git)
jigc task list; jigc doc list; jigc config list   # exit 0 each  HEAD unchanged

# R-V — fresh rig per value; start + fill + `echo code > code.txt; git add code.txt`
CLAUDECODE=0 jigc task finalize land-the-work       # exit 0  co-authored-by(git)=1
CLAUDECODE=false jigc task finalize land-the-work   # exit 0  =1
CLAUDECODE=' ' jigc task finalize land-the-work     # exit 0  =1
# tree equality, same fixture: set e18ad42b24 =1 · unset e18ad42b24 =0 · empty e18ad42b24 =0

# R-R — bare rig per arm
arm $A jigc setup          # set: exit 0 =1          unset: exit 0 =0
arm $A jigc setup          # exit 0, HEAD unchanged (both arms)
git rm -q .jigc/AGENT.md; git commit -q -m "remove an install file"
arm $A jigc setup          # set: exit 0, a new `chore(jigc): install jigc workspace config` =1     unset: exit 0, the same subject =0
# reconciler addition (the driver's §5 item 4, "not driven"): setup under rejecting hooks
H=$(mktemp -d); printf '#!/bin/sh\necho "policy: R10-REJECT" 1>&2\nexit 1\n' > "$H/pre-commit"; cp "$H/pre-commit" "$H/commit-msg"; chmod 755 "$H"/*
git config core.hooksPath "$H"
arm $A jigc setup          # set: exit 0, install commit lands =1 ; unset: exit 0, lands =0 ; the hook's marker appears nowhere (the commit is --no-verify)

# R-H — the cell-6 fixture, squash=false, one code-carrying + one docs-only sub-task, both commit docs filled, built under $A1
H=$(mktemp -d)
printf '#!/bin/sh\nn=$(cat "%s/count" 2>/dev/null || echo 0); n=$((n+1)); echo $n > "%s/count"\nif [ $n -ge 2 ]; then echo "policy: R10-REJECT-SECOND" 1>&2; exit 1; fi\nexit 0\n' "$H" "$H" > "$H/pre-commit"; chmod 755 "$H/pre-commit"
git config core.hooksPath "$H"; pre=$(git rev-parse HEAD)
arm $A1 jigc milestone finalize cache-rework --format json    # exit 1 · code milestone-finalize.chain-commit-rejected · count file = 2 · marker on stderr · HEAD = $pre
git config --unset core.hooksPath
arm $A2 jigc milestone finalize cache-rework; rep $pre
#   A1=set,   A2=unset: exit 0  feat(cache): rework the low cache path =0 ; Finalize milestone cache-rework (2 sub-tasks) =0
#   A1=unset, A2=set:   exit 0  =1 ; =1

# R-I — per arm a fresh rig; stdout normalized with  sed -E 's/[0-9a-f]{40}/<SHA40>/g; s/([^0-9a-f])[0-9a-f]{7}([^0-9a-f])/\1<SHA7>\2/g'
arm $A jigc task finalize land-the-work --format json     # set: 1015 B, =1 ; unset: 1015 B, =0 ; cmp: equal ; stderr 0 B both ; "subject": "feat(cache): land the work" ; %s identical
arm $A jigc milestone create "Cache rework" --format json # set: 390 B, =1 ; unset: 390 B, =0 ; cmp: equal ; %s identical
# hook stream: core.hooksPath -> a commit-msg hook that prints HOOK-SAW-BEGIN, `cat "$1"`, HOOK-SAW-END on stderr
arm $A jigc task finalize land-the-work
#   set:   exit 0 =1 ; relay under "--- hook output ---": subject / blank / A driven change. / blank / Co-Authored-By: Claude <noreply@anthropic.com>
#   unset: exit 0 =0 ; relay ends "A driven change."
```

## 3. Driver defects — each re-driven once

| key | door | reproduces | tier | why this tier, and the tier-1 test in both directions |
|---|---|---|---|---|
| (R10, F1) | `task finalize` | **yes** — and **wider than the driver recorded** (datum X2 below) | **3** | Exit 0 is shown. The loss half is **missing**: the `Signed-off-by` line is still in `%B` after the act (before-control and after-control both find the bytes); what changes is that git's trailer parser stops reading it. No committed byte, tree, parent or index entry is lost or damaged. The design's placement sentence names git's placement and the binary differs from it in this shape → a surface says what the binary does not do. |
| (R10, F2) | `task finalize (amend)` | **yes** | **3** | Exit 0 is shown, and a before-control finds one co-author trailer where the after-control finds none — but that is not the predicate's loss: the amend door's declared function is to **replace** `HEAD`'s message (the composed step says trailers are re-authored, and the amend finalize's own success lines say *"the message is re-authored"* and *"the superseded commit stays reachable in the reflog"* — the reconciler read `git cat-file -t <old sha>` = `commit` after the act), the tree and parent are unchanged, and the only thing the design promises to keep is this one credit. The promise (*carried over*) is missed in a shape git reads and jigc's block reader does not → tier 3. The driver's tier stands; argued the other way (a credit silently dropped through a committing door) it still fails the loss half, because the dropped bytes are message bytes the door was asked to rewrite. |
| (R10, F3) | `setup` | **yes** (name `""` and mode 000) | **3** | Exit **1**, blocking, nothing written (`git rev-list --count HEAD` 1 → 1, clean worktree) — the exit-0 half is missing, so not tier 1. Not a dead end either: the message names the fault exactly and the embedded profile installs (control: the same rig, `JIGC_ADAPTERS_DIR=` empty → exit 0, =1). The route sentence blames the embedded profile for a directory-selected one → tier 3. |
| (R10, F4) | `milestone finalize` (both arms) | **yes** (G7, G6; controls G5, G2) | **3** | Exit 0 is shown, and an item authored through `jigc doc add-item`/`set-field` (before-control: `jigc doc show commit:area-low --task area-low` prints it) reaches no commit. Tested toward tier 1: what is dropped is one item of a **transient** sub-task commit doc whose every byte (type, scope, summary, body) is unused behind a `squash: true` boundary by `design/finalize.md`'s own statement; no committed byte and no repository state is harmed, so the loss half is missing. Tested toward tier 2 (a route dead end): the reconciler drove a **working jigc route** to record the credit afterwards — `jigc task amend` over the boundary commit with a `Co-Authored-By` trailer item lands it (0 → 1, tree unchanged; block F4-route) — so the state is not a dead end. The design's *Declared bounds* sentence names `#trailers` as the route and the binary honours it in one of three shapes → tier 3. |
| (R10, F5) | `task amend` / `task finalize (amend)` | **yes** (C19 and C20) | **3** | Exit 0; nothing lost — a trailer is **added** beyond what the printed step says. The step text (*"carries only the trailer items you add here"*) is contradicted by the binary → tier 3. The tier-2 reading (no jigc path lands an amended message without the profile's trailer once `HEAD` has it) is real as a behaviour — re-driven: a human amend that adds no item lands 1 → 1, and neither `task validate` nor `task finalize` prints a line naming a trailer — but the carry is the design's stated intent and no printed route promises removal, so nothing printed dead-ends. |
| (R10, F6) | `task finalize` | **yes** | **3**, marginal | Exit 0; nothing lost — git counts 2 for one address when the doc's value is the bare address without angle brackets. A strike candidate at triage, as the driver says: a bracket-less value is not a `Name <address>` identity. |

### Datum X2 — F1 is reachable from `#trailers` items alone (the driver's "narrow" sentence does not hold)

The driver's F1 says *"a sign-off authored as a `#trailers` item is unaffected"*, and `trailer_block`'s own doc comment says it reads *"the all-trailer shape jigc's own renderer emits"*. Driven: `jigc doc add-item … --title` accepts a key that is not a git trailer token (it refuses whitespace and a colon — `BREAKING CHANGE` is refused with `schema-conformance.field-value-conformant` — but accepts an underscore), so jigc's **own renderer** emits a git-recognized mixed block from items alone, and the signing then demotes the sign-off item. Same tier (3), same reason; the reach is wider than a body-typed sign-off.

```sh
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
jigc start --workflow single-task "land the work"; fill land-the-work
jigc doc add-item commit:land-the-work#trailers --title "Signed-off-by"                                          # exit 0
jigc doc set-field commit:land-the-work#trailers/signed-off-by/value --value "Pat Example <pat@example.com>"     # exit 0
jigc doc add-item commit:land-the-work#trailers --title "Reviewed_by" --format json                               # exit 0, "item": "reviewed"
jigc doc set-field commit:land-the-work#trailers/reviewed/value --value "Sam Example <sam@example.com>"          # exit 0
echo code > code.txt; git add code.txt
# arm unset                                             -> exit 0
env -u CLAUDECODE jigc task finalize land-the-work
git log -1 --format='%(trailers:only,unfold)'           # Signed-off-by: Pat Example <pat@example.com>     (git reads the sign-off)
# arm set (a second, identical rig)                     -> exit 0
CLAUDECODE=1 jigc task finalize land-the-work
git log -1 --format=%B
#   feat(cache): land the work
#
#   A driven change.
#
#   Signed-off-by: Pat Example <pat@example.com>
#   Reviewed_by: Sam Example <sam@example.com>
#
#   Co-Authored-By: Claude <noreply@anthropic.com>
git log -1 --format='%(trailers:only,unfold)'           # Co-Authored-By: Claude <noreply@anthropic.com>   (the sign-off is no longer read)
git log -1 --format='%(trailers:key=Signed-off-by,valueonly)' | grep -c .     # 0
# controls: `--title "BREAKING CHANGE"` -> exit 1, schema-conformance.field-value-conformant, "the trailer key contains whitespace";
#           a value carrying a newline  -> exit 1, schema-conformance.field-value-conformant, "must not contain control characters"
```

### The re-drives, as run

```sh
# F1 — the driver's block, both arms, a fresh rig each
#   unset: exit 0 ; git reads [Signed-off-by: Pat Example <pat@example.com>] ; signed-off-by(git)=1 co-authored-by(git)=0
#          `git log -1 --format=%B | git interpret-trailers --trailer "Co-Authored-By: Claude <noreply@anthropic.com>"` appends
#          the line INTO the paragraph (after "with one line of prose under it"); 0 ambient trailer.* keys
#   set:   exit 0 ; a new block after one blank line ; git reads [Co-Authored-By: Claude <noreply@anthropic.com>] ; signed-off-by(git)=0 co-authored-by(git)=1

# F2 — the driver's block
#   unset: plain commit =1 (before-control) -> amend exit 0, finalize exit 0 -> =0 ; tree unchanged, parent unchanged, sha moved ;
#          `git cat-file -t <superseded sha>` = commit
#   set:   =1 -> =1

# F3 — the driver's block; stdout 0 B; stderr the findings envelope with code setup.profile-load and
#   "route": "reinstall jigc — the embedded adapter profile is missing or malformed"; commits 1 -> 1; status empty.
#   control, same rig: JIGC_ADAPTERS_DIR= CLAUDECODE=1 jigc setup -> exit 0, =1.   mode 000: exit 1, same code, same route, "Permission denied (os error 13)"

# F4 — the driver's cell-6 fixture (one code-carrying + one docs-only sub-task), sub-tasks built under set, boundary under unset
#   squash=true  (pack default), commit:area-low#trailers/co-authored-by = "Claude <noreply@anthropic.com>" (add-item exit 0, set-field exit 0,
#                `jigc doc show commit:area-low --task area-low` prints the value)
#                env -u CLAUDECODE jigc milestone finalize cache-rework -> exit 0 ; Finalize milestone cache-rework (2 sub-tasks) =0 ; 0 output lines name a trailer   (G7)
#   squash=false, same item on area-low  -> exit 0 ; feat(cache): rework the low cache path =1 ; Finalize … =0                                             (G5)
#   squash=false, the item on area-docs  -> exit 0 ; feat(cache): rework the low cache path =0 ; Finalize … =0                                             (G6)
#   squash=true,  no item                -> exit 0 ; Finalize … =0                                                                                          (G2)
# F4-route — the reconciler's tier-2 test: is there a jigc route after a squash boundary a human ran?
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
CLAUDECODE=1 jigc milestone create "Cache rework"; CLAUDECODE=1 jigc milestone add-task cache-rework "Area low"; CLAUDECODE=1 jigc milestone provision cache-rework
( cd .jigc/worktrees/area-low; CLAUDECODE=1 jigc workflow sub-task --task area-low; mkdir -p src; echo 'pub fn low() {}' > src/low.rs; git add src/low.rs )
env -u CLAUDECODE jigc milestone finalize cache-rework          # exit 0 ; Finalize milestone cache-rework (1 sub-task) =0
env -u CLAUDECODE jigc task amend "credit the agent"            # exit 0
fill credit-the-agent feat cache "rework the cache" "Finalize milestone cache-rework."
jigc doc add-item commit:credit-the-agent#trailers --title Co-Authored-By                                              # exit 0
jigc doc set-field commit:credit-the-agent#trailers/co-authored-by/value --value "Claude <noreply@anthropic.com>"     # exit 0
env -u CLAUDECODE jigc task finalize credit-the-agent           # exit 0 ; =1 ; tree unchanged

# F5 — the driver's block: agent-signed HEAD (=1) ; `env -u CLAUDECODE jigc task amend "repair the message"` prints at line 58
#   "Trailers are re-authored too — the amended message carries only the trailer items / you add here, so re-add any the old message had that still apply:"
#   no item added -> `task validate` exit 0 (0 lines name a trailer) -> `task finalize` exit 0 (0 lines name a trailer) -> =1, %B ends in the trailer
#   variant: a second amend adding only Pat Example <pat@example.com> -> exit 0 ; =2 (Pat Example, then Claude)

# F6 — the driver's block: exit 0 ; git reads  noreply@anthropic.com  and  Claude <noreply@anthropic.com> ; =2
```

## 4. Codex claims — each entered as a lead, then driven

### lead(codex, 1) — a commit is attributed to the profile the commit-time environment selects, not the one `jigc setup` installed from → **CONFIRMED** as **(R10, F7)**, origin codex, **tier 3 (marginal)**

- **Door:** `task finalize` (also driven at `milestone create`, `milestone add-task`, `task finalize (amend)` and after a hook rejection). **Exit 0, code none.**
- **Contract:** `design/assistant-adapter.md` → *Which profile*: *"The one `jigc setup` installs"*.
- **Observed:** the co-author is whatever profile the process environment selects at the moment of each commit. Setup under a profile declaring `Agent A <a@example.com>`, then a finalize under one declaring `Agent B <b@example.com>`, lands `Agent B`; the same repository's next door with no directory set lands the embedded `Claude <noreply@anthropic.com>`. Nothing the install writes records the identity (control: zero files under the tracked tree, `.jigc/` or `.claude/` carry the installed address — `command grep` under `.jigc/`).
- **Tier, tested in both directions.** Codex calls it *"relevant to the tier-1 exit rule"*. **That half is REFUTED:** exit 0 is shown, but there is **no loss and no repository harm** — the tree, the parent and the index are exactly what the door was asked for, no byte a before-control finds is absent afterwards, and the trailer's value is the one the profile in force declares. The swap needs `JIGC_ADAPTERS_DIR` in jigc's own environment, a seam no adopter guide names (it appears in `DECISIONS.md`, the roadmap and the crate changelog only), and whoever controls that environment already controls the commit's author and committer through git's own variables. Not tier 2: nothing dead-ends. **Tier 3**, and marginal: the design sentence says *the one setup installs* while the binary re-reads the source per process — and the same bullet already says no per-repo record of the choice exists. A strike candidate at triage, kept because it reproduces.
- **The derived rows** (what follows from the same re-read; none is a separate finding): the amend carry reads `HEAD` against the **commit-time** profile's address, so a claim made under profile A is dropped by a human's amend under profile B (1 → 0) and replaced by B under an agent's (A → B); and a refused commit leaves nothing of the refused run's profile behind (rejected under A, re-run under B → `Agent B` only; 0 raw-message lines name A's address).

```sh
PA=$(mktemp -d); cp <repo>/crates/cli/adapters/claude-code.yaml "$PA/"
sed -i '' 's/^  name: Claude$/  name: Agent A/; s/^  email: noreply@anthropic.com$/  email: a@example.com/' "$PA/claude-code.yaml"
PB=$(mktemp -d); cp <repo>/crates/cli/adapters/claude-code.yaml "$PB/"
sed -i '' 's/^  name: Claude$/  name: Agent B/; s/^  email: noreply@anthropic.com$/  email: b@example.com/' "$PB/claude-code.yaml"
rig=$(dev/jigc-rig bare --binary ~/.local/bin/jigc) || exit; eval "$rig"; [ -n "$REPO" ] || exit
JIGC_ADAPTERS_DIR="$PA" CLAUDECODE=1 jigc setup                              # exit 0
git log -1 --format='%(trailers:key=Co-Authored-By,valueonly)'                # Agent A <a@example.com>
git grep -l 'a@example.com' | wc -l; command grep -rl 'a@example.com' .jigc .claude | wc -l      # 0 ; 0   (no record of the installed identity)
jigc start --workflow single-task "land the work"; fill land-the-work; echo code > code.txt; git add code.txt
JIGC_ADAPTERS_DIR="$PB" CLAUDECODE=1 jigc task finalize land-the-work        # exit 0
git log -1 --format=%B
#   feat(cache): land the work
#
#   A driven change.
#
#   Co-Authored-By: Agent B <b@example.com>
JIGC_ADAPTERS_DIR="$PB" CLAUDECODE=1 jigc milestone create "Cache rework"    # exit 0 ; Agent B <b@example.com>
CLAUDECODE=1 jigc milestone add-task cache-rework "Area low"                 # exit 0 ; Claude <noreply@anthropic.com>   (no directory set: the embedded profile)

# amend carry-over — fresh rig; HEAD built with  JIGC_ADAPTERS_DIR="$PA" CLAUDECODE=1 jigc task finalize land-the-work  (Agent A, =1)
env -u CLAUDECODE JIGC_ADAPTERS_DIR="$PB" jigc task amend "repair one"; fill repair-one fix cache "first repair" "Body one."
env -u CLAUDECODE JIGC_ADAPTERS_DIR="$PB" jigc task finalize repair-one      # exit 0 ; 1 -> 0
# the same over a second A-signed HEAD with CLAUDECODE=1 on both calls       # exit 0 ; 1 -> 1, the value is now Agent B <b@example.com>

# rejection under A, re-run under B — fresh rig, a rejecting pre-commit behind core.hooksPath
JIGC_ADAPTERS_DIR="$PA" CLAUDECODE=1 jigc task finalize land-the-work        # exit 1 ; HEAD unchanged
git config --unset core.hooksPath
JIGC_ADAPTERS_DIR="$PB" CLAUDECODE=1 jigc task finalize land-the-work        # exit 0 ; Agent B <b@example.com> ; git log -1 --format=%B | grep -c a@example.com = 0
```

**Open remainder of this lead:** Codex says the substitution applies to *every* seam-backed door. Driven at `setup`, `task finalize`, `task finalize (amend)`, `milestone create` and `milestone add-task`; **not** driven against a swapped profile at `rename`, `migrate-corpus`, either `milestone finalize` arm, `milestone add-from-spec`, `milestone discard` or `task discard` — those share the seam's one profile load by a source read only (the driver's §5 item 3 says the same of cell 5). It stays an OPEN lead for those seven rows, not a confirmed one.

### The other claims in the source pass

| # | lead(codex, …) | status | datum |
|---|---|---|---|
| 1a | claim 1's rider: an unreadable or malformed commit-time profile silently contributes no trailer (the load error is discarded) | **CONFIRMED as behaviour — not a finding** | It is the design's own sentence (*"`setup.profile-load` at install, no trailer at a commit"*). Driver P3–P12; reconciler: `name: "Cl<aude"` + `CLAUDECODE=1 jigc task finalize` → exit 0, =0, 0 output lines name the profile; and an amend under that profile over an agent-signed `HEAD` → exit 0, 1 → 0 (consistent with the driver's P21/P22). |
| 2 | opening claim *carried* — the eleven registered constructions route through the seam and setup signs separately | **CONFIRMED** | Driver A1–A33, M1–M3 (1 / 0 / 0 at all 12 doors); registry count re-read (§1). The *"profile its agent names"* exception is lead 1. |
| 3 | *"The commit doctype is untouched"* — signing happens after render; no trailer field in the schema | **CONFIRMED (driven half) / classification (the schema half)** | Driven: the envelopes are byte-equal between arms (I rows; re-driven I2, I6) and a doc trailer still needs `add-item`. The schema half is a source read: `git diff --stat` between the rc.23 and rc.24 release tags over `crates/cli/packs` names one file (the methodology `planning-record` schema, one line), and every command in this ledger ran under the pack-load manifest assertion at exit 0. |
| 4 | *Which profile*, sentence 1 — not carried under an environment change | **CONFIRMED** | = lead 1 / (R10, F7). |
| 5 | *Which profile*, sentence 2 — no stored per-repository selection | **CONFIRMED** | The control in lead 1's block: after setup under profile A, 0 files carry the address. |
| 6 | *Which profile*, sentence 3 — prospective | **OPEN LEAD** | Not drivable: it describes a selector that does not exist in rc.24. Nothing to refute. |
| 7 | *When*, sentence 1 — set **and non-empty** | **CONFIRMED** | Driver's three-arm sweep at every door; V1–V3 re-driven (`0`, `false`, a single space all sign). |
| 8 | *When*, sentences 2–4 — the profile names the variable; the engine is uninvolved | **CONFIRMED (driven half)** | Driver P14–P18: the trailer follows the named variable, not `CLAUDECODE`. *The engine is never consulted* is a source read. |
| 9 | *When*, sentence 5 — what the assistant exposes to its agent | **OPEN LEAD** | Not drivable without recording an environment value, which this review forbids; the driver records only that both variables are set and non-empty in its session. |
| 10 | *Where*, sentence 1 — the seam owns the message flag | **CONFIRMED** | J rows (re-driven J1/J2): the message git hands a `commit-msg` hook already carries the trailer under set and does not under unset. |
| 11 | *Where*, sentence 2 — setup signs its own message before `--no-verify` | **CONFIRMED** | A1–A3, R rows; reconciler: setup under rejecting `pre-commit` and `commit-msg` hooks lands at exit 0 with 1 / 0 (block R-R). |
| 12 | *Placement*, sentence 1 — **carried** (*"the place `git interpret-trailers` gives a trailer added at the end"*) | **REFUTED** in the git-recognized mixed paragraph | (R10, F1)'s repro: on the identical message `git interpret-trailers` appends into the existing block and git keeps reading the `Signed-off-by`; jigc opens a new block and git reads only the new one. Datum X2 shows the shape is reachable from `#trailers` items alone. The sentence holds in the other shapes (S1–S10, S12; W1–W4 below). |
| 13 | *Placement*, sentence 2 — **carried** (address and key compared case-insensitively inside the detected block; a matching block returned unchanged) | **CONFIRMED** | D1–D6, D8–D11, W2. The claim is accurate **as Codex worded it** (*"within the detected final trailer block"*); the two corners outside it are (R10, F6) (a bracket-less address is not read as the address) and (R10, F2) (a block git detects and jigc does not). |
| 14 | *Amend sentence* — **carried** (`SessionOrHead` preserves a declared address already present) | **REFUTED** in the mixed-block `HEAD` | (R10, F2)'s repro: git counts 1 before, a human's amend lands 0. Holds for C1–C3, C7–C10. |
| 15 | *No-key sentence* — carried | **CONFIRMED** | P2, P21/P22. |
| 16 | *Malformed-declaration sentence* — carried | **CONFIRMED** | P3–P12 (re-driven for `name: ""` and mode 000); the route text is (R10, F3). |
| 17 | bound: a human finalization without the variable adds no trailer | **CONFIRMED** | Every unset arm; G1, G2 (re-driven G2). |
| 18 | bound: a human command inside an agent environment is indistinguishable | **OPEN LEAD** | Not drivable — a driver cannot be a human (the design's own *not driven*). The nearest driven datum: any non-empty value signs (V rows). |
| 19 | bound: signing cannot change `subject` or `committed.subject` | **CONFIRMED** | I1–I12 (re-driven I2, I6). |
| 20 | bound: the Conventional-Commits header stays the first line | **CONFIRMED** | `%s` identical in all twelve pairs (I rows). |
| 21 | bound: cargo forces `CLAUDECODE` empty | **OPEN LEAD (classification)** | No drive on the installed binary through cargo exists (K1 is a fence; K2 cites the empty arm, which is driven at every door). |
| 22 | bound: the integration suite sets the variable per cell | **OPEN LEAD (classification)** | A statement about the suite, not the binary; K1 (8 passed) is the fence. |
| 23 | the DECISIONS census — eleven doors plus setup; the only production `merge` is `--ff-only`; no `commit-tree` | **CONFIRMED (driven half)** | 12 doors land signed commits; `HEAD` does not move at 14 leaves outside the axis (N1–N11 re-driven, plus `unmanage`, `migrate`, and `relocate`'s refusal path). `relocate`'s **success** path was not reachable (`relocate.frozen-doctype`, exit 1) — that leaf's no-commit classification stays a source read. |
| 24 | completeness: the doc-only path-scoped commit and every `squash: false` per-sub-task and aggregate commit use the seam | **CONFIRMED** | B1–B3; A10–A12; G8/G9; re-driven G5/G6 and the partial chain. |
| 25 | suite-coverage gaps Codex names (statements about the tests, each a place a defect could hide) | **driven; clean except lead 1** | installed/current profile mismatch → (R10, F7) · empty `CLAUDECODE` through the binary → the empty arms · unreadable profile at a commit → P12 · hook rejection then a differently sourced profile → lead 1's last block, clean · **CRLF / trailing whitespace → block W below, clean** · pinned JSON → I rows. |
| 26 | no schema-boundary violation | **CONFIRMED (classification)** | See row 3. |

```sh
# W — message shapes the driver's §5 item 10 left undriven (CRLF, trailing whitespace); fresh rig per row, task finalize
printf 'A driven change.\r\n\r\nRefs: TKT-1\r\n' | jigc doc set-slot commit:land-the-work#body --from-file -        # exit 0
#   W1 set:   exit 0 ; %B carries no \r (od -c) ; … / A driven change. / blank / Refs: TKT-1 / Co-Authored-By: Claude <noreply@anthropic.com> ; git reads both ; =1
#   W1 unset: exit 0 ; git reads [Refs: TKT-1] ; =0
#   W2 set:   doc trailer Co-Authored-By value "Claude <noreply@anthropic.com>   " (trailing blanks)      -> exit 0 ; one line ; =1
#   W3 set:   body 'A driven change.   \n\n   \n\n'                                                       -> exit 0 ; own block after one blank line ; =1
#   W4 set:   body 'A driven change.\r\n\r\nCo-Authored-By: Claude <noreply@anthropic.com>\r\n'           -> exit 0 ; not doubled ; =1
```

## 5. Baseline rows

**None — first drive; nothing to mark CLOSED or STILL-OPEN.** The named neighbour `(2, DEFECT C)` (a posture breach at the commit seam prints no state-truth clause) was **not re-driven** here: it is numbered axis 2's row, and nothing in this ledger changes its status.

## 6. Summary

| key | title | origin | tier | door |
|---|---|---|---|---|
| (R10, F1) | signing opens a new block under a git-recognized mixed last paragraph, so git stops reading the `Signed-off-by` — reachable from `#trailers` items alone | driver (widened by the reconciler) | 3 | `task finalize` |
| (R10, F2) | the amend carry misses a `HEAD` trailer git reads inside a mixed block | driver | 3 | `task finalize` (amend arm) |
| (R10, F3) | `setup.profile-load`'s route blames the embedded profile for a directory-selected one | driver | 3 | `setup` |
| (R10, F4) | the declared bound's `#trailers` route is honoured only for a code-carrying sub-task under `squash: false` | driver | 3 | `milestone finalize` |
| (R10, F5) | the composed amend step says the amended message carries only the trailers you add; it carries the profile's too | driver | 3 | `task amend` |
| (R10, F6) | a bracket-less `Co-Authored-By: <address>` is not read as the same address (marginal) | driver | 3 | `task finalize` |
| (R10, F7) | the co-author is the commit-time environment's profile, not the one setup installed from; no per-repo record (marginal) | codex | 3 | `task finalize` |

**Refuted:** Codex's tier-1 relevance for claim 1 (no loss, no harm) · Codex's *Placement, sentence 1: carried* (F1's repro) · Codex's *Amend sentence: carried* (F2's repro) · the reconciler's own lead that a `BREAKING CHANGE` item widens F1 (the key is refused at `add-item`).

**Open leads:** claim 1 at the seven doors not driven against a swapped profile · *Which profile* sentence 3 (prospective) · *When* sentence 5 (what the assistant exposes) · a human typing into an agent's session · the run-through-cargo bound on the installed binary · `relocate`'s success path as a non-committing leaf · and the driver's own §5 list (cell 5 at the other doors, cell 9 on the doc-only finalize, cell 10 for the doc-only and migration envelopes, cell 11 on eight doors, cells 3/4 off the ordinary finalize, any other git or platform, a non-ASCII message or multi-line trailer value, ambient `commit.cleanup` / `core.commentChar`).

# Doors covered

Every clap leaf that is the door of at least one driven row, in `VERB_KINDS` spelling.

**The axis's doors (10 leaves carrying the 12 doors):**

- `setup`
- `task finalize` (ordinary · amend · doc-only · migration shapes)
- `milestone finalize` (`squash: true` · `squash: false`)
- `rename`
- `migrate-corpus`
- `milestone create`
- `milestone add-task`
- `milestone add-from-spec`
- `milestone discard`
- `task discard`

**A surface row's door:** `task amend` (C19 / F5 — its composed step text).

**Negative controls (rows asserting `HEAD` does not move, under set):** `ingest` · `validate` · `upgrade` · `describe` · `config set` · `milestone list-tasks` · `milestone provision` · `milestone execute` · `milestone join` · `uninstall` · `unmanage` · `migrate` · `relocate` (refusal path only) · `task list` · `doc list` · `config list`.

**Fixture-side rows with an asserted exit (datum X2 and its controls):** `doc add-item` · `doc set-field`.
