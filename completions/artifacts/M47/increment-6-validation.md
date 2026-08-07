# M47 Increment 6 — independent validation record (2026-08-05)

The increment-6 validator ran during the build and raised two findings, which were fixed
in `b85662d` and `90f2d8d`. Those fixes landed **after** the validator finished, so the
increment was left built-but-unrevalidated when the build was halted for the test-target
consolidation (`ba8743e`). This is the re-validation that closes it.

**Range validated:** `20686cf..90f2d8d` (increment 5 ended at `20686cf`). `ba8743e` — the
build-harness consolidation — is explicitly outside this assessment.

**Verdict: clean. Gate green from clean, every grouped-scope bullet holds through the real
binary, no blocking findings.** Gate: `fmt` 0 · `clippy --all-targets -D warnings` 0 ·
probe prebuild OK · `cargo test` unscoped 0 · `build` 0 — **2460 passed / 0 failed** over
17 targets (12 group targets + 3 unittest + 2 doc-test). All four new/changed suites are
registered in a group root, so none is silently unrun.

Both prior fixes were re-verified rather than taken on their commit messages: `90f2d8d`
holds across all six item-addressing doors plus bare `add-item` at a top-level undeclared
section, and `b85662d` holds over all three create branches (staged-with-prior-work,
committed copy-in, fresh mint) with bytes byte-identical and no leaked staged file.

## Advisory 1 — the undeclared **nested**-section hop is the increment's own class, un-swept one level down

`crates/engine/src/write.rs:6565` — `section_undeclared()` consults **top-level** sections
only, so the rank-1 predicate never sees an undeclared *nested* repeatable id. The six
item-addressing doors therefore disagree, returning four different codes for one defect:

| door | code |
|---|---|
| `set-slot` | `write.not-present` |
| `set-field --value` | `write.wrong-shape` |
| `set-field --unset` | `write.unknown-field` |
| `remove-item` | `write.not-present` |
| `retitle-item` | `write.wrong-shape` |
| `add-item` | `write.wrong-shape` |

The `--unset` cell is verbatim the sentence `90f2d8d`'s own message names as the defect it
removed. `item_chain_absent`'s doc comment (`write.rs:5886`) says the whole family should be
one shape code.

**Not a regression** (the `set-slot`/`remove-item` answer predates the increment —
`git show 20686cf:crates/engine/src/write.rs`, `set_nested_item_slot` maps
`nested_parsed_item` `None` to `SpliceError::NotPresent`), **not a named grouped-scope
bullet**, and harmless in effect: every door blocks non-zero, bytes stay byte-identical,
and every emitted route runs at exit 0. Advisory for that reason — but it is **the wave's
own named class through an un-swept axis**, which is what M45's complete-fix contract
exists to catch, so it belongs on the axis table as six rows rather than being rediscovered
in a later trial.

## Advisory 2 — `design/validation.md:73` states `write.unknown-section` universally; the bound lives only in a commit message and a test comment

Two address forms falsify the sentence *"the address names a section the schema does not
declare"*:

1. **Section-level forms never reach a write door** — `set-slot` at `#nope`, `set-field` at
   `#nope/bogus`, and `doc author` with an undeclared `- id:` all exit 1 with a bare
   `{"error": "no slot addressed by …"}`: code-less, route-less, outside the finding
   envelope. Nothing is written and no prior work is lost, so this is a **surface** gap, not
   a correctness one.
2. **A nested undeclared hop earns three other codes** (advisory 1).

The bound is stated at `crates/cli/tests/write_miss_shape_axis.rs:74-79` and in `90f2d8d`'s
message, but **not in the design of record** — which the wave's own statement-is-a-fence
rule governs. Either the bullet carries its declared bound, or the seam comes under the
taxonomy.

## Disposition

Neither advisory blocks increment 6, and neither is in its named scope. Both are carried to
the **M47 completion audit** — advisory 1 as an axis-completeness item (the wave that ships
the complete-fix contract should not leave its own class half-swept), advisory 2 as a
law-1 surface-truth item.
