# M52 baseline — area `contracts` (axis 5, the pinned envelope contracts)

**Provenance.** Binary `/Users/maurice/.local/bin/jigc` → `jigc 1.0.0-rc.15`,
sha256 `126f1584f183636bb6cd9e782b1dc26aca28dd1afaf2fb83da1fd0e5febc8fa9`, **release** posture
(no `#[cfg(debug_assertions)]` route fence). Repo HEAD `7637a46f1908af7cffd83cc2e5e96dc2365744a0`,
tree clean, **no cargo run, no edit to the working repo**. Date 2026-09-16/17.
Rig states used: `bare` · `fresh` · `committed-singletons` · `refs-post-hoc`, each built by
`rig=$(dev/jigc-rig <state> --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"` (two-step
eval, `mktemp -d` roots, no teardown, no `rm -rf` on a variable path). Every sweep cell that could
mutate state ran **in its own `copytree` copy** of the rig repo under the rig's own `mktemp -d` root
— a first pass without that isolation produced two wrong readings (`setup` reporting
`dirty-install-path` because `uninstall` had run in the same copy three argv earlier), which are
discarded and not reported.
Rows assigned: DEFECT A · B · C (origin driver) and DEFECT D (origin codex) of
`completions/artifacts/M51/per-axis-review/axis-5.md`. Per the brief, **the §A rows were not
re-verified**; what follows is the class behind them.

---

## §1 · The class enumeration

### 1.1 The registries (grep, hit count, what the pattern misses)

| registry | grep | file:line | rows |
|---|---|---|---|
| `VERB_KINDS` | `awk 'NR>=1669&&NR<=1790' crates/cli/src/cli.rs \| grep -o '(&\[[^]]*\], VerbKind::[A-Za-z]*)'` | `crates/cli/src/cli.rs:1669` | **47** |
| `ENVELOPE_ARMS` | `awk 'NR>=5316&&NR<=6100' crates/cli/src/render.rs \| grep -n 'path:\|arm:\|outcome:\|shape:\|origin:\|status:\|root:'` | `crates/cli/src/render.rs:5316` | **60** (58 leaf-owned + 2 `path: &[]` reject rows) |
| `STORE_EXIT_FLIPS` | `awk 'NR>=948&&NR<=1080 && /id: "/' crates/cli/src/render.rs` | `crates/cli/src/render.rs:948` | **6** (`probe-unreliable`, `oob-rename`, `unmigrated-corpus`, `ahead-corpus`, `orphaned-instance`, `foreign-squatter`) |
| exit taxonomy | `grep -n "pub const EXIT" crates/cli/src/task.rs` | `crates/cli/src/task.rs:99-130` | **5** (`EXIT_CODES`, 0/1/2/3/4) |

**What the `ENVELOPE_ARMS` extraction misses, and it is the whole point of this axis:** the table is a
*declaration*, so grepping it can only tell you what is declared, never what the binary emits. The
registry's own doc-comment says so (*"a suite that renders a witness cannot see an arm the dispatch
chooses"*). Every class below was therefore enumerated **by driving**, with the grep used only to
bound the *production sites* a fix would touch.

`ArmOutcome` (`render.rs:5135`) has three members; the declared partition over the 60 rows is
**55 `Success` · 3 `Adjudicated(n)` · 2 `Reject`** (`Adjudicated`: `task finalize | Blocked` (3),
`task finalize | MigrationReviewHold` (4), `milestone finalize | Blocked` (3)).

### 1.2 The bare-`Finding` producer set (DEFECT A's production cost)

```
$ grep -rn "setup_block" crates/cli/src/ crates/cli/tests/
crates/cli/src/render.rs:3383   pub fn setup_block(format: Format, finding: &Finding) -> String   <- the producer
crates/cli/src/render.rs:6085   fn the_setup_block_seam_passes_the_declared_singleton()            <- the unit test pinning the shape
crates/cli/src/cli.rs:738       eprintln!("{}", render::setup_block(format, &finding));            <- run_setup
crates/cli/src/cli.rs:766       eprintln!("{}", render::setup_block(format, &finding));            <- run_uninstall
$ grep -n "json(finding)\|json(&finding)" crates/cli/src/render.rs
3389:        Format::Json => json(finding),      <- the ONE line that emits the third shape
```

**1 producer · 2 call sites · 1 in-crate unit test.** Both doors' surfaces are unchanged
(`Result<_, Finding>`), stated by name at `crates/cli/src/locate.rs:97`.

The **code axis under those two doors** is much larger than the producer count:

```
$ grep -o '"setup\.[a-z-]*"\|"uninstall\.[a-z-]*"' crates/cli/src/setup.rs | sort -u   # 20 setup + 12 uninstall
$ grep -o '"setup\.[a-z-]*"' crates/cli/src/pack.rs | sort -u                          # + setup.dirty-install-path
$ grep -o '"uninstall\.[a-z-]*"' crates/cli/src/milestone.rs | sort -u                 # + uninstall.dirty-worktree
```
**21 `setup.*` + 11 `uninstall.*` = 32 distinct codes** can be minted at those two doors. At least two
(`setup.forced-install-path`, `uninstall.remove-guide`) ride the *success* envelope's `findings` key
rather than the `Err` path, so the reject-shape class is **≈30 codes**, of which the review drove 4
and this baseline drove 2 more (§2.1). *What this grep would miss:* a code composed by
`format!`/a const rather than a literal — none found, but the pattern cannot prove it.

### 1.3 The pre-dispatch failure-point set (DEFECT D's class)

Read top-down through `crates/cli/src/main.rs` → `Cli::try_parse()` → `Cli::dispatch()`
(`cli.rs:568`). **Eleven** failure points can occur before or beside a per-verb reject funnel:

```
$ grep -n "current_dir()" crates/cli/src/cli.rs      # 24 hits
cli.rs:540  (.ok()? inside refuse_on_posture)
cli.rs:695,720,753,776,794,810,830,875,900,923,941,959,983,1067,1428,1444,1470,1495,1519,1543,1568,1590,1610  (23 match arms)
$ grep -rn 'eprintln!("warning' crates/cli/src/*.rs  # 3 hits
milestone.rs:4595 · pack.rs:1786 · pack.rs:1793
```

| # | failure point | pre/post funnel | driven verdict |
|---|---|---|---|
| 1 | `current_dir()` — **24 sites** | **PRE both funnels** | plain text, format ignored — **DEFECT D** |
| 2 | clap usage error (unknown flag, unknown subcommand, missing operand) | pre-dispatch | exit 2, plain stderr — **declared carve-out** |
| 3 | clap invalid `--format` **value** | pre-dispatch | exit 2, plain stderr — declared |
| 4 | `--help` / `--version` | pre-dispatch | exit 0, plain stdout — declared |
| 5 | not a git repository | post-funnel | `{error}` ×45 / bare `Finding` ×2 |
| 6 | no project cascade layer (`bare`) | post-funnel | `{error}` / findings envelope |
| 7 | **malformed project `packs.yaml`** — 2 sites, `pack.rs:1786/1793` | **beside the funnel, format-blind** | plain `warning:` lines **prepended to the reject envelope on stderr** — **LATENT DEFECT LD-1** |
| 8 | pack resource missing (`JIGC_PACK_DIR` garbage/absent) | post-funnel | `{error}` carrying a flattened `pack.resource-missing` |
| 9 | project schema shadow with a bad key | post-funnel | `{error}` |
| 10 | `.jigc/` unreadable (chmod 000) | post-funnel | `{error}` (mis-diagnosed, LD-4) + bare `Finding` at the two doors |
| 11 | `$HOME` unset | post-funnel | `{error}` ×45; bare `Finding` at 2 — the **declared bound** at `locate.rs:104-113` |
| — | `.jigc/version` absent | — | **not a failure**: exit 0 at every leaf driven |

**Two of the eleven bypass or corrupt both funnels** (#1 and #7). Three (#2–#4) are a *declared*
carve-out, not a bypass.

### 1.4 The `doc show` projection space (DEFECT B's class)

Enumerated from `design/doc-read-surface.md:28-33` + `:73-84` and `engine::address::Fragment`
(`crates/engine/src/address.rs:80-88` — `UnitLeaf` · `UnitItem` · `UnitItemLeaf`, three fragment
variants plus the bare-unit and whole-doc forms). The design doc declares **nine** projections;
`ENVELOPE_ARMS` declares **four**.

---

## §2 · The drives

### 2.1 Class 1 — the reject-envelope shape at every leaf, in four cells

Harness: `scratchpad/contracts/{leaves.py,drive.py,sweep_iso.py}` — the 47 `VERB_KINDS` leaves with a
minimal clap-satisfying argv each, driven `--format json`, stdout and stderr each classified as
`{error}` · `{findings, schema_version}` · bare `Finding` (8 keys) · plain text · object/array · empty.

**Cell A — outside a git repository (47/47).**
```
D=$(mktemp -d "${TMPDIR:-/tmp}/nogit.XXXXXX"); cd "$D"; HOME="$D"
jigc --format json <each of the 47 leaves>
observed tally: {'ERROR-ENVELOPE': 45, 'BARE-FINDING': 2}
  setup      exit=1 stdout 0 bytes stderr bare Finding, code "setup.repo-root"
  uninstall  exit=1 stdout 0 bytes stderr bare Finding, code "uninstall.repo-root"
  the other 45: exit=1 stdout 0 bytes stderr {"error": "not inside a git repository (no `.git` found from …) — run jigc from inside …"}
```
**built + proven** for the 45; **latent defect (already §A DEFECT A)** for the 2.

**Cell B — a git repo with NO `jigc setup` (`bare` rig), each leaf in its own copy (47/47). NOT SWEPT BY THE REVIEW.**
```
rig=$(dev/jigc-rig bare --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
# each leaf run in a fresh copytree of $REPO
observed tally: {'ERROR-ENVELOPE': 21, 'FINDINGS-ENVELOPE': 21, 'stdout:OBJECT': 4, 'stdout:ARRAY': 1}
  21 × exit=1 stderr {"error": "this project isn't set up — run `jigc setup` (no `.jigc/config/` cascade layer found)"}
  21 × exit=1 stderr {"findings":[…],"schema_version":3}   (finalize.no-task ×13, milestone.unknown ×8)
   5 × exit=0 on stdout  (start | UnsetProject · setup | Installed · uninstall | TornDown · task list | Rows · milestone create | RecordOnlyAck)
BARE-FINDING count: 0
```
**Result: the DEFECT A door class does not widen in this cell — `setup` and `uninstall` both *succeed*
here.** Classification for the 42 rejecting leaves: **built + proven**.

**Cell C — a set-up repo (`fresh` rig), unknown work-unit id / unknown address at every leaf (47/47).**
```
rig=$(dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
observed tally: {'FINDINGS-ENVELOPE': 22, 'stdout:OBJECT': 19, 'ERROR-ENVELOPE': 5, 'stdout:ARRAY': 1}
  the 5 {"error"} rows: migrate (unreadable source) · rename (store.not-found, flattened) ·
                        relocate (store.unknown-type, flattened) · config insert-step · config replace-step
BARE-FINDING count: 0
```
**built + proven.**

**Cell D — each door's own named refusal (the cause axis), driven individually.**

| argv | exit | stderr shape | code | class |
|---|---|---|---|---|
| `jigc --format json setup` (CLAUDE.md dirty) | 1 | **bare `Finding`**, keys `check, code, key, location, message, probe, route, severity` | `setup.dirty-install-path` | DEFECT A |
| `jigc --format json setup` (`.jigc` chmod 000) | 1 | **bare `Finding`** | `setup.write-bootstrap` | **DEFECT A — a 5th code, new** |
| `jigc --format json setup` (`$HOME` unset) | 1 | **bare `Finding`** | `setup.repo-root` | DEFECT A + the `locate.rs:104` declared bound |
| `jigc --format json uninstall` (`.jigc/scratch.txt` untracked) | 1 | **bare `Finding`** | `uninstall.untracked-workbench-file` | DEFECT A |
| `jigc --format json uninstall` (`.jigc` chmod 000) | 1 | **bare `Finding`** | `uninstall.untracked-workbench-file` | DEFECT A |
| `jigc --format json uninstall` (`$HOME` unset) | 1 | **bare `Finding`** | `uninstall.repo-root` | DEFECT A — **a 6th code**, new |
| `printf 'not: [valid\n' \| jigc --format json doc author adr --from-file - --task <gone>` | 1 | `{findings, schema_version}` | `finalize.no-task` | built + proven |
| `jigc --format json doc show decisions-log#decisions` | 1 | `{findings, schema_version}` | `store.no-such-section` | built + proven |
| `jigc --format json doc show vision#meta/grounded-in` (committed, absent field) | 1 | `{findings, schema_version}` | `store.no-such-leaf` | built + proven |
| `jigc --format json uninstall` (tracked files under `.jigc` **and** an untracked one) | 1 | **bare `Finding`**, one document, **narration suppressed** | `uninstall.untracked-workbench-file` | the guard fires *before* the M46 narration — **built + proven** (no two-document break) |

**Class 1 verdict.** 47 leaves × 3 uniform cells = **141 leaf-cells driven, plus 10 cause-specific
cells.** The bare-`Finding` shape appears at **exactly 2 doors**, confirming the review's door bound —
but on **6 distinct codes**, not the 4 the row names, out of ≈30 that can reach the same producer. The
class the wave must iterate is therefore **`{setup, uninstall} × every `setup.*`/`uninstall.*` code
returned as `Err`**, closed at one seam (`render::setup_block`'s `Format::Json` branch), not the four
reported codes.

### 2.2 Class 2 — the pre-dispatch failure points

**DEFECT D re-driven at three `VerbKind` positions (the review swept all 47; this is the sample the
brief asks for).**
```
W=$(mktemp -d "${TMPDIR:-/tmp}/gone.XXXXXX"); cd "$W"; rmdir "$W"      # cwd now does not exist
$ jigc --format json doc list        -> exit=1 stdout=[] stderr=[cannot determine the current directory: No such file or directory (os error 2)]   (Read)
$ jigc --format json uninstall       -> exit=1 stdout=[] stderr=[same]                                                                             (destroying)
$ jigc --format json task finalize nope -> exit=1 stdout=[] stderr=[same]                                                                          (committing)
```

**The timing question the brief asks — answered by driving, from the same deleted cwd:**
```
$ jigc --format json --no-such-flag doc list  -> exit=2 · stderr "error: unexpected argument '--no-such-flag' found"
$ jigc --format zzz doc list                  -> exit=2 · stderr "error: invalid value 'zzz' for '--format <FORMAT>'"
$ jigc --format json --version                -> exit=0 · stdout "jigc 1.0.0-rc.15" · stderr 0 bytes
```
**Measured constraint:** clap's parse — and therefore `--format`'s resolution, including its *value*
validation — completes with the cwd already gone. `Cli::dispatch()` (`cli.rs:568`) holds `self.format`
and already runs `refuse_on_posture(&self.command, self.format)` **before** the match, strictly
earlier than all 24 `current_dir()` sites. So **a single pre-dispatch funnel is armable with the format
in hand**; nothing about where the flag is parsed forces the present shape. (Not a recommendation —
a measurement of what the fix shape is *not* blocked by.)

**Collateral, driven in source and confirmed by the above:** `refuse_on_posture` itself reads
`std::env::current_dir().ok()?` (`cli.rs:540`) and **silently returns `None`** on failure, so the
M51 posture family is skipped, without a word, by the same fault. No harm follows *here* only
because the leaf then fails too.

**LD-1 (latent, §4) — a malformed project `packs.yaml`.**
```
rig=$(dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
# in a per-argv copy:  printf 'packs: [unclosed\n  : :\n' > .jigc/config/packs.yaml
$ jigc --format json task finalize nope
exit=1 · stdout 0 bytes · stderr 976 bytes:
  warning: …/.jigc/config/packs.yaml is not a valid pack-set list: …
  warning: …/.jigc/config/packs.yaml is not a valid pack-set list: …
  { "schema_version": 3, "findings": [ { … "code": "finalize.no-task" … } ] }
  >> json.loads(stderr) : NO -> Expecting value: line 1 column 1 (char 0)
$ jigc --format json doc show adr:nope   -> exit=1, FOUR warning lines then the envelope, same failure
$ jigc --format json doc list            -> exit=0, FOUR warning lines on stderr, the success document intact on stdout
$ jigc --format json validate / start    -> exit=0, warnings on stderr, success document on stdout
```

**The other nine points, driven at a 6-leaf sample (`doc list` · `validate` · `start` ·
`task finalize` · `uninstall` · `setup`), each in its own copy:**

| point | observed |
|---|---|
| `JIGC_PACK_DIR` at a garbage dir | `doc list`/`validate` → exit 1 `{"error": "blocking · pack.resource-missing — no composed pack ships `config/knobs` …"}`; `start` → same for `config/defaults`; `setup`/`uninstall` → exit 0. **built + proven** |
| `JIGC_PACK_DIR` at a nonexistent path | identical to the above. **built + proven** |
| `.jigc/version` removed | every one of the six → exit 0 / unchanged arm. **built + proven** (not a failure point) |
| `.jigc` chmod 000 | `doc list`/`validate` → `{"error": "this project isn't set up — run `jigc setup` …"}`; `start` → `UnsetProject` exit 0; `setup` → bare `Finding` `setup.write-bootstrap`; `uninstall` → bare `Finding`. **shape-limited — LD-4** |
| project schema shadow with a bad key | all → exit 1 `{"error": "the project schema shadow …"}`. **built + proven** |
| `$HOME` unset | 45 leaves `{error}`; `setup`/`uninstall` bare `Finding` `*.repo-root` — **declared bound**, `crates/cli/src/locate.rs:104-113` names it a law-1 wobble by name |

### 2.3 Class 3 — `doc show`'s projection set

Driven on `committed-singletons` (`changelog`, `roadmap`, `vision`, `decisions-log`), on a `fresh` rig
carrying a `milestone-record`, and on `refs-post-hoc` for the staged arm.

| # | argv (`jigc --format json doc show …`) | exit | JSON root | keys / value | declared row |
|---|---|---|---|---|---|
| 1 | `changelog` | 0 | object | `fields, item-count, schema-version, sections, slug, type` | `WholeDoc::Committed` |
| 2 | `vision --task <id>` | 0 | object | + `staged` | `WholeDoc::Staged` |
| 3 | `vision#thesis` | 0 | **scalar** | `"A deterministic CLI assembles …"` | `SlotSlice` |
| 4 | `milestone-record:cache-rework#meta` | 0 | object | `base, schema-version, status` (doc-keyed) | `FieldsGroupSlice` |
| 5 | `changelog#releases` | 0 | **array** | elements `changes, date, id, link, title` | **none** |
| 6 | `changelog#unreleased-changes` | 0 | **array** | elements `category, id, notes` | **none** |
| 7 | `roadmap#milestones` | 0 | **array** | elements `decomposition, id, proves, title` | **none** |
| 8 | `changelog#releases/1-0-0` | 0 | object | `changes, date, id, link, title` (doc-keyed) | **none** |
| 9 | `roadmap#milestones/m-alpha` | 0 | object | `decomposition, id, proves, title` | **none** |
| 10 | `changelog#releases/1-0-0/changes` | 0 | **array** (nested block) | elements `category, id, notes` | **none** |
| 11 | `changelog#releases/1-0-0/changes/added` | 0 | object (nested item) | `category, id, notes` | **none** |
| 12 | `changelog#releases/1-0-0/changes/added/notes` | 0 | scalar | `"- the trial-shaped fixture builder"` | `SlotSlice` |
| 13 | `changelog#releases/1-0-0/link` | 0 | scalar | the URL | `SlotSlice` (item field, not a slot) |
| 14 | `milestone-record:cache-rework#meta/status` | 0 | scalar | `"active"` | `SlotSlice` |
| 15 | `milestone-record:cache-rework#meta/base` | 0 | **object** | `{"sha": …, "short": …}` | **none** — a *compound field leaf* |
| 16 | `vision#meta/grounded-in --task <id>` | 0 | **array of strings** | `["research:context-loss"]` | **none** — a *ref field leaf* |
| 17 | `milestone-record:…#tasks/warm-the-read-cache/status` | 0 | scalar | `"active"` | `SlotSlice` |

**Measured: 17 addressable cells collapsing to 7 distinct root shapes, against 4 declared rows.**
The review reported six projections; the class is larger in the direction that matters most —
`ArmShape::ArrayOf`'s doc-comment (*"`jigc task list` is **the surface's one array**"*) is falsified by
**two structurally different arrays**: an array of *item objects* (rows 5–7, 10) and an array of
*strings* (row 16, a `ref` field's value). Cell 15 is a third undeclared shape — a **single field leaf
answering with an object**, which neither `SlotSlice(Scalar)` nor `DataKeyed` (*"the doc's field
**map**"*) describes, and which `doc show`'s own `Dispatch` reason (*"whole doc, field group, slot"*)
enumerates away.

**Cross-surface disagreement, driven (LD-2, §4):**
```
$ jigc --format json doc schema milestone-record   -> fields: [('base','string'), ('status','enum'), ('schema-version','int')]
$ jigc --format json doc show milestone-record:cache-rework#meta/base
   -> { "sha": "b029d10ef…", "short": "b029d10" }     # an OBJECT, where the other pinned contract says `string`
```

**Not populated, and worth the plan's knowledge:** no shipped doctype declares a **list**-cardinality
field (measured by projecting `doc schema` over all 16 doctypes — types present are `string`, `enum`,
`date`, `int`, `ref`, `code-anchor`, `owned-location`), yet `design/doc-read-surface.md:66` declares
*"a **list**-cardinality field as a json array of strings"*. A doctype author adding one would land a
**third** array projection against a registry that declares the surface has one array in total.

### 2.4 Class 4 — `ArmOutcome` vs the driven exit

`ArmOutcome::Success` (`render.rs:5136`) reads *"Exit 0, the document on stdout, no JSON on stderr."*
Driven against the **6 `STORE_EXIT_FLIPS` members** and the gate states, each in its own copy of the
`committed-singletons` rig.

| verb (declared arm / outcome) | `unmigrated-corpus` | `ahead-corpus` | `oob-rename` | `orphaned-instance` | `foreign-squatter` |
|---|---|---|---|---|---|
| `validate` (`StoreSweep` / **Success**) | **exit 1** | **exit 1** | **exit 1** | **exit 1** | exit 0 |
| `migrate-corpus --dry-run` (`Report` / **Success**) | exit 0 | **exit 1** | exit 0 | exit 0 | exit 0 |
| `upgrade` (`Swept` / Success, root `ValidationReport`) | 0 | 0 | 0 | 0 | 0 |
| `doc list` (`Index` / Success) | 0 | 0 | 0 | 0 | 0 |
| `ingest` (`Triaged` / Success) | 0 | 0 | 0 | 0 | 0 |

In every non-zero cell the **declared key set is exact** on stdout and stderr is 0 bytes:
`validate` → `blocking_probes, findings, report_only, schema_version, scope`;
`migrate-corpus` → `already_current, blocked, commit, dry_run, hook_output, migrated, unadopted, unfilled`.

Fixtures (all built by driving git + the binary, never by writing into `.jigc/`):
`unmigrated-corpus` = `sed` CHANGELOG.md's stamp to 1 + commit · `ahead-corpus` = docs/roadmap.md stamp
to 99 + commit · `oob-rename` = `git mv docs/roadmap.md docs/roadmap-old.md` + commit ·
`orphaned-instance` = a committed `docs/stray.md` with `schema-version: 1` front matter ·
`foreign-squatter` = a committed un-adopted `docs/decisions-log-two.md`.

**The gate rows:**
```
rig=$(dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
jigc start --workflow single-task "harden the cache"; jigc doc create adr --title "Cache strategy" --task harden-the-cache
$ jigc --format json task validate harden-the-cache
exit=3 · stderr 0 bytes · stdout keys ['findings','schema_version']
  codes: file-state.staged-copy, schema-conformance.required-slot-present ×4, schema-conformance.field-value-conformant
   -> `task validate | Report` is declared **Success**.  DEFECT C, re-driven.
$ jigc --format json milestone finalize cache-rework      (zero-contribution)
exit=3 · stdout keys ['findings','schema_version'] · code milestone.zero-contribution
   -> declared `Adjudicated(3)`.  matches contract.
$ jigc --format json milestone join cache-rework          (provisioned, sub-task untouched)
exit=0 · stdout keys ['findings','milestone','no_docs_from','overlay','schema_version']
   -> declared Success.  matches contract in this cell; a *blocked* join was NOT driven (§5).
$ jigc --format json doc set-field adr:cache-strategy#supersedes --value adr:does-not-exist --task <t>
exit=0 · findings [] · declared key set exact   -> a dangling ref does not flip a DocAck exit.  built + proven.
```

**Class 4 verdict.** The offending-**row** count is **3**, exactly as the review states
(`validate | StoreSweep`, `migrate-corpus | Report`, `task validate | Report`). What the review's row
does **not** say, and the plan needs: the **condition axis under `validate` is 4 driven members of 6**,
not the two the row names — so a fix that re-declares the two reported conditions is incomplete by the
wave's own complete-fix contract. And the class does **not** widen to the other two
`ArmRoot::ResultContract("engine::result::ValidationReport")` rows: `upgrade | Swept` held Success at
exit 0 across all five conditions, and `run_validate` (`cli.rs:1094`) is the **only** call site of
`render::validation_store_exit_flips` in the whole crate
(`grep -rn "validation_store_exit_flips("` → 1 production hit + 2 in-file tests), so the flip cannot
reach a third row today.

---

## §3 · What changed against the review's rows

1. **DEFECT A's door bound holds; its *code* bound does not.** The review bounds the class at
   *"2 doors × 4 driven codes"*. Driven: the 2 doors survive **141 leaf-cells across three uniform
   states** with no third offender, but the codes are **6 driven** (`setup.write-bootstrap` and the
   `$HOME`-unset arm of `uninstall.repo-root` are new) out of **≈30** that reach the same producer.
   The class is `{setup, uninstall} × the `Err`-returned `setup.*`/`uninstall.*` code set`.
2. **DEFECT A's fix cost, measured both ways** (per the brief; no recommendation):
   - *Move the two doors onto a declared arm.* **1 production function**
     (`render::setup_block`, `render.rs:3383`, the single line `Format::Json => json(finding)`),
     **2 call sites** (`cli.rs:738`, `cli.rs:766`), **1 in-crate unit test** that currently pins the
     bare shape (`render.rs:6085`). Both doors' `Result<_, Finding>` signatures are untouched, so
     `setup.rs` does not change. Routing them into the existing `{findings, schema_version}` arm needs
     **no version move at all** — that arm already exists and its keys are unchanged
     (`engine::result::SCHEMA_VERSION = 3`, `crates/engine/src/result.rs:27`).
   - *Declare a third arm instead.* **+1 or +2 `ENVELOPE_ARMS` rows** (the existing two reject rows
     carry `path: &[]`; a door-owned pair would carry `&["setup"]` / `&["uninstall"]`). No new
     `ArmShape`/`ArmOutcome` member is needed — `Object(&[…]) × Reject` already exists.
     **A measured correction to the brief's premise:** `doc schema`'s `contract-version` is **not**
     involved. Driven, it reads **6** (`jigc --format json doc schema milestone-record` →
     `contract-version = 6`) and `design/doc-read-surface.md:108` scopes it to *"any structural change
     to **this projection**"* — the schema projection, not the envelope registry. The reject envelope
     carries `engine::result::SCHEMA_VERSION = 3` or, for `{error}`, no version key at all. Whichever
     shape is chosen, the version question is **`SCHEMA_VERSION`'s and only if keys move**.
3. **DEFECT B is wider than "six projections where four are declared".** Driven: **17 addressable
   cells / 7 distinct root shapes**, including **two structurally different top-level arrays**
   (array-of-item-objects and array-of-strings from a `ref` field leaf) and an **object-valued single
   field leaf** (`#meta/base` → `{sha, short}`). `ArmShape::ArrayOf`'s *"the surface's one array"* is
   falsified twice over, not once.
4. **DEFECT C's row count is right; its condition axis is not.** 3 rows, as stated — but `validate`
   flips on **4 of 6** `STORE_EXIT_FLIPS` members driven, and the review names 2. The class does not
   extend to `upgrade` (driven Success at exit 0 on all five conditions) and cannot extend to any
   further row, because `validation_store_exit_flips` has exactly **one** production call site.
5. **DEFECT D's honest bound — *"a deleted cwd … is exotic as a hand-typed state"* — is overtaken by a
   sibling that is not exotic.** LD-1 below reaches the same broken discrimination predicate from a
   one-character typo in a file the operator is invited to hand-edit. The two are one class:
   *anything that writes non-JSON to stderr under `--format json`*, which is **26 sites** —
   24 `current_dir()` + 2 `pack.rs` warnings — not the 24 the review's source bound counts.
6. **A cell the review's D1/D2 never entered: a git repo with no `jigc setup`.** Swept here at 47/47
   with zero envelope violations — so the plan can treat it as clean *for the envelope question*.
   It is **not** clean for the state question: see LD-3.

---

## §4 · Latent defects (not in any §A row), each driven

### LD-1 · A malformed project `packs.yaml` prepends plain `warning:` lines to the reject envelope on stderr — DEFECT D's reachable sibling

```
rig=$(dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
printf 'packs: [unclosed\n  : :\n' > .jigc/config/packs.yaml
$ jigc --format json doc show adr:nope
exit=1 · stdout 0 bytes · stderr 1752 bytes:
  warning: …/.jigc/config/packs.yaml is not a valid pack-set list: …      (×4 — one per make_pack() call)
  { "schema_version": 3, "findings": [ { … "code": "store.not-found" … } ] }
  >> json.loads(stderr) raises: Expecting value: line 1 column 1 (char 0)
```
Producers: `crates/cli/src/pack.rs:1786` and `:1793`, both `eprintln!("warning: {err:#}")` inside
`make_pack()`, neither taking `format`. The comment above them states the intent — *"A malformed
`packs.yaml` is a real authoring fault; surface it rather than silently falling back to the base"* —
and the intent is right; the **stream and the format are the fault**. At exit 0 the same warnings ride
stderr while the success document rides stdout, so the contract's *"parse stdout; if stdout is empty,
parse stderr"* predicate survives — but at **every rejecting leaf** it does not.
**Classification: latent defect.** Against `design/command-output-contract.md` → Stream discipline
(a reject *"leaves stdout empty and writes exactly one document to stderr"*).

### LD-2 · `doc schema` and `doc show` disagree on the type of `milestone-record.base` — two pinned contracts, one field

```
$ jigc --format json doc schema milestone-record        # fields: ('base','string')
$ jigc --format json doc show milestone-record:<slug>#meta/base
{ "sha": "b029d10ef38430a57db331a50e634f444d2d3a8b", "short": "b029d10" }     # an OBJECT
```
`design/doc-read-surface.md:66` documents the object projection deliberately (*"clean lossless access
to both SHAs … json-projection-only"*) — but the **type surface a driver reads to know what to expect**
says `string`, and `doc schema` is itself contract-pinned at `contract-version 6`. A driver
type-checking `doc show` against `doc schema` is told the wrong type on the one compound field the
pinned surface carries. **Classification: latent defect** (a contradiction between two locked
artifacts' shipped outputs, not merely an undocumented shape).

### LD-3 · Six of nine `milestone` doors operate over a repo with **no `jigc setup`**, at exit 0, leaving only an untracked `.jigc/`

```
rig=$(dev/jigc-rig bare --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"   # a git repo, NO jigc setup
$ jigc --format json milestone create "Cache rework"      -> exit 0  {"hook_output":"","text":"minted milestone:cache-rework (shared base 00d7a5b)…"}
$ jigc --format json milestone add-task cache-rework "Warm the read cache"  -> exit 0
$ jigc --format json milestone provision cache-rework     -> exit 0  "provisioned 1 worktree(s) …"      # a real git worktree
$ jigc --format json milestone join cache-rework          -> exit 0
$ jigc --format json milestone finalize cache-rework      -> exit 3  milestone.zero-contribution        # a real adjudication
$ jigc --format json milestone discard cache-rework       -> exit 0
$ jigc --format json milestone execute cache-rework       -> exit 1  {"error":"this project isn't set up — run `jigc setup` (no `.jigc/config/` cascade layer found)"}
$ git status --porcelain
?? .jigc/
```
In that same state **21 of the 47 leaves refuse with the not-set-up `{error}`** (Cell B). No
`milestone-record` doc is written — `doc list` in that repo refuses, and the whole arc's state lives in
an untracked `.jigc/` that no clone sees, which is the precise inverse of *"`.jigc` is the workbench,
the repo is the record"* (`design/team-ready-state.md`). **Classification: latent defect** — an
un-gated door class, adjacent to this axis because it is the `RecordOnlyAck` rows shipping their
declared `Success` envelope in a state their siblings call unusable.

### LD-4 · An unreadable `.jigc/` is reported as *"this project isn't set up"*

```
rig=$(dev/jigc-rig fresh …) || exit; eval "$rig"; chmod 000 .jigc      # in a per-argv copy
$ jigc --format json doc list -> exit 1 {"error":"this project isn't set up — run `jigc setup` (no `.jigc/config/` cascade layer found)"}
$ jigc --format json validate -> the same
$ jigc --format json start    -> exit 0, OrientationView::UnsetProject
```
The project **is** set up; the layer is unreadable. The route (`jigc setup`) is followable and would
fail for the same reason. The envelope shape conforms, so this is a **law-1 wording** finding, not a
contract one. **Classification: shape-limited** — the detection is a `.is_some()` on a directory read
that cannot distinguish absent from unreadable. Same family as the `locate.rs:104` declared bound
(`$HOME` unset reported as `*.repo-root`), which is already **stubbed / declared** with its citation.

---

## §5 · Honest bounds

1. **The §A rows themselves were not re-verified** (the brief forbids it). DEFECT A's and C's repros
   were re-driven incidentally as the class's base cells; DEFECT B and D were re-driven at a sample.
2. **`STORE_EXIT_FLIPS`' `foreign-squatter` did not flip with my fixture** (a committed un-adopted
   `docs/decisions-log-two.md`). I do not claim the member is unreachable — my fixture did not reach
   it; the honest reading is *not driven*. **`probe-unreliable` was not driven at all**: it needs a
   pack probe that exits non-zero without a report, which means manufacturing a pack.
3. **A blocked `milestone join` was not driven.** A code collision between sub-tasks needs two
   worktrees with overlapping writes; I drove only the clean join (exit 0) and the zero-contribution
   `milestone finalize` (exit 3). So the `milestone join | Report` row's Success claim is confirmed in
   one cell only.
4. **`setup.pack-load` was not reached.** `JIGC_PACK_DIR` at a garbage directory left `jigc setup`
   at exit 0 on a `bare` rig; whatever reaches that code, it is not that. ≈24 of the ≈30 codes in
   DEFECT A's class are undriven — I drove 6 and bounded the rest by grep, which is a source read.
5. **`finalize.carried-staged` was not reached.** My fixture staged the file *after* `jigc start`, so
   the path was the task's own work and finalize landed it at exit 0 with the file in the manifest —
   a fixture error, not a gate finding. The blocking-finalize envelope is driven via
   `milestone finalize`/`task validate` instead.
6. **Cells B and C used one minimal argv per leaf.** A leaf with several miss shapes (`doc show` at
   nine address depths, `config *` at five target kinds) contributes one row per cell, not one per
   shape — those axes belong to the write-path and read-surface areas.
7. **No `--pack-from-dev` / project-pack composition was driven.** A manufactured pack could add a
   doctype with a **list**-cardinality field and so a third array projection; I measured that no
   *shipped* doctype has one and left the manufactured case undriven.
8. **The `--format agent` / `--format human` surfaces were not driven at all.** This axis is the JSON
   contract; a text-surface claim would need its own sweep.
9. **Everything in §1's production-site counts is a grep**, i.e. a lead. The counts I treat as
   load-bearing (1 producer, 2 call sites, 1 test for DEFECT A; 1 call site for
   `validation_store_exit_flips`) were each read in context; the ≈30-code figure was not.
10. **One discarded measurement.** The first pre-dispatch sweep ran all six argv in one repo copy and
    produced two false readings (`setup.dirty-install-path` where the tree was clean, caused by an
    earlier `uninstall` in the same copy). Every figure reported here comes from the per-argv isolated
    re-run; the first pass is not cited anywhere above.
