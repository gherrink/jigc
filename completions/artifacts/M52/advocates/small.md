# M52 Settle — robust-advocate, the five smaller forks

One-sided by charter. Everything below driven on `~/.local/bin/jigc` = `1.0.0-rc.15`, rigs from
`dev/jigc-rig … --binary`, roots under the scratchpad, **nothing written into the working repo**.
I settle nothing.

---

## (i) C-2 — the root-placement orphan

**Cheap:** restate residual 2 as still-declared, with the amplifier written into the declaration.
**Robust:** re-take it — widen `orphan::Territory` to the root-placement homes.

**My case — neither, and the seam the record lacks.** First, the residual's *population* is false.
Driven on the shipped pack, `committed-singletons`, no manufactured pack, no `JIGC_PACK_DIR`:

- foreign `api-notes.md` at the repo root carrying `schema-version: 3` → `validate` **exit 0**, silent
  (M51's narrowing **holds** — the HIGH stays closed);
- the identical bytes at `docs/api-notes.md` → **exit 1**, blocking `schema-conformance.orphaned-instance`;
- `git mv CHANGELOG.md HISTORY.md` + commit — one ordinary human act on a stock corpus — then
  `find .jigc/state -mindepth 1 -delete` (**the fresh-clone shape**: `.jigc/state/` is gitignored, so
  this is what every clone and every CI runner has) → `jigc validate` **exit 0**,
  *"4 finding(s) — report-only at store scope (exit 0)"*, `CHANGELOG.md` and `HISTORY.md` named on **no**
  surface, and `jigc doc list` drops `changelog` **entirely** — three rows, not even an `orphaned` one.
  With the cache present it exits 1 on `reconciliation.rename`. So the only thing between an adopter and
  a green CI over a lost managed document is a gitignored cache that does not survive `git clone`.

`validation.md:710` frames residual 2 as *"a departed doctype homed like `VISION.md`"* — needing
`JIGC_PACK_DIR` or PB-1. Driven, it fires on a `git mv`. The cheap cut would re-declare a residual whose
own premise is falsified — the exact shape (a locked doc asserting a bound the binary disproves) this
wave's razor leg 1 exists to catch, and the shape M51's completion audit already struck twice.

**Does widening re-open the M51 HIGH? Not if you widen the right thing — and the naive widening is not
the fix anyway.** `HISTORY.md` is at no declared home, so an exact-file territory (`{CHANGELOG.md,
VISION.md}`) misses it too; only a whole-repo subject reaches it, which *is* the HIGH. The question the
product should be asking is not *what is this file* but *a home jigc committed into is now empty*, and
that is answerable from **git history**, which survives clone. Driven both directions:
`git log HEAD -1 -- CHANGELOG.md` → `d169ca3 rename` with `git ls-files -- CHANGELOG.md` empty (fires);
on a stock `fresh` repo where `CHANGELOG.md` never existed → no history, `validate` **exit 0**, correctly
silent. That predicate is **already shipped** — M45's file-state history gate is `git log HEAD -1 -- <path>`
verbatim — it names **one exact path per declared home**, so no team document anywhere can be its subject,
and `orphan::Territory` is not touched by one byte. The HIGH cannot re-open through a rule that never
enumerates a directory.

**What the cheap cut costs.** Fork 4's tier-0 fix repairs `migrate-corpus` and leaves `validate` green:
`schema-conformance.schema-version-current` iterates `committed_instances` over **resolved** schemas, and
the stranded file sits at no resolved home. So the adopter who does not run `migrate-corpus` still gets a
clean CI over a doc that is gone — **the amplifier survives its own tier-0 fix**, which is the one thing
the charter says this row exists to decide.

**Robust scope:** a declared-home emptiness check keyed at each doctype's resolved home, gated on git
history, joining `STORE_EXIT_FLIPS` (its 7th member — an adopter-visible exit change, to be priced).
Reuses one shipped predicate; no territory change; no schema hash.

**Honest concession.** The *departed-doctype* half (L-5: `placement-root: .`, pack drops, three stamped
orphans at root) is **not** reachable by the history oracle either — nothing declares the home once the
doctype is gone. That half genuinely needs the namespaced stamp, and the cheap cut is **right for it**.
Split the residual on that seam; do not re-take it whole.

**Verdict: robust-now** for the declared-home half (an ordinary `git mv` false-greens CI on a fresh clone,
and fork 4 does not fix it); **cheap-cut-is-correct** for the departed-doctype half.

---

## (ii) Tier-0 scope against the latent set

**Cheap:** tier 0 = the 12 charter rows; latents ride classes or defer with triggers.
**Robust:** admit by the charter's own predicate, whatever instrument found the cell.

**LD-3 driven** (`dev/jigc-rig bare`, no `jigc setup` anywhere):
`milestone create` → exit **0** · `add-task` → exit **0** · `provision` → exit **0**, and
`git worktree list` shows a **real detached worktree** at `.jigc/worktrees/do-the-thing`. `git log` holds
**only** `initial` — **no record commit**. `git check-ignore -v .jigc/milestones/probe-milestone/base.json`
→ `.jigc/.gitignore:4:milestones/`. So the milestone exists **only** in the gitignored workbench: M39's
load-bearing settlement — *the committed `.md` is the source of truth, the `.jigc` JSON a rebuildable
cache* — is **inverted**. It is not "a milestone without setup"; it is a milestone that does not exist in
the repository and cannot be continued from a fresh clone, produced at exit 0 by three doors, one of
which (`provision`) writes `.git/worktrees/`. `milestone discard` then acks *"workbench removed"* at exit 0.

**And the predicate is already in the binary:** `jigc milestone execute probe-milestone` → **exit 1**,
*"this project isn't set up — run `jigc setup`"*. One of nine doors knows the rule. That is verbatim M45's
complete-fix shape — *a fence applied where its wave pointed is not applied at all* — and the charter's own
standing rules make it binding.

**The admission argument.** The charter's tier-0 predicate is *exit-0 loss or repository harm through a
committing, destroying or moving door*. `provision` mutates `.git/` on the user's behalf; `create`/`add-task`
manufacture unrecoverable state. The 12 rows came from a **review** with stated coverage bounds; the
baseline is the **class** instrument the charter itself commissioned precisely because M51's audit widened
every class it was handed. A predicate that admits a cell only when a review row happened to name it is
not a predicate — it is a list, and admitting only the list is the M34 shape this gate exists to stop.
Read the human's own ground literally: *"the 1.0.0 call is not taken over exit-0 loss behind a committing
door."* If that is about the **condition**, these are in. If it is about the **provenance of the row**, the
sentence means *loss a review found* — and the next review finds the next one, which is the
five-consecutive-wave signature already on this record.

**Robust scope — not "admit all 20", admit the four the baseline names as outside every charted class,
each small:** LD-3 = lift `milestone execute`'s existing predicate to a door class on the shipped
`BEHALF_DOORS`/`WORK_UNIT_ID_DOORS` mold · L-1 = flip `leftover_at`'s error arm from `Absent` to the
fail-closed verdict **its own doc-comment already claims** · L-2 = give `workbench_paths`' name match the
shape check its doc-comment records M49 teaching it · `rename --slug` on a placement head rides fork 5's
seam at ~zero marginal cost.

**Honest concession.** The **surface tier** genuinely defers: §2 row 20's batch (fixed ack strings, `""`
roots acked `= .`, `x/../y`), LD-2, LD-4 and `staged_doc_ids` are false-green or prose, not loss, and
admitting them is scope inflation the razor should refuse on leg 0. The cheap cut is right for those.

**Verdict: robust-now** for the four loss/harm cells (the predicate must bind on the condition, not on
which instrument found it); **cheap-cut-is-correct** for the surface-tier latents.

---

## (iii) `task validate`'s posture preview

**Cheap:** leave `task validate` silent (M47 D1 — a preview must not flip an exit).
**Robust:** a `GATE_COVERAGE` family row + the probe.

**Driven**, `committed-singletons`, open task with staged code, under `git merge --no-ff --no-commit`:
`jigc task validate <id>` → **exit 0**, prints only the changelog advisory, says nothing about the merge.
`jigc task finalize <id>` → **exit 1** `blocking · repo.operation-in-progress`. And the datum the fork does
not carry: **`jigc task finalize <id> --dry-run` → exit 1, the identical refusal.**

That kills the cheap cut's ground. The product **already ships two previews of the same door and they
disagree**; `task finalize --dry-run` is a preview and it flips. So keeping `task validate` silent
preserves no rule — it preserves an inconsistency, and M47 D1's premise (*a preview must not flip an
exit*) is not a rule this binary obeys.

M47 Increment 3's own claim is what is false: *"`jigc task validate` previews what finalize gates on"*,
stated on nine promise surfaces and scoped to the truth **by M47 itself**. `GATE_COVERAGE` has 12 rows and
`grep -in 'posture\|repo\.'` → **0**. The claim was true at M47 and became false at M51, when the posture
family was *built* and joined `finalize` while nobody joined the preview. Repairing that is not new
capability — it is M51's own fix reaching its declared preview surface, inside this wave's boundary.

The flip is **declared twice** already: `DECISIONS.md:2621` bound (iii) — *"the exit-0 → non-zero flip at
`task validate ""` … is intended"* — so a declared flip here is a **revise** of D1's ground, not an
override; and `DEFECT C` (tier 1, already in the wave) is the row that reconciles `task validate | Report`
→ 3. The wave has already opened that set.

**Cost of the cheap cut:** an agent runs the verb the composed `what's-left:` names, gets exit 0, then
`finalize` refuses — the ambush law 3 forbids, at the surface built to prevent it, on a committing door's
precondition, in exactly the duress cell six trials have been measuring.

**Robust scope:** one `GATE_COVERAGE` row (`Door::Previewed`, `Tier::Previewed`, fragment + token) and
`preview_gates` (`task.rs:1736`, CLI-side, reachable) calling the **shipped** `repo::posture` that
`refuse_on_posture` already calls. No new predicate, no new probe.

**Honest concession.** The **exit code** is a real decision and is separable: rendering the blocking
finding while leaving the exit to the previewed set's existing rule closes the law-1 lie without touching
D1's ground at all. If the human wants D1 intact, take the row and the probe and argue the exit
separately. What is not defensible on any reading is **silence**.

**Verdict: robust-now** for the row + probe (the claim is false on nine surfaces and the sibling preview
already refuses); the exit flip is a separate, declarable call.

---

## (iv) The composed off-verb lies

**Cheap:** reword the 12 `migrate-*` step texts and the `sub-task` body conditionally.
**Robust:** refuse at the compose door; close the off-verb `task finalize` at the `source-path` seam.

**The fork's own question, driven: a compose-door refusal is not new capability — it already ships.**
`jigc workflow milestone-execution --preview` → **exit 1**: *"workflow 'milestone-execution' mints no task,
so there is nothing to preview — run it directly: `jigc start --workflow milestone-execution`"*. Mechanism,
shape and exit all exist. And note what that route does: it points at **the door that composes the lie** —
`jigc start --workflow milestone-execution "off verb"` → exit 0, three `Run:` lines carrying
`<MILESTONE_ID>`, with a real milestone present in the repo (baseline §2.2). jigc's shipped refusal is
currently a **route into the defect** — M46's PT-1 shape at jigc's own door.

`jigc start --workflow migrate-adr "off verb adr"` → exit 0, mints, `{{ source }}` renders as three blank
lines, and the exit-4 promise sits verbatim at lines 118–121 (*"a plain finalize commits NOTHING … holds
(exit 4) … `--approve` is the one destructive gate"*). The baseline drove it to its end: plain
`task finalize` → **exit 0, promoted the doc**.

**Why rewording cannot carry this.** `is_migration` is `self.dir.join(SOURCE_FILE).exists()`
(`task.rs:2026`); in the off-verb cell that file was never staged, so the **binary is consistent and the
text is the lie**. A conditional rewording asks the reader to branch on a gitignored workbench file it
cannot read — structure delegated to prose, the determinism boundary inverted, across 14 surfaces that
each become conditionally-true. And it leaves the exit-0 promotion untouched.

**The robust cut asks a question the door already has the data for.** `describe` **states the constraint
per workflow** (*"verb-routed — reached only through `jigc migrate <path> --as adr`, which stages the
foreign source bytes the author step rewrites"*), so the set of 14 of 34 is **authored, not invented** —
the compose door reads a fact the pack already declares and the composing doors simply never repeat
(0 of 12). Law 2 and law 3 at one seam, versus 14 prose edits.

**Robust scope:** one predicate at the two compose doors (`start --workflow`, `workflow --preview`) over
that declared set, routed at the real verb; plus `is_migration` gaining its off-verb arm so the exit-4
promise is true. The tier-0 half must land either way — a committing door doing what its own composed
contract says it cannot is not a wording defect.

**Honest concession.** The wording work is not wasted and part of it is genuinely the right fix. A6-3's 14
assert-then-empty renders include workflows reachable **legitimately** with an empty set
(`implement-from-spec` over a spec-less corpus), and for those the empty-case sentence is correct — the
pack **already ships that shape twice** (`single-task`'s supersedes slice, `form-vision`'s grounding). So:
**refuse** where the composition is unreachable-as-composed; **state the empty case** where it is
legitimately empty. Two rules, and the cheap cut is right for the second.

**Verdict: robust-now** for the 14 verb-routed members (the refusal already ships at one door and
currently routes *into* the defect); **cheap-cut-is-correct** for the legitimately-empty renders.

---

## (v) `GIT_DIR` scope + `refuse_on_posture`'s `current_dir()` skip

**Cheap:** write both bounds (the claim closes over the family as probed from the cwd's repo; `GIT_DIR`
declared out; the `current_dir()` fault skips the probe).
**Robust:** close the `current_dir()` skip; declare `GIT_DIR` as the **one** residual.

**Driven** from a deleted cwd: `jigc validate --format json` and `jigc task list --format json` both →
**exit 1** with plain text `cannot determine the current directory: No such file or directory (os error 2)`
on stderr — i.e. DEFECT D, already tier 1 in this wave. So the pre-dispatch funnel is being built anyway,
and `refuse_on_posture`'s `.ok()?` sits **in that same funnel**, two lines above `dispatch()`.

**The argument is about kind, not size.** `refuse_on_posture`'s own doc-comment justifies carrying **no
override** because *"a posture is a repository state the user can resolve"* — and the guard is written
`std::env::current_dir().ok()?` and `discover_repo_root(&cwd)?`: **two fail-open `?`s inside a fail-closed
guard.** That is the same shape the wave's fork-3 rider already indicts (`leftover_at` mapping every
`symlink_metadata` error to `Absent`, against its own doc-comment) and that M48's fail-closed leftover
classifier was built to kill. Shipping a *written bound* saying "this guard skips on a fault" on the same
binary that ships a fail-closed classifier three files over is two rules disagreeing in the record — the
false-completeness shape M51 exists to correct, re-enacted in the fix for it.

**`GIT_DIR` is different in kind and the must-not-re-open list is right about it:** the rationale is
unfalsified, and a `GIT_DIR` redirect is a deliberate operator act that **moves the subject** rather than
defeating the guard. One declared exemption with a reopening condition is a bound a reader can hold; two,
one of which is an implementation accident, is not.

**Robust scope:** turn `.ok()?` into a refusal (a door cannot act on behalf of a repository it cannot
name), keep `discover_repo_root → None` as the honest *not-a-git-repository* answer M49 already ships, and
write `GIT_DIR` as the single residual with its reopening condition. One `match` arm, inside an edit the
wave is making anyway.

**Honest concession, and it is load-bearing.** **I could not demonstrate an exploit.** Both legs fail
together — every door also resolves the repo from `current_dir()` — so on my drive the command refused at
exit 1 rather than committing under an unadjudicated posture. The robust case here is therefore *not*
"there is a hole"; it is "there is an unproven fail-open in a guard whose contract is fail-closed, and
closing it is nearly free." If the plan can show **by driving** that no `CommitsOnBehalf`/`MovesOnBehalf`
leaf reaches its act when `current_dir()` fails, then the cheap cut is correct and this is a doc-comment
repair, not a behaviour change. That drive is one hour and should precede the decision. What must **not**
happen is writing the bound as if the escape were proven exploitable — that is the same overstatement in
the other direction.

**Verdict: robust-now, weakly** — close the skip because it is inside DEFECT D's own edit and reconciles a
guard with its own stated contract; but the exploitability claim is **not established**, and if the plan
drives it closed, **cheap-cut-is-correct** and this becomes a doc repair.
