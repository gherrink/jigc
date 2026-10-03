---
kind: bug
found-in: review:M52-per-axis/(7,A7-F3)
about: jigc doc show
jigc-version: 1.0.0-rc.16
status: resolved
tier: tier-2
pinned-by: doc_show_relocated::a_doc_at_a_recorded_prior_home_is_routed_at_the_migration
date: 2026-10-03
schema-version: 1
---

# Doc show over a relocated doc routes at no repair

## Description

`jigc doc show` refused a relocated managed doc with a route that named none of the repair paths the same commit's other doors name. At one commit, `jigc validate` routed at `jigc migrate-corpus`, and `jigc doc list` called the doc `managed` at its stale home. `jigc doc show` exited 1 with `store.not-found — could not read … at <new home>`. It offered *create the referenced doc, fix the reference, or read it with `--task`*, and none of those is the repair. The reconciler widened it from the `location` home kind to the `placement` one.

## Repro

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

## Resolution

Fixed in M53's pre-v1 usability batch (DECISIONS.md → *2026-09-23 — the pre-v1 usability batch: six surface rows*). The read's reroute sits at the verb boundary, on the seam the `store.unparseable` adoption arm already uses. It asks the two walks that own *where a managed document lives*, in the order `jigc validate` asks them. A recorded prior home routes at `jigc migrate-corpus`, and a knob re-point routes at the strand repair. An address that names nothing keeps the shipped route, byte for byte.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `a768fe75`). A v1-era changelog at its prior home makes `jigc doc show changelog` exit 1 with `store.not-found`. Its route names where the doc is committed and routes at `jigc migrate-corpus`, the verb `jigc validate` names over the same corpus. After that verb runs, the read exits 0. The pinning test's three siblings in the same suite cover the `docs-root` strand, the `placement-root` strand and the absent-address control.
