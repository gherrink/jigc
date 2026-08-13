# M48 — the verified capability ledger (baseline for Detect gaps)

Consolidated from five `capability-auditor` fan-outs that **exercised the real binary**
(`jigc 1.0.0-rc.10`) in throwaway repos at HEAD `8979f16`. **Verified-at-this-sha map, not gospel** —
the gap pass still spikes the specific new shape M48 needs.

Charter under audit: `implementation/decisions-pending.md` → *The rc.11 wave (M48)*.
Trial findings: `completions/artifacts/RC-pre-1.0/findings-verification.md`.

---

## A · Charter premises CORRECTED (ranked by how much they move the plan)

1. **F3's trigger is not "a crashed run" — it is "you copied your repo."** The charter and the walk
   reach the leftover state by hand (`rm -rf .git/worktrees/<id>`). Two ordinary operations reach it
   with no manual step, both reproduced: **`cp -R` of the repo** (copied admin records carry the
   source's absolute gitdir, so the copy's own worktree is non-registered) and **`mv` of the repo**
   (old path goes `prunable`). Copying a corpus is how every RC trial corpus is made.

2. **A standing test pins F3's loss as expected output.** `crates/cli/tests/milestone.rs:2129-2141`
   plants a junk file in a non-registered leftover and asserts provision **clears it and exits 0**.
   The F3 fix **reddens `milestone.rs` and must revise that assertion**, not merely add a new one.
   The junk-dir shape must keep working. Unrecorded cost on F3.

3. **F3 is two unguarded doors, not one.** Both existing guards enumerate *registered* worktrees, so
   over F3's own shape `jigc uninstall` **also destroys at exit 0** (`uninstall.dirty-worktree` never
   fires); `milestone discard` is blind-but-harmless (exits 0, orphans the leftover). The "guarded at
   two doors, unguarded at a third" framing holds only for the *registered* shape.

4. **The existing guard is NOT drop-in reusable at provision's call site.** `dirty_worktrees` shells
   `git status --porcelain` with cwd = the leftover and bails on non-zero. Three leftover shapes,
   three behaviours: deleted admin record → **exit 128** (`fatal: not a git repository`), turning the
   destroyer into a hard error; copied repo → correctly reports dirt (resolves to the *source's*
   admin dir); junk dir with no `.git` → git **walks up** and reports the **main checkout's** status,
   so naive reuse refuses provision whenever the checkout is dirty, i.e. always.
   `git worktree repair` does **not** rescue the deleted-record shape (exit 1).

5. **F2's class is 3 cells, and the charter's fork addresses none of them completely.**
   - *Cell A* (reported): same-slug re-author → title dropped, stale H1, success ack.
   - *Cell B* (**unreported, worse**): a **divergent** title is not ignored — it **mints a second
     doc**. Two staged ADRs; `task finalize` exits 0 committing **both**. A correction silently ships
     a spurious ADR into the corpus.
   - *Cell C*: `doc create` is the same door — byte-identical ack whether the title applied or not;
     `doc create --slug <other>` over a staged doc mints a **third**.
   Root cause `doc.rs:2133 run_author`: `plan.title` is consumed only by `state::create_gated`; on the
   copy-in branch it is discarded and the H1 never revisited. The charter's fork ("reject the whole
   payload" vs "ack `title ignored — create-only`") answers neither B nor C. G3's underlying ask (an
   in-task doc-level title change) is the only shape that answers all three.

6. **F1's named fence precedent does not reach the steps fork 3 is about.** M44's D5 fence derives its
   owe-set from `{{schema:<T>}}` refs → **14 of the 31** write-soliciting steps, structurally missing
   every ordinary `author-adr`/`author-spec`/`author-commit` and all 11 methodology `author-*` steps.
   F1 needs a **new owe-set derivation over the command catalog** (`{{cli.*}}` refs — complete,
   machine-readable, every ref in all 66 steps resolves, verb literally in `args`). The rest of the
   pipeline (`states-constraints:`, `CONSTRAINT_REQUIRED_TOKENS`, the pack-load bail) is reusable with
   **no new front-matter key and no new parse path**. `<<author:>>` is unusable as the marker — 4 steps
   dev-pack, 0 methodology.

7. **F8's routing half is an entry in an existing fenced map, not new machinery.**
   `cli.rs:1327 unknown_subcommand_tip` is a curated code-side `(parent, guess)` map (2 entries), each
   span built through `Route::mechanical` so a ghost verb cannot compile, fenced by
   `cli_parse::every_curated_sibling_tip_passes_the_route_parse_fence`. Two wrinkles: jigc's tip
   **appends after** clap's whole error block, so clap's wrong `'set'` still stands above it
   (suppressing needs `default-features = false` on clap — repo-wide); and **`jigc config list` yields
   no tip at all** — the class is misdirection **and** silence.

8. **F14's stated cause is REFUTED.** The catalog carries the intent verbatim: `single-task —
   implement one scoped change end-to-end, recording its decisions as ADRs`. The real defect is
   **bundling, not routing** — `single-task` is the only catalog workflow pairing code + ADR without
   requiring a spec, and it bundles the changelog gate, so every non-user-facing single-task raises
   `gate-granted-unused` by construction. `implement-from-spec` is the exact shape wanted, gated
   behind a committed spec. Fix = unbundling / gate-relevance, not a new route.

9. **The hook's *"the exit code is wrong-way-round"* is accurate, not a stale lie.** It refers to
   `validate` exiting 0 on found drift but non-zero on not-a-project / probe-integrity — still true.
   `STORE_EXIT_FLIPS` has four members, **none `doc-code`**. M47 changed *what warns*, not the exit
   semantics. Residue is register only (it calls a thrice-documented design a bug) — tier-2 at most.

10. **F5's quoted rationale is misattributed.** *"committed by the operator's next commit, never here"*
    (`config.rs:289-293`) governs the **`docs-root` relocation `git mv`s** — and that branch **does**
    say so on the wire. The manifest write (`write_scalar`, `config.rs:245`) carries no such statement
    anywhere. The fix does not overturn a recorded policy; it **extends an honesty pair the same
    function already ships one door over**.

11. **F7's preferred arm is far more expensive than "either omission drops the heading or the guidance
    stops saying omit".** Conformance matches body sections **positionally** (`validate.rs:2656-2663`,
    the shipped M22 semantic: *"headings present, the flag governs content"*). Deleting the heading
    from a committed ADR today yields three findings (2× `section-renamed`, 1× `section-missing`).
    Arm 1 touches `empty_instance` **plus** the conformance matcher, `surplus_sections_absent`, and the
    `AddedOptionalSection` classifier. **No** schema-hash/manifest collision (schema unchanged), but
    every byte-stability golden moves. The splice half already exists (`write.rs:3469
    render_generated_section`). **Axis is tiny**: 3 `optional:` declarations pack-wide, 2 of them slot
    sections (`adr#options`, dev+methodology `commit#body`); the commit sink already strips it, so the
    persisted-doc path is the sole offender. Arm 2 (fix 2 guidance strings) is ~2 lines.

12. **F9 is pure wording and NOT gated** — `SLUG_RULE_VERSION` (=3) hashes `slugify`'s *behaviour* over
    a generated input vector; the statement is not an input, and the manifest fixed-point assert is
    untouched. **But the fence built to prevent exactly this drift explicitly exempts the hyphen**:
    `slug.rs:1111` — `if ch == '-' { continue; } // "-" is the join char, not a separator`. The fix
    must delete that exemption. **84 compose goldens** carry the sentence.

13. **F12: the projection is wrong, the acceptor is right by a declared convention** —
    `design/write-commands.md:83`: *"`--title` is always literally `--title`, whatever the doctype's
    `id-from` field is called — the flag names the role, not the field."* Affected:
    `changelog#unreleased-changes` and `changelog#releases/<id>/changes`. The `--format json` item body
    carries **no** id-from/write-key marker at all. `contract-version` is **4** today; adding the key
    is a pinned-contract change → **4→5**.

14. **F15's structural half is a one-way door.** No field on `roadmap` (block = `title` · `proves` slot
    · `decomposition` slot) or `milestone-record` (`meta`: base/status; task item: task-id/intent/status)
    can hold the edge, and **both are manifest-frozen** (`roadmap` v1 `f02a981…`, `milestone-record` v2
    `4c725b8…`). Closing it structurally forces a version bump **plus** a corpus migration. A non-schema
    half is available: planning's finalize *naming* `jigc milestone create <the title you just used>`.
    Separately, the positional-vs-`--title` asymmetry is **deliberate and documented in both help
    texts** (work-unit verbs take the id-source positionally) — discoverability, not a defect.

15. **F10's `-block` name is deliberate and load-bearing.** `file_state.rs:1066-1073` records that it
    **reuses the existing check id at `Severity::Advisory`** under the M21 *"no new check ids"*
    invariant. Renaming re-opens that invariant **and** splits a stable `(code, target)` finding key.
    No "severity implied by code name" convention is fenced anywhere; the opposite rule is.

16. **`doc create --slug` already exists** (verbatim override, inert for singletons) — the smaller
    finding is true **only of `doc author`**.

17. **The gate is ~4–7 min, not 30–50.** M47's twelve-group consolidation cut it; the old figure also
    double-counted (~6.4 runs of each suite; per-suite medians = 7.9 min). Cold with clippy ≈ 20 min.
    Sources: `M47/VERDICT.md:48`, `.claude/agents/build-executor.md:17-18`, `DECISIONS.md:432`.

18. **66 pack step files, not 69** (30 dev + 36 methodology). The "exactly one" numerator is confirmed
    (`locate-from-spec.yaml`) — but *"not an authoring step"* is imprecise: it **does** solicit writes;
    what disqualifies its mention is **direction** (it names `doc show` to recover item ids off a
    **committed** spec, never to read back the agent's own staged work).

19. **Composed density: ~200 lines → 154** (the true dev-pack max). The diagnosis holds exactly: the
    `{{schema:changelog}}` block is **58 of 154 lines (38%)**, rendered unconditionally inside a step
    that then instructs *"OMIT the `releases` entry"*.

---

## B · Defects and gaps found OUTSIDE the charter

- **`jigc uninstall` destroys authored prose with no worktree at all.** M47's fix docstring
  (`setup.rs:1370`) claims it protects *"the sub-tasks' authored doc prose in `.jigc/tasks/<id>/docs/`,
  which is in no object DB at all"* — but the probe (`dirty_fanout_worktrees`) is
  **registered-worktrees-only** and short-circuits at `setup.rs:1385`. Reproduced: setup → start →
  `set-slot` (exit 0) → `uninstall` → **exit 0**, prose gone from the repo entirely. M47's own
  incomplete-sweep class, one axis over, on the fix its audit shipped.

- **`set-slot` writes `<<…>>` markers literally into committed prose at exit 0.** An agent carrying the
  `doc author` grammar one verb sideways corrupts a doc with no warning
  (`set-slot … --from-file` with wrapped prose → exit 0, *"(17 chars)"*, `doc show` reads back the
  literal markers).

- **`doc list` has no staged route.** With a staged ADR: `doc list` → exit 0, *"no committed docs"*,
  **no route** to `doc show --task`; `doc list --task <id>` → **exit 2**, bare clap parse error, also
  no route. Its `--help` does state the task-less contract, so law-2 (nothing hides), not law-1.
  Plausibly the **first** surface an agent reaching for "what have I got?" hits — F1 as chartered would
  not touch it.

- **One foreign file yields two different codes depending on door.** Store scope:
  `schema-conformance.unadopted-instance` with M42's managed-vs-foreign discriminator route (exit 0).
  Task scope: the pre-M42 `reconciliation.conformance-block`. **M42's discriminator swept the store
  family and never reached the task-scope gate** — an incomplete sweep, M45's lens on M42's work.

- **F13 has a sibling and a design constraint.** The header fires **only** when the left-out set is
  non-empty, and is emitted **deliberately pre-commit** per an M42 decision (`render.rs:1200-1206`,
  `design/finalize.md`) — so *moving* it undoes that decision; *rewording* preserves it. A second
  present-tense stem lives at `render.rs:1232` (the carry-over render). A complete fix takes both.
  F11 is the same class through a third door.

- **`uninstall` leaves `.git/worktrees/*` admin records dangling** (M47 declared bound, reproduced) —
  **and those dangling records are exactly the state in which a later `provision` sees a non-registered
  path.** The bound is the *doorway into F3*, not an isolated tidiness issue.

- **`describe --format json` exists and is structured** (`schema_version: 2`, `{kind,id,prose}`) — but
  the router-hidden flag exists **only as a substring of `prose`**, so even JSON cannot separate
  selectable from suppressed without string-matching. 33 workflows shown vs 12 offered by the router;
  **18 of 33** carry *"It is hidden from the router catalog"*; **12 of 33** are `migrate-*`. 101 lines /
  24,407 bytes, longest line 1,167 chars, all 17 commands on one line. `describe`'s prose tier is
  deliberately non-contractual (fenced), which constrains what a fix may pin.

- **`setup --format json` still cannot name the hooks dir** (M46 entry 12, confirmed live): four keys
  only, `hook_file` absent from the envelope though computed and rendered in text.

- **F4's root cause is M47's own incomplete sweep**: `setup.rs:999 install_tracked_paths()` is a
  hardcoded 7-path list whose doc comment states the now-stale premise *"the pre-commit hook … never a
  tracked file"* — true for `.git/hooks`, **false** under in-repo `core.hooksPath`. The hook
  *installer* became hooks-path-aware; the install-commit pathspec did not. The suite that swept that
  axis (`setup_names_the_core_hookspath_hook`) asserts the **printed path** and nothing about the
  commit's pathspec.

- **A test-registration fence gap:** `test_target_registration.rs::registrations()` reads group roots
  from `tests/groups/*.rs` **on disk**, not from `Cargo.toml`'s `[[test]]` list — a new group root
  added without its `[[test]]` entry would look registered while compiling nowhere. Also both
  `Cargo.toml`'s comment and the fence's module doc still say **"ten"** group binaries; there are
  twelve. Submodules under `pinned_facts/` are not scanned by that fence at all.

---

## C · Built-and-proven (no M48 work owed) — the reuse surface

- **The three-tier pack-load fence pipeline.** Structural `assert_stated_at` (4 `AMBUSH_CLASS_CODES`)
  · per-soliciting-step `assert_singleton_copy_in_stated` (derived owe-set + biconditional) ·
  named-fact `assert_named_facts_stated` (`CONSTRAINT_REQUIRED_TOKENS`, 5 rows, whitespace-collapsed
  token match). All three fire at pack-load, every door blocks, redness is exit 1 with a bail message.
  **Both proven by applied mutation** in this audit (drop the declaration → exit 1; corrupt the
  required token → exit 1). `stated_at_fence::` 17 passed / 0 failed, genuinely mutation-driven.
- **`jigc doc show --task <id>`** — drives clean; the committed-miss route is correct and followable.
  F1 is *purely* discoverability; the capability is solid.
- **`is_machine_maintained_absolute`** (M45 settability predicate) — both arms proven (rejects
  `schema-version`, allows a `set: on-create` `date`). Feeds both the projection and the write path.
  **`title` is outside every settability mechanism** — it is not a `Field`, has no `set:` kind.
- **`write.identity-change`** on enum id-from `retitle-item` — proven with a working route.
- **The write-verb × miss-shape matrix** (`write_miss_shape_axis.rs`) — bijected against the clap `doc`
  leaves; `create`/`author` are on a **declared exempt list**, so F2 is a genuinely new class, not a
  hole in an old fence.
- **`resolve_hooks_dir`** — proven on `core.hooksPath` and linked-worktree installs.
- **The survivable hook-rejection frame** — intact at finalize (no-commit statement · verbatim hook
  stderr · door-specific state-truth clause · copy-runnable re-run argv). Only the header lies.
- **`milestone discard` / `uninstall` guards** — proven, for the **registered** shape.
- **The commit-message sink strips empty optional sections** (so F7's persisted path is the sole
  offender).

---

## D · Registries available to drive flow 48's axis arms

| Registry | Location | Enumerates | tests/-reachable |
|---|---|---|---|
| `COMMITTING_DOORS` | `invocation_log.rs:126` pub | 9 committing doors | ✅ |
| `ERROR_CODE_REGISTRY` | `invocation_log.rs:183` pub | 10 route-exempt identities | ✅ |
| `EXIT_CODES` | `task.rs:126` pub | 5 outcome classes | ✅ |
| `CONSTRAINT_REQUIRED_TOKENS` | `pack.rs:641` pub | 5 constraint→token rows | ✅ **F1's precedent** |
| clap verb tree | `cli.rs` via `CommandFactory` | **44 leaf verbs** | ✅ |
| Pack registry | `pack.rs` `PackSource::list(kind)` | workflows/schemas/steps per pack | ✅ |
| `origin_packs` | `packsource.rs:191` | owning packs of a shadowed id | ✅ |
| settability predicate | `engine::schema:645` pub | per-leaf three-state settability | ✅ |
| `INTRINSIC_CHECK_KEYS` | `knobs.rs:55` pub | intrinsic check keys | ✅ |
| `SLUG_RULE_VERSION` | `slug.rs:561` pub (=3) | slug rule generation | ✅ |
| `STORE_EXIT_FLIPS` | `render.rs:622` **pub(crate)** | 4 exit-flipping conditions | ❌ needs a visibility promotion |
| `ROUTE_PLACEHOLDERS` | `finding.rs:296` pub | route placeholders | ✅ but **no suite iterates it** |
| `DUMMY_SUBSTITUTIONS` | `route_fence.rs:22` **private** | placeholder → dummy argv | ❌ |
| `State::ALL` | `tests/support/trial_corpus.rs` | 6 fixture states, 19 consumers | ✅ |

**Fixture states M48 needs that the shared builder CANNOT construct** (all three must be built
fresh per arm): a **leftover non-registered worktree with dirty work** (`copy_state()` hard-refuses any
corpus with `.git/worktrees` — absolute-path contamination); an **in-repo `core.hooksPath` with a
tracked foreign hook** (nearest shape is a *local helper* inside `flow47_acceptance.rs:1690-1725`); a
**re-authored doc with a changed title** (`batch_author_rerun.rs` uses the **identical** title in both
payloads — the one shape where F2's drop is invisible).

---

## E · Build tax, measured

- **12 `[[test]]` group targets**, `autotests = false`, **263 suite files**, bijective with 263
  `#[path]` registrations. A new suite must be registered in **exactly one** group root; if it uses the
  shared module the group root must already declare `support` (**7 of 12 do**; `g_config`,
  `g_finalize`, `g_methodology`, `g_migration`, `g_solo_store_sweep` do **not**); an env-mutating suite
  must be **alone in its group**; `--test <file>` no longer addresses a suite.
- **612 compose goldens** (36 composite · 276 dev · 300 methodology), enumerated from the loaded
  registries so a new workflow/doctype joins with zero test edits. Compare ≈ **13 s**; regen is the same
  plus a **612-file review diff**; regen refused under `CI`. Sequencing: regen lands **after** the
  wave's surface-changing fixes.
- **Full gate ~4–7 min warm, ~20 min cold with clippy.** Run it **full and unscoped**, backgrounded,
  unpiped, `$?` captured directly — a scoped run silently misses the `--bin jigc` pack/describe
  goldens.

---

## F · The conversion ledger's mechanics

**There is no mechanical check that the ledger is closed, and that is deliberate, recorded and
argued.** `pinning.md` §3: `pinned-by:` is *"declared list-enforced — audit-checked prose verified at
the wave's completion audit … because the machine-checkable half (a test *name* exists) is not the
load-bearing half (the test *content* pins the claim); a symbol-existence parser would be a finder
wearing a fence's badge."* A `pinned-by:` symbol parser is listed under *Deliberately out*, rejected at
cross-review. Nothing in `crates/` or any script reads `findings-verification.md`.

**The M42 precedent, if M48 wants one:** it is **not** a name-parser. It is the **finding-serialization
seam** (`engine/src/finding.rs`) — `Finding`'s `Serialize` impl carries four asserts keyed on **single
declared exception lists in one place** (`is_declared_singleton`, `is_route_exempt`,
`ROUTE_PLACEHOLDERS`). Design of record `DECISIONS.md:2322` (T9): *"a claim checkable only by a census
someone must remember to repeat **will** rot"* — membership becomes a **predicate**, the check lands
**at the seam**, and it lands last, as *"the sweep's own auditor."* **Honest bound:** these are
`debug_assert!`s, so they fence the test build, not a release binary a trial runs — which is why
`commit_rejected_axis.rs` exists to drive the same axis through the real binary.
