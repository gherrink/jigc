# Increment 1 — the seed/fork chain, driven live

**The claim:** a conversation can be driven to a chosen point, frozen, and forked
any number of times, so that every arm starts from an identical mid-arc position.

**Proven live on `jigc-gate:rc11`, 2026-08-25** — and proven *behaviourally*, not
just structurally. Evidence in [increment-1-evidence/](increment-1-evidence/).

## What was driven

Two seed turns, deliberately short, against the adopted `millrace` corpus:

```
Read src/store.ts and tell me in one sentence what MemoryStore does. Change nothing.
We have decided the store must evict the oldest series once it hits a cap of 500.
Treat that as settled; do not implement it yet.
```

Frozen as session `11880526-…`, `session_sha 1f257da6e2d0f766`, 2 turns. Then forked
**twice**, each with the same question:

> What did we settle about eviction, and at what number? One sentence. Change nothing.

| fork | minted id | inherited | answer |
|---|---|---|---|
| A | `32c88eb0-…` | ✅ | *"once the store hits 500 distinct series, it evicts the oldest (least-recently-updated)"* |
| B | `57f1b689-…` | ✅ | identical |

**Why the answer is the proof.** The number 500 and the eviction policy appear
**only** in seed turn 2. A fork that began cold could not produce them. The marker
check (`seed_inherited`) says the file was carried; the answer says the
*conversation* was. Both ids differ from the seed's, confirming `--fork-session`
minted new ones rather than extending the fixture.

## Three defects it cost, all found by driving rather than reading

**1 · The staged transcript was unreadable by the container's user.** `docker cp`
preserves the **host** uid, so the transcript landed as `uid 501, -rw-------`, and
the container runs as `node` (1000). The image's entrypoint chowns `/work` and not
`/home/node`. The CLI reported `No conversation found with session ID` and the turn
died — the harness's own documented `/work` hazard, one directory over.

Fixed with **modes, not ownership**, because ownership cannot be set at that point:
the container is not running yet (`create` → `cp` → `start`), so there is no
`docker exec` to chown from, and teaching the entrypoint means rebuilding a pinned
image — which invalidates every isolation record keyed on its digest. The container
is single-use and destroyed at the end of the turn.

**2 · The seed's own failure guard was blind.** `run-session.sh` exits **0** even
when the driven arm exits non-zero — deliberately, because for a scripted arm that
"is data, not necessarily failure". For a seed turn it is neither: a failed turn
leaves the next `--resume` with nothing. The first attempt therefore froze a fixture
built on a **dead turn 2** and reported success. The arm's own exit code is now read
back out of `PROVENANCE.txt`.

**3 · The rig's evidence was being planted in the corpus.** A turn's out-dir becomes
the next turn's corpus, and that dir also holds `.session-transcript/`,
`PROVENANCE.txt`, `stream.jsonl` and `stderr.txt`. Without `carry_forward()` those
are copied into `/work` as corpus content on every turn after the first — apparatus
artefacts in the tree under test, in every arm. Caught before the live run, by
writing out what the next turn would receive.

## A harvested assumption that does not hold, in the safe direction

The mechanism was harvested with a prominent warning attached:

> `--resume` on a session the CLI has never seen does not fail: it starts a fresh
> conversation, which is the one apparatus failure that manufactures a plausible arm
> out of a dead fixture.

**On CLI 2.1.233 in this image, it does fail** — loudly, at exit 1, with
`No conversation found with session ID: …`, `subtype: error_during_execution` and
zero turns.

It was first seen by accident on the failed first attempt, and **that attempt's
stderr was lost** when the work directory was rebuilt by the re-seed. It is recorded
here from a **deliberate repro** rather than from recall — the command is in
[increment-1-evidence/README.md](increment-1-evidence/README.md) and anyone can
re-run it.

**The `seed_inherited` check stays, and stays load-bearing.** This is one CLI build
on one image; the check costs nothing, the failure it guards is silent by
construction wherever the refusal does *not* happen, and a fixture whose verification
depends on a version-specific error message is not verified. What changes is the
expected failure *mode*, not whether to check for it.

## Bounds

- **One CLI build, one image, one corpus, two forks.** This shows the mechanism
  works; it says nothing about how far a seed generalises, and nothing about
  whether two forks of one seed are *independent* in any statistical sense.
- **Restoration is the caller's job and is not yet fenced.** `project/` carries no
  `.git` by design (git objects are mode 0444 and make a tree unrestorable on the
  second restore), so history comes from the template. Here that was done by hand in
  the driving script; a mismatch between the frozen tree and the restored history is
  not currently detected.
- **The `0o777`/`0o666` staging modes are a contained workaround**, justified by the
  container being single-use. If the image is ever rebuilt for another reason, moving
  the chown into the entrypoint is the better fix.
