# rc.7 rerun — findings verification (2026-07-20)

Every load-bearing claim from the [trial feedback](trial-record.md) verified against the rc.7 code at HEAD `7e7d8c9` (which is the shipped rc.7 surface) and the installed binary (`jigc 1.0.0-rc.7`), by four independent read-only verification passes. Verdicts: **4 refuted/reclassified** (the capability exists — the discoverability evidence), **4 defects confirmed**, **4 discoverability gaps confirmed at the surface**, **1 log-discovered corollary**.

## Refuted / reclassified — the capability exists

### R1 — "`--format json` is still prose-in-a-box; no structured task id" → REFUTED

`jigc migrate --format json` emits the pinned `{task, text}` projection with the minted id as a structured field. `migrate::run` renders through the identical `render::composed` path `jigc start` uses (`crates/cli/src/migrate.rs:47-64` → `crates/cli/src/render.rs:149-153`), and the migrate composition sets the id explicitly (`crates/cli/src/start.rs:840-844`, citing command-output-contract §1; `crates/engine/src/compose.rs:2674-2686` names migrate as an id-carrying producer — "retiring the prose scrape"). The `task minted: <id>` line the agent regex-scraped is agent/human-format-only presentation (`render.rs:188-193`) — JSON never even emits it. **Residue (chartered as a papercut):** `design/command-output-contract.md` §1 names only `start`/`workflow` as composed-output producers; migrate's guarantee holds by code-path inheritance, not explicit pin.

### R2 — "`doc schema` doesn't expose field→section" → REFUTED

Exposed in both projections: the JSON `ContractField.section` populated for every top-level field (`crates/cli/src/doc.rs:2252`, `:2339`; contract-version 3 at `:2367`) and the agent text listing's `(section: <id>)` suffix (`doc.rs:2474-2482`, the M42 papercut fix). The agent's subagent read the *pack source tree*, not the binary — the stale-source hazard (D4 below).

### R3 — "singleton append undocumented" → REFUTED globally, CONFIRMED locally (the path-local half)

Behavior confirmed correct: `doc author` over a committed singleton copies the committed body in (`crates/engine/src/state.rs:891-917`, doctype-blind since M43) and repeatable payload leaves append via `apply_add_item_target` (`crates/cli/src/doc.rs:1760-1811`) — 11 + 11 = 22, nothing lost. Documented on the authoring-workflow path: `packs/methodology/steps/author-ledger.yaml:20-21` ("appends — existing entries are untouched") + the four sibling author steps + `doc author --help` (`doc.rs:168-169`). **But the migrate-path step the agent actually walked (`packs/methodology/steps/author-migration-deferral-ledger.yaml`) carries no append/copies-in statement at all** — "undocumented" was locally true underfoot. This is the path-local guidance finding (D5).

### R4 — "`jigc ingest` still emits 220 rows (your V9)" → RECLASSIFIED

The M41 per-directory collapse exists and works, but only on the agent/human render arm, gated to `verdict == "unmanaged"` (`crates/cli/src/render.rs:1541-1557`); actionable verdicts stay itemized by design, and the JSON arm is full-rows by contract (`render.rs:1535`). The log shows the agent's one real ingest was `--format json` (53,977 bytes) — **the collapse was never exercised in this trial.** Not a regression and not V9 recurring; the consumer aggregating JSON is the contract working. Residue (chartered as a papercut): an additive `summary` block (verdict counts, per-directory counts) in the ingest JSON so drivers don't re-derive the histogram.

## Confirmed defects

### C1 — task-id collision / serial reuse on bulk migrate — CONFIRMED (example overstated)

The migration task id derives from the repo-relative source path, directory-first (`crates/cli/src/start.rs:213-236`); the 5-word cap keeps the *leading* words (`crates/engine/src/slug.rs:335-336` — `.split('-').take(MAX_WORDS)`), so the directory prefix survives and the filename is what truncation discards; `mint_task` hard-rejects an active-task collision, never suffixes (`crates/engine/src/state.rs:494-498`, `task.serial-collision`); and `--slug` on migrate drives only the *document* slug — it is never passed to the task mint (`crates/cli/src/migrate.rs:212`, `start.rs:192` passes `None`). The M41/M42 slug work (word-boundary retreat, `SLUG_RULE_VERSION`) addressed slug quality and versioning, not collision — the rc.4 log's 13-way collision was observed data never chartered. **Overstatements:** "every file in `.planning/_migration/` mints the identical id" is false (that shallow directory contributes only 2 of 5 words; files differing in their first 3 filename words get distinct ids), and the quoted example id has the word order reversed vs the directory-first fold.

### C2 — the `write.not-present` route dead end — CONFIRMED

`write.not-present` shares the shape-question route arm with `unknown-field`/`unknown-section`/`wrong-shape` (`crates/engine/src/write.rs:5071-5077`) and routes to `jigc doc schema <doctype>` — the declared-shape read, which structurally cannot reveal a *minted item id* (the trial's `v1-0`). The route that survives being followed is the M43 staged read `jigc doc show <doc> --task <id>`. Fired exactly once in the log (09:27:34, the nested changelog address). Same class as the lacon V8/V13 route repairs; the fix is splitting `not-present` out of the shape arm.

### C3 — fidelity-scan version-token false positives — CONFIRMED

`scan_version_tokens` (`crates/cli/src/render.rs:2196-2215`) extracts maximal dotted-numeric runs bounded only by digits and `.` — any letter or dash merely resets the scan — so `project-alpha-2.0` yields `2.0` and `v1.0` yields `1.0`, with no check that the token is a substring of a longer path/identifier; matching is exact set-membership (`render.rs:2171-2188`). The described ~50% false-positive rate is mechanically plausible. Mitigating: the summary self-labels heuristic/advisory and feeds no gate (`render.rs:2142`, `:2165-2170`) — but it is the surface the operator checked every time, and noise trains skimming on the trial's most-praised trust builder. Fix shape per the feedback: boundary-guard the extraction; split "package@version absent" from "version-like token absent".

### C4 — no from-knowledge authoring path for `adr` — CONFIRMED for adr only

Full `allows-create` survey across both packs: `adr` is creatable only via `migrate-adr` and the code-implementing loops (`implement-from-spec`, `single-task`, `sub-task`) — no standalone authoring workflow. **`spec` and `arch-doc` have paths the claim missed:** `plan.yaml` authors a spec from knowledge; `architecture-documentation.yaml` authors arch-docs (the agent admits skipping it). The sharp edge is the **placeholder-source loophole** the gap invited: 11 docs migrated from one-byte sources, the fidelity diff comparing authored prose against the literal `p` — a guarantee degraded to zero with no objection. Candidate guards: the adr authoring path itself, plus a triviality threshold on the migrate source.

## Confirmed discoverability gaps (the pull tier)

### D1 — AGENT.md carries zero machine-output facts

The generated bootstrap (`crates/cli/src/adapter.rs:466`, `bootstrap_file()`) is four paragraphs — routing sentence (`:1026`), read rule (`:1040`), framing (`:1051`), output contract (`:1067`). The output-contract paragraph covers exit codes only; `--format json` and `.task` appear nowhere in the one surface preloaded before the model plans.

### D2 — the read rule licenses the stale-source trap

`BOOTSTRAP_READ_RULE` ends: "Everything else — source, tests, any file not in that set — you read freely." A checked-out jigc/pack source tree falls under "read freely," and no surface warns that it diverges from the installed binary. The agent followed the rule as written and nearly designed around two already-fixed bugs (V1 folding, the D/I enum).

### D3 — no mutation-free preview of a `creates-task: true` workflow's step text

`jigc describe` is a whole-menu projection — per-workflow `when`/`description`/`usage` one-liners, no step text (`crates/cli/src/describe.rs`); bare `jigc start` orients read-only but composes nothing. The composed instructions of a work-minting workflow are unreadable without minting a task. This is the verified reason a mutation-cautious agent went to the source tree for `ingest-existing.yaml` — the one *capability* gap in the discoverability cluster.

### D4 — the stale-source hazard (second-order, genuinely new)

`~/Projects/gherrink-jigc` is neither a managed doc nor the target project's source, and it diverges from the installed binary by construction. Reading it is the natural move for payload shapes, and nothing anywhere says "derive tool behavior from the binary." Confirmed live twice in this trial (R2's false negative; the near-miss on V1/V3).

### D5 — path-local guidance

The composed output for a workflow does not necessarily carry the guidance its own path needs: the append-semantics statement exists on every authoring-path step but on none of the migrate-path author steps (R3). Generalization chartered: a step that authors into a possibly-existing singleton states the copy-in/append semantics *on that step* — checkable via the M43 `states-constraints:` stated-at mechanism rather than template diligence.

## Log-discovered

### L1 — serial task-id reuse makes the invocation log ambiguous

The hard-reject (C1) only blocks *concurrent* duplicates; a completed task frees its id, so bulk same-directory migration serially reuses one identical id — the log's 18-invocation `migrate-adr-planning-migration-adr` finalize burst (14:33:41–55) cannot be attributed to individual ADRs. The task id silently fails as a log-analysis key under exactly the bulk usage the migration story depends on. Folds into C1's fix shape (any of suffixing / filename-weighted derivation / an id override restores per-doc identity in the log).
