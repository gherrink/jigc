# M49 — the pre-1.0 completion wave · VERDICT

**Green.** Built, audited, **7 findings confirmed live and fixed axis-complete**, re-verified
**3155 passed / 0 failed** with `fmt` / `clippy -D warnings` / `build` all clean on the combined tree.

- **Base** `8a5d73d` · **100 commits** (94 build + 6 audit-fix)
- **Planning**: [settle-record.md](settle-record.md) · [baseline-ledger.md](baseline-ledger.md) ·
  [planning-gate-record.md](planning-gate-record.md) · [charter.md](charter.md) (superseded where they disagree)
- **Acceptance**: [worked-examples.md](../../../design/worked-examples.md) → flow 50 ·
  `crates/cli/tests/flow50_acceptance.rs`

---

## The claim, and whether it held

> **A fence applied where its wave pointed is not applied at all — and the cost of that is silent,
> committed data corruption.**

**It held, and the wave's own centrepiece proved it twice over.** The defect that opened M49 was found
by driving, not reading: an item's own leaf region ended at the first *deeper* heading, but an item's
slot sub-labels **are** deeper headings, so on any ≥2-slot item block a `set-field` duplicated the
field bullet, **committed at exit 0**, `jigc validate` reported *"the committed store validates clean"*,
and the pinned 1.0 `doc show --format json` returned the **stale** value — contradicting the write it
had just acknowledged.

It had never fired because `roadmap.milestones` is the only shipped 2-slot item block and carries no
settable field. **The corpus was safe by accident of shape.** And the reason nobody had swept it is one
sentence in a locked doc: `design/doc-read-surface.md:85` asserted *"no shipped doctype declares a
multi-slot repeatable"* — **false since M16** — which is why M47 pinned the *read* side against a
synthetic fixture and left the *write* side alone.

**The audit then found the same shape three more times**, in this wave's own new work and in two
prior waves' fences. See *What the audit found*.

---

## What shipped — twelve increments

1. **The item-region class, both seams.** The boundary became schema-keyed; multi-slot ∧ nested became
   **legal rather than fenced**. 18-cell manufactured shape space.
2. **The doors that lie about state.** A closed `set:` vocabulary · `milestone list-tasks` stops
   mutating under `VerbKind::Read` · `add-task --workflow` validated · sub-task discard settles the record.
3. **The freeze stops being bypassable** at the project layer, and a mis-keyed pack leaf stops silently
   erasing a section.
4. **`AddedItemSlot`**, and the classifier stops blessing the backstop's residual.
5. **The engine capability batch** — the nesting ceilings reconciled, `add-item --slug`, `doc author`'s
   batch path.
6. **PB-1** — a project pack composes with the embedded pair at a declared precedence, and may not
   shadow a doctype the freeze governs.
7. **Placement stops being the one home no knob can reach** — with a detect+route+move floor.
8. **The pinned contracts** — N1's convergence · N2 additively · D1's located text findings · D5's
   enumerated route exemption.
9. **The schema bumps** — `planning-record` (new, 14 required gate slots) · `completion-record` 1→2 ·
   `milestone-record` 2→3.
10. **The pack tells the truth about who acts** — delegation named in prose, the false-statement fence
    repaired, the fan-out spawn line reading the recorded workflow.
11. **The surface batch** — and 20 falsified statements across 12 docs.
12. **Flow 50, the goldens, the fold-back.**

---

## What the audit found — 7 findings, all confirmed live before any fix, all fixed axis-complete

**Every one was a larger class than the finding reported.** That is the single most important line in
this record: six independent fixers, each told to derive the axis rather than trust the enumeration,
each found more.

| # | severity | finding | reported → actual | commit |
|---|---|---|---|---|
| 1 | **HIGH** | `config set placement-root .git` moves managed docs into git's own dir, **reports success, exits 0**, `validate` grades the store clean — **the doc is gone from every clone** | **2 → 8 movers** (`relocate` and `rename` unreported) | `5fb6138` |
| 2 | MEDIUM | In the wave's **own centrepiece cell**, a slot payload at the depth jigc's message just prescribed is refused with a **false reason** | **1 → 3 faces**, one of them **silent data loss** | `1cf9295` |
| 3 | MEDIUM | Six new user-facing messages ship their source indentation (14–22 space runs) | **6 → 9 literals** | `9706016` |
| 4 | LOW/MED | `write.non-reparseable` emits an unresolvable `key.target` on one branch | **1 → nearly every break shape** | `2262e02` |
| 5 | LOW | `milestone provision` over a leftover *file* answers with a bare error, no code, no route | **1 → 3 doors**, plus **unreported data loss** | `2262e02` |
| 6 | LOW | `FREEZE_DOORS` is a hand-listed 5-member sample feeding a test named `…_blocks_every_door` | **5 → 47 rows**, bijected with `VERB_KINDS` | `6d376d6` |
| 7 | LOW | A `location:` without a trailing slash keys file-state at a path that cannot exist | **1 → 8 concatenation sites** | `ad7452b` |

### The two unreported data-loss faces

Neither was in any finding; both were found because a fixer derived the class instead of patching the repro.

- **`{single-slot, nested}` silently truncated slot prose.** Exit 0, bytes landed, and
  `doc show --format json` returned everything *before* the heading and nothing after. The reported
  cell was a **rejection**; its sibling was **loss**.
- **`jigc uninstall` destroyed planted files at exit 0.** `fanout_worktree_paths` filtered its subject
  with `path.is_dir()` — **a claim about shape where the door's question is about bytes** — so a
  non-directory leftover was invisible to M48's destroying-door guard *and* to its loss narration,
  while `remove_dir_all` took it anyway.

### Root causes worth carrying forward

- **`git mv <src> .git/<dst>` prints `error: invalid path` and exits 0.** It moves the file on disk,
  drops the source from the index, and adds nothing. Every mover read that `0` as success. The bug was
  not a wrong path — it was **trusting an exit code that lies**.
- **Two of jigc's own rules disagreed by one level.** `slot_ceiling` reserves a *level* and
  `first_allowed` is one deeper, while both region seams bounded at *"the first heading deeper than the
  item."* Following the tool's own advice therefore produced a refusal.
- **An invariant stated in a comment and enforced nowhere.** `start.rs:3206` said *"the slash must
  survive"*; eight sites concatenated anyway.

---

## Honest bounds, carried

1. **The whitespace fence has a floor of 9 spaces** — a literal wrapped at ≤8 columns evades it. The
   floor is calibrated on measurement (≈521 legitimate runs at 2–8, **zero** at ≥9), not taste.
2. **Untrackable homes are guarded at the two user-settable doors and the shared `move_doc` primitive.**
   `migrate-corpus`'s relocation arm and `finalize`'s promote/retire take their destination from a
   *schema declaration*, so no CLI sequence reaches an untrackable home; hand-authoring a project schema
   shadow containing `.git/` remains open — an engine-side axis where a git query does not belong.
3. **`provision_worktrees` keeps four bare-`anyhow` pre-flight paths** (`canonicalize`,
   `create_dir_all`, `git worktree prune`, `registered_worktrees`) — environment failures before any
   subject exists. Converting generic I/O plumbing into findings is a repo-wide change with no stated
   contract behind it.
4. **`jigc setup` completes at exit 0 over a shape-changing project schema shadow** — declared, not
   accidental. It is the bootstrap door, resolves no doctype, and refusing to install over a drifted
   corpus is circular. The counter-case is on the record: the operator is told the install succeeded
   while every next door refuses.
5. **The 18-cell acceptance axis is a manufactured shape space, not a registry enumeration** — the
   shipped registry populates 2 of 18. A deliberate departure from M45/M47's pattern, stated in code
   and in prose.
6. **Planning's own instrument failed twice and was corrected both times** — the razor could not refuse
   until a necessity leg was added, and the settle record disposed ten *forks* while stating no *scope*
   until the review caught it.

---

## What this milestone claims

That the pre-1.0 window was spent deliberately: the one-way doors are closed, the migration surface can
express the changes the product's own doctypes need, and **three silent data-loss paths that no test and
no trial had ever reached are shut** — two of them found only because every fixer was told to derive its
class rather than trust the finding it was handed.

It does **not** claim the surface is complete. It claims the fences now bind over their classes, and
that where they do not, the bound is written down.
