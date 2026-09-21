<!-- M52 per-axis review (re-run) — axis 1 ·  — RECONCILED · driven on the installed `jigc 1.0.0-rc.16` (built from commit a3eb026b), 2026-09-21. -->

<!-- M52 per-axis review RE-RUN — axis 1 · caller tokens — the OPUS DRIVER — driven on the installed `jigc 1.0.0-rc.16`, 2026-09-21 -->

# M52 per-axis review (re-run) — AXIS 1 · caller tokens — DRIVER TABLE

**Binary asserted first, before anything else:**

```
$ /Users/maurice/.local/bin/jigc --version
jigc 1.0.0-rc.16            exit=0
```

**RELEASE posture** — the debug-only `debug_assert!` route fences do not exist in this binary, so a
route-fence violation shows up as a *bad emitted command*, never as a panic. Repo at `a3eb026b`
(M52's six audit-fix commits `79e54c75` `6d95756c` `fe8f29c4` `c96137e4` `b9ab6a70` `1b036264`
all present, plus the closing record commit). Every table below is of the **fixed** binary.

**All rigs:** `rig=$(dev/jigc-rig <state> --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"`
— two-step eval throughout, never `eval "$(…)"`, every root from `mktemp -d`, **no `rm -rf` on a
variable path anywhere in this review**. States used: `committed-singletons` (rigs A, B, C, E, F, G, H),
`fresh --pack-from-dev` (rig D — the only way to reach a freeze-exempt doctype). Rig assignment files
were persisted and re-`source`d between calls so one construction served many probes.

**Scope discipline:** the Codex source pass for this axis was **not** read. Reconciliation is a
separate agent.

---

## 0 — the door set, derived from the code (counts I read at HEAD, not the design doc's numbers)

| registry | file:symbol | count I read | vs M51 |
|---|---|---|---|
| `ARG_TOKENS` | `crates/cli/src/cli.rs:2399` | **35** ids, **6** `Plain(PathBearing)` (`path` `file` `from_file` `from` `target` `value`) | unchanged |
| `PATH_ARG_OCCURRENCES` | `cli.rs:3110` | **14** rows / **18** arms / **12** distinct leaf verbs | unchanged |
| `DOCTYPE_DOORS` | `cli.rs:2522` | **16** rows — **10** `Address`, **6** `Bare` | unchanged |
| `SLUG_DOORS` | `cli.rs:2829` | **6** | unchanged |
| `WORK_UNIT_ID_DOORS` | `cli.rs:2589` | **25** rows / **25** doors (doc-comment: *"Twenty-five doors."*) | unchanged — M51's recorded doc/code divergence against the *design text*'s "26 rows" still stands |
| `VERB_KINDS` | `cli.rs:1834` | **47** leaves | unchanged |
| `BEHALF_DOORS` | `cli.rs:2003` | **47** rows | unchanged |
| `COMMITTING_DOORS` | `invocation_log.rs:171` | **10** | unchanged |
| `DESTROYING_DOORS` | `milestone.rs:3275` | **6** (`provision`, `discard`, `uninstall`, `task discard`, `task finalize`, `milestone finalize`); `WORKTREE_DOORS` **4** | **4 → 6** (M52 D3/D8) |
| `ENVELOPE_ARMS` | `render.rs:6030` | **64** arms | **60 → 64** |
| `ENVELOPE_OWED_CODES` | `render.rs:5415` | **4** (`store.not-found`, `store.no-such-leaf`, `store.unknown-type`, `task::FIXED_IDENTITY`) | **new (M52 F4)** |
| `STORE_EXIT_FLIPS` | `render.rs:968` | **7** | **6 → 7** (`unadopted-instance` joined) |
| `ROLLBACK_POPULATIONS` | `rollback.rs:169` | **11** | new |
| `TASK_AREA_FILES` | `engine/src/state.rs:129` | **13** | new |
| `RelocateRefusal::ALL` | `relocate.rs:91` | **10** members / **9** codes | new (M52 F5) |
| `InProgress::ALL` | `repo.rs:234` | **9** git states | new — axis 2's |
| `PRE_DISPATCH_FAULTS` | `crates/cli/tests/pre_dispatch_faults.rs:145` | **4** rows (test-crate registry) | new — axis 6's |
| `ROOT_KNOBS` | `config.rs:56` | **2** (`docs-root`, `placement-root`) | unchanged |
| `SchemaChangeKind` | `engine/src/schema_diff.rs` | **18** variants | unchanged |

**Axis-1 door set** = `PATH_ARG_OCCURRENCES` (12 verbs) ∪ `DOCTYPE_DOORS ▸ Address` (10) ∪
`SLUG_DOORS` (6) ∪ `WORK_UNIT_ID_DOORS` (25) = **35 distinct leaf verbs** (union, not sum) —
identical to M51's, so no leaf loses an axis.

**Cell set** (M51 acceptance-design Part 2, axis 1, unchanged): absolute · `../` escape · symlink
escape · `.git/` component · workbench root · untracked in-repo · leading-colon pathspec magic ·
the `-` stdin sentinel · OS name ceiling · well-formed control. **Plus one cell this re-run adds**,
because M52 minted the predicate that answers it: **well-formed-but-non-canonical head** (the
*fixed-identity* cell — M51's A1-D1 lived in the *control* cell, and the fix gave that shape its own
answer, so it is now its own column).

---

## 1 — counts

* **Matrix rows driven: 378.** Every one's argv ran on the installed rc.16 with its verdict recorded.
  * Group D — `WORK_UNIT_ID_DOORS`: 25 doors × 4 cells = **100**
  * Group B — `DOCTYPE_DOORS ▸ Address`: 10 doors × 8 cells = **80**
  * Group C — `SLUG_DOORS`: 6 doors × 8 cells = **48**
  * Group A — `PATH_ARG_OCCURRENCES`: 15 occurrence-arms × 10 cells = **150**
* **Supplementary drives: 135** — the fixed-identity class sweep (5 doctypes × 9 doors = **45**),
  `RelocateRefusal::ALL` (**9** of 10), the `task bind` declared-role re-drive (**8**), the JSON
  envelope-arm probes (**26**), the M51 row re-drives and their consequence/boundary walks (**47**).
  **Total invocations of the installed binary in this review: 513.**
* **Matrix rows n/a: 191** — enumerated with reasons in §5 (same five groups as M51, one changed).
* **New defects: 2** (1 MEDIUM-HIGH, 1 MEDIUM) + **1 record defect** (a carried deferral whose own
  enumeration is falsified). **3 observations** recorded as matches-contract-with-a-note.
* **M51 rows: 5 of 5 CLOSED**, each re-driven — §2.
* **Doors covered (door of ≥1 driven row): 40**, `VERB_KINDS` spelling — §7 (35 axis-1 door-set members + 5 setup/consequence doors).

---

## 2 — M51 rows: CLOSED / STILL-OPEN (the re-run's first deliverable)

### A1-D1 (HIGH) — bogus `<slug>` head on a singleton accepted by five write doors; `finalize` drops a payload → **CLOSED**

```
setup: rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit
       eval "$rig"; jigc start --workflow single-task "repro"

$ jigc doc set-slot "adr:ghost#context" --from-file - </dev/null
  exit 1 — no staged instance for `adr:ghost#context` — provision it first …   ← unchanged control
$ printf 'PAYLOAD-ALPHA\n' | jigc doc set-slot "vision:alpha#thesis" --from-file -
  exit 1
  blocking · store.fixed-identity — `vision:alpha#thesis` is not an address `vision` can have:
    its identity is fixed — the CLI supplies the slug, so `vision:vision` is the one instance this
    doctype has
    at: vision:alpha#thesis
    route: `jigc doc show vision:vision` — a fixed-identity doctype has one instance at a fixed slug
```

**The whole class re-driven over its axis** — the **fixed-identity doctype set derived from the
binary** (`doc schema <ty> --format json` → `identity.kind == "fixed"`): `vision` · `roadmap` ·
`decisions-log` · `changelog` · `deferral-ledger` (5 of the 14 shipped; the other 9 are `slugged`),
× the 9 reachable `DOCTYPE_DOORS ▸ Address` doors = **45/45 exit 1, `store.fixed-identity`, 1 route each.**

```
$ ls .jigc/tasks/axis-one/docs/
commit:axis-one.md  provenance.json  roadmap:roadmap.md  vision:vision.md
```

— after a sweep that fired the bogus head at **eight** address doors × five doctypes, the task's
staged docs directory holds **only canonical identities**. The second payload `finalize` used to
drop can no longer be written. The read door and the write doors now say one thing. **CLOSED.**

### A1-D2 (MEDIUM-HIGH) — `relocate <freeze-exempt> --from .jigc` moves jigc's own install artifacts → **CLOSED**

```
setup: rig=$(dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc --pack-from-dev) || exit; eval "$rig"
$ git ls-files
.claude/settings.json  .claude/skills/jigc/SKILL.md  .jigc/.gitignore  .jigc/AGENT.md
.jigc/config/.gitkeep  .jigc/config/packs.yaml  .jigc/version  CLAUDE.md  README.md

$ jigc relocate adr --from .jigc
blocking · config.workbench-root — `.jigc` is inside jigc's own workbench (`.jigc/`) — `--from` sweeps
  every committed doc under the prior home into this doctype's home, and jigc's own state was never a
  home any doctype's instances sat at
  route: re-run `--from` with the directory this doctype's instances actually sat at — `jigc doc list` …
exit=1

$ jigc relocate adr --from .claude
blocking · config.unusable-root — `.claude` is inside jigc's own adapter install — …
exit=1

$ jigc relocate adr --from .jigc/config      → config.workbench-root, exit 1
$ git status --porcelain                     → (empty)
```

The refusal is at the **root set**, not a hand-written pair: `.jigc/config` (a sub-path) is refused
too. **CLOSED.**

### A1-D3 (MEDIUM) — the `relocate`/`from` registry row's declared argv cannot reach its own arm → **CLOSED**

Two halves, both re-driven:

```
$ for ty in vision roadmap decisions-log changelog adr spec prd arch-doc idea research \
            milestone-record completion-record deferral-ledger planning-record commit; do
    jigc relocate "$ty" --from docs/old; done
  15/15 exit 1 · relocate.frozen-doctype · 1 route each
  e.g. blocking · relocate.frozen-doctype — `adr` is a frozen doctype — relocate it through the
       version-gated `jigc migrate-corpus`, not the freeze-exempt path
       at: adr
       route: `jigc migrate-corpus` walks every prior home the doctype's versioned snapshots declare …
```

The refusal that M51 recorded as *code-less, route-less* now carries an identity, a locus and a
route (M52 F5 / `RelocateRefusal::FrozenDoctype`). And the registry row's `when:` clause now **names
its witness** verbatim in the source I read for the door set: *"the witness is `path_arg_occurrence_axis`'s
`FixturePack::from_dev_pack` corpus, which ships no freeze manifest"* — the prose half of the repair.
**CLOSED.** *Declared bound, carried forward from M52's own record: no mechanical checker asserts
that `when:` sentence.*

### A1-D4 (MEDIUM) — OS-name-ceiling at the address `<slug>` head leaks a bare OS error at five write doors → **CLOSED**

```
setup: rig committed-singletons; jigc start --workflow single-task "axis one"
argv:  <head> = python3 -c "print('a'*300)"
$ jigc doc set-slot "vision:<A300>#thesis" --from-file /dev/null --task axis-one
blocking · write.slug-name-ceiling — the `<slug>` head of address `vision:<A300>#thesis` is 300 bytes
  — over the 165-byte ceiling a doc identity carries
  at: vision:<A300>#thesis
  route: `jigc doc list` lists the committed docs and the identity each one carries; a slug is one
         filesystem path component, so an identity is at most 165 bytes — jigc refuses rather than
         truncating, because a truncated identity names a different doc
exit=1
```

**10/10 address doors** now answer with the code, an `at:` and a route — including the five M51
recorded as leaking `File name too long (os error 63)` bare (`doc set-slot` · `add-item` ·
`remove-item` · `retitle-item` · `set-field`), and including `task bind` once a **declared** role is
in play (`jigc start --workflow implement-from-spec`; see §4-O3). The boundary is exact at both
sides: `--slug` at **165** mints (`renamed adr:seed-decision -> adr:<A165>`, exit 0), at **166**
refuses (`write.slug-name-ceiling`, 6/6 slug doors). **CLOSED.** *One residual, new and separate —
the arm this finding takes under `--format json`: §3, A1-N3.*

### A1-D5 (MEDIUM) — `config set docs-root <component over NAME_MAX>` accepted at exit 0 → **CLOSED**

```
$ jigc config set docs-root "$(python3 -c "print('a'*300)")"
blocking · config.unusable-root — `<A300>` cannot be the `docs-root`: `<A300>` is 303 bytes — a path
  component the filesystem cannot name
exit=1                                    # measured BARE, not through a pipe
```

Both `ROOT_KNOBS` members carry it. **CLOSED.** *But its sibling cell — the one M51 explicitly left
ungraded (`jigc config set docs-root -`, exit 0, "downstream behaviour was not driven far enough to
claim harm") — was driven to harm in this re-run: §3, A1-N1/A1-N2.*

**M51 rows: 5 CLOSED · 0 STILL-OPEN.**

---

## 3 — new defects

### A1-N1 (MEDIUM-HIGH) — a route whose operand's first byte is `-` is re-read as an option by the receiving CLI and dead-ends at exit 2

**Doors:** `config set` (the `ROOT_KNOBS` `Adjudicated` arm — the preimage) → `validate` /
`task validate` (the producer) → `unmanage` (the route's target). **Cells:** *well-formed control*
at the `value` occurrence, then the emitted route.

```
setup: rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"

1. $ jigc config set docs-root -
     config: set `docs-root` = `-` — written to `.jigc/config/`, uncommitted …
     exit=0                                               ← the Adjudicated row admits it
   (and the multi-byte spelling, which needs clap's own separator to be received at all:
    $ jigc config set docs-root -- -x     → exit 0, `docs-root = -x  (project)`)

2. $ jigc start --workflow single-task "dash" ; jigc doc create adr --title "Dash Probe"
   $ for sl in context decision consequences; do printf 'Prose.\n' | \
       jigc doc set-slot "adr:dash-probe#$sl" --from-file -; done
   $ jigc doc set-field commit:dash#header/type --value docs
   $ printf 'dash\n' | jigc doc set-slot commit:dash#summary --from-file -
   $ jigc task finalize dash
     exit=0   promoted -/decisions/dash-probe.md

3. $ git rm -q -- "-/decisions/dash-probe.md"            # an ordinary out-of-band deletion

4. $ jigc validate
     blocking (gates at finalize) · reconciliation.rename — tracked managed doc adr:dash-probe
       (-/decisions/dash-probe.md) is missing
       at: -/decisions/dash-probe.md
       route: restore -/decisions/dash-probe.md, or confirm the deletion by dropping it from the
              index: `jigc unmanage -/decisions/dash-probe.md`

5. $ jigc unmanage -/decisions/dash-probe.md              # the route, verbatim, measured BARE
     error: unexpected argument '-/' found
       tip: to pass '-/' as a value, use '-- -/'
     Usage: jigc unmanage [OPTIONS] <PATH>
     ROUTE EXIT=2

6. $ jigc unmanage '-/decisions/dash-probe.md'            # quoting does not help — it is not a shell fault
     exit=2
7. $ jigc unmanage -- -/decisions/dash-probe.md           # the form that works, which no surface prints
     exit=0
```

Reproduced identically at the `-x` spelling (`jigc unmanage -x/decisions/dash-x.md` → `error:
unexpected argument '-x' found`, exit 2).

**What it contradicts.** [surface-contract.md](../../../../design/surface-contract.md) → *The route
fence*, the M51 completion audit's own closing sentence about the axis this belongs to:
*"it now carries a tenth [cell], and asserts of **every** cell that the command spans a door prints
**can be run**."* Driven, this span cannot be run. The reason is one byte in a shared predicate:
`engine::finding::shell_safe`'s inert alphabet is `[A-Za-z0-9._:#/=@+-]`, and its stated subject is
*"whether one emitted command-line token survives a real shell as exactly itself"* — which a
leading-`-` token does. **The shell is not the problem; the receiving CLI is.** The subject fence at
`Finding::graded` asks the located subject the same shell-safety question, so it passes too. This is
the **space class one byte over**: M51 closed *an unquoted path holding a space is two inert tokens*
and left *an inert token the callee reads as a flag*.

**Bound, stated:** the preimage needs a `docs-root`/`placement-root` whose first byte is `-`, which
the knob accepts at exit 0 (step 1) — so it is adopter-reachable through a documented knob, not only
through a hand-edited config. No bytes are lost; the file is on disk and in HEAD. What is lost is
the recovery: the surface names one command and it errors.

**Same producer as a carried deferral, different dead end.** [decisions-pending.md](../../../../implementation/decisions-pending.md)
carries M52 (a): *`reconciliation.rename`'s route names `jigc rename` on a fixed-identity doctype's
uncommitted move.* This is a second dead end at that producer, on an orthogonal axis.

### A1-N2 (MEDIUM) — a `ROOT_KNOBS` value the door accepts makes the door's own `git mv` unparseable, and the refusal's route prescribes re-running it

**Door:** `config set` (`PATH_ARG_OCCURRENCES` row 13 arm 1 — `key ∈ ROOT_KNOBS`, one of only three
`Adjudicated` rows, and one of two `BEHALF_DOORS ▸ MovesOnBehalf`). **Cell:** the `-` stdin sentinel
read as an ordinary value.

```
setup: rig=$(dev/jigc-rig committed-singletons …) || exit; eval "$rig"
$ jigc config set placement-root -
relocating the committed doc(s) stranded by the `placement-root` re-point to `-` …
blocking · config.repoint-failed — `placement-root` was not set to `-`:
  `git mv docs/decisions-log.md -/decisions-log.md` failed: error: unknown switch `/'
  usage: git mv [-v] [-f] [-n] [-k] <source> <destination> …
  — the re-point was undone
  at: docs/decisions-log.md
  route: `placement-root` is unchanged and every doc this re-point moved is back at its prior home.
         Fix what this message names, then re-run `jigc config set placement-root -`
(and a second, identical finding for docs/roadmap.md)
exit=1
```

**Three things are true at once, and the third is the defect.** (1) The transaction is sound — *"the
re-point was undone"*, driven: `git status --porcelain` empty, `config get placement-root` unchanged.
No loss. (2) The failure is **jigc's own argv**, not the user's: the door builds
`git mv <src> -/decisions-log.md` with no `--` separator, so git reads the destination as an option
cluster. (3) The route says *"Fix what this message names, then re-run `jigc config set placement-root -`"*
— there is nothing the operator can fix; the value **is** the fault, and the identical re-run is
guaranteed to fail the same way. A **law-2** route that prescribes a repeat of a deterministic failure.

**And the knob's admissibility depends on corpus shape, not on a value rule.** The same
`jigc config set docs-root -` lands at **exit 0** on a corpus with no `location:` doctype instance to
move (§A1-N1 step 1, driven) and **exit 1** on one that has (rig A, which held a
`milestone-record` under `docs/milestone-records/`). So the value is neither accepted nor refused by
a rule — it is refused by an accident of what happens to be committed. The row's three declared
predicates (`untrackable_reason` · `is_workbench_root` · `unusable_root_reason`) cover absolute, edge
whitespace, pathspec magic, symlinked/file-shaped and — since M52 — nameability; **none covers
"a value the door's own git invocation and the CLI's own parser will read as an option."**

### A1-N3 (MEDIUM, a record defect) — a carried deferral's enumeration of the flatten exemptions is falsified by a fourth family, which diverges from its own sibling inside one door

`decisions-pending.md` carries, out of the M52 completion audit:

> **(b)** `command-output-contract.md:197`'s rule *a reject that carries a finding takes the findings
> arm* is broader than the binary: **`store.malformed-slug`, `pack.resource-missing` and the
> `workflow-refs.*` flatten path** are deliberate exemptions the sentence does not carve out.

Driven, there is at least a fourth, and it is the one family the contract's **own target-form table**
lists under a declared URI target (`command-output-contract.md:226`, `:246` — *`write.*` — the
address-bearing writes … `type:slug#<fragment>`*):

```
setup: rig committed-singletons; jigc start --workflow single-task "axis one"

$ jigc doc set-field "roadmap:roadmap#milestones/m-beta/title" --value X --task axis-one --format json
{ "schema_version": 3,
  "findings": [ { "code": "write.not-present",
                  "key": {"code":"write.not-present","target":"roadmap:roadmap#milestones/m-beta/title"}, … } ] }

$ jigc doc set-field "roadmap:<A300>#milestones/m-alpha/title" --value X --task axis-one --format json
{ "error": "blocking · write.slug-name-ceiling — the `<slug>` head of address `roadmap:<A300>#…` is 300 …" }
```

**One door, one argument, one `write.*` family, two arms.** The exemption's two measured M50 grounds
do not transfer: `write.slug-name-ceiling` at an *address* head **carries a locus** (`at:
roadmap:<A300>#milestones/m-alpha/title`, a well-formed URI in the declared normal form) and a route
— it is not the null-target case `store.malformed-slug` is excused for. Driven at the sibling doors,
the same split: `doc show <over-ceiling head>` → `{error}`, `doc show roadmap:nosuchdoc` →
`{findings, schema_version}` with `store.fixed-identity` keyed.

This is **not** a new *binary* behaviour — it is the same wider-rule gap the deferral names. What is
defective is the **record**: an enumeration of exemptions that omits the family the contract's own
table declares as keyed is the false-completeness shape this whole review exists to catch. The
declared `relocate.*` / `task-bind.*` exemption (M52 F5: *"the new codes flatten … that set is scoped
to codes the contract lists under a declared target form"*) is stated and correct — it is precisely
the test `write.*` passes and is missing anyway.

---

## 4 — observations (matches contract, with a note)

**O1 — the `--slug` grammar reject still carries an identity at exactly one of six doors. STILL-OPEN
as an observation, unchanged from M51.**

```
$ jigc rename adr:seed-decision --to "Axis Title" --slug '../../x'
  blocking · write.malformed-slug — `--slug "../../x"` is not a valid slug …          exit 1
$ jigc doc create adr --title X --slug '../../x' --format json
  { "error": "`--slug \"../../x\"` is not a valid slug — use lowercase letters, digits, and single
              hyphens (no leading, trailing, or doubled `-`)" }                        exit 1
  — identical sentence, no `blocking ·` prefix, no code, no route, at start · migrate · doc create ·
    doc add-item · doc rename (5/6).
```

The **ceiling** code reaches all six doors; the **grammar** code reaches one. Already on the record
(M50 Increment 2), re-driven and unchanged on rc.16.

**O2 — `relocate`'s partial-block cell prints a `blocking` finding and exits 0, by declared shape.**

```
$ jigc relocate adr --from notes          # freeze-exempt pack, two foreign .md at the prior home
freeze-exempt relocation: 0 moved, 0 displaced, 2 blocked
  blocked   notes/My Notes.md
    blocking · ingest.unaddressable-identity — … its name is not a doc id …
  blocked   notes/foo.md
    blocking · write.already-present — the destination `docs/decisions/foo.md` is occupied …
exit=0
$ jigc relocate adr --from notes --format json
{ "moved": [], "blocked": [["notes/My Notes.md", "blocking · ingest.unaddressable-identity — …\n  at: …\n  route: …\n"], …], "displaced": [] }
```

`ENVELOPE_ARMS`' `relocate` row declares exactly this: `arm: "Report"`, `ArmOutcome::Success`,
`ArmShape::Object(["blocked","displaced","moved"])`, `ArmStatus::Pinned`. So the exit 0 and the
string-carried finding are the **pinned** shape, not a divergence. The note is that a driver reading
`blocked[]` gets the finding as **rendered text**, code inside a message — the complement the
contract's own membership test names — on a *success* arm the pin closes over. Recorded, not graded.

**O3 — `task bind`'s role check still precedes the slug guard, and is `task-bind.undeclared-role` now.**
On `single-task` every head cell answers `blocking · task-bind.undeclared-role — role \`spec\` is not
a declared read-role of this task (declared: none)` (a **code** it did not carry in rc.15 — M52 F5's
uncounted pair). With a declared role (`jigc start --workflow implement-from-spec`) the guard fires
correctly at 8/8 cells (§2, A1-D4). Recorded because a reviewer driving the registry's obvious argv
on `single-task` would read the guard as absent when it is present.

**O4 — pre-dispatch ordering is not uniform across the two token families, and both answers are honest.**
Outside a git repository: `jigc task discard "../.."` → `work-unit.malformed-id` (the token guard
wins), while `jigc doc show "vision:../../etc/passwd"` → *not inside a git repository (no `.git`
found …)* (the pre-dispatch fault wins). Both exit 1, both name a real condition. Recorded for axis 6's
reconciler rather than graded here.

**The composite integrity assertion, driven once at the end of the group-A rig:**

```
$ cat "$RIG/outside/planted.md"   → "# Planted\n\nCANARY-BYTES-9f3\n"    28 bytes, unchanged
$ git log --oneline -1            → 5ad6b7f tracked source               HEAD unmoved
$ find "$RIG/outside" -newer "$RIG/outside/planted.md"   → (nothing)
$ git status --short              → only .jigc/config/{fills,steps,manifest.yaml}, escape-link,
                                    untracked-source.md
$ jigc task discard "" ; ls .jigc/tasks | wc -l   → 4 task dirs still present  (RC-m50's blocker)
```

The canary planted outside the repository reached **no stream** across 513 invocations.

---

## 5 — (door, cell) pairs I did **not** drive, and why

**191 pairs**, five groups, each stated rather than presented as driven:

1. **`relocate` × the 9 non-control cells on a *stock* corpus — 9 pairs.** The door refuses at the
   doctype before `--from` is read — which is no longer a silent refusal (`relocate.frozen-doctype`,
   15/15 doctypes driven, §2/A1-D3). The cells were driven instead on the manifest-less
   `--pack-from-dev` pack, which is the witness the registry row's `when:` now names.
2. **`DOCTYPE_DOORS ▸ Address` × *untracked in-repo* and × *stdin sentinel* — 10 × 2 = 20 pairs.**
   n/a by shape: an address head is an identity, never a filesystem path the caller points at. The
   *path* half of those doors is covered by their `from_file` occurrence in group A.
3. **`SLUG_DOORS` × *untracked* and × *stdin* — 6 × 2 = 12 pairs.** Same reason: `--slug` is a
   verbatim mint override, not a path.
4. **`WORK_UNIT_ID_DOORS` × the six filesystem-shaped cells — 25 × 6 = 150 pairs.** The guard is
   `engine::slug::is_slug`, so every non-slug token is one equivalence class; the three driven
   members of it (`""`, `../..`, `/etc/passwd`) answer identically at all 25 doors, and the fourth
   driven cell (well-formed unknown) is the discriminating one. More spellings of *not a slug* would
   add rows, not information.
5. **`RelocateRefusal::UntrackableDestination` — 1 member of 10.** Reaching it needs a manufactured
   schema whose home resolves inside `.git/`, which is the engine-side axis M49 declared open (*"a
   hand-authored shadow containing `.git/` remains open"*). The other **9/10** members were driven
   (§6).

Also **not driven**, and named: the `owned-location` gate's *presence* and *trackedness* legs
(escape legs only were driven); `config set docs-root -`'s behaviour on a corpus holding a
*placement* doc at the repo root (the `-`-rooted `placement-root` cell was driven and refused,
A1-N2); and any **concurrent** drive — every row here is single-process.

---

## 6 — `RelocateRefusal::ALL`, driven (9 of 10)

One repro block; the argv differs only by cell. Rig D (`fresh --pack-from-dev`), except the last row.

```
setup: rig=$(dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc --pack-from-dev) || exit; eval "$rig"
       mkdir -p notes docs/decisions
       printf '# A\n' > notes/foo.md; printf '# B\n' > docs/decisions/foo.md
       printf '# S\n' > "notes/My Notes.md"; git add -A && git commit -qm setup
```

| member | argv | exit | code | verdict |
|---|---|---|---|---|
| `UnknownDoctype` | `relocate nosuchtype --from notes` | 1 | `store.unknown-type` | matches |
| `PriorHomeMissing` | `relocate adr --from ""` | 1 | `relocate.malformed-prior-home` | matches |
| `PriorHomeUnusable` | `relocate adr --from /` · `--from //` | 1 | `relocate.malformed-prior-home` | matches (2 cells) |
| `WorkbenchPriorHome` | `relocate adr --from .jigc` · `--from .jigc/config` | 1 | `config.workbench-root` | matches (2 cells) |
| `InstalledPriorHome` | `relocate adr --from .claude` | 1 | `config.unusable-root` | matches |
| `FrozenDoctype` | `relocate adr --from docs/old` *(stock pack)* | 1 | `relocate.frozen-doctype` | matches — 15/15 doctypes |
| `TransientDoctype` | `relocate commit --from notes` | 1 | `store.transient-type` | matches |
| `OccupiedDestination` | `relocate adr --from notes` | 0 | `write.already-present` (per-file `blocked`) | matches (O2) |
| `UnaddressableDestination` | same run | 0 | `ingest.unaddressable-identity` | matches (O2) |
| `UntrackableDestination` | — | — | — | **not driven** (§5.5) |

---

## 7 — doors covered (door of ≥1 **driven** row), `VERB_KINDS` spelling

**40 leaves.** The 35 axis-1 door-set members, plus 5 driven as a row's setup or consequence
assertion (`validate` — the store sweep that produced A1-N1's route and A1-D5's post-state;
`doc schema` — the fixed-identity/home projection that *derived* the doctype set and read
`contract-version: 7`; `ingest` — the adoption route over a leading-dash filename):

`start` · `workflow` · `migrate` · `unmanage` · `relocate` · `rename` · `validate` · `ingest` ·
`doc create` · `doc author` · `doc add-item` · `doc remove-item` · `doc retitle-item` ·
`doc rename` · `doc set-field` · `doc set-slot` · `doc show` · `doc list` · `doc schema` ·
`task diff` · `task validate` · `task discard` · `task finalize` · `task bind` · `config set` ·
`config get` · `config insert-step` · `config replace-step` · `config remove-step` · `config fill` ·
`config fork` · `milestone create` · `milestone add-task` · `milestone add-from-spec` ·
`milestone list-tasks` · `milestone provision` · `milestone execute` · `milestone join` ·
`milestone finalize` · `milestone discard`

The 5 beyond the door set, each with the driven row it served: **`validate`** (the store sweep that
produced A1-N1's route and A1-D5's post-state), **`doc schema`** (the identity/home projection that
*derived* the fixed-identity doctype set, read at `contract-version: 7`), **`ingest`** (the adoption
route over a leading-dash filename), **`milestone create`** (minted the milestone the
`add-from-spec` rows address), **`config get`** (read both `ROOT_KNOBS` back after every
`config set` cell).

**No leaf loses an axis against M51**: M51 covered 39, this re-run covers those 39 plus `ingest`.

---

## 8 — what this adds over flow-53 arm 5 (and arms 1, 6)

Flow 53's arms that touch this axis are **arm 5** (the fixed-identity class: the derived
fixed-identity doctype set × `DOCTYPE_DOORS ▸ Address` + `SLUG_DOORS`' `rename` row, asserting
`store.fixed-identity`, the ceiling code, and `doc schema` at contract-version 7), **arm 1** (the
rollback populations) and **arm 6** (`ENVELOPE_ARMS` membership). This review re-drove arm 5's
subject and it **holds** — 45/45 fixed-identity rows, `contract-version: 7`, the staged set clean.

What the review adds that those arms' subjects cannot contain:

1. **The preimage arm 5 has no reason to build.** Arm 5 drives a *non-canonical head*. A1-N1's
   preimage is a **canonical everything** — a well-formed knob value, a legal slug, a clean finalize
   — and the defect appears only three verbs later, in a *route* emitted by a fourth door. No
   fixed-identity arm reaches it.
2. **The cells no arm carries.** Arm 5's cell set is `{non-canonical head, rename --slug, OS-ceiling
   head}`. Axis 1 adds **workbench root**, **leading-colon pathspec magic**, the **`-` stdin
   sentinel**, **untracked in-repo** and the **well-formed control** — and two of the three new
   findings live in the `-` cell, which is the cell M51 explicitly left ungraded.
3. **Runnability, not emission.** Arm 1 and the M51 path-arg axis assert that a door *refuses* and
   that nothing is written. Neither runs the route it printed. A1-N1 is invisible to any assertion
   that stops at the refusal: the refusal is correct, the route is correct English, and the command
   exits 2.
4. **The consequence half of an *accepted* write.** Arm 6 asserts each envelope arm's key set at its
   declared arm. A1-N3 is about a code whose arm nobody declared — it is neither a registry member
   nor a listed exemption — and it is visible only by driving two *siblings of one family at one
   door* and comparing, which no single-arm assertion does.
5. **A carried deferral's enumeration checked against the binary.** The record says the exemptions
   are three. Driven, they are at least four. A test that pins today's behaviour cannot falsify a
   sentence in a decisions ledger; a driver reading the ledger and then driving the binary can.

---
---

<!-- ══════════ RECONCILIATION — added by the reconciler, 2026-09-21, driven on `jigc 1.0.0-rc.16` ══════════ -->

# Reconciliation ledger — AXIS 1 · caller tokens

**Everything above this line is the Opus driver's table, unchanged.** Below is the reconciliation of
that table against the Codex source pass (`codex/axis1-codex.md`, commit `a3eb026b`), under the rule
in `acceptance-design.md` → *The reconciliation rule*: **a claim by one that the other cannot
reproduce is a lead, not a finding.** Every Codex claim was entered as a lead and then **driven**, or
recorded OPEN with the reason it could not be. Every driver defect was **re-driven once by the
reconciler** before being carried.

**Reconciler posture.** Binary asserted first: `/Users/maurice/.local/bin/jigc --version` →
`jigc 1.0.0-rc.16`, exit 0. Rigs built exactly as the driver built them
(`rig=$(dev/jigc-rig <state> --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"`, two-step
eval, every root from `mktemp -d`, no `rm -rf` on a variable path anywhere in this review). States
used by the reconciler: `committed-singletons` (rigs A, B, C, E, F, G), `fresh --pack-from-dev`
(rigs D, H), plus one `mktemp -d` non-repo directory for the outside-a-git-repo cell. **~90 further
invocations of the installed binary**, all of them recorded below as repro blocks.

**Codex's headline is the axis's completeness question, and it returned empty:** *"No grounded
completeness defect found … neither an omitted Axis 1 door nor a caller-token bypass with
file-and-line evidence."* So there is no Codex-origin CONFIRMED **defect** on this axis. What the
pass does carry — five M51 row dispositions, seven consistency claims, one declared bound and one
boundary check — is entered below as fifteen leads and driven.

---

## A — Codex claims, each driven

### C1 — `lead(codex, A1-D1 is CLOSED: non-canonical identities refuse through the shared fixed-identity predicate at `parse_verb_addr` + three siblings)` → **CONFIRMED (repro)**

```
setup: rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit
       eval "$rig"; jigc start --workflow single-task "axis one recon"

$ printf 'PAYLOAD-ALPHA\n' | jigc doc set-slot "vision:alpha#thesis" --from-file -
blocking · store.fixed-identity — `vision:alpha#thesis` is not an address `vision` can have: its
  identity is fixed — the CLI supplies the slug, so `vision:vision` is the one instance this
  doctype has
  at: vision:alpha#thesis
  route: `jigc doc show vision:vision` — a fixed-identity doctype has one instance at a fixed slug
exit=1                                                            # measured BARE, not through a pipe
```

Driven at two of Codex's three named sibling boundaries as well — `milestone add-from-spec`
(`vision:alpha#thesis` → `store.fixed-identity`, C7 below) and `task bind`
(`vision:alpha` → `store.fixed-identity`, C4 below). The driver's own 45/45 class sweep is not
re-run here; the predicate is confirmed at four doors by the reconciler.

### C2 — `lead(codex, A1-D2 is CLOSED: `relocate --from` refuses the installed roots, derived from the adapter profile rather than a `.claude` literal)` → **CONFIRMED (repro)**

```
setup: rig=$(dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc --pack-from-dev) || exit; eval "$rig"

$ jigc relocate adr --from .jigc
blocking · config.workbench-root — `.jigc` is inside jigc's own workbench (`.jigc/`) …     exit=1
$ jigc relocate adr --from .claude
blocking · config.unusable-root — `.claude` is inside jigc's own adapter install — `.claude` is one
  of the paths `jigc setup` writes …                                                       exit=1
$ jigc relocate adr --from .jigc/config
blocking · config.workbench-root — `.jigc/config` is inside jigc's own workbench (`.jigc/`) … exit=1
$ git status --porcelain     → (empty)
```

The sub-path cell (`.jigc/config`) refusing is the half that makes it a **root set** rather than a
hand-written pair, which is Codex's claim and the driver's; both hold.

### C3 — `lead(codex, A1-D3 is CLOSED: the registry row names its manifest-less witness, so the registered argv reaches its caller-token arm instead of faulting on frozen-doctype posture)` → **CONFIRMED (repro, both halves)**

```
setup: rig committed-singletons (the STOCK pack)
$ for ty in vision roadmap decisions-log changelog adr spec prd arch-doc idea research \
            milestone-record completion-record deferral-ledger planning-record commit; do
    jigc relocate "$ty" --from docs/old; done
15/15 → exit 1 · relocate.frozen-doctype · an `at:` · exactly 1 route each
e.g.  blocking · relocate.frozen-doctype — `adr` is a frozen doctype — relocate it through the
      version-gated `jigc migrate-corpus`, not the freeze-exempt path
        at: adr
        route: `jigc migrate-corpus` walks every prior home the doctype's versioned snapshots declare …

setup: rig fresh --pack-from-dev (the manifest-less witness the row's `when:` names)
$ jigc relocate adr --from .jigc      → config.workbench-root   (NOT relocate.frozen-doctype)
```

The second block is the load-bearing half: on the witness corpus the same argv **passes** the
doctype gate and is answered by the *caller-token* predicate, which is what the row claims. Carried
forward unchanged: no mechanical checker asserts the `when:` sentence (declared bound, M52's own).

### C4 — `lead(codex, A1-D4 is CLOSED: every user-address boundary runs the shared malformed-slug-head / name-ceiling guard before filesystem resolution — `doc`, `task bind`, `rename`, `milestone add-from-spec`)` → **CONFIRMED (repro)**

```
setup: rig committed-singletons; A300=$(python3 -c "print('a'*300)")
       jigc start --workflow single-task "axis one recon"

$ jigc doc set-slot "vision:$A300#thesis" --from-file /dev/null
blocking · write.slug-name-ceiling — the `<slug>` head of address `vision:<A300>#thesis` is 300
  bytes — over the 165-byte ceiling a doc identity carries
  at: vision:<A300>#thesis
  route: `jigc doc list` lists the committed docs and the identity each one carries; a slug is one
         filesystem path component, so an identity is at most 165 bytes — jigc refuses rather than
         truncating, because a truncated identity names a different doc
exit=1                                                            # BARE. No `os error 63` anywhere.
```

**And the driver's one un-reproduced sub-claim, driven here rather than taken on trust** — §2/A1-D4's
*"including `task bind` once a **declared** role is in play … 8/8 cells"* carries no repro block in
the table above, and the driver's persisted sweep (`driver/addr_sweep.out`) shows only the
**undeclared**-role arm. The reconciler built the declared-role state and drove it:

```
setup: rig=$(dev/jigc-rig committed-singletons …) || exit; eval "$rig"
       jigc start --workflow implement-from-spec "bind probe"      # declares a `spec` read-role
argv:  jigc task bind spec <ADDR-cell> bind-probe

  spec:../../etc/passwd   → blocking · store.malformed-slug        exit 1
  spec:/etc/passwd        → blocking · store.malformed-slug        exit 1
  spec::(top)pwn          → blocking · store.malformed-slug        exit 1
  spec:.git               → blocking · store.malformed-slug        exit 1
  spec:a--b               → blocking · store.malformed-slug        exit 1
  spec:<A300>             → blocking · write.slug-name-ceiling     exit 1
  vision:alpha            → blocking · store.fixed-identity        exit 1
  spec:nosuch             → blocking · store.not-found — no committed doc `spec:nosuch` to bind
                            (expected at `docs/specs/nosuch.md`)   exit 1
```

**8/8 adjudicated at the head, none reaching a filesystem fault.** The sub-claim is **not demoted** —
it is now driven, by the reconciler, with the repro block it lacked.

### C5 — `lead(codex, A1-D5 is CLOSED: `unusable_root_reason` checks every component against the OS-name ceiling before the filesystem walk, at both `ROOT_KNOBS` members)` → **CONFIRMED (repro)**

```
$ jigc config set docs-root "$(python3 -c "print('a'*300)")"
blocking · config.unusable-root — `<A300>` cannot be the `docs-root`: `<A300>` is 300 bytes — a path
  component may carry at most 255, so every directory the re-point has to create fails to be *named*
  while the knob lands anyway, and the store then resolves its docs to homes nothing is at
  route: re-run with a repo-relative directory … `jigc config list` shows the value in force …
exit=1                                                                                      # BARE
```

*Datum note, not a finding:* the driver's block records the message as *"`<A300>` is **303** bytes"*;
driven here on the same binary with a 300-byte component it reads **300**. The driver's cell was
presumably a 303-byte spelling. The code, the exit and the refusal are identical; only the byte
count in the driver's transcription differs from mine.

### C6 — `lead(codex, `ARG_TOKENS` remains total over the clap vocabulary and classifies exactly `file` · `from` · `from_file` · `path` · `target` · `value` as path-bearing)` → **CONFIRMED (source, and its drivable half driven)**

Read at `crates/cli/src/cli.rs:2399-2438` at HEAD `a3eb026b`: **35** ids, **6** `Plain(PathBearing)`
— the same six, and the same counts the driver's §0 recorded independently. The two reads agree.

The **drivable** half is Codex's own declared bound one step on (C15): the three `Other`-classified
arguments whose value *does* become a path component were driven over the escape shapes —

```
setup: rig committed-singletons
$ jigc start --workflow single-task "../../escape"   → task minted: escape      (sanitized)
$ jigc start --workflow single-task ".git"           → task minted: git         (sanitized)
$ jigc start --workflow single-task "$(python3 -c "print('a'*300)")"
                                                     → task minted: aaaa…(50)   (word-capped)
$ jigc doc create adr --title "../../x" --task …     → adr:x                    (sanitized)
$ jigc rename adr:seed-decision --to "../../x"       → adr:x       docs/decisions/x.md
$ jigc rename adr:x            --to ".git"           → adr:git     docs/decisions/git.md
$ jigc rename adr:git          --to "/etc/passwd"    → adr:etc-passwd
$ jigc rename adr:etc-passwd   --to ":(top)pwn"      → adr:toppwn  docs/decisions/toppwn.md
$ ls docs/decisions   → toppwn.md          # every landing inside the declared home, no escape
```

### C7 — `lead(codex, `DOCTYPE_DOORS` includes every bare-doctype/address leaf, `milestone add-from-spec` among them)` → **CONFIRMED (repro at that door)**

```
setup: rig committed-singletons; jigc milestone create "Recon MS"     → milestone:recon-ms
$ jigc milestone add-from-spec recon-ms 'spec:../../x#scope'   → store.malformed-slug        exit 1
$ jigc milestone add-from-spec recon-ms 'spec:/etc/passwd#scope' → store.malformed-slug      exit 1
$ jigc milestone add-from-spec recon-ms 'spec::(top)pwn#scope' → store.malformed-slug        exit 1
$ jigc milestone add-from-spec recon-ms "spec:$A300#scope"     → write.slug-name-ceiling     exit 1
$ jigc milestone add-from-spec recon-ms 'vision:alpha#thesis'  → store.fixed-identity        exit 1
```

The door M50's audit found reading a doc from outside the repository is adjudicated at the head on
every cell driven.

### C8 — `lead(codex, `WORK_UNIT_ID_DOORS` and `SLUG_DOORS` are projections of the same classification, not independent name lists)` → **CONFIRMED (source read + behavioural sample)**

Source: `cli.rs:2486-2504`, `:2589-2783`, `:2829-2938` — both derive from `ARG_TOKENS`' answer, as
Codex states and as the driver's §0 read. Behaviourally sampled by the reconciler at **8 doors × 4
cells**:

```
setup: rig committed-singletons (four open tasks, one milestone absent)
cells: ""  ·  "../.."  ·  "/etc/passwd"  ·  "no-such-id"
doors: task discard · task validate · task diff · task finalize ·
       milestone discard · milestone list-tasks · milestone provision · start --task

  "" / "../.." / "/etc/passwd"   → blocking · work-unit.malformed-id   at all 8 doors
  "no-such-id"                   → finalize.no-task  (the five task doors)
                                   milestone.unknown (the three milestone doors)
$ ls .jigc/tasks    → 4 task directories, all present   (RC-m50's `task discard ""` blocker stays closed)
```

And the `SLUG_DOORS` ceiling boundary the driver claimed, re-driven at `rename`:

```
$ jigc rename vision:vision --to T --slug "$(python3 -c "print('b'*166)")"
blocking · write.slug-name-ceiling — `--slug "<B166>"` is 166 bytes — over the 165-byte ceiling a
  minted id carries
  route: re-run with a `--slug` of at most 165 bytes …
$ jigc rename vision:vision --to T --slug "$(python3 -c "print('b'*165)")"
  → passes the ceiling and is answered by the next guard (`rename.in-flight`), i.e. 165 is admitted
```

### C9 — `lead(codex, the occurrence fence is bidirectional over every real `(leaf, argument)` pair and checks that each row's argv parses to its declared leaf and argument (`cli.rs:4413-4488`))` → **OPEN LEAD**

**Reason it stays open:** the fence is a **compile/test-time** assertion, not a behaviour any argv
against the installed binary can exhibit. Confirming it means running the `cli` test group, which on
this repo is the multi-minute gate, not a probe — and a *source read agreeing with a source pass* is
not a drive. Recorded as an open lead rather than promoted on the read. Source read is **consistent**
(the symbol exists at the cited span and iterates the real clap tree in both directions).

### C10 — `lead(codex, migration admission and destructive retirement share `resolve_source_token`, covering pathspec magic, outside-repository resolution, `.git`, `.jigc` and symlink components)` → **CONFIRMED (repro, 8 cells)**

```
setup: rig committed-singletons;  mkdir -p "$RIG/outside"
       printf '# Planted\n\nCANARY-BYTES-9f3\n' > "$RIG/outside/planted.md"
       ln -sfn "$RIG/outside" escape-link ; printf '# Untracked\n\nfoo\n' > untracked-source.md

$ jigc migrate "$RIG/outside/planted.md" --as adr → migrate.source-untrackable (outside)   exit 1
$ jigc migrate ../outside/planted.md      --as adr → migrate.source-untrackable (outside)   exit 1
$ jigc migrate /etc/passwd                --as adr → migrate.source-untrackable (outside)   exit 1
$ jigc migrate escape-link/planted.md     --as adr → migrate.source-untrackable (symlink)   exit 1
$ jigc migrate ':(top)README.md'          --as adr → migrate.source-untrackable (pathspec)  exit 1
$ jigc migrate .git/config                --as adr → migrate.source-untrackable (.git)      exit 1
$ jigc migrate untracked-source.md        --as adr → migrate.source-untracked               exit 1
$ jigc migrate -                          --as adr → could not read the foreign `adr` source at `-`
                                                     + a route, no code                     exit 1
$ cat "$RIG/outside/planted.md"  → CANARY-BYTES-9f3   unchanged
```

### C11 — `lead(codex, the unlink sink independently re-adjudicates the recorded source immediately before `remove_file`, so no raw source-path read bypasses the typed sink)` → **CONFIRMED (repro — the strongest positive on this axis)**

This is the claim worth driving rather than reading, because it is about a **second** adjudication
after the first one has already passed. The reconciler tampered with the recorded source **between**
`jigc migrate` and the retiring finalize:

```
setup: rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
       mkdir -p "$RIG/outside"; printf 'CANARY-OUTSIDE-7b2\n' > "$RIG/outside/canary.md"
       mkdir -p notes
       printf '# Foreign Decision\n\n## Context\n\nc\n\n## Decision\n\nd\n\n## Consequences\n\ne\n' > notes/foo.md
       git add -A && git commit -qm "foreign source"

1. $ jigc migrate notes/foo.md --as adr
     task minted: migrate-adr-notes-foo-d7b32c1051bf          # admission passed: tracked, in-repo
2. $ jigc doc create adr --title "Foreign Decision" --task <id>   (+ slots, commit type, summary)
3. $ ln -sfn "$RIG/outside/canary.md" notes/foo.md           # TAMPER — the recorded source is now
                                                             #  a symlink pointing outside the repo
4. $ jigc task finalize <id> --approve
     blocking · finalize.retire-untrackable — this migration's recorded source is not a path this
       repository can retire: `notes/foo.md` resolves outside the repository root
       at: notes/foo.md
       route: nothing was committed and the promote was rolled back. The retire target is recorded
              in the task's working area at `source-path`: restore it to the path `jigc migrate`
              recorded, or abandon the migration and re-run `jigc migrate` against a file inside
              this repository
     exit=1                                                                            # BARE
5. $ cat "$RIG/outside/canary.md"  → CANARY-OUTSIDE-7b2       # the outside bytes are intact
   $ git status --porcelain        → ` T notes/foo.md`        # only the tamper; nothing committed
```

The sink re-adjudicates and refuses; the promote is rolled back; the outside file the symlink
pointed at is untouched. Codex's claim holds **under an adversarial state its source read could not
construct**.

### C12 — `lead(codex, routes continue to quote caller bytes through `shell_token` / `shell_operand`; the command-span parser remains centralized)` → **CONFIRMED as stated, with its scope named**

```
setup: rig committed-singletons;  printf '# N\n' > "my notes.md"
$ jigc migrate "my notes.md" --as adr
blocking · migrate.source-untracked — `my notes.md` is in neither this repository's index nor its HEAD …
  at: my notes.md
  route: stage it with `git add -- 'my notes.md'`, then re-run `jigc migrate 'my notes.md' --as adr` …
$ git add -- 'my notes.md'          # the emitted span, verbatim
exit=0        git status → `A  "my notes.md"`
```

**Scope, stated because the two passes could otherwise be read as contradicting each other:** quoting
is not runnability. `shell_token`'s subject is *whether one emitted token survives a real shell as
exactly itself*, and a leading-`-` token does survive — it is the **receiving CLI** that then reads it
as an option. So C12 is true **and** A1-N1 stands; Codex is silent on runnability, not contradicting
it. See the driver-defect ledger below, where the reconciler widened A1-N1 to a second producer.

### C13 — `lead(codex, completeness: no omitted Axis 1 door and no caller-token bypass — "no grounded completeness defect found")` → **CONFIRMED within its stated subject**

The reconciler's independent read of `ARG_TOKENS` (C6) and its drives at `doc` · `task bind` ·
`rename` · `milestone add-from-spec` · `migrate` · `relocate` · `config set` · the 25
`WORK_UNIT_ID_DOORS` sample · the six `SLUG_DOORS` found **no argument that becomes a path component
and carries no registry row**, and no resolve seam a caller token reached unvalidated.

**Not refuted by the driver's three defects**, and this is the reconciliation's substantive point:
A1-N1 / A1-N2 / A1-N3 are **behaviour at registered doors** — a route that cannot be run, a route that
prescribes a deterministic failure, and a record whose enumeration is falsified — none of which is a
missing row or a bypass. The two passes are asking different questions of the same axis and **both
answers stand**.

### C14 — `lead(codex, zero schema-hash movement: `git diff --name-only c59ec3d7..HEAD` over both schema trees, both manifests and both snapshot trees produces no paths)` → **CONFIRMED (repro)**

```
$ git rev-parse --short HEAD    → a3eb026b
$ git diff --name-only c59ec3d7..HEAD -- \
    crates/cli/pack/config/schema-manifest.yaml packs/methodology/config/schema-manifest.yaml \
    crates/cli/pack/schemas packs/methodology/schemas
  (empty)
```

M52's declared boundary holds.

### C15 — `lead(codex, declared bound: an argument incorrectly classified `PlainValue::Other` that does reach a path could evade the occurrence registry (`cli.rs:2380-2396`))` → **OPEN LEAD**

**Reason it stays open:** it is a *possibility* statement about a future mis-answer, which no drive
can close. What the reconciler **could** do was look for an instance today, and did (C6): the three
`Other`-classified arguments whose values genuinely reach a path component — `intent`, `title`, `to`
— were driven over five escape shapes each (`../../`, `.git`, `/etc/passwd`, `:(top)…`, 300 bytes)
and **all sanitize through the slug rule**; every landing was inside the declared home. **No instance
found — the bound is unfalsified and remains, correctly, a bound.**

---

## B — driver defects, each re-driven by the reconciler

| driver defect | source pass | reconciler verdict |
|---|---|---|
| **A1-N1** (MEDIUM-HIGH) — a route whose operand's first byte is `-` dead-ends at exit 2 | **silent** (its completeness question does not reach runnability; C12 is not a contradiction) | **CONFIRMED — re-driven end to end, and WIDENED** |
| **A1-N2** (MEDIUM) — a `ROOT_KNOBS` value the door accepts makes the door's own `git mv` unparseable, and the refusal's route prescribes re-running it | **silent** | **CONFIRMED — re-driven, plus one residue note** |
| **A1-N3** (MEDIUM, record defect) — a carried deferral's enumeration of the flatten exemptions is falsified by a fourth family | **silent** | **CONFIRMED — both arms re-driven, the contract text and the deferral text read** |

### A1-N1 — re-driven whole, on a rig the reconciler built

```
setup: rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"

1. $ jigc config set docs-root -
     config: set `docs-root` = `-` — written to `.jigc/config/`, uncommitted …      exit=0
   $ jigc config get docs-root        → docs-root = -  (project)
2. $ jigc start --workflow single-task "dash"; jigc doc create adr --title "Dash Probe"
   (+ three slots, commit type `docs`, summary)
   $ jigc task finalize dash
     finalized d8d5fdb — docs: dash
       promoted -/decisions/dash-probe.md                                            exit=0
3. $ git rm -q -- "-/decisions/dash-probe.md"                                        exit=0
4. $ jigc validate
     blocking (gates at finalize) · reconciliation.rename — tracked managed doc adr:dash-probe
       (-/decisions/dash-probe.md) is missing
       at: -/decisions/dash-probe.md
       route: restore -/decisions/dash-probe.md, or confirm the deletion by dropping it from the
              index: `jigc unmanage -/decisions/dash-probe.md`
     exit=1                                                                          # BARE
5. $ jigc unmanage -/decisions/dash-probe.md          # the printed route, VERBATIM, measured BARE
     error: unexpected argument '-/' found
       tip: to pass '-/' as a value, use '-- -/'
     Usage: jigc unmanage [OPTIONS] <PATH>
     ROUTE EXIT=2
6. $ jigc unmanage '-/decisions/dash-probe.md'        → exit 2   (quoting does not help)
7. $ jigc unmanage -- -/decisions/dash-probe.md       → exit 0   (the form no surface prints)
```

**Reconciler widening — a second producer, and it needs no knob preimage at all.** The driver stated
the bound as *"the preimage needs a `docs-root`/`placement-root` whose first byte is `-`"*. Driven, an
**ordinary leading-dash filename** reaches the same class through a different producer, on a corpus
with no knob set:

```
setup: rig committed-singletons (docs-root untouched);  printf '# D\n' > ./-dash-note.md

$ jigc migrate ./-dash-note.md --as adr
blocking · migrate.source-untracked — `-dash-note.md` is in neither this repository's index nor its HEAD …
  at: -dash-note.md
  route: stage it with `git add -- -dash-note.md`, then re-run `jigc migrate -dash-note.md --as adr`
         — the index is enough, the source need not be committed first
exit=1

$ git add -- -dash-note.md            # the route's FIRST half: works (git's own `--` is in the span)
exit=0
$ jigc migrate -dash-note.md --as adr # the route's SECOND half, verbatim — jigc's own re-run
  error: unexpected argument '-d' found
    tip: to pass '-d' as a value, use '-- -d'
  Usage: jigc migrate [OPTIONS] --as <AS> <PATH>
  ROUTE EXIT=2
$ jigc migrate -- -dash-note.md --as adr   → error: unexpected argument '--as' found     exit=2
$ jigc migrate --as adr -- -dash-note.md   → task minted: migrate-adr-dash-note-64db6402e3e3  exit=0
```

Three things this adds to the driver's row, all driven: (a) the class has **at least two producers**,
not one; (b) it is reachable with **no `ROOT_KNOBS` value at all** — a filename is enough, which
raises the finding's reachability well above *"adopter-reachable through a documented knob"*;
(c) the producer emits a span for **jigc's own verb**, and jigc's own `--` placement is
non-obvious enough that the naïve fix (`jigc migrate -- <path> --as adr`) also fails — the working
form puts `--as` before the separator. Note in passing that the **same route's git half is correct**,
because `git add -- <path>` was already written with the separator; nothing generic protects the
jigc half.

### A1-N2 — re-driven

```
setup: rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
       (the corpus holds placement docs: docs/roadmap.md, docs/decisions-log.md)

$ jigc config set placement-root -
relocating the committed doc(s) stranded by the `placement-root` re-point to `-` …
blocking · config.repoint-failed — `placement-root` was not set to `-`: `git mv docs/decisions-log.md
  -/decisions-log.md` failed: error: unknown switch `/'
  usage: git mv [-v] [-f] [-n] [-k] <source> <destination> … — the re-point was undone
  at: docs/decisions-log.md
  route: `placement-root` is unchanged and every doc this re-point moved is back at its prior home.
         Fix what this message names, then re-run `jigc config set placement-root -`
(a second, identical finding for docs/roadmap.md)
exit=1                                                                                      # BARE
$ git status --porcelain      → (empty)
$ jigc config get placement-root → placement-root =   (pack-default)
```

All three of the driver's claims reproduce: the transaction is sound, the failing argv is **jigc's
own** (`git mv <src> -/… ` with no `--`), and the route prescribes an identical re-run of a
deterministic failure. The corpus-shape half reproduces too — the same value at `docs-root` lands at
**exit 0** on a corpus with nothing to move (A1-N1 step 1, above), so admissibility is decided by what
happens to be committed rather than by a value rule.

*Residue the driver did not record, added here, not graded:* after the undone re-point an **empty
`-` directory** is left in the worktree (`find . -maxdepth 1 -name '-' -type d` → `./-`). It is
untracked and empty, so `git status` stays clean and no doc is misplaced — the *"the re-point was
undone"* sentence is true about the config and the docs. Mentioned so a later reader driving this
cell is not surprised by it.

### A1-N3 — both arms re-driven, and the two documents read

```
setup: rig committed-singletons; jigc start --workflow single-task "axis one recon"

$ jigc doc set-field "roadmap:roadmap#milestones/m-beta/title" --value X --task <id> --format json
{ "schema_version": 3,
  "findings": [ { "severity":"blocking", "probe":"write", "check":"not-present",
                  "code":"write.not-present",
                  "key":{"code":"write.not-present","target":"roadmap:roadmap#milestones/m-beta/title"},
                  "location":{"address":"roadmap:roadmap#milestones/m-beta/title", …},
                  "route":"`jigc doc show roadmap:roadmap#milestones --task <id>` …" } ] }
exit=1

$ jigc doc set-field "roadmap:$A300#milestones/m-alpha/title" --value X --task <id> --format json
{ "error": "blocking · write.slug-name-ceiling — the `<slug>` head of address
            `roadmap:<A300>#milestones/m-alpha/title` is 300 bytes — over the 165-byte ceiling …" }
exit=1

$ jigc doc show "roadmap:$A300#milestones" --format json   → { "error": … }      (same flatten)
$ jigc doc show "roadmap:nosuchdoc"        --format json   → { "schema_version":3, "findings":[ …
                                                               store.fixed-identity, keyed … ] }
```

The two documents the driver cites were read at HEAD and say what the driver says they say:
`implementation/decisions-pending.md:24` carries M52 **(b)** enumerating exactly three exemptions
(`store.malformed-slug`, `pack.resource-missing`, the `workflow-refs.*` flatten path), and
`design/command-output-contract.md`'s target-form table lists **`write.*` — the address-bearing
writes** under the `type:slug#<fragment>` form. `write.slug-name-ceiling` is a `write.*` code that
**carries** a well-formed URI locus and a route, and it flattens. So the enumeration is falsified by a
fourth family. **CONFIRMED as a record defect**; the binary behaviour itself is the known wider gap,
not a new one.

### The driver's four observations, re-driven

* **O1 — the `--slug` grammar reject carries an identity at exactly 1 of 6 doors. CONFIRMED.**

```
setup: rig committed-singletons (+ one open task for the --task doors)
$ jigc start --workflow single-task "x" --slug '../../x'      → no code, no route          exit 1
$ jigc migrate README.md --as adr --slug '../../x'            → no code, no route          exit 1
$ jigc doc create adr --title X --slug '../../x' --task <id>  → no code, no route          exit 1
$ jigc doc add-item "roadmap:roadmap#milestones" --title X --slug '../../x' --task <id>
                                                              → no code, no route          exit 1
$ jigc doc rename "roadmap:roadmap" --to X --slug '../../x' --task <id>
                                                              → no code, no route          exit 1
$ jigc rename vision:vision --to "T" --slug '../../x'
  blocking · write.malformed-slug — `--slug "../../x"` is not a valid slug …               exit 1
```

  5/6 emit the bare sentence, `rename` alone carries `blocking · write.malformed-slug`. Unchanged
  from M51, as the driver says.

* **O2 — `relocate`'s partial-block arm prints a blocking finding and exits 0. CONFIRMED, with a
  state-dependency the driver's row does not name.**

```
setup: rig=$(dev/jigc-rig fresh --binary … --pack-from-dev) || exit; eval "$rig"
       mkdir -p notes docs/decisions; a foreign `notes/foo.md`, a foreign `docs/decisions/foo.md`,
       a `notes/My Notes.md`; git add -A && git commit
$ jigc relocate adr --from notes
freeze-exempt relocation: 1 moved, 1 displaced, 1 blocked
  moved     notes/foo.md -> docs/decisions/foo.md
  displaced docs/decisions/foo.md -> .jigc/displaced/foo.md (foreign squatter → workbench)
  blocked   notes/My Notes.md
    blocking · ingest.unaddressable-identity — … its name is not a doc id …
      at: notes/My Notes.md
      route: rename it to a doc id — `mkdir -p docs/decisions && git mv 'notes/My Notes.md' …`
exit=0
```

  The structural claim holds exactly: a **blocking** finding rendered inside a **success** arm the
  `ENVELOPE_ARMS` row pins, at exit 0. *Difference from the driver's cell, recorded rather than
  smoothed over:* the driver's run reported `write.already-present` for `notes/foo.md`; two
  reconciler constructions (with and without a prior `jigc ingest`) both reached the **Displace** arm
  instead, because the destination was a *foreign squatter* rather than a managed doc. So §6's
  `OccupiedDestination` row is **state-dependent** — it needs an **adopted** destination — and is
  carried on the driver's repro, not on mine. No contradiction: `Displace` is a declared disposition
  of that axis.

* **O3 — `task bind`'s role check precedes the slug guard and now carries `task-bind.undeclared-role`.
  CONFIRMED (both arms driven — the declared-role arm is C4's block above).**

```
$ jigc task bind spec 'spec:../../etc/passwd' undecl        # a `single-task` task
blocking · task-bind.undeclared-role — role `spec` is not a declared read-role of this task
  (declared: none)
```

* **O4 — pre-dispatch ordering differs between the two token families, both answers honest.
  CONFIRMED.**

```
setup: W=$(mktemp -d "${TMPDIR:-/tmp}/probe.XXXXXX"); cd "$W"     # not a git repository
$ jigc task discard "../.."
blocking · work-unit.malformed-id — "../.." is not a valid work-unit id
  route: use lowercase letters, digits, and single hyphens (no leading, trailing, or doubled `-`)
exit=1
$ jigc doc show "vision:../../etc/passwd"
not inside a git repository (no `.git` found from /private/var/…/probe.6Bfg6U) — run jigc from
  inside the target git repository; if this project isn't one yet, `git init` here first
exit=1
```

  The token guard wins at the work-unit family, the pre-dispatch fault wins at the address family.
  Carried for axis 6's reconciler, as the driver says.

---

## C — the "driven with no repro block" check

**One sub-claim was demoted-then-restored, and no graded finding is demoted.**

1. **§2/A1-D4's `task bind` "8/8 cells with a declared role"** carried no repro block in the table and
   is **not** in the driver's persisted sweeps (`driver/addr_sweep.out`'s eight `task bind` lines are
   all the *undeclared*-role arm). Under the rule it was not driven. Rather than demote it, the
   reconciler **built the state and drove it** — C4 above. It is now driven, by this pass.
2. **§1's aggregate counts — 378 matrix rows, 135 supplementary drives, 513 total invocations —**
   carry no repro block *in the table*. They are **not demoted**: the per-row evidence exists as
   persisted artifacts with the driving scripts beside them, and the reconciler opened and read all
   four —
   `driver/wuid_sweep.out` (104 lines, `door|exit|code|routes|message` per row, four cells),
   `driver/addr_sweep.out` (88 lines), `driver/slug_sweep.out` (56 lines),
   `driver/path_sweep.out` (163 lines), plus `*_sweep.sh`, `arms.json`, `minimal_argv.json`,
   `doors.sh`, `leaves.py`, `owed.py` and the rig `.env` files.
   **Transparency note for the axis reader:** the aggregate counts are auditable only through those
   artifact files, not through the table, so any later citation of "378 rows" should cite the sweep
   file, not this document.
3. **§6's `RelocateRefusal::UntrackableDestination`** is already marked *not driven* by the driver,
   with its reason (needs a manufactured schema whose home resolves inside `.git/` — M49's declared
   open engine-side axis). Correctly withheld; carried as withheld.
4. **§5's 191 n/a pairs** are each stated with a reason rather than claimed driven. Carried as
   stated. The reconciler re-read the five groups and found no pair presented as driven.

---

## D — reconciler observations (neither pass graded these; none is promoted to a finding)

* **R1 — `jigc unmanage <absolute outside-repo path>` answers `no-op` at exit 0.** Visible in
  `driver/path_sweep.out:2`, ungraded in both passes. **Matches contract:** the occurrence row is a
  *declared* `PathArgDisposition::NoRule` — *"the token is a LOOKUP KEY … never joined onto a path
  jigc reads, writes or unlinks … A spelling no record carries selects nothing and earns the
  idempotent no-op the verb documents"* (`cli.rs`, the `unmanage`/`path` row). No finding.
* **R2 — `jigc config insert-step --file <absolute outside-repo path>` lands at exit 0.** Also
  **declared**: that row's `Adjudicated` predicate is `trackable::source_read_reason`, whose stated
  rule is *"no `.git` component, not reached through jigc's transient workbench — **an out-of-repo
  source is admitted and copied in**"*. Read at `trackable.rs:391-430`: the function refuses only the
  `.git` and transient-workbench legs. No finding.
* **R3 — `jigc migrate - --as adr` answers a read fault with a route and no code** (`could not read
  the foreign `adr` source at `-`` + a route, exit 1). The token is named as the operator typed it,
  which is M51's fix; the code-lessness is the pre-existing flattened read-fault family, not a new
  shape. Recorded, not graded.
* **R4 — the empty `-` directory left by A1-N2's undone re-point** (see that block).

---

# Doors covered

Every clap leaf that is the door of ≥1 **driven** row in this reconciled file, `VERB_KINDS` spelling.
**The driver's 40 stand** (its rows are unchanged); the reconciler independently re-drove **26** of
them and added none beyond the set:

`start` · `workflow` · `migrate` · `unmanage` · `relocate` · `rename` · `validate` · `ingest` ·
`doc create` · `doc author` · `doc add-item` · `doc remove-item` · `doc retitle-item` ·
`doc rename` · `doc set-field` · `doc set-slot` · `doc show` · `doc list` · `doc schema` ·
`task diff` · `task validate` · `task discard` · `task finalize` · `task bind` · `config set` ·
`config get` · `config insert-step` · `config replace-step` · `config remove-step` · `config fill` ·
`config fork` · `milestone create` · `milestone add-task` · `milestone add-from-spec` ·
`milestone list-tasks` · `milestone provision` · `milestone execute` · `milestone join` ·
`milestone finalize` · `milestone discard`

**The 26 the reconciler re-drove itself**, each with its repro block above:
`start` · `migrate` · `unmanage` · `relocate` · `rename` · `validate` · `ingest` · `doc create` ·
`doc add-item` · `doc rename` · `doc set-field` · `doc set-slot` · `doc show` · `doc list` ·
`task bind` · `task diff` · `task validate` · `task discard` · `task finalize` · `config set` ·
`config get` · `milestone create` · `milestone add-from-spec` · `milestone list-tasks` ·
`milestone provision` · `milestone discard`

The other 14 of the driver's 40 are carried on the driver's own rows. Two leaves the reconciler
reasoned about but did **not** drive an argv at — `config insert-step` (R2) and `doc schema` — are
named in this file from the source registry and the driver's persisted sweep, and are counted in the
driver's 40, not in the 26.

---

## Counts

* **Codex claims: 15** — **13 CONFIRMED** (12 with a repro block; C6's source half is a read that
  agrees with the driver's independent read, its drivable half driven), **0 REFUTED**, **2 OPEN
  LEADS** (C9 — a compile/test-time fence no argv can exhibit; C15 — a possibility statement about a
  future mis-answer, with no instance found among the three `Other`-classified path-reaching
  arguments driven).
* **Driver defects: 3** — **3 CONFIRMED** (all re-driven; **A1-N1 widened** to a second producer
  reachable with no knob preimage). **0 refuted, 0 demoted.**
* **Driver observations: 4** — **4 CONFIRMED** (O2 with a named state-dependency).
* **Rows demoted: 0.** One un-reproduced sub-claim (§2/A1-D4's `task bind` 8/8) was driven by the
  reconciler instead of demoted; §1's aggregate counts are carried with their artifact citation and a
  transparency note.
* **Reconciler observations: 4**, none promoted.
* **Integrity, asserted at the end of every rig:** no canary planted outside a repository was read,
  moved or removed; `.jigc/tasks/` survived every `task discard` cell; no `rm -rf` on a variable path
  was run anywhere in this pass; no repository edit, no commit, no fix.
