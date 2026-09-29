# M25 — migration-quality measure (the generalized done-bar)

**Run 2026-06-17.** The generalized migration-quality baseline mandated by
[auto-migration.md](../../../design/auto-migration.md) → *Migration-quality measure* +
*Generalizing to adr/spec/prd* + *Honest bounds*, [worked-examples.md](../../../design/worked-examples.md)
→ flow 27, [measurement.md](../../../design/measurement.md) (recorded-alongside posture), and
[roadmap.md](../../../implementation/roadmap.md) → M25 Increment 7 (T2). Where the M24 measure
([M24/migration-quality-measure.md](../M24/migration-quality-measure.md)) drove the changelog
*singleton* through one batch, this is the M25 done-bar: the **location-bearing, multi-instance**
doctypes `adr`/`spec`/`prd`, **one file at a time** (NOT a sweep — [auto-migration.md](../../../design/auto-migration.md)
→ Migration model), on throwaway clones, measured on the four facts incl. **agent-call count**.

## Binary under test

| | |
|---|---|
| `jigc` sha256 | `aa61ac5ba865581353124fac3bfd8535940e88b1c3a5bedc5fb2a0a1639f5fe4` |
| HEAD commit | `c5fef70` (Inc-7 T1, the flow-27 marquee; clean tree) |
| Built | `cargo build --release` |
| Pinned to | `~/.local/bin/jigc` **and** `~/.cargo/bin/jigc` (the roadmap "before any exercise" re-pin; `which jigc` → `~/.local/bin/jigc`; all three sha256 identical) |
| Invocation | `jigc` from `PATH`, with the **embedded** dev pack (no `JIGC_PACK_DIR`) |
| Driver | [`drive.sh`](evidence/drive.sh) — one detached throwaway clone / scratch repo per arm; the `payload-*.yaml` files are the agent's declarative batch payloads (Framing A) |

## Honest posture — protocol-counted, not engine-emitted

Per [measurement.md](../../../design/measurement.md) and the settled posture (M17/M23/M24): this is
an **owner-artifact recording**, not a TDD red→green test — its done-criterion is this committed
artifact plus the clean end-to-end runs. The generalized spine + the six reds are independently
binary-proven by `crates/cli/tests/flow27_marquee.rs` (Inc-7 T1) and the per-increment
`migrate_adr.rs` / `migrate_spec.rs` / `migrate_prd.rs` tests.

- **Framing A — the agent IS the rewriter.** The `payload-*.yaml` files stand in for the coding
  agent: each reads the foreign source the CLI staged and authors the canonical doc as **ONE
  declarative payload** (the MADR-section→schema-section map, the foreign-status→3-enum mapping,
  the supersedes edge-or-prose call, and the criteria/requirement extraction are the agent's
  editorial judgment, recorded as a payload). The payloads **place nothing** — `jigc doc author
  --from` applies create + N `add-item` + `set-slot`/`set-field` over one staged buffer and owns
  every placement, the slug, the retire, and the commit (determinism boundary intact; the payload
  never touches the `{{source}}` seam).
- **Only metric (a) is CLI-reported.** Round-trip conformance is the binary's own yes/no (finalize
  `--approve` exit 0; `jigc ingest` → adopted; a second `ingest` idempotent). **Byte-stability**
  (`render(parse(x)) == x`) over these exact doctype shapes is asserted by `flow27_marquee` (T1)
  against this same binary — not re-implemented here. (b) fidelity-acceptance and (c)
  content-preservation are **agent-judged** and the corpus is **agent-reproduced** (the payloads).
  (d) agent-call count is **protocol-counted** from the lowered leaf plan. Recorded, not hidden.

## Corpus — a real ADR set (two supersession chains) + a real spec + a synthetic prd

| arm | source | foreign shape | files | canonical home |
|---|---|---|---|---|
| **adr** (headline) | `thomvaill/log4brains` `docs/adr/` | real MADR/Nygard set; `- Status:` / `- Date:` list-marker headers; MADR sections (Context and Problem Statement · Considered Options · Decision Outcome · Decision Drivers · Positive/Negative Consequences · Pros and Cons); **two supersession chains** | **4** | `decisions/` |
| **spec** | `semver/semver` `semver.md` | real, idiosyncratic: a Summary, an Introduction, **11 numbered RFC-2119 normative clauses**, a BNF grammar, "Why Use…", an FAQ | **1** (373 lines) | `specs/` |
| **prd** | **SYNTHESIZED — labeled synthetic** | a fabricated product brief (`evidence/synthetic-prd-source.md`); prd has **no real foreign-corpus standard** (the honest bound) | **1** | `prds/` |

Each foreign file migrated **one at a time**, sequentially, through the doctype-general spine:
`jigc migrate <path> --as <doctype>` (mints the **per-file** task `migrate-<doctype>-<slug(path)>`,
surfaces the source) → `jigc doc author <doctype> --from -` (ONE batch) → `jigc task finalize
<task>` **without** `--approve` (review-gate block, exit 4) → `jigc task finalize <task> --approve`
(promote + retire + adopt, one commit) → `jigc ingest` ×2. Full logs + payloads + canonical outputs
+ byte-fingerprints in [evidence/](evidence/).

### The two real supersession chains (the edge-wiring headline)

Migrated **target-first** (the dependency-ordering contract — one-file-one-task has no cross-task
transaction machinery, so a supersession target must be committed before the ADR that supersedes
it). The foreign `- Status: superseded by […]` maps to the 3-enum `superseded` on the **target**,
which authors **no** `supersedes` ref (the inverse `superseded-by` edge is derived from the
superseder's `supersedes`):

| order | foreign ADR | canonical slug | status | `supersedes` (bracket-list, 0..*) |
|---|---|---|---|---|
| 1 | `20200926-use-the-adr-number-as-its-unique-id` | `use-the-adr-number-as-its-unique-id` | `superseded` | — (target) |
| 2 | `20201016-use-the-adr-slug-as-its-unique-id` | `use-the-adr-slug-as-its-unique-id` | `accepted` | `[adr:use-the-adr-number-as-its-unique-id]` |
| 3 | `20240926-transition-to-simplified-git-flow` | `transition-to-simplified-git-flow` | `superseded` | — (target) |
| 4 | `20241217-switch-back-to-github-flow` | `switch-back-to-github-flow` | `accepted` | `[adr:transition-to-simplified-git-flow]` |

Files 2 and 4 finalized **clean** (exit 0) because their `supersedes` target was already committed
to the store — **edge integrity across the migrated set**, the live proof of the ordering contract's
positive arm.

## Scores

| arm | (a) round-trip conformance (CLI) | (b) fidelity-acceptance (agent) | (c) content-preservation | (d) agent-call count (authoring) |
|---|---|---|---|---|
| **adr** (4 files) | **YES** — each: block(exit 4)→approve(exit 0)→adopt; all 4 ingest **adopted**, idempotent; edges resolve across the set | **accepted** — MADR→schema map + status→enum map + edge-or-prose faithful | **clean within accepted-drops** — Context/Decision/Consequences preserved; Considered Options · Decision Drivers · Pros-and-Cons dropped (the adr bound) | **4 batches** (vs **26** per-leaf) |
| **spec** (1 file) | **YES** — block→approve→adopt; ingest **adopted**, idempotent | **accepted** — 11/11 normative clauses → criteria, RFC-2119 wording preserved | **clean within accepted-drops** — Summary→goal, Introduction→context, 11 clauses→criteria; BNF grammar · "Why Use…" · FAQ · About · License dropped (the spec bound) | **1 batch** (vs **25** per-leaf) |
| **prd** (1 file, **SYNTHETIC**) | **YES** — block→approve→adopt; ingest **adopted**, idempotent | **accepted** — vision + 4 requirements + context faithful | **clean** — all 4 requirements as repeatable items, vision + context preserved | **1 batch** (vs **11** per-leaf) |

All six files: bare finalize **blocked at the review gate** (exit 4, foreign byte-intact, nothing
committed); `--approve` **landed** (exit 0) writing the canonical doc, retiring the foreign original,
and adopting — each in **one** commit with the **auto-provisioned** formulaic message
(`docs(<type>): adopt <foreign> as a managed <type>`; git records `D <foreign>` + `A
<canonical-home>/<slug>.md`). The first commit per repo also lands jigc's tracked config layer
(`.jigc/config/`, `.jigc/.gitignore`) — the narrowed migration `git add` (Hardening #9).

### (a) round-trip conformance — 6/6 (CLI-reported)

The headline. Every foreign file parsed + conformance-gated + adopted against the re-pinned HEAD
binary; a follow-up `jigc ingest` reported the managed doc **adopted** (never `unmanaged` /
`needs-reconcile`) and a second `ingest` was idempotent (file-state hash stable). Across the four
ADRs the `adr.supersedes` edges (widened `0..*`, bracket-list) **resolve against the committed
store**. **Byte-stability** (`render(parse(x)) == x`) over these exact shapes — adr-with-supersedes,
spec-with-criteria-no-anchor, prd-with-repeatable-requirements — is asserted by `flow27_marquee`
(Inc-7 T1) against this same binary.

### (b) fidelity-acceptance — 6/6 accepted (agent-judged)

The review gate rendered the raw foreign-vs-canonical diff (no per-doctype structural summary — the
M25 fallback; the changelog version-scan has no analog here). The agent reviewer accepted all six.
Non-trivial editorial judgments the diff makes visible:

- **adr** — mapping the foreign `Status: superseded by […]` / `Status: accepted` onto the 3-enum;
  folding MADR's `Positive Consequences` + `Negative Consequences` into the single `consequences`
  slot; deciding the `supersedes` edge (in-set → structural ref) vs prose (out-of-set, not used in
  this all-in-set corpus); transcribing the historical `Date` (date-survival).
- **spec** — extracting 11 testably-phrased criteria from the numbered normative clauses, condensing
  clause 11's nested sub-clauses into one criterion statement.
- **prd** — splitting the prose "What it must do" bullets into 4 individually-addressable repeatable
  requirement items.

### (c) content-preservation — clean within the accepted-drops bound

- **adr**: the foreign Context / Decision / Consequences content is preserved in the matching slots;
  the surfaces with no schema home — **Considered Options**, **Decision Drivers**, **Pros and Cons
  of the Options** (and the inline GitHub discussion/issue links) — are **dropped** (the adr honest
  bound). The `superseded-by` relation is preserved structurally (as the inverse of the superseder's
  `supersedes`); no foreign decision content is silently lost.
- **spec**: **11/11** normative clauses preserved as criteria with their RFC-2119 wording; the
  Summary and Introduction map to goal/context. The **BNF grammar**, **"Why Use Semantic
  Versioning?"**, **FAQ**, **About**, and **License** sections are **dropped** (the spec honest
  bound — no schema home). `maps-to-test` is correctly **absent** (optional anchor; a migrated spec
  that names no test fabricates none).
- **prd**: all 4 requirements + vision + context preserved (synthetic source, nothing to drop).

### (d) agent-call count — the batch payoff, generalized

Each file authored in **ONE** `jigc doc author --from` payload. The **per-leaf-equivalent**
authoring round-trips (create + one `set-field` per header field + one `add-item`/`set-slot` per
repeatable item + one `set-slot` per fixed slot), counted from the lowered plan:

| arm | per-leaf authoring round-trips | batch author calls | cut |
|---|---|---|---|
| adr-number (target, no edge) | create + 2 field + 3 slot = **6** | 1 | 6× → 1× |
| adr-slug (superseder, +edge) | create + 3 field + 3 slot = **7** | 1 | 7× → 1× |
| adr-gitflow (target, no edge) | create + 2 field + 3 slot = **6** | 1 | 6× → 1× |
| adr-ghflow (superseder, +edge) | create + 3 field + 3 slot = **7** | 1 | 7× → 1× |
| spec-semver (11 criteria) | create + 2 slot + 11×(add-item + set-slot) = **25** | 1 | 25× → 1× |
| prd-focusflow (4 requirements) | create + 2 slot + 4×(add-item + set-slot) = **11** | 1 | 11× → 1× |
| **total** | **62** | **6** | **62× → 6×** |

The full agent protocol per file is **4** CLI round-trips (`migrate` · `doc author` · bare
`finalize` review · `finalize --approve`) → **24** for the whole corpus. The batch is what keeps the
*authoring* portion flat (one call per doc) as the repeatable item count grows — the larger the spec
/ prd, the larger the cut.

## The reds — where the live corpus carries them

The corpus is a real **happy-path** migration, so it carries the reds it can carry; the rest are
binary-proven by `flow27_marquee` (T1) on the same shapes and noted so the completion audit can't
charge them:

- **Red 6 — human-reject byte-safe**: **carried, every file.** Each bare `finalize` (no `--approve`)
  blocked at the review gate (exit 4), leaving the foreign original byte-intact and committing
  nothing; only the explicit `--approve` retired + adopted.
- **Red 2 — ordering-contract**: the **positive arm carried** — files 2 and 4 resolve their forward
  `supersedes` against the committed target (edge integrity across the set). The **dangle block**
  (superseding a not-yet-migrated sibling) is T1's.
- **Red 3 — doc-level date-suppression**: **not carried** — every log4brains ADR is **dated**, so
  this corpus exercises the **date-survival** path instead (all four transcribed dates render
  verbatim: `2020-09-26`, `2020-10-16`, `2024-09-26`, `2024-12-17`). The dateless-renders-no-date
  arm is T1's + `migrate_adr.rs`'s.
- **Red 1 (write-time ref-shape reject)**, **Red 4 (finalize-promote clobber guard)**, **Red 5
  (out-of-set supersedes dropped to prose)**: **not carried** by this in-set, distinct-slug,
  well-formed corpus; each is binary-proven by T1.

## Bounds (carried in honestly)

- **adr accepted-drops**: Considered Options · Decision Drivers · Pros and Cons of the Options ·
  generic/discussion links have no schema home and drop to prose loss (only `supersedes` is a
  structural edge). The foreign `Deprecated`/`superseded by` status maps onto the 3-enum.
- **Slug-in-the-H1 (accepted pre-existing bound)**: every migrated doc renders its H1 as the slug —
  `# use-the-adr-slug-as-its-unique-id`, `# semantic-versioning`, `# focusflow` — not the human
  title. A faithful lowercased-kebab of the title, a cosmetic degradation, not data loss; the
  title→H1 round-trip is its own keyed concern, out of a migration milestone's scope.
- **prd is synthesized + labeled synthetic**: prd has no real foreign-corpus standard, so its
  acceptance arm is a fabricated brief (`evidence/synthetic-prd-source.md`, labeled in-file). The
  measured-on-real-corpus claim is honestly weaker for prd than for adr/spec.
- **Off-canonical foreign paths only**: the ADRs sit at `docs/adr/*.md` and the spec at the repo
  root — off-canonical, so the in-location-squatter case is not exercised here (it is T1's). No
  title-slug collision in this corpus (every title slugs distinctly), so the finalize-promote
  clobber guard is not re-exercised here.
- **spec is the best-fit doctype**: `maps-to-test` optional (none authored, finalize clean); a
  foreign "decided by ADR-X" reference would migrate as prose (no `decided-by` edge — deferred).
- **n = 6 files across 3 doctypes (4 real ADRs + 1 real spec + 1 synthetic prd)** — a done-bar
  shape check, not a distribution. Metrics (b)/(c) agent-judged, the corpus agent-reproduced; only
  (a) CLI-reported. The throwaway clones/scratch repos are detached and discarded; the real repos
  under `~/Projects` and the public clones under `/tmp` are untouched.

## Verdict — done-bar met

The generalized migration transform arm works end-to-end against the rebuilt + re-pinned HEAD binary
on a **real cross-referencing ADR corpus (two supersession chains), a real idiosyncratic spec, and a
synthesized-labeled prd**, each migrated **one file at a time**: **6/6 round-trip-conformant** (parse
+ conformance-gate + adopt, byte-stable per T1, ingest idempotent), **6/6 fidelity-accepted**,
**content preserved within the accepted-drops bound** (11/11 spec clauses, all ADR
context/decision/consequences, all prd requirements), and the **authoring agent-call count cut 62× →
6×** via the declarative batch. The two `supersedes` edges resolved across the migrated set under the
dependency-ordering contract; dated ADRs kept their dates; the review gate blocked then approved;
each migration committed only its own promote+retire set + jigc's tracked config with the
auto-provisioned message. The honest bounds (accepted-drops, slug-in-H1, synthesized prd,
off-canonical paths) hold; Framing A intact — the agent proposed prose, the CLI strict-parsed +
placed every structural act. **The M25 done-bar is met.**
