# Pre-trial findings — found while building the instrument, before any session ran

Findings the **preparation** turned up. They are recorded here, with repro blocks, because a finding
does not stop being a finding for having been found by an operator rehearsing a plant rather than by
a blind worker. Each is classified against [protocol.md](protocol.md) §1 **from its evidence, before
its consequence was looked up** — the rule that binds the adjudicator.

---

## PT-1 · `migrate-corpus` claims a never-adopted foreign file, and its printed route cannot run

**Found:** 2026-08-16, rehearsing B3's mid-stream foreign-ADR plant.
**Reachable by:** a blind B3 worker. The plant lands a foreign ADR in an adopted corpus; running
`jigc migrate-corpus` is a plausible next move for an agent that has just been told a doc is not
current.
**Not a regression** — identical output and exit code on `jigc-gate:rc10` (8979f16, pre-M48).

### What happens

`migrate-corpus` takes a **never-adopted foreign file** as an in-scope migration subject (unstamped
reads as v0), blocks at **exit 1**, and prints a route whose premise the tool's *own other surface*
explicitly denies.

```console
$ jigc migrate-corpus ; echo "exit=$?"
corpus migration: 0 migrated, 0 already current, 1 blocked
  blocked    docs/decisions/0002-keep-the-sample-store-in-memory.md
    migrate-corpus.prose-needed: `…0002-keep-the-sample-store-in-memory.md`'s migration mints a new
    **required** prose slot, which no transform can fill …
    route: author the new required prose in `…0002-keep-the-sample-store-in-memory.md` through the
    write verbs, then re-run `jigc migrate-corpus`
exit=1
```

Following that route dead-ends — the doc id it implies does not exist:

```console
$ jigc doc show adr:keep-the-sample-store-in-memory
blocking · store.not-found — could not read `adr:keep-the-sample-store-in-memory` at
  `…/docs/decisions/keep-the-sample-store-in-memory.md`: No such file or directory (os error 2)

$ jigc doc list | grep 0002
adr:0002-keep-the-sample-store-in-memory  docs/decisions/0002-…md  unregistered
```

While `jigc validate` — the correct surface — routes it properly **and states the other surface's
premise is wrong**:

```console
$ jigc validate
advisory · schema-conformance.unadopted-instance — committed file `…0002-…md` sits at the `adr` home
  but was never adopted by jigc — it carries no schema-version stamp and parses against no known
  `adr` schema version
  route: adopt — run `jigc ingest` to route it, or `jigc migrate …  --as adr` to rewrite it into the
  managed `adr` shape; **it is a foreign file, not an unmigrated managed doc**
```

### Repro

```sh
completions/trial-corpus-template/instantiate.sh --clean-prose /tmp/pt1 svc
cd /tmp/pt1 && jigc setup && mkdir -p docs/decisions
cp completions/artifacts/RC-pre-1.0/plants/g3-foreign-adr.md \
   docs/decisions/0002-keep-the-sample-store-in-memory.md
git add -A && git commit -m "docs: record the in-memory store decision"
jigc migrate-corpus            # exit 1, prose-needed, route cannot be followed
jigc validate                  # unadopted-instance, routes to ingest/migrate
```

### Why this is a finding rather than a papercut

M42 shipped the **managed-vs-foreign discriminator** precisely so *"stale-managed routes to
`migrate-corpus`, never-adopted to `ingest`"*. `validate` applies it. `migrate-corpus` does not — it
claims a file the discriminator says is not its subject, and the disagreement is visible to a worker
in one session.

### Classification — stated with its ambiguity rather than resolved to the cheaper row

Two §1 rows have a genuine claim on this, and the difference is BLOCKS vs SHIPS RECORDED:

- **"A blocking dead end — a refusal whose route cannot run"** → **BLOCKS.** The first clause fires
  literally: the route as printed cannot be followed to success.
- **"A wrong result on a non-destructive path"** → **SHIPS RECORDED.** No bytes move (the message
  says the bytes are rolled back untouched, and they are), it is a false *red* rather than a false
  green, and no pinned `--format json` contract is violated.

What separates them is whether "the route cannot run" means *the printed route*, or *the state has no
recovery*. Here the printed route dead-ends but the state does have a recorded recovery — `validate`
names it correctly, and nothing is stuck or unrecoverable.

### Disposition — provisional, decided by the human 2026-08-16, rechecked at trial close

The human's rule: **it depends on what the file carries.** Root files and anything holding project
knowledge → blocking dead end; incidental, non-project-carrying files → wrong result that ships. Now
recorded as §1's blast-radius qualifier.

**Applied here, the rule resolves to BLOCKING — and the reachable set was demonstrated, not
assumed.** The defect is not specific to the ADR it was found on: it fires on *any* never-adopted
file sitting at a managed doctype's home, and two of those homes are **repo-root files**. Verified on
rc.11 with a foreign root `CHANGELOG.md`:

```console
$ jigc migrate-corpus ; echo "exit=$?"
corpus migration: 0 migrated, 0 already current, 1 blocked
  blocked    CHANGELOG.md
    route: author the new required prose in `CHANGELOG.md` through the write verbs, then re-run …
exit=1

$ jigc validate
advisory · schema-conformance.unadopted-instance — committed file `CHANGELOG.md` sits at the
  `changelog` home but was never adopted by jigc …
  route: … it is a foreign file, not an unmigrated managed doc
```

So the reachable set is `CHANGELOG.md`, `VISION.md`, `docs/decisions/*` — root files and decision
history, the exact category the rule calls blocking.

**Two mitigations, recorded because they bear on the final call and because omitting them would make
this finding look worse than it is:**

1. **The documented path routes correctly.** [MIGRATING.md](../../../MIGRATING.md) tells an adopter to
   *"ask `jigc validate` whether you need to migrate — it blocks if you do"*, and for a foreign file
   `validate` does **not** block: it emits the `unadopted-instance` advisory at exit 0 with the right
   route. PT-1 is reached by **initiative, not by instruction** — someone who runs `migrate-corpus`
   unprompted, which a blind B3 worker plausibly will.
2. **Nothing is destroyed or stranded.** The bytes are rolled back untouched, and a correct recovery
   exists one verb away.

**Recheck at trial close, per the human's instruction**, on two questions the trial can answer that
this rehearsal cannot: whether a blind worker actually reaches it, and whether any further instance
lands on a path where `validate` is *not* the correct second opinion. Until then the provisional
class is **blocking dead end (project-carrying reach)**, and it is carried into the 1.0.0 record as
an open disposition rather than a settled one.

---

## PT-2 · `docker exec` into a live session container lands as root and breaks every git call

**Found:** 2026-08-16, rehearsing the B3 mid-stream plant against the container rig.
**Class:** apparatus defect, mine — not a product finding.

`docker exec` bypasses the image ENTRYPOINT, so the gosu drop to `node` never happens and the shell
is root. Every git call in `/work` then fails with *"detected dubious ownership in repository at
'/work'"*, which would strand the operator mid-session at exactly the moment a plant must land.

**Fixed:** `run-session.sh` now prints `docker exec -it -u node <cid> bash -l`, with the reason in a
comment so it is not "simplified" back later.

---

## PT-3 · The reused foreign-ADR body reintroduces the prose wart the corpora deliberately removed

**Found:** 2026-08-16, rehearsing the B3 plant on a `--clean-prose` corpus.
**Class:** prep defect, caught before it ran.

RC-pre-1.0's ADR body argues from *"rollup cache"* and *"the caller already has a durable store"* —
the exact contradiction [§2.1](protocol.md) removes from every corpus, and false on all five built
ones. Planting it verbatim would have re-imported the noise `--clean-prose` exists to eliminate, and
handed a worker a decision arguing from a premise its own README denies.

**Resolved:** a clean-prose variant of the body ships alongside the original, and the plant script
selects by corpus. Both directions are demonstrated in
[plants/README.md](plants/README.md).

---

## PT-4 · Drained: RC-pre-1.0's `core.hooksPath` watch item

RC-pre-1.0 recorded a watch: `setup` splices its block into a tracked foreign hook and leaves that
file *"modified and uncommitted — its install commit does not carry it."* On rc.11 the install commit
**does** carry it (`53 0 .githooks/pre-commit`), which is M48 Increment 5 live in the field. Recorded
as drained, not re-watched.
