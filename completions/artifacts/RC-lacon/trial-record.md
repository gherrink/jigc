# RC trial — combined migration + implementation on rc.6 — gherrink-lacon (2026-07-15)

**Status: RC input — the third adoption corpus, and the first trial to run both halves (migration *and* implementation) in one arc on one repo.** A full GSD→jigc corpus migration of `~/ideas/gherrink-lacon` (**lacon** — the bash-output token-filter CLI: a Rust workspace at its own v1.0 close, doc-heavy — 14 ADRs, 4 specs, vision/arch-doc/prd/roadmap/deferral-ledger/completion-record) followed by four blind, handover-driven task sessions on the migrated corpus, all on the installed **1.0.0-rc.6**. This record archives the provenance, the invocation-log analysis, the task handovers (the probe designs), and the executing agents' verbatim feedback; the adversarial verification of every claim against the rc.6 code lives in [findings-verification.md](findings-verification.md).

## Provenance

- **Repo:** `~/ideas/gherrink-lacon` — a real, GSD-managed Rust project (377 commits at trial end; the pre-trial history is lacon's own v1.0 development).
- **Binary:** jigc **1.0.0-rc.6** (installed release build) for the entire run — the invocation log confirms 100% of records on `1.0.0-rc.6`, no stragglers.
- **Method:** the migration half ran as ~26 `jigc migrate` tasks (24 finalized adoptions — 14 adr · 4 spec · vision · arch-doc · prd · deferral-ledger · roadmap · completion-record) plus link-repair and cleanup; the implementation half ran as **four handover-driven sessions** (quick-fix · dev-task · decided-task · milestone — the handovers archived below are the probe designs), each executing agent working blind and asked for candid feedback afterward.
- **Invocation log:** ON throughout — `.jigc/logs/invocations.jsonl`, **294 records** spanning 2026-07-15 03:40–18:53 UTC (~15 h wall).
- **Commits:** 38 in the trial window; **35 went through `task finalize`** (24 adoptions + 11 post-migration tasks), 3 direct-git (see the adherence note in the log analysis).

**Headline outcomes, one line each** (detail + evidence in the log analysis and [findings-verification.md](findings-verification.md)):

- **Zero correctness bugs across all five feedback reports** — "Every command exited 0 and did exactly what it said"; the migration tester's net: "I'd reach for it again for exactly this kind of work."
- **The verified through-line is discoverability, not absence:** most of what the testers called missing **exists** on rc.6 — the single-active-task `--task` default, `jigc start --task <id>` as the resume verb, `task validate` as the what's-left list, the `#type` enum on three surfaces, `doc schema`, `doc list` as the managed-set discriminator — but no composed output or bootstrap surface names them. The testers could use every command they were handed and none they weren't.
- **The strongest new defect:** `decided-task` — the exact workflow Task 3 declared structurally missing — has shipped in the methodology pack since 2026-06-13 but carries `selectable: false`, whose recorded rationale ("off-router until the router-flip is built") expired when the router shipped and was never re-evaluated.
- **Identity friction is real but small and front-loaded:** the log puts the task-id/slug fumbles at 5 failed invocations, all in the migration phase, all recovered in ≤40 s — but the *slug drift* they caused forced a ~40-inbound-link repair phase, because `jigc migrate` (unlike `start`/`doc create`) still has no `--slug`.
- **The same-path migration finalize contract revealed itself in stages** (clobber-block → `git rm` → *then* `--approve`), costing 4 attempts on the first adoption — and the recorded decisions-pending trigger for exactly this ("the existing-project RC trial hits it live") has now fired.
- **The onboarding framing finding:** bringing an existing repo under management is simultaneously jigc's roughest path and its most common first contact — 30 sequential human-gated commits plus a manual link sweep is the sum of N single migrations, not a first-class flow.
- **The reference-doctype gap confirmed on a third independent corpus, three ways** — four docs left unmanaged, spec examples bent into the `context` slot as a workaround, and (log-found) a benchmarks doc committed entirely outside jigc because no doctype fit.

## Invocation-log analysis

**Source:** `.jigc/logs/invocations.jsonl` — schema per record: `timestamp, argv, exit_code, duration_ms, finding_codes, output_bytes, binary_version, error_code`.

### Totals & provenance

**294 invocations, 100% on `1.0.0-rc.6`**, spanning **2026-07-15 03:40:13Z → 18:53:03Z** (~15 h 13 m wall). The log carries no session field, but a >20-min-gap clustering yields **four sessions**: **A** recs 1–177 (03:40–04:43 — orient + the full migration + two quick-fixes), **B** recs 178–189 (06:07–06:08 — one quick-fix), **C** recs 190–232 (16:30–17:13 — four flat tasks), **D** recs 233–294 (18:08–18:53 — the planning workflow + three increments). Git corroboration is exact: **38 commits** in the trial window, **35 = the 35 successful `task finalize`s**, 3 direct-git (see Adherence). Tool overhead is negligible: median **7 ms**, p95 211 ms, max 658 ms; 1.16 MB total output across all 294 calls.

### Command distribution

`validate` 45 · `task finalize` 40 (35 ok / 5 fail) · `doc set-slot` 36 (33 ok) · `migrate` 32 (28 ok) · `doc author` 25 (+1 `--help`) · `doc set-field` 25 · `start` 23 (incl. 3 bare orients + 6 router calls) · `doc schema` 9 · `doc show` 9 · `doc list` 6 · `task diff` 6 · `doc add-item` 4 · `ingest` 1 (+2 `--help`) · `task list` 3 · `task validate` 3 · `task discard` 3 · `doc create` 3 · `describe` 2 · `doc retitle-item` 2 · `upgrade` 1 · 17 help/version probes (recs 4–11 are a single up-front help sweep; the rest are point lookups after a fumble).

**Read surfaces:** all four were exercised, thinly and lopsidedly. `doc show` 9× — but only in sessions C/D, **zero during migration** despite 25 `doc author`s (consistent with the tester's admission of reading managed files directly). `doc schema` 9× — **all in the migration phase**, incl. a programmatic 8-doctype sweep in one second (recs 112–119); never touched again. `doc list` 6× (one returned **0 bytes** — see Surprises). `describe` 2×, orientation only. **Never used:** `rename`, `migrate-corpus`, `set-field --unset`, `config set`, and — notably — **any `milestone` verb** (below).

### Exit-code analysis

**33 non-zero (11.2%)**: 1×24 · 2×5 · 3×2 · 4×2. But 15 of the exit-1s are `validate --format json` during the migration — every one carrying `schema-conformance.schema-version-current`/`unadopted-instance`, i.e. **the M42 unmigrated-corpus non-zero doing its designed job** over a partially-adopted store (the same sweep goes exit-0 with findings once each doc lands, e.g. rec 122). Excluding those, **real friction is 18 failures = 6.1%**, clustering as:

1. **The finalize `--approve` two-round-trip — confirmed, cost 42 s and 4 attempts on the first migration task.** Rec 32 `task finalize migrate-adr-docs-decisions-0001` → **exit 3** `finalize.promote-clobber` (the promote target = the still-present GSD source file) → source `git rm`'d (unlogged) → rec 33 **exit 4** (`EXIT_REVIEW_PENDING`, the migration fidelity hold) → rec 34 `task list` → rec 35 **identical retry, exit 4 again** → rec 36 validate → rec 37 `--approve` **ok** (04:12:08→04:12:50). Task 2 still cost one extra attempt (rec 40: `--approve` present but exit 3 promote-clobber — the git-rm not yet done; rec 42 ok). From task 3 onward: **single-shot `finalize --approve`, 22 consecutive clean**. Overall 26/40 finalizes carried `--approve`. Note the exit-4 records log `finding_codes: []` (4,167 bytes of output, no codes) — the same instrumentation gap the rc.5 analysis flagged for blocked finalizes.

2. **Task-id truncation fumbles — 2 distinct wrong ids, 3 failed invocations, both slug-word-cap mispredictions.** Rec 141/142: `migrate-roadmap-docs-bundled-rules-roadmap` guessed (author + finalize both exit 1 in the same second); the minted id was `migrate-roadmap-docs-bundled-rules` (rec 143, +30 s). Rec 168: `link-specs-to-v1-scope-prd` guessed; minted `link-specs-to-v1-scope` (rec 169, +6 s). Both errors are the same shape: **the agent re-derived the id from the intent and kept one word past the cap**. Rec 141/142 is the reported git-rm-before-author ordering mishap: the source file was already staged for removal when both the author and the finalize bounced off the phantom id — the window where a wrong id + a pre-staged `git rm` coexist is exactly the state M40 F7 hardened finalize against.

3. **Guessed-but-nonexistent surfaces — 3 of the 4 reported guesses are in the log, each self-recovered in <15 s via `--help`.** Rec 210 `doc slots roadmap:roadmap` (exit 2) → `doc --help` → `doc show --format json` (rec 212). Rec 237 `doc show … --task <id>` (exit 2 — task-scoped read guessed) → bare `doc show` (rec 238). Rec 254 `task status <id>` (exit 2) → `task --help` + `doc --help` → `task diff` (rec 257). **`doc show --slots` never appears** — no argv contains `--slots`; the tester's recollection likely conflates it with the `doc slots` guess.

4. **`set-slot` payload grammar — 2 clap usage errors** (recs 26/27: first-ever set-slots, `--from-file -` missing; corrected +9 s) and **exactly 1 `write.slot-heading-depth` rejection** (rec 179, corrected +29 s at rec 180 — singular, not plural as the feedback implied).

5. **Migrate re-run churn — 4 exit-1 re-invocations + 3 `task discard`** (see Surprises).

6. Rec 285 `doc show changelog:changelog` → `store.not-found` (a genuine no-such-doc probe, routed).

### Phase structure

The migration phase is sharply visible, and it is smaller than remembered: **28 successful `migrate` invocations minting ~26 tasks → 24 adoption finalizes** (matching the 24 `docs(<type>): adopt …` commits one-to-one), not ~30.

| Phase | Recs | Span (UTC) | n | finalize-ok |
|---|---|---|---|---|
| Orient/setup (help sweep, describe, ingest) | 1–19 | 03:40–03:49 | 19 | 0 |
| **Migration** (migrate→author→validate→finalize --approve loop) | 20–156 | 04:08–04:35 | 137 | **24** |
| Quick-fix: stale arch-doc path | 157–166 | 04:40–04:41 | 10 | 1 |
| Quick-fix: link specs → prd | 167–177 | 04:43 | 11 | 1 |
| Quick-fix: restore spec examples | 178–189 | 06:07–06:08 | 12 | 1 |
| Quick-fix: flaky test | 190–196 | 16:30–16:36 | 7 | 1 |
| dev-task: pip-install rule | 197–206 | 16:45–16:59 | 10 | 1 |
| single-task: roadmap reclassify | 207–223 | 17:01–17:03 | 17 | 1 |
| dev-task: LACON_TOOL_USE_ID | 224–232 | 17:05–17:13 | 9 | 1 |
| **planning + 3 increments** (Tier-2 milestone) | 233–294 | 18:08–18:53 | 62 | **4** |

The steady-state cadence after migration is strikingly clean: each build task is `start` (router → workflow) → work → `set-field type/scope` → `set-slot summary/body` → `validate` → `finalize`, **6–9 jigc calls per task, zero failures** in six of the eight post-migration tasks.

### Fumble/retry & identity friction, quantified

**Total identity-friction cost: 5 failed invocations (3 wrong-task-id + 2 unknown-command probes at a task), all recovered in ≤40 s, none recurring after correction.** Consecutive-identical-argv retries: 8 pairs, of which only recs 33&35 (finalize exit 4 → identical exit 4 — retrying without changing anything) and 179→180 (heading-depth fix) are error→correction; the rest are benign re-reads or the migrate double-fire pattern below. The router two-step (`start "<intent>"` → guidance → `start --workflow X "<intent>"`, 6×, 1,214 bytes each) worked every time — no workflow-selection fumbles. Post-migration, the failure rate in the four build sessions was **4 of 105 invocations (3.8%)**, all four being read-surface guesses — the write path was flawless there. The identity friction the tester remembers is real but concentrated in ~90 seconds of a 15-hour trial.

### Adherence signal

**35 of 38 trial-window commits went through `task finalize`.** The 3 direct-git commits: `e37b45b` (.planning tree removal — unmanaged files, expected), `8c093fe` (repoint inbound links across CLAUDE.md/README/code comments — unmanaged surfaces), and **`11d07ea` — `docs/benchmarks.md` + a README section, a prose docs commit made entirely outside jigc at 06:08, immediately after a finalize, with no task minted**: the cold-start benchmark tables had no fitting doctype, so the work routed around the tool — a clean doctype-gap data point (same lens as the 2026-07-10 parked batch). On reads: the migration authored 25 docs with zero `doc show` and the four `--help`-driven recoveries all reached for a read surface *after* guessing, which corroborates the tester's stated habit of reading managed files directly — the log can't see those reads, but the near-absence of logged ones over 15 hours is the shadow they cast. Standing input for the [managed-doc-enforcement-hook](../../../ideas/managed-doc-enforcement-hook.md) trigger and the read-path-must-be-the-path-of-least-resistance cluster.

### Surprises the feedback did not mention

1. **The migrate output is one-shot, and the agent paid for it 7 times.** Successful `migrate` → immediate exit-1 re-invocation ("task already in flight"-shaped) at recs 91→92, 124→125, 146→147, 149→150; twice the agent then ran `task discard` + re-`migrate` purely to re-print the rewrite instructions (arch-doc: 126–127; completion-record ran the full loop **twice**, recs 146–152 — 4 migrates, 2 discards for one document). There is no "re-show the in-flight migration instructions" path; discard-and-remint was the only recovery. *(Note `start --task <id>` re-composes an in-flight task — whether it re-prints the migrate author-template was not exercised; verify at fix time.)*
2. **`doc create` on an already-existing singleton exits 0 and prints the existing id.** Rec 236 `doc create roadmap` (existed since the migration) and rec 242 `doc create deferral-ledger` (existed) both silently acked; rec 247 `decisions-log` genuinely created one. Get-or-create may be intended, but nothing in the ack distinguishes "created" from "already there".
3. **`doc list <doctype>` with zero instances prints 0 bytes** (rec 284) — empty stdout, no "no instances" line; the agent had to fire a `doc show` (rec 285, exit 1) to learn the changelog didn't exist.
4. **The milestone surface was never touched.** The Tier-2 session ran the `planning` workflow, wrote the roadmap/decisions-log/deferral-ledger, then executed three increments as **flat `dev-task`/`single-task` starts** — no `milestone create`, no milestone-record, no fan-out. After rc.5's milestone arc died at first use, rc.6's first planning session simply never reached for it; whether the workflow routed away from it or the agent did is worth asking the tester. Standing datum for [sequential-milestone](../../../ideas/sequential-milestone.md).
5. **Two pre-ingest `validate` runs** (recs 1–2, exit 1, 54 KB of findings each) — the M42 `schema-version-current` + unadopted sweep fired correctly over the raw GSD corpus before any migration began; the detect+route floor's first field contact was the very first invocation of the trial.
6. **Blocked-finalize opacity persists**: the two exit-4 review holds log `finding_codes: []` against 4,167 bytes of output — the rc.5 instrumentation gap (block reasons invisible to log analysis) reproduced on rc.6.

## The migration half — verbatim feedback

Three reports from the migrating agent (the friction report, the v1-readiness lens, and two follow-up notes), unedited.

### Report 1 — the field report

> Happy to give you a candid field report — I just drove ~30 migrations through it end-to-end, so this is from actual use, not a skim.
>
> Overall
>
> It went well and largely as planned. The model is opinionated in the right way: it kept forcing the question "what doctype is this actually?", which is exactly what a migration needs. Nothing derailed. The deviations from plan were all discovery, not breakage — e.g. realizing the reference specs were a looser fit for the spec doctype, and that the milestone audit was outdated and needed a corrected verdict.
>
> What genuinely worked
>
> - The migrate flow is excellent. Staging the foreign source read-only + handing me a precise author-template with the mapping guidance inline ("map the foreign Decision heading to the decision slot", the status-enum mapping) meant I almost never had to guess. That inline guidance is the standout feature.
> - The --approve fidelity gate (foreign-vs-canonical diff before commit) is the right safety design. It caught nothing bad here, but it's the correct guardrail for LLM-authored rewrites.
> - Code-anchored arch-docs (implemented-by: path#symbol, validated against the tree) and jigc validate's graph checks (the derived-from inverse-cardinality advisory) are real value — docs that can't silently drift from code.
> - The owner-artifact "ladder" for the completion-record (point-at-existing / copy-under-home / source-is-the-artifact) was genuinely well-thought-through.
> - Discoverability was good: doc schema <type>, describe, doc list were all consistent and gave me what I needed without spelunking.
>
> Friction I actually hit
>
> 1. Task-id truncation was the #1 pain. IDs are derived-and-truncated unpredictably: migrate-roadmap-docs-bundled-rules-roadmap became …-bundled-rules; link-specs-to-v1-scope-prd became …-v1-scope. The mint output does print the real id, but any time I constructed it myself I got it wrong — and once that caused a git rm-before-author ordering mishap. This bit me 2–3 times.
> 2. Slug drift forced an entire link-repair phase. Title→slug truncates (0004: Project > User > Bundled config precedence → 0004-project-user-bundled-config, dropping "precedence"), so filenames drifted from the originals and I had to repoint ~40 inbound links. migrate/author have no --slug; only post-hoc rename --slug exists.
> 3. The migrate template understated the schema. For ADRs it presented a "fixed four-part schema" (status/context/decision/consequences), but the real doctype has an ## Options section too — I only found it by inspecting the staged diff (it rendered as an empty heading). The template should show every section, including optional ones.
> 4. The finalize contract revealed itself in stages. First finalize → promote-clobber; I git rm'd the foreign file; then it told me it needs --approve. Two round-trips to learn the rule for same-path migrations.
> 5. No inbound-reference report for moves. jigc repoints managed→managed edges, but prose mentions inside managed docs and all plain-doc/code links are on me. Expected — but for onboarding an existing repo, a "what references this doc?" helper would remove most of Phase D.
>
> (One trip-up was mine, not jigc's: I assumed $FILES word-splits in zsh for a bulk sed. Not your problem.)
>
> Top suggestions, prioritized
>
> 1. Make task IDs predictable or don't require them verbatim — accept a path/doc selector, or stop truncating silently. Biggest practical win.
> 2. --slug on migrate/author so you can preserve identity when onboarding a repo with cross-referenced filenames. This alone would have deleted my entire link-repair phase.
> 3. Migrate template = full resolved schema, optional sections included, so nobody leaves an empty ## Options or has to discover it.
> 4. Surface --approve + the clobber/retire behavior up front in the migrate task instructions, not on the second finalize attempt. Ideally --approve retires the same-path original itself.
> 5. A jigc references <doc> helper (inbound path links across the repo) for the rename/move case.
>
> Net: I'd reach for it again for exactly this kind of work. The doctype discipline and the migrate/approve loop are strong. The rough edges are all in identity (task ids, slugs) and discovery of the finalize contract — mechanical, fixable, and not architectural.

### Report 2 — the v1-readiness lens

> Good — here's the v1-readiness lens specifically, which surfaces a few things my "friction during use" report didn't. I've deliberately left out the 5 earlier suggestions; these are additive.
>
> The framing that matters most for v1
>
> Onboarding an existing repo is simultaneously the roughest path in jigc and the most common first contact with it. My entire session was that path (ingest-existing on a real, doc-heavy repo), and it's where every rough edge lived: slug drift, task-id guessing, the 30-migrations grind, the link-repair phase. Meanwhile the steady-state path (one new task, one commit) felt clean. For v1 I'd consciously treat "bring an existing repo under management" as a distinct, first-class flow that deserves its own affordances — not the sum of N single migrations. If a new user's first hour is 30 sequential human-gated commits plus a manual link sweep, that shapes their whole impression of a tool that otherwise feels solid.
>
> New things I'd add for v1
>
> 1. finalize committing your staged git index is a quiet footgun. "One task = one commit" silently becomes "one task + whatever I happened to have staged." It does helpfully list left-out untracked files, but the inverse — "here's everything I'm about to fold into this commit" — should be just as loud, ideally with a confirmation when the index contains files the task didn't touch. This is the kind of thing that produces a "why is my ADR commit also carrying an unrelated code change?" bug report in week one.
> 2. The jigc-state ↔ git-working-tree boundary needs to be crystal clear. When I mis-typed a truncated task id, I landed in a half-state: the git rm had run but the author hadn't. I recovered by hand, but a user won't know whether jigc's staged buffer or their working tree is the source of truth. task discard exists (good), but the "who owns what, and how do I cleanly back out of a half-done migration" story should be explicit and tested for v1.
> 3. Give "maps to no doctype" a name and a blessing. Four of my docs (a primitive reference, a testing guide, a tutorial, a resolved-questions log) fit no doctype and stayed plain. ingest labels them unmanaged, which reads like a failure rather than a deliberate, fine outcome — awkward for a tool whose pitch is "manage everything." Either ship a generic reference/doc catch-all, or explicitly frame "these stay as plain files and that's correct." Right now the tool feels slightly incomplete when it's actually behaving sensibly.
> 4. Strengthen the migration fidelity check. The --approve diff is the right guardrail, but the automated part it prints ("fidelity heuristic — fuzzy, advisory, feeds no gate") is weak, and the scary failure mode of an LLM-authored migration is silently dropping a normative sentence. A content-coverage check ("these claims/bullets/enum values from the source don't appear in the rewrite") would meaningfully raise trust in the one step where trust is most fragile. For me the human reviewer caught fidelity; a tired user clicking --approve on doc #27 won't.
> 5. Test and document "someone hand-edited a managed doc in their editor." It will happen. The file-state baseline / un-baselined machinery clearly anticipates it, but the user-facing story ("I edited the file directly — what does jigc do, and how do I reconcile?") should be an explicit, tested, documented path for v1, not something users discover via a validate finding.
> 6. Finish the "every blocking error prints the exact next command" pass. Some errors nailed this (the owner-artifact ladder was superb). Others were discover-by-failure (set-slot needing --from-file -; --approve only revealed on the second finalize). It's 90% there — an audit to close the last 10% is cheap and disproportionately improves the felt quality.
>
> Blunt v1 verdict
>
> The core is v1-solid: the doctype model, the migrate/approve loop, code-anchored validation, and — importantly — the schema-version + migrate-corpus byte-stable evolution story (that's mature thinking; most tools don't plan for their own format migrations at v1). None of my findings are architectural. Every one lives at the edges — identity (task ids, slugs), onboarding-at-scale, and a couple of trust/footgun items — and all are mechanical to fix. If I were deciding, I'd gate v1 on #1 (finalize scope) and the onboarding affordance, treat #3–#6 as fast-follows, and ship. It's close.
>
> One meta-note for your writeup: weight this by sample bias — I only exercised the migration/onboarding surface hard. I never really touched milestone/planning/increment execution, so I can't speak to whether the steady-state authoring loop has its own edges. Worth getting a second report from someone who's run a milestone through it.

### Report 3 — two follow-up notes

> Two extra jigc notes for your v1 writeup (both surfaced this turn)
>
> 1. The reference-doctype gap is now confirmed three ways. The specs needed their examples put back into the context slot as a workaround because there's no examples section on spec — same root cause as the "loose fit" and the four unmanaged plain docs. A reference/guide doctype with freeform sections + examples would fix all three at once. Right now, reference-style content has no proper home and gets bent into spec or left unmanaged.
> 2. A new good guardrail I hit: slot prose rejects ##/### headings (write.slot-heading-depth, reserved for section structure) — forced me to ####. That's a correct rule; just flagging it's undocumented until you trip it, so it belongs in the "surface constraints before failure" audit I mentioned.

## The implementation half — the four task sessions

Each session was driven by a handover (the probe design — what the executing agent was given) and ended with a candid-feedback request. Handovers and feedback archived verbatim.

### Task 1 — quick-fix: flaky `cli_stats` tests

**Handover (the probe):** make three `cli_stats.rs` tests deterministic under parallel `cargo test` — a test-only change with a pre-researched isolation audit (the real hazard: an unisolated `lacon run` spawn writing the developer's real `~/.local/share/lacon/history.db`), reproduction commands, and a verify-by-property acceptance bar (every `lacon run` spawn sets `XDG_DATA_HOME` to a per-test tempdir).

**Feedback (verbatim):**

> Happy to. Caveat up front so you can weight this correctly: I only exercised one path this session — quick-fix — so I touched jigc start, doc set-field, doc set-slot, and task finalize, nothing else. I never saw planning, research, or the doc-slice assembly that AGENT.md headlines. So this is deep on a narrow slice, not a broad survey.
>
> Bugs
>
> None. Every command exited 0 and did exactly what it said. No crashes, no wrong behavior, no surprises in output. Genuinely solid on that axis.
>
> What worked well
>
> - Every command ends with the exact next command to run (Run: jigc ...). This is the single best thing about the tool. The workflow was self-documenting — I never once had to guess what came next or read docs. Keep this invariant religiously.
> - The stdin slot pattern (set-slot ... --from-file - + heredoc) is clean for multi-line prose and kept me out of managed files, as intended.
> - Auto-minted task ID from the intent string is a nice touch.
> - Finalize output (commit hash + files committed) closed the loop clearly.
>
> Friction — roughly in priority order
>
> 1. jigc start doesn't route; it makes me route. AGENT.md says start "routes among the workflows," but in practice it printed a menu and told me to re-run with --workflow. I picked quick-fix myself from one-line descriptions. That's fine as a fallback, but the framing oversells. At minimum, echo a recommended workflow based on the intent string ("intent looks like → quick-fix; re-run to confirm or override"). Right now the "context compiler" does zero routing work on the routing step.
>
> 2. The headline value ("assembles the doc slices your task needs") never appeared in quick-fix. I did all my own investigation — grepped, read three ADRs, read the specs — directly. jigc surfaced no doc context. It functioned purely as a commit-ceremony wrapper. That may be correct by design for quick-fix (small fix, no context needed) — but a first-time agent reads AGENT.md, expects assembled context, and gets a menu + a commit ritual instead. Either quick-fix should pull the relevant slice (e.g. the deferral-ledger/handover context for this task), or AGENT.md should scope the promise so the lightest tier doesn't set an expectation it won't meet.
>
> 3. --task <slug> on every subcommand is pure boilerplate. I pasted make-flaky-cli-stats-tests into three consecutive commands. There's clearly one active task in a workflow — give it a "current task" default so --task becomes an override, not a requirement. This is also an error-surface: one typo'd slug and the write lands nowhere or errors.
>
> 4. The lightest workflow has the heaviest ceremony-to-value ratio. A one-line test fix required three jigc calls after the work (set-field #type, set-slot #summary, finalize). For quick-fix specifically, consider letting finalize take --type and --summary inline (or from stdin) in a single call. The smallest change shouldn't need the most orchestration.
>
> 5. No introspection command. I only knew the exact slot addresses (commit:<task>#summary, #type) and what finalize still required because start printed them verbatim. If context had truncated, I'd have had no way to re-derive "what does finalize still need from me?" A jigc task show <id> / jigc status that lists the active task, its unfilled-but-required fields, and the next command would make the workflow resumable, not just followable. Right now discoverability lives entirely in echoed strings.
>
> 6. The finalize advisory leaks internals I'm told not to look at. It said:
> advisory · file-state.baseline-adopt — baseline adopted: docs/commit:make-flaky-cli-stats-tests.md
>   route: no action needed
> Two problems: it names a docs/commit:*.md managed file that AGENT.md explicitly forbids me from reading/editing, and "baseline-adopt" is a term I have no model for. If it's truly "no action needed," either suppress it or make it a one-line human-readable note. Surfacing a forbidden file + an internal concept together, with "ignore this," is the kind of thing that erodes an agent's confidence that it understands the tool's state.
>
> For v1, concretely
>
> 1. Current-task context so --task is optional (biggest quality-of-life win).
> 2. jigc status / task show for resumability after context loss.
> 3. Recommended-workflow hint in start, so routing isn't fully punted to the caller.
> 4. Single-call finalize for quick-fix (--type + --summary inline).
> 5. Scope the AGENT.md promise to match what the lightest tier actually does — or make quick-fix pull the relevant doc slice.
> 6. Quiet the "no action needed" advisories and stop naming forbidden managed files in them.
>
> Net: the skeleton is excellent — the always-print-the-next-command discipline makes it genuinely pleasant to drive blind, and I hit zero bugs. The gaps are about statefulness (current-task, introspection) and honest scoping (routing and doc-assembly promises that the quick-fix path doesn't deliver on). Fix those two clusters and the quick-fix path is v1-ready from where I sit — but I'd want to actually run a planning/research workflow before signing off on the parts that are supposed to carry the real context-compiler weight.

### Task 2 — dev-task: a Tier-2 bundled rule (pip-install)

**Handover (the probe):** add a bundled `pip-install` filter rule with captured real fixtures (success + failure), riding lacon's data-driven fixture harness — test-first, hermetic-CI constraints, `pkg-install.yaml` as the template, plus a follow-on roadmap-bookkeeping decision to weigh.

**Feedback (verbatim):**

> Happy to give you a candid debrief. I used jigc across two full tasks this session (the dev-task for the rule, the single-task for the roadmap), so this is grounded in what actually happened rather than a general impression. No true bugs — everything did what it said. But there were a few real friction points, one of which I'd call a v1 blocker for agent ergonomics.
>
> What worked well
>
> - The commit assembly is genuinely good. Setting #type/#scope/#summary/#body and having finalize render the Conventional-Commits message and co-commit the managed docs it owns — that's the right model. I never hand-wrote a commit message and never had to think about whether the roadmap edit and the code were in sync.
> - jigc task diff is the star. Showing the staged managed-doc content before finalize is exactly what I needed to self-verify. I used it to confirm the roadmap edit read correctly before committing.
> - The advisories are transparent, not magic. baseline-adopt and the changelog-gate note told me what jigc did and why, with a clear "no action needed" route. I trusted it.
> - The "everything through jigc" invariant held. Once I knew the roadmap was managed, the path was unambiguous.
>
> Friction, roughly in priority order
>
> 1. Discovering a slot's address is the biggest gap. This was my only real fumble. To edit the roadmap I needed roadmap:roadmap#milestones/tier-2-post-v1-rule/decomposition. There's no command that lists the writable addresses of a doc. I guessed jigc doc slots (doesn't exist) and jigc doc show --slots (doesn't exist), then fell back to dumping --format json and writing a Python walker to find the leaf. That's four exploratory calls to discover one address an agent needs constantly.
> - Ask for v1: jigc doc addresses <doc> (or jigc doc show --addresses) that prints every settable leaf/slot/item address, one per line. This alone would remove the sharpest edge.
>
> 2. jigc doc show silently reads the committed store, not the open task's buffer. Correct design, but a trap: after I edited the roadmap slot, doc show would have shown stale content with no warning. I only knew to use task diff because I read it in the help.
> - Ask: when doc show targets a doc with uncommitted edits in an open task, emit a one-line hint: "N staged edits in task X — see jigc task diff."
>
> 3. The jigc doc --help text is a wall. Descriptions like author, show, and schema are dense paragraphs with internal references (design/team-ready-state.md, contract-version: 2). Great as a spec, intimidating as a first read. Lead each verb with the one-line common case; defer the contract details.
>
> 4. Workflow routing is list-only. jigc start "<intent>" with no --workflow printed all ten workflows and made me re-run. That's a deliberate "you choose," but with the intent already in hand, a recommended route ("intent looks like dev-task") would save a round-trip and reduce misrouting. The ten one-liners are good; a ranked suggestion on top would be better.
>
> 5. Small: the changelog gate fires on docs-only changes. docs(roadmap) will essentially never need a changelog, yet single-task granted the gate and nudged about the unused grant. Consider suppressing the changelog-unused advisory when #type == docs.
>
> Things I wanted but didn't have
>
> - A "which docs will this touch" hint at task start. I learned the roadmap was managed only by running jigc doc list. A workflow could surface likely-relevant managed docs upfront so an agent knows early that a given file is jigc-owned.
> - Scope-vocabulary surfacing. jigc mints the #scope field but I had to git log | grep to learn the project's convention (v1 used phase numbers like 05-09; post-v1 has none). Since jigc owns the commit docs, it could offer the existing scope vocabulary as a hint.
>
> Bottom line
>
> For v1, I'd rank #1 (slot-address discovery) as must-fix — it's the one place the tool made an agent guess and fall back to raw JSON parsing for a routine operation. #2 (staged-vs-committed hint) is a cheap safety win against a silent-staleness trap. Everything else is polish. The core loop — start → edit through addressed slots → task diff → finalize — is sound and I'd happily work in it again once addressing a slot is as discoverable as reading one.

### Task 3 — decided-task: the emitted-but-unused `LACON_TOOL_USE_ID`

**Handover (the probe):** a genuine decide-then-implement task — the Claude Code adapter emits `LACON_TOOL_USE_ID` into every wrapped command but nothing consumes it; decide its fate (wire it through with a permanent append-only migration, or remove the dead surface), implement the chosen path, and **record the decision and its rationale as the durable output**.

**Feedback (verbatim):**

> Happy to — this is one data point (one dev-task, one session), so weight it accordingly. Overall it stayed out of my way and the guardrails were well-judged. No hard bugs. The friction was mostly about routing and about where a decision goes. Ranked by how much it mattered:
>
> The one real gap: a decision task had no home for the decision
>
> This task's own intent said it plainly: "record the decision and its rationale (this is the durable output; the code is downstream)." But dev-task is an implementation workflow — the only structured place the rationale could land was the commit body. Meanwhile AGENT.md forbids me from writing managed decision docs (docs/decisions/, docs/open-questions.md, docs/deferral-ledger.md) directly. So for a project whose whole culture is "the record is rebuttable" — 14 ADRs, an open-questions log, a deferral ledger — a decision made mid-task is structurally homeless: I can't write the managed doc, and no workflow I was offered updates one.
>
> That's the thing I'd fix first for v1. Either:
> - dev-task grows an optional "this task also records a decision → here's the doc + fields" branch (like the commit-doc mechanism, but targeting the decision log), or
> - there's a distinct workflow for decide-then-implement (spike → choose → implement chosen path → emit rationale into a managed decision artifact).
>
> Right now the contrast in the menu even implies this exists — quick-fix is described as "records no decision," which reads as "the others do record decisions." dev-task didn't.
>
> Routing doesn't actually route
>
> jigc start "<intent>" bills itself as routing "among the workflows," but in practice it printed all 10 and said "pick one, re-run with --workflow." I had to guess dev-task vs single-task vs plan from one-liners. The distinctions that actually matter — records a decision? test-first? touches managed docs? — aren't the axes the descriptions disambiguate on. Two concrete options: have start ask 1–2 routing questions and pick, or have it recommend a workflow from the intent and let me confirm. Given the intent here contained the literal words "decide … or … implement, and record the decision," a router had plenty to work with.
>
> The whole runbook arrives at t=0
>
> Minting dev-task dumped the entire lifecycle up front — scope guidance, test-first discipline, the gate, and the full finalize recipe with exact jigc doc set-field commit:…#type commands — before I'd written a line. By the time I finalized, that recipe had scrolled way up and I re-derived it from memory. A staged reveal (scope → implement → finalize, each surfaced when I reach it) would keep instructions adjacent to when they're used. Related: I never saw a "where am I / resume" affordance. The footer always says "run jigc start for orientation," but start re-routes; it doesn't resume. Across a context compaction I'd have only scrollback to re-orient from. A jigc task status (current step, what's staged, what's left) would matter a lot for long or interrupted tasks.
>
> Smaller things
>
> - Test-first is "yours to police." The mint said so honestly — I appreciated the candor — but it's the one promise the workflow makes and can't keep. An optional jigc task check-red <test> that runs the test, records that it failed, and gates the implement step would turn the discipline from aspirational to verifiable.
> - No-op advisories train me to ignore advisories. Finalize printed advisory · file-state.baseline-adopt … route: no action needed. If advisories are usually "no action needed," I'll stop reading them — and then miss the one that isn't. Suppress the no-ops or make them debug-level.
> - Commit-field vocabulary is invisible. I picked type=refactor and it was accepted silently. Is there an enum (fix/feat/chore/…)? If yes, echo or validate it; if it's free-form, saying so avoids the guess.
>
> What genuinely worked (keep it)
>
> - The finalize output — commit hash, subject, and the exact files committed — gave me real confidence about what landed and would've caught a stray unstaged file. The repeated "commits only what you staged" warning made me do an explicit git add rather than trust the index. Good friction in the right place.
> - The forced intent-restatement + done-criterion at scope, with "if your restatement reveals a different problem, stop and check." That's exactly the behavior this task needed (it was a stop-and-check), and jigc prompting it up front is right.
> - Honesty about its own limits ("nothing enforces test-first; hold the discipline yourself") beats pretending. Keep that tone.
>
> One caveat on all of the above: some of what I call "missing" (status, resume, a decision doc flow) may exist and I just didn't discover it from the surfaced output — which, if so, is itself feedback that discoverability is the thing to invest in for v1. The commands I was handed, I could follow without a stumble; it's the ones I wasn't pointed to that I couldn't use.

### Task 4 — milestone: Tier-2 bundled-rule expansion

**Handover (the probe):** a whole milestone — extend lacon's bundled rule library by a coherent Tier-2 batch: scope-first (pick families, settle a shared `install-base` inheritance question, decompose), build the rules in parallel, audit the assembled batch before calling it done. The full milestone surface was in reach: planning, Settle, decomposition, execution, completion.

**Feedback (verbatim):**

> Happy to. I used jigc end-to-end here — one planning workflow, two dev-tasks, one single-task, plus a lot of doc verbs — so this is from actual use, not impressions. Overall: the core model is genuinely good and I'd want it. The structured-doc writes and the workflow scaffolding did real work for me. The friction was almost entirely around reading state back, not writing it.
>
> What worked well (keep / lean into)
>
> - Structured writes with explicit addressing (roadmap:roadmap#milestones/<id>/proves). I never once hand-placed a heading, anchor, or cross-ref — add-item minted the slug and returned it, set-slot took the prose. This is the whole value prop and it delivered. Zero structural mistakes.
> - Workflow routing as discipline. start → pick workflow → guided steps actually kept me honest: settle-before-build, record decisions/deferrals, test-first, one-logical-commit. Left to my own devices I'd have skipped the deferral-ledger entry for bundle.
> - finalize as the single commit boundary (stages my git + promotes managed docs) is clean and predictable.
> - Create-or-update singletons ("safe whether it exists, copied in for append") — roadmap/decisions-log/deferral-ledger all just worked without me checking existence.
> - --from-file - + heredoc for prose slots is the right ergonomic.
> - retitle-item with frozen anchor, and the enum-id refusal note — thoughtful invariants.
>
> Friction & gaps (prioritized for v1)
>
> 1. Reading staged state back is the #1 rough edge. After a successful set-slot (it even echoed the char count), my instinct was jigc doc show to verify — but show reads the committed store, so it showed the OLD content and for a moment I thought my write hadn't landed. The fix (jigc task diff <id>) is documented deep in --help, but the mental model (committed store vs. open-task working set) isn't obvious at the moment you need it.
> → Add jigc doc show <addr> --task <id> returning the staged view. That single command would erase the confusion. Today --task is rejected on show, which actively points you the wrong way.
>
> 2. The read/write --task asymmetry surprises. Write verbs require --task; read verbs reject it. The error tip (use '-- --task') is good but misleading here — it implies I wanted a positional. A message like "show reads the committed store; for an open task's staged version use task diff <id>" would teach the model instead of deflecting.
>
> 3. No "where am I / what's next" for an in-flight task. jigc task list shows tasks but not the current phase or remaining steps. Mid-increment I scrolled back to the start output to recover the finalize incantation.
> → jigc task next <id> (or task show) re-printing the current workflow's remaining steps + the exact set-field/finalize commands for this task would help a lot, especially after a context compaction.
>
> 4. planning dumps every phase at once. The start --workflow planning output was one long block covering scope → settle → review → decompose → record → commit. Comprehensive, but I couldn't tell which phase I was "in," and it front-loaded commit instructions I didn't need for 20 minutes. dev-task felt more focused. Consider phase-gated reveal, or at least visual phase separators with a "you are here."
>
> 5. Minor discoverability: jigc task status doesn't exist (it's list); I guessed wrong first. An alias or a status view wouldn't hurt.
>
> Things I missed / would add
>
> - A staged render/preview of a single slot before finalize (doc show <addr>/proves --task) so I can proof one slot without diffing the whole changeset.
> - Surface doc author (the batch verb) in start guidance. I defaulted to N individual add-item/set-slot calls because that's what the workflow text modeled; I only noticed author when reading doc --help afterward. For authoring a fresh milestone it'd have been one call instead of four.
> - The human-gate is convention-only. The Settle gate says "the human owns it," but jigc doesn't track it — I drove the AskUserQuestion myself and nothing recorded that the gate was cleared. If gate-clearing matters for v1's integrity story, a lightweight task gate-cleared stamp would make it auditable rather than trust-based.
>
> One boundary question
>
> AGENT.md says "never read or edit managed docs directly." Early on I used the raw Read tool on docs/roadmap.md (a managed doc) for planning context, and on the bundled-rule YAML (which I think is source, not managed). The line between "managed doc → go through jigc doc show" and "source file → read freely" isn't crisp. Clarifying it — and nudging jigc doc show as the read path — would prevent an agent from doing what I did.

## Routing

Every finding above is verified in [findings-verification.md](findings-verification.md) and routed per the drain rule — idea files updated/parked and fired decisions-pending triggers annotated in the same motion as this record ([DECISIONS.md](../../../DECISIONS.md) → 2026-07-15 lacon trial). The fix-shaped findings **chartered 2026-07-16 as M43, the rc.7 surface-contract wave** — three checkable laws (nothing lies · nothing hides · nothing ambushes) + a surface style guide, the fixes landing as law instances ([decisions-pending.md](../../../implementation/decisions-pending.md) → The rc.7 wave; [DECISIONS.md](../../../DECISIONS.md) → 2026-07-16 charter).
