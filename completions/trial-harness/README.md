# The 1.0.0-gate trial's isolation harness

Built 2026-08-15, **before the trial runs**, and rehearsed end to end before anything was
staged. The trial it serves is [protocol.md](../artifacts/RC-1.0-gate/protocol.md); this file is the
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
([M17/pilot/runbook.md](../artifacts/M17/pilot/runbook.md)) — and it was recorded as
insufficient. Only filesystem isolation works.

## The lineage, and what came back

This repo is the origin: `implementation/dogfood/` (hooks + tally),
[differentiator-pilot-study1/harness/](../artifacts/differentiator-pilot-study1/harness/) (the
container rig that produced the 0/32 figure), [workflow-eval/](../workflow-eval/)
(the generalized version), then the long-horizon and cross-doc-refint harnesses.

The mechanism was borrowed by a sibling project and improved in four places, all of which
are folded back in here: the CLI is **installed into the image** rather than mounted (a
Mach-O arm64 binary cannot execute in a Linux container — the pilot mounted it because
that host was Linux); the project is **copied in and out** rather than mounted (colima
serves host mounts read-only, silently producing sessions that appear to change nothing);
the CLI version, and the binary under test, are **pinned**; and permissions are explicit
rather than inherited from the operator's `settings.json`.

## The five scripts

| Script | What it does |
|---|---|
| `build-image.sh <sha> [tag]` · `build-image.sh --registry <version> [tag]` | **Source mode** builds jigc for Linux from a `git archive` of that exact commit; **registry mode** runs `cargo install jigc --version <version> --locked` from crates.io instead, with no tree of ours in the build context, and refuses anything but one exact version. Either then installs the pinned Claude CLI, prints what it is building (sha, version stamp and commit subject, or the registry version) **before** building, and **asserts** `jigc --version` equals the stamp afterwards, exiting 1 on a mismatch. **Layout-aware:** a tree carrying `crates/cli/probes/doc-code/` builds `separate-probe` (the probe beside `jigc`), any other tree and every registry install builds `single-binary`; the image records which at `/usr/local/share/jigc-image/layout`, and where the binary came from at `…/source` (`sha <sha>` or `registry jigc <version>`), and the build-time `ldd` gate runs over every executable staged. **Source mode builds on a kept base layer** (below): it reads the commits from `JIGC_REPO`, else from the repository it sits in, and prints `base kept:` or `base not kept:` with the base's tag, and the wall seconds of each build, then asserts the image was built on that base. **Either mode builds under a temporary tag** and moves `[tag]` to the image only once every check after the build has passed; a failed check exits 1, leaves `[tag]` where it was, and names the temporary tag the image was left under. |
| `verify-image.sh [tag] [version]` | **Seven** assertions that the rig is sound, in six numbered sections (§6 emits two). Run it before trusting any image. Check 2 runs the arm for the layout the image **records**, and fails an image that records none. The list below enumerates five; the script is the authority. |
| `verify-pair.sh [old] [new]` | Proves the two images are **different trees**, behaviourally. The probe set is **selected per trial** with `PAIR_PROBES` (`m48` rc.10→rc.11 · `m46` rc.11→rc.12 · `m49` rc.12→rc.13 · `m50` rc.13→rc.14, the default), each probe stating what it expects on **both** sides — because the original three-verb set was vacuous on any pair where both sides carried those verbs, which an rc.11/rc.12 pair did. The default tracks the current trial on purpose: a default that is wrong for the trial in front of you is a trap wearing a convenience. |
| `check-base-layer.sh <case> [tags]` | The harness's own checks of the kept base layer that need docker, one case per call, **run by hand**: `a_second_candidate_with_the_same_lockfile_builds_on_the_kept_layer <first> <second>` and `the_binary_in_the_image_is_the_commits <first> <second>` read two built images, and `a_toolchain_the_kept_base_lacks_fails_loudly <image>` runs the base one records. The half that needs no docker — what moves the key and what does not, and the repository the commits are read from — is a suite of the gate, [`tooling-tests/trial_image_base_key.rs`](../../tooling-tests/trial_image_base_key.rs). |
| `run-session.sh [opts] <corpus> <out> [tag]` | Drives one session: corpus in, session, everything back out, container destroyed. Options precede the positionals: `--shell` (a plain `bash -l`) · `--exec <script>` · `--headless` with `--prompt-file <f>` · `--home <dir>` and `--arg <v>` (repeatable — how `seed`/`fork` pass `--session-id`/`--resume`/`--fork-session`) · `--cid-file <path>` (written **before** `docker start`, so a plant poller can attach) · `--strict-permissions`. There is **no `--bypass-permissions`**: `bypassPermissions` is the default and an unknown `-*` exits 2. |

### Two ways to build an image (M54 — settled 2026-09-28, built in Increment 7)

M54 changed what a commit's tree looks like — the `doc-code` probe moved inside `jigc` (one binary, spawned by self-exec), and the version moved from the root `Cargo.toml`'s `[workspace.package]` to `crates/cli/Cargo.toml`'s `[package]` — while this harness keeps building images of commits from **both** sides of that change. So `build-image.sh` gained a mode, and its source mode learned the layout:

- **Registry mode** installs a **published** version from crates.io (`cargo install jigc --version <v> --locked`) instead of building a sha. This is the **blind trial's instrument**: what an adopter can install is exactly what the session runs, with no tree of ours in between ([implementation/release.md](../../implementation/release.md) → Verifying a publish).
- **Source mode stays sha-pinned and becomes layout-aware.** A tree that still carries `crates/cli/probes/doc-code/` is built with today's recipe (two executables, the probe beside `jigc`); a tree without it is built as the single binary. The version stamp is read from `crates/cli/Cargo.toml` and falls back to the root manifest for older shas.
- **`verify-image.sh` checks whichever layout it built** — check 2 below proves the separate `doc-code` loads on an old-layout image, and that `jigc`'s own probe child runs on a new one.

Both modes are built as described (M54 Increment 7). Source mode was driven on the tip and on `ce3d86bf`. Registry mode was driven **before the first publish**, when crates.io held only the `0.0.0` placeholder: `--registry 1.0.0-rc.22` exits non-zero before any image is tagged, surfacing cargo's *could not find `jigc` in registry `crates-io` with version `=1.0.0-rc.22`*, and `--registry 0.0.0` builds, then fails the stamp assertion, because the placeholder prints its reservation notice; that image fails `verify-image.sh` at checks 1, 2 and 5. **The positive registry run was made after the publish** (M54 Increment 11, 2026-10-01): `./build-image.sh --registry 1.0.0-rc.22` installs `jigc 1.0.0-rc.22` and `jigc-engine 0.1.0-rc.1` from crates.io and exits 0. It records the layout `single-binary` and the source `registry jigc 1.0.0-rc.22`, and the binary prints `jigc 1.0.0-rc.22`. `./verify-image.sh jigc-gate:registry-1.0.0-rc.22 1.0.0-rc.22` then reports **7 passed, 0 failed**, with check 2 on the `single-binary` arm. The image is `linux/arm64`, the host's ([publish-proof.md](../artifacts/M54/publish-proof.md) → The trial harness's registry image).

### The kept base layer (K7 — built 2026-11-06)

On the canary's candidate, 1,256 of the image build's 1,285 seconds were the toolchain sync
and the crate downloads, and both are the same for every candidate with the same
`rust-toolchain.toml` and `Cargo.lock` ([reshape plan](../artifacts/M55/stabilization-build/reshape/plan.md)
§5, candidate 7). So source mode builds them once, as the image `jigc-gate-base:<key>` (the
Dockerfile's `toolchain` and `base` stages, `docker build --target base`), and keeps it:

- **The key** (`base_key_of`) is a sha256 over the toolchain file, the lockfile, the
  Dockerfile above its `build` stage — comments and blank lines dropped, parser directives
  (`# syntax=`, `# escape=`, `# check=`) at its head kept — and the platform the layer is
  built for. A candidate with the same key builds on the kept layer; a changed lockfile,
  toolchain file, base recipe or platform builds a new one. BuildKit reads the directives
  with two readers that disagree — `escape` may be indented, `syntax` and `check` may follow
  a first-line shebang (driven with `docker build --check`) — so the key keeps whatever
  either reads; a heredoc above the build stage is refused, since its `#` lines are content.
  The platform is `DOCKER_DEFAULT_PLATFORM`, else the docker server's own, and both builds
  are given it with `--platform`, so a kept base of another platform is never the one
  taken. It is taken in **one spelling**, `<os>/<arch>` in lower case with Go's architecture
  name and no variant (`linux/arm64`, `linux/amd64`): docker builds `linux/aarch64`,
  `LINUX/ARM64` and `linux/arm64/v8` as `linux/arm64` too, and rather than copy part of
  docker's alias table the script refuses them, naming the spelling it takes. The
  manifests are not in the key: under `--locked` a tree's resolve is its lockfile, so one
  lockfile needs one set of crates.
- **The layer holds no tree and compiles nothing.** `cargo fetch --locked` runs over the tree
  bind-mounted, so every candidate compiles its own crates from its own tree, and its jigc
  build is `--frozen` — a crate the base lacks fails the build rather than downloading
  quietly. `--frozen` does not reach the toolchain, so the base ends with
  `RUSTUP_AUTO_INSTALL=0`: a toolchain the layer lacks fails with rustup's own *"is not
  installed"* instead of being synced quietly (rustup 1.28.1 and later read it; the base
  image carries 1.29.0). A separate probe (an old tree) keeps its own lockfile and stays
  `--locked`.
- **The image records its base** at `/usr/local/share/jigc-image/base`: the key, a random
  mark of the one layer it was built on, and the platform BuildKit built that layer for
  (its own `TARGETPLATFORM`), so two images say whether they share a layer.
  `build-image.sh` reads it back after each build (`base_check`, which the gate's suite
  drives): an image that does not record the key it computed, the layer its base tag holds
  and the platform keyed, or that docker reports as another platform, variant included,
  fails the check — `[tag]` is not moved to it.
  Registry mode has no lockfile of ours and builds on the plain `toolchain` stage, as before.
- **Kept bases are kept until removed by hand.** Each new key mints an image of about
  1.6 GB, and `docker builder prune` does not remove it, since it is a tagged image and not
  build cache. List them with `docker image ls jigc-gate-base`, and remove one that no
  candidate will use again with `docker image rm jigc-gate-base:<key>`. A trial image does
  not contain its base — the base is a build stage, and the session image starts from its
  own — so removing one breaks no image already built, and the next candidate with that
  key builds it again.

**Driven 2026-11-06**, on two commits with one lockfile and different manifests — `91834b5e`
(rc.24's release commit) then `f6026ca3`: the first printed `base not kept`, built the base in
16 s (its sync-and-fetch step 8.3 s) and the image in 43 s; the second printed `base kept`,
built on the same layer, its compile step 27.6 s with no download and no sync, the image in
32 s. That day's network was fast, so the saving measured is small — the canary's 1,256 s
were a slow day's, and they are what a kept layer removes whatever the day. Both images pass
`verify-image.sh` 7/7 and all four cases `check-base-layer.sh` then had (its two that need no
docker are now the gate's suite); their binaries differ by
sha256, as their sources do. `verify-pair.sh` over the two passes its liveness and identity
half 5/5; no probe set exists for an rc.24 → candidate pair, so its probe lines do not apply.

**Driven again 2026-11-06, after the review's fixes** (the platform in the key, the
directives kept, `RUSTUP_AUTO_INSTALL=0` in the base, the check after the build). The key
moved for every commit, so the same two commits built a new base: `91834b5e` printed `base
not kept`, built the base in 91 s (its sync-and-fetch step 20.4 s) and the candidate in
150 s, 249 s in all; `f6026ca3` printed `base kept`, its compile step 58.9 s with no sync and
no download, 65 s in all. Both printed `base checked` — the key, the tag's layer and
`linux/arm64` — and pass `verify-image.sh` 7/7 and the three docker cases of
`check-base-layer.sh`; the toolchain case, red on the base built before the fix (it began a
sync), refuses a toolchain the new base lacks with rustup's *"is not installed"*.
`--platform` with the local base tag resolved it from the local store, pulling nothing.
**Registry mode driven, not only linted:** `./build-image.sh --registry 1.0.0-rc.24` built in
54 s, recording `single-binary`, `registry jigc 1.0.0-rc.24` and no base, and
`verify-image.sh` passes it 7/7.

**Driven a third time 2026-11-06, after the second review's fixes** (one platform spelling,
`base_check`, the temporary tag, the base's own `TARGETPLATFORM` in its record). With
`DOCKER_DEFAULT_PLATFORM=linux/aarch64` the script refuses before any build, naming the
spelling it takes, and no tag is made. With the new key's tag forged onto the earlier base,
the build printed `base kept`, then `MISMATCH` — the key and the missing platform — and exited
1; the asked-for tag was never made, and the image was left under its unchecked tag. On the
real key, `91834b5e` printed `base not kept` (base 13 s, compile step 25.1 s, 42 s in all),
its base recording `platform linux/arm64`; `f6026ca3` printed `base kept` (compile step
25.5 s, 29 s in all). Both passed `base_check`, `verify-image.sh` 7/7 and the three docker
cases of `check-base-layer.sh`. Registry mode, through the temporary tag, passes 7/7.

## What each check buys, and why it is there

Every one of these is a failure this rehearsal actually hit. None is hypothetical.

1. **The binary reports the version its tree carries.** Cheap, and it catches a build that
   silently used the wrong source.
2. **The `doc-code` probe runs, in the layout the image records.** A probe that cannot
   load reads as *a whole validation family finding nothing* — which this trial would
   otherwise write down as a result about jigc. The first image built clean and died at
   exec: built on trixie, run on bookworm, needing `GLIBC_2.39` against 2.36. The
   Dockerfile now fences this **at build time** with `ldd`, over every executable it
   staged, so an image that cannot run its own binaries does not exist. On a
   `separate-probe` image the check is that `doc-code` executes; on a `single-binary`
   image it is that `/usr/local/bin/doc-code` is **absent** and that `jigc __probe
   doc-code --build <its version>`, fed an empty-anchor request, answers an empty findings
   response at exit 0 — the self-exec intercept, told apart from clap, which answers
   `jigc __nope` with exit 2. Each arm fails on the other layout's image, and an image
   that records no layout (every image built before M54's harness) fails outright, since
   guessing the arm is how a probe-less image would pass.
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

**Two commits stamp `1.0.0-rc.10`**: `1d4f9bc` (the genuine pre-M48 binary) and `78b288f`
(rc.10-stamped but containing all of M48, because the bump landed late at `1d2f146`).
`jigc --version` **cannot tell them apart**, so an upgrade arm built from the wrong one
compares rc.11 against itself with nothing in the output to reveal it.

`verify-pair.sh`'s `m48` set probes three verbs M48 shipped — `doc rename`, `config get`,
`describe --workflows` — each of which a genuine pre-M48 tree must lack. Confirmed 3/3 on
2026-08-15. Later pairs use behavioural sets (`m46`, `m49`), since a wave that changes only
behaviour has no verb to probe for. Because `build-image.sh` takes a **sha**, there is no version string to search
for and get wrong: the trap is unreachable rather than documented. Registry mode takes a
version, which is safe for the other reason: crates.io never serves two builds under one version.

## Running a session

```sh
./build-image.sh HEAD      jigc-gate:rc11    # the binary under test
./build-image.sh 1d4f9bc   jigc-gate:rc10    # the upgrade arm's baseline
./build-image.sh --registry 1.0.0-rc.22      # a published version -> jigc-gate:registry-1.0.0-rc.22
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
