# Corpora as built

Built 2026-09-04, **before any session ran**, from the committed
[trial-corpus-template](../../trial-corpus-template/) with `--clean-prose`. Order, not
negotiable: **instantiate → `check-corpus.sh` (11/11) → adopt → plant**. Adoption ran through
**the container's own binary** (`run-session.sh --exec arms/adopt.sh … jigc-gate:rc13`), so the
state under test was produced by the exact build the sessions run; every adopted corpus is 7
template commits + setup's install commit + the adopt commit = **9**, clean, invocation log ON.

| Corpus (frozen state) | Arm | Transport | State at freeze |
|---|---|---|---|
| `~/ideas/larkspur` | **B1** cold start | interactive | **PLANTED, naive** — rejecting docs-gate hook under `core.hooksPath=.githooks`, marker absent; `A scripts/retention-sweep.sh` · `M src/router.ts` staged before the first mint; `jigc setup` runs *in* the session |
| `~/ideas/quillon-planted` | **B2** design altitude | interactive | adopted, then **plant E** |
| `~/ideas/ashgrove-planted` | **B3** corpus accretes | headless | adopted, then **plant E**; the foreign-ADR plant fires mid-session |
| `~/ideas/brackenmoor-planted` | **B3-h2** | headless | adopted, then **plant E** — the second headless reading of the duress cell |
| `~/ideas/saltmarsh-adopted` | **B4-h** plan the first milestone | headless | adopted, **no plant** — an observation arm |
| `~/ideas/ashgrove-strict` | **B3-strict** | headless `--strict-permissions` | a directory copy of `ashgrove-planted`; **unscored** |
| `~/ideas/r3-corpus-planted` | **R3** rehearsal | headless | adopted + plant E |
| `~/ideas/r4-corpus` | **R4** rehearsal | headless | B1's two plants, naive |
| `~/ideas/walk-m50` | the walk | per arm | naive at instantiation; each arm builds its own state |
| `~/ideas/walk-m50-mig` | the migration pair | rc.12 then rc.13 | naive; arm 14 authors it on rc.12 |

**Naming.** Product-plausible, no `rc`/`trial`/`probe`/`gate` token in a blind corpus's
`package.json` name; the walk and rehearsal corpora may say what they are.

## Gated, then frozen — asserted rather than assumed

`check-corpus.sh --clean-prose` on each naive corpus, before adoption or planting:

```
larkspur 11/11 · quillon 11/11 · ashgrove 11/11 · brackenmoor 11/11 · saltmarsh 11/11
walk-m50 11/11 · walk-m50-mig 11/11 · r3-corpus 11/11 · r4-corpus 11/11
```

Plant E, driven through the container's binary on each adopted corpus, **11/11 bars** each —
including the one the headline depends on, *the plant left no records in the channel it is
measured on*:

```
quillon-planted      commits=9  tree=clean  task=record-the-ingest-queue-overflow  log=0 bytes
ashgrove-planted     commits=9  tree=clean  task=record-the-ingest-queue-overflow  log=0 bytes
brackenmoor-planted  commits=9  tree=clean  task=record-the-ingest-queue-overflow  log=0 bytes
r3-corpus-planted    commits=9  tree=clean  task=record-the-ingest-queue-overflow  log=0 bytes
larkspur             commits=8  tree=DIRTY BY DESIGN (the index is the plant)  hook=present  marker=absent
saltmarsh-adopted    commits=9  tree=clean  no task
```

The plants are reused **byte-identical** from the prior trials, not copied; their sources and
hashes are in [plants/README.md](plants/README.md).

## The preload split, verified two-sided

Protocol §4's arms split on whether `.jigc/AGENT.md` is in the worker's context at session start.
Probed headless on the frozen corpora with one question (*"do your loaded instructions mention a
rule that all writes to project documents go through a CLI called jigc?"*, one word out):

```
larkspur            NO    <- naive; setup has not run, so the adapter cannot be loaded
quillon-planted     YES   <- adapter loaded
saltmarsh-adopted   YES   <- adapter loaded
```

It discriminates. B1's read-back is reported **separately and discounted**, with this stated.

## Reproducing

```sh
T=completions/trial-corpus-template
$T/instantiate.sh --clean-prose ~/ideas/<name> <name>
$T/check-corpus.sh ~/ideas/<name> --clean-prose                    # 11/11 BEFORE anything else
./completions/trial-harness/run-session.sh --exec completions/trial-driver/arms/adopt.sh \
    ~/ideas/<name> <out>/adopt-<name> jigc-gate:rc13
python3 completions/trial-driver/run.py carry <out>/adopt-<name> ~/ideas/<name>-adopted
./completions/trial-harness/run-session.sh --exec \
    completions/artifacts/RC-1.0-final/plants/e-abandoned-task.sh \
    ~/ideas/<name>-adopted <out>/plantE-<name> jigc-gate:rc13
python3 completions/trial-driver/run.py carry <out>/plantE-<name> ~/ideas/<name>-planted
# B1 shape, on the naive corpus, host side:
completions/artifacts/RC-1.0-gate/plants/b1-hook.sh   ~/ideas/<name>
completions/artifacts/RC-1.0-gate/plants/b1-staged.sh ~/ideas/<name>
```
