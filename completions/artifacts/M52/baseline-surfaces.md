# M52 baseline — area `surfaces` (axis 6 composed surfaces · axis 8 adopter docs / help / advisory arms)

**Provenance.** Binary `/Users/maurice/.local/bin/jigc` — `jigc 1.0.0-rc.15` (asserted before every
rig). Repo HEAD `7637a46f`, tree clean, **no cargo run**. Date 2026-09-16/17. Rig states used:
`fresh` (×6), `bare` (×2), `committed-singletons` (×4), `vendored` (×1) — every one via
`rig=$(dev/jigc-rig <state> --binary ~/.local/bin/jigc) || exit; eval "$rig"; cd "$REPO"`.
Nothing was written into the working repository. Captured output under
`…/scratchpad/surfaces/`.

Rows assigned: **A6-1 · A6-2 · A6-3 · CX-1 · CX-2 · CX-3 · D-1 · D-2.**

---

## §1 — the class enumeration

### 1.1 The workflow universe (classes 2 and 3)

```
grep -rn "creates-task:\|selectable:\|suppressed:" crates/cli/pack/workflows/ packs/methodology/workflows/
```
→ 17 dev-pack + 17 methodology-pack workflow files. Driven projection
`jigc describe --workflows --format json` → **34 definitions**, 22 carrying `router_hidden`,
12 on the catalog. `creates-task: false` = **4** (`router`, `milestone-execution`,
`ingest-existing`, `increment`). **What the grep would miss:** a workflow whose flags sit
outside lines 4–7 of its YAML, and any workflow a *project* pack contributes — neither exists
in the two embedded packs; the `describe` count (34) is the check on the grep (34).

### 1.2 Composed-argv forms (class 2)

Every `jigc start --workflow <w> "probe <w>"` and `jigc workflow <w> --preview` driven for all
34, one `fresh` rig → 68 captures. Extracted every backticked `` `jigc …` `` form:
**182 unique forms**. Each form's leaf resolved against the real CLI by longest-prefix
`jigc <leaf> --help` (exit 0) and every `--flag` in the form asserted present in that leaf's
help text. **Suspect count: 0** — no composed surface names a verb, subcommand or flag that
does not exist. **What this would miss:** an argv that parses but whose *operand* cannot be
supplied — which is precisely A6-2's and A6-3's class, so the class was re-enumerated on
placeholders instead (below).

Placeholder tokens across all 34 composed outputs (`<[A-Za-z]…>` inside a backticked form):

| token | workflows | kind |
|---|---|---|
| `<TITLE>` | 12 | author-supplied value, explained in situ |
| `<COMMIT_TYPE>` | 9 | author-supplied value |
| `<TYPE>` / `<SCOPE>` | 9 / 9 | author-supplied value |
| `<MILESTONE_ID>` | 1 (`milestone-execution`) | **identity the door neither resolves nor sources** |
| `<SPEC_ADDRESS>` / `<slug>` | 1 (`implement-from-spec`) | **identity from a list the door renders empty** |
| `<path>`/`<doctype>`/`<milestone-id>`/`<spec-addr>` | 1 each | generic instructional prose |

### 1.3 Set-rendering placeholders (class 3)

```
grep -rhno '{{[^}]*}}' crates/cli/pack/ packs/methodology/ | sed 's/.*:{{/{{/' | sort | uniq -c
```
→ the placeholders that render a **set or slice** (as against `{{task.id}}`, `{{include:}}`,
`{{cli.*}}`, `{{schema:*}}`):

| placeholder | occurrences | home |
|---|---|---|
| `{{ source }}` | **12** | the 12 `author-migration-*.yaml` steps |
| `{{ store.specs }}` | 1 | `steps/locate-from-spec.yaml` |
| `{{ milestone.tasks }}` | 1 | `steps/implement-tasks.yaml` |
| `{{ @task.spec#criteria }}` | 1 | `steps/locate-from-spec.yaml` |
| `{{ @task.decision.supersedes#decision }}` | 1 | `steps/superseded-context.yaml` |
| `{{ @task.vision.grounded-in#findings }}` | 1 | `steps/author-vision.yaml` |
| `{{ catalog }}` | 1 | `steps/present-catalog.yaml` |
| `{{ task.spec#goal }}`, `{{ task.spec#context }}`, `{{ task.commit#summary }}`, `{{ task.arch-doc#overview }}` | 4 | single-value, not a set |

**Detector run over the 34 driven compositions** (a line ending `:` followed by ≥2 blank
lines — an assertion followed by an empty render): **14 hits across 13 workflows.**

### 1.4 No-write doors (class 4a)

```
grep -n 'long = "dry-run"' crates/cli/src/cli.rs   →  1 hit (MigrateCorpus)
grep -rn 'dry_run: bool' crates/cli/src/*.rs       →  6 hits (2 clap-facing)
```
Driven against the binary (`jigc <leaf> --help | grep -- --dry-run`) over 12 candidate leaves:
**exactly 2 `--dry-run` leaves** (`migrate-corpus`, `task finalize`) and **1 `--preview`**
(`workflow`). Unqualified no-write *claims* in shipped help: **2** —
`migrate-corpus`'s summary and `jigc validate`'s *"read-only sweep"*. `task finalize --dry-run`
("commit nothing"), `workflow --preview` ("without minting a task") and `migrate-corpus`'s own
`--dry-run` arg help ("no migrated bytes, no relocation move, no commit") are **qualified and
true**.

### 1.5 `COMMITTING_DOORS` (class 5b)

`crates/cli/src/invocation_log.rs:130` — **10 rows over 9 clap leaves**
(`milestone finalize` contributes two squash arms).

### 1.6 The write-ack surface (class 5a)

`crates/cli/src/render.rs:2182` `enum DocAck` — **9 variants**, every one carrying
`findings: Findings`. `render::doc_ack`'s own doc-comment: *"`agent` / `human` emit a terse
one-line confirmation (**no footer**)"* — i.e. the text arm of all 9 drops `findings[]` by
construction, while every JSON arm serializes it.

---

## §2 — the drives

### 2.1 Class 1 — the sub-task composed surface × posture × boundary (A6-1)

Rig `fresh`; `milestone create "Surf probe"` (base pin `961f355`), two `sub-task` sub-tasks,
`milestone provision`.

**The state set is 4, not 3, and the door set is 2 — a 2×2 that is byte-identical in every cell.**

| # | state | door | argv | exit | observed |
|---|---|---|---|---|---|
| 1 | (a) pre-provision, off pin | `workflow` | `jigc workflow sub-task --task beta-work` | 1 | *"task `beta-work` is pinned to base 961f355 but you're on aba6848 … run `jigc milestone provision surf-probe` … then re-run this from that worktree"* |
| 2 | (a) | `start --task` | `jigc start --task beta-work` | 1 | **byte-identical to #1** (`diff` clean, stdout and stderr) |
| 3 | (a′) **provisioned**, off pin | `workflow` | same argv, shared checkout on `main` | 1 | **byte-identical to #1** — the same refusal for a state whose recovery is `cd .jigc/worktrees/beta-work`, not `milestone provision` |
| 4 | (b) in `.jigc/worktrees/beta-work` | `workflow` | `jigc workflow sub-task --task beta-work` | 0 | full composition |
| 5 | (b) | `start --task` | `jigc start --task beta-work` | 0 | **byte-identical to #4** |
| 6 | (c) shared checkout **at the pin** (`git checkout --detach 961f355`) | `workflow` | same | 0 | **byte-identical to #4** |
| 7 | (c) | `start --task` | same | 0 | **byte-identical to #4** |

`diff` of #4/#5, #6/#7, #4/#6, #5/#7 — all four empty.

**Classification: shape-limited → the class is 2× wider than the A6-1 row states.** The row
drives `workflow` × {(b),(c)}; the M51 baseline's cell 13 (`start --task` vs
`workflow sub-task --task`) is *also* undiscriminated, in **both** postures. And state (a′) —
provisioned but standing in the wrong checkout — reuses state (a)'s bytes verbatim.

**The footer's state-claiming lines, checked per cell** (`resume:` · `what's-left:` ·
`task scope:` · `create-gates:` · `also open:`):

| line | sub-task cells (b)/(c) | orphan cell (§4.1) |
|---|---|---|
| `resume:` | `jigc workflow sub-task --task beta-work` + *"run it from this sub-task's own worktree at `.jigc/worktrees/beta-work`"* — **the same bytes in both postures** | the **generic** `jigc start --task <id> — re-composes this workflow if context is lost` |
| `task scope:` | *"this task is a sub-task of milestone `surf-probe`, whose `jigc milestone finalize surf-probe` is its only commit boundary"* — **state-aware, correct** | the generic several-open-tasks text — **state-aware, correct** |
| `what's-left:` | identical in every cell (by design — one generator, `whats_left_coverage()`) | identical |
| `create-gates:` | `adr` — matches the workflow's declared gate | `adr` — correct |
| `also open:` | **absent** — rides only the two mint forms (`design/write-commands.md`:185); driven present on `start --workflow …` and absent here | present on the mint |

**So the un-swept axis is the composed *body*, not the footer.** `task scope:` already
discriminates sub-task-of-a-milestone from not; the body's *"The parent milestone's finalize is
the only commit boundary … this worktree's staged index … never `jigc task finalize` here"*
is emitted unconditionally.

**The boundary, driven end to end** (alpha works inside its worktree: 1 staged file, 1 unstaged
tracked edit, 1 untracked file; beta works in the shared checkout at the pin: `beta-code.txt`
staged — exactly what its composed body invited):

```
jigc milestone join surf-probe            → exit 0, "2 doc(s) merged"
jigc milestone finalize surf-probe        → exit 1  blocking · repo.head-detached
git checkout main                         (the shared checkout was detached to reach state (c))
jigc milestone finalize surf-probe        → exit 0
  finalized 0eea672 — Finalize milestone surf-probe (2 sub-tasks)
    added alpha-staged.txt
    modified docs/milestone-records/surf-probe.md
    2 files committed
    sub-tasks: alpha-work: 1 doc, 1 code file · beta-work: 1 doc
    discarded with the fan-out worktrees (not committed, not recoverable):
      alpha-work: README.md (never staged) · alpha-untracked.txt (never staged)
git ls-tree -r --name-only HEAD | grep -c beta-code.txt   → 0
git status --short                                        → A  beta-code.txt
```

**Two corrections to the A6-1 row, both load-bearing for a fix:**

1. **A left-out narration *does* exist at the milestone boundary — for one axis only.** The
   row says the boundary "has no equivalent" of `task finalize`'s `left-out` manifest
   (`grep left_out crates/cli/src/milestone.rs` finds only a comment). Driven, the ack carries
   *"discarded with the fan-out worktrees (not committed, not recoverable)"* naming alpha's
   unstaged and untracked files, plus a stderr `warning:` repeating them. What it does **not**
   cover is the *shared-checkout staged* axis: `beta-code.txt` is named on no surface. So the
   class is **a narration with one of two axes swept**, not an absent narration — which is a
   different fix shape.
2. **State (c) is reachable only with a detached HEAD, and `milestone finalize` now refuses
   there.** `milestone create` commits its record *after* pinning the base, so `main` is always
   ahead of the pin and the sub-task door refuses in the shared checkout on `main` (drive #3).
   Reaching state (c) requires `git checkout --detach <pin>`, and from there
   `jigc milestone finalize` exits 1 on `repo.head-detached` (M51's posture family). The A6-1
   loss therefore needs the operator to re-attach between composing and finalizing — the
   defect stands, its repro path is one step longer than the row shows.

**Latent finding in the same cell** (not in any §A row): standing in state (c), the sub-task's
own `what's-left:` route mis-diagnoses and mis-routes —
```
jigc task validate beta-work            → exit 0
advisory · reconciliation.rename — tracked managed doc milestone-record:surf-probe
  (docs/milestone-records/surf-probe.md) is missing, but the path has no history —
  the checkout moved underneath the file-state cache, not a deletion
  route: prune the stale baseline: `jigc unmanage docs/milestone-records/surf-probe.md`; …
```
The record is not missing — the checkout is at a pin that predates it. Following the route
unmanages the milestone's own record. Driven clean inside the provisioned worktree
(`jigc task validate alpha-work` → only the two real content findings), so it is
state-(c)-specific. **Classification: latent defect.** It is also the *only* signal in the
product that distinguishes state (c) — and it is on a different surface from the one A6-1 is
about.

### 2.2 Class 2 — `start --workflow <w>` over the off-catalog set (A6-2)

Driven: all 34 × {`start --workflow`, `workflow --preview`}.

**`milestone-execution`, both doors, same repo, a real milestone (`wt-probe`) present:**

```
jigc milestone execute wt-probe          → exit 0
  Run:   `jigc milestone provision wt-probe`
  Spawn: `cd .jigc/worktrees/alpha-work && jigc workflow sub-task --task alpha-work`
  Run:   `jigc milestone join wt-probe`
  Run:   `jigc milestone finalize wt-probe`          grep -c MILESTONE_ID → 0

jigc start --workflow milestone-execution "off verb"   → exit 0
  Run: `jigc milestone provision <MILESTONE_ID>`
  Run: `jigc milestone join <MILESTONE_ID>`
  Run: `jigc milestone finalize <MILESTONE_ID>`       grep -c '^Spawn:' → 0
  grep -c 'milestone execute' → 0
```
**A6-2 holds, and one datum it does not carry: the milestone existed in the repo at the time.**
The off-verb door has the information it needs and neither uses it nor names the verb that
would.

**The class is 13 members, not 1.** The same shape — a `selectable: false`, verb-routed
workflow composed at exit 0 through `start --workflow`, with its verb-supplied input absent —
holds for **all 12 `migrate-*` workflows**:

```
jigc start --workflow migrate-adr "probe migrate-adr"   → exit 0
task minted: probe-migrate-adr
Migrate the foreign ADR into a managed `adr`. Below is the foreign source the CLI
staged for you (read-only context — …):
                     ← three blank lines: {{ source }} rendered empty
… 160 further lines instructing the reader to map "the foreign ADR's headings"
```
and `jigc describe --workflows` states the constraint one surface over, as with
`milestone-execution`: *"verb-routed — reached only through `jigc migrate <path> --as adr`,
which stages the foreign source bytes the author step rewrites; a router pick would compose
with no staged source."* Zero of the 12 composing doors repeat it.
**Classification: latent defect (12 members), same class as A6-2.**

**Following one of them to its end is worse than a dead end — it lands a commit the composed
text promised it would not.** Driven to completion on a `fresh` rig:

```
jigc start --workflow migrate-adr "off verb migrate"   → exit 0
jigc doc author adr --from-file - --task off-verb-migrate  → exit 0   adr:off-verb-decision
jigc doc set-field commit:…#header/type --value docs / set-slot summary,body
jigc task finalize off-verb-migrate                    → exit 0
  finalized 34687a0 — docs: adopt the off verb decision
    promoted docs/decisions/off-verb-decision.md
    1 file committed
```
The composed body states: *"Finalizing a migration adds a review hold: a plain finalize commits
NOTHING — it renders the foreign source against the canonical rewrite and holds (exit 4) …
re-run the same finalize with `--approve`."* Driven in the state that door created, the plain
finalize **committed at exit 0** with no hold and no `--approve`. **Classification: latent
defect** — a law-1 contradiction between the composed step text and the binary, in the cell the
door itself produces.

**Composed-argv table (workflow × form × runnable), the summary of 182 forms:**

| class of form | count | runnable |
|---|---|---|
| fully-resolved `jigc …` (real ids substituted) | 168 | **yes** — leaf + every flag verified against the real CLI |
| value placeholder (`<TITLE>`/`<TYPE>`/`<SCOPE>`/`<COMMIT_TYPE>`) | 11 | yes once the author supplies the value the surrounding prose asks for |
| `<MILESTONE_ID>` (`milestone-execution`, off-verb) | 3 | **no** — no source on the surface |
| `<SPEC_ADDRESS>` / `spec:<slug>` (`implement-from-spec`, empty store) | 2 | **no** — the list it points at is empty |
| generic instructional (`<path>`, `<doctype>`, `<milestone-id>`, `<spec-addr>`) | 2 | n/a — prose, not a `Run:` |
| **`Run:`/`Spawn:` lines** | 125 `Run:` + 0 `Spawn:` across 34 compositions | 3 of the 125 carry an unresolvable identity (the `milestone-execution` trio) |

### 2.3 Class 3 — composed enumerations over an empty set (A6-3)

Detector output over the 34 driven compositions — **14 assert-then-empty renders**:

| workflow | placeholder | the asserting sentence | verdict |
|---|---|---|---|
| `implement-from-spec` | `{{ store.specs }}` | *"Pick the spec this work implements **from the committed specs below**…"* | **lies** = A6-3 |
| `implement-from-spec` | `{{ @task.spec#criteria }}` | *"**The bound spec's criteria** — `spec:<slug>#criteria`, one item per criterion, each carrying the `{#id}` anchor that addresses it:"* | **lies** — *not named in the A6-3 row* |
| `migrate-adr`, `-arch-doc`, `-changelog`, `-completion-record`, `-decisions-log`, `-deferral-ledger`, `-idea`, `-prd`, `-research`, `-roadmap`, `-spec`, `-vision` (12) | `{{ source }}` | *"**Below is the foreign source the CLI staged for you** (read-only context …)"* | **lies** in the off-verb cell |

**The honest controls, driven in the same sweep** — two set-renders that state their own empty
case inline and are therefore *not* members:

* `single-task` / `record-decision`, `{{ @task.decision.supersedes#decision }}`:
  *"…the superseded decision then appears below for reference … **(nothing appears if it
  supersedes none)**."*
* `form-vision`, `{{ @task.vision.grounded-in#findings }}`:
  *"The findings of ALL grounding research appear here for reference **(nothing appears until
  you set `grounded-in` and re-compose** …)"*

**So the fix shape is already shipped twice in the pack**, and the class the wave must iterate
is *"every set-rendering placeholder whose surrounding sentence does not state its empty
case"* — **14 renders / 13 workflows**, of which the A6-3 row names **1**.

`jigc doc list spec` over the same corpus is the third honest control (verbatim):
`jigc doc list — no committed \`spec\` docs` + a `note:` naming the open task and the
`--task` form.

### 2.4 Class 4a — the no-write doors with the invocation log ON (CX-1)

Rig `committed-singletons`, `jigc config set invocation-log true`, `find .jigc -type f` byte
snapshot before/after each door:

| argv | exit | `.jigc/logs/invocations.jsonl` | any other `.jigc` write | `git status` |
|---|---|---|---|---|
| `jigc migrate-corpus --dry-run` | 0 | **created / +194 B** | none | unchanged |
| `jigc validate` | 0 | +216 B | none | unchanged |
| `jigc task finalize <id> --dry-run` | 0 | + one record | none | unchanged |
| `jigc workflow single-task --preview` | 0 | + one record | none | unchanged |
| `jigc doc list` · `doc show vision` · `doc schema adr` · `task list` · `describe --workflows` | 0 | + one record each | none | unchanged |

Control, log **off**: the same 8 doors, byte-identical `.jigc` snapshot before and after —
*"NO `.jigc` write with the log off"*.

**CX-1 holds; its class is 2 unqualified claims, not 1.** Driven verbatim:
* `jigc migrate-corpus --help`: *"…`--no-commit` leaves the writes unstaged, **`--dry-run`
  writes nothing at all**."*  (+ the ack `corpus migration (**dry run — nothing written**): …`)
* `jigc validate --help`: *"Re-check the committed store and report drift — the store-wide,
  **read-only** sweep."*

Both are falsified by the same opt-in, gitignored log write. The **qualified** siblings
(`task finalize --dry-run`'s *"commit nothing"*, `workflow --preview`'s *"without minting a
task"*, `migrate-corpus --dry-run`'s arg help *"no migrated bytes, no relocation move, no
commit"*) are **true as written** and are **not** members — driven.

### 2.5 Class 4b — the installed guide against the binary (CX-3)

Rig `bare` → `jigc setup` → drive from the adopter repo.

**The brief's premise, checked:** the installed `SKILL.md` (34 217 B) is **not** a verbatim
concatenation of `QUICKSTART.md` (14 979 B) + `MIGRATING.md` (18 411 B). Longest matching
prefix: QUICKSTART **9 766 / 14 978 chars**, MIGRATING **823 / 18 410**. The divergence is a
link transform — repo-internal markdown links are flattened to bare paths
(`[MIGRATING.md](MIGRATING.md)` → `MIGRATING.md`). So the guides are the *body source*, edited
on the way in; a fix to a guide sentence lands in the adopter copy, a fix to a *link* does not
transfer verbatim.

**Fenced commands: 11.** Inline backticked commands: **32 `jigc …` forms + 5 `cargo`/`git`
forms.** All 32 inline `jigc` forms resolve against the real CLI (leaf + flags). Driven from
the adopter repo:

| guide line | argv | exit | observed |
|---|---|---|---|
| 30 | `cargo install --path crates/cli` | **101** | `error: …/repo/crates/cli` is not a directory. `--path` must point to a directory containing a Cargo.toml file.` — `ls crates` → no such directory |
| 47 | `jigc --version` | 0 | `jigc 1.0.0-rc.15` |
| 57 | `jigc setup` | 0 | install ack |
| 130 | `jigc start "add per-client rate limit at the gateway"` | 0 | router catalog |
| 139 | `jigc start --workflow single-task "…"` | 0 | `task minted: add-per-client-rate-limit` |
| 158 | `jigc doc list --task <id>` | 0 | `commit:add-per-client-rate-limit … managed` |
| 159 | `jigc doc show adr:<slug> --task <id>` | — | not driven (no adr in that task) |
| 160 | `jigc doc schema adr` | 0 | `doctype: adr (schema-version 2)` |
| 272 | `jigc task validate <id>` | 3 | the two expected content findings, each routed |
| 273 | `jigc task diff <id>` | 0 | `# staged docs …` |
| 184 | `jigc task finalize <id>` | 3 | the same two findings, routed |

**CX-3 holds, and the class is exactly 1 among path references.** The generated preamble
disclaims *documents* — *"every other jigc document named below lives in the jigc project's own
repository, not in this one, which is why none of them are links here"* — and 7 `design/*.md`
/ `implementation/*.md` references fall under it. **`crates/cli` at line 30 is the guide's only
non-document repo-path reference and is not covered by that disclaimer.** No other fenced or
inline command fails from an adopter repo.

### 2.6 Class 4c — what `setup` claims about the guide file (CX-2)

Rig `bare`, `G=.claude/skills/jigc/SKILL.md`, three states:

| state | argv | exit | guide sha256 (12) | observed |
|---|---|---|---|---|
| untouched | `jigc setup` (2nd run) | 0 | `051c91618015` → **unchanged** | ack carries the `jigc guides → …` row |
| user-modified, **uncommitted** (` M` in `git status`) | `jigc setup` | 0 | user bytes intact (`MY EDIT` still last line) | `advisory · adapter-guide.user-modified — … jigc left it untouched rather than clobber your edits — it is no longer version-matched to jigc 1.0.0-rc.15`; **no `jigc guides` row in the ack**; **`setup.dirty-install-path` did not fire on this path** |
| user-modified, **committed** | `jigc setup` | 0 | user bytes intact | identical advisory, identical ack |

**CX-2 holds, and the class is 3 sentences across 2 homes — one of them binary-generated.**

1. `crates/cli/src/setup.rs:80`, rendered into every adopter's `SKILL.md` line 8:
   *"`jigc setup` wrote this file from jigc 1.0.0-rc.15 and **owns it**: re-run `jigc setup`
   after upgrading the binary to refresh it."*  ← **generated by the binary**, and false in
   both modified states. The CX-2 row does not name this home.
2. `QUICKSTART.md` → installed line 71: *"…and **every `jigc setup` rewrites it**, so the
   guidance in your repo always matches the binary in your `PATH`."*  ← the row's home.
3. installed line 72: *"Re-run `setup` after upgrading and **the copy follows**."*

All three are false the moment the classifier says `adapter-guide.user-modified`, and the stale
copy carrying them is the one the adopter reads.

### 2.7 Class 5a — the advisory arm of the text manifests (D-1)

State: `committed-singletons` → `record-decision` task → adr authored → commit doc filled →
`jigc doc rename adr:use-redis-caching --to "Use Memcached caching" --task <id>`.
Each surface driven **twice** (default `agent`, then `--format json`), counting
`commit-recording.stale-title`:

| surface | text | json | verdict |
|---|---|---|---|
| `jigc doc rename … --task <id>` (producer) | **0** | 3 | **D-1** |
| `jigc task finalize <id> --dry-run` | **0** | 3 | **D-1** |
| `jigc task validate <id>` | 1 | 3 | matches |
| `jigc start` (orientation) | 1 | 3 | matches |
| `jigc task finalize <id>` (real) | 1 | — | matches |
| `jigc start --task <id>` (resume compose) | 0 | 0 | *the finding is on neither arm* |
| `task diff` · `doc list --task` · `doc show … --task` · `task list` · `validate` · `doc set-field` · `doc set-slot` | 0 | 0 | no producer reaches them |

**The class at the dry-run surface is every advisory, driven — not the one code F-9 named.**
Independent state (`single-task`, one staged file, changelog gate granted and unused):

```
jigc task validate tweak-the-vision        → exit 0
  advisory · changelog-recording.gate-granted-unused   (text: PRESENT)

jigc task finalize tweak-the-vision --dry-run          → exit 0
  finalize --dry-run — pre-commit manifest (nothing committed)
  would commit — docs: tweak the vision statement
    added c.txt                                        (text: NO findings line at all)

jigc task finalize tweak-the-vision --dry-run --format json
  keys: ['dry_run','findings','left_out','manifest','subject']
  findings: [('changelog-recording.gate-granted-unused','advisory')]
```
A second, unrelated finding family, same divergence. **Classification: the D-1 row's "the text
manifest arm drops *every* advisory" is confirmed by drive, on a code outside the
`commit-recording` family.**

**The producer half is structurally 9 arms wide, 1 driven.** `render::doc_ack` renders the
`agent`/`human` arm as *"a terse one-line confirmation (no footer)"* for all 9 `DocAck`
variants, each of which carries `findings: Findings` into JSON. Only `DocAck::Renamed` has a
known non-empty producer (`STALE_TITLE_CODE`, `task.rs:935/1039`), and driving `doc set-field`
/ `doc set-slot` in the post-rename state produced `findings: []` on both arms — so the other
**8** arms are *structurally* in the class and *not* driven to a non-empty finding. Stated as a
bound, not a claim.

### 2.8 Class 5b — the committing doors' help (D-2)

`jigc <leaf> --help` read for all 9 `COMMITTING_DOORS` leaves, and the commit driven where the
help is silent:

| # | door | help says it commits | ack says it commits | driven |
|---|---|---|---|---|
| 1 | `task finalize` | **yes** — *"The commit boundary — validate, render, stage, `git commit`, post-commit."* | yes | — |
| 2/3 | `milestone finalize` (both arms) | **yes** — *"The milestone commit boundary…"* | yes | `0eea672` |
| 4 | `rename` | **yes** — *"…`git mv`s it, and **commits** as one atomic transaction"* | — | — |
| 5 | `migrate-corpus` | **yes** — *"lands its own migration in a pathspec-limited commit"* | yes | — |
| 6 | `milestone create` | **NO** — and *"opening its **gitignored** area"* reads the other way | yes (`record commit: 7969071`) | `7969071 chore(milestone): open record for milestone:help-probe` |
| 7 | `milestone add-task` | **NO** | **NO** — ack is `added task:do-a-thing to milestone:help-probe` | `f3ec179 chore(milestone): record task:do-a-thing…` |
| 8 | `milestone add-from-spec` | **NO** (its two "committed" hits are *"a committed spec"*) | **NO** — ack is `seeded 1 sub-task(s) into milestone:spec-probe from spec:padding: pads-the-input` | `f148dbf chore(milestone): record task:pads-the-input…` |
| 9 | `milestone discard` | **yes** — *"in one record-only commit"* | — | — |
| 10 | `task discard` | **NO** (its two "commit" hits are inside `--force`'s *"no commit has a copy of"*) | yes (`record commit: 8fa4b04`) | `8fa4b04 chore(milestone): discard task:do-a-thing…` |

**D-2's class is 4 of 10 rows, not 1** — `task discard`, `milestone create`, `milestone
add-task`, `milestone add-from-spec` — and **2 of those 4 are silent in the ack as well**
(`milestone add-task`, `milestone add-from-spec`), which `task discard` (D-2's own row) is not.
`milestone create` is the sharpest: its help volunteers *"gitignored area"*, which an adopter
reads as *nothing is committed*, while the door moves HEAD.

Non-member checked for completeness: **`jigc setup` commits** (`install commit → e29437f`) and
its help says so (*"Refuses its own install commit when…"*) while it is **not** a
`COMMITTING_DOORS` member — noted, not graded (its identity is `setup`, not a rejection frame).

---

## §3 — what changed against the review's rows

| row | the row's claim | driven |
|---|---|---|
| **A6-1** | the surface separates (a) from {(b),(c)} — one door, three states | **2 doors × 4 states, all one byte-string**; `start --task` is undiscriminated too, and state (a′) (provisioned, wrong checkout) reuses (a)'s refusal verbatim |
| **A6-1** | *"the milestone boundary has no `left-out` equivalent"* | a loss narration **exists** and is driven (*"discarded with the fan-out worktrees (not committed, not recoverable)"* + a stderr `warning:`) — it covers the **worktree** axis and not the **shared-checkout-staged** axis. One axis swept of two |
| **A6-1** | the repro reaches finalize from state (c) | state (c) needs a **detached** HEAD (the record commit always moves `main` past the pin), and `milestone finalize` there exits 1 on `repo.head-detached`. The repro needs a `git checkout main` between compose and finalize |
| **A6-2** | 1 member (`milestone-execution`) | **13** — the 12 `migrate-*` workflows compose the same way at exit 0 with `{{ source }}` empty, and `describe` states the same unrepeated constraint for each |
| **A6-2** | the door lacks the milestone id | driven with a **real milestone present** in the repo — the door has the datum and neither uses it nor names `jigc milestone execute` |
| **A6-3** | 1 empty render | **14 renders / 13 workflows**; the second one is in `implement-from-spec`'s *own* composed text (`{{ @task.spec#criteria }}`). Two honest controls ship in the same pack (`single-task`'s supersedes slice, `form-vision`'s grounding), so the fix shape is already precedent |
| **CX-1** | `migrate-corpus --help` | **2 unqualified claims** — that one plus `jigc validate --help`'s *"read-only sweep"*; both falsified by the same log write, driven, with the log-off control |
| **CX-2** | 1 guide sentence | **3 sentences, 2 homes**, one of them **generated by the binary** (`setup.rs:80` → installed line 8, *"and owns it … to refresh it"*) |
| **CX-3** | 1 install line | **1 member confirmed, and the class is bounded**: the preamble's disclaimer covers *documents*; `crates/cli` is the guide's only non-document repo-path reference. Also: the guide body is **not** a verbatim copy of the two files (link transform) — a claim in the brief, checked and corrected |
| **D-1** | 2 silent surfaces | confirmed; **the dry-run text arm drops an advisory from a different family too** (`changelog-recording.gate-granted-unused`, driven), and the producer half is structurally **9 `DocAck` arms** (1 driven) |
| **D-2** | 1 door | **4 of 10 `COMMITTING_DOORS` rows**, 2 of them silent in the **ack** as well; `milestone create`'s help actively says *"gitignored"* |

---

## §4 — latent defects (not in any §A row), each driven

### 4.1 `jigc start --workflow sub-task "<intent>"` mints a milestone-less sub-task whose composed body forbids the only commit boundary it has

```
rig: fresh
$ jigc start --workflow sub-task "orphan sub task"                    → exit 0
task minted: orphan-sub-task
…
Author this sub-task's own commit prose. The parent milestone's finalize is
the only commit boundary: `git add` your code edits so the milestone can fold
this worktree's staged index, but never `git commit` and never `jigc task finalize`
here — either lands a commit on this worktree's detached HEAD and strands the
sub-task's work outside the milestone boundary.
…
resume: `jigc start --task orphan-sub-task`   — re-composes this workflow if context is lost
task scope: … (the GENERIC several-open-tasks text — no milestone named)

$ jigc milestone list        → exit 2 (no such subcommand; `list-tasks` is the verb)
$ jigc task list             → exit 0   orphan-sub-task  [sub-task]  orphan sub task
$ git worktree list          → the main checkout only
```
There is no parent milestone, no worktree and no detached HEAD. An agent obeying the composed
text never commits. Driven the other way:
```
$ printf 'x\n' > code.txt && git add code.txt
$ jigc doc set-field commit:orphan-sub-task#type --value feat --task orphan-sub-task    → 0
$ printf 'add the thing' | jigc doc set-slot commit:orphan-sub-task#summary … --from-file -  → 0
$ jigc task finalize orphan-sub-task                                   → exit 0
  finalized 2efdfa7 — feat: add the thing
    added code.txt
    1 file committed
```
So the body's prohibition is false in this cell **and** the affordance it forbids is the only
one that works. Note the **footer already knows**: `task scope:` renders its generic variant
here and its milestone-aware variant for a real sub-task — so one renderer in the same output
discriminates the state the body does not. `describe --workflows`' reason for `sub-task`
(*"…it has no commit boundary of its own, so a router pick could never land"*) is falsified in
this same cell. **Classification: latent defect.**

### 4.2 Following the off-verb `migrate-*` composition lands a commit the composed text says is impossible

Full repro in §2.2. `jigc task finalize off-verb-migrate` → **exit 0**, `finalized 34687a0`,
`promoted docs/decisions/off-verb-decision.md`, against the composed body's *"a plain finalize
commits NOTHING … holds (exit 4) … re-run the same finalize with `--approve` — approval is the
one destructive gate."* **Classification: latent defect** (12 workflows carry this step text).

### 4.3 A sub-task standing in the shared checkout at the base pin is told its milestone record is missing and routed to `jigc unmanage` it

Repro in §2.1. `jigc task validate beta-work` → exit 0,
`advisory · reconciliation.rename — tracked managed doc milestone-record:surf-probe … is
missing, but the path has no history — the checkout moved underneath the file-state cache, not
a deletion`, `route: prune the stale baseline: jigc unmanage docs/milestone-records/surf-probe.md`.
The record is not missing; the checkout predates it. Clean inside the provisioned worktree.
**Classification: latent defect.**

### 4.4 `jigc milestone add-task` and `jigc milestone add-from-spec` commit and say so on no surface

Repro in §2.8. Both move HEAD (`f3ec179`, `f148dbf`) with neither `--help` nor the ack naming a
commit, while the four sibling record-committing doors (`milestone create`, `task discard`,
`milestone discard`, `milestone finalize`) all name it in the ack.
**Classification: latent defect** (D-2 reaches the help half of `task discard` only).

### 4.5 `jigc validate --help` calls itself a *"read-only sweep"* and writes an invocation record

Repro in §2.4, with the log-off control. Same shape as CX-1, different door, not in any row.
**Classification: latent defect (prose precision), tied to CX-1's fix.**

### 4.6 The provisioned-but-wrong-checkout refusal is byte-identical to the never-provisioned one

Repro in §2.1 drives #1 and #3. Both print *"run `jigc milestone provision surf-probe`"* first;
in state (a′) that verb is a no-op and the actual recovery is the clause's tail (*"then re-run
this from that worktree"*). Honest as written (*"adds a worktree that is missing and leaves one
that exists untouched"*) but the leading route is inert in half the states that print it.
**Classification: shape-limited** — one refusal serving two states with different recoveries.

---

## §5 — honest bounds

* **Not driven to a landed outcome: 29 of the 34 composed workflows.** I ran every one of the
  34 through both doors and mechanically verified all 182 composed `jigc` forms against the
  real CLI, but only `single-task`, `sub-task`, `migrate-adr` (off-verb), `record-decision` and
  `milestone-execution` were followed to a commit or a declared hold.
* **8 of the 9 `DocAck` arms are in §2.7's class structurally, not by drive.** The only known
  producer of a non-empty write-ack finding is `commit-recording.stale-title` at
  `DocAck::Renamed`. Whether the other 8 can ever carry one is a source question I did not
  settle.
* **`jigc rename` and `jigc milestone discard` were not driven to a commit** in §2.8 — their
  help already states they commit, so the row was decided on the help text alone.
* **`migrate-corpus` was never driven over a corpus with an actual migration to do** — the
  `committed-singletons` rig reports `0 would migrate, 4 already current`, so CX-1's bound
  ("the only byte written is the log") is measured on the no-op path, not on a real migration's
  dry run.
* **The A6-1 boundary was driven once**, with one sub-task in the worktree and one in the shared
  checkout. I did not drive the symmetric cell (both sub-tasks in the shared checkout), nor the
  `finalize.fan-out.squash: false` arm, nor a worktree that was provisioned and never entered.
* **The `setup.dirty-install-path` non-firing on an uncommitted guide edit (§2.6) was observed,
  not investigated** — I did not establish whether the guide path is carved out of M51
  Increment 3's pre-write predicate deliberately or incidentally.
* **No project pack was manufactured.** Everything is the two embedded packs; a
  `--pack-from-dev` workflow could add a 35th member to §1.1's universe and was not tried.
* **Source reads are leads, not measurements.** `render::doc_ack`'s doc-comment (§1.6),
  `setup.rs:80` (§2.6) and `task.rs:935` (§2.7) are cited as the *home* of a behaviour I drove;
  nothing in §2 rests on a source read alone.
