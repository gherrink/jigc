# Reconciliation

How the CLI handles edits to managed documents that happen **outside** its write path — humans editing files directly, `git checkout` switching branches, `git pull` bringing in upstream changes. Reconciliation is the deterministic state machine that classifies what changed and routes it: **absorb** cleanly, **conformance-block**, or **conflict-block**. Never silent.

Builds on [VISION.md](../VISION.md) principle #3 ("files are truth"; out-of-band edits are detected and reconciled, never forbidden), [write-commands.md](write-commands.md) (the OOB philosophy; staging in `.jigc/tasks/<id>/`), [validation.md](validation.md) (the `file-state` engine-native probe whose output this consumes; severity is tunable per its [Severity inventory](validation.md#severity-inventory)), [storage.md](storage.md) (`.jigc/state/` hash records, the rebuildable edge index), [finalize.md](finalize.md) (the commit phase that updates hashes), and [implementation/parsing.md](../implementation/parsing.md) (the canonical-Markdown parse the classifier calls). For the *why*, see [DECISIONS.md](../DECISIONS.md). Notation is **illustrative** ([what that disclaims](../CLAUDE.md#how-we-work-together)).

## The principle

**Files are truth, so a clean external edit is honored — not merely tolerated** ([VISION.md](../VISION.md) principle #3). The CLI never forbids file edits and never silently discards them. Reconciliation is what turns "the file on disk differs from what the CLI last saw" into one of three outcomes deterministically.

## The state machine

Two axes per managed doc instance.

**Committed-state axis** — the on-disk file relative to the recorded `file-state` hash ([validation.md](validation.md)):

- **`UNKNOWN`** — no recorded hash (fresh checkout, new repo, or first-encountered doc)
- **`IN_SYNC`** — file hash matches recorded hash
- **`DRIFTED`** — file hash differs from recorded hash

**Task-state axis** — the same doc relative to the active task's working area:

- **`UNTOUCHED`** — task has not written to this doc
- **`TOUCHED`** — task has staged at least one write to this doc

The classifier reads the pair and routes:

| committed | task | classifier output |
|---|---|---|
| `UNKNOWN` | (any) | **baseline-adopt** |
| `IN_SYNC` | `UNTOUCHED` | **clean** (no-op) |
| `IN_SYNC` | `TOUCHED` | **task-only change** (proceed normally) |
| `DRIFTED` | `UNTOUCHED` | **OOB edit** — see parse classifier below |
| `DRIFTED` | `TOUCHED` | **conflict** — block at file level; **absorb** when the on-disk bytes equal the doc's blob at the task's base pin (a pulled edit, M55 — see below) |

### Baseline adoption

When the CLI first encounters a managed doc with no recorded hash (`UNKNOWN`), it adopts the current on-disk content as the baseline: hash is computed and recorded; state becomes `IN_SYNC`; the agent sees "baseline adopted: `<doc>`" as an informational line. **Absent-hash is not drift** — a fresh checkout or new repo is the normal first-encounter case.

### OOB edit — the parse classifier

For `DRIFTED + UNTOUCHED`, the CLI re-parses the file against its type's schema ([document-type-schema.md](document-type-schema.md), [implementation/parsing.md](../implementation/parsing.md)):

- **Parse succeeds + schema validates** → **absorb**:
  - re-hash and record the new hash (the `IN_SYNC` baseline shifts forward),
  - update the edge index for any cross-ref changes the edit introduced,
  - surface "external edit absorbed: `<doc>`" to the agent.

  The state returns to `IN_SYNC`. The CLI now treats the on-disk content as the source of truth, which it always was.

  **Persistence of the shifted baseline (pinned at M17 planning, 2026-06-12).** Every sweep absorbs *in memory* for its own run, but the **durable** file-state is persisted **only by a landed `finalize`** — the post-sweep state written in the same motion as the landing commit's re-hash. `validate`, `start`, and read-path sweeps stay pure readers (no write side effects on a read verb). Until M17 this persistence never happened anywhere, so an absorbed edit re-fired its advisory in every later task — the verified over-count defect the M17 measurement substrate fixes ([measurement.md](measurement.md) → The capture substrate). Within-task re-fires before the next landed finalize remain by design; they collapse to one finalize-time emission, which is the counted event.

- **Parse fails OR schema fails** → **conformance-block**:
  - the state stays `DRIFTED`,
  - the agent (and any active task touching this doc) is blocked,
  - the CLI surfaces a precise conformance error — file, line, expected shape (e.g., "missing `{#id}` anchor on item heading `### Rate limit holds`" or "field `date` value `2026/13/01` not a valid date").

  Resolution is human-side: fix the file, or revert the edit. The CLI never auto-repairs in the MVP (see [Auto-repair scope](#auto-repair-scope)).

**The two register-only doors absorb too, and say so — `file-state.absorbed` (M52).** The classifier above is not the only code that advances a committed doc's baseline. Two **register-only** doors do it outside `reconcile_committed` entirely: **`jigc ingest`**, whose `engine::ingest::adopt` re-gates parse + conformance and then records the on-disk hash, and **`jigc rename`**, whose move primitive re-keys the renamed doc at its *post*-drift bytes and whose step 5 re-keys every repointed referrer at theirs. Through `1.0.0-rc.15` both did it **in silence** — a blocking `file-state.hash-matches` was simply gone from the next `jigc validate`, with no line on any surface saying an external edit had been taken, and at `rename` the absorbed bytes were **committed** (`completions/artifacts/M52/baseline-freeze.md` §2.2 F1/F3 and §4 L-1, both driven). That is *What reconciliation does NOT do*'s **"every absorb surfaces"** violated at two doors with no exemption written anywhere.

**Nothing about the classification moves; what was missing was the surface.** The absorb arm at `ingest` *is* the parse + conformance re-gate this section describes, and the block arm already fires on a non-conformant drift (driven: an out-of-band `## Invariants` → `## Invariants Renamed` yields `needs-reconcile` + a routed `conformance.section-renamed`, and the drift finding survives the run). So both doors now emit the advisory **`file-state.absorbed`** ([validation.md](validation.md) → The M52 registrations — Increment 8) on their **success** envelope — one per absorbed path, exit unchanged — naming the path and the `file-state.hash-matches` it retired. The predicate is asked once, at `engine::file_state::absorbed_drift`, so the two doors cannot disagree with each other or with the classifier about what counts as an out-of-band edit. **The subject is every path the door re-baselines, not the one the defect report named:** at `rename` that is the renamed doc **and every repointed referrer**, across all three of its success arms — the reslug, the retitle-only, and the **idempotent no-op**, which stages nothing, commits nothing, and re-keys the baseline anyway. **And the two doors do not gate alike, which is why the advisory's words claim only the baseline:** `ingest` runs the re-gate above, while `rename` has no conformance gate at all and absorbs a non-conformant drift too — driven, the blocking `conformance.section-renamed` survives into the next `jigc validate`, so nothing is greened, but the absorb is real and is now named. Whether `rename` should gate is a question about the *classification*, which M52 deliberately left where it stands.

**Root / placement docs reconcile identically (M38).** A **placement** doctype ([storage.md](storage.md) → Placement) is managed at a literal home — a repo-root file (`VISION.md`, `CHANGELOG.md`) or a direct `docs/roadmap.md` — with `location: None`. Its instance is keyed at that **exact literal path** (fixed slug = type id), and the sweep visits it there, so an out-of-band edit to a managed root file runs the *same* state machine as any managed doc: a conformant edit **absorbs**, a non-conformant one **conformance-blocks / routes** (`needs-reconcile`). Ownership is **exact-path equality against `placement.file`** — every *other* root `.md` (`README.md`, `CLAUDE.md`) is outside the store and untouched (a literal home is not a dir-glob). The M38 sweep learned this placement branch (previously the `location: None` loop skipped placement docs, silently missing their drift; [DECISIONS.md](../DECISIONS.md) → 2026-07-04 M38 Increment 2).

### Conflict — block at file level

For `DRIFTED + TOUCHED`, both sides have moved: the on-disk file changed since the last recorded hash, **and** the task's working area has staged writes to the same doc. The MVP blocks at file granularity:

```
blocking · reconciliation.conflict-block — conflict on `decisions/rate-limit-at-the-gateway.md`: an external edit and this task's staged writes both changed it
  route: `jigc task discard add-rate-limiter` to drop this task's staged writes (discard retires the whole task — no per-doc discard exists), or revert the external edit on disk to keep them — the damage was made out-of-band, so it is repaired where it happened
```

**The conflict route belongs to the caller, not the classifier (M47).** The classifier sees a path, a hash and a *touched* flag — it has no task id, and at one of its callers there is no task at all. It used to hard-code the presentation above, so the **milestone-record doors** (the CLI writing a machine-owned record, [team-ready-state.md](team-ready-state.md) → No-silent-overwrite discipline) blocked with an **inapplicable verb** carrying an **unsubstituted `<task-id>`** — the pair the M43 route floor exists to prevent on a *blocking* finding ([surface-contract.md](surface-contract.md) → The route fence). So the mover clause **and** the route are now supplied per caller, from what that caller actually holds: the task-scope sweep passes the **real task id** (no placeholder survives the print), the milestone join gate routes at the sub-task listing (the merged area belongs to no single task), and the record door names the *record* and routes a human revert to what jigc last wrote — an external edit to a machine-maintained record is never merged and never clobbered. The frame the classifier still owns is only `` conflict on `<path>`: `` + the location.

**A pulled edit is not a conflict — the base pin decides (M55, L1).** A teammate's committed edit reaches a clone by `git pull`, which moves the file and leaves the gitignored baseline where the last landed finalize put it — so the doc reads `DRIFTED`, and the next task to edit it read `DRIFTED + TOUCHED` and was refused, routed at reverting the teammate's edit ([findings-channel.md](findings-channel.md) → §6, L1). Both sides had *not* moved: the drift predates the task. The arm now asks one more fact, the doc's **blob at the caller's base pin** — a caller-supplied lookup the engine hashes with the drift hash, since the engine shells out to nothing — and when the on-disk bytes equal it, runs the `DRIFTED + UNTOUCHED` **absorb** above whole: the conformance gate, the re-hash, the edge-index update, the `reconciliation.absorb` advisory. Never only the re-hash.

- **A change made during the task still conflict-blocks.** The pin is fixed at mint, so an edit landed after it — on disk, or committed and pulled after the mint — differs from the pin's blob, and the route above is unchanged.
- **A pinned edit that does not conform keeps the caller's conflict-block, byte-identical — never a conformance-block.** A non-conformant edit is never baselined, and the conflict presentation is the caller's: re-grading it would silently retire the migration source's path-keyed third exit below, whose subject — a managed doc hand-broken out of band, then migrated — is exactly an edit committed before `jigc migrate`, and so exactly at the pin.
- **The pin is the caller's.** The per-task gate binds the task's base pin; a sub-task's pin is its milestone's, so a pull **before** `milestone create` is absorbed at the join, while a pull **after** it moved the doc past the pin every sub-task inherited and still blocks. The **milestone-record door passes no pin** — every drift of the machine-owned record conflict-blocks there, pulled or not ([team-ready-state.md](team-ready-state.md) → No-silent-overwrite discipline, F3). A pin with no blob for the path, and any git failure, keep the conflict-block.
- **The absorb is in memory, like every sweep's.** *Persistence of the shifted baseline* above stands as it is: `start` and `task validate` report the absorb and write nothing; the baseline advances only when the task's finalize lands, and that finalize re-promotes the task's staged copy over the pulled bytes it carried in.
- **At store scope the same pull is a lagging baseline, not an out-of-band edit.** `jigc validate` absorbs nothing — its file-state twin is detect-without-absorb ([validation.md](validation.md) → read-only file↔CLI-state at store scope) — but it tells the two apart with the same kind of fact, a caller-supplied lookup bound to **`HEAD`**. A recorded doc drifted from its baseline whose on-disk bytes equal its blob at `HEAD` **and** pass the conformance gate grades the existing **`file-state.hash-matches` at advisory**, routed informationally *"the baseline lags `HEAD`; absorbed at the next finalize"*. The `(code, target)` key and the message do not move; the severity and the route are the change — the no-new-id pattern `rename_dangling_baseline_finding` set by reusing `reconciliation.rename` at advisory. Its population is wider than a pull: a conformant hand edit committed with plain `git commit` reads the same, and the route is true of both, since any landed finalize's sweep walks every committed doc and absorbs a conformant drift. **A committed edit that fails the conformance gate keeps the blocking finding** (and family 5 its conformance finding) — matching bytes are not on their own a clean doc — and an uncommitted edit is not at `HEAD`, so it keeps it too. Nothing is baselined at store scope either way.

**The revert exit carries its sanction, and the migration source gets a third exit (M46).** Two repairs to the same route, both from the pre-guard repair audit ([roadmap](../implementation/roadmap.md) → Milestone 46, Increment 5).

The general route's second exit — *revert the external edit on disk* — is, over a managed doc, precisely the act the adapter's routing sentence forbids ("never read or edit managed docs directly", [assistant-adapter.md](assistant-adapter.md) → Inject the bootstrap). The route was right and the prohibition is right; what was missing is the clause that reconciles them, and it already existed one screen away as the conformance-block's hand-repair sanction — *the damage was made out-of-band, so it is repaired where it happened*. It is now **one source, three producers**: the blocking conformance-block, the advisory `UNKNOWN` arm, and this route.

The **migration** case needed more than a sanction. A `jigc migrate <path> --as <type>` task's staged rewrite of **its own recorded source** is `DRIFTED + TOUCHED` whenever that source is a managed doc that was hand-broken out of band — which is the whole reason the migration was started. Both general exits are then dead: discarding retires the migration, and the revert restores exactly the corruption being repaired. So the *source path, and only the source path*, is offered a third exit — `jigc unmanage <source>`, which drops that path's baseline (and its forward edges) and leaves the bytes on disk, after which the migration finalizes normally through its review hold. The exit is **keyed on the path**: the same task conflicting on any *other* doc, and every non-migration task, keeps the general route, because dropping a baseline guard is only free where the task's staged version is already the intended replacement ([storage.md](storage.md) → What none of this buys states what a lost baseline costs; [surface-contract.md](surface-contract.md) → A route offered on a wider domain must be gated on that domain). The route states that cost inline, and states it **as it is**: the on-disk bytes are not merged, the staged rewrite replaces them — and that rewrite was authored against the source *as the task recorded it at mint*, so an edit landing on the file after the mint is replaced without appearing in the `--approve` fidelity diff, which renders the recorded seam and not what is on disk now. The route therefore keeps naming the general exit that **does** preserve those bytes — `jigc task discard <id>`, which retires the migration and leaves the file untouched — so the operator still chooses between replacing the file and keeping it, rather than being sent to a review that cannot show what is about to go (M46 Increment 5, validate→fix).

Resolution paths (task scope):

- Agent discards the **whole task** (`jigc task discard <id>`) — no per-doc discard exists. (A per-write discard verb was sketched at MVP and never built; the route was repaired to the real verb at M43, [DECISIONS.md](../DECISIONS.md) → 2026-07-16 Settle. Doc-granular discard is deferred with the same machinery as three-way merge.)
- Human reverts the on-disk edit; the task's writes are kept — sanctioned, out-of-band damage being repaired where it happened.
- On a **migration task's own source**: `jigc unmanage <source>` drops the stale baseline, and the migration lands its rewrite over it — the rewrite of the source *as recorded at mint*, so a later on-disk edit is replaced unreviewed (the `--approve` fidelity diff renders the recorded seam). Not offered on any other path, and offered **beside** the whole-task discard, which is what keeps the on-disk bytes.
- **Three-way merge** of both sides is deferred — it parallels override-conflict resolution ([overrides.md](overrides.md)) and rides on the same future machinery.

**Discard is always explicit, never the default, never silent** — no data loss.

## Rename detection

Files-as-identity means a **path change is an identity change** ([storage.md](storage.md) → Identity, order, fields, slots, items). A human running `git mv decisions/rate-limit.md decisions/gateway-rate-limit.md` isn't editing the file's *content* — they're (effectively) renaming the artifact. The hash-on-known-path state machine above doesn't catch this case: it sees the old path as *missing* and the new path as *new untracked*.

So rename detection is a separate classifier, running at the same trigger points as the state machine ([Detection timing](#detection-timing)). Two signals, two outcomes:

- **Strong signal — suspected rename.** A tracked path is missing **and** an untracked path has the **same content hash** as the last recorded hash for the missing one. The CLI surfaces a **blocking** finding naming the detected rename and the resolution — routing to the owned op first, revert second:
  ```
  error: tracked managed doc adr:rate-limit (decisions/rate-limit.md) is missing
    hint: a file at decisions/gateway-rate-limit.md has the same content hash —
          likely renamed via a bare `git mv`.
    resolution:
      adopt it as a CLI-owned rename (re-points every referrer atomically):
        $ jigc rename adr:rate-limit --to "Gateway rate limit"
      or revert the out-of-band move:
        $ git -C /abs/path/to/repo mv decisions/gateway-rate-limit.md decisions/rate-limit.md
  ```
  **The candidate space, and the placement carve-out it used to carry** (M53 — the pre-v1 usability batch, row 1). A `location:` doctype's candidates are the unrecorded `.md` files in its location directory. A **`placement`** doctype has one literal home, and the census enumerated exactly that path — so for the whole placement family (every managed singleton a stock corpus has: `VISION.md`, `CHANGELOG.md`, `docs/roadmap.md`, `docs/decisions-log.md`) the strong signal was **structurally unreachable**, because a rename is precisely the event that empties the declared home. Every out-of-band `git mv` of a singleton therefore reached the weak arm below and was routed to `jigc unmanage` — the one act that drops the identity of a doc sitting right there under a new name — and the installed pre-commit hook's blocking backstop, which keys on this route's `mv` pair, was inert over that family. So when a placement doctype's declared home is **absent**, the `.md` siblings in that home's own directory are candidates; when it is present, the census is unchanged and reads no sibling. What a candidate can *become* is still gated by the content-hash match, so an unrelated sibling contributes a candidate and never a finding.
- **Weak signal — tracked path missing, no content-matching new file.** The tracked file is simply gone (deleted, accidentally removed). The CLI surfaces a conformance error: *"tracked managed doc adr:rate-limit (decisions/rate-limit.md) is missing — restore the file, or run `jigc delete adr:rate-limit` to confirm deletion (post-MVP)."* Routes to restore.

**Policy: an out-of-band move is blocked and routed to the owned op; the CLI never auto-honors an untracked rename.** A path/slug rename is an identity change, and identity changes are CLI-orchestrated through **`jigc rename`** ([write-commands.md](write-commands.md) → `jigc rename`) — the prevention happy-path that re-keys `file-state` and repoints every referrer atomically. A bare `git mv` is the *out-of-band* case: detected and routed ("adopt as `jigc rename`" / revert), **never** silently absorbed and never auto-rewritten. Same shape as the rest of strict scope ([Auto-repair scope](#auto-repair-scope)): the finding names exactly what happened; the agent runs the owned op (or reverts). `jigc delete` (deletion confirmation) stays post-MVP.

**Amended at M17 planning (2026-06-12) — "missing" means absent from disk, not absent from the walk.** The implementation inferred *missing* from a baselined path not appearing in the sweep's walked set — but the walk is a per-schema, non-recursive `<location>/*.md` glob, so a baselined path outside every walked location (the case that fired: a promoted **owner-artifact** under `completions/artifacts/<run>/`, baselined at its finalize) was declared missing on every subsequent sweep, false-blocking every later `finalize` in the repo via `reconciliation.rename`. The fix (the dogfood pre-fix set, [DECISIONS.md](../DECISIONS.md) 2026-06-12): before classifying a tracked path as missing, **check disk presence** — present-on-disk-but-outside-the-walk is *not* missing (no finding; content stays baselined as-is). Chosen over excluding `completions/artifacts/**` from the baseline, which would blind the sweep to a genuinely deleted artifact dangling behind a committed `completion-record`'s `owner-artifact` field. The strong/weak rename signals above are unchanged — they now simply fire only on genuine disk absence.

**`jigc rename` is a single atomic transaction with its own rollback boundary — *not* `finalize`'s** (M35). `finalize` promotes a task working area by **copy** (rollback = delete the copy); `jigc rename` mutates **committed paths + file-state + the edge index directly**, so it owns a distinct transaction: capture pre-image bytes of the old path + every referrer to be rewritten → rewrite each referrer's ref-field → `git mv` → stage → run the user's git hooks (never `--no-verify`) → commit; on **any** failure restore all captured bytes and reverse the `git mv` → then re-baseline `file-state` (`forget(old)` + `record(new)` + re-record each rewritten referrer) and invalidate/rebuild the index. The migrate-corpus *no-rollback / idempotent-rerun* model is **rejected** for rename: rename is **not idempotent-safe** (its re-run keys on `<old>`, which the `git mv` already destroyed, so a half-done rename can't resume — it would leave the exact dangling-ref violation rename exists to prevent). Full spec: [write-commands.md](write-commands.md) → `jigc rename`.

**Out-of-band rename — store-scope detection is the backstop, and it blocks (M35, the A+B decision).** `jigc rename` is the prevention; the store-wide sweep is the safety net under it for the case where an agent does a bare `git mv` and commits *without* the verb. Two parts:

- **A — wire the strong rename classifier into store scope, firing first.** The strong signal (missing tracked path + content-hash match) is the *only* detector that catches a **referrer-less** OOB move (no dangling ref to find), and it carries the actionable "adopt as `jigc rename`" route. It runs in the store sweep **before** the store-wide `ref-resolves` family ([validation.md](validation.md) → forward-ref integrity), so a moved-with-referrers doc is diagnosed as *a rename to adopt/revert* rather than as N independent dangling refs — the two never emit competing diagnoses for one event.
- **B — the OOB-rename finding is blocking / exit-flipping, and the pre-commit hook blocks the commit.** This is a narrow, deliberate amendment to the store-scope **report-only** stance and the **warn-only** pre-commit hook ([validation.md](validation.md) → Exit semantics; → Auto-firing the sweep). *Engaging that rationale (record-is-rebuttable):* the report-only/warn-only stance exists so the sweep never gates on **pre-existing, unrelated content rot** (the masking-trap — a commit blocked by drift it didn't cause). An OOB rename is **not** pre-existing content drift — it is a **structural-identity corruption introduced *by this commit***, so blocking it is *consistent with* that rationale, not a violation. It joins the `pack-probe-integrity.*` exit-flipping exception class (a trust/integrity event, not a tunable content opinion); general content `ref-resolves` stays report-only. The hook blocks only on the rename detected among the files **this commit** moves, never on pre-existing store drift, so the masking-trap guard is preserved.

## Relocation collisions — two resolutions for one collision (M39)

When a doctype's **home moves** and its pre-existing instances are detected + routed to the new home (the freeze-exempt relocation floor + `jigc config set docs-root`; [storage.md](storage.md) → Placement, [validation.md](validation.md) → Orphan detection), the destination path may **already be occupied**. The collision is resolved by the *kind* of the occupant — one collision, two resolutions, chosen deterministically:

- **A *managed* instance already at the destination → adopt-in-place.** The destination already holds a conformant managed doc (the M24 in-location squatter path — `jigc migrate-corpus` / the foreign-root-`CHANGELOG.md`-adopts-in-place case): the instance there is kept and adopted at its canonical path, no move. Moving the *incoming* managed instance *into* an already-managed destination — a **managed move-INTO** — is **deferred**: it needs an unmodeled "managed-but-uncommitted" doctype state, so the relocation **surfaces the collision as a block** (never a clobber), routed to a human. (Move-INTO for a managed instance is out of scope for M39; [DECISIONS.md](../DECISIONS.md) → 2026-07-06 move-INTO foreign-only.)
- **A *foreign* (untracked/unmanaged) file at the destination → move-into-workbench.** The destination is squatted by a file the store does not manage (no `file-state` baseline). It is **relocated into the gitignored `.jigc/` workbench** (a dedicated `displaced/` subpath — [team-ready-state.md](team-ready-state.md) → `.jigc/` is the workbench), **out of** the destination and **uncommittable**, so the managed instance can land *and* a stray working file is never accidentally committed and never silently clobbered. The foreign bytes are preserved verbatim in the workbench; a committed-but-unmanaged squatter's tracked-path index slot is freed so the managed `git mv` into the vacated destination succeeds.

Classification is by **baseline**: a managed instance is recorded in the `file-state` map (adopt-in-place / block); a foreign file is not (move-into-workbench). This is the same managed-vs-foreign line the rest of reconciliation draws — the store owns what it has baselined; everything else is foreign and detected + routed, never absorbed or destroyed. The workbench move-into is the settled **foreign-only** bound: it is the safe half of the collision (a foreign file has no identity or referrers to preserve), while the managed half waits on the managed-but-uncommitted state it needs.

## Detection timing

The `file-state` probe fires at six points; reconciliation runs the classifier at each:

| trigger | scope | purpose |
|---|---|---|
| **task `start`** | every managed doc the workflow will read or write | baseline-adopt unknowns, absorb clean drift, block before the task does any work |
| **read through the CLI** | the doc being read | absorbs benign drift transparently; the agent sees current truth |
| **write through the CLI** | the target doc | re-probe before staging; block if the doc has become conflicted since the task started |
| **`jigc task validate`** | every doc in the task's scope (working area + referenced docs) | full sweep — the same `file-state` sweep `finalize`'s preflight runs |
| **`jigc task finalize` preflight** | same as validate | `finalize` runs the same validate phase, then the commit-time gates ([finalize.md](finalize.md) → 2. Validate) |
| **`jigc validate` (ad-hoc)** | scope-flexible (doc / store) | manual check; no task context required |

Probing is cheap (one hash per doc); the classifier and the parse only run when state is `DRIFTED`.

## Hash re-baselining

The recorded `file-state` hash updates at the sites below — keyed to "last-known-good committed state". **This sentence read *"at exactly four sites"* until M52; the claim is struck with its datum rather than renumbered.** Driven at HEAD, two more writers put a fresh hash into the record and neither is any of the four: a landed `jigc migrate-corpus` re-baselines every doc it rewrote (`crates/cli/src/migrate_corpus.rs`, the per-doc gated write, which re-registers a moved doc at its destination and re-hashes an in-place one at its key) and `jigc rename`'s own transaction re-baselines every referrer it rewrote (`crates/cli/src/rename.rs`, step 5 of the boundary). It is struck rather than re-counted because what the list is *for* is the reconciliation reader's question — which sites decide whether a doc reads as drifted — and that is a judgment about this state machine, not a registry: a numeral over a set the code can move, with nothing fencing it, is the shape this repo corrects rather than restates (`crates/cli/tests/count_fences.rs`).

- **Baseline adoption** — first encounter (`UNKNOWN`). Hash = current on-disk content's hash.
- **Absorb** — clean external edit accepted. Hash = new on-disk content's hash.
- **Commit** — `finalize` phase 7 ([finalize.md](finalize.md)). Hash = just-committed content's hash for every managed doc the commit touched.
- **A landed milestone record-op commit** — one of the **five record-only doors**, whose enumeration is `cli::rollback::ROLLBACK_POPULATIONS`' `milestone-record` row and not a list here ([team-ready-state.md](team-ready-state.md) → The commit model). Hash = the just-committed record's hash. *This site was live since M39 and this list said "exactly three" until M47 — a doc that under-counts its own re-baselining sites makes the next commit door undiscoverable, and M47's remedy was to enumerate here rather than leave it to the code. That remedy did not hold: the copy this bullet carried read* `create` · `add-task` · `add-from-spec` · `discard` *— four where the code has five, `jigc task discard <sub-task>` landing its own record-only commit and named in no design doc — so by M52 the prose list had gone stale exactly as the count above did. The remedy that holds is to point at a list something checks* (`completions/artifacts/M52/settle-record.md` → §19; the pointer is fenced against the registry by `crates/cli/tests/count_fences.rs`). The **binding condition is "landed"**: the door captures the record's pre-image and, on a rejected commit, restores it and re-baselines **nothing** — nothing landed, so the rule above is honoured untouched and the restored bytes still match the hash the last landed write recorded ([finalize.md](finalize.md) → Rollback discipline, the record-only-door row).

Task writes do **not** update the committed-state hash. The working area is separate; the hash tracks "what the committed file looked like the last time we agreed with it." Until `finalize`, the task's writes live in the working area and the committed-state hash stays pinned.

## Auto-repair scope

The MVP does **not** auto-repair conformance gaps, even ones that look mechanically obvious:

- **Dropped `{#id}` anchor** — even when the heading text still matches the recorded id-source field value, the MVP blocks and asks the human to paste the anchor back. The id is frozen and recoverable from the recorded state, but silently reconstructing identity is precisely the kind of "the CLI just does things" behavior the determinism boundary is meant to prevent.
- **Field value re-formatting** (`2026/05/28` → `2026-05-28`) — block; the human picks the canonical form.
- **Trailing whitespace, line endings, CRLF vs LF** — caught at parse time; whether the parser ignores these silently or surfaces them as conformance findings is a parser-implementation question ([implementation/parsing.md](../implementation/parsing.md) → open questions).

The rule: **the conformance error names exactly what's missing; the human fixes it.** Auto-repair (anchor re-mint, whitespace normalize, date canonicalize) is a post-MVP feature that requires its own decision per category. Strict MVP keeps the boundary visible and the failure modes audible. Easier to relax later than to tighten.

## MVP scope vs post-MVP

**MVP ships:**

- Detection — the `file-state` probe ([CLAUDE.md](../CLAUDE.md) → MVP scope).
- The full state-machine classifier above.
- Baseline adoption on first encounter.
- Clean-absorb path (parse + schema-validate → re-hash + edge-index update).
- Conformance-block with precise error surfacing.
- File-level conflict-block with explicit-discard resolution.
- Hash re-baselining at the sites [above](#hash-re-baselining) — the four that decide whether a doc reads as drifted, the count struck with its datum.
- **Rename detection** — strong signal (path missing + content-hash match) and weak signal (path missing alone), at task scope (blocking) and **store scope** (M35: blocking, firing before `ref-resolves`), routed to "adopt as `jigc rename`" / revert per [Rename detection](#rename-detection).

**Post-MVP (deferred):**

- **Auto-repair categories** — anchor re-mint, whitespace normalization, date canonicalization. Each is a separate per-category decision; none ships until demand is concrete.
- **Section/leaf-level conflict granularity** — partial absorption when OOB-edit and task-touch don't overlap section-by-section. Pairs with three-way merge.
- **Three-way merge for both-sides-changed conflicts** — currently deferred ([overrides.md](overrides.md); shared machinery with override-conflict resolution).
- **`jigc import` for entirely new untracked files** — adopting a brand-new `.md` file authored outside the CLI ([implementation/parsing.md](../implementation/parsing.md) → "mint-on-import lands with the full import flow"). MVP imports only edits to *managed* docs the CLI already knows about.
- **`jigc delete`** — confirms a deletion (removes the file-state record, surfaces dangling referrer refs). MVP detects OOB deletions and routes to restore; the op ships post-MVP ([write-commands.md](write-commands.md) → `jigc delete`). *(`jigc rename` shipped at M35 — the [Rename detection](#rename-detection) backstop now routes to it; see [write-commands.md](write-commands.md) → `jigc rename`.)*

## What reconciliation does NOT do

- **No silent discard, ever.** Every block surfaces; every absorb surfaces — including the two **register-only** absorbs that never reach the classifier (`jigc ingest`, `jigc rename`), which say so with `file-state.absorbed` ([above](#oob-edit--the-parse-classifier)). There is **no door exemption** from this sentence, and the two doors that had one in practice are the reason it now names them.
- **No three-way merge.** Both-sides-changed blocks, never auto-resolves.
- **No file forbidding.** The CLI never tries to prevent edits — it detects and reconciles.
- **No identity reconstruction.** A dropped `{#id}` blocks; the CLI doesn't guess the id from context, even when it could.
- **No silent rename.** A path change is an identity change ([Rename detection](#rename-detection)); the CLI never auto-rewrites referrer refs to follow a *bare* `git mv`. The owned op **`jigc rename`** (M35) does it as one explicit, atomic transaction; an out-of-band move is detected and **blocked** (store scope), routed to adopt-or-revert — never silently followed.
- **No LLM call.** The classifier is deterministic — same `(file content + recorded hash + task working area + schema)` in → same classification out ([VISION.md](../VISION.md) principle #1).

## Open questions

- **Parser-tolerant vs parser-strict for cosmetic drift** — trailing whitespace, line endings, CRLF vs LF. Which categories pass parse silently vs surface as conformance findings is parser-implementation territory ([implementation/parsing.md](../implementation/parsing.md)) and not yet decided.
- **Concurrent OOB edits during a task** — if the human edits a file while the task is writing to its working area, the write-through-CLI probe catches it (timing row 3 above), but the window *between* two writes is unprotected. The file-level conflict-block fires at the next probe; only missed if the edit is reverted before the next probe (in which case there's nothing to reconcile).
- **External edit notification surface** — "external edit absorbed: `<doc>`" is the agent-facing line; how the assistant adapter surfaces this (inline in workflow output? separate notification?) is part of the broader blocked/error-payload question ([write-commands.md](write-commands.md#open-questions)).
