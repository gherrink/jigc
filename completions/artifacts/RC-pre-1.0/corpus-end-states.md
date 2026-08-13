# Corpus end states — captured 2026-08-13, immediately before the corpora were destroyed

The corpora themselves are gone; this and `logs/` are what survive them. Counts in
[trial-record.md](trial-record.md) are re-derivable from `logs/*.jsonl`.

## gaugeline

```
commits: 10
7bbf67f docs(ingest): record the ingest overflow drop-oldest policy as an ADR
e80e0c2 chore(jigc): install jigc workspace config
6b70b9f chore: docs review policy hook
e9f6abb feat: router and service wiring
5948dd4 feat: aligned window rollups and summaries
da17846 feat: in-memory per-series sample store
f5d92dd feat: bounded ingest queue and wire-line parser
7677e3a feat: injectable clock and env-read config
6234012 feat: sample validation with a named offending field
b864ea3 chore: project skeleton

-- managed docs --
id  path  state
adr:ingest-overflow-drops-the-oldest  docs/decisions/ingest-overflow-drops-the-oldest.md  managed

-- working tree at close --
 M .githooks/pre-commit
 M src/router.ts
?? .githooks/docs-approved
?? scripts/
```

## windowpane

```
commits: 16
f7d1110 chore(milestone): record task:i4-bound-the-series-count on milestone:m1-the-service-runs-and
4cb23e4 chore(milestone): record task:i3-drive-store-prune-from on milestone:m1-the-service-runs-and
28c77a9 chore(milestone): record task:i2-route-router-ingest-through on milestone:m1-the-service-runs-and
63da122 chore(milestone): record task:i1-bind-a-socket-serving on milestone:m1-the-service-runs-and
628ca52 chore(milestone): open record for milestone:m1-the-service-runs-and
0fb08a4 docs(planning): plan M1 and open the milestone spine
ec3a392 docs(vision): commit to the recent-window query service that keeps its samples
7f03f92 docs(research): survey the in-memory rollup landscape and locate windowpane in it
73796f3 chore(jigc): install jigc workspace config
ef39878 feat: router and service wiring
ef238dd feat: aligned window rollups and summaries
047ce5a feat: in-memory per-series sample store
be8c622 feat: bounded ingest queue and wire-line parser
d60b857 feat: injectable clock and env-read config
d79a2a7 feat: sample validation with a named offending field
a711517 chore: project skeleton

-- managed docs --
id  path  state
decisions-log:decisions-log  docs/decisions-log.md  managed
deferral-ledger:deferral-ledger  docs/deferral-ledger.md  managed
milestone-record:m1-the-service-runs-and  docs/milestone-records/m1-the-service-runs-and.md  managed
research:what-already-exists  docs/research/what-already-exists.md  managed
roadmap:roadmap  docs/roadmap.md  managed
vision:vision  VISION.md  managed

-- working tree at close --
```

## tidepool

```
commits: 15
dad9ca6 docs(changelog): record the write-ahead log as unreleased
9db7efe docs(architecture): document how ingest, store and rollup fit together
6946369 feat(store): persist samples to a write-ahead log
ac08936 docs(decisions): bring the in-memory store ADR under management
dcaf27d docs(store): specify durable sample persistence
009d43a docs: record the in-memory store decision
653dc2a docs(changelog): record what shipped in 0.3.0
75cb7bf chore(jigc): install jigc workspace config
d45bab6 feat: router and service wiring
e9bd7cc feat: aligned window rollups and summaries
d01417f feat: in-memory per-series sample store
26129fb feat: bounded ingest queue and wire-line parser
0a281ae feat: injectable clock and env-read config
57a6178 feat: sample validation with a named offending field
8520a34 chore: project skeleton

-- managed docs --
id  path  state
adr:keep-the-sample-store  docs/decisions/keep-the-sample-store.md  managed
adr:persist-samples-to-a-write  docs/decisions/persist-samples-to-a-write.md  managed
arch-doc:ingest-store-and-rollup  docs/architecture/ingest-store-and-rollup.md  managed
changelog:changelog  CHANGELOG.md  managed
spec:durable-sample-persistence  docs/specs/durable-sample-persistence.md  managed

-- working tree at close --
```

## rc10-walk

```
commits: 12
52e4d30 chore(jigc): migrate the managed corpus to the current schema versions
98b3200 chore: docs metadata sync
b6c5327 docs(adr): adopt the backpressure design notes as an ADR
6a286a7 docs: backpressure notes from the design discussion
d368854 chore(jigc): install jigc workspace config
0bb9f80 feat: router and service wiring
02f8731 feat: aligned window rollups and summaries
535f784 feat: in-memory per-series sample store
4355a78 feat: bounded ingest queue and wire-line parser
23cae8e feat: injectable clock and env-read config
8328153 feat: sample validation with a named offending field
63b1f72 chore: project skeleton

-- managed docs --
id  path  state
adr:backpressure-drops-the-oldest-sample  docs/decisions/backpressure-drops-the-oldest-sample.md  managed

-- working tree at close --
```

## rc10-fanout

```
commits: 11
622c636 chore(milestone): record task:replay-the-log-on-startup on milestone:durable-sample-tier
0379320 chore(milestone): record task:add-a-write-ahead-log on milestone:durable-sample-tier
c755011 chore(milestone): open record for milestone:durable-sample-tier
ecec53e chore(jigc): install jigc workspace config
6aa52b5 feat: router and service wiring
c5cb321 feat: aligned window rollups and summaries
059a12b feat: in-memory per-series sample store
85a7d7f feat: bounded ingest queue and wire-line parser
f864b5c feat: injectable clock and env-read config
23a5246 feat: sample validation with a named offending field
e6f24cd chore: project skeleton

-- managed docs --
id  path  state
milestone-record:durable-sample-tier  docs/milestone-records/durable-sample-tier.md  managed

-- working tree at close --
?? .jigc/config/manifest.yaml
```

## rc9-legacy

```
commits: 17
a3ab50e fix: clamp the retention prune bound at zero
3decee1 rename docs/decisions/sample-store-stays.md -> docs/decisions/durability-belongs-to-the-caller.md
201b520 chore(milestone): discard record for milestone:durable-sample-tier
f763587 chore(milestone): record task:add-a-write-ahead-log on milestone:durable-sample-tier
da59294 chore(milestone): open record for milestone:durable-sample-tier
2c1b700 docs: start the changelog
d5cca25 docs: document the ingest to rollup path
608c03d docs: spec the durable sample tier
9a075f1 docs: record the in-memory store decision
d41eca4 chore(jigc): install jigc workspace config
026e982 feat: router and service wiring
e6555e5 feat: aligned window rollups and summaries
2612cc4 feat: in-memory per-series sample store
1d45b92 feat: bounded ingest queue and wire-line parser
244f955 feat: injectable clock and env-read config
f851f82 feat: sample validation with a named offending field
39013cb chore: project skeleton

-- managed docs --
id  path  state
adr:durability-belongs-to-the-caller  docs/decisions/durability-belongs-to-the-caller.md  managed
arch-doc:ingest-to-rollup-path  docs/architecture/ingest-to-rollup-path.md  managed
changelog:changelog  CHANGELOG.md  managed
milestone-record:durable-sample-tier  docs/milestone-records/durable-sample-tier.md  managed
spec:durable-sample-tier  docs/specs/durable-sample-tier.md  managed

-- working tree at close --
```

