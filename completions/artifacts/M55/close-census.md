# The M55 close census — M55 Increment 11, T8

This file records the close's census ([roadmap.md](../../../implementation/roadmap.md) → Milestone 55 → Increment 11; [DECISIONS.md](../../../DECISIONS.md) → *2026-10-03 — M55 Increment 11 planning*, T8). It has two parts, on the method of [the M54 record](../M54/close-census.md). **§13** confirms that each of the twelve rows of [findings-channel.md](../../../design/findings-channel.md) → 13 was revised by the increment that changed its rule. **The markers** count every live-doc line that names `M55` on four trees, and give each matching line of the last tree one disposition. **Flipped** means the line spoke of M55 in the future tense, or stated as pending something that has now happened, and was reworded; the new wording is given. **Kept** means the line is true today, and the reason is given.

## §13 — the twelve rows

Each row's commits are read off `git log -p bea46dd8..HEAD -- <doc>` and attributed to their increment by the merge that brought them onto `milestone/findings-channel/main`. The line cited is the one `git blame` names for the rule today. **No row lacks a rule-revising commit, so nothing halts.**

| Doc | Revised by | What the doc says now |
|---|---|---|
| [reconciliation.md](../../../design/reconciliation.md) | Inc 4: `8b4f012f` (`:34`, `:77`), `2c284c48`; Inc 5: `edaa428c` (`:130`) | The `DRIFTED` + `TOUCHED` row absorbs when the on-disk bytes equal the base-pin blob (*A pulled edit is not a conflict*, `:77`). The store sweep's lag arm is advisory, and the weak rename signal with no history at `HEAD` is advisory at store scope too (`:130`). D10 is engaged where it stands: *The absorb is in memory, like every sweep's* — `start` and `task validate` write nothing. |
| [validation.md](../../../design/validation.md) | Inc 4: `2c284c48` (`:367`, `:524`); Inc 5: `6ad92379`, `edaa428c` (`:386`, `:621`) | *Exit semantics* narrows the `oob-rename` exception to the rename's blocking arm (`:386`). The M45 registration's dangling-baseline route names the branch switch and offers switching back, never `jigc unmanage`, at both scopes (`:621`). |
| [storage.md](../../../design/storage.md) | Inc 5: `6ad92379`, `edaa428c` (`:313`); Inc 1: `d07d4df4` (`:268`) | Derived caches: the history-gated downgrade's route is the branch switch at both scopes, and M45's prune route is retired (`:313`). |
| [finalize.md](../../../design/finalize.md) | Inc 1: `d07d4df4` (`:279`), `3b570c20`, `7036d783` (`:265`, `:287`), `0f32c485` (`:194`); Inc 8: `e11f859c` (`:172`) | The code-less census is **sixteen** (`:172`). *The doc-only arm* is the third commit model (`:277`–`:287`): path-scoped, with its left-out guidance and `left-staged` kind, the pre-commit advisory stem and `--dry-run` forecast keyed on `CommitModel`, and the carryover gate skipped at all three positions. `cli::gate_coverage`'s `Respelled` carries the index gate's third spelling, *path scope* (`:265`). |
| [write-commands.md](../../../design/write-commands.md) | Inc 2: `e9f7f081`, `415efc9a`, `fc307600` (`:52`, `:283`) | The create-gate's object form may carry `new: true`, consulted by `doc create` and `doc author` before copy-in (`:283`). `write.title-ignored` over a committed doc routes at a distinct title or `--slug` (`:52`). |
| [workflow-dialect.md](../../../design/workflow-dialect.md) | Inc 2: `a2017524`, `e9f7f081` (`:192`); Inc 3: `197287bf`, `f37d385b` (`:161`, `:268`) | Emitted format: a fan-out sub-task's composed text omits the commit-boundary steps and carries the sub-task trailer (`:161`). On-disk format: the `allows-create` entry's keys are closed at `{type, as, new}`, and an unknown key is a load error (`:192`). |
| [command-output-contract.md](../../../design/command-output-contract.md) | Inc 2: `e9f7f081` (`:246`); Inc 1: `0f32c485` (`:564`, `:566`); Inc 6: `489353a3` | `create.already-exists` joins the instance-scoped `create.*` rows at a doc address (`:246`). |
| [doc-read-surface.md](../../../design/doc-read-surface.md) | Inc 6: `489353a3` (`:66`), `03ecc2ba` (`:67`), `7976d7fa` (`:173`) | `title` on `doc show`'s whole-doc serve (`:66`). `fields` reports the effective value, an absent defaulted field projecting its default (`:67`). `doc list` rows carry `title` and `fields` with their per-state values (`:173`, `:196`). |
| [methodology-docs.md](../../../design/methodology-docs.md) | Inc 7: `9dcb664d` (`:85`); Inc 8: `e11f859c` (`:87`); Inc 10: `7c83a30d` (`:61`) | *The finding doctypes (M55)* names both doctypes and the four workflows in the work-doc surface (`:85`–`:87`). The `cheap-vs-robust` gate row carries S14's clause (`:61`). |
| [pinning.md](../../../implementation/pinning.md) | Inc 7: `9dcb664d` (`:56`) | §3 stays the one home of the `pinned-by:` grammar and names `jigc-feedback`'s field as its second consumer. |
| [doctype-map.md](../../../implementation/doctype-map.md) | Inc 7: `9dcb664d` (`:27`, `:28`, `:102`) | A row per new doctype, each **v1** and free at the freeze (`:27`, `:28`). *Deliberate outs* says why neither is the excluded backlog doctype (`:102`). |
| [worked-examples.md](../../../design/worked-examples.md) | Inc 11: T1 `619252cd` (55), T2 `dac733b1` (56), T3 `d23c04b6` (57), T4 `652bac85` (58), T5 `d76d4ee6` (59) | Flows A–E as chapters 55–59, each with its suite in `g_methodology`. The route edits of Increments 3–5 (`f37d385b`, `2c284c48`, `6ad92379`) moved earlier chapters' text with their rules. |

## The marker counts

**The census set** is the M54 close's: the tracked `*.md` files minus `DECISIONS.md`, `implementation/roadmap.md`, `implementation/project-history.md` and everything under `completions/artifacts/`. **The count** is matching **lines** per file.

```sh
# a committed tree: 62c76009 (the baseline ledger's commit), bea46dd8 (the decomposition), 6acd9d6d (this increment's base)
git grep -c 'M55' <rev> -- '*.md' ':!DECISIONS.md' ':!implementation/roadmap.md' \
  ':!implementation/project-history.md' ':!completions/artifacts/'

# after: the working tree this record is committed with, over the same tracked set
git ls-files -z -- '*.md' ':!DECISIONS.md' ':!implementation/roadmap.md' \
  ':!implementation/project-history.md' ':!completions/artifacts/' \
  | xargs -0 command grep -c 'M55' | command grep -v ':0$'
```

- **`62c76009`**: **24 lines in 7 files**, the planning measurement reproduced.
- **`bea46dd8`**: **72 lines in 18 files**, reproduced. The eleven files new since `62c76009` are the design of record, the eight ideas the Settle parked or annotated, and the two docs the gate-speed PR (#8) measured into.
- **`6acd9d6d`**: **152 lines in 32 files**, reproduced. The fourteen new files are the docs the build revised.
- **after**: **162 lines in 32 files**. The ten added lines are the five chapters T1–T5 wrote into `worked-examples.md`, a heading and a *What it adds* line each. T7's edit to `CLAUDE.md` and T8's three flips change no count, because every flipped line still names M55.

| File | `62c76009` | `bea46dd8` | `6acd9d6d` (base) | after |
|---|--:|--:|--:|--:|
| `CLAUDE.md` | 2 | 3 | 3 | 3 |
| `VISION.md` | 1 | 2 | 2 | 2 |
| `design/command-output-contract.md` | — | — | 3 | 3 |
| `design/doc-read-surface.md` | — | — | 6 | 6 |
| `design/finalize.md` | — | — | 4 | 4 |
| `design/findings-channel.md` | — | 22 | 28 | 28 |
| `design/methodology-docs.md` | — | — | 2 | 2 |
| `design/reconciliation.md` | — | — | 4 | 4 |
| `design/storage.md` | — | — | 3 | 3 |
| `design/team-ready-state.md` | — | — | 1 | 1 |
| `design/validation.md` | — | — | 4 | 4 |
| `design/worked-examples.md` | — | — | 4 | 14 |
| `design/workflow-dialect.md` | — | — | 3 | 3 |
| `design/write-commands.md` | — | — | 2 | 2 |
| `ideas/branches-and-planned-releases.md` | 1 | 1 | 1 | 1 |
| `ideas/cardinality-split-merge-transform.md` | — | 2 | 2 | 2 |
| `ideas/committed-doc-edit-gate.md` | — | 4 | 4 | 4 |
| `ideas/doc-list-field-filter.md` | — | 2 | 2 | 2 |
| `ideas/feedback-web-service.md` | — | 3 | 3 | 3 |
| `ideas/finding-doctype.md` | — | 1 | 1 | 1 |
| `ideas/pack-doctype-visibility.md` | — | 1 | 1 | 1 |
| `ideas/project-authored-doctype-packs.md` | — | 2 | 2 | 2 |
| `ideas/symbol-mention-sweep.md` | 1 | 1 | 1 | 1 |
| `ideas/version-pinned-readme-links.md` | — | 3 | 3 | 3 |
| `implementation/decisions-pending.md` | 16 | 19 | 31 | 31 |
| `implementation/dev-workflow.md` | — | 1 | 1 | 1 |
| `implementation/doctype-authoring.md` | — | — | 18 | 18 |
| `implementation/doctype-map.md` | — | — | 5 | 5 |
| `implementation/machine-setup.md` | — | 2 | 2 | 2 |
| `implementation/milestone-planning-workflow.md` | 1 | 1 | 1 | 1 |
| `implementation/pinning.md` | — | — | 1 | 1 |
| `implementation/release.md` | 2 | 2 | 4 | 4 |
| **Total** | **24 lines, 7 files** | **72 lines, 18 files** | **152 lines, 32 files** | **162 lines, 32 files** |

## The forward-tense markers

```sh
git ls-files -z -- '*.md' ':!DECISIONS.md' ':!implementation/roadmap.md' \
  ':!implementation/project-history.md' ':!completions/artifacts/' \
  | xargs -0 command grep -n -E 'until M55|once M55|when M55 lands|M55 will|owed by M55|built by M55'
```

At the base and after, it matches **one line**, and the record keeps it: `implementation/doctype-authoring.md:91`, *They are listed here because until M55 this file named none of them (D9)*. That is past tense. The checklist below it names every fence, landed at Increments 7 and 8.

The pattern is narrow, so the census also ran the planning's wider sample, adding `M55 adds|ships|makes|generates|fixes|runs|publishes|lands|spends`, *M55's Settle to say / is the / confirms* and *the next publish, M55's*. It matches 19 occurrences on 18 lines, each disposed below. One of them is flipped: `implementation/decisions-pending.md:163` (*What is left for M55's Settle is the confirmation and the close*). The other two flips, `design/findings-channel.md:273` and `:356`, were found by reading every line, not by the sample. The rest are kept for one of two reasons. Some state what the build did in the present tense of a design of record. The others name an act that is still owed after the build: the `1.0.0-rc.23` publish, its crates.io re-read, and the release job's first real run.

## Every line, disposed

**159 kept and 3 flipped**, over the 162 lines of the after tree. *past* means dated or past-tense history that is true today. *design* means the present tense of a design of record, stating a rule that the build made true. The commit or increment named is the one that made it true.

| Line | Disposition | New wording, or the reason it is kept |
|---|---|---|
| `CLAUDE.md:7` | **kept** | current truth T7 wrote (`0217c2a9`): M55 built, not audited; what comes next |
| `CLAUDE.md:38` | **kept** | current rule's example (*M55 → `findings-channel`*) |
| `CLAUDE.md:43` | **kept** | reading-order entry for `findings-channel` (M55), current |
| `VISION.md:220` | **kept** | parked idea, dated 2026-10-01; *for the port to show, through M55's findings channel*. The channel is built, so this is true |
| `VISION.md:221` | **kept** | idea index, *The 2026-10-02 M55-Settle batch*, dated |
| `design/command-output-contract.md:246` | **kept** | design: `create.already-exists` (M55), landed Inc 2 (`e9f7f081`) |
| `design/command-output-contract.md:564` | **kept** | past (*since M55 `left-staged`*), landed Inc 1 (`0f32c485`) |
| `design/command-output-contract.md:566` | **kept** | *The M55 additive kind … (declared as it ships)*. The section's own declaration, with `left-staged` landed at Inc 1. *M55 adds the value* states what shipped |
| `design/doc-read-surface.md:66` | **kept** | design: `title` (additive, M55), landed Inc 6 (`489353a3`) |
| `design/doc-read-surface.md:67` | **kept** | design: effective value (M55), landed Inc 6 (`03ecc2ba`); *before M55* is past |
| `design/doc-read-surface.md:95` | **kept** | *M55 spends the window again … M55 lands the additive `title`*, landed Inc 6. The window's account, true today |
| `design/doc-read-surface.md:173` | **kept** | design: row `title` + `fields` (M55), landed Inc 6 (`7976d7fa`) |
| `design/doc-read-surface.md:196` | **kept** | past (*since M55 `title` and `fields` too*) |
| `design/doc-read-surface.md:200` | **kept** | *Engaged at M55, where it stands*, a past engagement |
| `design/finalize.md:172` | **kept** | past (*four more since M55*), landed Inc 8 (`e11f859c`) |
| `design/finalize.md:194` | **kept** | design: the doc-only model's left-out list (M55), landed Inc 1 |
| `design/finalize.md:265` | **kept** | design: the third spelling (M55), landed Inc 1 (`7036d783`) |
| `design/finalize.md:279` | **kept** | design: *The doc-only arm* (M55), landed Inc 1 (`d07d4df4`) |
| `design/findings-channel.md:1` | **kept** | title of the design of record |
| `design/findings-channel.md:3` | **kept** | head paragraph: *M55 design of record*, *Settled at M55 planning*, dated revisions, citation paths |
| `design/findings-channel.md:7` | **kept** | heading *What M55 settles*: the Settle's table |
| `design/findings-channel.md:22` | **kept** | S11 row: *a seed generated in M55, adopted at M56*. Generated at Inc 9 (`af2bb8f8`), and the adoption is keyed to M56 |
| `design/findings-channel.md:27` | **kept** | S16 row: M55's partial re-review axes, still owed on the release candidate |
| `design/findings-channel.md:32` | **kept** | the mechanism line, the Settle's decision (*M55's charter inherited it*; *M55 admits these*) |
| `design/findings-channel.md:95` | **kept** | grammar example (`review:M55-completion/F3`), illustrative |
| `design/findings-channel.md:123` | **kept** | citation paths (gap-list, gate-record row 3); the fences were moved at Incs 7 and 8 |
| `design/findings-channel.md:191` | **kept** | past (*M55 Increment 2 planning, P3*) |
| `design/findings-channel.md:193` | **kept** | design: *M55 makes the entry strict*, landed Inc 2 (`a2017524`) |
| `design/findings-channel.md:243` | **kept** | past (*P1, M55 Increment 4*) |
| `design/findings-channel.md:245` | **kept** | past (*settled at M55 Increment 3*) |
| `design/findings-channel.md:253` | **kept** | design: *M55 generates the seed … M56 adopts it*. Generated at Inc 9 |
| `design/findings-channel.md:255` | **kept** | design: the seed increment *drives the M55 build in a rig*, which Inc 9 did; seed-ledger citation |
| `design/findings-channel.md:265` | **kept** | source row, citation of seed-ledger.md |
| `design/findings-channel.md:267` | **kept** | source row, citation of planning-findings.md |
| `design/findings-channel.md:271` | **kept** | the seed's rules (*re-driven against the M55 build*), followed at Inc 9; label examples |
| `design/findings-channel.md:273` | **flipped** | *`DECISIONS.md`'s 2026-09-27 "go to the ledger M55 mints" gets a dated correction to "the seed M56 adopts" at the planning commit.* → *… **got** a dated correction to "the seed M56 adopts" at the planning commit (Corrected 2026-10-02, in that entry).* The correction is in `DECISIONS.md`'s 2026-09-27 entry |
| `design/findings-channel.md:318` | **kept** | flow B table header (*Report finalize, after M55*) |
| `design/findings-channel.md:332` | **kept** | heading *Around M55* |
| `design/findings-channel.md:335` | **kept** | still owed after the build: rc.23 through release PR #2 (CLAUDE.md → Project state) |
| `design/findings-channel.md:340` | **kept** | still owed: the partial re-review on the release candidate carrying M54 and M55 |
| `design/findings-channel.md:342` | **kept** | axis table header |
| `design/findings-channel.md:356` | **flipped** | *This design is their pointer until then, never their replacement.* → *This design **was** their pointer until then, never their replacement: all twelve were revised by their increments, confirmed at the close ([close-census.md] → §13).* |
| `design/findings-channel.md:365` | **kept** | §13 row text (*an M55 revision on the `create-gates:` line's mold*), revised at Inc 3 |
| `design/findings-channel.md:379` | **kept** | citation path (settle-log.md) |
| `design/findings-channel.md:381` | **kept** | past (*Settled at M55 Increment 1 (T4)*) |
| `design/findings-channel.md:382` | **kept** | past (*Settled at M55 Increment 3*) |
| `design/methodology-docs.md:85` | **kept** | heading *The finding doctypes (M55)* |
| `design/methodology-docs.md:87` | **kept** | design: *M55 adds two one-per-doc doctypes*, landed Incs 7 and 8 |
| `design/reconciliation.md:34` | **kept** | design: the absorb cell (M55), landed Inc 4 (`8b4f012f`) |
| `design/reconciliation.md:77` | **kept** | design: *A pulled edit is not a conflict (M55, L1)*, landed Inc 4 |
| `design/reconciliation.md:130` | **kept** | design (M55, L2), landed Inc 5; *Until M55* is past |
| `design/reconciliation.md:188` | **kept** | past (*M55 at store scope*) |
| `design/storage.md:268` | **kept** | design: the doc-only finalize leaves the version stamp (M55), landed Inc 1 |
| `design/storage.md:313` | **kept** | past (*the store sweep since M55*; *M55 retired it*), landed Inc 5 |
| `design/storage.md:315` | **kept** | the live-record case; *M55 branch-switch route* names today's route |
| `design/team-ready-state.md:213` | **kept** | past (*`title` M55* in the witness enumeration) |
| `design/validation.md:367` | **kept** | design: the lag arm (M55, L1), landed Inc 4 |
| `design/validation.md:386` | **kept** | design: the rename's blocking arm (M55), landed Inc 5 |
| `design/validation.md:524` | **kept** | design: the store-scope advisory (*the baseline lags `HEAD`, M55*) |
| `design/validation.md:621` | **kept** | dated brackets *[M55: the route was …]* and *[M55: the grading holds at both scopes]*, landed Inc 5 |
| `design/worked-examples.md:2732` | **kept** | past (*M42–M54 … since M55*) |
| `design/worked-examples.md:2781` | **kept** | past (*Since M55 (L1's store arm …)*) |
| `design/worked-examples.md:2984` | **kept** | past (*M45–M54 … since M55*) |
| `design/worked-examples.md:2997` | **kept** | past (*M55: M45's prune route retired*) |
| `design/worked-examples.md:3782` | **kept** | chapter 55 heading (M55), T1 |
| `design/worked-examples.md:3786` | **kept** | past (*M55 Increment 8*), T1 |
| `design/worked-examples.md:3838` | **kept** | chapter 56 heading, T2 |
| `design/worked-examples.md:3848` | **kept** | past (*M55 Increment 1*), T2 |
| `design/worked-examples.md:3893` | **kept** | chapter 57 heading, T3 |
| `design/worked-examples.md:3897` | **kept** | past (*M55 Increment 2*), T3 |
| `design/worked-examples.md:3941` | **kept** | chapter 58 heading, T4 |
| `design/worked-examples.md:3945` | **kept** | past (*M55 Increments 8 and 3*), T4 |
| `design/worked-examples.md:3991` | **kept** | chapter 59 heading, T5 |
| `design/worked-examples.md:3995` | **kept** | past (*M55 Increments 4 and 5*), T5 |
| `design/workflow-dialect.md:161` | **kept** | design: *Composing for a fan-out sub-task (M55)*, landed Inc 3 |
| `design/workflow-dialect.md:192` | **kept** | design: *An `allows-create` entry's keys are closed (M55)*, landed Inc 2 |
| `design/workflow-dialect.md:268` | **kept** | design (sub-task composition; M55), landed Inc 3 |
| `design/write-commands.md:52` | **kept** | design: `write.title-ignored` over a committed doc (M55, F3), landed Inc 2 |
| `design/write-commands.md:283` | **kept** | design: *Create-only — the `new` key (M55)*, landed Inc 2 |
| `ideas/branches-and-planned-releases.md:22` | **kept** | parked; *reports through M55's findings channel* is the port's future, over a channel that is built |
| `ideas/cardinality-split-merge-transform.md:3` | **kept** | status line, *Parked by the M55 Settle*, dated |
| `ideas/cardinality-split-merge-transform.md:7` | **kept** | past (*M55 met it head-on*) |
| `ideas/committed-doc-edit-gate.md:3` | **kept** | status line, dated |
| `ideas/committed-doc-edit-gate.md:7` | **kept** | citation of the M55 baseline (C7, F14) |
| `ideas/committed-doc-edit-gate.md:9` | **kept** | past (*M55 met it*) |
| `ideas/committed-doc-edit-gate.md:17` | **kept** | *recorded as a `jigc-feedback` row in the M55 seed*: true, ledger row `m55-f14` → `seed/jigc-feedback/any-task-can-edit-any.md` |
| `ideas/doc-list-field-filter.md:3` | **kept** | status line, dated |
| `ideas/doc-list-field-filter.md:7` | **kept** | *M55 makes … by adding `title` and the header `fields`*, landed Inc 6 |
| `ideas/feedback-web-service.md:3` | **kept** | status line, dated |
| `ideas/feedback-web-service.md:7` | **kept** | *M55 ships the `jigc-feedback` doctype*, landed Inc 7 |
| `ideas/feedback-web-service.md:26` | **kept** | past (*No schema change was made for this at M55*) |
| `ideas/finding-doctype.md:5` | **kept** | dated head note (*2026-10-02*); *shipped at M55* is true since Inc 7 |
| `ideas/pack-doctype-visibility.md:17` | **kept** | dated addendum; *M55 ships a second jigc-only doctype*, landed Inc 7 |
| `ideas/project-authored-doctype-packs.md:3` | **kept** | status line, dated |
| `ideas/project-authored-doctype-packs.md:7` | **kept** | past (*the M55 planning measured*) |
| `ideas/symbol-mention-sweep.md:31` | **kept** | the port's future, through *the findings channel M55 ships*, which is built |
| `ideas/version-pinned-readme-links.md:3` | **kept** | status line, dated |
| `ideas/version-pinned-readme-links.md:7` | **kept** | *M55 fixes … by generating a committed crate README*: the generator landed at Inc 10 (`6cc95d82`, `390d7e28`). The page changes from the first version published after M55 (`release.md:121`) |
| `ideas/version-pinned-readme-links.md:21` | **kept** | *the post-publish 200-check M55 runs once*: still owed after the build, keyed to the `1.0.0-rc.23` publish (`decisions-pending.md:45`). The line describes what the parked idea would generalize |
| `implementation/decisions-pending.md:29` | **kept** | section heading naming the road's four milestones |
| `implementation/decisions-pending.md:31` | **kept** | pointer to the roadmap's charters |
| `implementation/decisions-pending.md:37` | **kept** | graduated M54 row, kept as recorded. Its *M55 publishes the next* is still owed after the build: rc.23 through release PR #2 |
| `implementation/decisions-pending.md:38` | **kept** | graduated M54 row, as recorded (*why M54 is ordered before M55*) |
| `implementation/decisions-pending.md:45` | **kept** | graduated at M55's Settle; *BUILT 2026-10-03 (M55 Increment 10)*; the re-read stays owed, keyed to rc.23 |
| `implementation/decisions-pending.md:52` | **kept** | section head: *All six rows graduated 2026-10-02 at M55's Settle* |
| `implementation/decisions-pending.md:54` | **kept** | graduated row (S1); *Trigger: M55's Settle* as recorded |
| `implementation/decisions-pending.md:55` | **kept** | graduated row (S2), as recorded |
| `implementation/decisions-pending.md:56` | **kept** | graduated row (S3), as recorded |
| `implementation/decisions-pending.md:57` | **kept** | graduated row (S4). The M54 note's *M55's Settle to say* is answered by the row's own head bracket, which states the Settle's reading |
| `implementation/decisions-pending.md:58` | **kept** | graduated row (S5), as recorded |
| `implementation/decisions-pending.md:59` | **kept** | graduated row (S6), as recorded |
| `implementation/decisions-pending.md:64` | **kept** | dated note (*Noted 2026-10-02 (M55 planning)*); the trigger stays M56 |
| `implementation/decisions-pending.md:65` | **kept** | M56 row; *the channel M55 ships* is built |
| `implementation/decisions-pending.md:67` | **kept** | M56 row; *M55's channel* is built |
| `implementation/decisions-pending.md:68` | **kept** | dated M56 row; *M55 generates … and commits them … under `completions/artifacts/M55/seed/`*, done at Inc 9 (`af2bb8f8`); the adoption stays keyed to M56 |
| `implementation/decisions-pending.md:81` | **kept** | seeded pointer (Inc 9) |
| `implementation/decisions-pending.md:84` | **kept** | seeded pointer (Inc 9) |
| `implementation/decisions-pending.md:85` | **kept** | seeded pointer (Inc 9) |
| `implementation/decisions-pending.md:105` | **kept** | dated brackets (*Superseded 2026-09-27*, *Settled differently 2026-10-02 (M55 Settle …)*), kept as the record |
| `implementation/decisions-pending.md:163` | **flipped** | The M54 note's *M55 must not seed this row … its Settle confirms the landed line and closes it* and *What is left for M55's Settle is the confirmation and the close* are kept as recorded. Appended: ***[Confirmed and closed 2026-10-03 (M55 close, Increment 11 T8): M55's Settle recorded neither act, so the close records both. The row was not seeded as open — the seed's sources take no M51 row ([seed-ledger.md] → its source table) — and `crates/cli/guides/QUICKSTART.md` → Install still carries the landed line, held there by `install_line.rs`.]*** |
| `implementation/decisions-pending.md:406` | **kept** | settled D3: the blind trial on the release candidate carrying M54 and M55, still owed |
| `implementation/decisions-pending.md:407` | **kept** | seeded pointer (Inc 9) |
| `implementation/decisions-pending.md:408` | **kept** | seeded pointer (Inc 9) |
| `implementation/decisions-pending.md:449` | **kept** | past (*Seeded 2026-10-03 (M55 Increment 9)*) |
| `implementation/decisions-pending.md:451` | **kept** | seeded pointer (Inc 9) |
| `implementation/decisions-pending.md:453` | **kept** | seeded pointer (Inc 9) |
| `implementation/decisions-pending.md:455` | **kept** | seeded pointer (Inc 9) |
| `implementation/decisions-pending.md:457` | **kept** | seeded pointer (Inc 9) |
| `implementation/decisions-pending.md:459` | **kept** | seeded pointer (Inc 9) |
| `implementation/decisions-pending.md:461` | **kept** | seeded pointer (Inc 9) |
| `implementation/dev-workflow.md:46` | **kept** | citation path (gate-speed-measurement.md) |
| `implementation/doctype-authoring.md:22` | **kept** | past (*`jigc-feedback` and `inconsistency`, M55, the latest*) |
| `implementation/doctype-authoring.md:64` | **kept** | past (*the M55 `inconsistency.sides` precedent*) |
| `implementation/doctype-authoring.md:91` | **kept** | marker *until M55*: past tense (*until M55 this file named none of them*), and the measured reds of Incs 7 and 8 follow it |
| `implementation/doctype-authoring.md:98` | **kept** | past (*19 schemas + 6 snapshots at M55*) |
| `implementation/doctype-authoring.md:100` | **kept** | past (counts *at M55*) |
| `implementation/doctype-authoring.md:103` | **kept** | past (*19 shipped schemas dedup to 18 at M55*) |
| `implementation/doctype-authoring.md:104` | **kept** | past (*M55's two doctypes added exactly 24 goldens*) |
| `implementation/doctype-authoring.md:108` | **kept** | past (*At M55 that was Increment 8*) |
| `implementation/doctype-authoring.md:110` | **kept** | past (*35 before M55's four*) |
| `implementation/doctype-authoring.md:112` | **kept** | past (*At M55 that was 28 goldens*; *M55's four workflows … added exactly 48*) |
| `implementation/doctype-authoring.md:113` | **kept** | current (*M55's four findings workflows are its members*) |
| `implementation/doctype-authoring.md:114` | **kept** | past (*twelve before M55's four*) |
| `implementation/doctype-authoring.md:118` | **kept** | past (*That is how M55's spike went red*) |
| `implementation/doctype-authoring.md:125` | **kept** | past (*M55 Increment 8*) |
| `implementation/doctype-authoring.md:129` | **kept** | past (*M55 Increment 8*) |
| `implementation/doctype-authoring.md:133` | **kept** | past (*M55 Increment 7, T2*) |
| `implementation/doctype-authoring.md:134` | **kept** | past (*from Increment 7 until Increment 8 shipped the report workflows*) |
| `implementation/doctype-authoring.md:136` | **kept** | past (*thirteen schemas, twelve persisted at M55*) |
| `implementation/doctype-map.md:27` | **kept** | the `jigc-feedback` row (M55, v1), landed Inc 7 |
| `implementation/doctype-map.md:28` | **kept** | the `inconsistency` row (M55, v1), landed Inc 7 |
| `implementation/doctype-map.md:44` | **kept** | past (*minted new at M55*) |
| `implementation/doctype-map.md:55` | **kept** | past (*Landed M55*) |
| `implementation/doctype-map.md:102` | **kept** | current (*Deliberate outs*: the M55 finding doctypes are not the backlog) |
| `implementation/machine-setup.md:27` | **kept** | dated measurement (2026-10-02), citation path |
| `implementation/machine-setup.md:33` | **kept** | dated measurement (2026-10-02), citation path |
| `implementation/milestone-planning-workflow.md:57` | **kept** | the slug rule's example (*M55 → `findings-channel`*) |
| `implementation/pinning.md:56` | **kept** | *A second consumer of the `pinned-by:` grammar (M55)*, landed Inc 7 |
| `implementation/release.md:67` | **kept** | *M55 publishes the next*: still owed after the build. rc.23 publishes through release PR #2 after M55's merge (CLAUDE.md → Project state) |
| `implementation/release.md:108` | **kept** | *The reshaped `release` job's first real run is the next publish, M55's. Until that run …*: that run has not happened, so the line is still true |
| `implementation/release.md:121` | **kept** | past (*since 2026-10-03 (M55 Increment 10)*; *settled by M55*); *crates.io shows it from the first version published after M55* is a true conditional |
| `implementation/release.md:131` | **kept** | past (*since M55 Increment 10*) |

## Beyond the grep

The design of record keeps its design voice on lines that do not name `M55`: *the build adds a multi-path commit helper* (`findings-channel.md:166`), *the seed increment runs the full set* (`:328`), and the open questions' *the finalize increment's call* and *the S2 increment's to settle* (`:381`, `:382`). Each states a rule the build made true. The two questions carry their own *Settled at M55 Increment …* brackets, which is how the design says an item has been built ([findings-channel.md](../../../design/findings-channel.md) → Open questions). The remaining forward lines, *After rc.23 publishes, its crates.io page is re-read* (`:284`) and the `rc.23` examples (`:97`), name acts and versions that are still owed or illustrative. No line was flipped beyond the three above.

## Bounds

- **The pattern is the literal `M55`.** A line about M55 that names only an increment, a settle id (S1–S17) or `findings-channel` is not counted. The section above lists the ones found by searching for `Increment 11`, `rc.23`, *the build adds*, *the seed increment* and *the … increment's*. It is not a full sweep of the S-ids.
- **A kept line is judged true today, not re-proved here.** Each reason names the increment or commit it rests on. The facts behind them are the increments' own records in `DECISIONS.md` and this directory. The §13 confirmation reads each row's rule in its doc as it stands, without re-driving the binary. The flows 55–59 and the increment suites are what drive it.
