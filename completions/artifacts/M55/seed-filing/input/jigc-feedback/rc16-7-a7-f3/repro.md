```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
mkdir -p docs/changelog
git mv CHANGELOG.md docs/changelog/changelog.md            # the v1 home
sed -i '' -e 's/schema-version: 2/schema-version: 1/' \
    -e '1,/# Changelog/s/# Changelog/# changelog/' docs/changelog/changelog.md   # GNU sed: -i alone
git add -A; git -c core.hooksPath=/dev/null commit -qm "v1-era changelog at its prior home"
$JIGC validate              # schema-conformance.schema-version-current → `jigc migrate-corpus`
$JIGC doc list changelog    # changelog:changelog  docs/changelog/changelog.md  managed
$JIGC doc show changelog    # exit 1, store.not-found
#   route: `changelog:changelog` is committed at `docs/changelog/changelog.md`, a prior home of
#     `changelog` — … run `jigc migrate-corpus` to land it at the home this read resolves,
#     then read it again
PATH="$(dirname "$JIGC"):$PATH" jigc migrate-corpus
$JIGC doc show changelog    # exit 0
```
