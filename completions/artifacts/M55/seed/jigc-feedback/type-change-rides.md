---
kind: bug
found-in: milestone:M55-planning/F12
about: jigc migrate-corpus
jigc-version: 1.0.0-rc.22
status: open
date: 2026-10-03
schema-version: 1
---

# A type change rides an optional-flag change through migrate-corpus

## Description

A string→`code-anchor` type change rode along unclassified when it was combined with an optional-flag change. `jigc migrate-corpus` stamped it, *3 would migrate, 0 blocked*, and left blocking anchors behind. A pure type change has no transform kind (`crates/engine/src/schema_diff.rs:13-15`) and is refused by the empty-diff backstop, but the backstop fires only on an otherwise-empty diff, so one classified change beside it lets the unclassified one through. The doctypes gap-detector found it.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `982f910f`): **still open**. Over a dev-pack copy whose `adr` is bumped 2→3, with the v2 snapshot declaring `cites-code` a `string` and the `options` slot required, a committed v2 ADR whose `cites-code` is prose migrates: `jigc migrate-corpus` prints `1 migrated, 0 already current, 0 blocked`, exits 0, commits, and restamps the doc `schema-version: 3`. The next `jigc validate` reports a blocking-at-finalize `doc-code.symbol-exists` at `adr:alpha#status/cites-code`. The same type change alone is refused, which `corpus_migration_backstop::the_empty_diff_backstop_fires_at_the_default_docs_root` pins.

## Repro

```sh
src=$PWD                                              # the jigc checkout
rig=$("$src"/dev/jigc-rig bare) || exit; eval "$rig"
export JIGC_PACK_DIR=$RIG/pack; cp -R "$src"/crates/cli/packs/dev "$JIGC_PACK_DIR"
# the v2 adr: `cites-code` a string, and the `options` slot required
sed -e 's/{ id: cites-code, type: code-anchor }/{ id: cites-code, type: string }/' \
    -e 's/slot: { optional: true, hint: "Alternatives/slot: { hint: "Alternatives/' \
    "$JIGC_PACK_DIR/schemas/adr.yaml" > "$JIGC_PACK_DIR/schema-snapshots/adr.v2.yaml"
sed -i.bak '/- type: adr/{n;s/schema-version: 2/schema-version: 3/;}' "$JIGC_PACK_DIR/config/schema-manifest.yaml"
mkdir -p .jigc/config docs/decisions
printf -- '---\nstatus: accepted\ndate: 2026-06-25\ncites-code: the session cache module\nschema-version: 2\n---\n\n# Alpha decision\n\n## Context\n\nC.\n\n## Options\n\nO.\n\n## Decision\n\nD.\n\n## Consequences\n\nQ.\n' > docs/decisions/alpha.md
git add docs && git commit -qm "a v2 adr"
$JIGC migrate-corpus; echo "migrate-corpus: exit $?"   # 0: 1 migrated, 0 blocked
grep '^schema-version' docs/decisions/alpha.md         # schema-version: 3
$JIGC validate                                          # blocking (gates at finalize) doc-code.symbol-exists
```

## Resolution
