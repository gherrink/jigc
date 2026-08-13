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
./instantiate.sh ~/ideas/<product-name> <product-name> ["<tagline>"]
```

Run it once per corpus. Each instantiation is an independent git repo with a 7-commit history and a
clean tree, so parallel blind sessions cannot contaminate each other.

**Clean-room naming rule:** give blind corpora **product-plausible** names with no `trial`/`rc*`/
`probe` token — a worker that reads its own `cwd` must learn nothing. Operator-walk corpora are not
blind and may be named for what they are.

## What is load-bearing about its shape

Each property was chosen for a probe, so changing it changes what a trial can measure:

| property | what it serves |
|---|---|
| **TypeScript** source (`.ts`) | `symbol-exists` resolves through the vendored TS grammar (`grammar_for`, `crates/cli/probes/doc-code/src/resolve.rs`), so `cites-code` / `implemented-by` anchors bind to **real** symbols |
| 9 modules with exported classes **and** functions | gives both symbol kinds real targets, and gives an arch-doc genuine components |
| 3 test files, 23 passing tests | `maps-to-test` — **0 writes in its entire history** — finally has somewhere to point |
| `node --test` with **zero dependencies** | the suite actually runs on any machine with Node ≥ 22.6, with no `npm install`, no network, no `node_modules` to pollute the ingest funnel |
| **zero managed docs**, README only | the "from nothing" premise is exact rather than approximated; the lone tracked `.md` is a realistic README, and whether the ingest funnel handles it sensibly is itself observable |
| 7 commits in dependency order | a plausible history for `git log` orientation, and each commit is a coherent working tree |
| an injectable `Clock`, a bounded queue, a per-series cap | real design decisions worth recording as ADRs — the corpus has something to *decide about*, which a toy CRUD app does not |

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
recorded in [RC-pre-1.0/operator-log.md](../RC-pre-1.0/operator-log.md).

If you *want* a clean corpus, fix the tagline before instantiating and say so in the protocol.

## Provenance

Built for the pre-1.0.0 trial (2026-08-12/13) and committed afterwards on the reasoning that made
that wave generous in the first place: **paying a cost once beats paying it every time.** Trial
protocol conventions that outlive a single run are recorded in
[RC-pre-1.0/findings-verification.md](../RC-pre-1.0/findings-verification.md) → *Process changes for
the next trial*.
