```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
printf 'compose-embedded-methodology: false\n' > .jigc/config/packs.yaml
git add -A; git -c core.hooksPath=/dev/null commit -qm "methodology off"
$JIGC validate --format json       # exit 1
#   schema-conformance.orphaned-instance  docs/decisions-log.md
#   schema-conformance.orphaned-instance  docs/roadmap.md
#   each route: "ask `jigc ingest`, which re-reads the file: …"
$JIGC ingest                       # exit 0
#   unmanaged docs/ — 2 file(s) parse against no schema (left untouched — fine to stay plain)
#   unmanaged — matches no managed schema; staying a plain file is a legitimate end-state —
#     no action needed. …
$JIGC ingest --format json         # the rows: verdict "unmanaged", finding null, annotations []
```
