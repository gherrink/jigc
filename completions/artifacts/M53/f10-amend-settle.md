# F-10 — `jigc task amend`: the settled shape

**Decided by the human 2026-09-23**, bending the pre-1.0 *no new capability* rule once, on the
rc.14 trial's datum: a worker who finalized with a wrong summary found no jigc path and rewrote the
commit with raw `git commit --amend` — the trial's only adapter bypass. Baseline:
[f10-amend-baseline.md](f10-amend-baseline.md) (every fact driven on the installed `1.0.0-rc.19`).
Shape **(D)** of the five the baseline priced: **re-author, never parse.**

## The claim

*An agent that landed a finalize with a wrong commit message repairs it through jigc, authoring
the new message as a commit doc jigc renders — never by typing `<type>(<scope>): <summary>` itself
— and the repair cannot fold unrelated staged work into the rewritten commit.*

## The two doors, and what each does

1. **`jigc task amend`** — a **mint** door, `VERB_KINDS`/`BEHALF_DOORS` **`Neither`** (it commits
   nothing). Mints a transient task area whose `base.json` pins **HEAD**, workflow `amend`
   (a tiny dev-pack workflow whose one step solicits the commit doc and states the read-back —
   the M48 fence applies), and provisions an **empty** `commit` doc at `commit:<amend-task-id>`.
   The agent authors it through the ordinary `set-field` / `set-slot` / `add-item` verbs and reads
   it back with `doc show … --task <id>`. Joins **`MINT_DOORS`** (`Snapshot::Written`). Subject to
   the single-active-task default and `jigc start`'s `also open:` block like any task.
   Optional `"<intent>"` positional for the task's intent line; the id mints from it, else from
   HEAD's short sha (`amend-<sha7>`).
2. **`jigc task finalize <id>`** over an amend task takes finalize's **second commit model**
   (the `milestone finalize` two-arm precedent): posture → gate (the commit doc's conformance,
   as today) → render → **`git commit --amend -F <msg>`** with the tree untouched → post-commit
   teardown (the area goes, as every task's does). `COMMITTING_DOORS` 10→**11** (the amend arm of
   `task finalize`, its own `commits:` help clause, its own error identity
   `finalize.amend-rejected` in `ERROR_CODE_REGISTRY` 11→**12** — the `surface-contract.md` mirror
   row and its count word move with it; the nine `count_fences.rs` prose homes move). The
   survivable hook-rejection frame's state-truth clause for this arm: **HEAD is unchanged**
   (driven: a rejected amend leaves HEAD byte-identical).

An amend task is recognised by a marker file in the area — **`amend`**, a `TASK_AREA_FILES`
member (14→15, the writer-count fence in both crates) holding the sha it pins.

## Refusals, each with a shipped or named identity and a route

| cell | answer |
|---|---|
| repository posture (any `InProgress` member, detached, unborn) | ~~`repo.operation-in-progress` / `head-*` — **transfers unchanged** at both doors (`amend` is a mint door; `finalize`'s arm is commit-on-behalf)~~ **[Corrected 2026-09-26 (the F-10 review's LOW-7):** *transfers unchanged at both doors* reads as *both doors refuse*, and the mint door refuses **nothing** — driven, `jigc task amend` mints at exit 0 under all eight members. That is correct behaviour, not a gap: `jigc task amend` is `BEHALF_DOORS`' `Neither`, so it adjudicates no member of the family, exactly as `jigc start` does. What *transfers* is the family's own rule — a door refuses the posture iff it acts on the repository's behalf — and under that rule the mint door is silent and the finalize arm refuses every member. [design/finalize.md](../../../design/finalize.md) → The amend arm records this correctly (*"the mint door is `BEHALF_DOORS`' `Neither` and adjudicates none of it"*); the settle row was the stale one.**]** |
| **a non-empty index at finalize** — `git diff --cached --quiet HEAD` false | **refuse**, blocking, at the finalize arm: `finalize.amend-index-dirty` — *the amend would fold N staged path(s) into the rewritten commit* — route `git restore --staged -- <paths>` (aimed through `git_at`) or `git stash push --staged`. Driven hazard, a data-loss cell; not overridable (no `--carry-staged` on this arm — stated in help). The carryover gate is **exempt** for amend tasks with the reason stated at its row: the amend stages nothing, and the dirty-index refusal is its stricter replacement. |
| HEAD is a **root** commit or a **merge** commit | refuse at `task amend`: `amend.head-shape` — *amend rewrites a single-parent commit; HEAD is <root/merge>* — route naming `git commit --amend` by hand for a root, nothing for a merge. Blocking. |
| **HEAD moved between `amend` and `finalize`** (the pin no longer equals HEAD) | refuse at the finalize arm: **`finalize.base-mismatch`** — the shipped code, its existing route re-worded for this arm (*HEAD is no longer the commit this amend was minted against*), plus `jigc task discard <id> --force`. |
| **which door made HEAD** — a boundary commit under `squash: true`, a record-only commit, a foreign commit | **jigc cannot tell** (driven: no trailer, no persisted sha, task ids re-mintable). So the verb's name and help say what is true: *amend HEAD, whatever HEAD is*. The composed step names the three shapes an agent should not re-author (a milestone boundary's structural message; a record-only bookkeeping commit; a commit jigc did not make) and prints HEAD's current subject line in the mint ack so the agent sees what it is about to replace. No refusal — a refusal would need a discriminator that does not exist, and inventing one (matching jigc's own message format) is the pattern-match M53 refused elsewhere. |
| pushed status | **undetectable in practice** (driven). The mint ack carries an advisory sentence, not a code: *if this commit has been pushed, amending it rewrites shared history*. Stated in help as advice, never as a fence. |
| `squash: false` chain — HEAD is one of N | amends HEAD; the composed step says so. |
| a promoted doc in HEAD's tree | message-only by construction: the tree is untouched, `file-state.json` stays valid (driven across three amends). **Content amend is not built** — stated in help. |
| amend twice | each amend is a fresh mint against the new HEAD; the first area is torn down at its finalize. |
| an open task at amend time | the single-active-task default; `also open:` reports it. |
| inside a fan-out worktree | posture refuses detached HEAD at the commit arm (shipped). `task amend` from a worktree: `jigc_home` binding per the cwd arc; the amend pins the **standing checkout's** HEAD (the C2-09 rule: `task finalize` commits where you stand) and the ack names the checkout when it is not the workbench's. **[Made real 2026-09-26 (the F-10 review's LOW-7):** the trailing clause was **not real** as built — driven from an attached linked worktree, the mint ack named no checkout at all, and the `committed / would commit in the linked worktree at …` line appeared only on the forecast and the landed ack, after the fact. It was **built rather than struck**, because the fact is this door's own: the reader is about to have *that* worktree's `HEAD` rewritten while the roster and workbench they reached the door through belong to the main checkout. `AmendTarget.checkout` now rides the mint — and the mint only, since *which* checkout was pinned is a fact about the pinning invocation and the marker records the sha alone — computed by the same producer the finalize surfaces use (`render::CommitSite::differing`) and rendered as its own sentence, because that producer speaks in a commit tense and this door has committed nothing.**]** |
| `--dry-run` | forecasts *would rewrite <sha7> "<subject>" → "<new subject>"*, the dirty-index probe included; `task validate` previews the same. |

## Envelopes (the pre-pin window is open — declared as it ships, in its own paragraph)

- `jigc task amend` → the composed `{task, text}` arm (§1 of `command-output-contract.md`, unchanged shape); ~~`--format json` carries the pinned sha under `text`, no new key needed — **verify**; if a key is needed it is declared in the additive-key paragraph.~~ **[Corrected 2026-09-27 (the rc.20 per-axis review's `(2, A2-3)` = `(3, F-D)`):** the row ended in *— **verify***, the verification came back **negative**, and it was acted on — but this row was never struck. Driven on `1.0.0-rc.20`: `jigc --format json task amend "json probe"` exits 0 with keys `['task', 'text']`, and `\b[0-9a-f]{7,40}\b` matches **nothing** over `text` or over the whole envelope; the text arm of the same door carries the short sha once, in its `amending:` header. **The binary is right and this row is the false statement.** What was acted on is recorded in full at the MEDIUM-2 fix commit `c65d3495`, and the authority is a code-side census row in `crates/cli/tests/text_json_parity_axis.rs` — `Disposition::DeclaredOut` with a three-clause reason: the `amending:` block is presentation, the sha and subject are facts about the **repository** that any driver reads from `git log -1 HEAD`, and the one fact that is jigc's own the driver already has, because it typed the verb. So the JSON arm carries **no sha anywhere**, by declaration, and needs none. Rows 43 and 53 of the table above took dated correction brackets from the same review pass; this row did not, which is the whole of the finding.**]
- `jigc task finalize` (amend arm) → the landed envelope with `committed.commits[0].hash` = the new sha and an **`amended`** key carrying the superseded sha — one additive key, declared. `ENVELOPE_ARMS` 64→66 with both arms' key sets pinned; `text_json_parity_axis` and `format_json_success_axis` cover the new leaf.

## What does not move

`ROLLBACK_POPULATIONS` (+0 — a rejected amend leaves HEAD and the tree as they were, nothing promoted) · `DESTROYING_DOORS` (+0 — no path removed; the superseded commit stays in the reflog, named in the ack) · `WORK_UNIT_ID_DOORS` (+0 — `task amend` takes no id; `task finalize` has its row) · every schema-hash, `schema-version`, corpus, `contract-version` (the `commit` doctype is reused as-is) · the adapter deny floor (unchanged; `git commit --amend` stays un-denied — the product now offers the path, it does not police the alternative).

## Guides and record

`AGENT.md`/`SKILL.md`/`QUICKSTART.md`: one paragraph — *a landed commit's message is repaired with `jigc task amend`* — **one batch, one hash move**; `design/finalize.md` gains the amend arm as a section under the commit models; `design/write-commands.md` → Task origination gains the door; `command-output-contract.md` the two declared arms; the workflow catalog the `amend` workflow with its `when:`/`description:`.

## Acceptance

One arm through the real binary iterating **the refusal table above as a set**: every row driven at both doors × `{text, --format json}`, the hook-rejected cell (HEAD byte-identical, the frame's clause), the dirty-index cell (a planted staged file; HEAD's tree unchanged after the refusal), the moved-HEAD cell, the root/merge cells, the happy path (new sha, old sha in `amended`, tree byte-identical to the superseded commit's, message rendered from the doc), a promoted-doc control (file-state valid before and after), and a fan-out-worktree control.
