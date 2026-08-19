# Findings verification — the 1.0.0-gate trial

The adversarial pass over every claim the trial produced: the four workers' feedback
(`feedback-B1.md`, `feedback-B2.md`, `feedback-B3a.md`, `feedback-B3b.md`), the running record
(`session-findings.md`), the pre-trial findings (`pre-trial-findings.md`), and the operator walk
(`v1-walk.md`).

**Every claim below carries a live repro — CONFIRMED, PARTIAL and REFUTED alike**, per
[protocol.md](protocol.md) §7 and [pinning.md](../../../implementation/pinning.md) §3. A REFUTED
finding is valuable, not a failure: five of the last six trials produced a headline complaint that
dissolved into shipped capability, and that pattern *is* the discoverability signal this trial
measures.

**Method.** `jigc 1.0.0-rc.11` on PATH; throwaway corpora built with
`completions/trial-corpus-template/instantiate.sh --clean-prose`, worked entirely inside the
session scratchpad — no trial corpus under `~/ideas/` was read or written. Regression questions were
settled against the container image `jigc-gate:rc10`, confirmed genuine pre-M48 in-band (it answers
`jigc doc rename --help` and `jigc config get` with **exit 2**, both M48 verbs). **Every exit code
below was measured unpiped** — redirected to a file, `$?` read directly — because three readings
earlier in this trial were `head`'s status rather than jigc's.

---

## Verdict counts

Seventeen claims, one verdict each (the two-sided ones — B1's schema gap, B1's docs-gate — are
counted as the two rows they are).

| Verdict | Count | Claims |
|---|---|---|
| **CONFIRMED** | 11 | F-1 · B2-2 · B2-4 · B3b-5 · B1-1 · D-1 (the discoverability half) · B3b-1 (router) · B3b-2 · B3b-3 · PT-1 · S-2 |
| **PARTIAL** | 3 | B2-1 · B2-3 · S-1 (B1-3) |
| **REFUTED** | 3 | D-1 (the capability half) · B1-2 (docs-gate surface) · S-3 (task-id predictability) |
| **Unsettled** | 0 | — |

**Regressions against rc.10: zero.** Every finding that could plausibly be one was driven on both
binaries and came back identical (F-1, B1-1, PT-1 explicitly; the rest sit on surfaces M48 did not
touch).

**Class distribution under [protocol.md](protocol.md) §1** — classes fixed from evidence *before*
the consequence column was looked up. The three REFUTED rows carry no class, by construction.

| §1 class | Findings | Consequence |
|---|---|---|
| Data loss or corruption on any path | **none** | — |
| A regression | **none** | — |
| A seventh discoverability landing | see §7 and §8 | measured, and it landed — routed to the §3 reading, not to this table |
| A blocking dead end (project-carrying reach) | PT-1 | **the human's call**, unchanged — see §11 |
| A wrong result on a non-destructive path | F-1 · B2-2 | SHIPS RECORDED |
| A surface/wording finding | B1-1 · B2-1 · B2-3 · B3b-1 · B3b-2 · B3b-3 · D-1 · S-2 | SHIPS RECORDED |
| A capability gap | B2-4 · B3b-5 · S-1 | SHIPS RECORDED, routed to M46 |

**No finding in this pass is a one-way door.** None touches a pinned `--format json` contract: every
CONFIRMED row is prose, a route, a predicate or an absent verb, all reversible after 1.0.0. The
one-way-door qualifier therefore promotes nothing.

---

## 1 · F-1 · `changelog-recording.gate-granted-unused` fires on a task that DID record an entry

**VERDICT: CONFIRMED** — reproduced independently, and the exact boundary established, which the
first observation did not have.

### The boundary, measured on three tasks

The advisory is **not** keyed on the verb. It is keyed on the **count of repeatable items** the
staged changelog carries versus the committed one. Three tasks on one corpus, each differing only in
how the changelog was touched:

| # | what the task did to the changelog | advisory? |
|---|---|---|
| 1 | `doc create changelog` + `add-item …#unreleased-changes --title added` (fresh doc) | **no** |
| 2 | `set-slot …#unreleased-changes/added/notes` on the **existing** `added` category | **YES** |
| 3 | `add-item …#unreleased-changes --title changed` — a **new** category on the **committed** doc (copy-in) | **no** |

So B3a's account (*"the gate tracker apparently only counts a changelog write as recorded when it
comes through `add-item`"*) is right in effect and slightly off in mechanism: **any item-count
increase at any nesting depth suppresses it — a release, an unreleased change-group, a nested
category group — and any prose-only edit to an existing item does not.** `retitle-item` and a
`set-field` on a release's `date` fall on the same wrong side by construction.

Confirmed at the source — one predicate, one call site:

```rust
// crates/cli/src/task.rs:1858 (fn changelog_gate_advisory)
if changelog_entry_count(schema, &staged) > changelog_entry_count(schema, &committed) {
    return Ok(None);
}
```

with `changelog_entry_count` documented as *"the number of repeatable **items** a doc source carries,
at every nesting depth"* (`crates/cli/src/task.rs:553`).

### Repro

```console
$ completions/trial-corpus-template/instantiate.sh --clean-prose $SP/cg svc
$ cd $SP/cg && jigc setup            # exit 0 — install commit 181476c

# --- task 1: create + add-item on a fresh changelog ---
$ jigc start --workflow single-task "add a retention knob to the store" --format json
  .task = add-a-retention-knob
$ jigc doc create changelog --title Changelog --task add-a-retention-knob
changelog:changelog
$ jigc doc add-item changelog:changelog#unreleased-changes --title added --task add-a-retention-knob
changelog:changelog#unreleased-changes/added
$ printf -- '- A retention knob on the store.\n' \
    | jigc doc set-slot 'changelog:changelog#unreleased-changes/added/notes' --from-file - --task add-a-retention-knob
$ jigc task finalize add-a-retention-knob ; echo "exit=$?"
finalized a3feae6 — feat: Add a retention knob to the sample store.
  promoted CHANGELOG.md
  modified src/store.ts
  2 files committed
exit=0
                                   ← NO changelog-recording advisory

# --- task 2: set-slot on the SAME, now-committed category ---
$ jigc start --workflow single-task "cap the sample window size" --format json
  .task = cap-the-sample-window-size
$ printf -- '- A retention knob on the store.\n- A cap on the sample window size.\n' \
    | jigc doc set-slot 'changelog:changelog#unreleased-changes/added/notes' --from-file - --task cap-the-sample-window-size
set slot changelog:changelog#unreleased-changes/added/notes (68 chars) (copied in for update — the committed doc is now this task's staged copy, re-promoted at finalize)
$ jigc task finalize cap-the-sample-window-size ; echo "exit=$?"
advisory · changelog-recording.gate-granted-unused — workflow `single-task` grants the `changelog` create-gate and this task recorded no changelog entry
  route: if the change is user-facing, record it — `jigc start --workflow record-change "<what changed>"` (or, before finalize, in-task: `jigc doc create changelog --title Changelog --task cap-the-sample-window-size`, then `jigc doc add-item changelog:changelog#unreleased-changes --title <category> --task cap-the-sample-window-size`); if it is not user-facing, no action is needed
— jigc · run `jigc start` for orientation; all writes through `jigc`.

finalized f4eefbd — feat: Cap the sample window size.
  promoted CHANGELOG.md                        ← the same output promotes the file it says was not written
  modified src/store.ts
  2 files committed
exit=0

# --- task 3: add-item of a NEW category on the COMMITTED doc ---
$ jigc doc add-item changelog:changelog#unreleased-changes --title changed --task expose-the-window-cap
changelog:changelog#unreleased-changes/changed (copied in for update — …)
$ jigc task finalize expose-the-window-cap ; echo "exit=$?"
finalized 500c67c — feat: Expose the window cap in the router.
  promoted CHANGELOG.md
  2 files committed
exit=0
                                   ← NO advisory: the copy-in path is fine, the count is what matters
```

### Two aggravations the first observation did not have

**(a) Both halves of the printed route are wrong for the case that triggers it.** The route's
in-task option is `jigc doc add-item changelog:changelog#unreleased-changes --title <category>` —
which, for the category the task actually edited, is *refused*:

```console
$ jigc doc add-item changelog:changelog#unreleased-changes --title added --task tighten-the-summary-rounding ; echo "exit=$?"
blocking · write.already-present — write rejected: item "added" in section "unreleased-changes" is already present
  route: the target already exists — edit it in place (`set-field`/`set-slot`) instead of re-creating it
exit=1
```

The other option, `jigc start --workflow record-change "<what changed>"`, is the path that produces
the duplicated user-facing history B3a named — a *second* entry for the change already recorded. So a
worker following the route lands on a refusal or a duplicate, never on a correct action. The
`write.already-present` route then sends it back to `set-slot` — the very verb that fired the
advisory. **The two routes point at each other.**

**(b) It is a post-commit ambush, and the code comment beside it says otherwise.** The advisory is
computed at exactly one call site, inside `finalize` (`crates/cli/src/task.rs:1190`), after the
commit lands. `jigc task validate` on the identical task state does **not** surface it:

```console
$ jigc task validate tighten-the-summary-rounding ; echo "exit=$?"
advisory · file-state.staged-copy — staged copy of `CHANGELOG.md` — this task's in-flight version of the doc
  route: no action needed — the staged copy is validated in-task and baselined when its finalize lands
exit=0
                                   ← no changelog-recording row
```

That is by design (`crates/cli/tests/changelog_gate_advisory.rs` opens *"becomes a finalize
advisory"*), but the route-construction comment two lines above the emission claims the opposite —
*"the in-task form follows, marked as the before-finalize option (it is live on the `task validate`
preview)"* (`crates/cli/src/task.rs:1893`). **That parenthetical is false.** It is an internal doc
comment, not a printed surface, so it is not itself a law-1 violation — but it is the false premise
that produced a route naming in-task verbs the worker can no longer run.

### Regression? **No** — identical on rc.10

```console
# jigc-gate:rc10 (8979f16, pre-M48), the same corpus, the same task-2 sequence
$ jigc --version
jigc 1.0.0-rc.10
$ jigc task finalize tighten-the-summary-rounding ; echo "exit=$?"
advisory · changelog-recording.gate-granted-unused — workflow `single-task` grants the `changelog` create-gate and this task recorded no changelog entry
  route: if the change is user-facing, record it — `jigc start --workflow record-change "<what changed>"` (or, before finalize, in-task: …)
finalized cc4efbf — fix: Tighten the summary rounding.
  promoted CHANGELOG.md
exit=0
```

The defect is M42's, carried unchanged through M43–M48.

### Class under §1

**A wrong result on a non-destructive path** — a check that does not fire (a false negative on the
gate tracker) surfacing as a false claim. Not a false green over managed state (nothing managed is
mis-written — the entry really lands), no pinned `--format json` contract violated, and the wording
plus the predicate are both reversible after 1.0.0 → **SHIPS RECORDED**, and near the top of that
list, because its route actively instructs a duplicate.

### Conversion ledger ([pinning.md](../../../implementation/pinning.md) §3)

**Converted 2026-08-19 (M46 Increment 6), from `UNPINNED` to a citation.** This row was
`UNPINNED` on the ground that the fact it establishes *is the defect*: a standing test over the
prose-only cell would have pinned the false positive as expected output
([pinning.md](../../../implementation/pinning.md) §5's *"would pinning it now pin the bug?"*), and
this trial shipped no fix. M46 Increment 6 shipped it, which is the rule's other half — the
moment the bug is fixed, the cell that would have pinned it is the cell that pins the correct
behaviour.

`pinned-by: changelog_write_touch::every_doc_write_verb_reads_as_a_write_touch` — verified by
reading the assertions: ten cells over the eight `["doc", …]` write verbs, each driven against a
**committed** `CHANGELOG.md` inside a gate-granting task, asserting what the advisory does at that
task's finalize. `set-slot` on an existing category, `retitle-item` on a cut release and
`set-field` on a release `date` — the three count-stable writes this verdict measured, task 2
above among them — assert **suppressed**; `create` over the committed doc, `create` with nothing
committed, and a **refused** `set-field` (which copies in and then rejects, leaving the baseline's
own bytes staged) assert **still fires**; `doc rename` is declared inapplicable with its refusal
quoted. `changelog_write_touch::every_doc_write_verb_has_a_cell` fences that table against
`cli::cli::VERB_KINDS` **both ways**, so the axis cannot lose a verb silently.

**Aggravation (b)** — the `task validate` omission — is fenced by
`validate_previews_the_gate::a_granted_but_unused_changelog_gate_is_previewed_at_validate`: the
advisory previews at exit 0 keyed `task:<id>`, and every backticked `jigc …` span extracted from
its **own printed route** is run verbatim at that door. The landing-door half is
`the_landing_finalize_door_prints_no_dead_argv` in the same suite (cited in full at S-4 below) —
the arm that was red at HEAD. **Aggravation (a)** loses its premise rather than its wording: a
task that edited a category no longer draws the advisory at all, and the cycle the two routes
formed is broken at the `set-slot` cell above — the hop `write.already-present` sends a worker to
is now the hop that records the entry.

**Deliberately not cited** ([pinning.md](../../../implementation/pinning.md) §3):
`crates/cli/tests/changelog_gate_advisory.rs`, the suite this row cited before. Its three arms
stayed green and byte-unmodified, but they touch none of the cells this finding is about — a
citation that reads apt by name is exactly what §3 refuses.

---

## 2 · B2-1 · The sub-task step instructs `doc set-field commit:<task>#type`, and the create gate is `[adr]`

**VERDICT: PARTIAL.** The refusal chain is real and reproducible; the printed step is **not** itself
unrunnable. B2's account compresses two steps into one, and the compression changes the class.

### What the step says, and what actually happens

`crates/cli/pack/steps/sub-task-commit.yaml` → `{{ include: step:author-commit }}` renders
`Run: jigc doc set-field commit:<sub>#type …`, while
`crates/cli/pack/workflows/sub-task.yaml` carries `allows-create: [{type: adr, as: decision}]`.
B2 reported that running the printed command *"gave `create.gate-blocked`"*. It does not:

```console
# a real milestone, two sub-tasks, provisioned
$ jigc milestone create "bound the store"        # exit 0 — shared base d46c317
$ jigc milestone add-task bound-the-store "add a configurable rollup max"
$ jigc milestone add-task bound-the-store "enforce the rollup max in the router"
$ jigc milestone provision bound-the-store       # exit 0 — 2 worktree(s)

# (A) the sub-task's workflow HAS been composed → the printed command works, from either tree
$ cd .jigc/worktrees/add-a-configurable-rollup-max
$ jigc workflow sub-task --task add-a-configurable-rollup-max > /dev/null
$ jigc doc set-field commit:add-a-configurable-rollup-max#type --value feat --task add-a-configurable-rollup-max ; echo "exit=$?"
set commit:add-a-configurable-rollup-max#type = feat
exit=0
$ cd ../../..                                     # back in the main tree
$ jigc doc set-field commit:add-a-configurable-rollup-max#scope --value store --task add-a-configurable-rollup-max ; echo "exit=$?"
set commit:add-a-configurable-rollup-max#scope = store
exit=0
```

**cwd is irrelevant** — the sub-task's staged docs live in the *main* repo's
`.jigc/tasks/<sub>/docs/`, which jigc resolves identically from inside a worktree. What matters is
whether the sub-task's workflow has been **composed**, because that is what provisions the commit
doc:

```console
# (B) a sub-task whose workflow was NEVER composed
$ jigc doc set-field commit:enforce-the-rollup-max#type --value feat --task enforce-the-rollup-max ; echo "exit=$?"
no staged instance for `commit:enforce-the-rollup-max#type` — provision it first (`jigc start` / `jigc doc create <type> --title 'X'`). Note: `jigc doc create <type> --title 'X'` derives the id from the title (`X` → slug), not the task id — address writes at that title-derived id
exit=1

# following the SECOND of that refusal's two options:
$ jigc doc create commit --title "add a configurable rollup max" --task add-a-configurable-rollup-max ; echo "exit=$?"
blocking · create.gate-blocked — the workflow does not allow `jigc doc create commit` in-task; allowed doctypes: [adr]
  route: create `commit` in a task minted from a workflow that grants it (`jigc start` lists the catalog) — the gate is the workflow's own `allows-create:` front-matter, pack authoring, not a project-config knob
exit=1
```

### What is confirmed, and what is refuted

- **CONFIRMED:** a two-option route where the second option is categorically refused for the workflow
  that raised it. `no staged instance … provision it first (`jigc start` / `jigc doc create <type>`)`
  is a *generic* refusal that does not know it is speaking to a sub-task whose gate forbids the verb
  it names. That is a law-1 wobble on a printed route.
- **REFUTED:** *"the printed steps described an action the tool then refused."* The composed step
  text is runnable, and it is runnable precisely because reading it requires having composed the
  workflow that provisions the doc. The failure is reachable only by running the command **out of
  order** — before, or without, composing the sub-task.
- **REFUTED:** *"commit authorship is a milestone-level concern, not a sub-task-level one."* Each
  sub-task genuinely owns a `commit:<sub>` doc, and both were authored and joined in this repro:

```console
$ jigc milestone join bound-the-store ; echo "exit=$?"
joined milestone:bound-the-store — 2 doc(s) merged
  - commit:add-a-configurable-rollup-max  (created · from add-a-configurable-rollup-max)
  - commit:enforce-the-rollup-max  (created · from enforce-the-rollup-max)
exit=0
```

The *subject line* is milestone-synthesized; the per-sub-task commit docs are real.

### Regression? No — the pack shape and the refusal text predate M48.

### Class under §1

**A surface/wording finding** — a route that names a verb the raising context forbids, on a
non-blocking path with a correct sibling option (`jigc start`) in the same sentence. Reversible →
**SHIPS RECORDED**. The good fix is for the `no staged instance` route to drop the create option when
the active workflow's `allows-create` does not carry the doctype.

### Conversion ledger ([pinning.md](../../../implementation/pinning.md) §3)

**`pinned-by: milestone::milestone_finalize_squash_false_genuine_reentry_authors_per_subtask_commits`**
— both REFUTED halves, verified by content: it drives a genuine `jigc workflow sub-task --task
<sub>` re-entry **from inside each provisioned worktree**, asserts the re-entry provisions
`.jigc/tasks/<sub>/docs/commit:<sub>.md`, and then runs the printed
`jigc doc set-field commit:<sub>#type --value feat --task <sub>` and asserts **exit 0** — its own
comment names that call as *"exactly the call that exits 1 (no staged instance — provision it
first)"* without the compose. The same test authors each sub-task's `commit:<sub>` prose and
carries it through join + finalize, which is the second refutation (commit authorship is
genuinely sub-task-level).

**Converted 2026-08-19 (M46 Increment 8 / T3), from `UNPINNED` to a citation.** The CONFIRMED half
— the `no staged instance` refusal offering `jigc doc create <type>` without knowing the active
workflow's `allows-create` forbids it — was `UNPINNED` because a standing test over the gate-blind
route would have pinned the mis-route as expected output
([pinning.md](../../../implementation/pinning.md) §5). T3 shipped the fix, so that same cell now
pins the correct behaviour: the refusal derives its provisioning act from the task's own gate.

`pinned-by: anyhow_route_spans::absent_instance_refusal_never_routes_to_a_forbidden_create` —
verified by reading the assertions: it drives the 2×2 {the gate admits the addressed doctype, it
forbids it} × {a `commit:<task>` address, a non-commit address} and asserts each cell's **full
emitted stderr bytes** at exit 1 — the forbidding cell now names the gate that closed it (*"its
`allows-create:` gate lists []"*) and routes at the catalog rather than at a create, and the
`commit` column names the compose that provisions it — then lifts **every** backticked `jigc …`
span out of the refusal and runs it from the state that raised it: verbatim, and again with
`<type>` substituted for the addressed doctype (a literal-placeholder run answers `unknown
doctype` and would mask the defect), asserting **no run answers `create.gate-blocked`** ·
`anyhow_route_spans::the_gate_aware_refusal_routes_land_where_they_claim` — the two claims the new
text makes about the state it prints in: `jigc doc list --task qf` really lists `commit:qf` (the
compose really does provision it) and bare `jigc start`'s catalog really lists `record-decision` (a
workflow that grants `adr`).

The gate the refusal used to collide with stays fenced where it was —
`doc_author::disallowed_doctype_is_gate_blocked_through_the_batch` asserts an un-allowed doctype is
refused with `create.gate-blocked` and the staged set is byte-unchanged — and the two can no longer
disagree by construction: the repaired refusal reads `engine::state::create_admission`, the same
predicate `create.gate-blocked` fires from.

---

## 3 · B2-2 · Seeding sub-tasks makes them un-resumable from the main tree

**VERDICT: CONFIRMED** — and the finding is *sharper* than reported: the route is wrong for the only
case that raises it.

### Repro

```console
$ jigc milestone create "bound the store"
minted milestone:bound-the-store (shared base d46c317)
record commit: 864310b
next: `jigc milestone add-task bound-the-store "<intent>"`   — add the milestone's first sub-task

$ jigc milestone add-task bound-the-store "add a configurable rollup max"
added task:add-a-configurable-rollup-max to milestone:bound-the-store
                                            ← no `next:` line here
$ jigc milestone add-task bound-the-store "enforce the rollup max in the router"
added task:enforce-the-rollup-max to milestone:bound-the-store

$ git log --oneline | head -4
4803ec2 chore(milestone): record task:enforce-the-rollup-max on milestone:bound-the-store
f4cfa8c chore(milestone): record task:add-a-configurable-rollup-max on milestone:bound-the-store
864310b chore(milestone): open record for milestone:bound-the-store
d46c317 chore(jigc): install jigc workspace config      ← the milestone's base pin

$ jigc start --task add-a-configurable-rollup-max ; echo "exit=$?"
task `add-a-configurable-rollup-max` is pinned to base d46c317 but you're on 4803ec2 — switch back with `git checkout d46c317` or `jigc task discard add-a-configurable-rollup-max`
exit=1
```

The milestone's own three bookkeeping commits move HEAD three commits past the pin every sub-task is
created against. B2's *"the very act of seeding two sub-tasks made both un-resumable from the main
tree"* is exactly right.

### The route is wrong, and the code knows which case it is serving

This message is emitted by a function whose own doc comment names its exclusive subject:

```rust
/// The **blanket base-pin refusal** — the one message every door of a *milestone
/// sub-task* raises when the checkout has moved off the milestone's shared pin.
fn blanket_base_pin_refusal(id: &str, pinned: &BasePin, head: &BasePin) -> anyhow::Error {
    anyhow!(
        "task `{id}` is pinned to base {} but you're on {} — switch back with `git checkout {}` or {}",
        …, engine::finding::Route::mechanical(["jigc", "task", "discard", id], ""),
    )
}
// crates/cli/src/start.rs:1681–1698
```

It is raised **only** for milestone sub-tasks — a top-level task resumes cleanly after HEAD moves,
via M47's overlap-aware re-pin:

```console
$ jigc start --workflow quick-fix "tweak the summary output" --format json    # .task = tweak-the-summary-output
$ git commit -q -m "docs: unrelated commit" -- README.md      # HEAD moves
$ jigc start --task tweak-the-summary-output ; echo "exit=$?"
Reason about the change. The intent is:
tweak the summary output
…
exit=0
```

So the refusal is milestone-sub-task-exclusive, and **neither of the two routes it offers is the
right one for that case**:

- `git checkout d46c317` detaches the *main working tree* onto the base pin — the milestone's own
  record commits then sit ahead of a detached HEAD. The correct isolation mechanism
  (`.jigc/worktrees/<id>`) is never named.
- `jigc task discard <sub>` runs at **exit 0** and leaves the milestone still naming the task:

```console
$ jigc task discard add-a-configurable-rollup-max ; echo "exit=$?"
discarded task add-a-configurable-rollup-max
exit=0
$ jigc milestone list-tasks bound-the-store
milestone:bound-the-store tasks (2): add-a-configurable-rollup-max, enforce-the-rollup-max
```

**No data loss:** `milestone provision` and `milestone join` both still run clean afterwards
(`provisioned 2 worktree(s)` / `joined … 0 doc(s) merged — no docs staged from: …`), so the state is
recoverable. But following a printed route at exit 0 leaves a milestone naming a task whose working
area was just removed, with nothing said about it.

### The correct route exists, one verb away, and it is not the one printed

`jigc milestone execute <m>` composes text that states the whole mechanism — provision, worktree
paths, and the code fold:

```
Run: `jigc milestone provision <MILESTONE_ID>`
Spawn: `cd .jigc/worktrees/add-a-configurable-rollup-max && jigc workflow sub-task --task add-a-configurable-rollup-max`
…
Each sub-task reaches this boundary through two channels: the docs it wrote through the CLI
(an overlay merged by task id) and the code it `git add`-staged, folded from that worktree's
staged index — a sub-task's unstaged code edits never reach the commit
```

So B2's *"nothing up to that point told me"* is **PARTIAL on its own terms** — the information
exists, in the workflow the worker had not yet run. What is genuinely missing is the hand-off:
`milestone create` prints `next: jigc milestone add-task …`, and `milestone add-task` prints **no
`next:` at all**. The chain that would have carried the worker to `milestone execute` breaks at
exactly the verb it had just run twice.

### Regression? No — the blanket refusal is M47 Inc 8, present and identical on rc.10.

### Class under §1

**A wrong result on a non-destructive path** — a route that mis-routes on the only case it serves,
plus a discoverability break in the `next:` chain. Nothing is destroyed and a correct recovery exists
(`milestone provision` + work from the worktree), so it does **not** meet the blocking-dead-end bar
under the blast-radius qualifier: the state has a recorded recovery, and the reachable set is the
milestone workbench, not a project-carrying file → **SHIPS RECORDED**. The two mechanical fixes are
small: route this refusal at `jigc milestone provision <m>` / `cd .jigc/worktrees/<id>`, and give
`add-task` a `next:` line.

### Conversion ledger ([pinning.md](../../../implementation/pinning.md) §3)

**`pinned-by: start_resume::sub_task_read_doors_keep_the_blanket_base_pin_refusal`** — **revised
at M46 Increment 8 / T4, not doubled** (precision note 2 at the foot of this file called that out
at plan time, and [pinning.md](../../../implementation/pinning.md) §3's addendum names this very
test as the hazard). Verified by reading the *revised* assertions: over **both** sub-task read
doors (`start --task <sub>`, `workflow <W> --task <sub>`) and **both** provisioning states, the
refusal still carries `is pinned to base`, must **not** name `jigc task discard do-the-thing`, must
name `.jigc/worktrees/do-the-thing`, and every `jigc …` span lifted out of the emitted bytes is run
**verbatim** from the state that raised it — after which the arm reads back `git show
HEAD:docs/milestone-records/rework.md` (still `status: active`, still naming the sub-task),
`milestone list-tasks` (still lists it) and the sub-task's working area (still present), so a route
that contradicts the record it leaves standing cannot pass; the top-level control still resumes at
exit 0 in the same repo state, and work planted in the provisioned worktree survives the route's
second run · **`anyhow_route_spans::pinned_base_mismatch_routes_by_unit_kind`** — the same refusal
read at the **span** level and split on the axis the route now splits on: the sub-task arm asserts
the emitted `jigc milestone provision rework` route and the absence of `jigc task discard <sub>`,
the top-level arm asserts its own `git checkout` + `jigc task discard top-task` bytes are unmoved.

**`UNPINNED:`** two halves T4 did not buy, each with its reason **restated** rather than left as it
stood. **(1)** *The milestone's own record commits are what move HEAD off the pin every sub-task is
created against.* The cited arm now runs in the committed-record state — it writes the
`[dev ▸ methodology]` compose marker precisely so `milestone create` / `add-task` land committed
records — but it also makes one disjoint commit, so what it fences is the **refusal and its route**,
not the trial's causal claim; isolating that cause needs an arm that makes no other commit, and no
shipped behaviour turns on which commit moved HEAD. **(2)** *`milestone add-task` prints no
`next:`.* Refused by the wave's razor as a preference with no qualified rule behind it
([razor-ledger.md](../M46/razor-ledger.md) §3 — *"B2-2's `next:` half"*), so the sentence still does
not exist and there is nothing to cite. The `next:`-chain fence one verb earlier —
`milestone::milestone_create_names_its_record_commit_its_path_and_the_next_step`, which lifts
`create`'s emitted `next:` argv and runs it verbatim — is still where the chain would be extended
the day that changes.

---

## 4 · B2-3 · `milestone join --help` describes only a docs union while finalize folds code

**VERDICT: PARTIAL.** The observation is real; the verb it is aimed at is the wrong one. `join` is
truthful — it *is* docs-only. `milestone finalize --help` is the one that undersells.

### Repro — `join` really does only docs

```console
$ jigc milestone join --help
Merge the milestone's sub-task areas into the parent working overlay by the by-task-id join —
enumerate sub-areas by sorted task id, disjoint-union their staged docs (collision-suffixing
distinct created instances), and report the merged outcome. Commits nothing. …

$ jigc milestone join bound-the-store ; echo "exit=$?"
joined milestone:bound-the-store — 2 doc(s) merged
  - commit:add-a-configurable-rollup-max  (created · from add-a-configurable-rollup-max)
  - commit:enforce-the-rollup-max  (created · from enforce-the-rollup-max)
exit=0
```

Two docs, zero code. The help text is accurate for the verb it documents.

### Repro — `finalize`'s help is the undersell

```console
$ jigc milestone finalize --help
The milestone commit boundary — run the by-task-id join, materialize its suffix-resolved doc
bodies into the parent staging area, and commit them as one logical boundary with a
CLI-synthesized message. A blocking join finding (a same-doc clash, an unknown milestone) routes
to stderr and commits nothing

Options:
      --carry-staged  … The aggregate commit is built from the sub-task worktrees, so the carried
                        entries never ride it …
      --format …
  -h, --help
```

*"materialize its suffix-resolved **doc bodies** … and commit **them**"* — the description enumerates
only docs. The single oblique hint that code exists at all (*"built from the sub-task worktrees"*) is
buried in a flag whose subject is the carryover gate. What it actually commits:

```console
$ jigc milestone finalize bound-the-store --format json ; echo "exit=$?"
{
  "committed": {
    "files": 3,
    "hash": "815028a",
    "manifest": [
      { "kind": "modified", "path": "docs/milestone-records/bound-the-store.md" },
      { "kind": "modified", "path": "src/router.ts" },
      { "kind": "modified", "path": "src/store.ts" }
    ],
    "sub_tasks": [
      { "code_files": 1, "discarded": [], "docs": 1, "id": "add-a-configurable-rollup-max",  "provisioned": true },
      { "code_files": 1, "discarded": [], "docs": 1, "id": "enforce-the-rollup-max",         "provisioned": true }
    ],
    "subject": "Finalize milestone bound-the-store (2 sub-tasks)"
  }
}
exit=0
```

Two source files, folded from two independent detached worktrees, in a commit whose `--help`
mentions only doc bodies. B2's *"the `--help` text for the verb that actually does the work undersold
what it does"* is **correct — about `finalize`, not `join`.**

The `milestone-execution` workflow text does state the code fold explicitly (quoted in §3), so this
is help-text scope only, not a system-wide silence.

### Regression? No — both help strings are pre-M48.

### Class under §1

**A surface/wording finding** on a non-blocking path, reversible → **SHIPS RECORDED**. One sentence
in `milestone finalize`'s `about` closes it.

### Conversion ledger ([pinning.md](../../../implementation/pinning.md) §3)

**`pinned-by:`** both halves of the *behaviour* the help mis-describes:
`milestone::milestone_join_suffixes_a_created_collision_and_reports_the_decision` — after the join
it re-reads `git rev-parse HEAD` + `git status --porcelain` and asserts the pair is **identical**
to the pre-join snapshot (*"the join must commit nothing and leave the working tree unchanged"*),
so `join --help`'s docs-only claim is true · `milestone::milestone_finalize_squash_true_genuine_reentry_materializes_transient_then_lands_aggregate`
— asserts the single aggregate commit's tree carries `src/low.rs` **and** `src/zed.rs`, folded
from two independent worktrees, beside the merged persisted docs (the code fold `finalize --help`
omits).

**Converted 2026-08-19 (M46 Increment 8 / T2), from `UNPINNED` to a citation.** The finding itself
— `milestone finalize`'s `about` — was `UNPINNED` because no suite asserted any `milestone` help
string and pinning the shipped sentence would have pinned the undersell as expected output. T2
repaired the `about`: it now names the join's suffix-resolved doc bodies **and** the code staged in
each sub-task worktree, folded in by task id.

`pinned-by: help_truth::milestone_finalize_about_names_the_code_fold_beside_the_doc_bodies` —
verified by reading the assertions: it drives the real binary's `milestone finalize --help`, cuts
the `about` out of that output and requires all three of `doc bodies`, `code` and `sub-task
worktree` in it, then takes the text after `--carry-staged` **in the same output** and requires it
to name that same `sub-task worktree` subject. The fence is the **agreement**, not mere presence —
which is the shape of this finding, a contradiction inside one help screen — so the `about` cannot
drift back into describing a different boundary than the flag three lines below it.

---

## 5 · B2-4 / B3b-5 · No `--dry-run` for `milestone finalize`; no bulk/patch edit for existing items

**VERDICT: CONFIRMED (both)** — with one qualifier on each.

### 5a · No milestone dry-run

```console
$ jigc task finalize --help | grep -c -- --dry-run
1
$ jigc milestone finalize --help
…
Options:
      --carry-staged  …
      --format <FORMAT>  …
  -h, --help  Print help
                                   ← no --dry-run
```

`task finalize --dry-run` exists and is richly documented (it forecasts validation findings, the
empty-commit guard and the carryover gate). The milestone equivalent has no such flag.

**Qualifier:** `milestone join` is a *partial* preview — its own help says **"Commits nothing"**, and
it prints the merged doc set (repro in §4). What is genuinely unpreviewable is the **code** half and
the commit manifest: nothing short of the real `finalize` will tell you which files a two-worktree
fold will carry. That is exactly the uncertainty B2 named as the direct cause of its `git reset
--hard` workaround, so the qualifier narrows the gap without dissolving it.

### 5b · No bulk/patch edit for existing items

```console
# a committed spec with four criteria, and a task from a spec-granting workflow
$ cat payload2.yaml
title: Retention cap
sections:
  - id: criteria
    items:
      - title: cap is configurable
        set: { maps-to-test: test/store.test.ts#cap }
      - title: cap defaults to 1000
        set: { maps-to-test: test/store.test.ts#default }

$ jigc doc author spec --from-file payload2.yaml --task revise-the-retention-cap-spec ; echo "exit=$?"
blocking · write.already-present — write rejected: item "cap-is-configurable" in section "criteria" is already present
  route: the target already exists — edit it in place (`set-field`/`set-slot`) instead of re-creating it
exit=1

# the only path is N calls
$ jigc doc set-field spec:retention-cap#criteria/cap-is-configurable/maps-to-test --value 'test/ingest.test.ts#IngestQueue' --task revise-the-retention-cap-spec ; echo "exit=$?"
set spec:retention-cap#criteria/cap-is-configurable/maps-to-test = test/ingest.test.ts#IngestQueue (copied in for update — …)
exit=0
```

The whole payload is rejected on the first collision, exactly as `jigc doc author --help` documents
(*"a payload item whose title mints an id the doc ALREADY holds is refused (`write.already-present`)
with the whole payload rejected and nothing staged"*). So B3b's *"might well be deliberate (author is
create-shaped, not patch-shaped)"* is right, and the behaviour is stated up front rather than
ambushing.

**Qualifier for B3b's own session:** in its actual context — an `implement-from-spec` task —
`doc author spec` never reaches that rule at all; it is refused earlier:

```console
$ jigc doc author spec --from-file payload2.yaml --task implement-the-retention-cap ; echo "exit=$?"
blocking · create.gate-blocked — the workflow does not allow `jigc doc create spec` in-task; allowed doctypes: [adr]
exit=1
```

### Regression? No — both surfaces are pre-M48.

### Class under §1

**A capability gap** — *"I wanted a verb that does not exist"* — for both → **SHIPS RECORDED, routed
to M46.** Neither is a defect: M48 refused seven of these deliberately.

### Conversion ledger ([pinning.md](../../../implementation/pinning.md) §3)

**5a · `UNPINNED:`** a capability gap — `milestone finalize` has no `--dry-run`, and an absent
flag has no behaviour to fence. The qualifier's fact (join *is* a partial preview, because it
commits nothing) is the same join assertion cited in §4.

**5b · `pinned-by: author_write_contract::a_colliding_payload_item_rejects_the_whole_author_over_a_committed_singleton`**
— asserts the colliding payload is refused with `write.already-present` **and** that the task's
staged `roadmap:roadmap.md` does not exist afterwards (whole payload rejected, nothing staged),
then that a fresh title appends beside the committed entry; `::every_step_stating_the_append_half_states_the_collision_reject`
and `::the_composed_batch_author_note_states_the_collision_reject` assert the composed surfaces
state that reject up front, which is the *"stated rather than ambushing"* half. B3b's own context
is fenced too: `doc_author::disallowed_doctype_is_gate_blocked_through_the_batch` (an
`implement-from-spec`-shaped gate refuses `doc author spec` earlier, staging nothing). The **gap**
— a patch-shaped bulk edit — is a verb that does not exist, so nothing there is pinnable.

---

## 6 · B1-1 · The "staged" vocabulary collision in the hook-rejection frame

**VERDICT: CONFIRMED.** Here is the text jigc prints, verbatim, and the git state it prints it over.

### Repro — the misleading branch (docs-only task, git index empty)

```console
# a rejecting pre-commit hook, the shape B1's plant installed
$ cat .git/hooks/pre-commit
#!/bin/sh
echo "docs-gate: ask the docs reviewer to sign off" >&2
exit 1

$ jigc task finalize revise-the-retention-cap-spec ; echo "exit=$?"
`git commit` was rejected (no commit was made):
docs-gate: ask the docs reviewer to sign off

task revise-the-retention-cap-spec is intact — nothing was committed and your staged changes are still staged. Fix the hook's complaint, then re-run `jigc task finalize revise-the-retention-cap-spec`.
exit=1

$ git diff --cached --stat ; echo "exit=$?"
exit=0
                                   ← git's index is EMPTY. "your staged changes are still staged"
                                     describes .jigc/tasks/<id>/docs/, which the sentence never names.
```

This is the exact detour B1 described: the sentence borrows git's word at the moment a worker is
deciding whether its work survived, `git diff --cached` returns nothing, and the worker must go find
`.jigc/tasks/<id>/docs/` to convince itself.

### Repro — the true branch (the same sentence, when code *was* staged)

```console
$ printf '\n// cap\n' >> src/store.ts && git add src/store.ts
$ jigc task finalize revise-the-retention-cap-spec ; echo "exit=$?"
`git commit` was rejected (no commit was made):
docs-gate: ask the docs reviewer to sign off

task revise-the-retention-cap-spec is intact — nothing was committed and your staged changes are still staged. …
exit=1
$ git diff --cached --stat
 src/store.ts | 2 ++
 1 file changed, 2 insertions(+)
```

**Boundary:** the sentence is *true* when the worker has `git add`-ed code, and reads *false against
`git diff --cached`* on a docs-only task — the exact task shape a doc-review hook most often
rejects. It is one sentence covering two mechanisms without distinguishing them.

### On B1-2 (no jigc surface for the docs gate), settled in passing

The gate in B1's session was a **foreign hook the plant installed**, not jigc's — so "jigc doesn't
expose it" is correct by design, and jigc's own `setup`-installed hook is warn-only. Recorded as
**REFUTED as a defect**, retained as the observation that a worker reached for `jigc doc approve` /
`jigc docs-gate status` and found nothing. jigc *does* surface the hook's own stderr verbatim
(`docs-gate: ask the docs reviewer to sign off` above), which is the whole of what it can honestly
know about a third-party hook.

### Regression? **No** — byte-identical on rc.10

```console
# jigc-gate:rc10, same rejecting hook, docs-only task
$ jigc task finalize record-the-rounding-policy ; echo "exit=$?"
`git commit` was rejected (no commit was made):
docs-gate: ask the docs reviewer to sign off

task record-the-rounding-policy is intact — nothing was committed and your staged changes are still staged. Fix the hook's complaint, then re-run `jigc task finalize record-the-rounding-policy`.
exit=1
$ git diff --cached --stat        # empty
```

### Class under §1

**A surface/wording finding** — accurate about jigc's mechanism, ambiguous in a foreign vocabulary,
on a path that is already refusing (so nothing is lost). Reversible → **SHIPS RECORDED.** The fix is
to name the staging area: *"your task's staged docs are intact in `.jigc/tasks/<id>/`; your git index
is unchanged."*

### Conversion ledger ([pinning.md](../../../implementation/pinning.md) §3)

**B1-1 · Converted 2026-08-19 (M46 Increment 8 / T1), from `UNPINNED` to a citation.** The
confirmed fact is a *wording* ambiguity — one sentence saying "staged" in two mechanisms' senses —
and it was `UNPINNED` because pinning the shipped sentence would have pinned the ambiguity as
expected output. T1 scoped the clause per `surface-contract.md:136` (*the repair is scope, never a
behaviour change*): it names the task's staged-doc area and git's index separately, and nothing
about the rollback moved.

`pinned-by: commit_rejected_axis::the_task_door_names_its_own_staged_docs_over_an_empty_git_index`
— verified by reading the assertions: it drives a **docs-only** task (its whole contribution one
in-task ADR, nothing `git add`-ed — this row's own branch), asserts `git diff --cached --name-only`
is empty **going in and again at the moment the frame prints** (driven, not assumed: the refused
finalize runs four scoped rollback axes over the index before the text is emitted), then reads the
`task <id> is intact` clause out of the emitted stderr and requires it to name
`.jigc/tasks/<id>/docs/` and **not** to say *"your staged changes are still staged"* — and finally
that the named area genuinely still holds the staged `adr:eviction-policy.md`. The other half of
the repaired sentence, git's index, is the shipped
`finalize_message_truth::hook_rejection_says_the_task_is_intact_and_the_rerun_lands`, whose re-run
commits exactly the `git add`-ed path.

The claim *underneath* stays fenced where it was:
`commit_rejected_axis::every_committing_door_frames_its_rejection_names_itself_and_recovers`
asserts, per door, that the run exits non-zero, **HEAD is unmoved**, the hook's own bytes survive
verbatim under git's unwrapped rejection line, the state-truth sentence names what survived (for
this door, `task <id> is intact`), and the argv **lifted out of the emitted message** exits 0 once
the hook is removed. Its `task finalize` fixture stages `code.txt` before driving the door — the
*true* branch only — which is why the docs-only branch needed the arm above rather than an
extension of that sweep.

**B1-2 · `pinned-by: precommit_hook_acceptance::commit_warns_on_stale_anchor_but_succeeds`** —
jigc's own installed hook is warn-only: the drift warning appears in the commit output **and the
commit lands** (asserted against `git log`), so there is no jigc-side docs gate to expose; ·
`commit_rejected_axis::every_committing_door_frames_its_rejection_names_itself_and_recovers`
(clause 2) — a foreign hook's own bytes reach the reader verbatim, which is the whole of what jigc
can honestly know about a third-party gate. The refuting fact has standing tests; no duplicate is
minted (§3).

---

## 7 · D-1 · `jigc doc schema` answers the need B1 said it could not meet

**VERDICT: REFUTED as a capability gap · CONFIRMED as a discoverability finding.**

B1's claim, verbatim: *"A schema/introspection command for a doctype's exact address tree (which
leaves are fields vs. slots, required vs. optional) before writing to it. `describe` and the workflow
preview give prose, not a structured schema. I ended up reverse-engineering it empirically (create
then `show --format json`)."*

### What refutes it, by what it does

```console
$ jigc doc schema adr ; echo "exit=$?"
doctype: adr (schema-version 2)
fields:
  - status: enum [proposed|accepted|superseded] (section: status) (default: proposed) (set-field: adr:<slug>#status/status)
  - date: date (section: status) (set: on-create) (set-field: adr:<slug>#status/date)
  - supersedes: ref (section: status) (set-field: adr:<slug>#status/supersedes)
  - cites-code: code-anchor (section: status) (set-field: adr:<slug>#status/cites-code)
  - schema-version: int (section: status) (set: schema-version)
sections:
  - context: slot (set-slot: adr:<slug>#context)
  - options: slot (optional) (set-slot: adr:<slug>#options)
  - decision: slot (set-slot: adr:<slug>#decision)
  - consequences: slot (set-slot: adr:<slug>#consequences)
exit=0
```

Point by point against the stated need:

- **"which leaves are fields vs. slots"** — two headed lists, `fields:` and `sections:`, each leaf
  typed (`enum` / `date` / `ref` / `code-anchor` / `int` / `slot`).
- **"required vs. optional"** — `options` carries `(optional)`; unmarked is required. On doctypes
  with author-required *item* leaves the listing additionally leads with a legend, e.g.
  `jigc doc schema spec` opens `* = author-required` and marks `- title: string … *`.
- **"the exact address tree"** — every settable leaf prints its literal write address, including
  nested repeatables: `jigc doc schema changelog` prints
  `(set-slot: changelog:<slug>#releases/<id>/changes/<id>/notes)`.
- **The machine-readable form** exists too and is separately versioned: `--format json`,
  `contract-version 5` since M48.

B1's specific confusion — *"I wasn't sure whether `status` was a slot or a plain field"* — is answered
by the first line of that output, and the exact write address it needed
(`adr:<slug>#status/status`) is printed there. The bad YAML payload it drafted and discarded would
never have been drafted.

`pinned-by:` (verified by reading what each test asserts, per §7):
- `crates/cli/tests/doc_schema.rs::doc_schema_plain_listing_surfaces_write_addresses` — asserts the
  `status` **field** line contains `(set-field: adr:<slug>#status/status)`, that the CLI-stamped
  `schema-version` line stays address-less, and that the `context` **slot section** line reads
  `- context: slot (set-slot: adr:<slug>#context)`. That is precisely the field-vs-slot,
  address-bearing projection B1 wanted.
- `crates/cli/tests/doc_schema.rs::doc_schema_plain_listing_legends_the_marker_where_it_lands` —
  asserts `* = author-required` sits on line 2, exactly once, ahead of any marker.
- `crates/cli/tests/doc_schema.rs::doc_schema_json_is_the_pinned_contract` — the pinned `--format
  json` contract.
- `crates/cli/tests/schema_projection.rs::composed_schema_projection_names_every_adr_section_and_its_skeleton_authors`
  — asserts the **composed workflow text** already names all five adr sections including
  `` - `options`: prose slot (optional) `` and projects `enum, one of: proposed | accepted |
  superseded`.

That last citation partly refutes B1's *"the workflow preview gives prose, not a structured schema"*
as well. The live `record-decision --preview` names all three required slots, the optional `options`
slot with its rationale, `status`'s default, and every write address; what it does **not** enumerate
is the `status` section's other three fields (`date`, `supersedes`, `cites-code`) or the enum
members. So the preview is structured but partial, and `doc schema` is the complete answer.

### The discoverability half — countable, on the same instrument M48 used for `doc show`

M48's fence made the surface *name* `jigc doc show --task`. Applying the identical count to
`doc schema` on this binary:

```console
$ grep -rho "jigc doc [a-z-]*" crates/cli/pack/ packs/methodology/ | sort | uniq -c | sort -rn
  34 jigc doc show
  32 jigc doc set-field
  29 jigc doc set-slot
  26 jigc doc author
  12 jigc doc add-item
   2 jigc doc create
                                   ← `jigc doc schema`: 0.  `jigc doc list`: 0.
$ grep -c "doc schema" .claude/skills/jigc/SKILL.md
0
```

It **is** named in the preload — once, in `.jigc/AGENT.md` line 3:

> *"That freedom is for **project** source; to learn how `jigc` itself behaves, ask the installed
> binary (`jigc --help`, `jigc describe`, `jigc doc schema`), never a checked-out jigc or pack source
> tree."*

— under the heading *"to learn how jigc itself behaves"*, i.e. framed as **tool introspection**, not
as *"learn a doctype's shape before writing to it."* B1 was not looking for how jigc behaves; it was
looking for what an adr looks like. The one mention is filed under the wrong intent.

This is the signature the protocol pre-registered, landing a **seventh** time: a headline complaint
that dissolves into shipped capability. It is also the sharpest evidence yet for §9's declared bound
— the read-back fence worked for the verb it names, and the lens moved straight to the next unnamed
one.

### Class under §1

**A surface/wording finding** (the AGENT.md mention is filed under a framing that does not match the
need; zero pack steps reach for it) → **SHIPS RECORDED, routed to M46.** The capability gap half is
**REFUTED**, so nothing is owed there.

### The discoverability half — `UNPINNED:`

The refutation above carries its `pinned-by:` citations. The **CONFIRMED** half —
`jigc doc schema` named zero times across both packs' step files and `.claude/skills/jigc/SKILL.md`,
once in `.jigc/AGENT.md` under a *"how jigc itself behaves"* framing — is a **count over pack
sources plus the framing of one sentence**: an apparatus measurement, not a product behaviour, and
pinning *"zero pack steps name `jigc doc schema`"* would pin the gap as expected output. The
nearest fenced neighbour pins a **statement about** such a count, not the count itself:
`foldback_truth::the_pack_step_count_is_stated_once_and_names_its_measurement_point` (the `doc
show` count is stated in exactly one home and names the binary it was measured on). Routed to M46
with the finding, unfenced by design.

---

## 8 · B3a-1 / B3b-1 · The router does not surface off-router workflows

**VERDICT: CONFIRMED**, reported independently by two of four sessions.

### Repro

```console
$ jigc start "add a changelog entry for the new window cap" ; echo "exit=$?"
These are the selectable work-workflows, each with the situation it fits:

- architecture-documentation — document the architecture of a part of the system, …
- decided-task — implement one scoped change test-first, recording its design decision …
- dev-task — implement one scoped change test-first, recording no decision, …
- do-research — you need to investigate or gather evidence …
- form-vision — you're forming or revising the project's vision from research
- implement-from-spec — a committed spec already covers the intent, …
- park-idea — a shaped-but-unscheduled idea occurs mid-work …
- plan — draft the specification for upcoming work before writing any code
- project-setup — bootstrap a brand-new project …
- quick-fix — apply a small commit-only fix …
- record-decision — capture a choice you have settled, …
- single-task — implement one scoped change end-to-end, …

Pick the workflow whose situation best fits the intent, then re-run with that
choice and the original intent:

jigc start --workflow <chosen> "<intent>"
exit=0
```

Twelve entries; `record-change` — the workflow whose entire purpose is *"record a user-facing change
on the project changelog"* — is not among them, and the output names no other place to look.

### How a worker would find it

Exactly one way, and it is the way both workers found it:

```console
$ jigc describe --workflows | grep -A1 "^record-change"
record-change is Record a change on the changelog — create-or-update the singleton, author a
release (or a staged change-group) with its nested category groups, and commit. Reach for it when a
user-facing change needs recording on the changelog — staged now, or cut into a versioned release.
It is hidden from the router catalog: reached by name for the deliberate record-a-change/cut-a-release
pass (`jigc start --workflow record-change`); routine change recording already rides `single-task`'s
record-changelog step, so a catalog line would duplicate it.
```

The suppression is **deliberate and correctly declared** in the pack, with a permanent expiry —
M48's `suppressed:` fence working as designed:

```yaml
# crates/cli/pack/workflows/record-change.yaml
selectable: false
suppressed:
  reason: reached by name for the deliberate record-a-change/cut-a-release pass (`jigc start
    --workflow record-change`); routine change recording already rides `single-task`'s
    record-changelog step, so a catalog line would duplicate it
  expires: never
```

So the design is sound and self-documenting. **The finding is that the router's own output is a
closed list with no exit** — nothing in it says "this catalog is the selectable subset; `jigc
describe --workflows` shows every workflow, including the ones reached by name." The rationale is
readable only *after* you already found the thing.

The single in-flow surface that ever names `record-change` is the F-1 advisory's route (§1) — which
fires **after** finalize, on a task that has already recorded the entry.

### Regression? No — `selectable: false` on `record-change` predates M48.

### Class under §1

**A surface/wording finding** — a missing route on a non-blocking path (the router terminates without
naming the fuller catalog) → **SHIPS RECORDED.** A one-line addendum to the router's closing text
closes it, and does so without re-listing hidden workflows in the catalog the suppression exists to
keep clean.

### Conversion ledger ([pinning.md](../../../implementation/pinning.md) §3)

**`pinned-by: routing_loop::step_5_the_live_router_catalog_lists_exactly_the_real_work_workflows`**
— drives bare `jigc start`, parses the emitted `- <id> — <when>` lines and asserts the id set is
**exactly** the eight selectable dev-pack workflows, so `record-change`'s absence from the catalog
is asserted rather than incidental (the suite also asserts the pairwise `when` non-overlap over
the live list) · **`describe::describe_projects_the_suppression_reason_for_hidden_workflows`** —
derives the hidden set from **both** pack sources and asserts every hidden workflow's `describe`
entry says it is hidden from the router catalog and carries its declared reason verbatim: the one
path both workers took, fenced · **`suppression_fence::a_stripped_suppressed_block_is_blocked_at_pack_load`**
— stripping `suppressed:` from a hidden workflow makes the real binary exit non-zero at pack load,
naming it, so the declaration `record-change` carries cannot quietly lapse.

**Converted 2026-08-19 (M46 Increment 8 / T6), from `UNPINNED` to a citation.** The finding itself
— the router's closing text naming no fuller catalog — was `UNPINNED` because an absent sentence
has nothing to assert and the shipped closing bytes would have pinned the missing exit as expected
output (they ride the compose goldens, which exist to *notice* change, not to fence content). T6
added the exit: the catalog is named as the **selectable subset**, a workflow outside it as
reachable by name with the same `--workflow` form, and `jigc describe --workflows` as the fuller
read carrying each hidden workflow's declared reason.

`pinned-by: routing_loop::step_6_the_router_closing_text_names_the_fuller_catalog_and_that_read_answers`
— verified by reading the assertions: it drives bare `jigc start "<intent>"`, parses the emitted
`- <id> — <when>` option lines and requires `record-change` to be **off** the catalog and absent
from the whole output (so the repair cannot quietly become an un-hiding of a workflow whose
`suppressed.expires` is `never`), requires the closing text to name its catalog as a `selectable
subset`, then lifts the backticked spans out of that emitted stdout, requires **exactly one** `jigc
describe …` read among them, runs it **verbatim** from the state that printed it, and asserts
`record-change` comes back — the read is proven to answer, never restated.

---

## 9 · B3b-2 · `doc show --task` refuses with `store.not-staged` right after `task bind`

**VERDICT: CONFIRMED**, with the route verified runnable.

### Repro

```console
$ jigc start --workflow implement-from-spec "implement the retention cap" --format json
  .task = implement-the-retention-cap
$ jigc task bind spec spec:retention-cap implement-the-retention-cap ; echo "exit=$?"
bound spec = spec:retention-cap (task implement-the-retention-cap)
exit=0

$ jigc doc show spec:retention-cap --task implement-the-retention-cap ; echo "exit=$?"
blocking · store.not-staged — `spec:retention-cap` is not staged in this task — only its committed copy exists
  route: `jigc doc show spec:retention-cap` — the task-less read serves the committed copy
exit=1

$ jigc doc show spec:retention-cap ; echo "exit=$?"        # the route, followed verbatim
…the committed spec…
exit=0
```

The behaviour is correct — `bind` records a context-role binding for the resume re-compose, it does
not copy a doc into the task's working area — and the refusal is a **model** one under M43's laws: it
names the state, gives one route, and that route works when followed. B3b's own judgment
(*"sensible in hindsight, not obvious in the moment"*) is the accurate reading.

**Why it is still recorded:** it is the read-back verb this trial exists to measure, refusing at the
exact moment a worker reaches for it, and the ack that precedes it (`bound spec = spec:retention-cap`)
says nothing about staging. A worker that has just internalised *"with `--task` the read serves THIS
task's staged copy"* — the sentence every pack step repeats — meets a refusal one command later. The
smallest honest fix is in the `bind` ack, not the refusal: *"…bound for resume; it is not staged —
read it with `jigc doc show spec:retention-cap`."*

### Regression? No — `bind` and `store.not-staged` are pre-M48.

### Class under §1

**A surface/wording finding** on a path that refuses cleanly with a working route → **SHIPS
RECORDED.**

### Conversion ledger ([pinning.md](../../../implementation/pinning.md) §3)

**`pinned-by: doc_show_staged::committed_json_carries_no_marker_and_not_staged_routes_task_less`**
— its second arm drives `doc show <addr> --task <other>` from a task that never staged the doc and
asserts the read **blocks**, that the stderr states the real state (`not staged`), and that the
route names the task-less `jigc doc show <addr>` — the refusal-plus-working-route this row calls
correct. The `bind` side is fenced for what it *does*:
`task_bind::task_bind_enforces_the_five_steps_and_records_the_binding` asserts bind's whole
persisted effect is `.jigc/tasks/<id>/roles.json` mapping the role to the committed doc, and
`::the_task_state_verbs_ack_their_mutation` pins the ack's fields (`op`/`task`/`role`/`target`/
`findings`) — which is why the ack can say nothing about staging.

**Converted 2026-08-19 (M46 Increment 8 / T7), from `UNPINNED` to a citation — on the defect,
which is not where this row first put it.** The row's suggested repair was the `bind` ack naming
the un-staged state. The wave's razor refused that as a preference with no qualified rule behind it
([razor-ledger.md](../M46/razor-ledger.md) §3 — *"the `bind` ack"*) and located the law-1 violation
in the pack step instead: `locate-from-spec.yaml:43-48` printed *"Read your write back … **the
write you just made**"* **unconditionally** over the spec, while `:34-36` sanctions leaving that
write off — the state this row's repro is in. T7 scoped the read-back to the write the step
solicits unconditionally (`commit:{{task.id}}`) and named what the spec does instead in each arm.

`pinned-by: spec_read_back_arms::every_printed_read_answers_whether_or_not_the_optional_spec_write_was_made`
— verified by reading the assertions: it mints, binds and re-composes an `implement-from-spec` task
twice — arm A wiring a criterion to a real staged test, arm B taking the path the step's own prose
sanctions — runs the step's own emitted `jigc doc set-field` lines, then lifts **every** `jigc doc
show …` argv the `locate-from-spec` region prints (the standalone line *and* the inline backticked
span, unwrapped across its hard wrap) and runs each **verbatim**, requiring exit 0 in **both** arms.
It also asserts arm B's staged spec read really does still block `store.not-staged` — so a "fix"
that staged the spec regardless cannot green it, and the refusal this row calls correct stays
correct — and that the step keeps stating a staged read-back at all, so deleting the promise
reddens.

**`UNPINNED:`** the `bind` ack sentence, with its reason restated: it is no longer merely
un-written, it is **refused** by name in the razor's OUT set, so nothing is owed and there is
nothing to cite. What `bind` does is pinned above.

---

## 10 · B3b-3 · An unadopted foreign file's advisory repeats on every unrelated task

**VERDICT: CONFIRMED.**

### Repro

```console
# a corpus with ONE never-adopted foreign ADR committed at the adr home
$ jigc doc list
id  path  state
adr:0002-keep-the-sample-store-in-memory  docs/decisions/0002-keep-the-sample-store-in-memory.md  unregistered

# unrelated task #1 — `quick-fix`, touching only src/router.ts
$ jigc task validate fix-a-typo ; echo "exit=$?"
blocking · schema-conformance.field-value-conformant — `commit:fix-a-typo`: field `type` … is empty
blocking · schema-conformance.required-slot-present — `commit:fix-a-typo`: required slot in section `summary` is empty
advisory · schema-conformance.unadopted-instance — committed file `docs/decisions/0002-keep-the-sample-store-in-memory.md` sits at the `adr` home but was never adopted by jigc …
exit=3

$ jigc task finalize fix-a-typo ; echo "exit=$?"
advisory · schema-conformance.unadopted-instance — committed file `docs/decisions/0002-…md` sits at the `adr` home but was never adopted by jigc …
  route: adopt — run `jigc ingest` to route it, or `jigc migrate … --as adr` …
finalized 0ffae70 — fix: Fix a typo in the router.
exit=0

# unrelated task #2 — `quick-fix`, touching only src/store.ts
$ jigc task finalize rename-a-local ; echo "exit=$?"
… 1 × unadopted-instance, again …
exit=0
```

Four surfacings across two unrelated tasks (`task validate` + `task finalize` each). B3b's *"every
validate/finalize call across three unrelated tasks"* holds.

**No mute exists.** The code has no cascade knob:

```console
$ grep -rn "unadopted-instance" crates/cli/pack/config/ packs/methodology/config/ crates/engine/src/knobs.rs
                                   ← no output: not a tunable severity key
```

Compare `changelog-recording.gate-granted-unused`, which *is* tunable
(`validation.changelog-recording.gate-granted-unused.severity`). So the only way to silence this row
is to adopt the file — which is exactly what the route says, and what B3b eventually did. The
finding is that until then it rides every gate, and it has no owner: the row is about the *store*,
not about the task it prints on.

### Regression? No — `unadopted-instance` is M42's, and the store-scope-rides-task-gate behaviour is older still.

### Class under §1

**A surface/wording finding** — correct, correctly routed, non-blocking, repeated → **SHIPS
RECORDED.** Worth noting it is the *good* half of M42's managed-vs-foreign discriminator being noisy,
not wrong; the honest fix is scope-aware suppression (print store-scope rows once per store state, or
only on `jigc validate`), not a severity downgrade.

### Conversion ledger ([pinning.md](../../../implementation/pinning.md) §3)

**`pinned-by: foreign_at_both_doors::a_foreign_file_answers_one_code_and_one_route_at_every_door`**
— verified by content, and it asserts *this row's own fact*: the foreign file draws the same
`(code, target)` and a **byte-identical route** at `jigc validate`, `jigc task validate` and the
`task finalize` preflight, is never baseline-adopted into `.jigc/state/file-state.json`, and
**re-fires unchanged on a second, unrelated commit-only task's finalize** (*"the advisory re-fires
unchanged, never silently absorbed"*), with both task doors at exit 0. So the repetition B3b met
is **asserted, deliberate behaviour**, and this finding is a scope judgment against it, not a
defect — the honest disposition, and nothing further is owed.

*Cross-check worth recording:* [RC-pre-1.0's ledger](../RC-pre-1.0/findings-verification.md) marks
F10's second half `UNPINNED` on the ground that a standing test over *"the advisory still fires on
a task that never touched the file"* would pin the habituation floor as expected output. The suite
it cites in F10's **first** half already contains exactly that arm. The disposition there was
generous to itself; the fact was fenced all along.

---

## 11 · PT-1 · `migrate-corpus` claims a never-adopted foreign file with an unrunnable route

**VERDICT: CONFIRMED. Not a regression.** Re-verified independently on rc.11 and on rc.10.

### Repro — rc.11, the ADR instance

```console
$ completions/trial-corpus-template/instantiate.sh --clean-prose $SP/pt1 svc
$ cd $SP/pt1 && jigc setup && mkdir -p docs/decisions
$ cp completions/artifacts/RC-1.0-gate/plants/b3-foreign-adr-clean-prose.md \
     docs/decisions/0002-keep-the-sample-store-in-memory.md
$ git add docs/decisions && git commit -q -m "docs: record the in-memory store decision" -- docs/decisions

$ jigc migrate-corpus ; echo "exit=$?"
corpus migration: 0 migrated, 0 already current, 1 blocked
  blocked    docs/decisions/0002-keep-the-sample-store-in-memory.md
    migrate-corpus.prose-needed: `docs/decisions/0002-keep-the-sample-store-in-memory.md`'s migration mints a new **required** prose slot, which no transform can fill (the CLI owns structure; the prose is the agent's — the determinism boundary), so the doc does not gate clean and its bytes are rolled back untouched
    route: author the new required prose in `docs/decisions/0002-keep-the-sample-store-in-memory.md` through the write verbs, then re-run `jigc migrate-corpus` (the schema-version stamp flips only once it gates clean)
exit=1

# the route, followed: the doc id it implies does not exist
$ jigc doc show adr:keep-the-sample-store-in-memory ; echo "exit=$?"
blocking · store.not-found — could not read `adr:keep-the-sample-store-in-memory` at `…/docs/decisions/keep-the-sample-store-in-memory.md`: No such file or directory (os error 2)
exit=1
$ jigc doc list
adr:0002-keep-the-sample-store-in-memory  docs/decisions/0002-keep-the-sample-store-in-memory.md  unregistered

# the correct surface disagrees, in writing, and routes properly
$ jigc validate ; echo "exit=$?"
advisory · schema-conformance.unadopted-instance — committed file `docs/decisions/0002-…md` sits at the `adr` home but was never adopted by jigc — it carries no schema-version stamp and parses against no known `adr` schema version
  route: adopt — run `jigc ingest` to route it, or `jigc migrate docs/decisions/0002-…md --as adr` to rewrite it into the managed `adr` shape; it is a foreign file, not an unmigrated managed doc
1 finding(s) — report-only at store scope (exit 0); each gates nowhere …
exit=0
```

### Repro — the blast radius, on a repo-root managed doc

The defect is not specific to ADRs; it fires on **any** never-adopted file at a managed doctype's
home, and two of those homes are repo-root files:

```console
$ printf '# Changelog\n\n## 0.3.0\n\n- Aligned window rollups.\n' > CHANGELOG.md
$ git add CHANGELOG.md && git commit -q -m "docs: add a changelog" -- CHANGELOG.md

$ jigc migrate-corpus ; echo "exit=$?"
corpus migration: 0 migrated, 0 already current, 1 blocked
  blocked    CHANGELOG.md
    migrate-corpus.prose-needed: `CHANGELOG.md`'s migration mints a new **required** prose slot, …
    route: author the new required prose in `CHANGELOG.md` through the write verbs, then re-run `jigc migrate-corpus` …
exit=1

$ jigc validate ; echo "exit=$?"
advisory · schema-conformance.unadopted-instance — committed file `CHANGELOG.md` sits at the `changelog` home but was never adopted by jigc …
  route: adopt — run `jigc ingest` …, or `jigc migrate CHANGELOG.md --as changelog` …; it is a foreign file, not an unmigrated managed doc
exit=0
```

For the singleton the *other* read surface routes correctly too, which bounds the damage:

```console
$ jigc doc show changelog:changelog ; echo "exit=$?"
blocking · store.unparseable — `changelog:changelog` at `…/CHANGELOG.md` does not parse: section heading "0.3.0" does not match required section `unreleased-changes`
  route: adopt — run `jigc ingest` to route it, or `jigc migrate CHANGELOG.md --as changelog` …; it is a foreign file, not an unmigrated managed doc
exit=1
```

So the reachable set is confirmed as `CHANGELOG.md`, `VISION.md`, `docs/decisions/*` — root files and
decision history — and `migrate-corpus` is the **only** surface that gets it wrong; every neighbour
(`validate`, `doc show`, `doc list`) applies M42's discriminator correctly.

### Regression? **No** — identical on rc.10

```console
# jigc-gate:rc10 (8979f16, pre-M48) — the same corpus, docker cp'd in
$ jigc --version
jigc 1.0.0-rc.10
$ jigc migrate-corpus ; echo "exit=$?"
corpus migration: 0 migrated, 0 already current, 1 blocked
  blocked    docs/decisions/0002-keep-the-sample-store-in-memory.md
    migrate-corpus.prose-needed: … route: author the new required prose in `…0002-….md` through the write verbs, then re-run `jigc migrate-corpus` …
exit=1
```

Byte-identical output, identical exit. The image was confirmed genuinely pre-M48 in the same session
(`jigc doc rename --help` → exit 2, `jigc config get docs-root` → exit 2 — both M48 verbs).

### The trial's answer to PT-1's open questions

[pre-trial-findings.md](pre-trial-findings.md) carried PT-1 as **provisionally blocking**, pending a
recheck on *"whether a blind worker actually reaches it, and whether any further instance lands on a
path where `validate` is not the correct second opinion."* Both are now measured:

- **Reach:** demonstrated above and unchanged — root managed docs and decision history.
- **Likelihood:** **0 of 4 blind sessions** ran `migrate-corpus`. The one session (B3b) that met the
  exact triggering file went `validate` → `ingest` → `migrate … --as adr` and adopted it at
  `f536882`, taking M42's route rather than the wrong one.
- **Second opinion:** verified present on both instances found here — `validate` for both, plus
  `doc show` for the singleton. No instance was found where `validate` is not the correct second
  opinion.

### Class under §1

The two competing rows and the human's blast-radius qualifier are unchanged by this pass:

- **"A blocking dead end — a refusal whose route cannot run"** fires literally, and the reachable set
  is project-carrying (root `CHANGELOG.md`, `VISION.md`, decision records) → **BLOCKS.**
- **"A wrong result on a non-destructive path"** — no bytes move, it is a false *red* not a false
  green, no pinned `--format json` contract is violated, and a correct recovery exists one verb away
  → **SHIPS RECORDED.**

**This pass adds evidence, not a decision.** What it establishes: the defect is **not a regression**,
the reachable set **is** project-carrying (demonstrated on root `CHANGELOG.md`, not assumed), a
correct second opinion **is** present on every instance found, and the empirical reach in four blind
sessions was **zero**. The disposition remains the human's under §1, and *"judged not to matter"*
remains barred.

### Conversion ledger ([pinning.md](../../../implementation/pinning.md) §3)

**`UNPINNED:`** the fact is the defect — `migrate-corpus` claims a never-adopted foreign file and
blocks it with a route naming a doc id that does not exist. **No suite drives `migrate-corpus`
over a foreign file**: the four `migrate-corpus.prose-needed` assertions in
`crates/cli/src/migrate_corpus.rs` are all over genuinely **managed** docs (stamped, or in a
shipped prior shape), i.e. legitimate prose-needed cells. A standing test over the current
behaviour would pin the misclassification as expected output.

The second-opinion half **is** fenced, which is what bounds the blast radius:
`managed_vs_foreign::a_foreign_changelog_at_the_placement_home_is_an_adoption_case_not_an_unmigrated_corpus`
— the squatter draws **exactly one** finding, advisory, addressed at its **path**, whose route
names `jigc ingest` and `jigc migrate CHANGELOG.md --as changelog` and **must not contain
`migrate-corpus`**, at exit 0 ·
`::doc_show_over_a_foreign_squatter_routes_at_adoption_and_the_surfaces_tell_one_story` — `doc
show` (plain **and** `--format json`), `doc list` (`unregistered`) and `validate` are asserted to
tell one story about the same file. `migrate-corpus` is precisely the surface those two sweeps do
not include; that gap in the suite and the defect are the same hole.

---

## Supplementary — claims settled in passing

These were not on the verification list but were cheap to settle while the corpora were live, and
each closes a row in the workers' feedback.

### S-1 · B1-3 · No way to redo or narrow a finalize after the fact — **CONFIRMED (capability gap), PARTIAL as stated**

```console
$ jigc task --help | sed -n '/Commands:/,/Options:/p'
  list      Enumerate the active tasks …
  diff      Show the working changeset vs base …
  validate  Run `validate(task)` …
  discard   Abandon the task …
  finalize  The commit boundary …
  bind      Bind an already-committed doc …
                                   ← no `redo`, no `amend`, no `split`
$ jigc task finalize --help | grep -c -- '--dry-run'
1
```

**PARTIAL** because the *pre*-commit half of the need is served and was in front of B1: `--dry-run`
prints the exact manifest, and every real finalize prints a `left-out (unstaged/untracked — git add
to include):` block before committing. What does not exist is any **post-commit** narrowing, which is
where B1 was when it reached for `git reset --soft HEAD~1`. **Capability gap → SHIPS RECORDED, routed
to M46.** B1's reasoning for the manual fix was sound, and it verified reconciliation afterwards.

`pinned-by: carryover_gate::the_landed_json_labels_carried_over_and_forecast_landed_stay_identical`
for the **PARTIAL** half — verified by content: it asserts the `--dry-run` forecast's `manifest`
**and** `left_out` are equal to the landed commit's (*"dry-run and landed agree on the labeled
included set"* / *"…on the left-out set"*), so the pre-commit preview really is exact, which is why
the row is PARTIAL rather than a clean gap. **`UNPINNED:`** the gap itself — no post-commit
narrowing verb (`redo`/`amend`/`split`) exists, and an absent verb has no behaviour to fence;
routed to M46.

### S-2 · B2 · `jigc describe <item>` has no single-item lookup — **CONFIRMED**

```console
$ jigc describe milestone ; echo "exit=$?"
error: unexpected argument 'milestone' found

Usage: jigc describe [OPTIONS]

For more information, try '--help'.
exit=2
$ jigc describe --help | grep -E '^\s+--'
      --format <FORMAT>
      --workflows
      --doctypes
      --commands
```

Kind filters exist; item lookup does not. Note the contrast with M48's read-intent rule, which gives
the sibling near-miss a helpful tip — `jigc doc read` answers with *"reading a managed doc is its own
verb — `jigc doc list` … `jigc doc show <address>` …"* (v1-walk arm 4) — while `jigc describe
<item>` gets a bare clap error with no tip. **Class: a surface/wording finding** (a read-shaped
near-miss that answers with nothing) → **SHIPS RECORDED.**

`pinned-by: describe::describe_still_refuses_a_positional_argument` — asserts **both** positional
forms (`jigc describe single-task` and `jigc describe --workflows single-task`) exit non-zero and
print **no menu**, i.e. *"the single-item form is not built"*: the *kind filters exist, item lookup
does not* half of this row.

**Converted 2026-08-19 (M46 Increment 8 / T5), from `UNPINNED` to a citation.** The other half —
that the near-miss carries **no tip** — was an absence on stderr nothing asserted, and it sits
**outside** the read-intent fence by construction:
`unknown_subcommand_tip::no_read_intent_is_answered_with_a_write_verb` iterates `(parent node,
token naming no real child)` pairs off the clap tree — unknown *subcommands*, never an unexpected
positional at a leaf verb, which is exactly this shape. T5 filled it under law 2: jigc renders this
clap error kind itself (same four parts, same stream, same exit 2), so the refusal now carries the
answer `cli.rs:317` had already recorded.

`pinned-by: describe::the_foreclosed_positional_refusal_names_what_answers_it` — verified by
reading the assertions: over **both** foreclosed forms (`jigc describe adr` and `jigc describe
--workflows single-task`) it finds the `tip:` line in the real binary's stderr, requires it to name
all three kind filters (`--workflows`, `--doctypes`, `--commands`) and `jigc start --explain`, then
lifts every backticked `jigc …` span out of that tip and runs each **verbatim**, requiring exit 0 —
a tip naming an affordance that does not run is law 1's lie moved into the tip slot, and reddens
here. The single-item lookup stays foreclosed: `describe::describe_still_refuses_a_positional_argument`
holds unmodified.

### S-3 · B3b-4 · Minted task ids are not predictable — **REFUTED as a defect**

The id is printed three ways on the mint that creates it, and a fourth on demand:

```console
$ jigc start --workflow quick-fix "implement restart durability from the store" ; echo "exit=$?"
task minted: implement-restart-durability-from        ← line 1
…
resume: `jigc start --task implement-restart-durability-from`   — re-composes this workflow if context is lost
exit=0
$ jigc task list
jigc task list — 1 active task(s)

  implement-restart-durability-from  [quick-fix]  implement restart durability from the store
```

plus `.task` in `--format json`. The worker's guess (`implement-restart-durability-from-the`) differs
from the printed id by one word; the id was on line 1 of the output it was reading. This is a *pull*
observation — the model predicting rather than reading — not a product defect. **No consequence under
§1.**

`pinned-by:` `crates/cli/tests/checkpoint_acceptance.rs::resume_re_compose_is_byte_identical_to_a_fresh_compose`
— verified by content: it constructs `format!("task minted: {slug}\n\n")` and asserts the fresh
compose opens with it (`fresh.strip_prefix(&header)`, panicking otherwise), and asserts a resume
contains no `task minted:` line.

### S-4 · A source-level falsehood found while verifying F-1 — recorded, not classified

`crates/cli/src/task.rs:1893`, in the comment constructing the `gate-granted-unused` route:

> *"the in-task form follows, marked as the before-finalize option (it is live on the `task validate`
> preview)"*

It is not. The advisory has exactly one call site (`crates/cli/src/task.rs:1190`, inside `finalize`),
and `jigc task validate` on the triggering state exits 0 without it (repro in §1). This is an
internal doc comment, not a printed surface, so it is **not** a §1 finding — but it is the false
premise behind the route half that sends a worker at in-task verbs it can no longer run, so it should
be corrected alongside F-1 rather than separately.

**Converted 2026-08-19 (M46 Increment 6).**
`pinned-by: validate_previews_the_gate::the_landing_finalize_door_prints_no_dead_argv` — the
comment was corrected where it stood, and its *consequence* is fenced rather than argued. Verified
by reading the assertions: the arm drives a landing `finalize`, pulls every backticked `jigc …`
span out of the advisory's **own emitted route**, asserts none carries `--task` (the working area
is gone by then) and runs each verbatim, failing on any answer containing ``no task ` `` — which is
precisely the dead offer this false premise produced. The premise is no longer false either: the
advisory does preview, pinned by
`validate_previews_the_gate::a_granted_but_unused_changelog_gate_is_previewed_at_validate`. An
internal doc comment still has no standing test of its own; what changed is that the behaviour it
lied about has one. (Not one of the seventeen; carried here so no block in this file is silent.)

---

## What this pass did not settle

Nothing on the list was left unsettled. Two bounds worth stating so the record does not imply more
coverage than was bought:

1. **The B2 milestone repros were driven by the operator, not re-run blind.** They establish what the
   binary does; they do not re-measure whether a blind worker would reach the same sequence. B2's own
   session is the only evidence for the latter, and it is a single session.
2. **The cue-card correction path remains untested, not passed** — unchanged from
   [session-findings.md](session-findings.md). This pass verifies findings, not the instrument; the
   read-back measurement stands on 4/4 unprompted VERB, which is stronger evidence than the design
   anticipated, and the record must keep saying the correction was never delivered.

---

## The conversion ledger — closed

**The gate this closes** ([decisions-pending.md](../../../implementation/decisions-pending.md) →
*The rc.11 wave (M48)*, carried forward to the 1.0.0 call): *the 1.0.0 call is not taken until
every trial repro block carries `pinned-by:` or a stated `UNPINNED: <why>`.* All **17** rows now
do. Every citation was verified by **reading what the cited test asserts** — never by its name
looking apt — per [pinning.md](../../../implementation/pinning.md) §3, which refuses a symbol
parser for exactly that reason, and per §5's caution that `verb_suite_coverage` green means *named
by a suite*, never *fenced*.

| # | Row | Verdict | Disposition |
|---|---|---|---|
| 1 | F-1 changelog gate | CONFIRMED | `pinned-by` — the write-touch axis over every `doc` write verb · both preview doors (converted at M46 Inc 6) |
| 2 | B2-1 sub-task step / create gate | PARTIAL | `pinned-by` — both refuted halves · the gate-aware refusal + its routes (converted at M46 Inc 8 / T3) |
| 3 | B2-2 un-resumable sub-tasks | CONFIRMED | **mixed** — `pinned-by` the refusal + its **revised** route, over both doors and both provisioning states (M46 Inc 8 / T4) · `UNPINNED` the causal claim + the razor-refused `next:` |
| 4 | B2-3 `join`/`finalize` help | PARTIAL | `pinned-by` — both behaviours · the repaired `about`, fenced on its **agreement** with the flag below it (M46 Inc 8 / T2) |
| 5a | B2-4 no milestone `--dry-run` | CONFIRMED | `UNPINNED` — an absent flag has no behaviour to fence |
| 5b | B3b-5 no bulk/patch author | CONFIRMED | `pinned-by` — the create-shaped contract and its stated-up-front surfaces |
| 6 | B1-1 "staged" collision | CONFIRMED | `pinned-by` — the docs-only branch, clause read off an empty git index (converted at M46 Inc 8 / T1) |
| 6′ | B1-2 no jigc docs gate | **REFUTED** | `pinned-by` — warn-only own hook · verbatim foreign-hook relay |
| 7a | D-1 capability half | **REFUTED** | `pinned-by` — four `doc_schema` / `schema_projection` citations (pre-existing) |
| 7b | D-1 discoverability half | CONFIRMED | `UNPINNED` — a count over pack sources, not a product behaviour |
| 8 | B3b-1 router closed list | CONFIRMED | `pinned-by` — catalog membership + suppression fences · the added exit, its emitted read run verbatim (M46 Inc 8 / T6) |
| 9 | B3b-2 `store.not-staged` after `bind` | CONFIRMED | **mixed** — `pinned-by` the refusal + route, bind's real effect, and the repaired pack step over both arms (M46 Inc 8 / T7) · `UNPINNED` the razor-**refused** ack clause |
| 10 | B3b-3 repeating adoption advisory | CONFIRMED | `pinned-by` — the recurrence is asserted, deliberate behaviour |
| 11 | PT-1 `migrate-corpus` foreign claim | CONFIRMED | `UNPINNED` — pinning it would pin the defect; the fenced neighbours are cited |
| S-1 | B1-3 no post-commit narrowing | PARTIAL | **mixed** — `pinned-by` the exact `--dry-run` forecast · `UNPINNED` the absent verb |
| S-2 | `describe <item>` | CONFIRMED | `pinned-by` — the positional refusal · the tip that now answers it, every span run verbatim (M46 Inc 8 / T5) |
| S-3 | task-id predictability | **REFUTED** | `pinned-by` — `checkpoint_acceptance` (pre-existing) |

**Counts** (as of the two 2026-08-19 conversions — M46 Increments 6 and 8). **14 of 17** rows
carry at least one `pinned-by:` citation; **3** are `UNPINNED` outright; **3 of the 14** are still
**mixed** — a citation for the half that is verified-or-refuted behaviour, an `UNPINNED` for the
half that is an unfixed defect, a razor-refused preference, or a surface that does not exist yet.
**All three REFUTED rows carry citations**, so §3's *"a refuted fact **has** a standing test"*
obligation is discharged without minting a duplicate.

**The shape of the three outright-`UNPINNED` rows is one shape, and it is the intended one.** Two
(PT-1 and 5a's absent flag) would, if fenced today, **pin the defect or the gap as expected
output** — [pinning.md](../../../implementation/pinning.md) §5's *"would pinning it now pin the
bug?"*, the rule the latent-surface sweep already applied once when the leftover re-`provision`
left its scope. The third (D-1's discoverability half) is a measurement about pack content rather
than a behaviour of the binary. Each routes to a wave, not to a test:
[next-wave-scope.md](next-wave-scope.md) carries the dispositions. **F-1 left this set on
2026-08-19**, when M46 Increment 6 shipped its fix, **and B1-1 left it the same day** at Increment
8 — the rule read from the other end: a row `UNPINNED` because pinning it would pin the bug
converts the moment the bug is gone, and its citation is the fix's own arm (§1).

**Increment 8 converted the other six admitted rows the same way** (2, 3's route half, 4, 8, 9 and
S-2), each citation written *from the cited test's own assertions* per the [§3
addendum](../../../implementation/pinning.md) — and two `UNPINNED` halves survive with **new**
reasons rather than their trial-time ones: B2-2's `next:` line and B3b-2's `bind` ack are
**refused** by the wave's razor as preferences ([razor-ledger.md](../M46/razor-ledger.md) §3), not
merely unbuilt, so nothing is owed on either.


---

## Two precision notes on the ledger, added by the integrating session

**1 · The B3b-3 cross-check is narrower than first stated, and the difference is the point.**
`foreign_at_both_doors::a_foreign_file_answers_one_code_and_one_route_at_every_door:551-567` does
assert *"the advisory re-fires unchanged, never silently absorbed"* — but its subject is **the next
sweep** (`// and it re-fires on the next sweep (routed-but-not-recorded recurrence)`), i.e. a repeated
`validate` over the same unadopted file.

[RC-pre-1.0/findings-verification.md](../RC-pre-1.0/findings-verification.md) → F10 declared
`UNPINNED` on a **different** fact: *"the advisory still fires on a task that never touched the
file."* Repeat-sweep recurrence and unrelated-task recurrence are adjacent, not identical.

So F10's `UNPINNED` **stands**; what does not fully stand is its stated *reason* — *"a standing test
over it would pin the advisory-habituation floor as expected output"* — because a standing test over
the neighbouring recurrence already exists and was cited in F10's own first half. The older,
dated artifact is left unedited; the correction lives here, where the claim was made.

**2 · B2-2's citation carries a hazard M46 must plan around, not just a pin.**
`start_resume::sub_task_read_doors_keep_the_blanket_base_pin_refusal` asserts:

```rust
err.contains("is pinned to base") && err.contains("jigc task discard do-the-thing")
```

It fences the refusal **and pins the route B2 found unhelpful as expected output**. So the M46 fix
cannot simply add an assertion: it must **revise this one**, or a fixer improving the route will meet
a red test and be tempted to revert a correct change. Recorded here so the wave meets it at plan
time rather than mid-build.

**Discharged 2026-08-19 (M46 Increment 8 / T4), the way this note asked for.** The quoted
conjunction no longer exists — the fix **revised** that assertion instead of adding one beside it,
and the route it now requires is read back against the committed record rather than restated (row 3's
ledger above carries what replaced it). The note is kept rather than deleted, because the hazard it
names is a *class*: any citation that asserts emitted route text is a test a later wave must revise,
never out-vote.
