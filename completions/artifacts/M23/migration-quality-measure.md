# M23 — migration-quality measure (the done-bar)

**Run 2026-06-15.** The measured migration-quality baseline mandated by
[auto-migration.md](../../../design/auto-migration.md) → *Migration-quality measure* and
[roadmap.md](../../../implementation/roadmap.md) → M23 Increment 4. A small corpus of
real public `CHANGELOG.md` files of varying conformance is driven end-to-end through the
re-pinned HEAD `jigc` binary's `migrate → author → review → --approve → adopt` path, scored
on the three metrics. This is the milestone's done-bar — it unblocks the Flow-B
existing-project live test.

## Binary under test

| | |
|---|---|
| `jigc` sha256 | `48448950e840527b367c457dc9a5e7c0426198bffcf1c044630076a6e67e0e1d` |
| HEAD commit | `c9033cc44d8e8213ba883c34b43c7671d67ff316` (the Inc-4 T1 marquee, clean tree) |
| Pinned to | `~/.cargo/bin/jigc` **and** `~/.local/bin/jigc` (`cargo install --path crates/cli --force` + copy to both bin dirs — the roadmap "before any exercise" re-pin) |
| Invocation | `jigc` from `PATH` (not `CARGO_BIN_EXE`), with the **embedded** dev pack (no `JIGC_PACK_DIR`) |
| Driver | [`drive.sh`](evidence/) reproduced below in [evidence/](evidence/) — one `git init` temp repo per file |

## Honest posture — protocol-counted, not engine-emitted

Per [measurement.md](../../../design/measurement.md) and the settled M23 Settle fork #4
(DECISIONS 2026-06-14; M17 "transcription is agent-performed and auditable"):

- **Framing A — the agent IS the rewriter.** The driver script stands in for the coding
  agent: it reads the staged foreign source the CLI surfaces and authors the canonical doc
  **through the write verbs** (`create` / `add-item` / `set-field` / `set-slot`). It places
  nothing; the CLI owns every placement. The foreign-category → enum mapping below is the
  agent's editorial judgment, recorded inline.
- **Only metric (a) is CLI-reported.** Round-trip conformance is the binary's own yes/no
  (does the rewrite parse + conformance-gate + adopt?). Metrics (b) fidelity-acceptance and
  (c) content-preservation are **agent-judged** and the corpus is **agent-reproduced** —
  counted by this protocol, not emitted by the engine. They are recorded, not hidden.
- This is an **owner-artifact recording**, not a TDD red→green test. Its done-criterion is
  this committed artifact. The path it exercises is independently proven by the binary e2e
  `crates/cli/tests/flow25_marquee.rs` (Inc-4 T1).

## Corpus — 3–5 real public CHANGELOG.md files of varying conformance

Reproduced excerpts of real public files (representative of each project's real shape; the
measure is about conformance-shape, not exact upstream bytes). Sources in [corpus/](corpus/).

| slug | project (real source) | conformance profile | foreign git-blob |
|---|---|---|---|
| `kac` | `olivierlacan/keep-a-changelog` `CHANGELOG.md` | **clean Keep-a-Changelog** (+ multi-release) | `dc150111` |
| `express` | `expressjs/express` `History.md` | **loosely-structured** (setext version/date headers, flat bullets, no categories) | `4765bd88` |
| `axios` | `axios/axios` `CHANGELOG.md` | **non-KaC categories** (`Bug Fixes` / `Features` / `Performance Improvements`) | `1d30e5eb` |
| `commander` | `tj/commander.js` `CHANGELOG.md` | **multi-release** (5 releases, incl. `Deprecated`) | `b0300894` |

All four profiles named in the roadmap (clean KaC · loosely-structured · non-KaC-categories ·
multi-release) are covered.

## Scores

| slug | (a) round-trip conformance (CLI) | (b) fidelity-acceptance (review gate) | (c) content-preservation spot-check |
|---|---|---|---|
| `kac` | **YES** — block→approve→adopt; ingest `adoptable → adopted` | **accepted** — diff faithful; only accepted drops lost | **clean** — 12/12 change bullets; per-release compare-links preserved via `link` field |
| `express` | **YES** — block→approve→adopt; ingest `adoptable → adopted` | **accepted** — agent-assigned categories faithful | **clean** — all content preserved; 1 nested sub-bullet flattened inline (not dropped) |
| `axios` | **YES** — block→approve→adopt; ingest `adoptable → adopted` | **accepted** — `Performance Improvements` remap noted inline | **clean** — 5/5 change bullets; compare-links preserved; category-label semantics partially lost on the remap |
| `commander` | **YES** — block→approve→adopt; ingest `adoptable → adopted` | **accepted** — all 5 categories incl. `Deprecated` map | **clean** — 12/12 change bullets across 5 releases |

Every file: `finalize` without `--approve` blocked at the review gate (exit **4**, fidelity
diff rendered, foreign byte-intact, nothing adopted); `finalize --approve` landed (exit
**0**) writing `changelog/changelog.md`, retiring the foreign original (`D CHANGELOG.md` +
`A changelog/changelog.md` in the **same** commit), and a follow-up `jigc ingest` reported
`adoptable → adopted`. Historical release dates survived (the on-create today-stamp was
overwritten with the foreign date). No `needs-reconcile` in any run.

### Metric (a) — round-trip conformance: 4/4

The headline. **Every** profile — including the loosely-structured (`express`, no foreign
categories) and the non-KaC-categories (`axios`) ones — round-tripped, because Framing A lets
the agent map foreign categories into the `category` enum (`added/changed/deprecated/removed/
fixed/security`) *before* the strict parser ever runs. The CLI never fuzzy-maps; it strict-
parses what the agent authored and adopts iff conformant. The enum is genuinely enforced: the
marquee RED-1 (`Performance` as a raw change-group) blocks at
`schema-conformance.field-value-conformant` — proven in `flow25_marquee.rs`.

### Metric (b) — fidelity-acceptance: 4/4 accepted (agent-judged)

For each file the review gate rendered the fidelity diff (foreign source minus / canonical
rewrite plus) and the agent reviewer accepted it. The non-trivial judgments:

- `express`: foreign bullets carry no category — the agent assigned `Added/Changed/Removed/
  Fixed` by reading each bullet's verb. An editorial call the diff makes visible.
- `axios`: `Performance Improvements` has no enum member — remapped to `Changed` with the
  original category recorded inline in the note (`(foreign category: Performance
  Improvements)`), so the label semantics are preserved in prose even though the structural
  category is `changed`.

### Metric (c) — content-preservation spot-check: clean within the accepted bounds

The accepted-drops bound (auto-migration.md): **KaC preamble · per-release prose summary ·
`[Unreleased]` compare-link**. Observed losses, all within bound:

| slug | accepted drops observed | beyond-bound loss? |
|---|---|---|
| `kac` | preamble ("All notable changes…", the format/semver note); empty `[Unreleased]` section + its compare-link | none — all 12 change bullets preserved; the two per-release compare-links preserved via the `link` field |
| `express` | none (no preamble, setext headers) | none — all content preserved; the one nested sub-bullet `- deps: qs@6.11.0` was **flattened inline** into its parent's parenthetical (KaC change-groups are flat prose, so nesting collapses — folded, not dropped) |
| `axios` | inline compare-link in the `## [x]` heading is re-homed to the `link` field | none — all 5 bullets preserved; `Performance Improvements` label semantics partially carried in prose |
| `commander` | preamble ("All notable changes…") | none — all 12 bullets across 5 releases preserved, incl. `Deprecated` |

No **silent** loss beyond the accepted-drops bound in any file. The one structural change worth
naming — `express`'s nested dependency sub-bullet flattening — is honest (the content string
survives in the parent bullet; only the nesting is lost, and the schema has no nested-bullet
shape under a change-group).

## Bounds (carried in honestly)

- **n = 1 per profile, 4 files.** A coverage check across conformance shapes, not a
  distribution. Reproduced excerpts, not full upstream files.
- **Metrics (b)/(c) are agent-judged, the corpus agent-reproduced** — protocol-counted, the
  settled posture. Only (a) is CLI-reported.
- **Changelog only.** Location-bearing doctypes (`adr`/`spec`/`prd`/`arch-doc`) and
  code-inferred docs generalize at M24.
- **No slug-collision case in this corpus** — every version string is distinct, so the
  accept-and-block collision guard (`flow25`/`migrate_retire_adopt` reds) is not re-exercised
  here; it is proven at the test level.
- **Harness note (not material to the metrics):** the driver did not `git commit` the `jigc
  setup` output before migrating, so each finalize commit also carries the setup files
  alongside `D CHANGELOG.md` + `A changelog/changelog.md`. The retire+adopt atomicity (both in
  the one finalize commit) is unaffected.

## Verdict — done-bar met

The migration transform arm works end-to-end against the rebuilt HEAD binary across all four
conformance profiles: **4/4 round-trip-conformant**, **4/4 fidelity-accepted**, **content
preserved within the accepted-drops bound** in every file. The determinism boundary held
throughout — the agent authored prose through the write verbs, the CLI owned every placement
and strict-parsed/gated/adopted. The measured baseline exists; the Flow-B existing-project
live test is unblocked.
