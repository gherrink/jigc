<!-- M51 per-axis review — axis 6 · reconciled · driven on the installed `jigc 1.0.0-rc.15` (commit 35195f56), 2026-09-16 -->

# M51 per-axis review — AXIS 6 · composed surfaces — RECONCILED

**Inputs:** the Opus driver table (`axis-review/driver/axis6.md`, carried over below verbatim
except where a demotion is marked) and the Codex source pass (`axis-review/codex/axis6-codex.md`,
entered as leads and driven in the ledger).
**Binary:** `/Users/maurice/.local/bin/jigc` — asserted **`jigc 1.0.0-rc.15`** before anything else.
Every drive in the ledger below is my own, on that binary, in `dev/jigc-rig` throwaway repos
(`rig=$(dev/jigc-rig <state> --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"`).
**Rule applied** (acceptance-design.md → The reconciliation rule): a claim one pass makes that the
other cannot reproduce is a LEAD, not a finding. Every Codex claim below was driven to a repro or
to a falsifying datum; every driver defect was re-driven once by me before it kept its status.

---

## Part A — the driver table (carried over)


**Binary:** `/Users/maurice/.local/bin/jigc` — asserted **`jigc 1.0.0-rc.15`** before anything else
(RELEASE posture: the `#[cfg(debug_assertions)]` route-fence panics do not exist here). Every row
below ran on that binary, in throwaway `dev/jigc-rig` repos (`mktemp -d` roots, no teardown, no
`rm -rf` on a variable path). The table reflects the **fixed** binary — the audit's four fix commits
(`8a42fbbd` routes · `b5ccd818` orphan territory · `0fc80bab` setup guard · `dc508994` LOWs) are in it.

**The Codex source pass for this axis was not read** (per brief).

---

## 1. The door set, derived from the code

Counts read at HEAD, not taken from the design doc's prose:

| registry | file | count I read |
|---|---|---|
| `VERB_KINDS` | `crates/cli/src/cli.rs` | **47** leaves |
| `BEHALF_DOORS` | `crates/cli/src/cli.rs` | **47** rows (total classification) |
| `PATH_ARG_OCCURRENCES` | `crates/cli/src/cli.rs` | **14** rows |
| `DOCTYPE_DOORS` | `crates/cli/src/cli.rs` | **16** rows (**10** `DoctypeArg::Address`, 6 `Bare`) |
| `SLUG_DOORS` | `crates/cli/src/cli.rs` | **6** rows |
| `WORK_UNIT_ID_DOORS` | `crates/cli/src/cli.rs` | **25** rows |
| `COMMITTING_DOORS` | `crates/cli/src/invocation_log.rs` | **10** rows |
| `DESTROYING_DOORS` | `crates/cli/src/milestone.rs` | **4** (`[&DestroyingDoor; 4]`) |
| `ENVELOPE_ARMS` | `crates/cli/src/render.rs` | **60** rows |
| `STORE_EXIT_FLIPS` | `crates/cli/src/render.rs` | **6** rows |
| `ManifestKind::ALL` | `crates/cli/src/render.rs:1834` | **6** members |
| `SchemaChangeKind::ALL` | `crates/engine/src/schema_diff.rs` | **18** members |

**This axis's own door set is derived from the render seam, not from a door table.**
`render::composed` has **7** production call sites and `render::composed_preview` **1** — eight
render sites over **four clap leaves**, which is where the design's "four `render::composed`
producers" comes from:

| render site | dispatch fn | clap leaf |
|---|---|---|
| `cli.rs:1453` | `run_compose_named_no_intent` | `start` |
| `cli.rs:1479` | `run_compose` | `start` |
| `cli.rs:1504` | `run_compose_named` | `start` |
| `cli.rs:1552` | `run_resume` | `start` |
| `cli.rs:1577` | `run_reenter` | `workflow` |
| `cli.rs:1599` (`composed_preview`) | `run_preview` | `workflow` |
| `migrate.rs:81` | `dispatch_migrate` | `migrate` |
| `milestone.rs:3659` | `dispatch_execute` | `milestone execute` |

∪ `describe` ∪ the read verbs the read-back fence's owe-set names. The owe-set is
`pack.rs::assert_staged_read_back_stated` + `CONSTRAINT_REQUIRED_TOKENS[STAGED_READ_BACK_CODE] =
("jigc doc show", "--task")` (`pack.rs:1437`); its two read surfaces plus the previewed-gate read are
`doc show`, `doc list`, `task validate`.

**Door set = 8:** `start` · `workflow` · `migrate` · `milestone execute` · `describe` ·
`doc show` · `doc list` · `task validate`.

**Cell set = 5 (C1–C5):**
C1 the step text names a verb that answers · C2 the `resume:` line's three real states ·
C3 orientation's three states · C4 the catalog one-liner matches the verb's behaviour ·
C5 the off-catalog reason is stated.

8 × 5 = **40 cells · 25 applicable and driven · 15 n/a with reasons.**

---

## 2. The (door, cell) table

Route kind: **M**echanical / **H**uman / **I**nformational / **none**.

| # | door | cell | argv driven | exit | code | route | surface asserted | verdict |
|---|---|---|---|---|---|---|---|---|
| 1 | `start` | C1 | `start --workflow single-task "add a greeting helper"` → then each of the 13 verbs its text names, run for real | 0 (chain) | none | M (`Run:` ×4) | every `jigc …` line in the composed text; `task finalize` landed `7c93f83 — feat: add a greeting helper`, 3 files, one commit | **matches contract** |
| 2 | `start` | C1 | `start --workflow implement-from-spec "wire the parser"` → `task bind spec spec:padding wire-the-parser` → `start --task wire-the-parser` | 0/0/0 | none | M | the bound spec's `criteria` slice renders on re-compose with the `{#id}` anchors the text promises | **matches contract** |
| 3 | `start` | C1 | `start --workflow router` (the default front door) → `describe --workflows` | 0/0 | none | I + M | the router's closing sentence *"`jigc describe --workflows` lists every workflow … each of those carries the reason it is hidden"* — driven exhaustively (row 22) | **matches contract** |
| 4 | `start` | C1 | `start --workflow implement-from-spec "build the thing"` over a **zero-spec** corpus → `task bind spec spec:padding build-the-thing` | 0 then **1** | none | **none** | *"Pick the spec … from the committed specs below"* renders **two blank lines**; the `Run:` it then prints is unanswerable | **DEFECT A6-3** |
| 5 | `start` | C1 | `start --workflow milestone-execution` | 0 | none | M (×3, unresolved) | three `Run:` lines carrying a literal `<MILESTONE_ID>`; zero `Spawn:` lines; nothing on the surface says so | **DEFECT A6-2** |
| 6 | `start` | C2 | `start --workflow single-task …` then `start --task add-a-greeting-helper` | 0/0 | none | M | `resume: \`jigc start --task <id>\`` on the mint **and** the resume; `task minted:` header on the mint only | **matches contract** |
| 7 | `start` | C2 | `start --task alpha-work` (sub-task, HEAD moved off the pin) | 1 | none | M (`jigc milestone provision axis-six-probe`) | byte-identical to the `workflow` door's refusal (row 12); names the provisioning verb | **matches contract** |
| 8 | `start` | C2 | `start --task gamma-work` from a provisioned, never-entered sub-task worktree | 0 | none | M | `resume:` names `jigc workflow sub-task --task gamma-work` + the worktree path; no docs area was created by this door (correct — it is not the provisioning door, and the line says which is) | **matches contract** |
| 9 | `start` | C3 | `start` / `start --format json` in a `bare` repo | 0 | none | M (`jigc setup`) | text *"This project isn't set up."*; JSON `{"state":"unset-project","schema_version":3}` | **matches contract** |
| 10 | `start` | C3 | `start` / `start --format json` in a set-up repo, no task | 0 | none | M | catalog + `Run:`/`Preview:` block; JSON `state: "clean"`, no `tasks` key | **matches contract** |
| 11 | `start` | C3 | `start` / `start --format json` with one live task | 0 | advisory `file-state.staged-copy` ×2 (exit-0 by contract) | M (×4) | `Active task:` block (7 facts), four `Run:` directives; JSON `state: "active-task"`, `tasks: ["add-a-greeting-helper"]`, `schema_version: 3` | **matches contract** |
| 12 | `start` | C4 | `start` (orientation catalog) vs `start --workflow router` (router catalog) | 0/0 | none | M | the 12 one-liners are **byte-identical** between the two surfaces (`diff` clean) — one generator | **matches contract** |
| 13 | `start` | C4 | all 12 catalog one-liners cross-checked against the `create-gates:` each workflow composes | 0 ×12 | none | I | every decision-recording claim matches its gate set: `dev-task`/`quick-fix` *"records no decision"* → **no** `create-gates:` line; `single-task` *"ADRs and … changelog"* → `adr, changelog`; `decided-task` *"on the running decisions log"* → `decisions-log`; + 8 more | **DEMOTED as written** (the `argv driven` cell names no argv — not a repro) → **re-driven here, holds**: ledger D-13 |
| 14 | `start` | C5 | `start` (clean) | 0 | none | M ×2 | `Run: jigc start --workflow planning` and `… ingest-existing`, each with its gist — G6's contract is *"a route-prose line naming the verb, the same shape for each"*, not a reason | **matches contract** |
| 15 | `workflow` | C1 | `workflow <w> --preview` for all 12 catalog workflows; then `workflow router --preview` → follow its route `start --workflow router` | 0 ×12, then 1 → 0 | none | M | the refusal names the direct form and the direct form answers; `task list` confirms **no** preview minted a task | **matches contract** |
| 16 | `workflow` | C1 | `workflow sub-task --task alpha-work` from the worktree → the verbs its text names | 0 | none | M | provisions `docs/commit:alpha-work.md` + `provenance.json` on first entry, exactly as the `resume:` line claims; beta (unentered) has **no** `docs/` | **matches contract** |
| 17 | `workflow` | C1 | from the un-provisioned state: `doc set-field commit:gamma-work#type …` → follow its route | 1 → 0/0 | `write.not-present`-class refusal (flattened) | M ×2 | *"enter it with `jigc workflow sub-task --task gamma-work`, then list what task `gamma-work` stages with `jigc doc list --task gamma-work`"* — **both named verbs run**; this is the M51 Inc 9 repair of the baseline's empty-list dead end | **matches contract** |
| 18 | `workflow` | C2 | `workflow sub-task --task alpha-work`, HEAD moved off the pin | 1 | none | M | byte-identical to row 7 | **matches contract** |
| 19 | `workflow` | C2 | `workflow sub-task --task beta-work` from the **shared checkout** at `base == HEAD` vs from **its own worktree** — same task, both drives | 0/0 | none | M | `diff` of the two whole composed outputs: **BYTE-IDENTICAL** | **DEFECT A6-1** |
| 20 | `workflow` | C4 | `workflow <w> --preview` as the behaviour instrument for row 13 | 0 ×12 | none | I (banner) | the mint-first banner renders; `task: null` on the wire; `task list` unchanged | **matches contract** |
| 21 | `workflow` | C5 | `workflow {router, ingest-existing, milestone-execution} --preview` | 1 ×3 | none | I | *"workflow '<id>' mints no task, so there is nothing to preview — run it directly: …"* — the reason (`creates-task: false`) is stated at the refusal | **matches contract** |
| 22 | `describe` | C5 | `describe --workflows` + `--format json`, all 34 workflow entries partitioned against the orientation catalog | 0 | none | I | **22/22** off-catalog entries carry `router_hidden` **and** *"It is hidden from the router catalog: <reason>"*; **0/12** catalog entries carry it. Covers both kinds (`selectable: false` **and** `creates-task: false`) — the M46 router-sentence repair holds | **matches contract** |
| 23 | `describe` | C4 | `describe --commands` (31 entries) — 5 hints driven against behaviour | 0 | none | I | `milestone-provision` *"one detached base-pin worktree per sub-task"* → `git worktree list` shows `(detached HEAD)` at the pin `5ac9db5` · `milestone-join` *"commits nothing"* → HEAD byte-equal before/after · `validate-task` *"…carryover…without committing"* → `finalize.carried-staged` raised, nothing committed · `finalize-task` *"One task → one commit"* → one commit · `show-doc` *"with `--task`, the staged copy"* → staged bytes served | **matches contract** |
| 24 | `describe` | C1 | `describe --workflows` / `--commands` / `--format json`, named by the router step | 0 | none | I | exit 0, non-empty projection; `schema_version` + `origin_pack` present on the JSON arm | **matches contract** |
| 25 | `migrate` | C1 | `migrate legacy/old-decision.md --as adr` → `doc author adr`, `doc list adr --task`, `doc show adr:<slug> --task`, `task validate`, `task finalize` | 0/0/0/0/0/**4** | none; then the exit-4 hold | M | the walk's own claim — *"a plain finalize commits NOTHING … holds (exit 4) … re-run with `--approve`"* — driven: exit 4, `git log` unmoved, `git status` unchanged | **matches contract** |
| 26 | `migrate` | C2 | the same composed output's footer | 0 | none | M | top-level `resume: jigc start --task migrate-adr-legacy-old-decision-27a6965fe5b2`; `what's-left:`; `task scope:` | **matches contract** |
| 27 | `migrate` | C4/C5 | `describe`'s 11 `migrate-*` reasons vs the door | 0 | none | I | every one reads *"verb-routed — reached only through `jigc migrate <path> --as <T>`"*, and the `migrate-adr` walk in row 25 arrived from exactly that argv and no other | **matches contract** |
| 28 | `milestone execute` | C1 | `milestone execute axis-six-probe` → `milestone provision` → `workflow sub-task --task <each>` → `milestone join` → `milestone finalize` | 0/0/0/0/0 | none | M ×4 | the whole walk runs in order; finalize landed `06a61dc` with the join's docs + alpha's code | **matches contract** |
| 29 | `milestone execute` | C1 | `milestone execute second-probe` in the **partial**-provision state | 0 | advisory `milestone.worktrees-partial` | M (`jigc milestone provision second-probe`) | the advisory prints **before** the walk (law 3), names `delta-work` and the path, and says its `Spawn:` line *"would run in a working area that does not exist"* | **matches contract** |
| 30 | `milestone execute` | C1/C2 | `milestone execute second-probe --format json` | 0 | advisory (stderr) | M | stdout is exactly `{task, text}` with `task: null` (no `resume:`/`what's-left:` — correct: it mints nothing); the advisory is on **stderr** and `grep -c worktrees-partial` on stdout = **0** | **matches contract** |
| 31 | `milestone execute` | C4/C5 | `describe`'s `milestone-execution` reason — *"composes a degenerate walk — zero `Spawn:` lines and an unresolved `<MILESTONE_ID>` off-verb"* — driven at the off-verb door | 0 | none | I | driven **TRUE**: `grep -c '^Spawn:'` = 0, three `<MILESTONE_ID>` placeholders. The reason is honest; see DEFECT A6-2 for the surface that does not repeat it | **matches contract** (the reason) |
| 32 | `doc show` | C1 | `doc show adr:<slug> --task <id>` / `doc show changelog:changelog --task <id>` / `doc show commit:<id> --task <id>` — each run from the read-back line that named it, in 3 workflows | 0 ×5 | none | none (success) | the staged bytes just written, incl. the `{#id}` anchors and the `<!-- fields -->` block | **matches contract** |
| 33 | `doc show` | C4 | the `show-doc` catalog hint | 0 | none | I | *"…with `--task`, the staged copy of your own in-flight write"* — driven against a doc with **no** committed copy | **matches contract** |
| 34 | `doc list` | C1 | `doc list adr --task <id>` / `doc list --task <id>` / `doc list spec` — each from the composed index line | 0 ×4 | none | I | `id · path · state` rows in `managed` state; over a zero-`spec` store it prints **`jigc doc list — no committed \`spec\` docs`** plus a `note:` naming the open task that stages docs and the `--task` form (law 2 — the very sentence DEFECT A6-3's composed surface does not use) | **matches contract** |
| 35 | `task validate` | C1 | `task validate <id>` from the `what's-left:` line, in 4 tasks | 0 / 3 | `schema-conformance.*`, `finalize.carried-staged`, `file-state.staged-copy` | M on every finding | exit 3 iff blocking; nothing committed | **matches contract** |
| 36 | `task validate` | C4 | the generated coverage clause vs `task finalize --help` | 0 | none | I | the span *"previews part of the finalize gate: this task's content findings, the carryover gate, the owner-artifact causes that need no staging, and the granted-but-unused changelog gate; the staged set, promotion and the commit surface at finalize"* is **byte-identical** in the composed footer and in `task finalize --help` — one generator (`whats_left_coverage()`), two surfaces | **matches contract** |
| 37 | `task validate` | C4 | the clause's carryover claim, driven | 3 | `finalize.carried-staged` | M (`git restore --staged -- carry.txt`, + `--carry-staged`) | the gate it claims to preview is actually raised | **matches contract** |

*(Rows 1–37 cover the 25 applicable (door, cell) pairs; several pairs took more than one drive and are
listed once per drive where the argv differs by more than the door.)*

### n/a rows (15), each with its reason

| door | cell | reason |
|---|---|---|
| `describe`, `doc show`, `doc list`, `task validate` | **C2** | none of the four renders `render::task_state_lines` — they carry no `resume:` line to be in a state |
| `workflow`, `migrate`, `milestone execute`, `describe`, `doc show`, `doc list`, `task validate` | **C3** | orientation is `run_orient`, reached only by bare `jigc start`; no other leaf produces an `OrientationView` |
| `doc list` | **C4** | the 31-entry command catalog ships **no** `list-docs` ref, so `doc list` is the door of no catalog one-liner (its behaviour claims live in `--help`, which is axis 8) |
| `doc show`, `doc list`, `task validate` | **C5** | off-catalog-ness is a property of a **workflow**, not of a verb; these three are verbs and are on no catalog to be off |

---

## 3. Repro blocks

Rig construction is identical for every block below and is not repeated:
`rig=$(dev/jigc-rig <state> --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"`.

### DEFECT A6-1 — a sub-task's composed surface is byte-identical inside and outside its worktree, and the milestone boundary then drops its staged code at exit 0

**Contract contradicted.** `implementation/roadmap.md` → Milestone 51, Increment 9, *Proves*:
> *"the sub-task `resume:` line **says which of its three states the repo is in**"*

and the Grouped scope's naming of those states — *"pre-provision refusal · post-provision
compose-without-provisioning · `base == HEAD` silent compose"*.

Driven, the surface separates **(a)** from **{(b), (c)}** and stops. States (b) and (c) render the
same bytes, and they are the two states whose consequences differ.

```
# setup — rig: fresh
jigc milestone create "Axis six probe"          # base pin 6fb231a, record commit ba847ee
jigc milestone add-task axis-six-probe --workflow sub-task "alpha work"
jigc milestone add-task axis-six-probe --workflow sub-task "beta work"
jigc milestone provision axis-six-probe         # -> provisioned 2 worktree(s) at base 6fb231a
git checkout --detach 6fb231a                   # shared checkout now AT the pin  => state (c)

# state (c) — the SHARED checkout
jigc workflow sub-task --task beta-work  > /tmp/c.txt     # exit 0
# state (b) — the sub-task's OWN worktree, same task
cd .jigc/worktrees/beta-work
jigc workflow sub-task --task beta-work  > /tmp/b.txt     # exit 0

diff /tmp/c.txt /tmp/b.txt
#   (no output) — BYTE-IDENTICAL
```

Both print, identically:

```
resume: `jigc workflow sub-task --task beta-work`   — re-composes this workflow and provisions
  this sub-task's write-ready docs area on first entry; run it from this sub-task's own worktree
  at `.jigc/worktrees/beta-work`, where its work happens
```

…and the composed body says *"`git add` your code edits so the milestone can fold **this
worktree's** staged index"* while the reader in state (c) is not in a worktree at all. Nothing on
the surface — no advisory, no differing clause — distinguishes the two. `grep -c advisory` on the
state-(c) output = **0**; `partial_worktree_advisories` (`milestone.rs:3060`) returns empty by
design for the all-provisioned state, so no sibling surface covers it either.

**The consequence, driven end to end** (not inferred from the fold's documentation):

```
# in the SHARED checkout, do exactly what the composed body invites
printf 'beta work output\n' > beta-code.txt && git add beta-code.txt
git diff --cached --name-only                                  # beta-code.txt
git -C .jigc/worktrees/beta-work diff --cached --name-only      # (empty)

# ... alpha does the same inside ITS worktree; both sub-tasks author their commit docs
jigc milestone join axis-six-probe        # exit 0 — 2 doc(s) merged
jigc milestone finalize axis-six-probe    # exit 0
> finalized 06a61dc — Finalize milestone axis-six-probe (2 sub-tasks)
>   added alpha-code.txt
>   modified docs/milestone-records/axis-six-probe.md
>   2 files committed
>   sub-tasks: alpha-work: 1 doc, 1 code file · beta-work: 1 doc

git ls-tree -r --name-only HEAD | grep -c beta-code.txt   # 0
git status --short                                        # A  beta-code.txt
```

`beta-code.txt` is a sub-task's work, staged by the agent the composed text told to stage it, and
the milestone boundary committed at **exit 0** without it and without naming it. The ack's
`beta-work: 1 doc` is the only report, and it is silent about the file. (`task finalize` carries a
`left-out` manifest by contract — `design/finalize.md`:169/171 — the milestone boundary has no
equivalent; `grep left_out crates/cli/src/milestone.rs` finds only a comment.)

**Severity note.** The *cause* is surface-tier (a render layer that does not discriminate); the
*effect* is silent, exit-0 loss of a sub-task's staged work from its only commit boundary. The loss
is recoverable — the bytes stay staged in the shared checkout — but nothing tells the operator to
look.

### DEFECT A6-2 — `jigc start --workflow milestone-execution` composes three unrunnable `Run:` lines at exit 0, and jigc's own preview refusal routes there

**Contract contradicted.** `design/surface-contract.md` → **Law 3 — nothing ambushes**:
> *"Every constraint is stated where it binds, before it can fail."*

The constraint — *this form composes a degenerate walk; the real door is `jigc milestone execute
<id>`* — is stated at `describe` and **nowhere at the door that composes it**.

```
# rig: fresh (any set-up repo)
jigc workflow milestone-execution --preview
> workflow 'milestone-execution' mints no task, so there is nothing to preview —
>   run it directly: jigc start --workflow milestone-execution          # exit 1

jigc start --workflow milestone-execution                                # exit 0
> Run: `jigc milestone provision <MILESTONE_ID>`
> Spawn a sub-agent per sub-task (one per `Spawn:` line below) and implement it.
>   ... (zero Spawn: lines follow)
> Run: `jigc milestone join <MILESTONE_ID>`
> Run: `jigc milestone finalize <MILESTONE_ID>`

grep -c '^Spawn:' -> 0
grep -in 'degenerate|hidden|advisory' -> only the three <MILESTONE_ID> lines
```

The project already knows and says so — one surface over:

```
jigc describe --workflows
> milestone-execution is ... It is hidden from the router catalog: composes a degenerate walk —
>   zero `Spawn:` lines and an unresolved `<MILESTONE_ID>` off-verb, so a router pick could
>   never land.
```

…and jigc's own `composed_preview` mold demonstrates exactly the banner this door is missing
(*"Below, `--task your-task-id` marks where the minted id goes."*). The route that lands the reader
here is jigc's own (`--preview`'s refusal). `describe`'s reason is **honest** — the class is
statement *placement*, not a lie.

### DEFECT A6-3 — `implement-from-spec` over a corpus with no committed spec claims a list that is not there, then names an unanswerable step

**Contract contradicted.** `design/surface-contract.md` → **Law 1 — nothing lies**
(*"Every claim a surface makes is generated from the thing it describes, or asserted against it"*)
and **Law 2 — nothing hides** (the designated recovery for the state is named by the surface that
produces it). jigc's own `doc list` already ships the sentence this surface needs.

```
# rig: fresh  (zero committed specs — confirmed: `jigc doc list spec` says so)
jigc doc list spec
> jigc doc list — no committed `spec` docs
> note: docs are also staged in open task gamma-work — this listing is the committed store; ...

jigc start --workflow implement-from-spec "build the thing"              # exit 0
> task minted: build-the-thing
> ...
> Pick the spec this work implements from the committed specs below and bind it by
> its FULL address exactly as listed — `spec:<slug>`, the `spec:` prefix included
> (a bare slug is rejected as a malformed address) — then re-run to read its
> criteria:
>                       <-- line 10: blank
>                       <-- line 11: blank
>                       <-- line 12: blank
> Run: `jigc task bind spec <SPEC_ADDRESS> build-the-thing`
> Run: `jigc start --task build-the-thing`
```

There is no `<SPEC_ADDRESS>` to supply, and the surface neither says so nor names the recovery
(author one — `jigc start --workflow plan "<intent>"` — or pick a different workflow). Following the
step dead-ends with no code and no route:

```
jigc task bind spec spec:padding build-the-thing     # exit 1
> no such doc `spec:padding`
jigc task bind spec spec:padding build-the-thing --format json
> { "error": "no such doc `spec:padding`" }          # exit 1
```

`jigc workflow implement-from-spec --preview` renders the identical empty block, so the reader
cannot discover the precondition by previewing first either. The step's *other* stated constraint
is true — the bare-slug reject is real (`malformed address 'padding': missing ':' between type and
slug`, exit 1) — so law 3 holds here and law 1/2 is what fails.

---

## 4. Observations and leads (driven, but **not** defects — no stated contract is contradicted)

1. **`jigc migrate` mints a task and is silent about the work already open.** `write-commands.md`:185
   says the `also open:` block *"rides only these two forms"* (`jigc start "<intent>"` and
   `jigc start --workflow <X> "<intent>"`), and driven it does exactly that — so the binary matches
   the record. But `migrate` is a **third** work-starting door that changes the repo (it stages
   foreign source), which is the shape M50's F-5 rationale names. Control drive, same repo, two
   tasks already live: `jigc migrate legacy/old-decision.md --as adr` → **no** `also open:` block;
   `jigc start --workflow quick-fix "tiny fix"` → `also open: 2 other tasks were already open …`
   with both rows. **Lead for the source pass / a later wave, not a finding.**

2. **`jigc task bind`'s two refusals carry no finding code and no route** (`malformed address …`,
   `no such doc …`; flattened to `{"error": …}` under `--format json`, exit 1). They are bare
   `anyhow` bails (`task.rs:2707`), and the route floor as `surface-contract.md` states it binds
   *blocking findings*, so this is outside it. Recorded because it is the last inch of DEFECT A6-3's
   dead end.

3. **`describe --commands --format json` carries `{id, pack, hint}` and never the argv the ref
   resolves to**, while the composed text does. `introspection.md`:100 declares the command-ref
   surface pack-only and non-contractual, so this is by design; noted only so the source pass does
   not re-derive it.

4. **`milestone execute`'s provisioning advisory is deliberately silent in the none-provisioned
   state** (`milestone.rs:3076`: *"The two settled states — all provisioned, none provisioned … 
   Nothing to say in any of them"*). Driven and sound: the walk's own step 1 is
   `Run: jigc milestone provision <id>`, so the none-provisioned state is already answered by the
   text. Recorded so the empty result is not mistaken for a hole.

---

## 5. What I did **not** drive, stated plainly

- **C1 end-to-end for 8 of the 12 catalog workflows.** I drove the full named-verb chain to a real
  outcome for `single-task`, `implement-from-spec`, `migrate-adr`, `sub-task` and
  `milestone-execution`. For `architecture-documentation`, `decided-task`, `dev-task`,
  `do-research`, `form-vision`, `park-idea`, `plan`, `project-setup`, `quick-fix` and
  `record-decision` I composed each (via `--preview`) and **mechanically** checked every
  `jigc <token>` in the text against `VERB_KINDS` — **18 distinct leaves named across all composed
  surfaces, every one a real leaf**; the only two non-leaf hits are the prose fragments
  `jigc doc` (in *"`jigc doc` writes default to the single active task"*) and *"jigc itself
  accepts"*. Their chains were **not** run to a commit.
- **C4 at `describe` for the 34 `description:`/`usage:` narrations.** I cross-checked all 12 catalog
  `when:` one-liners exhaustively on the decision-recording axis (row 13), all 22 off-catalog
  reasons for presence and partition (row 22), and 5 of 31 command-catalog hints against behaviour
  (row 23). The remaining 26 command hints and the 34 narration bodies were **not** driven.
- **C2 state (a) at `migrate` and `milestone execute`.** n/a by construction: `migrate` mints a
  top-level task (never a sub-task, so no base-pin refusal arm), and `milestone execute` renders no
  `resume:` line at all (`task: null`).
- **`doc list`'s `orphaned` third state and `doc schema`'s projection.** Axis 7 and axis 9 cells;
  not driven here.
- **The `GIT_DIR` redirect and every posture cell.** Axis 2.
- **The Codex source pass for axis 6** — deliberately not read (per brief); reconciliation is a
  separate agent.

---

## 6. What this adds over flow-52 arm 5

**Arm 5 is the arm that already pins part of this axis** (`design/worked-examples.md` → *"Arm 5 · the
pinned envelopes the wave moved. (a CODE-SIDE REGISTRY, production-side: `ENVELOPE_ARMS`)"*, 60 arms
over all 47 leaves). It reaches every one of this axis's eight doors, because proof (1) drives every
leaf to a success. What it proves about them is **the wire**: that each composed producer's
`--format json` carries exactly its declared key set, that the four pre-pin keys are gone, and —
decisively for this axis — that **7 arms are declared `Unpinned(<reason>)`, and those 7 are the
composed-prose ones.** Arm 5's contribution to axis 6 is therefore a *negative*: it pins that the
composed prose is not pinned.

This review drives the half arm 5 declares out of scope, and it does so in three ways arm 5
structurally cannot:

1. **It follows the prose instead of reading it.** Arm 5 asserts keys on an envelope. Rows 1, 2, 16,
   17, 25 and 28 take the `Run:`/`jigc …` lines a composed surface prints and **execute them in the
   state that surface put the reader in**, to a landed commit (`7c93f83`, `06a61dc`) or to the
   declared hold (exit 4). That is the only way to find DEFECT A6-3, whose envelope is a perfectly
   valid `{task, text}` pair whose *text* names a step nothing can answer.

2. **It crosses two surfaces that arm 5 measures separately.** Rows 12, 13, 22, 23 and 36 assert
   *agreement*: orientation's catalog is byte-identical to the router's; every one of the 12
   one-liners' decision claim equals the `create-gates:` its workflow composes; all 22 off-catalog
   reasons exist and no catalog entry has one; the `what's-left` span is byte-identical in the
   composed footer and `task finalize --help`. An envelope registry keyed per leaf cannot see a
   divergence between two leaves.

3. **It varies the repository state under a fixed argv.** Arm 5 drives each leaf once, to a success.
   DEFECT A6-1 is invisible to any single drive: it is `diff` of the **same** argv's output in two
   different repository postures, and the finding is that the diff is empty. Arm 6
   (`WORK_UNIT_ID_DOORS`, the unknown-id cell) varies the *token*; this axis varies the *world*.

The three defects are all in the half arm 5 declares unpinned, and none of them moves a key, which
is why the flow-52 suite is green over all three.

---

# Part B — Reconciliation ledger

The Codex pass opens with **"No grounded completeness defects found."** It is therefore silent on
all three driver defects; under the rule that silence is not a refutation, so each defect was
**re-driven by me** and kept only on my own repro. Codex's six *positive* ("consistent findings")
claims are entered as leads `lead(codex, …)` and driven below — a source-read agreement is not a
measurement either.

## B1 · Codex claims

### lead(codex, C1) — "all composed producers use the shared rendering seam; the registry carries start, workflow, migrate and milestone execute"
**CONFIRMED (driven).** The source side is as cited (8 render sites over 4 leaves):

```
grep -rn 'render::composed\b\|render::composed_preview' crates/cli/src/*.rs | grep -v render.rs
#  cli.rs:1453 1479 1504 1552 1577   ·  cli.rs:1599 (composed_preview)
#  migrate.rs:81   ·   milestone.rs:3659
grep -c 'EnvelopeArm {' crates/cli/src/render.rs   # 61 = 60 rows + the struct decl -> ENVELOPE_ARMS = 60
```

and the wire agrees at every one of the four leaves (rig: `committed-singletons`):

```
jigc start --workflow single-task "probe intent" --format json   -> keys ['task','text']   exit 0
jigc workflow single-task --task probe-intent   --format json    -> keys ['task','text']   exit 0
jigc workflow quick-fix --preview               --format json    -> keys ['task','text']   task=null
jigc migrate legacy/old.md --as adr             --format json    -> keys ['task','text']   task=migrate-adr-legacy-old-35249d478bd2
```
(`milestone execute --format json` is the driver's row 30, same shape.)

**Bound found while driving, recorded because it qualifies the claim:** the shared seam is shared
for the *machine* arm only. `render::composed`'s `Format::Json` branch is `json(&view.view)` with
the comment *"Exactly the pinned `{task, text}` projection — the presentation lines below reach no
tooling consumer"*, so **`resume:` / `what's-left:` / `task scope:` / `create-gates:` / `also open:`
exist on the text arm and on no JSON arm**:

```
jigc start --task probe-intent            | grep -n '^resume:'   -> line 205 (present)
jigc start --task probe-intent --format json  -> "'resume:' in text: False"  (also what's-left, task scope)
```
Declared at the seam, so not a defect — but it means every C2/C4/C5 guarantee below is a guarantee
about the **text** arm.

### lead(codex, C2) — "the three resume states are expressible: id-less omits the line; top-level routes through `jigc start --task`; sub-tasks route through `jigc workflow <W> --task` from their worktree; the discriminator is the same `owning_milestone` predicate as the lifecycle guard"
**CONFIRMED (driven), and it does NOT refute DEFECT A6-1 — different triple.**

```
# id-less
jigc start --workflow router        -> exit 0 ; grep -c '^resume:' = 0
jigc workflow quick-fix --preview   -> exit 0 ; grep -c '^resume:' = 0
# top-level
jigc start --workflow single-task "probe intent" ; jigc start --task probe-intent
  resume: `jigc start --task probe-intent`   — re-composes this workflow if context is lost
  (and the named door runs: exit 0)
# sub-task (milestone rig)
jigc workflow sub-task --task beta-work
  resume: `jigc workflow sub-task --task beta-work`   — … run it from this sub-task's own worktree
  at `.jigc/worktrees/beta-work`, where its work happens
```
`engine::milestone::owning_milestone` is indeed the one predicate (`start.rs:2425`, and the same
call at `orient.rs:167`, `task.rs:1951`, `repo.rs:338`).

**The reconciliation point:** Codex's triple is a **unit-kind** partition (id-less / top-level /
sub-task) and `owning_milestone` is a **membership** test. The roadmap's triple, which Increment 9
claims the line *"says which of its three states the repo is in"*, is a **posture** partition
(pre-provision refusal · post-provision compose-without-provisioning · `base == HEAD` silent
compose). A membership test cannot discriminate posture, which is exactly A6-1's mechanism. C2 is
true and A6-1 is true; they are claims about different partitions.

### lead(codex, C3) — "orientation covers exactly unset, clean and active-task; an unavailable sweep becomes `findings: unknown — <reason>`, not `none`"
**CONFIRMED (driven), including the unknown arm — which the driver did not reach.**

```
# rig: bare / fresh / fresh+task  (driver rows 9-11, re-confirmed on the JSON arm)
jigc start --format json   -> state=unset-project | clean | active-task     (ORIENTATION_ARMS = 3)

# the unknown arm, induced (rig: committed-singletons, one live task):
chmod 000 "$REPO/.jigc/tasks/probe-intent/docs"
jigc start                                   # exit 0
>   findings: unknown — enumerating the task's code-anchor surface at "…/.jigc/tasks/probe-intent":
>            Permission denied (os error 13)
chmod 755 "$REPO/.jigc/tasks/probe-intent/docs"
```
The door does not fail (exit 0), the task is still named, and *unknown* is not rendered as *none* —
`render.rs::findings_summary` as cited. A corruption of the staged doc's bytes does **not** reach
this arm (the sweep still runs and reports findings), so the reason string is I/O-shaped by
construction.

*Observation carried, not a defect:* the reason prints an **absolute host path**. Its producer is
`crates/cli/src/task.rs:1508`, and `crates/cli/tests/repo_relative_paths.rs`'s `UNSWEPT_PRODUCERS`
already counts `crates/cli/src/task.rs` at 3 with the stated reason *"two `with_context`
promote/probe faults and one `git archive --prefix=`"* — so this is a **declared, counted** member
of M50's remainder reached through an axis-6 door, not a new finding.

### lead(codex, C4) — "`describe`, `doc show`, `doc list` and `task validate` all have registry rows"
**CONFIRMED (driven).** Each door's `--format json` carries exactly its declared key set:

```
jigc describe       --format json -> ['commands','definitions','schema_version']        exit 0
jigc doc show vision --format json -> ['fields','item-count','schema-version','sections','slug','type'] exit 0
jigc doc list       --format json -> ['docs']                                            exit 0
jigc task validate probe-intent --format json -> ['findings','schema_version']            exit 3
jigc start          --format json -> ['header','next_steps','schema_version','state','tasks','workflows']
```

### lead(codex, C5) — "every executable `jigc …` form found in both step trees has a corresponding clap verb and named flags"
**CONFIRMED (driven), on a wider subject than Codex's read.** Codex read the step trees; a step tree
holds `{{cli.<id>}}` command-refs whose argv lives in the **command catalogs**, so I extracted from
both (69 step YAMLs + `crates/cli/pack/config/commands.yaml` + `packs/methodology/config/commands.yaml`)
and drove every distinct form against clap:

```
# 24 distinct leaf forms + their named long flags, each: jigc <leaf> --help
OK config set · describe [--workflows] · doc add-item [--task --title] · doc author [--from-file --task]
OK doc create [--task --title] · doc list [--task] · doc schema · doc set-field [--task --value]
OK doc set-slot [--from-file --task] · doc show [--task] · ingest · migrate [--as]
OK milestone {add-from-spec, add-task [--workflow], create, execute, finalize, join, provision}
OK start [--task --workflow] · task bind · task discard [--force] · task finalize · task validate
fail=0            # every leaf exits 0 on --help; every named long flag appears in that leaf's help
```
The only non-leaf hits are five prose fragments — *"jigc can manage"*, *"jigc checks only"*,
*"jigc has adopted"*, *"jigc itself accepts"*, *"jigc never auto-migrates"* — the same class as the
driver's two.

### lead(codex, C6) — "catalog membership and narration share front matter: selectable entries are mechanically required to carry non-empty `when`/`description`/`usage`; every off-catalog workflow must carry `suppressed: {reason, expires}`"
**CONFIRMED (driven) on all three legs — and with its scope measured, which Codex's read did not state.**
Each probe installs one manufactured workflow into a pack copy that **keeps its manifest**
(`dev/jigc-rig fresh --workflow <id> <f> --repin`):

```
# (a) creates-task: false, no suppressed:
pack-load suppression fence failed: workflow `probe-nosupp` declares `creates-task: false` — …
  route: add `suppressed: {reason: …, expires: never | …}` …                       rig exit 1
# (b) selectable: false, no suppressed:
pack-load suppression fence failed: workflow `probe-nosel` declares `selectable: false` — …   rig exit 1
# (c) selectable work-workflow with when: ""
pack-load catalog shape fence failed: selectable work-workflow `probe-nowhen` carries no `when:` — …  rig exit 1
```
Both *causes* fire, so M46's router-sentence repair is fenced, not just authored.

**The scope, driven:** the fence is keyed on manifest-shipping constituents. Re-run (a) with the
manifest dropped (`--pack-from-dev`, no `--repin`) and the same workflow loads clean and is narrated
by `describe` **with no reason at all**:

```
jigc describe --workflows            # exit 0
> probe-nosupp is A probe workflow that mints no task and declares no suppressed block. …
                                     # no "It is hidden from the router catalog: …" clause
jigc describe --workflows --format json -> {"id":"probe-nosupp","router_hidden":null,"origin_pack":"dev"}
jigc start | grep -c probe-nosupp    # 0  — off the catalog, as the fence would have required a reason for
```
This is **declared, not a hole**: `design/surface-contract.md`:127 states *"a manifest-less pack stays
on skip-on-absent"* and `design/introspection.md`:90 owns the rule. Recorded as an **OPEN LEAD for
the wave, not a finding**: axis 6's cell C5 ("the off-catalog reason is stated") is a guarantee about
manifest-shipping packs only; a project-local pack can put an unexplained absence on `describe`.

### lead(codex, C0) — "No grounded completeness defects found."
**REFUTED.** Falsifying data: the three driver defects, each re-driven by me below (A6-1, A6-2,
A6-3). A source pass that reads the seam cannot see any of them — A6-2 and A6-3 are true statements
*missing* from a surface, and A6-1 is an emptiness (`diff` of one argv's output across two repo
postures) that no single read can produce.

## B2 · driver defects — status after my own re-drive

### DEFECT A6-1 — **CONFIRMED**, and the data-loss half confirmed end to end (repro corrected in one step)
```
rig=$(dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
jigc milestone create "Axis six probe"            # base pin 65c7c4d, record commit 22c8cc9
jigc milestone add-task axis-six-probe --workflow sub-task "alpha work"
jigc milestone add-task axis-six-probe --workflow sub-task "beta work"
jigc milestone provision axis-six-probe           # provisioned 2 worktree(s) at base 65c7c4d

(cd .jigc/worktrees/beta-work && jigc workflow sub-task --task beta-work > /tmp/b.txt)   # exit 0
git checkout --detach 65c7c4d                     # shared checkout AT the pin => state (c)
jigc workflow sub-task --task beta-work > /tmp/c.txt                                     # exit 0
diff /tmp/b.txt /tmp/c.txt        # NO OUTPUT — BYTE-IDENTICAL
grep -ci advisory /tmp/c.txt      # 0
```
The consequence, driven to the commit:
```
printf 'beta work output\n' > beta-code.txt && git add beta-code.txt     # exactly what line 64 invites
git diff --cached --name-only                                  # beta-code.txt
git -C .jigc/worktrees/beta-work diff --cached --name-only     # (empty)
# both sub-tasks author their commit docs through jigc (the write half works identically from the
# shared checkout — `.jigc/` is shared, so `docs/commit:beta-work.md` was provisioned there)
jigc milestone join axis-six-probe        # exit 0 — 2 doc(s) merged
git switch main                           # <-- STEP THE DRIVER'S BLOCK OMITS (see below)
jigc milestone finalize axis-six-probe    # exit 0
> finalized 3542e95 — Finalize milestone axis-six-probe (2 sub-tasks)
>   added alpha-code.txt
>   modified docs/milestone-records/axis-six-probe.md
>   2 files committed
>   sub-tasks: alpha-work: 1 doc, 1 code file · beta-work: 1 doc
git ls-tree -r --name-only HEAD | grep -c beta-code.txt   # 0
git status --short                                        # A  beta-code.txt
```
**Correction to the driver's repro block (not to the defect):** with HEAD still detached at the pin,
`jigc milestone finalize` does **not** reach exit 0 — it blocks `blocking · repo.head-detached`
(exit 1). The exit-0 silent drop needs the re-attach shown above. The defect stands: exit 0, `2
files committed`, `beta-work: 1 doc` as the only report of a sub-task whose staged code is absent,
and `grep left_out crates/cli/src/milestone.rs` finds **one hit, a comment** (`milestone.rs:3647`).

**Severity bound I add, driven:** state (c) is reachable but not the default posture — after
`milestone create` + `add-task` the shared checkout is *ahead* of the pin, and there the same argv
**refuses correctly**, naming the state and the door:
```
git switch main ; jigc workflow sub-task --task beta-work      # exit 1
> task `beta-work` is pinned to base 65c7c4d but you're on e82b3d9 — … run
>   `jigc milestone provision axis-six-probe` … then re-run this from that worktree
```
So the un-discriminated cell is specifically *attached-or-detached at the pin*, reached by a
`git checkout --detach <pin>` / reset, not by the ordinary flow.

### DEFECT A6-2 — **CONFIRMED** (re-driven verbatim)
```
rig=$(dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
jigc workflow milestone-execution --preview          # exit 1
> workflow 'milestone-execution' mints no task, so there is nothing to preview —
>   run it directly: jigc start --workflow milestone-execution
jigc start --workflow milestone-execution > me.txt   # exit 0
grep -c '^Spawn:' me.txt                             # 0
grep -n 'MILESTONE_ID' me.txt
> 10:Run: `jigc milestone provision <MILESTONE_ID>`
> 20:Run: `jigc milestone join <MILESTONE_ID>`
> 29:Run: `jigc milestone finalize <MILESTONE_ID>`
grep -inE 'degenerate|hidden|advisory|milestone execute' me.txt    # NO MATCHES
```
The last line is the sharper form of the driver's claim: the composed surface names neither the
constraint **nor the real door** (`jigc milestone execute <id>`) anywhere in its 30 lines, while
`describe --workflows` states both. Statement placement, exactly as the driver framed it.

### DEFECT A6-3 — **CONFIRMED** (re-driven verbatim, plus the `--preview` arm)
```
rig=$(dev/jigc-rig fresh --binary /Users/maurice/.local/bin/jigc) || exit; eval "$rig"
jigc doc list spec                       # exit 0 — "jigc doc list — no committed `spec` docs"
jigc start --workflow implement-from-spec "build the thing"    # exit 0
>  6 Pick the spec this work implements from the committed specs below and bind it by
>  … (lines 10-12 blank)
> 13 Run: `jigc task bind spec <SPEC_ADDRESS> build-the-thing`
> 14 Run: `jigc start --task build-the-thing`
jigc task bind spec spec:padding build-the-thing                # exit 1 — no such doc `spec:padding`
jigc task bind spec spec:padding build-the-thing --format json  # exit 1 — {"error":"no such doc `spec:padding`"}
jigc task bind spec padding build-the-thing                     # exit 1 — malformed address `padding`…
jigc workflow implement-from-spec --preview                     # exit 0 — SAME empty block
jigc doc list spec        # with the task now open:
> note: docs are also staged in open task build-the-thing — this listing is the committed store; …
```
The contrast in the driver's framing holds verbatim: `doc list` ships the law-2 sentence for the
empty set that the composed surface, over the same empty set, does not.

## B3 · driver table — demotions and amendments

- **Row 13 — DEMOTED as written, then re-driven and restored (D-13).** Its `argv driven` cell names
  no argv, only a cross-check, so as written it is not a repro. Re-driven: all 12 catalog workflows
  minted (`jigc start --workflow <w> "probe <w>"`), each composed `create-gates:` line read back
  against its one-liner. It **holds** — `dev-task` / `quick-fix` say *"recording no decision"* and
  render **no** `create-gates:` line; `decided-task` → `decisions-log`; `single-task` *"ADRs and …
  changelog"* → `adr, changelog`; `record-decision` / `implement-from-spec` → `adr`; `plan` → `spec`;
  `project-setup` → `prd`; `architecture-documentation` → `arch-doc`; `do-research` → `research`;
  `form-vision` → `vision`; `park-idea` → `idea`. 12/12.
- **Row 12 — amendment to the wording, verdict unchanged.** The two catalogs are *not* literally
  byte-identical: orientation renders `  - <id> — …` (two-space list indent), the router renders
  `- <id> — …`. Identical after stripping that indent (`diff` clean, 12/12). "One generator" holds;
  "byte-identical / diff clean" as written does not.
- **Row 36 — amendment to the wording, verdict unchanged.** The span is identical
  **whitespace-normalised** — `jigc task finalize --help` re-wraps it to the help column, so a raw
  byte compare of the two renderings fails while the sentence is the same generator's
  (`whats_left_coverage()`).
- **Rows 15, 21, 22 — re-driven by me, hold exactly.** 12/12 previews exit 0 and mint nothing
  (`task list` line count unchanged, 16 → 16); the three `creates-task: false` previews refuse at
  **exit 1** measured bare, each naming the direct form, and both `router` / `ingest-existing` direct
  forms then run at exit 0; `describe --workflows --format json` partitions **34 = 22 + 12**, all 22
  `router_hidden` entries carry *"It is hidden from the router catalog: …"*, **0** catalog entries
  carry `router_hidden`, and the non-hidden set is **equal** to the orientation catalog set.
- **Registry counts in §1 spot-checked:** `VERB_KINDS` = 47, `ENVELOPE_ARMS` = 60 (the raw
  `grep -c 'EnvelopeArm {'` = 61 counts the struct declaration), 8 composed render sites over 4
  leaves. All as the driver read them.
- No other row is demoted: each remaining row's `argv driven` cell names a runnable argv and its
  `surface asserted` cell a measurable predicate, which is the compact repro form for a mechanical
  row.

## B4 · open leads (driven as far as the state allows, promoted by nothing)

1. **C5 is a manifest-scoped guarantee.** A manifest-less project-local pack can ship an off-catalog
   workflow that `describe` narrates with `router_hidden: null` and no reason (driven above, under
   lead C6). Declared at `design/surface-contract.md`:127 / `introspection.md`:90 — so a wave
   decision, not a defect.
2. **The JSON arm of every composed door carries no task-state lines** (`resume:`, `what's-left:`,
   `task scope:`, `create-gates:`, `also open:`). Declared at the seam (`render::composed`'s
   `Format::Json` comment). A driver reading only the envelope is told nothing about how to resume —
   recorded because this axis's C2/C4/C5 cells are, in consequence, text-arm-only guarantees.
3. **The driver's own leads 1-4 (§4) stand unchanged** — I re-read them against the code and drove
   none further: `migrate` mints silently (matches `write-commands.md`:185), `task bind`'s two
   refusals carry no code/route (bare `anyhow`, outside the route floor as stated), the
   `describe --commands` JSON carries no argv (declared non-contractual), and the none-provisioned
   advisory silence is by design (`milestone.rs`:3076).

---

# Part C — doors covered

Every clap leaf that is the door of ≥1 **driven** row (table or ledger), in `VERB_KINDS` spelling:

| leaf | rows / ledger drives |
|---|---|
| `start` | table 1-14, 9-11 orientation arms; ledger C1, C2, C3, C4, C6, A6-1, A6-2, A6-3, D-13 |
| `workflow` | table 15-21; ledger C1, C2, A6-1, A6-2, A6-3 (preview arm), row-15/21 re-drive |
| `describe` | table 22-24; ledger C4, C6, row-22 re-drive |
| `migrate` | table 25-27; ledger C1 (`--format json` envelope) |
| `milestone execute` | table 28-31; ledger A6-2 (the surface it is the honest door for) |
| `milestone create` | ledger A6-1 |
| `milestone add-task` | ledger A6-1 |
| `milestone provision` | ledger A6-1 |
| `milestone join` | ledger A6-1 |
| `milestone finalize` | ledger A6-1 (both the `repo.head-detached` block and the exit-0 drop) |
| `doc show` | table 32-33; ledger C4, C5 |
| `doc list` | table 34; ledger C4, A6-3 (the law-2 contrast sentence) |
| `doc schema` | ledger C5 (the commit payload grammar read during A6-1) |
| `doc set-field` | ledger A6-1 |
| `doc set-slot` | ledger A6-1 |
| `doc author` | ledger A6-1 (three refused payload shapes, `write.wrong-shape`) |
| `task validate` | table 35-37; ledger C4 |
| `task bind` | ledger A6-3 (three refusals) |
| `task finalize` | table 1, 25; ledger row-36 re-drive (`--help` span) |
| `task list` | ledger row-15 re-drive (the mint-count control) |
| `setup` | ledger C6 (the `setup.pack-load` advisory over a non-loading pack) |
| `ingest` | ledger C5 (`--help` leaf check only) |
| `config set` | ledger C5 (`--help` leaf check only) |

**23 leaves driven to behaviour** (the 8 doors this axis derives + the 15 the drives reach
through). Seven further leaves were reached by the C5 sweep at `--help` only and are listed there,
not here: `doc create`, `doc add-item`, `task discard`, `milestone add-from-spec`, plus `ingest`,
`config set` and `doc schema` (the last three also appear above where a drive used them). Every
other `VERB_KINDS` leaf — `rename`, `uninstall`, `doc rename`, `doc retitle-item`, `task diff`,
`migrate-corpus`, `milestone discard`, the config read verbs — is another axis's subject and was
not reached at all.
