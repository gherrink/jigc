# The 1.0.0-gate trial's operator plants — built and rehearsed to firing

Built 2026-08-16, **before any session ran**. The trial is
[protocol.md](../protocol.md); the corpora are [corpora.md](../corpora.md); this file is
the **plants** and, more importantly, the **evidence that each one fires**.

*A plant assumed to fire is not a plant.* Every plant below was run on a throwaway copy
of the real corpus template and shown to produce the behaviour the protocol claims for
it, with the verbatim output recorded here. Exit codes were measured **unpiped** — a
`cmd | tail` reports *tail's* status, not the command's.

**Rehearsed against:**

| | |
|---|---|
| host binary | `jigc 1.0.0-rc.11` (`~/.local/bin/jigc`) |
| image | `jigc-gate:rc11`, `JIGC_SHA=9a37f01`, stamp `jigc 1.0.0-rc.11` |
| corpora | `trial-corpus-template/instantiate.sh --clean-prose` (throwaway copies in a scratchpad; all destroyed) |

The B1 arc was rehearsed **twice** — once on the host for iteration, once **inside
`jigc-gate:rc11`** through `docker create` + `docker cp` + the image's own entrypoint,
because that is where the trial actually runs and `core.hooksPath` survives a `docker cp`
only if `.git/config` travels with the corpus. It does; both runs are recorded below.

---

## What is here

| file | what it is |
|---|---|
| `b1-hook.sh` | **B1 plant 1** — installs a rejecting `pre-commit` hook under `core.hooksPath` |
| `b1-release-hook.sh` | the operator's release (and `--revoke`) for that hook |
| `b1-staged.sh` | **B1 plant 2** — stages two files before the first mint, for the carryover gate |
| `b3-foreign-adr.sh` | **B3 plant** — lands the foreign ADR mid-stream, back-dated, explicit pathspec |
| `b3-foreign-adr.md` | the RC-pre-1.0 ADR body, **byte-identical** (`md5 c5db3fdd…`) |
| `b3-foreign-adr-clean-prose.md` | the same trap with the wart prose removed — see *Finding 2* |

Every script takes the corpus directory as its argument and **refuses a corpus that is
not in the state the plant assumes**. Those refusals are demonstrated to fire, not
asserted (see *The refusal bars*). Every refusal is overridable with `--force`, so a
bar can never become the reason a trial cannot run.

## Order of operations

```sh
T=completions/trial-corpus-template
P=completions/artifacts/RC-1.0-gate/plants

# 1. gate the corpus FIRST — the two B1 plants deliberately break FIVE of
#    check-corpus.sh's naive bars, verified by running it on a planted corpus:
#      expected 7 commits, found 8   ·   working tree dirty
#      corpus is not naive — found: .githooks/pre-commit
#      core.hooksPath is set to '.githooks'
#      reflog has 8 entries, expected 7
#    Gate first, plant second. Never the other way round.
$T/check-corpus.sh ~/ideas/harborlight --clean-prose      # must be 11/11

# 2. plant B1, in this order
$P/b1-hook.sh   ~/ideas/harborlight
$P/b1-staged.sh ~/ideas/harborlight

# 3. run the session; note the container id it prints
completions/trial-harness/run-session.sh ~/ideas/harborlight ~/out/b1 jigc-gate:rc11

# 4. release the hook when the worker reports a rejected finalize — IN THE CONTAINER
docker exec -it -u node <cid> bash -lc 'touch /work/.git/docs-approved'
```

B3's plant is **mid-stream**, so it is landed into the live container:

```sh
docker cp $P <cid>:/tmp/plants
docker exec -it -u node <cid> bash -lc '/tmp/plants/b3-foreign-adr.sh /work'
```

---

## Plant B1-1 — a rejecting `pre-commit` hook under `core.hooksPath`

**What it is.** `.githooks/pre-commit`, committed, with `core.hooksPath = .githooks` in
the corpus's local config. It refuses any commit whose staged set touches `docs/` — which
is where a promoted ADR lands — unless the sign-off marker exists.

**Why `core.hooksPath` and not `.git/hooks`.** `resolve_hooks_dir`
(`crates/cli/src/setup.rs`) resolves through `git rev-parse --git-path hooks`, so `jigc
setup` splices its own warn-only block into **this** file and `SetupSummary.hook_file`
prints what it resolved. A plant in `.git/hooks` would be bypassed outright.

**The release marker lives in the git dir** — `$(git rev-parse --git-dir)/docs-approved`,
not `.githooks/docs-approved` as RC-pre-1.0 used. Three reasons, each a way the worktree
version could have polluted the measurement: `git status` never shows it, so it cannot be
mistaken for the worker's own work; no `finalize` can sweep it into a commit; and it
cannot trip jigc's foreign-untracked-file gate at a task door.

**Dating.** The plant commit defaults to **HEAD's own author date**, not to an invented
earlier day. The template's seven commits carry the instantiation time, so a plant
back-dated to a "realistic" past date would land with an author date **preceding its
parents** — visible in `git log --date=short`. Flush with HEAD, there is no seam at all.
`--date` overrides.

### Rehearsal — the full arc, on the host

```
$ b1-hook.sh <copy>
planted the docs-gate hook in <copy>
  hooks dir        : .githooks
  release marker   : .git/docs-approved  (absent = rejecting)
  commit           : 5915c85 2026-08-16 12:57:46 +0200 chore: refuse doc commits until the docs review signs off
  working tree     : 0 entries (expect 0)
```

`jigc setup` then resolved and wrapped it, naming the real file:

```
  - pre-commit hook → .githooks/pre-commit   (warn-only doc↔code drift backstop)
  - install commit → b1b0e12
```

**Finalize 1 — the carryover gate (plant B1-2), `exit 3`, HEAD unmoved:** see below.
**Finalize 2 — the hook rejects the ADR-promoting finalize:**

```
$ jigc task finalize record-that-the-ingest-queue      # FINALIZE2_EXIT=1
finalize — about to commit the index; leaving out:
  left-out (unstaged/untracked — git add to include):
    src/router.ts
    scripts/retention-sweep.sh
--- stderr ---
`git commit` was rejected (no commit was made):
docs-gate: refusing this commit — it touches docs/ and the docs review has not signed off.
docs-gate: nothing was committed. Ask the docs reviewer to sign off, then commit again.

task record-that-the-ingest-queue is intact — nothing was committed and your staged changes are still staged. Fix the hook's complaint, then re-run `jigc task finalize record-that-the-ingest-queue`.
--- HEAD moved? ---
HEAD UNMOVED b1b0e12e6dee90fa57e3d8d612d8eec588ade166
```

That is M47's survivable frame in one output: **the hook's stderr verbatim**, the
**state-truth clause** (*"is intact — nothing was committed and your staged changes are
still staged"*), the **copy-runnable re-run argv**, and HEAD provably unmoved.

**Finalize 3 — released, then it lands:**

```
$ b1-release-hook.sh <copy>
docs-gate RELEASED — .git/docs-approved created; commits touching docs/ are allowed
$ jigc task finalize record-that-the-ingest-queue      # FINALIZE3_EXIT=0
finalized 966e617 — docs: record the overflow drop policy as a decision
  added .jigc/config/manifest.yaml
  promoted docs/decisions/drop-the-oldest-sample-when.md
  2 files committed
```

**The invocation log carries the rejection as its own error identity** — M42's claim,
live, and the only observation surface the operator has for it:

```
{"argv":["task","finalize","record-that-the-ingest-queue"],"exit_code":3,…,"finding_codes":["finalize.carried-staged","finalize.carried-staged"],…}
{"argv":["task","finalize","record-that-the-ingest-queue"],"exit_code":1,…,"finding_codes":[],…,"error_code":"finalize.commit-rejected"}
{"argv":["task","finalize","record-that-the-ingest-queue"],"exit_code":0,…,"finding_codes":["file-state.staged-copy"],…,"error_code":null}
```

### Rehearsal — the same arc inside `jigc-gate:rc11`

Driven non-interactively through the image's entrypoint (`docker create … bash
/tmp/arc.sh`; `docker exec` alone runs as **root** and every git call dies on *dubious
ownership in repository at '/work'* — worth knowing before an operator does it by hand
mid-session, and the reason `docker exec -it -u node <cid> bash -l` in run-session.sh's own hint
works only because it drops to `node`).

```
== git identity: Worker <worker@example.com>
== hooksPath: .githooks  resolved: .githooks
== staged at start:
scripts/retention-sweep.sh
src/router.ts
== jigc version: jigc 1.0.0-rc.11
SETUP_EXIT=0
== install commit carries the foreign hook?
53	0	.githooks/pre-commit
== staged plant survived setup:
scripts/retention-sweep.sh
src/router.ts
===== FINALIZE 1 (expect carryover gate) =====
FINALIZE1_EXIT=3
… HEAD UNMOVED
===== FINALIZE 2 (expect hook rejection) =====
FINALIZE2_EXIT=1
`git commit` was rejected (no commit was made):
docs-gate: refusing this commit — it touches docs/ and the docs review has not signed off.
…
HEAD UNMOVED
===== RELEASE + FINALIZE 3 (expect land) =====
FINALIZE3_EXIT=0
1b1feea docs: record the overflow drop policy as a decision
```

Identical exit codes and identical text to the host run. Two things the container run
proves that the host run cannot: **`core.hooksPath` survives `docker cp`** (`.git/config`
travels with the corpus), and **the staged plant survives `jigc setup`** — setup's install
commit uses an explicit pathspec, so the two pre-mint paths are still staged afterwards
and the carryover gate still has something to catch.

---

## Plant B1-2 — two files staged before the first mint

**Two shapes, not two of one:** `scripts/retention-sweep.sh` is index status **A** (new),
`src/router.ts` is index status **M** (a modification — a `/healthz` route inserted into
`dispatch`). The gate emits one finding **per path** and the two routes restore different
things, so a per-path claim proven on one shape is not proven. The router edit keeps the
suite green (`node --test`: 23 passed / 0 failed, re-verified after planting).

Nothing is committed. **The index is the plant**, and `docker cp` of the corpus carries
`.git/index` with it.

### Rehearsal — the gate fires per path, with its route, exit 3, HEAD unmoved

```
$ jigc task finalize record-that-the-ingest-queue      # FINALIZE1_EXIT=3
blocking · finalize.carried-staged — `scripts/retention-sweep.sh` was already staged before this task existed — refusing to let a pre-task staged change silently ride this task's commit
  route: unstage it (`git restore --staged -- scripts/retention-sweep.sh`) if it is not this task's work, or re-run the finalize with `--carry-staged` to declare the carry-over deliberate
blocking · finalize.carried-staged — `src/router.ts` was already staged before this task existed — refusing to let a pre-task staged change silently ride this task's commit
  route: unstage it (`git restore --staged -- src/router.ts`) if it is not this task's work, or re-run the finalize with `--carry-staged` to declare the carry-over deliberate
--- HEAD moved? ---
HEAD UNMOVED b1b0e12e6dee90fa57e3d8d612d8eec588ade166
```

**Both routes were run, not just read** — §1's blocking-dead-end row is about routes that
cannot run, so a plant whose route was never executed proves less than it looks:

- `git restore --staged -- …` → the next finalize reaches the hook (above);
- `--carry-staged` → `exit 0`, and the carry is **labelled at both render sites**:

```
$ jigc task finalize <id> --carry-staged               # CARRY_EXIT=0
finalize — about to commit the index; carrying over (staged before this task existed — declared with `--carry-staged`):
  carried-over scripts/retention-sweep.sh
  carried-over src/router.ts
…
finalized 03994ab — docs: record the overflow drop policy
  promoted docs/decisions/drop-the-oldest-sample-when.md
  carried-over scripts/retention-sweep.sh
  carried-over src/router.ts
  3 files committed
```

---

## Plant B3 — the foreign ADR, landed mid-stream

`docs/decisions/0002-keep-the-sample-store-in-memory.md`, hand-committed, never through
jigc. Two signals, per protocol §4: **detect-and-route** (the managed-vs-foreign
discriminator) and **the contradiction trap** (an accepted decision that the worker's
persistence task contradicts, naming `supersedes` as its own escape hatch).

**Mid-stream is checkable, not trusted.** The script counts the commits after the one that
**added `CLAUDE.md`** (jigc's install commit). A freshly adopted corpus has exactly one —
the adopt commit that turns the invocation log on — so the worker's first finalize has
landed only at two or more. Below that it refuses and prints what it counted.

**One date, two places.** The body's `Date:` and both `GIT_AUTHOR_DATE`/
`GIT_COMMITTER_DATE` come from a **single** `--date` value, so they cannot disagree.
RC-pre-1.0's worker caught the plant exactly on that seam (*"committed 07:41 today,
body-dated 2026-08-04"*).

**Explicit pathspec, both halves.** `git add -- <path>` then `git commit … -- <path>`: at
the instant the operator plants, the worker's task is live and its index may hold staged
work. Verified rather than reasoned about — see the rehearsal.

### Rehearsal — the mid-stream bar refuses before the first finalize

```
$ b3-foreign-adr.sh <adopted-copy>
commits since the jigc install commit:
76d072f chore(jigc): turn the invocation log on
refusing: only 1 commit(s) since install — the worker's first finalize has not landed.
         Land this AFTER it, or pass --force if you have checked by hand.
EXIT=1
```

### Rehearsal — after a finalize, with worker work staged at that instant

```
=== index before plant ===
src/store.ts

planted the foreign ADR in <adopted-copy>
  body     : b3-foreign-adr-clean-prose.md  (corpus is --clean-prose -> clean-prose body …)
  path     : docs/decisions/0002-keep-the-sample-store-in-memory.md
  date     : 2026-08-04  (body Date: and both commit dates)
  commit   : 2f99f28 2026-08-04 09:12:41 +0200 docs: record the in-memory store decision
  carried  : docs/decisions/0002-keep-the-sample-store-in-memory.md
  still staged elsewhere: src/store.ts
EXIT=0

=== index after plant (must still hold src/store.ts) ===
src/store.ts
=== body Date vs commit date ===
Date: 2026-08-04
author=2026-08-04 09:12:41 +0200 committer=2026-08-04 09:12:41 +0200
```

The operator's commit carried **one** path; the worker's staged `src/store.ts` was
untouched; body date and both commit dates agree.

### Rehearsal — signal 1, the discriminator

`jigc validate` (`exit 0`, store scope is report-only) routes it exactly as the protocol
pre-registers, and says so in words:

```
advisory · schema-conformance.unadopted-instance — committed file `docs/decisions/0002-keep-the-sample-store-in-memory.md` sits at the `adr` home but was never adopted by jigc — it carries no schema-version stamp and parses against no known `adr` schema version
  route: adopt — run `jigc ingest` to route it, or `jigc migrate docs/decisions/0002-keep-the-sample-store-in-memory.md --as adr` to rewrite it into the managed `adr` shape; it is a foreign file, not an unmigrated managed doc
```

`jigc doc list` discriminates it from the worker's own ADR:

```
adr:0002-keep-the-sample-store-in-memory  …  unregistered
adr:cap-the-ingest-queue                  …  managed
```

`jigc ingest` (`exit 0`) classifies and routes it:

```
needs-reconcile docs/decisions/0002-keep-the-sample-store-in-memory.md → adr
  blocking · conformance.section-renamed — section heading "Status" does not match required section `context`
  route: `jigc migrate docs/decisions/0002-keep-the-sample-store-in-memory.md --as adr` — it opens the `migrate-adr` workflow, which rewrites the file to conformant shape and adopts it at finalize
```

**But `jigc migrate-corpus` does not agree with any of them — see Finding 1.**

---

## The refusal bars, demonstrated

Each script's state bars were tripped on purpose. None is assumed:

| # | what was done | result |
|---|---|---|
| 1 | `b1-hook.sh /tmp` | `refusing: /tmp is not a git repository` · exit 1 |
| 2 | `b1-hook.sh` twice | `refusing: core.hooksPath is already set to '.githooks' — this corpus has been planted before` · exit 1 |
| 3 | `b1-hook.sh` on a dirty tree | `refusing: working tree is dirty: M README.md` · exit 1 |
| 4 | `b1-staged.sh` twice | `refusing: the index is already dirty: …` · exit 1 |
| 5 | `b1-staged.sh` on an adopted corpus | `refusing: .jigc/ is present — the staged set must predate the first mint, and it does not` · exit 1 |
| 6 | `b3-foreign-adr.sh` on a naive corpus | `refusing: .jigc/ is absent — B3 lands on an ADOPTED corpus, mid-stream` · exit 1 |
| 7 | `b3-foreign-adr.sh` twice | `refusing: docs/decisions/0002-… already exists — planted before?` · exit 1 |
| 8 | `b3-foreign-adr.sh --date 04-08-2026` | `refusing: --date must be YYYY-MM-DD` · exit 2 |
| 9 | `b3-foreign-adr.sh` before the first finalize | the mid-stream bar, above · exit 1 |
| 10 | `b1-hook.sh` onto an existing `.githooks/` | `refusing: .githooks/ already exists — this corpus has been planted before` · exit 1 |
| 11 | `--force` past bars 2 and 10 | exit 0, plant lands — a bar can never be the reason a trial cannot run |
| 12 | `b3-foreign-adr.sh` onto a corpus carrying the B1 docs-gate hook | the commit is **rejected by the hook**; the script unstages, deletes and refuses — *"NOTHING was planted and the file has been removed"* · exit 1, tree left clean |

Bar 12 was **found by the rehearsal, not designed**. The first version let `set -e` abort
after the rejected `git commit`, leaving the foreign ADR **staged and uncommitted** in the
repo — which, landed into a live worker's index mid-session, would have ridden the
worker's next `finalize` into their commit. It cannot arise in this trial's shape (B1 and
B3 are separate corpora), and it is now fail-safe rather than merely improbable.

Body auto-selection was demonstrated **both ways**: a wart corpus selects
`b3-foreign-adr.md` verbatim (*"corpus carries the wart prose"*), a `--clean-prose` corpus
selects `b3-foreign-adr-clean-prose.md`.

---

## What did NOT behave as the protocol expects

The most valuable output of a rehearsal. Nothing here was fixed; it is recorded so the
trial adjudicates it rather than discovers it.

### Finding 1 — `migrate-corpus` claims the foreign ADR, and its route cannot be followed

**Protocol §4 pre-registers the opposite:** *"the managed-vs-foreign discriminator (M42)
should route it to `jigc ingest`/`jigc migrate`, not to `migrate-corpus`."* `validate`,
`doc list` and `ingest` all obey that. **`migrate-corpus` does not** — it takes the
never-adopted foreign file as an in-scope migration subject (an unstamped doc reads as
schema-version 0), blocks on it, and routes back to itself:

```
$ jigc migrate-corpus                                  # MIGRATE_CORPUS_EXIT=1
corpus migration: 0 migrated, 1 already current, 1 blocked
  current    docs/decisions/cap-the-ingest-queue.md
  blocked    docs/decisions/0002-keep-the-sample-store-in-memory.md
    migrate-corpus.prose-needed: `…0002-keep-the-sample-store-in-memory.md`'s migration mints a new **required** prose slot, which no transform can fill …
    route: author the new required prose in `docs/decisions/0002-keep-the-sample-store-in-memory.md` through the write verbs, then re-run `jigc migrate-corpus` (the schema-version stamp flips only once it gates clean)
```

**That route was followed, and it does not run.** The file has no managed identity, so
neither write surface will touch it:

```
$ jigc doc show adr:0002-keep-the-sample-store-in-memory                 # exit 1
blocking · store.unparseable — … does not parse: section heading "Status" does not match required section `context`
  route: adopt — run `jigc ingest` …; it is a foreign file, not an unmigrated managed doc

$ jigc doc set-slot adr:0002-keep-the-sample-store-in-memory#context …   # exit 1
blocking · write.non-reparseable — write rejected: the source does not conform to the schema …
```

**Two of the tool's own surfaces name different repairs for the same file, and one of them
says in words that the other's premise is wrong** (*"it is a foreign file, not an
unmigrated managed doc"*).

**Not adjudicated here, deliberately.** It sits between §1's *blocking dead end* row (a
refusal whose route cannot run → BLOCKS) and its *surface/wording* row (SHIPS RECORDED):
the printed route genuinely cannot be run, but the refusals a worker hits **while trying**
each carry a runnable route back to `ingest`/`migrate`, so the worker is not stranded.
Which row it lands in is the trial's call under §1's rule that class is decided from
evidence before consequence is looked up. **It is reachable by a blind B3 worker**: a
worker that sees an unadopted doc and reaches for the corpus-wide verb lands on it
directly.

Repro: adopt a template corpus, land this plant, run `jigc migrate-corpus`.

### Finding 2 — the reused ADR body reintroduces the wart §2.1 removed

The RC-pre-1.0 body says the service is a *"rollup **cache**"*, that *"the caller already
has a durable store"*, and that *"we say so in the README"*. On a `--clean-prose` corpus —
which is **all five** 1.0-gate corpora — none of that is true any more. Protocol §2.1
removed exactly those claims so an **accidental** prose/code contradiction could not be
mistaken for a designed trap and burn operator interventions; pasting the old body back in
reintroduces it, in the one document a worker is most likely to read closely.

**Resolved rather than flagged:** `b3-foreign-adr-clean-prose.md` carries the same trap
(in-memory *by decision*, `supersedes` named as the escape hatch) with the cache/durable-store/README
claims gone, and the script **selects the body from the corpus** rather than assuming one.
The verbatim RC-pre-1.0 body ships unchanged for a wart corpus and is byte-identical
(`md5 c5db3fdd…`) to `RC-pre-1.0/plants/g3-foreign-adr.md`.

### Finding 3 — B3's second signal depends on a prompt that does not exist yet

The contradiction trap only fires if B3's prompt asks for **persistence across a restart**,
as RC-pre-1.0's G3 did. The 1.0-gate protocol describes B3 as *"changelog · spec →
implement-from-spec · arch-doc"* and does not name the spec's subject; the three verbatim
blind prompts are listed in [corpora.md](../corpora.md) → *Still owed*. **Whoever writes
B3's prompt must keep a durability/persistence task or signal 2 is inert** — and the plant
will still fire signal 1, which makes the loss silent.

### Finding 4 — a back-dated plant now sits *before* its parents

RC-pre-1.0's dent was a **body/commit mismatch**, and §8 rule 1's fix (back-date the
commit) removes it. But the template's seven commits carry the **instantiation** time, so a
plant dated 2026-08-04 lands with an author date twelve days older than its parent:

```
2026-08-04 2f99f28 docs: record the in-memory store decision
2026-08-16 c7a6950 docs: record the bounded queue size choice
2026-08-16 76d072f chore(jigc): turn the invocation log on
```

This is an ordinary git shape (a cherry-picked or rebased commit looks exactly like this)
and it is *far* weaker than the seam that was caught last time, so the default is kept —
but it is a seam, it is recorded rather than discovered, and it is one flag away:
`b3-foreign-adr.sh <repo> --date "$(date +%F)"` moves the body and both commit dates
together and removes the inversion entirely (verified). **`b1-hook.sh` has no such
trade** — it defaults to HEAD's own author date and leaves no seam.

### Finding 5 — RC-pre-1.0's watch item is DRAINED (a behaviour change, in the good direction)

RC-pre-1.0 recorded, unadjudicated: *"`setup` splices its block into the tracked foreign
hook and leaves that file **modified and uncommitted** — its install commit does not carry
it."* On rc.11 it does. The install commit carries the wrapped hook as a modification:

```
$ git show --stat --format='%h %s' HEAD
b1b0e12 chore(jigc): install jigc workspace config
 …
 .githooks/pre-commit         |  53 +++++++++
 …
$ git show --numstat --format= HEAD -- .githooks/pre-commit
53	0	.githooks/pre-commit
```

That is M48 Increment 5 (*the pathspec is derived from where the hook actually landed —
committable iff under the repo root and outside git's own dir*) working on the exact shape
that produced the watch item. Recorded here because it is the state the B1 corpus is in
when the session starts, and because a watch item that quietly resolves is worth writing
down.

### Finding 6 — `docker exec` into a live session container is root, and git refuses

`run-session.sh` prints `(a mid-stream plant runs with: docker exec -it -u node <cid> bash -l)`.
That works because a login shell drops to `node` — but a **non-interactive**
`docker exec <cid> bash /tmp/plant.sh` runs as **root**, and every git call in `/work`
dies:

```
fatal: detected dubious ownership in repository at '/work'
```

Not a jigc finding; an apparatus note that would cost an operator ten confusing minutes
mid-session. Use `docker exec -it -u node <cid> bash -l` (or `-u node`) for the B3 plant. Both
forms were exercised; only the login/`-u node` form works.

---

## Bounds — what was NOT verified

Stated rather than implied:

- **The hook plant assumes `docs/` is the docs-root.** It is, on every template corpus
  (`promoted docs/decisions/…` in the rehearsal), but a corpus with `docs-root` re-pointed
  would silently disarm the hook. The scripts do not check it, because they run before
  `jigc setup` and there is no cascade to read yet.
- **B1's arc was rehearsed with `record-decision`.** The blind worker may reach the ADR by
  another workflow (`single-task`, `decided-task`). The hook keys on the **staged path**,
  not on the workflow, so any doc-promoting finalize trips it — but only the
  `record-decision` path was executed.
- **Nothing here tests what a worker does when it *reads* the hook.** A worker that
  inspects `.githooks/pre-commit`, then unsets `core.hooksPath` or deletes the file to get
  past it, is a legitimate and interesting trial observation — and the plant neither
  prevents nor detects it. Watch the transcript.
- **The B3 rehearsal did not run `jigc migrate … --as adr` to completion.** Signal 1 was
  verified as far as the *route named*; the transform itself is RC-pre-1.0 walk arm 1's
  subject, not a plant property.
- **No plant was rehearsed against a real blind agent.** They were driven by an operator
  script. The plants fire; whether a worker *notices* is what the trial measures.
- **The B3 plant was rehearsed on the host, not in the container.** B1's was rehearsed
  both ways and was byte-identical; B3's commit path uses only git and `sed`, and its one
  container-specific hazard is Finding 6, which was exercised.
