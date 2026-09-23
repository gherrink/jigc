I drove the debug binary built at HEAD `7bf05636` against nine `dev/jigc-rig` corpora, including roots under `…/jigc space.rpTPYu/…` and `…/it's #odd.4e6xLP/…`, plus hand-built linked and fan-out worktrees. Every finding below is marked **DRIVEN** or **READ**.

---

# Findings

## MEDIUM 1 — the pre-commit rename backstop's "declared open" OPERAND axis is reachable through jigc alone, in one supported command, and the door then fails **open** while printing a false sentence

**Location:** `crates/cli/src/setup.rs:566-571` (the declared bound) and `:581` (`PRECOMMIT_RENAME_BLOCK`, the awk).

The bound as written reads *"a managed doc whose own path contains a space"*, which implies the operator must have named such a doc — and jigc refuses to mint or adopt one (**DRIVEN**: `ingest.unadopted-identity`… in fact `ingest.unaddressable-identity` — "`conformant adr at docs/decisions/my notes.md` is not adopted: its name is not a doc id"). But the store key is `<docs-root>/<...>/<slug>.md`, and **the root is a knob that accepts a space at exit 0**:

```
$ jigc config set docs-root "my docs"
config: set `docs-root` = `my docs` — written to `.jigc/config/`, uncommitted …          rc=0
$ jigc config set placement-root "my root"                                               rc=0
  relocating the committed doc(s) stranded by the `placement-root` re-point …
  - docs/roadmap.md → my root/roadmap.md
```

Then, **DRIVEN** end-to-end on `committed-singletons` with `docs-root = "my docs"`, an adopted adr at `my docs/decisions/old-cache.md` (adopted cleanly — `adoptable … → adr (adopted — indexed + baselined)`):

```
$ git mv "my docs/decisions/old-cache.md" "my docs/decisions/new-cache.md"
$ git status --short
R  "my docs/decisions/old-cache.md" -> "my docs/decisions/new-cache.md"
$ git commit -m "bare git mv of a managed doc under a spaced docs-root"                  rc=0
jigc: an out-of-band managed-doc rename exists in the committed tree … (not staged in
      this commit; commit not blocked).
```

The commit landed. The printed sentence is also false — the rename *was* staged in this commit. The two hook inputs show exactly why (**DRIVEN**, same repo, same state):

```
staged set : my docs/decisions/old-cache.md          (unquoted — git does not quote here)
             my docs/decisions/new-cache.md
moves      : git -C /…/repo mv 'my docs/decisions/new-cache.md' 'my docs/decisions/old-cache.md'
```

The awk's `$i == "mv" && ($(i+1) in S) && ($(i+2) in S)` finds `mv` at field 4, and `$(i+1)` is `'my`. No hit, no block, no message that a guard did not fire.

**How the bound was derived:** one knob-set each for the two `ROOT_KNOBS`, then the full sequence above. The class is *every* location-doctype store key under a spaced `docs-root`, and every placement doc under a spaced `placement-root` — not one hand-named file.

The *fix* in this range (the home axis) is genuinely closed — **DRIVEN** under `…/jigc space2.DnVn0A/…/repo`, the identical `git mv` of an adopted adr blocked at rc=1 with the `RENAME_BLOCK` message. What is wrong is the **statement of the residual**: it characterises the open cell as something an operator would have to contrive, when one shipped `config set` reaches it.

**Smallest correction:** either restate the bound naming `docs-root`/`placement-root` as its reachable cause, or close it — the operands could be emitted `--` separated and read from the tail (`… mv -- <a> <b>` is not what the route says, so the cheaper close is a shell-word tokenizer, as the bound already says). At minimum the bound must not read as unreachable.

---

## MEDIUM 2 — in the fan-out cell the new install-site line asserts the opposite of what just happened: it says the standing worktree was *not* the subject, while that worktree was destroyed with the install

**Location:** `crates/cli/src/render.rs:1990-1996` (`install_site_line`), reached from `uninstall_success` (`render.rs:4031-4034`).

```rust
"  {tense} at `{}` — the main checkout this repository's jigc install and `.jigc/` \
 workbench bind to, not the worktree you are standing in",
```

**DRIVEN**, `bare` rig set up, milestone provisioned, `cd $REPO/.jigc/worktrees/area-one && jigc uninstall` (no `--force`, clean worktree):

```
rc=0
jigc uninstall — repo-local install removed
  - removed .jigc/
  …
  removed at `/…/repo` — the main checkout this repository's jigc install and `.jigc/`
  workbench bind to, not the worktree you are standing in
```

After: `$REPO/.jigc` gone — and the caller's cwd, `$REPO/.jigc/worktrees/area-one`, gone with it. A fan-out worktree lives *below* `.jigc/`, so in this cell the standing worktree **is** part of what was removed. The one line in the output that mentions the standing worktree tells the reader it was untouched. Nothing else in the 9-line ack names the worktree, the removal list does not carry it, and the shell is left in a deleted directory unnarrated. (This is the behaviour the DECISIONS entry describes as intended — *"the caller's shell is left in a deleted directory"* — so the defect is the sentence, not the removal.)

The DECISIONS entry's own claim that the four WIP guards are now live is **confirmed DRIVEN from that same worktree**: `uninstall.dirty-worktree` (planted `dirty.txt`), `uninstall.dirty-worktree` via the un-concluded-`git bisect` leg with its `git -C <abs> bisect reset` route, `uninstall.untracked-workbench-file` (planted `.jigc/stray.txt`), and `uninstall.foreign-bytes` (planted `.jigc/tasks/area-one/foreign.bin`) — all rc=1.

**Smallest correction:** make the clause conditional on whether the caller's cwd is under the removed tree — *"…and it includes the worktree you are standing in, which this removed"* — one branch in `install_site_line`, since the door already knows both paths.

*(Related, deliberate, and noted rather than filed: from a linked worktree `jigc setup` prints `bootstrap reference → CLAUDE.md` and `pre-commit hook → .git/hooks/pre-commit`, spelled against jigc_home while the reader stands elsewhere. `crates/cli/tests/setup.rs:905-921` states that decision and the site line carries the root, so it is answered.)*

---

## LOW 3 — the `human_echoing_caller_token` carve-out suppresses the whole text, not the echoing span, and matches by substring

**Location:** `crates/engine/src/finding.rs:1197-1216` (`unbased_migrate_span`) against `:1468-1470` (the fence). **READ.**

```rust
pub fn unbased_migrate_span(text: &str) -> Option<String> {
    for span in backticked_spans(text) { … return Some(span.to_owned()); }   // :1213 — FIRST only
    None
}
…
if let Some(span) = unbased_migrate_span(text)
    && !caller_echo.is_some_and(|echo| span.contains(echo))                   // :1469
```

Two consequences, both answering the question the review posed about the carve-out's honesty:

1. `unbased_migrate_span` short-circuits on the **first** offending span. If a future producer constructed through `Route::human_echoing_caller_token` emits the echo span *and* a second span carrying a computed relative path, the predicate returns the echo span, `contains(echo)` is true, and the panic is skipped for the whole text — the second span is never examined.
2. The test is `span.contains(echo)`, not an operand identity. `caller_echo` is `shell_token(typed)`, i.e. operator input; with `typed = "a.md"` a computed span `jigc migrate docs/a.md --as adr` also contains it.

The ⇔ fence `git_span_aim.rs::every_production_caller_of_the_migrate_operand_home_is_a_row` narrows this — it enumerates callers of `migrate_at`/`migrate_operand`, so a producer that *does* reach the home while disposed `DeclaredOut` reddens. It does not catch a producer that emits a raw relative span alongside the echo, which is precisely the case the carve-out hides. The commit message's claim that the exemption is *"one call site wide"* is true about who may invoke the constructor, not about what it exempts inside that call.

**Smallest correction:** move the carve-out to the span — check `unbased_migrate_span` on the text with the echo span removed, or have the predicate return all offenders and exempt only the span whose *operand* equals `caller_echo`.

---

## LOW 4 — `unbased_migrate_span` still reads only a span's head, the exact blindness LOW 8 fixed in its two siblings in this same range

**Location:** `crates/engine/src/finding.rs:1198-1202`. **READ.**

```rust
for span in backticked_spans(text) {
    let tokens: Vec<String> = command_tokens(span).collect();
    if tokens.first() != Some("jigc") || tokens.get(1) != Some("migrate") { continue; }
```

`unaimed_git_span` (`:1123`, commit `94f6bb93`) and `unsafe_command_token` (`:1268`, commit `8076d9c5`) were both widened to `split_shell_sequence` in this range on the finding that a composite span is never checked. The migrate fence was minted one commit earlier (`46169f6c`) and was not swept with them, so `` `cd <abs> && jigc migrate rel.md --as adr` `` evades it. No producer emits that shape today; the gap is structural and is the same one that was just named a defect twice.

**Smallest correction:** run the loop over `split_shell_sequence(span)`, as its two siblings now do.

---

## LOW 5 — two records of the same measurement disagree: `DECISIONS.md` says 33 functions, the commit message says 44, and 44 is what the file carries

**Location:** `DECISIONS.md:11` — *"`setup.rs` threads **one** root through 33 functions from two seams"* — against commit `82a4c4a0`'s message, *"setup.rs threads ONE root through 44 functions"*.

**Measured** (`head -4013 crates/cli/src/setup.rs`, i.e. everything above `mod tests` at `:4014`): `jigc_home: &Path` occurs **44** times; `root: &Path` once (`write_version_stamp`, the declared exception); `repo_root: &Path` zero. The commit message's figure is the derivable one.

This repo fences its counts (`crates/cli/tests/count_fences.rs`); this one is prose in two homes with two values.

**Smallest correction:** correct `DECISIONS.md:11` to 44, or state the count once and cite it.

---

## LOW 6 — `task finalize`'s stamp refresh is checkout-bound while its reader is home-bound, so a finalize in a linked worktree leaves the store advisory permanently stale at both roots

**Location:** `crates/cli/src/setup.rs:370-377` (the declared split) and `crates/cli/src/task.rs:5234-5241` against `crates/cli/src/cli.rs:1393`.

**DRIVEN.** `fresh` rig, main's `.jigc/version` downgraded to `0.9.0` and committed, linked worktree `feat` cut from that commit, a `quick-fix` task authored and finalized **from `feat`**:

```
finalized 0335cbe — fix(core): do a thing
  modified .jigc/version
  committed in the linked worktree at `/…/feat` on branch `feat` …

feat  .jigc/version : jigc-version: 1.0.0-rc.18
main  .jigc/version : jigc-version: 0.9.0

$ jigc validate            # from $REPO *and* from feat
advisory · store-version.binary-mismatch — store last written by jigc 0.9.0; you are
  running 1.0.0-rc.18 — align versions or re-run `jigc setup`
```

Advisory, exit 0, and its route (`jigc setup`) works, so this is not a break — but the surface says *"store last written by jigc 0.9.0"* immediately after a jigc store-write at 1.0.0-rc.18, and it says it from the worktree whose own checked-out stamp reads the new version. The split is stated at `setup.rs:370-377`; the *consequence* is not, in either home.

**Smallest correction:** none required behaviourally (the stamp is a tracked file and converges on merge) — state the consequence on `write_version_stamp`'s doc-comment, which currently justifies the split without naming what it produces.

---

## LOW 7 — `uninstall` leaves git's worktree admin stale after taking a fan-out worktree

**Location:** teardown path in `crates/cli/src/setup.rs::uninstall` (removal of `.jigc/`), no `git worktree prune`. **DRIVEN**, immediately after the MEDIUM 2 run:

```
$ git worktree list
/…/repo/.jigc/worktrees/area-one   e12c5b0 (detached HEAD) prunable
$ ls .git/worktrees
area-one   feat
```

Pre-existing, not introduced here — the same residue followed a root-cwd `uninstall` before this range — so it is named, not charged to these fixes. **Smallest correction:** `git worktree prune` after the `.jigc/` removal, at the one door.

---

# Closure table

| # | Finding | Status | Evidence |
|---|---|---|---|
| HIGH 1 | Spawn `cd` unquoted on a spaced root | **CLOSED** | DRIVEN: spaced + `it's #odd` roots; `Spawn:` `cd` and `start --task`'s refusal byte-identical (`cd '…jigc space…/area-one'`); `sh -c` rc=0 |
| HIGH 2 | `jigc migrate` routes root-based vs cwd-based verb | **CLOSED** | DRIVEN from `docs/deep`: `validate`'s `adoption_route`, `doc show`'s reroute and `ingest`'s near-miss all emit an absolute; the route run verbatim rc=0 (`task minted: migrate-changelog-…`). `orphan`/`finalize` arms READ + fenced (`Route::human` fences in debug; `MIGRATE_SPAN_SITES` ⇔ fence green). Carve-out honest in *scope*, not in *span* → LOW 3/LOW 4 |
| MEDIUM 3 | Hook rename block on a spaced root | **CLOSED (home axis)** | DRIVEN under `…/jigc space2…/repo`: `git mv` of an adopted adr → `git commit` rc=1, `RENAME_BLOCK` on stderr. Operand axis open **and reachable** → MEDIUM 1 |
| MEDIUM 4 | `milestone create`'s base pin | **CLOSED** | DRIVEN: created from `feat` at `de22bc2` with main at `135894c` → pinned `135894c`; provisioned, staged work, `milestone finalize` from `feat` rc=0, no `base-mismatch` |
| LOW 5 | `setup` binds `jigc_home` | **CLOSED** | DRIVEN: from a linked worktree rc=0, main's `.jigc/{AGENT.md,config,state,version}` written, zero worktree-local `.jigc` write, site line names the home; `--format json` 7 keys, no `site` |
| LOW 6 | AGENT.md "the one exception" | **CLOSED** | READ + goldens: 6 files, exactly `1 +/1 −` each, the paragraph only; it now states two rules and covers the migrate and commit-site absolutes |
| LOW 7 | stale test citation in `migrate.rs` | **CLOSED** | READ: re-pointed at `path_arg_occurrence_axis.rs::every_path_arg_occurrence_resolves_its_token_against_the_base_it_states`, which exists |
| LOW 8 | aim-fence shape gaps | **CLOSED** | DRIVEN via suite: `git_span_aim::` 12 passed, asserting the unaimed composite `is_some` and `commit`/`log`/`show` admitted, `-- <path>` forms still caught; quoting twin `8076d9c5` likewise. Residual → LOW 4 |
| LOW 9 | `adapter::render_spawn` | **CLOSED** | READ: deleted; only test-local helpers and a struck comment remain |
| LOW 10 | `uninstall` binds `jigc_home` | **CLOSED** | DRIVEN: from a fan-out worktree rc=0 removing the home's `.jigc/`, hook, SKILL.md, preload; all four WIP guards fire (dirty / bisect / untracked-workbench / foreign-bytes); `--format json` 4 keys, no `site`, warning on stderr. Ack wording → MEDIUM 2 |
| LOW 11 | `provision_worktrees` on `jigc_home` | **CLOSED** | READ: `milestone.rs:2906-2912` takes `jigc_home: &Path` |

**Other checks:** `count_fences::` 14 passed · `posture_member_inventory::` 4 · `git_span_aim::` 12 · `repo_relative_paths::` 7 · `path_arg_occurrence_axis::` 4 · `text_json_parity_axis::` 5 · `precommit_hook_acceptance::` 12 · `cwd_verb_subject::` 11 — all rc=0, run bare. No golden was widened: the only golden movement in the range is 6 AGENT.md files at 1 line each, and it is a content decision, not an admission of new output. The parity fence asserts `site` non-emission **both** ways (key absent *and* value absent) for `setup` and `uninstall`, confirmed against the real binary. `f00800f9` is dangling — `git log --all` does not contain it, `git branch -a --contains` is empty, the working tree carries only the untracked review file, `DECISIONS.md` and the review file are intact. No census row was left undispositioned by these fixes; `cwd-census.md` is a dated measurement snapshot against rc.18 (its line 171 generalisation *"a `jigc` span resolves its token against the root"* was already falsified by `7cd03c59` before this range, and is superseded by the two DECISIONS entries rather than carried as a live claim).

---

# Verdict

**HOLDS WITH FINDINGS.**

All eleven closures are real, and nine of them I closed by driving the binary this range produces. HIGH 1 is closed on the hard cells (space, `'`, `#`) with the two producers byte-identical. HIGH 2 is closed at a single home with a ⇔ fence and a runtime fence live on all three `Route` constructors. MEDIUM 4 lands a milestone born in a linked worktree. LOW 5/10 is the largest piece of new work and it is the right shape: the door binds one root, the four teardown guards actually run from a worktree for the first time, and both envelopes are unmoved.

What stops it being clean is two residuals that are *statements* rather than mechanisms — and both are of the shape this review-and-fix cycle exists to catch. MEDIUM 1: a declared-open fail-open axis is characterised as unreachable and is one `jigc config set docs-root "my docs"` away, with the door then printing a sentence that is false. MEDIUM 2: the fan-out cell's new site line tells the reader their worktree was not the subject at the moment it is being deleted. The three LOWs on the fences (carve-out span scope, composite blindness in the one sibling not swept, the 33-vs-44 count) are small and local. None of the seven is data loss beyond what the range's own design already declares, and all have one-branch or one-sentence corrections.
