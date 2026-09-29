# M24 — migration-quality measure (the done-bar)

**Run 2026-06-16.** The hardened migration-quality baseline mandated by
[auto-migration.md](../../../design/auto-migration.md) → *Migration-quality measure* +
*Hardening (M24)*, [worked-examples.md](../../../design/worked-examples.md) → flow 26, and
[roadmap.md](../../../implementation/roadmap.md) → M24 Increment 7 (T4). Where the M23 measure
([M23/migration-quality-measure.md](../M23/migration-quality-measure.md)) drove a small corpus
through **per-leaf** authoring, this is the M24 done-bar: the **FULL** `project-delta` +
`project-gamma` live migration — **all** releases, not a sampler — on throwaway clones, each
authored through **ONE** declarative `jigc doc author changelog --from` batch payload (Hardening
#1), measured on the four facts incl. **agent-call count** (#d).

## Binary under test

| | |
|---|---|
| `jigc` sha256 | `25fa3cd1201b16ebfe3d7278d1a08050a87973eb5b4cf861c1c7707e940c5ce4` |
| HEAD commit | `72480ae3883c579dd61bd5ae0864f5a0ca6d1975` (Inc-7 T3, the flow-26 marquee; clean tree) |
| Built | `cargo build --release` |
| Pinned to | `~/.local/bin/jigc` **and** `~/.cargo/bin/jigc` (the roadmap "before any exercise" re-pin; `which jigc` → `~/.local/bin/jigc`) |
| Invocation | `jigc` from `PATH`, with the **embedded** dev pack (no `JIGC_PACK_DIR`) |
| Driver | [`drive.sh`](evidence/drive.sh) — one detached throwaway clone per repo; [`build-payload.py`](evidence/build-payload.py) is the agent's batch-payload authoring tool (Framing A) |

## Honest posture — protocol-counted, not engine-emitted

Per [measurement.md](../../../design/measurement.md) and the settled posture (M17/M23): this is an
**owner-artifact recording**, not a TDD red→green test — its done-criterion is this committed
artifact plus the clean end-to-end runs. The batch path it exercises is independently proven by the
real-binary e2e `crates/cli/tests/flow26_marquee.rs` (Inc-7 T3).

- **Framing A — the agent IS the rewriter.** [`build-payload.py`](evidence/build-payload.py) stands
  in for the coding agent: it reads the foreign source the CLI staged and authors the canonical doc
  as **ONE declarative payload** (the foreign-category/prefix/verb → `category`-enum mapping is the
  agent's editorial judgment, recorded as code). It **places nothing** — the CLI applies create + N
  `add-item` + `set-slot` over a single buffer and owns every placement (the determinism boundary
  stays intact; the payload never touches the `{{source}}` seam).
- **Only metric (a) is CLI-reported.** Round-trip conformance is the binary's own yes/no (finalize
  `--approve` exit 0; `jigc ingest` → adopted). (b) fidelity-acceptance and (c) content-preservation
  are **agent-judged** and the corpus is **agent-reproduced** (the payload). (d) agent-call count is
  **protocol-counted** from the lowered leaf plan. Recorded, not hidden.

## Corpus — the two named real dateless repos, FULL

| slug | repo | foreign shape | foreign `CHANGELOG.md` | releases | change-groups |
|---|---|---|---|---|---|
| `project-delta` | `~/Projects/project-delta` | KaC-ish, `## Changes`/`## Improvements`/`## Features`/`## Fixes`/`## Hotfix`/`## Others` category headings, **dateless** | 127 lines | **13** | 20 |
| `project-gamma` | `~/Projects/project-gamma` | mixed: conventional-commit `feat:`/`fix:`/`refactor:`/`chore:`/`docs:` prefixes + headingless plain bullets + nested sub-bullets, **dateless** | 607 lines | **80** | 127 |

Both are migrated in **full** (every release present in the canonical: 13/13 and 80/80 `### …`
headings — verified in [evidence/](evidence/) `*.log`). Neither carries a doc-level preamble; the
only dropped surface is the empty `# new` Unreleased placeholder (the accepted-drops bound).

### The map + MERGE the agent performed (Hardening #7)

Two foreign categories cannot become two groups under one member — the `category` enum is the
`id-from`, so they'd collide; the agent **merges many-to-one**:

- **project-delta headings** → `Features`→`added`; `Fixes`/`Hotfix`→`fixed`;
  `Changes`/`Improvements`/`Others`→`changed` (the merge: e.g. release 1.2.6's `Changes` +
  `Improvements` collapse into one `Changed` group).
- **project-gamma prefixes** → `feat:`→`added`; `fix:`→`fixed`;
  `refactor:`/`reactor:`/`chore:`/`docs:`→`changed`.
- **project-gamma headingless/plain bullets** → inferred from the leading verb (`fix*`→`fixed`,
  `add*`/`new`/`include*`→`added`, `remove*`→`removed`, else `changed`).

Bullet **prose is preserved verbatim** (only `* `→`- ` normalized, trailing whitespace trimmed); the
original conventional-commit prefix is kept inside the note so nothing is lost on the merge.

## Scores

| slug | (a) round-trip conformance (CLI) | (b) fidelity-acceptance (agent) | (c) content-preservation | (d) agent-call count |
|---|---|---|---|---|
| `project-delta` | **YES** — block(exit 4)→approve(exit 0)→adopt; ingest adopted, idempotent; byte-stable | **accepted** — map+merge faithful, dateless | **clean** — 53/53 bullets verbatim; only `# new` dropped | **1** batch (vs **54** per-leaf) |
| `project-gamma` | **YES** — block(exit 4)→approve(exit 0)→adopt; ingest adopted, idempotent; byte-stable | **accepted** — map+merge+infer faithful, dateless | **clean** — 433/433 bullets verbatim (incl. nested); only `# new` dropped | **1** batch (vs **335** per-leaf) |

Both repos: `jigc task finalize migrate-changelog` **without** `--approve` blocked at the review gate
(**exit 4**, fidelity diff + structural release-delta summary rendered, foreign byte-intact, nothing
committed); `jigc task finalize migrate-changelog --approve` landed (**exit 0**) writing
`changelog/changelog.md`, retiring the foreign `CHANGELOG.md`, and adopting — all in **one** commit
(`docs(changelog): adopt CHANGELOG.md as a managed changelog`, the **auto-provisioned** formulaic
message, Hardening #4; git records the retire+promote as a rename `R072`/`R067`, i.e. `D CHANGELOG.md`
+ `A changelog/changelog.md` collapsed). A follow-up `jigc ingest` reported the managed changelog
**adopted**, and a second `ingest` was idempotent (file-state hash stable). **Zero** date lines in
either canonical doc (`grep -c 'date:'` = 0) — the dateless source fabricated no history (#6).

### (a) round-trip conformance — 2/2

The headline. Both full changelogs parsed + conformance-gated + adopted against the re-pinned HEAD
binary. The enum is genuinely enforced (the agent maps every foreign category into
`added/changed/deprecated/removed/fixed/security` before the strict parser runs; a non-member would
block at `add-item` — proven in `flow26_marquee.rs`). **Byte-stable** confirmed by re-parsing each
committed canonical doc through `engine::write::instance_from_source` → `render` and asserting
`render(parse(x)) == x` (project-delta 2 775 bytes, project-gamma 27 157 bytes) — including project-gamma's
**nested sub-bullet** slot prose (release 1.4.0/1.3.0), a shape the synthetic flow-26 fixture does
not carry.

### (b) fidelity-acceptance — 2/2 accepted (agent-judged)

The review gate rendered the foreign-vs-canonical diff plus the structural release-delta summary; the
agent reviewer accepted both. Non-trivial judgments: the many-to-one **merge** (project-delta
`Changes`+`Improvements`→one `Changed`), and **inferring** the member from `feat:`/`fix:` prefixes
and from leading verbs on project-gamma's headingless plain bullets. All editorial calls the diff makes
visible.

**The structural release-delta summary (#5) behaved as designed — fuzzy/advisory.** Because every
real release was authored, no release was actually dropped; but the heuristic version-scan also picks
up dotted-numeric tokens in **body text** and reported them as "absent from the rewrite":

- `project-delta`: `9.0` (from the bullet "Laravel updated to 9.0").
- `project-gamma`: `1.2.19, 1.2.21, 1.2.23, 1.2.24, 1.2.26` (core versions in "update core to …"), `5.0.0`
  (ElasticPress), `6.0.3` (WordPress), `8.0` (the PHP env).

None is a release; all are body-text mentions — exactly the "fuzzy — can miss or invent a release"
caveat the line is **labeled** with. It fed **no** structural decision (the gate still exited 4); it
is display-only (Framing A, DECISIONS C4). This run is concrete evidence the label is warranted.

### (c) content-preservation — clean within the accepted-drops bound

An automated check (see the run, reproduced in [evidence/](evidence/)) confirmed **every** foreign
content bullet appears **verbatim** in the canonical doc: **53/53** (project-delta) and **433/433**
(project-gamma, counting nested sub-bullets). No silent loss. The only dropped surface in either repo is the
empty `# new` Unreleased placeholder (no content) — within the accepted-drops bound
([auto-migration.md](../../../design/auto-migration.md): preamble · per-release prose summary ·
`[Unreleased]`). Neither repo carries a preamble or per-release prose summary, and neither carries
release dates or compare-links, so no `date`/`link` field mapping was exercised here (both proven
elsewhere).

### (d) agent-call count — the batch payoff

The verbosity signal the declarative batch (#1) is built to cut. Each full migration was authored in
**ONE** `jigc doc author --from` payload. The **per-leaf-equivalent** round-trip count (the M23-style
`create` + one `add-item` per release + one `add-item` + one `set-slot` per change-group; no
date/link fields here) — counted from the lowered plan ([evidence/](evidence/) `*.counts`):

| slug | per-leaf round-trips | batch author calls | cut |
|---|---|---|---|
| `project-delta` | 1 create + 13 release `add-item` + 20 group `add-item` + 20 `set-slot` = **54** | **1** | **54× → 1×** |
| `project-gamma` | 1 create + 80 release `add-item` + 127 group `add-item` + 127 `set-slot` = **335** | **1** | **335× → 1×** |

A 54× / 335× collapse in agent authoring round-trips — the larger the changelog, the larger the cut.
(The CLI still applies every leaf internally over one staged buffer; the *agent* authors once.)

## Bounds (carried in honestly)

- **n = 2 repos, the two named M24 corpora, FULL.** A done-bar scale check, not a distribution.
- **Metrics (b)/(c) are agent-judged, the corpus agent-reproduced** (the payload) — protocol-counted,
  the settled posture. Only (a) is CLI-reported.
- **Changelog only.** Location-bearing doctypes (`adr`/`spec`/`prd`/`arch-doc`) generalize at M25.
- **No date/link mapping exercised** — both corpora are dateless and link-less; the date-survival and
  `link`-field paths are proven by the M23 measure + the test suite, not re-exercised here.
- **No slug-collision case** — every version string is distinct under `slugify` (dots dropped), so the
  accept-and-block collision guard is not re-exercised here; it is proven at the test level.
- **The throwaway clones are detached** (`git checkout --detach`) and discarded; the real repos under
  `~/Projects` are untouched. Canonical outputs, payloads, counts, and full logs are in
  [evidence/](evidence/).

## Verdict — done-bar met

The hardened migration transform arm works end-to-end against the rebuilt HEAD binary on **both full
real dateless repos** through **one batch payload each**: **2/2 round-trip-conformant** (parse +
conformance-gate + adopt, byte-stable incl. nested-bullet slots), **2/2 fidelity-accepted**, **content
preserved 53/53 and 433/433 bullets verbatim** within the accepted-drops bound, and the **agent-call
count cut 54× / 335× → 1×** via the declarative batch. Dateless releases fabricated no date; the
review gate blocked then approved; the migration committed only its own set + jigc's tracked config
with the auto-provisioned formulaic message; the structural release-delta summary behaved as the
labeled fuzzy/advisory aid. No new schemas; Framing A intact. The M24 done-bar is met.
