```sh
src=$PWD                                              # the jigc checkout
rig=$("$src"/dev/jigc-rig fresh) || exit; eval "$rig"
mkdir -p docs/ideas
for n in one two; do printf -- '---\ntrigger: never\ndate: 2026-10-01\nschema-version: 1\n---\n\n# Note %s\n\n## Description\n\nA hand-written idea, %s.\n' $n $n > docs/ideas/note-$n.md; done
$JIGC ingest; echo "ingest: exit $?"                  # 0, both adoptable … adopted
git status --short --untracked-files=all docs         # ?? docs/ideas/note-one.md, ?? docs/ideas/note-two.md
# the sweep half, in the shape driven: an unrelated migration, adr bumped 2 -> 3 by an added optional field
mkdir -p docs/decisions
printf -- '---\nstatus: accepted\ndate: 2026-06-25\nschema-version: 2\n---\n\n# Alpha\n\n## Context\n\nC.\n\n## Options\n\nO.\n\n## Decision\n\nD.\n\n## Consequences\n\nQ.\n' > docs/decisions/alpha.md
git add docs/decisions && git commit -qm "a v2 adr"
export JIGC_PACK_DIR=$RIG/pack; cp -R "$src"/crates/cli/packs/dev "$JIGC_PACK_DIR"
grep -v 'id: cites-code' "$JIGC_PACK_DIR/schemas/adr.yaml" > "$JIGC_PACK_DIR/schema-snapshots/adr.v2.yaml"
sed -i.bak '/- type: adr/{n;s/schema-version: 2/schema-version: 3/;}' "$JIGC_PACK_DIR/config/schema-manifest.yaml"
$JIGC migrate-corpus; echo "migrate-corpus: exit $?"  # 0, 1 migrated
git show --name-only --format=%s HEAD                 # docs/decisions/alpha.md alone
git status --short --untracked-files=all docs         # the two ideas, still untracked
```
