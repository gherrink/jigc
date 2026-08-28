# Corpora as built

Built 2026-08-28, **before any session ran**, from the committed
[trial-corpus-template](../../trial-corpus-template/) with `--clean-prose`.

| Corpus (frozen state) | Arm | Transport | State at freeze |
|---|---|---|---|
| `~/ideas/harbourgate` | **B1** cold start | interactive | **PLANTED** — rejecting docs-gate hook + a pre-mint staged set; `jigc setup` runs *in* the session |
| `~/ideas/pinewick-planted` | **B2** design altitude | interactive | adopted, then **plant E** |
| `~/ideas/stonecross-planted` | **B3** corpus accretes | headless | adopted, then **plant E**; foreign-ADR plant fires mid-session |
| *(a second copy of stonecross)* | **B3-strict** | headless `--strict` | same; **unscored** |
| `~/ideas/walk*` | the walk | per arm | naive at instantiation; each arm builds its own state |

**Naming.** Product-plausible, with no `rc`/`trial`/`probe`/`gate` token. The host path is not
what a worker sees — the corpus is copied to `/work` — so the load-bearing name is the one in
`package.json`, which is the corpus name above.

## Order, and it is not negotiable

```
instantiate  ->  check-corpus.sh (11/11)  ->  adopt  ->  plant
```

**Gate first, plant second.** Every plant here deliberately breaks bars the naive gate asserts —
B1's breaks four of them — so gating a planted corpus proves nothing and re-gating one is a
category error. Verified in that order:

```
harbourgate  11 passed, 0 failed      pinewick  11 passed, 0 failed      stonecross  11 passed, 0 failed
```

**Adoption runs the CONTAINER's binary**, through `run-session.sh --exec arms/adopt.sh`, so the
state under test is produced by the exact build the sessions run. 7 template commits + setup's
install commit + the adopt commit = **9**, clean tree.

**`run.py carry` between every step.** `run-session.sh` writes its own evidence
(`PROVENANCE.txt`, `stream.jsonl`, `stderr.txt`, `.session-transcript/`) into the out-dir, and
handing that on plants the rig's droppings in the corpus a worker reads.

## Frozen state, asserted rather than assumed

```
harbourgate          commits=8   tree=DIRTY BY DESIGN   hook=present  marker=absent (rejecting)
                     staged: A scripts/retention-sweep.sh · M src/router.ts   <- the carryover plant
pinewick-planted     commits=9   tree=clean   task=record-the-ingest-queue-overflow
stonecross-planted   commits=9   tree=clean   task=record-the-ingest-queue-overflow
```

B1's tree is dirty **because the index is the plant** — the carryover gate's subject is a staged
set that exists before the first mint. `check-corpus.sh` would now fail this corpus, correctly
and by design.

Plant E passed **11/11 bars** on both corpora, including the one that matters most for the
measurement: *the plant left no records in the channel it is measured on*. Both corpora's
invocation logs are **empty** at freeze, so every record a session produces is the worker's.
See [rehearsal-R1.md](rehearsal-R1.md) for why that bar exists — without it the plant inflated
the headline threefold.

## The preload split, verified two-sided

[protocol.md](protocol.md) §4's arms split on whether `.jigc/AGENT.md` — the only surface that
states the read rule in words — is in the worker's context at session start. A session that runs
`jigc setup` itself assembled its system prompt before that file existed.

Probed on the frozen corpora with the exact question, one word out:

```
harbourgate   NO     <- naive; setup has not run, so the adapter cannot be loaded
pinewick      YES    <- adapter loaded
stonecross    YES    <- adapter loaded
```

**It discriminates**, which is what the split rests on. B1's read-back measurement is therefore
reported **separately and discounted**, with its preload state stated.

## Rider C is dropped, and here is why

[cue-card-postmortem.md](../RC-1.0-gate/cue-card-postmortem.md) §5C proposes seeding a slug
collision as free opportunistic coverage, and names its own falsifier: *"measure the collision
rate. Take the four archived sessions' actual titles against a candidate seed slug; if fewer than
half collide, the rider is decoration."*

The 1.0.0-gate evidence answers it: across four sessions **no two workers titled the same concept
the same way** (`bound-distinct-series-count` vs `bound-the-number-of-distinct`, for one identical
prompt intent). The bet loses more often than it wins, **and a lost bet is silent** — criterion 5,
which is the property that killed the cue card.

Its coverage is bought elsewhere and deterministically: `write.title-ignored` and the whole
identity refusal set are driven by [walk arm 08](../../trial-driver/arms/walk/08-identity-refusals.sh),
which reaches them on purpose rather than by luck.

## Reproducing

```sh
T=completions/trial-corpus-template
$T/instantiate.sh --clean-prose ~/ideas/<name> <name>
$T/check-corpus.sh ~/ideas/<name> --clean-prose          # must be 11/11 BEFORE freezing
./completions/trial-harness/run-session.sh --exec completions/trial-driver/arms/adopt.sh \
    ~/ideas/<name> ~/out/adopt-<name> jigc-gate:rc12
python3 completions/trial-driver/run.py carry ~/out/adopt-<name> ~/ideas/<name>-adopted
./completions/trial-harness/run-session.sh --exec \
    completions/artifacts/RC-1.0-final/plants/e-abandoned-task.sh \
    ~/ideas/<name>-adopted ~/out/plantE-<name> jigc-gate:rc12
python3 completions/trial-driver/run.py carry ~/out/plantE-<name> ~/ideas/<name>-planted
```
