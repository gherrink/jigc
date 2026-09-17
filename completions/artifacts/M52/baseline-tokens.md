<!-- M52 baseline — area `tokens` (axis 1 rows A1-D1…A1-D5) · driven 2026-09-16/17 -->

# M52 baseline — AREA `tokens`

## Provenance

* **Binary:** `/Users/maurice/.local/bin/jigc` → `jigc 1.0.0-rc.15` (asserted at the top of the first
  rig). RELEASE posture. No cargo was run; no file in the working repository was written.
* **Repo HEAD at read time:** `85ad06c571945157d5b6df166659b74ce5f0cc86`, tree clean.
* **Rigs (all two-step `rig=$(dev/jigc-rig <state> --binary ~/.local/bin/jigc) || exit; eval "$rig"`,
  every root from `mktemp -d`, no teardown, no `rm -rf` anywhere):**
  `committed-singletons` ×8 (rig1…rig9, rigC/D/E/H) · `fresh --pack-from-dev` ×2 (rigA, rigB) ·
  `fresh --pack-from-dev --schema adr <manufactured singleton>` ×1 (rigG).
* **Scratch:** `/private/tmp/…/scratchpad/` — all captured output, no project writes.
* **Assigned rows:** A1-D1 (HIGH) · A1-D2 · A1-D3 · A1-D4 · A1-D5. Per the brief these were **not
  re-verified**; every drive below is a *class* drive around them.

---

## §1 — the class enumeration

### 1.1 The registries (grep + counts read at HEAD)

```
grep -n "pub const \(ARG_TOKENS\|DOCTYPE_DOORS\|SLUG_DOORS\|PATH_ARG_OCCURRENCES\|WORK_UNIT_ID_DOORS\)" crates/cli/src/cli.rs
grep -rn "ROOT_KNOBS" crates/cli/src/*.rs crates/engine/src/*.rs | grep "pub const"
```

| registry | file | count read | what the pattern would miss |
|---|---|---|---|
| `ARG_TOKENS` | `cli.rs:2234` | **35** ids; 6 `Plain(PathBearing)` (`file` `from` `from_file` `path` `target` `value`), 5 `Doctype` ids of which **5 are `Address`** (`addr`, `old_slug`, `spec_addr` + the two shared), 1 `SlugOverride`, 3 `WorkUnitId` | nothing on the clap tree (a `⇔` fence proves totality) — but it classifies **argument ids**, not *what the value becomes*, so the `<slug>` **head of an address** is classified `Doctype`, never `PathBearing`, even though it becomes a path component. That is the gap A1-D1/A1-D4 both sit in |
| `DOCTYPE_DOORS` | `cli.rs:2357` | **16** rows — **10 `Address`**, 6 `Bare` | a door taking an address *inside a payload* rather than as an argument (`doc author`'s `--from-file` payload) carries no row; driven, `doc author`'s payload has no address form, so this is empty in fact |
| `SLUG_DOORS` | `cli.rs:2664` | **6** | — |
| `PATH_ARG_OCCURRENCES` | `cli.rs:2935` | **18** arms / **15** occurrence rows / 12 leaf verbs | — |
| `WORK_UNIT_ID_DOORS` | `cli.rs` | **25** | — |
| `ROOT_KNOBS` | `config.rs:55` | **2** (`docs-root`, `placement-root`) | — |

### 1.2 The three classes, as measured

**Class 1 — the `<slug>` head that names no doc.** The necessary condition is **not** `singleton:
true`; it is `placement: {file: …}`, because only on the placement branch does `canonical_path`
*ignore* the slug and resolve to the one literal file. Driven proof of the boundary is in §2.4: a
manufactured `singleton: true` doctype homed by `location:` refuses the bogus head at every write
door, because `docs/decisions/bogus.md` does not exist.

*Subject set:* **every `placement:` doctype with a committed instance**. Enumerated by
`grep -rn "placement:" crates/cli/pack/schemas packs/methodology/schemas` → **5**:
`changelog` (dev pack, `CHANGELOG.md`), `vision` (`VISION.md`), `roadmap` (`docs/roadmap.md`),
`decisions-log` (`docs/decisions-log.md`), `deferral-ledger` (`docs/deferral-ledger.md`). All five
also carry `singleton: true`; the two sets coincide **in the shipped packs only** — a PB-1 project
pack can ship either half alone (§2.4).

*Door set driven:* the 10 `DOCTYPE_DOORS ▸ Address` doors **plus** `rename --slug` (a `SLUG_DOORS`
row the review's D1 table lists as refusing) — **11 doors**, all driven in §2.1.

**Class 2 — the OS name ceiling over every caller token that becomes a path component.** Enumerated
as the union: address `<slug>` heads (10 doors) ∪ `--slug` (6) ∪ work-unit ids (25, sampled 5) ∪
`ROOT_KNOBS` values (2) ∪ the `PATH_ARG_OCCURRENCES` path arms (18). Full verdict table in §2.5.

**Class 3 — `relocate --from` × the root family, and the "declared argv cannot reach its arm"
class.** §2.6 and §2.7.

---

## §2 — the drives

### 2.1 Class 1 — the door × head-shape matrix

Rig: `committed-singletons`; `jigc start --workflow single-task "class one"` → task `class-one`.
Head shapes: **(a)** bogus well-formed slug on a *placement* doctype · **(b)** bogus on a
non-singleton · **(c)** bare singleton · **(d)** canonical.

| # | door | (a) `vision:alpha` / `roadmap:bogus` | (b) `adr:ghost` | (c) `vision` | (d) `vision:vision` |
|---|---|---|---|---|---|
| 1 | `doc set-slot` | **exit 0**, mints `.jigc/tasks/<t>/docs/vision:alpha.md` | exit 1 code-less *no staged instance* | exit 0 (expanded to `vision:vision`) | exit 0 |
| 2 | `doc add-item` | **exit 0**, mints `roadmap:bogus.md` | (not driven, same seam) | — | exit 0 |
| 3 | `doc retitle-item` | **exit 0** | — | — | exit 0 |
| 4 | `doc remove-item` | **exit 0** | — | — | exit 0 |
| 5 | `doc set-field` | **exit 0** (`changelog:bogus#releases/1-0-0/date`, `decisions-log:bogus`) | — | — | exit 0 |
| 6 | `rename <addr> --to T --slug <bogus>` | **exit 0**, commits `rename VISION.md -> VISION.md`, rewrites the real H1 | exit 1 `store.not-found` | — | exit 0 |
| 7 | `rename <addr> --to T` (no `--slug`) | exit 1 `write.identity-change` | exit 1 `store.not-found` | — | exit 1 `write.identity-change` |
| 8 | `doc rename` | exit 1 `write.identity-change` (names the singleton rule) | — | — | exit 1 same |
| 9 | `doc show` (committed **and** `--task`) | exit 1 `store.not-found`, **names the singleton rule + route** | exit 1 `store.not-found` | exit 0 | exit 0 |
| 10 | `task bind` | **unreachable** — see §2.3 | — | — | — |
| 11 | `milestone add-from-spec` | **exit 1 but the head was ACCEPTED** — `store.no-such-section`, read off the real committed file, route names `jigc doc show vision:alpha` which `doc show` refuses | exit 1 `store.not-found` | — | — |

Verbatim, the two that matter most:

```
$ printf 'X\n' | jigc doc set-slot "vision:alpha#thesis" --from-file - --task class-one
exit=0
set slot vision:alpha#thesis (2 chars) (copied in for update — the committed doc is now this task's
staged copy, re-promoted at finalize)

$ jigc milestone add-from-spec probe-milestone "vision:alpha"
exit=1
blocking · store.no-such-section — `vision:alpha` names no `criteria` section to seed from
  at: vision:alpha
  route: `jigc doc show vision:alpha` — it carries `meta`, `thesis`, `invariants`, `open-questions`;
         seeding reads a `criteria` section's items
```

**Classification.**
* Doors 1–5: **latent defect / the A1-D1 instance** — confirmed as the review states.
* Door 6 (`rename … --slug <bogus>`): **latent defect, NOT in any §A row.** The review's D1 table
  lists `doc rename` as refusing and does not drive `rename` with `--slug`. Driven, `rename` is a
  **sixth** write door — and a **committing** one (`COMMITTING_DOORS` member) — that takes a bogus
  placement-singleton head at exit 0. Full block in §4.1.
* Door 11 (`milestone add-from-spec`): **latent defect, NOT in any §A row.** The review's D1 table
  presents the read doors as the ones that *know* the rule. This read door does not: it resolves the
  bogus head against the real committed file, reads its section list, and emits a route that its
  sibling read door refuses. Full block in §4.2.
* Doors 7–9: **built + proven** (they carry the rule).

**Where the singleton identity is known vs where the token is used raw** (source read *after*
driving, to explain the drives):

| stage | symbol | knows `singleton`/`placement`? |
|---|---|---|
| address parse at a `doc` verb | `cli::doc::parse_verb_addr` → `expand_bare_singleton` | **partly** — it expands a *bare* head (`vision` → `vision:vision`) but does not canonicalize a *wrong* one |
| slug-head guard | `cli::task::reject_malformed_slug_head` | **no** — grammar only (`engine::slug::is_slug`) |
| staged filename | `.jigc/tasks/<id>/docs/<type>:<slug>.md` | **no** — the raw token becomes the path component |
| copy-in / promote target | `canonical_path(schema, slug)` | **placement branch ignores `slug` entirely** — this is the whole mechanism |
| create | `create_gated` | **yes** — driven: `jigc doc create changelog --slug bogus` → `changelog:changelog (already existed — copied in for update)`; the override is discarded |
| read | `engine::store::resolve_read_schema` (`store.rs:256`) | **yes, but keyed on `schema.placement.is_some() && slug != schema.ty`** — *placement*-keyed, not *singleton*-keyed |

### 2.2 Class 1 — the consequence surfaces over a multi-alias state

Rig: `committed-singletons`, task `class-one`, `vision:alpha` + `vision:vision` staged.

```
$ jigc doc list --task class-one
id  path  state
commit:class-one  commit:class-one  managed
roadmap:bogus  docs/roadmap.md  managed
vision:alpha  VISION.md  managed      ← two ids, one path, both `managed`
vision:vision  VISION.md  managed

$ jigc task validate class-one      → exit 3, blocks only on the commit doc; the collision produces
                                       TWO identical `advisory · file-state.staged-copy` rows for
                                       `VISION.md`, route "no action needed"
$ jigc task finalize class-one --dry-run → exit 3, the two advisories are NOT shown at all
$ jigc task finalize class-one      → exit 0
  promoted VISION.md
  1 file committed
$ sed -n '/## Thesis/,/## Invariants/p' VISION.md   → PAYLOAD-CANON-yyy   (alpha's payload is gone)
```

**The ordering is deterministic and I drove it with three aliases** (rig `committed-singletons`,
task `order-probe`, `vision:zzz` written first, then `vision:aaa`, then `vision:vision`):

```
$ ls .jigc/tasks/order-probe/docs/ → vision:aaa.md  vision:vision.md  vision:zzz.md
$ jigc task finalize order-probe   → exit 0 · "promoted VISION.md" · "1 file committed"
$ VISION.md ## Thesis             → ZZZ-payload        ← lexicographic LAST wins; two payloads lost
```

**The pinned write-ack contract carries the bogus identity through.** Driven:

```
$ jigc doc set-slot "vision:bogus-head#thesis" --from-file x --task ack-probe --format json
{"chars":2,"copied_in":true,"findings":[],"op":"set-slot",
 "target":{"doctype":"vision","section":"thesis","slug":"bogus-head"}}
```

`findings: []` and a `target.slug` no read door accepts — so a driver keying on the M41/M42
machine contract (`design/command-output-contract.md` → findings-as-data, the stable `(code,
target)` key) sees a clean success. **Classification: latent defect, wider than the row states** —
A1-D1's "honest qualification" says the advisory *is* printed; it is printed at `task validate` and
at `finalize`'s text surface, but **not** at `finalize --dry-run`, and **never** on the pinned JSON
ack of the write that created the second copy.

**A bogus alias is a fully functional alias, not only a loss vector.** Driven on a second rig with a
*single* bogus copy (task `bogus-only`):

```
$ jigc doc add-item "roadmap:bogus#milestones" --title "Phantom Milestone" --task bogus-only  → exit 0
$ jigc task finalize bogus-only   → exit 3, blocking · schema-conformance.required-slot-present
    at: roadmap:bogus#milestones/phantom-milestone/proves
    route: `jigc doc set-slot roadmap:bogus#milestones/phantom-milestone/proves …`   ← an address
                                                                                      `doc show` refuses
$ (fill both slots) ; jigc task finalize bogus-only → exit 0 · "promoted docs/roadmap.md"
$ git show --stat HEAD → docs/roadmap.md | 10 ++++++++++
```

So the class has **two consequences, not one**: (i) ≥2 aliases ⇒ silent payload loss at exit 0;
(ii) exactly 1 alias ⇒ a successful write to the real file under an identity that every *read* and
every *route* the tool prints cannot resolve.

### 2.3 Class 1 — the members that are unreachable on a stock corpus

* **`task bind`** — `grep -rn "reads:" -A6 packs/methodology/workflows/*.yaml crates/cli/pack/workflows/*.yaml | grep "type:"`
  → **exactly one** `reads` role in both shipped packs: `implement-from-spec.yaml:6: reads: [{role: spec, type: spec}]`.
  `spec` is non-singleton, so the singleton cell of `task bind` cannot be reached on any shipped
  workflow. **Classification: not driven — unreachable, stated.**
* **`deferral-ledger`** — the fifth placement doctype, uncommitted in every rig state. Driven:
  `jigc doc add-item "deferral-ledger:bogus#entries" …` and the canonical spelling **both** fail
  identically (`no staged instance … its allows-create gate lists [adr, changelog]`). So the
  defect's precondition is **a committed instance**; the doctype itself is not the discriminator.

### 2.4 Class 1 — the adjacent shape: a `singleton:` doctype homed by `location:`

Built with `dev/jigc-rig fresh --pack-from-dev --schema adr <adr.yaml + "singleton: true" +
"display-title: Ledger">` (a manifest-less pack, the only way to reshape a frozen doctype).

```
$ jigc doc create adr --title "Anything"
  exit=1  blocking · write.title-ignored — `adr` is a singleton — its `# H1` is supplied by the
          schema (`Ledger`), never by the author
$ jigc doc create adr                      → exit 2, clap: "the following required arguments were
                                              not provided: --title"
$ jigc doc author adr --from-file pay.yaml (title-less) → exit 0 · adr:adr · adr:adr.md
$ (finalize) → promoted docs/decisions/adr.md
$ jigc doc show adr:bogus
  exit=1  blocking · store.not-found — could not read `adr:bogus` at `docs/decisions/bogus.md`:
          No such file or directory (os error 2)          ← NO singleton rule named
$ jigc doc set-slot "adr:bogus#context" --from-file p   → exit 1, code-less "no staged instance"
$ jigc doc set-slot "adr:adr#context"   --from-file p   → exit 0
```

**Two findings from this shape, both classification `shape-limited`:**
1. **The A1-D1 class boundary is `placement`, not `singleton`.** A location-homed singleton refuses
   the bogus head by accident of path resolution, not by any rule. That bounds the M52 subject
   precisely — and it means a fix keyed on `schema.singleton` would over-reach, while one keyed on
   `schema.placement` would leave the *read*-side guard's own asymmetry in place (see 2 below).
2. **The read-side guard `engine::store::resolve_read_schema` is placement-keyed too**, so for a
   location-homed singleton the tool enforces the "one instance at a fixed slug" rule **nowhere**:
   `adr:bogus` and `adr:adr` are simply two different files. `design/storage.md` and
   `crates/engine/src/schema.rs:106` describe `singleton` as "a running doc with a **fixed slug** =
   the type id"; driven, nothing enforces that for a location-homed one. PB-1 ships project packs as
   a supported capability (M49), so this shape is adopter-reachable.
3. **A location-homed singleton is unconstructible through `jigc doc create`** (title required by
   clap, any title refused by the singleton rule) and constructible only through `doc author`.
   **Classification: latent defect (door asymmetry), not in any §A row.**

### 2.5 Class 2 — the OS name ceiling: the full token → verdict table

`A300 = python3 -c "print('a'*300)"`. Every exit measured bare.

| # | token (what it becomes) | door(s) | verdict at 300 chars | class |
|---|---|---|---|---|
| 1 | address `<slug>` head → `.jigc/tasks/<t>/docs/<type>:<slug>.md` | `doc set-slot`, `add-item`, `remove-item`, `retitle-item`, `set-field` | **exit 1, code-less, route-less**: `could not copy in \`vision:<A300>#thesis\` for editing: File name too long (os error 63)`; `--format json` → `{"error": "…"}` | **A1-D4 confirmed — 5 doors** |
| 2 | same, non-singleton head (`adr:<A300>`) | same 5 | exit 1, code-less: `no staged instance for \`adr:<A300>#context\`` | code-less sibling cell of the same class |
| 3 | address head, read doors | `doc show` | exit 1 `store.not-found` **+ singleton route** | built + proven |
| 4 | address head | `milestone add-from-spec` | exit 1 `store.no-such-section` — **the head was accepted and the real file read** | see §4.2 |
| 5 | address head | `rename` (no `--slug`) | exit 1 `write.identity-change` (placement) / `store.not-found` (adr) — **not** `rename.in-flight` | **corrects the review**, see §3.2 |
| 6 | address head | `doc rename` | exit 1 `write.identity-change` | built + proven |
| 7 | `--slug` (6 `SLUG_DOORS`) | `start` `migrate` `rename` `doc create` `doc add-item` `doc rename` | **exit 1 `write.slug-name-ceiling`, 6/6** — EC-28 holds. Caveat: at `rename` the guard is only *reachable* when the target doc exists (§2.7) | built + proven (with a reachability caveat) |
| 8 | work-unit id (`WORK_UNIT_ID_DOORS`, 5 sampled: `task validate`, `start --task`, `task discard`, `milestone add-task`, `milestone list-tasks`) | — | exit 1 `finalize.no-task` / `milestone.unknown`; **no path is ever built from the token**, so no ceiling exists | built + proven |
| 9 | mint-time *derived* slug (`milestone create "<A300>"`, `jigc start --workflow single-task "<A300>"`) | — | **exit 0**, id capped at 50 chars (`aaaa…` ×50) by the word-cap rule — derived slugs are bounded by construction | built + proven |
| 10 | `config set docs-root <A300>` | — | **exit 0**; `jigc validate` then exits **1** with a code-less `File name too long (os error 63)` | **A1-D5 confirmed** |
| 11 | `config set placement-root <A300>` | — | **exit 0**; the relocation floor reports per-doc failure inside the *success* ack; `jigc validate` exits **0**; two managed docs vanish from `doc list` | **A1-D5's un-named second knob — §4.3** |
| 12 | `migrate <A300>.md`, `config insert-step <A300>`, `--from-file <A300>` | — | not re-driven (review §2.4 rows 1/4/7/9: code-less read failure — the token names a file that cannot exist) | not driven, stated |

### 2.5b Class 2 rider — the other `ROOT_KNOBS` value shapes

Driven over both knobs (each `config set` bare, the knob reset between cells):

| cell | `docs-root` | `placement-root` |
|---|---|---|
| 300-char component | **exit 0** | **exit 0** |
| whitespace *inside* (`a b`) | exit 0 | exit 0 |
| edge whitespace (` docs`, `docs `, `trailing. `) | exit 1 `config.unusable-root` | exit 1 same |
| `..` *inside* (`x/../y`) | **exit 0** — normalized, the ack says ``set `docs-root` = `y` `` | exit 0 same |
| `..` escaping (`docs/../../out`) | exit 1 `config.untrackable-root` | exit 1 same |
| **empty string** `""` | **exit 0** — ack says ``set `docs-root` = `.` `` | exit 0 same |
| `-` | exit 0 | exit 0 |
| Unicode NFC `café` / NFD `café` | exit 0 / exit 0 | exit 0 / exit 0 |
| `CON` | exit 0 | exit 0 |
| embedded newline | exit 1 `config.value-rejected` | exit 1 same |
| under a file (`README.md/sub`) | exit 1 `config.unusable-root` | exit 1 same |

Two of these are worth the plan's attention:

* **The empty string is coerced to `.`** at exit 0. `CLAUDE.md` → M50 records the design as
  *"`""` unset and `.` the repo root kept **unmixable at the type level**"*. Driven, the CLI door
  mixes them: `config set docs-root ""` silently means *the repo root*. **Classification: latent
  defect (a stated rule contradicted at a door), not in any §A row.** Not driven further: whether a
  doc then lands at the repo root.
* **The `config.unusable-root` refusal states the harm the ceiling cell produces.** Verbatim at
  `README.md/sub`: *"`README.md` is a file, not a directory — **every move into it fails while the
  knob lands anyway**"*. That sentence is the door's own statement of why it refuses; the ceiling
  value produces exactly that outcome and is not refused. The predicate's three legs
  (`untrackable_reason` · `is_workbench_root` · `unusable_root_reason`) test escape, workbench and
  shape — never **nameability**.

### 2.6 Class 3 — `relocate --from` × the root family

Rig: `fresh --pack-from-dev` (drops the freeze manifest, so `adr` is freeze-exempt — A1-D3's
consequence). `git reset --hard` between cells. Committed set is jigc's own install footprint.

| `--from` token | exit | ack | filesystem effect |
|---|---|---|---|
| `.jigc` | **0** | `1 moved, 0 displaced, 0 blocked` | `R .jigc/AGENT.md -> docs/decisions/AGENT.md` |
| `.claude` | **0** | `1 moved` | `R .claude/skills/jigc/SKILL.md -> docs/decisions/SKILL.md` |
| `.git` | 0 | `0 moved` | none (git does not track its own dir) |
| `docs` · `docs/decisions` (the doctype's own home) · `.` · `..` · absolute outside-repo path · a symlink to outside · `/etc` | 0 | `0 moved` | none — the predicate is `Path::starts_with` over `git ls-files` output, and no committed spelling starts with `.`, `..`, or an absolute path |
| `""` | 1 | clap: ``\`--from\` (the prior home) is required`` | none |

**Classification: A1-D2 confirmed; the sibling-door comparison holds** — `config set docs-root .jigc`
→ `config.workbench-root`; `jigc migrate .jigc/version` → `migrate.source-untrackable` (both per the
review, not re-driven). **The blast radius, measured:** every committed `.md` under the named prefix,
selected by prefix containment with no stamp, identity or conformance test. `.` and `..` reach
nothing, so the door cannot be made to sweep the whole repo in one call.

**The consequence half the review did not drive is worse than the move** — §4.4.

### 2.7 Class 3 rider — "the declared runnable argv cannot reach its own arm"

`PathArgArm::argv`'s doc-comment claims *"every other argument present and well-formed, **so the
token is the only thing the door can fault on**"*. I drove **every** `PATH_ARG_OCCURRENCES` argv
verbatim with a benign control token on a plain `committed-singletons` rig:

| row | fault before the token? | note |
|---|---|---|
| `migrate <tok> --as changelog` | no | `migrate.source-untracked` — about the token |
| `unmanage <tok>` | no | exit 0 no-op |
| **`relocate adr --from <tok>`** | **YES** | ``\`adr\` is a frozen doctype`` — code-less, route-less. **A1-D3, confirmed; class size 1 within this registry** |
| `config insert-step --workflow single-task --after implement <tok>` | no | exit 0 |
| `config replace-step workflow:single-task#locate <tok>` | no | reads the token (`config.step-id-collision` names the *source basename*) — order-dependent on prior rows, not a pre-token fault |
| `config fill step:implement#extra-guidance --from-file <tok>` | no | exit 0 |
| `doc set-slot adr:probe#context --from-file <tok>` | only without setup | needs an open task **and** a staged `adr:probe`; with both, the token is read |
| `doc author adr --from-file <tok>` | only without setup | needs an open task; with one, the token is read (payload-grammar fault) |
| `config replace-step <tok> replacement-step.yaml` · `config remove-step <tok>` · `config fill <tok> --from-file -` · `config fork <tok>` | no | each faults on the *token's* shape |
| `config set docs-root <tok>` · `config set default-workflow <tok>` | no | — |
| `doc set-field commit:probe#scope --value <tok>` | no | reachable — driven with a task named `probe`: `set commit:probe#scope = cli`, exit 0 |

**And one member outside that registry, found by driving the same question at `SLUG_DOORS`:**

```
rig: committed-singletons (no `adr:keeper` committed), task open
$ jigc rename adr:keeper --to Axis --slug "<A300>"
  exit=1  blocking · store.not-found — no managed doc `adr:keeper` to rename
                                       (expected at docs/decisions/keeper.md)
$ jigc rename adr:keeper --to Axis --slug ok-slug
  exit=1  same
# after actually creating + finalizing an adr titled "Keeper":
$ jigc rename adr:keeper --to Axis --slug "<A300>"
  exit=1  blocking · write.slug-name-ceiling — `--slug "aaaa…`
```

So `SLUG_DOORS`' `rename` row reaches its guard **only when the corpus holds `adr:keeper`** — the
store lookup precedes the slug guard at that one door, while the other five adjudicate the slug
first. **Classification: A1-D3's class is 2 members, not 1** — one hard (`relocate`, unreachable on
every shipped doctype) and one corpus-conditional (`rename`, silently proving nothing on a corpus
without the fixture). Both are *fence* weaknesses, not binary defects.

---

## §3 — what changed against the review's rows

1. **A1-D1's door count is 5 write doors in the review; driven it is 6 write doors + 1 read door.**
   `jigc rename <placement-type>:<bogus> --to T --slug <bogus>` takes the head at **exit 0** and
   **commits** (§4.1) — `rename` ∈ `COMMITTING_DOORS`, which is the registry M51's claim clause
   binds. And `milestone add-from-spec` — a `DoctypeArg::Address` **read** door the review's D1 table
   does not list — resolves the bogus head against the real file (§4.2). The review's framing
   ("the read doors know the rule, the write doors do not") is **half true**: one read door does not.

2. **A1-D4's `rename` cell is a rig artifact.** The review records `rename` at the ceiling cell as
   `rename.in-flight`; that is the answer of a rig with an open task. Driven with **no** task and
   **no** milestone open: `rename vision:<A300>` → `write.identity-change`, `rename adr:<A300>` →
   `store.not-found`. The **count of 5 code-less doors is unchanged**; the reason for one of the
   five non-members is different from what the row says. (Also: `write.identity-change`'s route at
   that cell prints ``--slug <A300>``, an argv the same binary refuses with
   `write.slug-name-ceiling` — a printed route that cannot run.)

3. **A1-D5 names one knob; `ROOT_KNOBS` has two and they fail differently.** `placement-root` at the
   ceiling is *not* a `validate`-can't-run state — it is a **`validate` exit-0 false green** over a
   store where two managed docs are unreachable (§4.3). A fix keyed on the reported symptom
   ("`validate` cannot run") would miss the worse half.

4. **The class boundary for A1-D1 is `placement:`, not `singleton:`** — driven in §2.4. A fix keyed
   on `schema.singleton` would refuse writes that are correct for a location-homed singleton; a fix
   keyed on `schema.placement` matches the mechanism (`canonical_path` ignoring the slug) exactly.

5. **A1-D2's real cost is downstream of the move, and the review stops at "recoverable".** It is
   recoverable as a *rename*, but what it leaves behind is a **managed identity no address reaches**
   (§4.4) — the exact state `design/validation.md:723` says `ingest.unaddressable-identity` exists
   to prevent.

6. **A1-D3's class is 2, not 1** (§2.7).

7. **Not a change but load-bearing for the plan:** `jigc doc schema <ty> --format json` (the
   1.0-pinned, separately-versioned schema read, contract-version **6**) projects
   `['contract-version','fields','schema-version','sections','type']` — **no `singleton`, no
   `placement`, no `location`**. Driven over all 16 doctypes, every one reports `singleton=None`.
   A driver reading the pinned contract has **no machine-readable way to learn that `vision:alpha`
   is illegal** — which is why the well-formed-control cell is the one an agent actually lands in.

---

## §4 — latent defects (driven, in no §A row)

### 4.1 `jigc rename` accepts a bogus placement-singleton head with `--slug`, commits, and rewrites the schema-supplied H1 — exit 0

```
rig=$(dev/jigc-rig committed-singletons --binary ~/.local/bin/jigc) || exit; eval "$rig"
$ jigc rename "vision:alpha" --to "Phantom"
  exit=1  blocking · write.identity-change — cannot reslug `vision:alpha` — a placement singleton's
          identity is fixed to its type … only a retitle is supported
  route: `jigc rename vision:alpha --to Phantom --slug alpha` keeps the identity that cannot move
         and rewrites only the title                         ← the route composes the BOGUS slug
$ jigc rename "vision:alpha" --to "Phantom" --slug alpha
  exit=0
  renamed vision:alpha -> vision:alpha (VISION.md -> VISION.md), repointed 0 referrer(s)
$ git log --oneline -1   → cda9cf5 rename VISION.md -> VISION.md
$ sed -n '5p' VISION.md  → # Phantom
```

Two faults in one drive:
* **the head**: a bogus slug reaches a committing, `MovesOnBehalf` door at exit 0 (the class of
  A1-D1, one door further than the row);
* **the route**: the refusal at the no-`--slug` cell *composes the caller's bogus slug into the
  escape hatch it prints*, so following jigc's own route verbatim is what lands the bogus write.
  That is a law-1 problem at a `Route::mechanical` producer.

**Separately (a pre-existing sibling, driven as a control on a clean rig):**
`jigc rename vision:vision --to "Phantom" --slug vision` → exit 0, `# Phantom` in `VISION.md`;
`jigc rename changelog:bogus --to "Phantom CL" --slug bogus` → exit 0, `# Phantom CL` in
`CHANGELOG.md`. So `rename` rewrites a `display-title:` singleton's H1 **at the canonical slug too**,
while `doc rename` refuses the identical act with *"its `# H1` is supplied by the schema (`Vision`),
so it carries no author-owned title or slug"*. `jigc validate` afterwards is **exit 0** and
`doc show vision:vision` serves the wrong H1. Two doors disagree about whether the doc has an
author-owned title, and the committing one takes it.

### 4.2 `jigc milestone add-from-spec` resolves a bogus placement-singleton head and routes at an address `doc show` refuses

```
$ jigc milestone create "Probe milestone"      → minted milestone:probe-milestone
$ jigc milestone add-from-spec probe-milestone "vision:alpha"
  exit=1
  blocking · store.no-such-section — `vision:alpha` names no `criteria` section to seed from
    at: vision:alpha
    route: `jigc doc show vision:alpha` — it carries `meta`, `thesis`, `invariants`, `open-questions`
$ jigc doc show vision:alpha
  exit=1  blocking · store.not-found — `vision:alpha` names no committed doc: `vision` is a
          singleton, so its only address is `vision:vision`
```

The door **read the real `VISION.md`** (it enumerates its sections) under an identity its sibling
read door refuses, and its route is a dead end. Same at `roadmap:bogus` and at the 300-char head.
This is a `DoctypeArg::Address` **read** door, so it is inside A1-D1's registry and outside its
door list.

### 4.3 `jigc config set placement-root <over NAME_MAX>` leaves two managed docs unreachable, and `jigc validate` is exit 0 over it

```
rig=$(dev/jigc-rig committed-singletons --binary ~/.local/bin/jigc) || exit; eval "$rig"
$ jigc config set placement-root "$(python3 -c "print('a'*300)")"
  exit=0
  config: set `placement-root` = `aaa…`
  relocating the committed doc(s) stranded by the `placement-root` re-point to `aaa…`:
    - docs/decisions-log.md: could not relocate (creating the destination dir for aaa… failed …)
    - docs/roadmap.md: could not relocate (creating the destination dir for aaa… failed …)
$ git status --short          → (only `?? .jigc/config/manifest.yaml`) — nothing moved
$ jigc doc list               → exit 0; only changelog:changelog and vision:vision are listed
$ jigc doc list --format json → top-level keys ['docs'] — roadmap and decisions-log are ABSENT,
                                 and there is no `orphaned` key holding them
$ jigc doc show roadmap:roadmap
  exit=1  blocking · store.not-found — could not read `roadmap:roadmap` at `aaa…/roadmap.md`
$ jigc validate               → exit 0 (advisory `file-state.orphaned-doc` ×2)
```

Four distinct problems, all from one accepted token: the knob lands despite the move floor failing
for **every** subject (a partial failure narrated inside a **success** ack, no code, no route,
exit 0); the contract-pinned index read **silently drops** two managed docs rather than reporting
them; `doc show` says a committed doc does not exist; and `validate` is **green** — the sibling
`docs-root` cell at least fails loudly at exit 1. `file-state.orphaned-doc` is not a
`STORE_EXIT_FLIPS` member (the six read at `render.rs:948` are `probe-unreliable`, `oob-rename`,
`unmigrated-corpus`, `ahead-corpus`, `orphaned-instance`, `foreign-squatter`), which is why the
sweep stays exit 0.

The **same door refuses the same harm one cell over**, in its own words:
`jigc config set placement-root README.md/sub` → exit 1 `config.unusable-root` — *"`README.md` is a
file, not a directory — **every move into it fails while the knob lands anyway**"*.

*Measured constraint (not a recommendation):* a door-only length check is sufficient for **this**
token because the sink (`create_dir_all` under the repo root) is reached from the same call, but the
`doc list` drop and the `validate` green are independent of how the knob was set — they reproduce
for **any** stranded placement home, including the short one I drove (`placement-root elsewhere`
works and moves; a knob set while the move fails does not).

### 4.4 `jigc relocate` mints managed identities the address grammar refuses — the state `ingest.unaddressable-identity` exists to prevent

```
rig=$(dev/jigc-rig fresh --binary ~/.local/bin/jigc --pack-from-dev) || exit; eval "$rig"
$ mkdir -p notes && printf '# My Note\n\nsome prose\n' > "notes/My Note.md"
$ printf '# Up\n\nx\n' > notes/UPPER_CASE.md && git add notes && git commit -qm notes
$ jigc relocate adr --from notes
  exit=0
  freeze-exempt relocation: 2 moved, 0 displaced, 0 blocked
    moved     notes/My Note.md -> docs/decisions/My Note.md
    moved     notes/UPPER_CASE.md -> docs/decisions/UPPER_CASE.md
$ jigc doc list
  id  path  state
  adr:My Note      docs/decisions/My Note.md      managed
  adr:UPPER_CASE   docs/decisions/UPPER_CASE.md   managed
$ jigc doc show adr:UPPER_CASE
  exit=1  blocking · store.malformed-slug — "UPPER_CASE" is not a valid doc slug — the `<slug>` head
          of address `adr:UPPER_CASE`
  route: `jigc doc list` lists the committed docs and the identity each one carries …
```

Followed, that route hands back the same unaddressable id. `design/validation.md:723` states the
rule for the sibling door: *"its subject is exactly the set `cli::ingest::home_identity` answers
`None` over — one class with two shapes and one consequence, **no `<type>:<slug>` address reaches
the file** … **One predicate, not two agreeing ones**"*. `relocate` is a **second producer** of that
exact state and asks no predicate at all. Driven on the `.jigc` cell of §2.6 the same way:
`.jigc/AGENT.md` → `adr:AGENT`, `doc list` calls it `managed`, `doc show adr:AGENT` refuses
`store.malformed-slug`.

*Measured constraint:* `relocate`'s destination basename **is** the source basename by design
(byte-faithful move), so the identity is decided by a filename the caller never typed — a guard on
the `--from` token alone cannot see it; the predicate has to be asked about the **destination**.

### 4.5 `jigc config set docs-root ""` silently means the repo root

```
$ jigc config set docs-root ""
  exit=0  config: set `docs-root` = `.` — written to `.jigc/config/` …
```

`CLAUDE.md` → M50 records the design as *"`""` unset and `.` the repo root kept **unmixable at the
type level**"*. The type-level distinction may well hold inside the resolver; the **door** mixes
them. Same at `placement-root`. Not driven further (what a doc created afterwards does).

### 4.6 `x/../y` is accepted and silently normalized at both root knobs

`jigc config set docs-root "x/../y"` → exit 0, ack ``set `docs-root` = `y` ``. The escaping form
(`docs/../../out`) is correctly refused `config.untrackable-root`. Recorded because the traversal
cell of the acceptance design is satisfied by the escaping spelling only; the in-bounds `..` is a
different answer (silent rewrite) and no surface says the token was changed before it was stored.

---

## §5 — honest bounds

* **I did not re-verify the five §A rows**, per the brief. Where I state one "confirmed" it is
  because a class drive passed through the same cell, not because I re-ran the row's repro.
* **`task bind` × the singleton cell was not driven** — it is unreachable on both shipped packs
  (one `reads` role exists, typed `spec`). I did not manufacture a workflow to reach it.
* **`deferral-ledger`** (the fifth placement doctype) was driven only in its *uncommitted* state; I
  did not build a corpus that commits it, so its membership in class 1 is inferred from the
  mechanism (`canonical_path` ignoring the slug) plus four driven siblings, not driven directly.
* **The `doc author` payload path was driven only far enough** to construct the manufactured
  singleton; I did not sweep `doc author` for an address-carrying payload form (the long help
  documents none, and `DOCTYPE_DOORS` gives it `Bare`).
* **Row 12 of §2.5** (`migrate`, `config insert-step`, `--from-file` at the ceiling) was **not
  driven** — I took the review's §2.4 verdicts rather than re-running them.
* **`WORK_UNIT_ID_DOORS` was sampled at 5 of 25** for the ceiling cell. The five all answer by
  lookup with no path built from the token; I did not drive the other 20.
* **§4.3's `doc list` drop was driven at the ceiling value and at `placement-root .`** (a value I set
  myself, which produced the same disappearance); I did **not** isolate whether the drop has any
  other trigger, nor drive `migrate-corpus` or `relocate` over that state.
* **§4.1's H1 rewrite**: I drove that `validate` is exit 0 and that `doc show` serves the wrong H1.
  I did **not** drive whether `migrate-corpus`, a compose step or the pack-load `display-title`
  fence later objects.
* **rigA/rigB contamination**: rigA accumulated jigc state across `git reset --hard` cycles (a stale
  `reconciliation.rename` for `adr:SKILL` appeared). Every §4.4 claim was **re-driven clean on rigB**
  from a fresh rig; the rigA output is cited only for the `.jigc`/`.claude` move itself, which is
  the first command run on that rig.
* **One measurement I corrected mid-run**: my first `dev/jigc-rig … --schema` capture folded stderr
  into the script and produced garbage shell. It was rebuilt with stdout only and `|| exit`; no
  verdict in this ledger rests on the bad rig.
* **Nothing in this ledger is a recommendation.** Where a fix shape looked forced (§4.3, §4.4) I
  stated it as a measured constraint on where a guard can sit, not as a proposal.
