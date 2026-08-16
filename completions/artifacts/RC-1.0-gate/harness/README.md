# The 1.0.0-gate trial's isolation harness

Built 2026-08-15, **before the trial runs**, and rehearsed end to end before anything was
staged. The trial it serves is [protocol.md](../protocol.md); this file is the
**apparatus**, not a restatement of the design.

## Why it exists

Every jigc RC trial through RC-pre-1.0 ran as an ordinary interactive session on the
operator's machine, which loads `~/.claude/CLAUDE.md`. The measured conduct was therefore
jigc **plus** whatever global instructions the operator keeps, and nothing separated the
two.

**Verified rather than assumed, 2026-08-15.** From `/tmp`, with no project anywhere near
it, a host session answered **YES** to *"do your loaded instructions mention a bash output
filter, or a rule about asking the user only one question at a time?"* — the operator's
`PRINCIPLES.md` and `LACON.md`, in the context of every worker. The same probe answers
**NO** inside this image.

That matters most for the headline measurement, because the confound's **direction is
unknown**: `PRINCIPLES.md` says *"never ask what you can find out yourself"* and
*"understand the existing code before changing it"*, which plausibly push a worker toward
reading — but toward reading **files**, which protocol.md §3.3 scores as FILESYSTEM, the
non-VERB outcome. A confound whose sign cannot be determined cannot be corrected for
afterwards, so it is removed instead.

**A redirected config dir does not defeat it.** This project's own M17 pilot tried exactly
that — `CLAUDE_CONFIG_DIR` at a clean home *plus* moving the global file aside
([M17/pilot/runbook.md](../../M17/pilot/runbook.md)) — and it was recorded as
insufficient. Only filesystem isolation works.

## The lineage, and what came back

This repo is the origin: `implementation/dogfood/` (hooks + tally),
[differentiator-pilot-study1/harness/](../../differentiator-pilot-study1/harness/) (the
container rig that produced the 0/32 figure), [workflow-eval/](../../../workflow-eval/)
(the generalized version), then the long-horizon and cross-doc-refint harnesses.

The mechanism was borrowed by a sibling project and improved in four places, all of which
are folded back in here: the CLI is **installed into the image** rather than mounted (a
Mach-O arm64 binary cannot execute in a Linux container — the pilot mounted it because
that host was Linux); the project is **copied in and out** rather than mounted (colima
serves host mounts read-only, silently producing sessions that appear to change nothing);
the CLI version, and the binary under test, are **pinned**; and permissions are explicit
rather than inherited from the operator's `settings.json`.

## The four scripts

| Script | What it does |
|---|---|
| `build-image.sh <sha> [tag]` | Builds jigc for Linux from a `git archive` of that exact commit, then installs the pinned Claude CLI. Prints sha, version stamp and commit subject **before** building. |
| `verify-image.sh [tag] [version]` | Five checks that the rig is sound. Run it before trusting any image. |
| `verify-pair.sh [old] [new]` | Proves the two images are **different trees**, behaviourally. |
| `run-session.sh <corpus> <out> [tag]` | Drives one interactive session: corpus in, session, everything back out, container destroyed. |

## What each check buys, and why it is there

Every one of these is a failure this rehearsal actually hit. None is hypothetical.

1. **The binary reports the version its tree carries.** Cheap, and it catches a build that
   silently used the wrong source.
2. **`doc-code` executes.** A probe that cannot load reads as *a whole validation family
   finding nothing* — which this trial would otherwise write down as a result about jigc.
   The first image built clean and died at exec: built on trixie, run on bookworm, needing
   `GLIBC_2.39` against 2.36. The Dockerfile now fences this **at build time** with `ldd`,
   so an image that cannot run its own binaries does not exist.
3. **No instruction files.** Deliberately *not* "the home is empty" — the image seeds a
   minimal `~/.claude.json` so interactive mode does not open on the auth screen. The image
   may contain state that makes a session **start**; never state that makes it **behave
   differently**.
4. **The discriminating probe flips.** Stated as a *difference*: YES on the host, NO in the
   container. The first probe written for this asked about the `!!` token and returned
   UNKNOWN on **both** sides — it would have certified a machine with no isolation at all.
   A uniform null in both directions is the signature of a broken instrument, not a result.
   Check the positive side first.
5. **A corpus round-trips with its git history intact**, and `jigc setup` lands inside.

### The pair differential, and the trap it closes

**Two commits stamp `1.0.0-rc.10`**: `8979f16` (the genuine pre-M48 binary) and `4fd7fbc`
(rc.10-stamped but containing all of M48, because the bump landed late at `9cb9b78`).
`jigc --version` **cannot tell them apart**, so an upgrade arm built from the wrong one
compares rc.11 against itself with nothing in the output to reveal it.

`verify-pair.sh` probes three verbs M48 shipped — `doc rename`, `config get`,
`describe --workflows` — each of which a genuine pre-M48 tree must lack. Confirmed 3/3 on
2026-08-15. Because `build-image.sh` takes a **sha**, there is no version string to search
for and get wrong: the trap is unreachable rather than documented.

## Running a session

```sh
./build-image.sh HEAD      jigc-gate:rc11    # the binary under test
./build-image.sh 8979f16   jigc-gate:rc10    # the upgrade arm's baseline
./verify-image.sh jigc-gate:rc11 1.0.0-rc.11
./verify-pair.sh

./run-session.sh ~/corpora/<name> ~/corpora/<name>-out jigc-gate:rc11
```

`run-session.sh` refuses a non-git corpus and refuses an out-dir that already exists, never
touches the original corpus (so a session can be re-run from a clean start), and copies
`/work` out in a trap — so a crash mid-session still yields its evidence. On exit it prints
the invocation-log record count and the `doc show` call count, which is protocol.md §3.3's
primary channel.

Auth is `CLAUDE_CODE_OAUTH_TOKEN_FOR_TESTING`, passed via `--env-file` rather than `-e`.
**Corrected 2026-08-16, verified with a canary:** that keeps the token out of the *process
list* only. `--env-file` is parsed client-side and the value lands in the container config
verbatim, so `docker inspect` still shows it — for the whole life of a session container.
Declared bound, not a guarantee: anything that can reach the docker socket can read it.

## Declared bounds

- **The operator accepts a `bypassPermissions` confirmation** at session start. It happens
  **before** the worker prompt and says nothing about jigc, so it is not contamination —
  but it is an operator touch and belongs in the operator log. The flag is used so a
  permission prompt on every edit does not become an operator touch on every edit; the
  container is throwaway, so there is nothing here to protect.
- **The network is not isolated.** The container must reach the API. A scenario that
  depends on a third-party host must be fixed at the *scenario* level, not by blocking the
  network.
- **`verify-image.sh` check 2 proves `doc-code` loads, not that it parses.** A real anchor
  in a real corpus is the first live session's job.
- **Isolation costs direct comparability with RC-pre-1.0**, whose sessions ran unisolated.
  protocol.md §2 chose its shape so the reachability result would be *directly* comparable;
  with isolation that comparison becomes indicative. The trade was taken deliberately: a
  clean absolute measurement is worth more than a comparison already weakened by six fresh
  corpora, a different binary, and a designed-need correction no prior trial carried.
  **This is a change to a pre-registered instrument and must be recorded as such in
  protocol.md before the trial runs**, not discovered in the record afterwards.
