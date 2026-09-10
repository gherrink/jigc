# The trial corpus template — a reusable pre-jigc project for adoption trials

**Why this exists.** Every RC trial needs a **foreign, pre-jigc repo** to adopt, and every one so far
built its own and then lost it: the dashboard corpora died with the machine, and the pre-1.0.0
trial's six corpora were built from scratch and destroyed at close. That is the same setup cost paid
once per trial, forever. This is that cost paid once.

**What it is *not*.** It is **not** a test fixture and must not be confused with one.
`crates/cli/tests/support/trial_corpus.rs` is the fixture builder — it constructs *managed* states by
driving the current binary, for the suites. This is the opposite: a **plain project that has never
met jigc**, with zero managed docs, which is exactly the state a fixture builder cannot produce
because everything it makes is already adopted. Nothing in the gate references this directory; it is
operator tooling for a trial, and it lives under `completions/artifacts/` for that reason.

## Usage

```sh
./instantiate.sh [--clean-prose] ~/ideas/<product-name> <product-name> ["<tagline>"]
./check-corpus.sh ~/ideas/<product-name> [--clean-prose]
```

Run it once per corpus. Each instantiation is an independent git repo with a 7-commit history and a
clean tree, so parallel blind sessions cannot contaminate each other.

**Always run `check-corpus.sh` before a corpus is frozen and pointed at a session.** It asserts the
starting state a trial protocol assumes and has never verified — **12 bars**, in script order:
7 commits · clean working tree · no `.jigc`/`.claude`/`CLAUDE.md` residue **including a non-sample
file in git's hooks dir** (a `pre-commit` survives `git reset --hard` and `git clean -fdx` and is
invisible to `git status`) · no remote URL · `core.hooksPath` unset · a 7-entry reflog · on `main` ·
`README.md` the only tracked `.md` · the suite green 24/24 · the `doc-code` anchor symbols present ·
**those symbols reached from the live write path, driven rather than grepped** · and, under
`--clean-prose`, no surviving forwarding-shaped claim. RC-pre-1.0 stated a shorter version
of that checklist as prose and nothing ever checked it — a corpus carrying a rehearsal's leftover
`.jigc/` is not a cold start, and nothing a worker does in one measures what the protocol says it
measures.

Its bars are demonstrated to fail, not assumed to: `self-test.sh` runs **13 mutations** against the
12 bars — including a dirty tree, an already-adopted corpus, a leftover hook, a second tracked `.md`,
and a wart corpus asked for clean prose — and requires that the *named* bar is the one reporting
FAIL, never merely that the gate went red.

**Clean-room naming rule:** give blind corpora **product-plausible** names with no `trial`/`rc*`/
`probe` token — a worker that reads its own `cwd` must learn nothing. Operator-walk corpora are not
blind and may be named for what they are.

## What is load-bearing about its shape

Each property was chosen for a probe, so changing it changes what a trial can measure:

| property | what it serves |
|---|---|
| **TypeScript** source (`.ts`) | `symbol-exists` resolves through the vendored TS grammar (`grammar_for`, `crates/cli/probes/doc-code/src/resolve.rs`), so `cites-code` / `implemented-by` anchors bind to **real** symbols |
| 9 modules with exported classes **and** functions | gives both symbol kinds real targets, and gives an arch-doc genuine components |
| 3 test files, 24 passing tests | `maps-to-test` — **0 writes in its entire history** — finally has somewhere to point |
| `node --test` with **zero dependencies** | the suite actually runs on any machine with Node ≥ 22.6, with no `npm install`, no network, no `node_modules` to pollute the ingest funnel |
| **zero managed docs**, README only | the "from nothing" premise is exact rather than approximated; the lone tracked `.md` is a realistic README, and whether the ingest funnel handles it sensibly is itself observable |
| 7 commits in dependency order | a plausible history for `git log` orientation, and each commit is a coherent working tree |
| an injectable `Clock`, a bounded queue, a per-series cap | real design decisions worth recording as ADRs — the corpus has something to *decide about*, which a toy CRUD app does not |

## A second wart, and this one is closed (PT-D)

`IngestQueue.push()` was called from **no live path** and `tick()` from nothing at all — the router
wrote straight to the store, so the queue's overflow policy, its dropped counter and
`MemoryStore.prune` were all unreachable outside the unit tests. It survived **three trials**. In
RC-m50 four workers found it and two filed it as a deferral, which is worker budget spent on the
fixture rather than on the product; worse, **plant E's entire subject is that overflow policy**, and
a plant whose subject is dead code has *"the worker fixes the code instead"* as its falsifier.

Closed 2026-09-09, before the trial that follows M50: `Router` takes the queue and buffers into it,
`main()` ticks, and the gate gained a **12th bar that drives the path rather than grepping for it** —
POST, assert the store is still empty, tick, assert the sample landed. It fails both ways, when the
queue is bypassed and when the drain is severed, and `self-test.sh`'s 13th mutation is the historical
defect itself (`this.queue.push(` → `this.store.put(`), so the bar is proven able to fail on the
exact shape it exists for. The suite moves 23 → 24 for the one test that covers the wiring.

**`src/ingest.ts` is deliberately untouched** — its doc-comment (*"Overflow drops the \*oldest\*
sample, not the newest"*) is the committed naming authority plant E's own bar greps verbatim, and the
plant is md5-pinned.

## A known wart, deliberately left in

`README.md` and `package.json` describe the service as *"a rollup cache … in front of whatever
long-term store you already have"*, while `src/router.ts` has **no egress and no upstream reader** —
so it is a buffer, not a cache that refills. **This is a genuine inconsistency and it was
unintentional**, introduced when the template was first authored for the pre-1.0.0 trial.

It is kept rather than fixed because two blind sessions found it independently, from different entry
points (`do-research` before forming a vision; reading `store.ts` before writing a spec), and both
routed it to the human as a fork instead of silently resolving it — which turned out to be one of
that trial's better observations about the design-altitude paths. **A future trial must not report it
as a designed trap**: it is an accident that proved useful, and the honesty of that distinction is
recorded in [RC-pre-1.0/operator-log.md](../artifacts/RC-pre-1.0/operator-log.md).

### The escape hatch, and why the old one did not work

This section used to say: *"If you want a clean corpus, fix the tagline before instantiating and say
so in the protocol."* **That advice did not produce a clean corpus**, and a trial that followed it
would have believed otherwise.

The tagline argument reaches `package.json` and `README.md`. The same claim lives in a **third**
place it never touched — `src/store.ts`'s header comment, *"this service is a rollup cache in front
of whatever long-term store the caller already has"* — which is precisely the file G3 read to find
the contradiction ([RC-pre-1.0/operator-log.md](../artifacts/RC-pre-1.0/operator-log.md)). The lever left the
wart in the most-read location.

**Use `--clean-prose`.** It rewrites all three sites and then **greps its own work**, failing the
instantiation if any occurrence survives — because a silent no-op here (someone rewords the header,
the template drifts) hands a trial a corpus it believes is clean and is not. Without the flag the
corpus is byte-identical to what the pre-1.0.0 trial ran on, so that trial stays reproducible.

`check-corpus.sh <dir> --clean-prose` re-checks the same property from the outside, so the claim does
not rest on the builder having done its job.

## Provenance

Built for the pre-1.0.0 trial (2026-08-12/13) and committed afterwards on the reasoning that made
that wave generous in the first place: **paying a cost once beats paying it every time.** Trial
protocol conventions that outlive a single run are recorded in
[RC-pre-1.0/findings-verification.md](../artifacts/RC-pre-1.0/findings-verification.md) → *Process changes for
the next trial*.
