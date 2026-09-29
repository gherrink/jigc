<!-- M51 per-axis review — axis 1 · reconciled · driven on the installed `jigc 1.0.0-rc.15` (commit 577a0099), 2026-09-16 -->

# M51 per-axis review — AXIS 1 · caller tokens — RECONCILED

**What this file is.** Part I is the Opus driver's table, carried over **unchanged** (no row was
demoted — see §8 for the demotion pass and what it found instead). Part II is the reconciliation
ledger: every Codex source-pass claim entered as a lead and then driven, and every driver defect
re-driven. Part III is the doors-covered list.

**Reconciliation binary asserted:** `/Users/maurice/.local/bin/jigc --version` → `jigc 1.0.0-rc.15`
(RELEASE posture — the debug-only `debug_assert!` route fences do not exist in it). Repo at
`577a0099`. Every rig in Part II was built the way the driver built its own:
`rig=$(dev/jigc-rig <state> --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"`, two-step,
every root from `mktemp -d`, no `rm -rf` on a variable path anywhere.

---

# PART I — the driver table (carried unchanged)

**Binary asserted first:** `/Users/maurice/.local/bin/jigc --version` → `jigc 1.0.0-rc.15`. RELEASE
posture — the debug-only route-fence `debug_assert!`s do not exist here.

**All rigs:** `rig=$(dev/jigc-rig <state> --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"`
(two-step eval throughout; no `rm -rf` on a variable path anywhere; every root from `mktemp -d`).
States used: `committed-singletons` (most), `fresh --pack-from-dev` (the only way to reach a
freeze-exempt doctype).

**Scope discipline:** the Codex source pass for this axis was **not** read.

---

## 0 — the door set, derived from the code (counts I read, not the design doc's numbers)

Read at `HEAD` (`67c0c369` + the four post-build audit fixes `6c2391c0` / `da5173a1` / `ff2bde99` /
`507c332d`, all present in the installed rc.15):

| registry | file:symbol | count I read | note |
|---|---|---|---|
| `ARG_TOKENS` | `crates/cli/src/cli.rs` | **35** ids | total over the clap tree; 6 `Plain(PathBearing)`: `path` `file` `from_file` `from` `target` `value` |
| `PATH_ARG_OCCURRENCES` | `cli.rs` | **14** occurrences / **18** arms / **12** distinct leaf verbs | the D1 registry |
| `DOCTYPE_DOORS` | `cli.rs` | **16** rows — **10** `Address`, **6** `Bare` | design text says 10 Address ✓ |
| `SLUG_DOORS` | `cli.rs` | **6** | |
| `WORK_UNIT_ID_DOORS` | `cli.rs` | **25 rows / 25 doors** | **the acceptance design says "26 rows / 25 doors" — the code carries 25 rows.** The registry's own doc-comment says "Twenty-five doors." Recorded as a doc/code count divergence, not a defect of the binary |
| `VERB_KINDS` | `cli.rs` | **47** leaves | |
| `BEHALF_DOORS` | `cli.rs` | **47** rows — 35 `Neither`, **10** `CommitsOnBehalf`, **2** `MovesOnBehalf` (`relocate`, `config set`) | |
| `COMMITTING_DOORS` | `invocation_log.rs` | **10** rows | |
| `DESTROYING_DOORS` | `milestone.rs:2625` | **4** (`provision`, `discard`, `uninstall`, `finalize`) | |
| `ENVELOPE_ARMS` | `render.rs` | **60** arms | |
| `STORE_EXIT_FLIPS` | `render.rs:948` | **6** (`probe-unreliable`, `oob-rename`, `unmigrated-corpus`, `ahead-corpus`, `orphaned-instance`, `foreign-squatter`) | the M51 orphan member is present |
| `ManifestKind` | `render.rs:1819` | **6** variants | |
| `SchemaChangeKind` | `engine/src/schema_diff.rs` | **18** variants | |

**Axis-1 door set** = `PATH_ARG_OCCURRENCES` (12 verbs) ∪ `DOCTYPE_DOORS ▸ Address` (10) ∪
`SLUG_DOORS` (6) ∪ `WORK_UNIT_ID_DOORS` (25) = **35 distinct leaf verbs** (union computed over the
four registries, not summed).

**Cell set** (acceptance design Part 2, axis 1): absolute · `../` escape · symlink escape · `.git/`
component · workbench root · untracked in-repo · leading-colon pathspec magic · the `-` stdin
sentinel · OS name ceiling · well-formed control.

---

## 1 — counts

* **Matrix rows driven: 368** — every one's argv ran on the installed rc.15 with its verdict
  recorded, plus **182 supplementary drives** (contamination re-drives, boundary walks, class-bounding
  sweeps, consequence drives, JSON/invocation-log funnels) = **550 invocations** of the installed
  binary in this review.
  * Group D (`WORK_UNIT_ID_DOORS`): 25 doors × 4 cells = **100**
  * Group B (`DOCTYPE_DOORS ▸ Address`): 10 doors × 7 cells = **70**
  * Group C (`SLUG_DOORS`): 6 doors × 8 cells = **48**
  * Group A (`PATH_ARG_OCCURRENCES`): 15 occurrence-arms × 10 cells = **150**
* **Matrix rows n/a: 191** — enumerated with reasons in §5.
* **Defects: 5** (1 HIGH, 1 MEDIUM-HIGH, 3 MEDIUM). 2 further observations recorded as
  *matches-contract-with-a-note*.
* **Doors covered (door of ≥1 driven row): 39** — all **35** axis-1 door-set members, plus **4**
  leaves driven as part of a row's setup or consequence assertion (§7).

---

## 2 — the driven table

### 2.1 `WORK_UNIT_ID_DOORS` — 25 doors × 4 cells (100 rows driven)

One repro block; the argv differs only by door, so the drive is grouped per the prompt.

```
setup:  rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit
        eval "$rig"; printf 'payload.yaml fixture\n' > payload.yaml
argv:   every WORK_UNIT_ID_DOORS row's own `argv`, with WORK_UNIT_ID_SLOT substituted by
        "" | "../.." | "/etc/passwd" | "ghost-task-x"
observed: 100/100 exit 1. Codes, verbatim from the run:
  ""            → work-unit.malformed-id   at all 25 doors
  "../.."       → work-unit.malformed-id   at all 25 doors
  "/etc/passwd" → work-unit.malformed-id   at all 25 doors
  "ghost-task-x"→ finalize.no-task         at 17 task doors
                  milestone.unknown        at the 8 milestone doors
```

| door set | cell | exit | code | route kind | surface asserted | verdict |
|---|---|---|---|---|---|---|
| all 25 | empty `""` | 1 | `work-unit.malformed-id` | `Human` (the grammar) | `blocking · work-unit.malformed-id — "" is not a valid work-unit id` + `route: use lowercase letters…` | matches contract |
| all 25 | `../..` | 1 | `work-unit.malformed-id` | `Human` | same shape, token echoed | matches contract |
| all 25 | absolute `/etc/passwd` | 1 | `work-unit.malformed-id` | `Human` | same | matches contract |
| 17 task doors | well-formed unknown | 1 | `finalize.no-task` | `Mechanical` `jigc task list` (at 16) / `jigc milestone list-tasks <milestone-id>` (at `workflow`) | findings envelope `{schema_version:3, findings:[{key:{code,target:"task:ghost-task-x"}…}]}` | matches contract |
| 8 milestone doors | well-formed unknown | 1 | `milestone.unknown` | `Human` (`create it first with …`) | envelope, `target:"milestone:ghost-mile"` | matches contract |

Notes recorded, not defects:
* `jigc workflow … --task <unknown>`'s route is `jigc milestone list-tasks <milestone-id>`, not
  `jigc task list`. Deliberate and documented at `start.rs:2165-2178` (a `workflow --task` re-entry
  is a milestone sub-agent's). **matches contract.**
* The **unknown**-id cell answers the *findings envelope* on `--format json`; the **malformed**-id
  cell answers the flattened `{"error": …}`. Driven and parsed:

```
argv: jigc task validate ghost-task-x --format json   → stderr parses OK, keys [schema_version, findings]
argv: jigc task validate ../..        --format json   → stderr parses OK, keys [error]
```

  That split is the M50 completion audit's **recorded, measured** decision (the malformed guards
  carry `location: None`, so the envelope key would be `(code, null)`). Both are valid JSON — I
  hexdumped the flattened one to be sure the embedded newline is escaped (`\ n` two bytes, `od -c`
  confirms). **matches contract.** Everything goes to **stderr**, stdout empty — stream discipline
  holds.

### 2.2 `DOCTYPE_DOORS ▸ Address` — 10 doors × 7 cells (70 rows driven)

```
setup:  rig=$(dev/jigc-rig committed-singletons …) || exit; eval "$rig"
        jigc start --workflow single-task "axis one"       # task axis-one
        jigc milestone create "Axis milestone"             # milestone axis-milestone
argv:   rename vision:<S> --to Axis
        doc add-item roadmap:<S>#milestones --title Axis --task axis-one
        doc remove-item roadmap:<S>#milestones/m-alpha --task axis-one
        doc retitle-item roadmap:<S>#milestones/m-alpha --title Axis --task axis-one
        doc rename vision:<S> --to Axis --task axis-one
        doc set-field roadmap:<S>#milestones/m-alpha/title --value Axis --task axis-one
        doc set-slot vision:<S>#thesis --from-file - --task axis-one
        doc show vision:<S>
        task bind spec vision:<S> axis-one          (re-driven on a declared role, below)
        milestone add-from-spec axis-milestone spec:<S>
   with <S> ∈ ../../etc/passwd | /etc/passwd | .git/config | .jigc/version | :(top)vision | <300×'a'> | vision
```

| cell | exit | code | route | verdict |
|---|---|---|---|---|
| `../` escape | 1 at 10/10 | `store.malformed-slug` | `Mechanical` `jigc doc list` + grammar | matches contract |
| absolute | 1 at 10/10 | `store.malformed-slug` | same | matches contract |
| `.git/` component | 1 at 10/10 | `store.malformed-slug` | same | matches contract |
| workbench root | 1 at 10/10 | `store.malformed-slug` | same | matches contract |
| leading-colon magic | 1 at 10/10 | `store.malformed-slug` | same | matches contract |
| **OS name ceiling (300)** | 1 at 10/10 | **5 doors: none** · `doc show` `store.not-found` · `milestone add-from-spec` `store.not-found` · `rename` `rename.in-flight` · `doc rename` `write.identity-change` · `task bind` `store.not-found`-ish bare | **5 doors: none** | **DEFECT A1-D4** |
| well-formed control | 0/1 as the door's own state dictates (`doc add-item` 0, `doc remove-item` 0, `doc set-slot` 0, `doc show` 0) | — | — | matches contract |

`task bind` re-driven with a **declared** role (`implement-from-spec` declares `reads:[{role: spec,
type: spec}]`), because with an undeclared role the role check masks the slug guard:

```
setup:  rig=$(dev/jigc-rig committed-singletons …) || exit; eval "$rig"
        jigc start --workflow implement-from-spec "axis bind"     # task axis-bind
argv:   jigc task bind spec "spec:<S>" axis-bind
observed: ../../etc/passwd | /etc/passwd | .git/config | .jigc/version | :(top)x
            → exit 1 · store.malformed-slug · route `jigc doc list` …            (5/5)
          <300×'a'> | ghost → exit 1 · code-less `no such doc \`spec:…\``
```

### 2.3 `SLUG_DOORS` — 6 doors × 8 cells (48 rows driven)

```
setup:  rig=$(dev/jigc-rig committed-singletons …) || exit; eval "$rig"
        printf '# Changelog\n\n## 1.0.0\n- a thing\n' > foreign-changelog.md
        jigc start --workflow single-task "axis c"
argv:   each SLUG_DOORS row's own argv, --slug <S>, for
        <S> ∈ ../../x | /tmp/x | .git/x | .jigc/x | :(top)x | 166×'a' | 165×'a' | ok-slug
```

| cell | `start` | `migrate` | `rename` | `doc create` | `doc add-item` | `doc rename` | verdict |
|---|---|---|---|---|---|---|---|
| `../` escape · absolute · `.git/` · workbench · colon (5 cells) | exit 1, **code-less** | exit 1, **code-less** | exit 1 `write.malformed-slug` | exit 1, **code-less** | exit 1, **code-less** | exit 1, **code-less** | matches contract (recorded asymmetry, §4-O1) |
| OS name ceiling (166 = ceiling+1) | 1 `write.slug-name-ceiling` | 1 same | 1 same | 1 same | 1 same | 1 same | **matches contract — 6/6, EC-28 landed** |
| ceiling boundary (165 = `SLUG_NAME_CEILING`) | 0, `task minted: <165×a>` | door's own state | door's own state | door's own state | door's own state | door's own state | matches contract (the constant is exact) |
| well-formed control | 0 | `migrate.source-untracked` (source untracked — the *untracked in-repo* cell of `migrate <path>`, driven here too) | `rename.in-flight` | door's own state | door's own state | door's own state | matches contract |

### 2.4 `PATH_ARG_OCCURRENCES` — 15 occurrence-arms × 10 cells (150 rows) + 49 clean re-drives

```
setup:  rig=$(dev/jigc-rig committed-singletons …) || exit; eval "$rig"
        OUT="$RIG/outside"; mkdir -p "$OUT"; printf '# Planted\n\nCANARY-BYTES-9f3\n' > "$OUT/planted.md"
        ln -s "$OUT" escape-link
        printf '…' > untracked-source.md
        printf '…' > tracked-source.md && git add tracked-source.md && git commit -qm "tracked source"
        jigc start --workflow single-task "axis a"        # task axis
tokens: absolute=$OUT/planted.md · traversal=../outside/planted.md · symlink=escape-link/planted.md
        · dotgit=.git/config · workbench=.jigc/version · untracked=untracked-source.md
        · colon=:(top)tracked-source.md · stdin=- · ceiling=<300×'a'>.md · control=tracked-source.md
```

| # | occurrence · arm | disposition declared | driven result over the 10 cells | verdict |
|---|---|---|---|---|
| 1 | `migrate` `<path>` · always | Adjudicated → `migrate.source-untrackable`, `migrate.source-untracked` | absolute/traversal/symlink/dotgit/workbench/colon → **`migrate.source-untrackable`** with the cell-specific clause ("resolves outside the repository" · "inside git's own directory" · "inside jigc's own workbench" · "begins with `:`"); untracked → **`migrate.source-untracked`**; control → exit 0, `task minted: migrate-changelog-tracked-source-83a6c34e0543`; stdin `-` and ceiling → exit 1 **code-less** "could not read the foreign `changelog` source at `-`" (the arm declares no `-` literal, so `-` is an ordinary caller path that does not exist) | matches contract |
| 2 | `unmanage` `<path>` · always | NoRule (lookup key) | **all 10 cells → exit 0** `no-op: <token> is not managed (nothing to drop)`; no filesystem op, canary untouched | matches contract — the no-rule's claim is driven true |
| 3 | `relocate` `--from` · always | NoRule (prefix over committed spellings) | **unreachable at every shipped doctype** (14 probed, all frozen) → **DEFECT A1-D3**; reached on a manifest-less pack → **DEFECT A1-D2** (`--from .jigc` / `--from .claude` moves jigc's own artifacts) | **DEFECT ×2** |
| 4 | `config insert-step` `<file>` · always | Adjudicated → `config.step-source-untrackable` | dotgit → **`config.step-source-untrackable`**; **transient workbench** (`.jigc/tasks/<t>/docs/…`) → same code; absolute/traversal/symlink/untracked/non-transient-workbench (`.jigc/config/packs.yaml`)/control → exit 0, step copied in (the SOURCE rule declares out-of-repo admitted); colon/stdin/ceiling → exit 1 code-less "could not read source step file" | matches contract |
| 5 | `config replace-step` `<file>` · always | Adjudicated → same code | identical answers; re-driven on **distinct anchors** after the first drive consumed `#locate` | matches contract |
| 6 | `config fill` `--from-file` · `-` | NoRule (stdin sentinel) | exit 0, fill written, no token reaches the filesystem | matches contract |
| 7 | `config fill` `--from-file` · path | NoRule (`-` hands identical bytes) | absolute/traversal/symlink/dotgit/workbench/untracked/control → exit 0; colon/ceiling → code-less read failure | matches contract |
| 8 | `doc set-slot` `--from-file` · `-` | NoRule | exit 0, `set slot … (0 chars)` | matches contract |
| 9 | `doc set-slot` `--from-file` · path | NoRule | every readable cell reaches the slot (incl. `.git/config` at exit 0 — *declared* admitted, since `-` hands the same bytes); markdown sources trip `write.slot-heading-depth`, which is the slot-content guard, not a path guard | matches contract |
| 10 | `doc author` `--from-file` · `-` | NoRule | `write.wrong-shape` on the empty payload — the grammar guard, not a path guard | matches contract |
| 11 | `doc author` `--from-file` · path | NoRule | every cell → `write.wrong-shape` (payload grammar) or code-less read failure; no token becomes a path component | matches contract |
| 12 | `config replace-step` `<target>` · always | NoRule (include-list position) | all 10 cells → `config.anchor-absent` (or `config.step-id-collision` when the *source* basename collides) — **no `.jigc/config/steps/<token>.yaml` is ever created**; `ls .jigc/config/steps/` after the sweep holds only basenames of real sources | matches contract |
| 13 | `config remove-step` `<target>` · always | NoRule (writes no native file) | all 10 → `config.anchor-absent` | matches contract |
| 14 | `config fill` `<target>` · always | NoRule (`{{fill:}}` closed vocabulary) | all 10 → `config.fill-point-absent` | matches contract |
| 15 | `config fork` `<target>` · always | NoRule (`check_anchor_present`) | all 10 → `config.anchor-absent` | matches contract |
| 16 | `config set` `<value>` · `key ∈ ROOT_KNOBS` | Adjudicated → `config.untrackable-root` · `config.workbench-root` · `config.unusable-root` | absolute → `config.unusable-root`; traversal/symlink/dotgit → `config.untrackable-root`; workbench → `config.workbench-root`; colon → `config.unusable-root`; untracked/control (file-shaped) → `config.unusable-root`; **stdin `-` → exit 0**; **ceiling → exit 0 → DEFECT A1-D5** | **DEFECT** |
| 17 | `config set` `<value>` · `key ∉ ROOT_KNOBS` | NoRule (scalar, typed) | all 10 → `config.value-rejected` with the enum members listed | matches contract |
| 18 | `doc set-field` `<value>` · always | NoRule (field's declared type; `ref` and `owned-location` carry their own rules) | 10/10 on an `enum` field → `write.malformed-value`; control `accepted` → exit 0; `ref` field (`supersedes`) → `write.malformed-value` "is not a ref"; **`owned-location`** → write accepts all 10 at exit 0 **and the owner-artifact gate refuses them at `jigc task validate` (exit 3)** with the cause named per cell ("is absolute, not a repo-relative path" · "contains a `..` component" · "is not under the owned artifact home `completions/artifacts/<milestone>/`") | matches contract — the declared deferral to the gate is real and complete over the escape shapes |

**The composite assertion, driven once at the end of the group-A rig:**

```
$ cat "$OUT/planted.md"          → "# Planted\n\nCANARY-BYTES-9f3\n"   (28 bytes, unchanged)
$ git log --oneline -1            → ff29002 tracked source              (HEAD unmoved)
$ git status --short              → only .jigc/config/{fills,steps,manifest.yaml}, escape-link,
                                     untracked-source.md — nothing outside the repository written
```

The canary planted outside the repository reached **no stream**: it was never printed, copied or
retired by any of the 150 drives.

---

## 3 — defects

### A1-D1 (HIGH) — a bogus `<slug>` head on a **singleton** is accepted by five write doors, and `finalize` silently drops all but one authored payload

**Door(s):** `doc set-slot`, `doc add-item`, `doc retitle-item`, `doc remove-item`, `doc set-field`
(`DOCTYPE_DOORS ▸ DoctypeArg::Address`). **Consequence door:** `task finalize`
(∈ `COMMITTING_DOORS`). **Cell:** *well-formed control* — the cell the axis treats as the pass cell.

```
setup:  rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit
        eval "$rig"; jigc start --workflow single-task "repro"      # task repro

argv/observed (verbatim):
  $ jigc doc set-slot "adr:ghost#context" --from-file - </dev/null
    exit 1  — no staged instance for `adr:ghost#context` — provision it first …      ← non-singleton: REFUSED

  $ printf 'PAYLOAD-ALPHA\n' | jigc doc set-slot "vision:alpha#thesis" --from-file -
    exit 0  set slot vision:alpha#thesis (14 chars) (copied in for update — …)
  $ printf 'PAYLOAD-BETA\n'  | jigc doc set-slot "vision:beta#thesis"  --from-file -
    exit 0  set slot vision:beta#thesis (13 chars) (copied in for update — …)

  $ jigc doc show vision:alpha --task repro
    exit 1  blocking · store.not-found — `vision:alpha` names no doc staged in task `repro`:
            `vision` is a singleton, so its only address is `vision:vision`
            route: `jigc doc show vision:vision --task repro` …          ← the READ door knows

  $ ls .jigc/tasks/repro/docs/
    commit:repro.md  provenance.json  vision:alpha.md  vision:beta.md   ← two path components
                                                                          minted from the token

  $ jigc doc set-field commit:repro#header/type --value docs
  $ printf 'repro\n' | jigc doc set-slot commit:repro#summary --from-file -
  $ jigc task finalize repro
    exit 0
      advisory · file-state.staged-copy — staged copy of `VISION.md` …   (printed twice)
      finalized <sha> — docs: repro commit
        promoted VISION.md
        1 file committed
  $ sed -n '/## Thesis/,/## Invariants/p' VISION.md
    ## Thesis
    PAYLOAD-BETA          ← PAYLOAD-ALPHA is gone; no finding, no warning, exit 0
```

**The class, bounded by driving** (same rig, `sweep` task):

| door | bogus singleton slug | exit | result |
|---|---|---|---|
| `doc set-slot vision:bogus#thesis` | yes | **0** | mints `vision:bogus.md` |
| `doc add-item roadmap:bogus#milestones --title Phantom` | yes | **0** | mints `roadmap:bogus.md` |
| `doc retitle-item roadmap:bogus#milestones/m-alpha` | yes | **0** | writes into it |
| `doc remove-item roadmap:bogus#milestones/m-alpha` | yes | **0** | writes into it |
| `doc set-field changelog:bogus#releases/1-0-0/date --value 2026-01-01` | yes | **0** | mints `changelog:bogus.md` |
| `doc show vision:bogus` (committed) | — | 1 | `store.not-found`, names the singleton rule |
| `doc show roadmap:bogus --task sweep` (staged) | — | 1 | `store.not-found`, names the singleton rule |
| `doc rename vision:bogus --to Phantom` | — | 1 | `write.identity-change` |

**What it contradicts.** `crate::task::reject_malformed_slug_head`'s own doc-comment states the rule
as *"**one** code for the whole family, **the read doors and the write doors alike, because it is one
fault**: the `<slug>` half of a `<type>:<slug>` address is what names the file, and a token that is
not a slug names no doc anywhere."* A bogus slug on a singleton **is** a token that names no doc
anywhere — the read door says so, in those words — and the write doors take it and turn it into a
path component. And M51's own claim (*"no caller-supplied token … reaches a door that destroys,
**commits** or moves without that door having adjudicated it"*) fails at `task finalize`, which
committed one of two authored payloads and dropped the other at exit 0.

**Honest qualification:** it is not *literally* silent — `finalize` prints one
`file-state.staged-copy` advisory per staged copy, so two advisories name `VISION.md`. But no
surface says a payload was dropped, the ack says *"1 file committed"*, and the advisory's own route
is *"no action needed"*. A driver keying on findings sees nothing blocking.

**Also reached inside this class** (a milder shape, same root): with ≥2 bogus-slug copies whose
required slots are empty, `task finalize` blocks at exit 3 with routes pointing at
`jigc doc set-slot vision:<bogus>#thesis` — an address `jigc doc show` refuses — so the only ways
out are filling the phantoms or `jigc task discard`.

### A1-D2 (MEDIUM-HIGH) — `jigc relocate <freeze-exempt type> --from .jigc` moves jigc's own install artifacts into the doctype's home at exit 0

**Door:** `relocate` (`PATH_ARG_OCCURRENCES` `from` · `BEHALF_DOORS` → **`MovesOnBehalf`**).
**Cell:** workbench root.

```
setup:  rig=$(dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc --pack-from-dev) || exit
        eval "$rig"          # --pack-from-dev DROPS the freeze manifest, so `adr` is freeze-exempt

$ git ls-files
.claude/settings.json  .claude/skills/jigc/SKILL.md  .jigc/.gitignore  .jigc/AGENT.md
.jigc/config/.gitkeep  .jigc/config/packs.yaml  .jigc/version  CLAUDE.md  README.md

$ jigc relocate adr --from .jigc
freeze-exempt relocation: 1 moved, 0 displaced, 0 blocked
  moved     .jigc/AGENT.md -> docs/decisions/AGENT.md
exit=0
$ git status --short
R  .jigc/AGENT.md -> docs/decisions/AGENT.md

$ git reset -q --hard HEAD
$ jigc relocate adr --from .claude
freeze-exempt relocation: 1 moved, 0 displaced, 0 blocked
  moved     .claude/skills/jigc/SKILL.md -> docs/decisions/SKILL.md
exit=0
```

**Mechanism, read after driving:** `is_stranded(rel, prior, current) = prior.contains(rel) &&
!current.contains(rel)` — pure prefix containment over `git ls-files`, filtered to `.md`. No stamp,
no conformance, no identity.

**What it contradicts.**
1. The registry's own no-rule for this arm: *"the prior home is a PREFIX matched against the
   committed spellings … **never a path opened, written, or handed to git as a pathspec**"* — driven,
   the door `git mv`s the selected file.
2. `jigc relocate --help`: *"Relocate a freeze-exempt doctype's pre-existing committed **instance(s)**
   … each **stranded instance** is `git mv`d byte-faithful"*. `.jigc/AGENT.md` and
   `.claude/skills/jigc/SKILL.md` are not instances of `adr` in any sense — they are jigc's **own**
   adapter artifacts, the ones M48's refuse-to-clobber protects at `setup` and `uninstall` takes
   back. Calling them instances is a law-1 lie on the help text and on the ack.
3. The workbench-root rule that **does** ship at the two sibling doors: `config set docs-root .jigc`
   → `config.workbench-root`; `jigc migrate .jigc/version` → `migrate.source-untrackable` *"is
   inside jigc's own workbench (`.jigc/`)"*. The same token at the third door is taken.
4. M51's claim clause for **movers** — `relocate` is one of only two `MovesOnBehalf` doors.

**Bounds, stated:** recoverable (the rename is staged, `git status` shows it, HEAD unmoved) and it
needs a freeze-exempt doctype, which no shipped doctype is (see A1-D3). It is adopter-reachable
through a PB-1 project-pack doctype, which M49 ships as a supported capability.

### A1-D3 (MEDIUM) — the `relocate`/`from` registry row's declared runnable argv cannot reach its own arm

```
setup:  rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
argv:   jigc relocate <ty> --from docs/old    for ty ∈ {vision roadmap decisions-log changelog adr
        spec prd arch-doc idea research milestone-record completion-record deferral-ledger planning-record}
observed: 14/14 exit 1, CODE-LESS, route-less:
  `<ty>` is a frozen doctype — relocate it through the version-gated `jigc migrate-corpus`,
  not the freeze-exempt path
```

`PathArgArm::argv`'s doc-comment: *"A **runnable** argv for this arm … every other argument present
and well-formed, **so the token is the only thing the door can fault on**."* The declared argv is
`["relocate", "adr", "--from", PATH_ARG_SLOT]`; driven, `adr` faults first, unconditionally, at every
doctype the shipped packs define. So the row is satisfied by a fence that drives a door which never
reads its subject — the axis suite proves nothing about the token.

Recorded alongside: the frozen-doctype refusal itself carries **no code and no route** on the text
surface and flattens to `{"error": …}` on `--format json`.

### A1-D4 (MEDIUM) — the OS-name-ceiling cell at the address `<slug>` head leaks a bare OS error at five write doors

```
setup:  rig=$(dev/jigc-rig committed-singletons …) || exit; eval "$rig"
        jigc start --workflow single-task "ceil"                 # task ceil
argv:   jigc doc set-slot "vision:$(python3 -c "print('a'*300)")#thesis" --from-file -
observed: exit 1
  could not copy in `vision:<A300>#thesis` for editing: File name too long (os error 63)
  (no `blocking ·` prefix, no code, no route, no `at:`)
  --format json (stderr): {"error": "could not copy in `vision:<A300>#thesis` for editing: File name too long (os error 63)"}
same shape at: doc add-item · doc remove-item · doc retitle-item · doc set-field   (5 doors)
boundary walk (head length → exit):  100→0  160→0  166→0  200→0  240→1  245→1  250→1
```

`crate::task::SLUG_NAME_CEILING_CODE`'s doc-comment names this exact surface as the defect M51
Increment 9 / EC-28 fixed at `--slug`: *"printed the OS error bare — no code, no route, no `at:`"*.
The fix was applied over the `ArgToken::SlugOverride` family (`--slug`, 6 doors — verified 6/6 in
§2.3) and not over the `DoctypeArg::Address` family, whose `<slug>` head becomes the **same** path
component `.jigc/tasks/<id>/docs/<type>:<slug>.md`. The complete-fix lens, one token family over.

**Counter-argument recorded:** `SLUG_NAME_CEILING_CODE`'s own scope sentence is *"One flag, one
ceiling, six doors"* — explicitly the flag. A defender can class the address head as an
**undeclared** cell rather than a violated rule. It is a defect against the *acceptance design's*
axis-1 cell set, which applies the OS-name-ceiling cell to the whole door set including
`DOCTYPE_DOORS ▸ Address`, and against surface-contract **law 3** (the ceiling binds here and is
stated nowhere this door prints). It is *not* a route-floor breach as that floor is written, since a
bare `anyhow` carrying no command span is explicitly bounded out
(`design/surface-contract.md` → *"Route text that lives in anyhow error strings … bounded to shipped
verbs' error strings that carry command spans"*).

### A1-D5 (MEDIUM) — `jigc config set docs-root <component over NAME_MAX>` is accepted at exit 0 and leaves `jigc validate` unable to run

```
setup:  rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
$ jigc config set docs-root "$(python3 -c "print('a'*300)")"
  config: set `docs-root` = `aaaa…` — written to `.jigc/config/`, uncommitted …
  exit=0
$ jigc validate                                  # measured BARE, not through a pipe
  validating the committed store at "/private/var/…/repo": File name too long (os error 63)
  exit=1
$ jigc validate --format json
  {"error": "validating the committed store at \"…\": File name too long (os error 63)"}
  exit=1
```

`config set` is the axis's other `MovesOnBehalf` door, and its `value` occurrence is one of only
three **Adjudicated** rows. Its declared predicate (`untrackable_reason · is_workbench_root ·
unusable_root_reason`) covers absolute · edge whitespace · git pathspec magic · symlinked or
file-shaped component — **not** nameability. Driven, a home the OS cannot name is accepted and the
store becomes unvalidatable, answered by a code-less OS error.

**Honest correction of my own first measurement:** I first read `validate`'s exit through
`… | head -4` and recorded exit 0 — head's status, the exact zsh trap. Re-measured bare, it is
**exit 1**, so there is **no false green**. The defect is the accepted home and the code-less
refusal, not a wrong exit code.

Sibling cell in the same row, recorded and **not** graded a defect: `jigc config set docs-root -`
exits 0. `-` is a legal POSIX directory name and no declared leg refuses it; downstream behaviour
was not driven far enough to claim harm.

---

## 4 — observations (matches contract, with a note)

**O1 — the `--slug` grammar reject carries an identity at one of six doors.** Driven, `jigc rename
… --slug '../../x'` answers `blocking · write.malformed-slug` with a route; `start`, `migrate`,
`doc create`, `doc add-item`, `doc rename` answer the identical *sentence* with **no code, no
`blocking ·` prefix, no route**. The invocation log agrees:

```
$ jigc config set invocation-log true
$ jigc doc create adr --title X --slug '../../x'    → finding_codes = []        error_code = None
$ jigc task discard '../..'                          → finding_codes = ['work-unit.malformed-id']
$ jigc doc show 'vision:../../etc/passwd'            → finding_codes = ['store.malformed-slug']
$ jigc doc create adr --title X --slug <300×'a'>     → finding_codes = ['write.slug-name-ceiling']
```

This is **already on the record** (M50 Increment 2: *"a mint rather than the reuse the plan expected
because the five siblings refuse with a code-less bare `anyhow`"*), so it is not a new defect — but
it is worth noting that the *ceiling* code reaches all six doors while the *grammar* code reaches
one, and that the log therefore has an identity for one of the two faults at the same argument.

**O2 — the malformed-id / malformed-slug identities DO reach the invocation log.** The M50 VERDICT
carried a declared bound that the malformed-id guard's identity *"reaches the printed surface and
not yet the invocation log"*. Driven on rc.15 (above), `work-unit.malformed-id`,
`store.malformed-slug` and `write.slug-name-ceiling` all appear in `finding_codes`. That bound reads
as discharged for these three; only the code-less `--slug` grammar reject logs nothing.

**O3 — `task bind`'s role check precedes the slug guard** and is code-less/route-less
(`role \`spec\` is not a declared read-role of this task (declared: none)`). With a declared role the
guard fires correctly (§2.2). Recorded because a reviewer driving the registry's obvious argv on
`single-task` would see the guard as absent when it is present.

**O4 — two doors disagree, both by declaration, about whether `.git/` is authored content.**
`jigc config insert-step .git/config` → `config.step-source-untrackable` (*"git's private files are
not authored content"*); `jigc doc set-slot vision:vision#thesis --from-file .git/config` → **exit 0**,
192 chars of git's config into the vision thesis. Both are the registry's declared answers (the
second's stated ground: `-` hands the same bytes). Not a defect; the divergence is exactly the
declared bound that the `Plain` family ships as *one stated rule or one stated no-rule per member*
rather than one shared predicate.

---

## 5 — (door, cell) pairs I did **not** drive, and why

**191 (door, cell) pairs**, in five groups, each stated rather than presented as driven:

1. **`relocate` × the 9 non-control cells on a *stock* corpus — 9 pairs** — unreachable: every shipped doctype
   is frozen, so the door refuses before the token is read (that unreachability is itself A1-D3).
   The cells were driven instead on a manifest-less pack, where the door is reachable.
2. **`DOCTYPE_DOORS ▸ Address` × *untracked in-repo* and × *stdin sentinel* — 10 doors × 2 cells = 20 pairs** —
   n/a by shape: an address head is an identity, never a filesystem path the caller points at, so
   "untracked" and "`-`" are not shapes that token can take. The *path* half of those doors is
   covered by their `from_file` occurrence in §2.4.
3. **`SLUG_DOORS` × *untracked in-repo* and × *stdin sentinel* — 6 doors × 2 cells = 12 pairs** — same reason: `--slug` is a
   verbatim mint override, not a path the caller points at.
4. **`WORK_UNIT_ID_DOORS` × the six filesystem-shaped cells (symlink · `.git/` · workbench ·
   untracked · colon · stdin) — 25 doors × 6 cells = 150 pairs** — the four cells driven (`""`, `../..`, absolute,
   well-formed-unknown) are the ones that discriminate: the guard is `engine::slug::is_slug`, so
   every non-slug token lands in one equivalence class and the three driven members of it all answer
   identically at all 25 doors. Driving six more spellings of "not a slug" would add rows, not
   information. Stated here rather than implied.
5. **`migrate-corpus`, `ingest`, `unmanage`'s relocation siblings, and the `milestone provision`
   worktree paths** — not axis-1 doors: no `PATH_ARG_OCCURRENCES`, `DOCTYPE_DOORS`, `SLUG_DOORS` or
   `WORK_UNIT_ID_DOORS` row keys a caller **token** at them (`unmanage`'s `path` row *is* driven, at
   §2.4 row 2).

Also **not driven**, and named: the `owned-location` gate's *presence* and *trackedness* legs over a
file that exists at the right home (the escape legs were driven); and `config set docs-root -`'s
downstream consequences.

---

## 6 — what this adds over flow 52 arm 1

Flow 52 **arm 1** iterates the same code-side registry (`PATH_ARG_OCCURRENCES`) and proves the
*composite* property once: five escape shapes go to **every occurrence** in one repository, each
refuses with its own code or is driven to show its token becomes no path component, and afterwards
`HEAD` is unmoved, `git ls-files` is byte-identical, and the canary planted outside the repository
reached no stream. This review re-drove that composite (§2.4's closing block — canary 28 bytes
unchanged, HEAD at `ff29002`, nothing written outside the repo) and it **holds**.

What the review adds is everything arm 1's subject does not contain:

1. **Three door registries arm 1 does not iterate.** Arm 1's subject is the path-argument registry.
   Axis 1's door set is that registry **∪** `DOCTYPE_DOORS ▸ Address` (10) **∪** `SLUG_DOORS` (6)
   **∪** `WORK_UNIT_ID_DOORS` (25). Four of the five defects live in the three registries arm 1 does
   not reach — including the HIGH, which is a `DOCTYPE_DOORS ▸ Address` row whose consequence lands
   at a `COMMITTING_DOORS` member.
2. **The cells arm 1 does not carry.** Arm 1 drives one traversal, one absolute, one symlink, one
   `.git/` component and one untracked source. Axis 1 adds **workbench root**, **leading-colon
   pathspec magic**, the **`-` stdin sentinel**, the **OS name ceiling** and the **well-formed
   control**. Three of the five defects were found in cells arm 1 does not have: the ceiling
   (A1-D4, A1-D5) and the workbench root (A1-D2).
3. **The cell arm 1 cannot have at all.** A1-D1 was found in the *well-formed control* cell — a
   grammatically perfect slug that names no doc. An escape-shape arm has no reason to drive that
   token, and it is the one that loses authored prose at exit 0.
4. **Reachability of the registry's own rows.** Arm 1 drives each row's declared argv and checks the
   outcome; it has no way to notice that one row's argv (`relocate adr --from <path>`) never reaches
   the arm it dispositions, because the door's refusal is also an exit 1. A1-D3 is only visible when
   a driver reads the refusal *text* against what the row claims the argv proves.
5. **The consequence half.** Arm 1 asserts `HEAD` unmoved. Axis 1 additionally drove what happens
   when the write is **accepted** — the four-copies-one-path finalize, the unvalidatable store after
   a too-long `docs-root`, the staged `R` rename of `.jigc/AGENT.md`. Two of the five defects are
   invisible to any assertion that only checks that nothing bad happened on the *refusal* path.

---

## 7 — doors covered (door of ≥1 driven row), `VERB_KINDS` spelling

**39 leaves.** The 35 axis-1 door-set members first:

`start` · `workflow` · `migrate` · `unmanage` · `relocate` · `rename` · `validate` ·
`doc create` · `doc author` · `doc add-item` · `doc remove-item` · `doc retitle-item` ·
`doc rename` · `doc set-field` · `doc set-slot` · `doc show` · `doc list` · `doc schema` ·
`task diff` · `task validate` · `task discard` · `task finalize` · `task bind` · `config set` ·
`config get` · `config insert-step` · `config replace-step` · `config remove-step` · `config fill` ·
`config fork` · `milestone create` · `milestone add-task` · `milestone add-from-spec` ·
`milestone list-tasks` · `milestone provision` · `milestone execute` · `milestone join` ·
`milestone finalize` · `milestone discard`

The list above is the 35 door-set members **plus** the 4 beyond it — `validate`, `doc schema`,
`config get`, `milestone create` — each of which ran on the installed binary with a recorded
outcome (the store sweep after a too-long `docs-root`; the settable-field and `owned-location`
schema projections; the cascade read-back for the invocation-log knob; the milestone the
`add-from-spec` rows needed), so each is the door of ≥1 driven row under §17's definition.

---

# PART II — Reconciliation ledger

The rule applied (acceptance-design.md → *The reconciliation rule*): **a claim by one that the other
cannot reproduce is a LEAD, not a finding.** Every Codex claim below was entered as
`lead(codex, …)` and then **driven** on the installed rc.15 — to a repro block (CONFIRMED, origin
codex) or to a refutation carrying the falsifying datum. Every driver defect was **re-driven once**
by the reconciler before its status was written.

The Codex source pass returned **zero defect claims** and nine "consistent findings" — positive
claims about the registries and seams. Those nine are the leads; the pass's headline is the tenth
and is the one that collides with the driver.

## 8 — the demotion pass (rows marked driven that carry no repro block)

**No row is demoted.** Every group in Part I carries a repro block: §2.1 carries `setup:`/`argv:`/
`observed:`; §2.2, §2.3 and §2.4 carry `setup:` + `argv:`/`tokens:` with the per-cell observations in
the table rather than in the block; §2.4 closes with an observed composite block; each defect in §3
carries its own verbatim block.

**Recorded weakness, not a demotion:** §2.2, §2.3 and §2.4's grouped blocks carry **no `observed:`
line** — the argv is reproducible but the output lives only in the table. Rather than take those on
faith, the reconciler **re-drove a sample of each group**, and every sampled cell reproduced the
driver's recorded verdict:

| group | what the reconciler re-drove | result |
|---|---|---|
| §2.1 | 3 doors × 4 cells (`task discard` · `task validate` · `milestone discard`) | 12/12 match — `work-unit.malformed-id` at `""`/`../..`/`/etc/passwd`; `finalize.no-task` / `milestone.unknown` at the well-formed unknown |
| §2.2 | **10 doors × 5 escape cells = 50**, plus **10 doors × the ceiling cell** | 60/60 match — `store.malformed-slug` 50/50; ceiling: exactly the 5 code-less doors the table names |
| §2.3 | 6 doors × ceiling(166) + 6 doors × `../../x` | 12/12 match — `write.slug-name-ceiling` 6/6; the grammar code at `rename` only, 1/6 |
| §2.4 | rows 1, 2, 4, 6, 9, 10, 12–15, 16 (11 of 18 arms) | all match, incl. row 2's 10/10 `no-op` and row 12–15's "no file created" (`ls .jigc/config/` after the sweep holds `packs.yaml` only) |

One numeric cross-check of §0, since the door count feeds §7: `WORK_UNIT_ID_DOORS` **25** `door:`
rows (its doc-comment says *"Twenty-five doors."*), `SLUG_DOORS` **6**, `DOCTYPE_DOORS ▸ Address`
**10**, `PATH_ARG_OCCURRENCES` **14** rows. The driver's §0 counts stand, and its recorded
doc/code divergence (the acceptance design says "26 rows / 25 doors") is against the **design text**,
not the binary.

## 9 — Codex claims, each driven

### C1 — headline: *"No grounded completeness defect found. I found neither an omitted door nor a caller-token path that bypasses the relevant adjudication seam."* → **REFUTED**

Three falsifying data, each re-driven this session on a fresh rig.

**(a) A caller token becomes two path components and reaches a `COMMITTING_DOORS` member, which
drops one of two authored payloads at exit 0** (= driver A1-D1, re-driven verbatim):

```
setup: rig=$(dev/jigc-rig committed-singletons --binary /Users/maurice/.local/bin/jigc) || exit
       eval "$rig"        # root …/jigc-rig-committed-singletons-fGmdse
       jigc start --workflow single-task "repro"

$ jigc doc set-slot "adr:ghost#context" --from-file - </dev/null
  exit 1 — no staged instance for `adr:ghost#context` — provision it first …   ← non-singleton REFUSED
$ printf 'PAYLOAD-ALPHA\n' | jigc doc set-slot "vision:alpha#thesis" --from-file -
  exit 0  set slot vision:alpha#thesis (14 chars) (copied in for update — …)
$ printf 'PAYLOAD-BETA\n'  | jigc doc set-slot "vision:beta#thesis"  --from-file -
  exit 0  set slot vision:beta#thesis (13 chars) (copied in for update — …)
$ jigc doc show vision:alpha --task repro
  exit 1  blocking · store.not-found — `vision:alpha` names no doc staged in task `repro`:
          `vision` is a singleton, so its only address is `vision:vision`
$ ls .jigc/tasks/repro/docs/
  commit:repro.md  provenance.json  vision:alpha.md  vision:beta.md
$ jigc doc set-field commit:repro#header/type --value docs
$ printf 'repro\n' | jigc doc set-slot commit:repro#summary --from-file -
$ jigc task finalize repro
  exit 0   advisory · file-state.staged-copy — staged copy of `VISION.md` …   (printed twice)
           finalized 54ec5a5 — docs: repro / promoted VISION.md / 1 file committed
$ sed -n '/## Thesis/,/## /p' VISION.md
  ## Thesis
  PAYLOAD-BETA                      ← PAYLOAD-ALPHA gone; no finding, no warning, exit 0
$ git show --stat --oneline HEAD
  54ec5a5 docs: repro / VISION.md | 2 +- / 1 file changed
```

The read door states the rule in words (*"names no doc … its only address is `vision:vision`"*); five
write doors take the same token and make a filename of it. That is a caller-token path with no
adjudicating seam, by the pass's own definition.

**(b) A caller token the registry's own no-rule says is *"never … handed to git as a pathspec"* is
handed to `git mv`** (= driver A1-D2, re-driven verbatim):

```
setup: rig=$(dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc --pack-from-dev) || exit
       eval "$rig"        # root …/jigc-rig-fresh-vSaxO8 — --pack-from-dev drops the freeze manifest
$ git ls-files
  .claude/settings.json  .claude/skills/jigc/SKILL.md  .jigc/.gitignore  .jigc/AGENT.md
  .jigc/config/.gitkeep  .jigc/config/packs.yaml  .jigc/version  CLAUDE.md  README.md
$ jigc relocate adr --from .jigc
  freeze-exempt relocation: 1 moved, 0 displaced, 0 blocked
    moved     .jigc/AGENT.md -> docs/decisions/AGENT.md
  exit=0
$ git status --short
  R  .jigc/AGENT.md -> docs/decisions/AGENT.md
$ git reset -q --hard HEAD && jigc relocate adr --from .claude
  freeze-exempt relocation: 1 moved, 0 displaced, 0 blocked
    moved     .claude/skills/jigc/SKILL.md -> docs/decisions/SKILL.md
  exit=0
```

The same `.jigc` token is refused at both sibling doors — `config set docs-root .jigc` →
`config.workbench-root`, `jigc migrate .jigc/version --as changelog` → `migrate.source-untrackable`
*"is inside jigc's own workbench"* — both re-driven below at C6/C7. The third door takes it.

**(c) A registry row's fence is satisfied by an argv that cannot reach the arm it dispositions**
(= driver A1-D3, re-driven): `PATH_ARG_OCCURRENCES`' `relocate`/`from` row declares
`["relocate", "adr", "--from", PATH_ARG_SLOT]` (`crates/cli/src/cli.rs:2981`), and `adr` faults
first at **14/14** shipped doctypes. The source pass's own C3 fence checks that an arm's argv
*parses* and *delivers the token through the declared arg* — not that the door **reads** it, which
is precisely the gap (c) names. Datum below at A1-D3.

### C2 — *`ARG_TOKENS` classifies every path-bearing argument id as `path`/`file`/`from_file`/`from`/`target`/`value`; its documented limitation is only a deliberate `PlainValue::Other` classification* → **CONFIRMED**

Driven as its fence rather than read:

```
$ cargo test -p cli --lib every_clap_argument_is_classified
  test cli::cli_parse::every_clap_argument_is_classified ... ok
  test result: ok. 1 passed; 0 failed …            exit=0
```

### C3 — *the clap-tree fences are bidirectional (every argument in `ARG_TOKENS`; every path-bearing occurrence has a `PATH_ARG_OCCURRENCES` row whose argv parses to that exact leaf and argument)* → **CONFIRMED, with a stated ceiling**

```
$ cargo test -p cli --lib every_path_arg_occurrence_is_registered
  test cli::cli_parse::every_path_arg_occurrence_is_registered ... ok      exit=0
```

**Ceiling, driven:** the fence's assertion 4 is *"each arm's `argv` actually parses, lands on the
leaf the row names, and delivers its token through the row's declared `arg`"*
(`crates/cli/src/cli.rs` → the test's doc-comment). Parsing is not reaching: A1-D3 is a green row
under this fence whose door refuses before the token is read. The claim is true; what it buys is
membership, not seam-exercise.

### C4 — *`DOCTYPE_DOORS` includes all bare-doctype and address-bearing leaves, including the non-obvious `milestone add-from-spec` `spec_addr`* → **CONFIRMED**

```
$ cargo test -p cli --lib every_doctype_door_is_registered      → ok, exit=0
setup: rig=$(dev/jigc-rig committed-singletons …) || exit; eval "$rig"
       jigc milestone create "Axis milestone"
$ jigc milestone add-from-spec axis-milestone 'spec:../../etc/passwd'
  exit 1  blocking · store.malformed-slug — "../../etc/passwd" is not a valid doc slug —
          the `<slug>` head of address `spec:../../etc/passwd`
          route: `jigc doc list` … use lowercase letters, digits, and single hyphens …
  same at  spec:/etc/passwd · spec:.git/config · spec:.jigc/version · spec::(top)vision   (5/5)
$ jigc milestone add-from-spec axis-milestone spec:ghost
  exit 1  blocking · store.not-found — could not read `spec:ghost` at `docs/specs/ghost.md` …
```

### C5 — *work-unit ids and verbatim `--slug` overrides are projected from the same total argument classification rather than independent name lists* → **CONFIRMED**

```
$ cargo test -p cli --lib every_slug_door_is_registered           → ok, exit=0
$ cargo test -p cli --lib every_work_unit_id_door_is_registered   → ok, exit=0
```

and behaviourally, the projection's two door sets answer uniformly (§8's 12/12 and 12/12 samples).

### C6 — *the path registry distinguishes occurrences and conditional arms — the `-` stdin sentinel, root vs non-root config values, address tails against closed vocabularies, deliberate unrestricted handoff-file reads* → **CONFIRMED, all four**

```
setup: rig=$(dev/jigc-rig committed-singletons …) || exit; eval "$rig"; jigc start --workflow single-task "c6"

root vs non-root `config set <value>`:
$ jigc config set docs-root /etc        → 1  config.unusable-root   (…an absolute path…)
$ jigc config set docs-root .jigc       → 1  config.workbench-root  (…inside jigc's own workbench…)
$ jigc config set docs-root ../x        → 1  config.untrackable-root(…resolves outside the repository root…)
$ jigc config set invocation-log ../../x→ 1  config.value-rejected  ("../../x" is not a bool)

address tails against closed vocabularies (no filesystem path is ever formed):
$ jigc config fill 'workflow:single-task#../../etc/passwd' --from-file -  → 1 slot-fill-target.wrong-scheme
$ jigc config fill '../../etc/passwd' --from-file -                       → 1 slot-fill-target.missing-hash
$ jigc config remove-step 'workflow:single-task#../../etc/passwd'         → 1 config.anchor-absent
$ jigc config remove-step '../../etc/passwd'                              → 1 structural-target.missing-colon
$ jigc config fork '../../etc/passwd'                                     → 1 structural-target.missing-colon
$ ls -R .jigc/config/                                                     → packs.yaml   (nothing minted)

the `-` sentinel arms:
$ printf 'hello\n' | jigc config fill 'step:locate#x' --from-file -  → 1 config.fill-point-absent (the token never became a path)
$ jigc doc author adr --from-file - --task <t> </dev/null            → 1 write.wrong-shape (payload grammar, not a path guard)

the deliberate unrestricted handoff read:
$ jigc doc set-slot vision:vision#thesis --from-file .git/config
  exit 0  set slot vision:vision#thesis (192 chars) (copied in for update — …)
$ sed -n '/## Thesis/,/## /p' .jigc/tasks/*/docs/vision:vision.md
  ## Thesis
  [core]
  	repositoryformatversion = 0 …
```

The last one is the registry's **declared** answer (`-` hands the same bytes), and it is the divergence
the driver records as O4; the reconciler drove the other half of that divergence too:
`jigc config insert-step --workflow single-task --after locate .git/config` →
`config.step-source-untrackable`, while `… --after locate .jigc/version` → **exit 0**, step copied in.
Both as declared.

### C7 — *migration-source admission uses `resolve_source_token`, covering pathspec magic, repository confinement, `.git`, `.jigc`, and symlinked components before recording the source* → **CONFIRMED, 8/8**

```
setup: rig=$(dev/jigc-rig committed-singletons …) || exit; eval "$rig"    # root …-omQ9RN
       OUT="$RIG/outside"; mkdir -p "$OUT"; printf '# Planted\n\nCANARY-BYTES-9f3\n' > "$OUT/planted.md"
       ln -sfn "$OUT" escape-link; printf '# Changelog\n\n## 1.0.0\n\n- a thing\n' > untracked-source.md

$ jigc migrate "$OUT/planted.md"        --as changelog → 1 migrate.source-untrackable "resolves outside the repository"
$ jigc migrate ../outside/planted.md    --as changelog → 1 migrate.source-untrackable "resolves outside the repository"
$ jigc migrate escape-link/planted.md   --as changelog → 1 migrate.source-untrackable "resolves outside the repository"
$ jigc migrate .git/config              --as changelog → 1 migrate.source-untrackable "inside git's own directory"
$ jigc migrate .jigc/version            --as changelog → 1 migrate.source-untrackable "inside jigc's own workbench (`.jigc/`)"
$ jigc migrate ':(top)untracked-source.md' --as changelog → 1 migrate.source-untrackable "begins with `:` … pathspec magic"
$ jigc migrate untracked-source.md      --as changelog → 1 migrate.source-untracked  route `git add -- untracked-source.md`
$ jigc migrate -                        --as changelog → 1 CODE-LESS "could not read the foreign `changelog` source at `-`" (+ route)
$ cat "$OUT/planted.md"                               → # Planted / CANARY-BYTES-9f3   (untouched)
```

### C8 — *the persisted `source-path` has one raw reader returning the provenance-only `MigrationSource` type; consumers receive only `recorded()` or lexical `normalized()` views* → **CONFIRMED (structure + a driven consequence)**

`crates/engine/src/state.rs:700-760`: `read_migration_source` is the only constructor, the field is
private, and the two accessors are `recorded()` and `normalized()` (`store::lexical_normalize`).
Driven consequence — the **normalized** view is the one that reaches the review-hold display:

```
setup: a migrate task minted from a tracked source, then `.jigc/tasks/<id>/source-path`
       hand-edited to `../outside/victim.md`   (see C9 for the full block)
$ jigc task finalize <id>
  exit 4  migration review required — nothing committed. Re-run … --approve to write the canonical
          doc, DELETE the foreign original `outside/victim.md`, and commit.
                                                  ↑ `../` consumed by the lexical normalize
```

### C9 — *retirement planning may compare or carry the normalized source, but the destructive sink re-adjudicates it into `ValidatedRetirement` immediately before `remove_file`* → **CONFIRMED on its safety claim; one driven rider recorded**

Driven directly at the sink by tampering with the persisted value — the hunt the Codex prompt named
(*"a raw read of source-path that bypasses the typed sink"*):

```
setup: rig=$(dev/jigc-rig committed-singletons …) || exit; eval "$rig"    # root …-rVqblE
       printf '# Old Decision\n\nWe chose X because Y.\n' > legacy-decision.md
       git add legacy-decision.md && git commit -qm "legacy decision"
       jigc migrate legacy-decision.md --as adr      # task migrate-adr-legacy-decision-402515bb538c
       printf 'VICTIM-BYTES-7a1\n' > "$RIG/outside/victim.md"
       printf '../outside/victim.md' > .jigc/tasks/<t>/source-path        ← the tamper
       jigc doc create adr --title "Old Decision" --task <t>; three slots filled

$ jigc task finalize <t> --approve
  exit 0   finalized da863c6 — docs(adr): adopt legacy-decision.md as a managed adr
           promoted docs/decisions/old-decision.md / 1 file committed
$ cat "$RIG/outside/victim.md"      → VICTIM-BYTES-7a1      ← NOT deleted. The sink held.
$ git show --stat --oneline HEAD    → docs/decisions/old-decision.md | 22 ++  (no deletion)
```

and the in-repo arm, to show the sink's guarantee is *recoverability*, not *identity*:

```
setup: same, but `printf 'README.md' > .jigc/tasks/<t>/source-path`      # root …-BkYpPA
$ jigc task finalize <t> --approve
  exit 0   finalized 030ee04 … deleted README.md / promoted docs/decisions/old-decision.md
$ git show --stat --oneline HEAD → README.md | 1 - ; docs/decisions/old-decision.md | 22 ++
```

**O5 — driven rider, premise-bound, NOT promoted to a defect (origin codex).** On the escaping arm
the approve is **silent**: the review hold promised *"DELETE the foreign original
`outside/victim.md`"*, `--approve` deleted nothing, retired nothing, printed no finding and exited 0
— because `normalized()` lexically resolves `../outside/victim.md` to `outside/victim.md` *before*
the sink, so `ValidatedRetirement::adjudicate` admits an in-repo path that does not exist and
`retire`'s idempotent *already-absent* arm swallows it. `retire_untrackable_finding`'s own text
exists for exactly this state (*"restore it to the path `jigc migrate` recorded"*, `task.rs:3655`)
and cannot be reached through the `..` shape. **Not promoted, and why:** the premise is a
hand-edited gitignored workbench file, and **no caller token can produce it** — `jigc migrate`
refuses every `../` spelling at the door (C7, driven). It is recorded here as a lead for whichever
axis owns workbench-state truth, not as an axis-1 caller-token defect.

### C10 — *printed route operands pass through `shell_token`/`shell_operand`; all route constructors invoke the backticked-command-span fence in debug builds* → **CONFIRMED on the operand half · OPEN LEAD on the fence half**

The operand half, driven with the **shell-unsafe name** cell (see §10 — a cell the Codex prompt
carries and the driver's cell set omits):

```
setup: rig=$(dev/jigc-rig committed-singletons …) || exit; eval "$rig"    # root …-mTOWR6
       printf '# Changelog\n\n## 1.0.0\n\n- a thing\n' > "weird name'; touch PWNED.md.md"
$ jigc migrate "weird name'; touch PWNED.md.md" --as changelog
  exit 1  blocking · migrate.source-untracked — `weird name'; touch PWNED.md.md` …
          route: stage it with `git add -- 'weird name'\''; touch PWNED.md.md'`, then re-run
                 `jigc migrate 'weird name'\''; touch PWNED.md.md' --as changelog`
$ ls PWNED.md                     → No such file or directory
$ git add "weird name'; touch PWNED.md.md" && git commit -qm weird
$ jigc migrate "weird name'; touch PWNED.md.md" --as changelog
  exit 0  task minted: migrate-changelog-weird-name-touch-pwned-md-3bde52ea8b0e
$ ls PWNED.md                     → No such file or directory
```

POSIX single-quote escaping is exact and copy-runnable; the token reaches no shell.

**OPEN LEAD (fence half), reason stated:** *"all route constructors invoke the backticked-command-span
fence **in debug builds**"* is not drivable here. The reconciliation binary is the release
`1.0.0-rc.15`, in which those `debug_assert!`s do not exist; and even against a debug build the fence
fires only on a *violating* route, which no CLI argv can construct from outside the process — it
would need a code change, which this review does not make. Left OPEN rather than promoted on the
source read or dropped.

## 10 — one cell-set divergence between the two passes, closed

The Codex prompt's cell set carries **shell-unsafe name**; the driver's §0 cell set (ten cells) does
not. Driven by the reconciler at the one family where the cell can exist — the `Plain(PathBearing)`
path family, since at `--slug` and at an address `<slug>` head the slug grammar refuses every
shell-unsafe spelling first (driven: `store.malformed-slug` / `write.malformed-slug` at §8's 50/50
and 6/6 samples). Result: **clean** (the C10 block above). No defect; the divergence is recorded so
the gap is visible rather than silently absent from the matrix.

## 11 — the driver's five defects, each re-driven

| id | severity | status after re-drive | the source pass on it |
|---|---|---|---|
| **A1-D1** | HIGH | **CONFIRMED** — re-driven verbatim on a fresh rig (block at C1(a)); fresh sha `54ec5a5`, `PAYLOAD-ALPHA` lost at exit 0 | silent on it; its headline denies the class → the headline is **REFUTED** by this repro |
| **A1-D2** | MEDIUM-HIGH | **CONFIRMED** — re-driven verbatim (block at C1(b)); both `.jigc/AGENT.md` and `.claude/skills/jigc/SKILL.md` staged `R` at exit 0 | silent; C1 refuted by this repro. Note the pass **read** `PATH_ARG_OCCURRENCES` and called the registry consistent — the row it read is the one whose no-rule this drive falsifies |
| **A1-D3** | MEDIUM | **CONFIRMED** — re-driven 14/14 (block below) | the pass's C3 asserts the fence is bidirectional and it **is**; the fence checks parse-and-deliver, never reach, so the two are compatible and the defect stands |
| **A1-D4** | MEDIUM | **CONFIRMED** — re-driven across all 10 address doors; exactly 5 code-less (block below) | silent |
| **A1-D5** | MEDIUM | **CONFIRMED** — re-driven incl. the driver's own honest correction (`validate` exit **1**, no false green) | silent |

**A1-D3 re-drive** (root …-yM0PKt):

```
$ for ty in vision roadmap decisions-log changelog adr spec prd arch-doc idea research \
            milestone-record completion-record deferral-ledger planning-record; do
    jigc relocate "$ty" --from docs/old; done
  14/14 exit 1, CODE-LESS, route-less:
  `<ty>` is a frozen doctype — relocate it through the version-gated `jigc migrate-corpus`,
  not the freeze-exempt path
$ jigc relocate adr --from docs/old --format json > f.json 2>&1; echo $?   # measured BARE
  1
  {"error": "`adr` is a frozen doctype — relocate it through the version-gated `jigc migrate-corpus`, …"}
$ sed -n '2975,2990p' crates/cli/src/cli.rs
  door: &["relocate"], arg: "from",
  argv: &["relocate", "adr", "--from", PATH_ARG_SLOT],
```

**A1-D4 re-drive** — the ceiling cell (300 × `a`) at **all ten** `DOCTYPE_DOORS ▸ Address` doors,
one task, one rig:

```
rename            exit=1 code=rename.in-flight
doc add-item      exit=1 code=NONE  | could not copy in `roadmap:<A300>#milestones` for editing: File name too long (os error 63)
doc remove-item   exit=1 code=NONE  | could not copy in `roadmap:<A300>#milestones/m-alpha` for editing: File name too long (os error 63)
doc retitle-item  exit=1 code=NONE  | could not copy in `roadmap:<A300>#milestones/m-alpha` for editing: …
doc rename        exit=1 code=write.identity-change
doc set-field     exit=1 code=NONE  | could not copy in `roadmap:<A300>#milestones/m-alpha/title` for editing: …
doc set-slot      exit=1 code=NONE  | could not copy in `vision:<A300>#thesis` for editing: File name too long (os error 63)
doc show          exit=1 code=store.not-found
task bind         exit=1 code=NONE  | no such doc `spec:<A300>`
milestone add-fs  exit=1 code=store.not-found
```

and the asymmetry driven in one place — the **same byte length** at the two token families:

```
$ jigc doc create adr --title X --slug <166×'a'>
  1  blocking · write.slug-name-ceiling — `--slug "<A166>"` is 166 bytes — over the 165-byte ceiling …
     route: re-run with a `--slug` of at most 165 bytes — jigc refuses rather than truncating …
$ jigc doc set-slot "vision:<166×'a'>#thesis" --from-file - </dev/null
  0                                      ← the address head accepts it and mints the file
  boundary walk at the head: 100→0 160→0 166→0 200→0 240→1 245→1 250→1
```

**A1-D5 re-drive** (root …-nXh5s4), every exit measured bare, never through a pipe:

```
$ jigc config set docs-root <300×'a'>
  0   config: set `docs-root` = `aaaa…` — written to `.jigc/config/`, uncommitted …
$ jigc validate                       → 1  validating the committed store at "…": File name too long (os error 63)
$ jigc validate --format json         → 1  {"error": "validating the committed store at \"…\": File name too long (os error 63)"}
$ jigc config set docs-root -         → 0   (the sibling cell the driver records and does not grade)
```

## 12 — ledger summary

**CONFIRMED (9)** — C2, C3, C4, C5, C6, C7, C8, C9 (safety claim), C10 (operand half): every Codex
"consistent finding" that could be driven, was, and held.

**REFUTED (1)** — C1, the pass's headline, by three independent repros (A1-D1 · A1-D2 · A1-D3).

**OPEN LEAD (1)** — C10's fence half (debug-only `debug_assert!`; release binary; a violating route
is not constructible from any argv).

**Driver defects (5)** — all five re-driven, all five CONFIRMED, none demoted.

**New, not promoted (1)** — O5, the tampered `source-path` silent no-retire, premise-bound and
unreachable by any caller token.

**The shape of the disagreement, stated plainly:** the source pass asked whether the **registries are
complete** and answered correctly — every ⇔ fence it cited is green, driven. The driver asked
whether the **seams the registries point at actually adjudicate**, and three of them do not: a row
can be a member and still be wrong (A1-D2's no-rule), unreachable (A1-D3's argv), or absent where
the token family crosses a registry boundary (A1-D1 and A1-D4, where the `<slug>` head of an address
is the same filename component as `--slug` and carries neither of its two guards). Membership is not
adjudication; that is the axis's finding.

---

# PART III — doors covered

Every clap leaf that is the door of ≥1 driven row, `VERB_KINDS` spelling. **39 leaves** — the driver's
35 axis-1 door-set members plus the 4 it drove as setup/consequence; the reconciliation added no new
leaf (`describe` was invoked once but rejected at clap exit 2, which is not a driven row, so it is
deliberately not listed).

`start` · `workflow` · `migrate` · `unmanage` · `relocate` · `rename` · `validate` ·
`doc create` · `doc author` · `doc add-item` · `doc remove-item` · `doc retitle-item` ·
`doc rename` · `doc set-field` · `doc set-slot` · `doc show` · `doc list` · `doc schema` ·
`task diff` · `task validate` · `task discard` · `task finalize` · `task bind` · `config set` ·
`config get` · `config insert-step` · `config replace-step` · `config remove-step` · `config fill` ·
`config fork` · `milestone create` · `milestone add-task` · `milestone add-from-spec` ·
`milestone list-tasks` · `milestone provision` · `milestone execute` · `milestone join` ·
`milestone finalize` · `milestone discard`

Of these, the reconciler independently drove **27**: `start` · `migrate` · `unmanage` · `relocate` ·
`rename` · `validate` · `doc create` · `doc author` · `doc add-item` · `doc remove-item` ·
`doc retitle-item` · `doc rename` · `doc set-field` · `doc set-slot` · `doc show` · `task validate` ·
`task discard` · `task finalize` · `task bind` · `config set` · `config insert-step` ·
`config remove-step` · `config fill` · `config fork` · `milestone create` · `milestone add-from-spec` ·
`milestone discard`.
