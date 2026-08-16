# The 1.0.0-gate trial — corpora as built

Built 2026-08-16, **before any session ran**, from the committed
[trial-corpus-template](../../trial-corpus-template/) with `--clean-prose`. Every one was gated by
`check-corpus.sh` in its naive state and passed **11/11**.

| Corpus | Instrument | Product name | State at freeze |
|---|---|---|---|
| `~/ideas/harborlight` | **B1** cold start | `harborlight` | naive — `jigc setup` runs *in* the session |
| `~/ideas/pinegrove` | **B2** design altitude | `pinegrove` | **adopted** — setup + invocation log committed |
| `~/ideas/stonefly` | **B3** corpus accretes | `stonefly` | **adopted** — setup + invocation log committed |
| `~/ideas/rc11-control` | **arm 0** positive control | `rc11svc` | naive |
| `~/ideas/rc11-walk` | **arms 1–4** operator walk | `rc11svc` | naive |

**Naming.** The three blind corpora carry product-plausible names with no `rc`/`trial`/`probe`/`gate`
token. The walk corpora are not blind and may say what they are. Note the host path is *not* what a
worker sees — the corpus is copied to `/work` in the container — so the load-bearing name is the one
in `package.json`, which is why the product name is listed above.

## Why B2 and B3 are adopted rather than naive

Per [protocol.md](protocol.md) §4's preload note: the only surfaces stating the read rule are
`CLAUDE.md` and `.jigc/AGENT.md`, **both written by `jigc setup`**. A session that runs setup itself
assembled its system prompt before either existed, so the adapter cannot be in its context — and a
0-VERB result would be substantially attributable to that, not to the product.

**Verified, two-sided, on the built corpora** (probe: *"do your loaded instructions tell you that a
managed doc staged in an open task must be read with a jigc command rather than from the file?"*):

```
pinegrove    YES        <- adapter loaded
stonefly     YES        <- adapter loaded
harborlight  NO         <- naive, setup has not run
```

That is the check the restructure rests on, and it discriminates.

**The adoption was performed by the container's own binary**, not the host's, so the state under test
is produced by the exact build the sessions run (`jigc setup` → `jigc config set invocation-log
true` → one commit). Resulting history: 7 template commits + setup's install commit + the adopt
commit = **9**, clean tree.

**The invocation log is pre-enabled on B2/B3.** §3.3's primary channel then does not depend on the
worker running `jigc config set invocation-log true`, and the prompt loses a line that is
operator instruction rather than task. B1 still carries it, because that session installs jigc.

## Reproducing

```sh
T=completions/trial-corpus-template
$T/instantiate.sh --clean-prose ~/ideas/<name> <product>
$T/check-corpus.sh ~/ideas/<name> --clean-prose       # must be 11/11 before freezing
```

To adopt (B2/B3 only), drive `jigc setup` **inside the image** rather than with the host binary, then
copy `/work` back out.

## Still owed before a session runs

Listed here rather than assumed: the three verbatim blind prompts · the three cue cards (trigger
string + verbatim correction) · the plants for B1 (rejecting `pre-commit` under `core.hooksPath`,
pre-staged files before the mint) and B3 (the mid-stream foreign ADR) **each rehearsed to firing on a
throwaway copy** · the answer key for the forks a worker predictably raises. A plant assumed to fire
is not a plant.
