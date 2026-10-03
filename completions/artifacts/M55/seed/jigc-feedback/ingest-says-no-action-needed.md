---
kind: bug
found-in: review:M52-per-axis/(7,A7-F2)
about: jigc ingest
jigc-version: 1.0.0-rc.15
status: open
tier: tier-3
date: 2026-10-03
schema-version: 1
---

# Ingest says no action needed on files validate blocks

## Description

`jigc ingest` says *no action needed* about the files `jigc validate` blocks on. At one commit, with the methodology pack taken out of the composition, `validate` exits 1 with `schema-conformance.orphaned-instance` on `docs/decisions-log.md` and `docs/roadmap.md`. Each route's first exit is *ask `jigc ingest`, which re-reads the file …*. `ingest` exits 0 and files both under *unmanaged docs/ — 2 file(s) parse against no schema (left untouched — fine to stay plain)*, with the legend *staying a plain file is a legitimate end-state — no action needed*. Its JSON rows carry `finding: null`. At the *ahead* and *below* cells `ingest` is currency-blind by declared design and makes no positive claim. Here it makes one, about a path another door blocks on, and it is the door `validate`'s own route hands the reader to first. The M52 review recorded this row as half 2 of M51's lead `(7, D-4)`, driven on `1.0.0-rc.15`, which is why `jigc-version` names that release.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `a768fe75`): **still open**. `validate --format json` exits 1 with the two `schema-conformance.orphaned-instance` rows, each routing first at `jigc ingest`. `jigc ingest` exits 0 with *unmanaged docs/ — 2 file(s) … fine to stay plain* and the *no action needed* legend. Its JSON rows carry no finding.

## Repro

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

## Resolution
