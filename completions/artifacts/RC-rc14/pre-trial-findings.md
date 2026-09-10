# Pre-trial findings — what verifying the handover and the apparatus turned up

**Written 2026-09-09, before any session runs**, by the session executing the trial. Each row
was **driven or computed**, never read off a record. They are recorded here rather than folded
silently into the protocol, because the protocol's own §1 rule — *a finding's class is decided
from its evidence before its consequence is looked up* — binds the apparatus too.

Six rows. Four are corrections to the handover ([handover.md](handover.md) → *Verified*); three
are defects in the trial apparatus itself, all found by running it; and **PT-1 is a defect in
this repo's own gate discipline** that the fold-back fence caught correctly and nobody read.

---

## PT-1 · The gate was red at the sha the handover certifies as green — **BLOCKING, fixed**

The handover records *"Gate at HEAD: **PASS · 3341 passed / 0 failed**, all five steps (probe ·
fmt · clippy · build · test), re-run **after** the version bump and the golden regen"* at
`95c79be6`. Driven at HEAD:

```
$ ./dev/gate
tests   passed=1519 failed=1  (over 7 test binaries)
GATE: FAIL (step: test)
failing tests:
  foldback_truth::claude_md_names_m50_and_claims_only_the_build
```

**The failure predates every change this session made** and is not at HEAD only — it is at the
certified sha. The chain, each link driven:

| fact | evidence |
|---|---|
| the fence existed at `95c79be6` | `git show 95c79be6:crates/cli/tests/foldback_truth.rs` contains `built, not audited` 3× |
| CLAUDE.md at `95c79be6` does **not** contain that phrase | `grep -c` → `0` |
| `95c79be6` is the commit that wrote `built + audited` + the `VERDICT` link | `git log -S"(built + audited — **4 audit findings"` — `95c79be6` |
| it changed **15 files**, and `foldback_truth.rs` was not one | `git show --stat 95c79be6` |
| the fence was in the tree at that point | `git merge-base --is-ancestor 32de1121 95c79be6` → yes |

So the audit-closing commit rewrote the sentence the fence guards and did not touch the fence.
**This is the fence working**, exactly as its own module doc-comment says it must:

> When the audit lands the fence goes red — *which is the fence working, not failing* — and it
> **inverts** rather than relaxing: the stale bound becomes itself the law-1 lie, the completed
> claim is required, and the cited verdict artifact must actually exist.

The inversion was owed in that same commit and was not performed. The gate then either was not
re-run or its failure was not read, and a **PASS with a test count** was recorded at that sha.

**Fixed here, as the fence prescribes** (`crates/cli/tests/foldback_truth.rs`): the arm inverts
to the post-audit direction — `built + audited` required, `built, not audited` now forbidden as
the understatement it has become, a `VERDICT` citation required, and **the cited artifact
checked against the filesystem** rather than accepted as a string, because an unchecked citation
is how this got here. It also forbids the overstatement available *today* — a claim that 1.0.0
is called, which is the human's and was not taken. Renamed to
`claude_md_names_m50_and_claims_exactly_what_the_audit_reached`.

**Why it is in this document and not just fixed.** The trial's whole premise is that a record
written by the session that built the thing can be confidently wrong. Here the record claimed a
green that a standing fence was, at that moment, printing red about. Nothing in the product is
affected — the failing test is a prose fence, and the rc.14 binary is untouched — but the
handover row is false, and the M50 completion audit's own "re-verified" line rests on the same
run.

---

## PT-2 · A third of `test_observe.py` had never run — **apparatus, fixed**

An `if __name__ == "__main__": unittest.main()` block sat **mid-file**, left when the RC-m50
fixes were appended after it. `unittest.main()` collects the module namespace as it stands when
called, so every class below that line did not yet exist.

```
before:  Ran 46 tests in 0.114s   OK
after :  Ran 59 tests in 0.139s   OK
```

The thirteen that never ran include `ABashReadOfAStagedDocumentIsADocumentRead` and
`AFindPipedIntoCatIsARead` — **both fixes for the duress cell's own misfilings**, the two
defects RC-m50's reader found in itself on the cell this trial's headline rests on. They were
fenced by tests that had never executed once, and the suite reported OK throughout.

---

## PT-3 · The reader read one transcript, and said nothing about whose (I-1, I-2) — **fixed**

Both owed items, discharged with tests: `_find` now returns every transcript main-first and
labels a delegated read by its agent; `session._find_transcript` gets the same fix for the
`fork` path (it globbed the *session* id, while subagent files are named by `agentId`); and
`observe --gate <record.json>` refuses evidence from another binary, from a rebuilt tag at the
same sha, or from a directory `run-session.sh` never wrote — and **says whose evidence it read**
on a match. `run-session.sh` now records `image-id`, the strong key the evidence lacked.

Recorded rather than fixed: the archived evidence directories are **flat**, so
`run.py observe <archived dir>` finds neither channel through `_find`, which
`evidence/README.md` implies it can. `--archive` uses its own paths and is unaffected.

---

## PT-4 · PT-D closed: the corpus template's `IngestQueue` is on the live path — **fixed**

Third trial running. `push()` was called from no live path, `tick()` from nothing at all, and
`MemoryStore.prune` was dead for the same reason. `Router` now buffers into the queue and
`main()` ticks. The gate gained a **12th bar that drives the path rather than grepping for it**
— POST, assert the store is still empty, tick, assert the sample landed — so it fails both when
the queue is bypassed and when the drain is severed, and `self-test.sh`'s **13th** mutation is
the historical defect itself (`this.queue.push(` → `this.store.put(`). Suite 23 → 24.

`src/ingest.ts` is untouched: its doc-comment is the committed naming authority plant E's own
bar greps verbatim, and the plant is md5-pinned.

**Declared as a protocol bound:** the fixture moved between RC-m50 and this trial. It is the
right trade — four of last trial's workers spent budget on this dead code, which is itself a
confound on the headline — but it is stated, not hidden.

---

## PT-5 · The rc.13 half of one ledger illustration did not reproduce — **recorded**

The behaviour-change ledger says: *"`jigc doc show 'research:../../outside'` no longer serves
bytes from outside the repository through the pinned JSON at exit 0."* Driven on
`jigc-gate:rc13`, at four traversal depths, on both the committed and the staged path:

```
adr:../../outside/leak          -> exit 1  store.not-found
adr:../../../outside/leak       -> exit 1  store.not-found
adr:../../../../outside/leak    -> exit 1  store.not-found
commit:../../…/outside/leak --task <id>  -> exit 1  store.not-staged
```

Never exit 0, never bytes. The committed read resolves through **git's index**, not through a
path, so a file outside the repository is not findable by traversal on that door; the staged
read names the file by the address literal (`.jigc/tasks/<t>/docs/adr:<slug>.md`), so a `/` in
the slug does not compose the path the claim assumes.

**What is confirmed:** rc.14 answers `store.malformed-slug` at every one of those shapes, which
is the behaviour a session meets and what the `m50` pair probe keys on. **And the outside-repo
read at exit 0 is real at a door the audit names** — M50 audit finding F1, `jigc milestone
add-from-spec`, with its own live repro. So the *class* stands; the ledger's `doc show`
illustration is the part that did not reproduce here, and the pair probe therefore keys on the
**code** (`store.not-found` → `store.malformed-slug`) rather than on an exit that never moved.

---

## PT-6 · The declared bound's own number, and four other handover rows

Recorded in full in [handover.md](handover.md) → *Verified*, and not restated here. The
load-bearing one for the measurement is the **`also open:` conflation** — neither M50 surface
names a read verb, so the headline's mechanism is pre-registered in the protocol rather than
discovered afterwards. The countable one is **79 host-path producers, not 60**, corrected in
all three homes; the per-file counts a test actually checks were right all along.

---

## PT-7 · The FILESYSTEM channel counts **attempts**, not reads — found by running B3-strict

Settlement 1 of the protocol is explicit that **VERB counts attempts**, with `VERB-effective`
shipped beside it and `attempts > 0 / effective == 0` pre-registered as a *finding*. The
FILESYSTEM channel has no such pair, and until B3-strict nothing made the difference visible.

Driven, `RC14-B3-strict` (unscored, `--strict-permissions`):

```
RC14-B3-strict      45    10     6    3   3       read back through the fence's verb
  fs? DOC  [Bash] find .jigc/tasks/record-the-ingest-queue-overflow -type f | xargs … (pipeline read of every file …)
```

That `find | xargs` is **B2's RC-m50 duress bypass, verbatim** — the read the reader was taught
to see after it was missed. It is scored FILESYSTEM here. **It was also denied**: it appears in
`permission_denials` alongside three `Edit` attempts on `src/store.ts`, and the session halted.
So the worker *reached* for the filesystem and the harness stopped it, and the channel reports a
read that did not happen.

**Nothing is mis-scored by this.** The three scored arms run `bypassPermissions`, where no tool
call is denied, so attempt and read coincide; and B3-strict is unscored by design. But the
asymmetry is real and belongs in writing: **VERB distinguishes attempt from effect and
FILESYSTEM does not**, so a FILESYSTEM figure taken under any denying posture is an upper bound.
Recorded rather than fixed — adding a `FILESYSTEM-effective` counter mid-trial would change a
registered measurement while the trial that depends on it is running, which is the one thing
[the driver's own README](../../trial-driver/README.md) says a reader must never do.

**The product observation underneath it is worth more than the reader one, and it is not
adjudicated here:** under the adopter's *real* permission condition, the worker's first move on
an unfamiliar staged task was the filesystem, not a read verb. That is the same reach RC-m50
recorded twice, and it is exactly what N27 is priced against. B3-strict is unscored, so this
does not enter the headline — but it is evidence the headline's adjudication must weigh.

---

## What this changes about the trial

1. The image is built from `21ffc0d4` and gated; the host binary is scored on nothing.
2. The gate is green before the first session runs — **after** PT-1's fix, not before.
3. The reader's control (`observe --archive`) reproduces the 1.0.0-gate table both before and
   after every apparatus change made here.
4. §0 of the protocol completes the behaviour-change ledger rather than citing it as it stands.
