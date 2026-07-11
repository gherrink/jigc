# RC adoption trial (migration half) — project-alpha-2.0 (2026-07-09)

**Status: counts as RC input — migration half only** (human call, 2026-07-10 — [DECISIONS.md](../../../DECISIONS.md) → 2026-07-10 RC adoption-migration trial). This run is the *migration* half of the planned existing-project adoption trial: a full GSD→jigc corpus migration, **no implementation job**. The adoption trial's *implementation half* (real dev work driven through jigc in the migrated repo) is **still owed before the 1.0.0 call**, and its log analysis with it.

> **Rerun on rc.4 (2026-07-11):** the migration half was re-run from a fresh copy of the original repo (corrected directory name `project-alpha-2.0`) on the installed 1.0.0-rc.4, with a different, multi-agent method — record, log analysis, and the 14 verified findings live in [rerun-rc4/](rerun-rc4/trial-record.md). The implementation half is running in that same repo.

## Provenance

- **Repo:** `~/ideas/project-alpha-2.0` (directory name carries the typo) — an existing GSD-managed project (`.planning/`, 215 files), agent-driven migration on the human's direction.
- **Binary:** jigc **1.0.0-rc.3** (installed release build; workbench stamp `jigc-version: 1.0.0-rc.3`).
- **Invocation log:** ON with the M39 output-size fd-tee — **416 records** over ~72 min (2026-07-09 12:28–13:40 UTC); snapshot at [invocations.jsonl](invocations.jsonl) (entries 0–415; three later read-only analysis invocations excluded).
- **Outcome:** 20 conventional commits, `.planning/` fully retired, 41 managed docs across 12 doctypes, store validates clean. Corpus kept in place (keeper).
- **Analysis inputs archived here:** [findings-verification.md](findings-verification.md) (the 12 findings verified against the code, with file:line evidence) · [self-migration-coverage.md](self-migration-coverage.md) (jigc-repo → doctype coverage map) · [capability-matrix.md](capability-matrix.md) (doctype × capability matrix + the new-doctype checklist draft). The latter two were commissioned off the trial's doctype-gap feedback and are the planning inputs for the post-1.0 doctype-completeness milestone.

## Feedback (verbatim, captured 2026-07-09)

> Genuinely useful tool, and I'd rather use it than hand-maintain a docs tree. But I hit real friction, and one thing I'd call a latent trap. Concretely, from this session:
>
> **What worked**
>
> The gates changed the artifact, not just checked it. This is the strongest thing I can say for jigc. Because arch-doc requires implemented-by anchors to resolve, I had to verify 39 path#symbol pairs against the codebase before authoring. That check is what surfaced that codebase/ARCHITECTURE.md was stale in two independent ways — every path predated the apps/dashboard/ move, and it omitted app/Services/ entirely, which is most of what v1.0 built. If I had migrated it as prose, I'd have produced a plausible, wrong document and neither of us would have noticed. A doc format that makes staleness a build error earns its complexity.
>
> The corollary held too: ResourcePolicy was cited by the old doc, I misread a truncated ls and concluded it didn't exist, and the anchor check would have caught me either way.
>
> Errors carry routes. ingest didn't just say "wrong location," it said move this file there, then re-run. The title-names-symbol failure named the offending token, the anchor's actual symbol, and the fix. That's the difference between a linter and a collaborator.
>
> Reversibility made exploration cheap. task discard and unmanage are both idempotent and non-destructive, so I probed five workflows by minting a task and throwing it away. unmanage leaving bytes on disk is exactly right — I used it twice to legally hand-edit the roadmap and re-adopt.
>
> doc author as a declarative batch. Authoring 38 components in one payload rather than 38 add-item calls is the difference between usable and not. Same for 22 ideas and 4 ADRs in single tasks.
>
> **What cost me time**
>
> The create path hides the schema; the migrate path shows it. Every migrate-* workflow printed a complete payload template. jigc doc create idea printed a skeleton with date and ## Description and nothing else — and then finalize blocked on a required trigger field in meta that the skeleton never rendered. Same guessing game for adr. There's no jigc doc schema <doctype>. I got the shape by creating a doc and reading the file off disk, which is precisely what AGENT.md tells me not to do.
>
> There's no way to rename a repeatable item. When the symbol gate rejected three of my 38 component titles, the route said "rename the component heading." There is no rename verb for an item. remove-item plus add-item appends, which would have reordered the document. My only option was task discard and re-author all 38. jigc rename exists, but only for doc slugs.
>
> Three commands disagreed about what "managed" means. ingest classified .planning/research/*.md as unmanaged — "parses against no schema." validate then flagged those same five files as file-state.orphaned-doc — "it looks managed but resolves under no doctype location." The routed remedy, jigc unmanage, replied "not managed (nothing to drop)." Three answers, one of them a no-op, for the same five files. That advisory would have followed the repo forever if we hadn't deleted the tree.
>
> ingest's conformance check is shallower than it sounds. This is the one I'd call a trap. It adopted docs/roadmap.md as a conformant roadmap. Then jigc doc show roadmap:roadmap --format json returned "milestones": []. It matched headings, not structure — the file was adopted, indexed, baselined, and structurally empty. I only caught it because I was about to delete v1.0-ROADMAP.md and went looking for where its content should live. Adoption confers a strong feeling that content is now under management, and here it wasn't. Ingest should report parsed item counts, or at minimum warn when a required repeatable section adopts with zero items.
>
> The title-names-symbol heuristic taxes prose. Observed behavior: WordPress in a heading was parsed as a symbol and had to match the anchor exactly; the audit service, the git host, and Redis passed. I inferred the trigger is an internal capital, but I never confirmed that. The practical effect is that "WP registry client" is legal and "WordPress registry client" is not — even though the anchor is CmsGlobalClient, which contains the rejected token. It also never fires on anchorless components, so the same title is legal or illegal depending on whether you supplied an anchor.
>
> The fidelity gate cried wolf on its first use. Migrating the PRD, it reported source releases absent from the rewrite: 2.0. The document is titled Project-alpha 2.0. It's self-labeled fuzzy and advisory and cost me nothing, but this is the one gate whose entire job is to make a human read a diff. Training the reader to dismiss it on the first encounter is the wrong first impression.
>
> Two smaller things. jigc migrate retires the foreign original itself — I pre-staged git rm .planning/PROJECT.md, and finalize died with a raw fatal: pathspec ... did not match any files rather than a routed finding. And ingest scanned 4,627 candidates including node_modules, producing a 630KB report on the very first command a new user runs; it should respect .gitignore.
>
> **Where I was wrong**
>
> I read authored docs straight out of .jigc/tasks/<id>/docs/ to check my work, because doc show explicitly refuses --task and reads only the committed store. jigc task diff exists and is the sanctioned read-back path. I didn't reach for it. That's my miss — but the fact that doc show's help goes out of its way to say it takes no --task, without pointing at what does, made the wrong path the obvious one.
>
> **If you fix one thing**
>
> Make ingest report what it actually parsed. "adopted — indexed + baselined" for a roadmap with zero milestones is a true statement that communicates something false, and it's the failure mode most likely to bite someone onboarding an existing repo — which is exactly the situation ingest exists to serve.
>
> After that: jigc doc schema <doctype>, and an item-rename verb.

## Doctype gaps (verbatim, asked as a follow-up 2026-07-10: "what doctypes did you miss?")

> Yes — four, and one of them actively degraded the output.
>
> **The one that hurt: a defect / finding record.** Three of the 22 idea docs I wrote are live security findings, verified present in the working tree: a hardcoded credential, missing authorization checks, and a secret read from the environment. idea is defined as "one shaped-but-unscheduled direction, with the trigger that would bring it back." A hardcoded credential is not a direction. It's a defect. The consequence is that docs/ideas/remove-a-hardcoded-credential.md and docs/ideas/support-multiple-domains-per-website.md are now the same doctype, with the same three fields, and nothing distinguishes "this is a security hole in main right now" from "this would be nice in v2." Severity is flattened, and I had to compensate in prose. What makes this frustrating is that jigc already has the vocabulary: a completion-record finding carries severity (blocking/advisory), disposition (fixed/deferred/contested), and evidence — imprisoned inside a per-milestone completion record. A standalone finding doctype with those three fields plus arch-doc-style implemented-by anchors would be the single highest-value addition. The anchor is the interesting part: jigc validate already fails when an anchored symbol vanishes. Point a finding at ImportOldCommand.php#ImportOldCommand, and the moment someone rips out the credential, validate tells you the finding is stale. A defect record that self-invalidates against the code is something almost no tracker does, and jigc is three fields away from it.
>
> **Postmortem / incident record.** .planning/debug/resolved/audit-chrome-econnrefused.md — symptom, root cause, fix, from a real debugging session. It is not an adr (no decision was made), not research (no forward question), not an idea (nothing is owed). So it was deleted. In practice this is the most re-read artifact type there is: "we've seen this error before, here's why." jigc ships dogfood-record for measuring its own runs but has nothing for "we hit a bug in the wild and here's what it turned out to be."
>
> **Reference / handbook.** codebase/STACK.md, CONVENTIONS.md, TESTING.md, STRUCTURE.md — about 1,300 lines, all true, all useful, matching no schema. arch-doc is close but wants "one part of the system and its components," which conventions and stack are not. We agreed to drop them as "CLAUDE.md's territory," and I want to name that for what it was: a dodge. CLAUDE.md is unmanaged, unvalidated, and invisible to jigc validate. A reference doctype — a topic, prose, optional code anchors, no required component structure — would have kept STRUCTURE.md and TESTING.md honest against the repo instead of leaving them to rot in a file nothing checks.
>
> **Runbook.** Minor, but a category. TODO.md carried two working curl invocations exercising the app's upload and download endpoints, with inline credentials and fixture paths. Reproducible operational procedure. Nothing holds it. Dropped.
>
> **Not a doctype gap, but adjacent.** v1.0-REQUIREMENTS.md had a traceability table mapping all 30 requirements to the phase that delivered them. Dropping the checkbox state was correct given the schema — but the information was real, and it now survives only as prose in the roadmap decomposition. spec has an optional maps-to-test on a criterion, which is one short step from "this criterion is satisfied, and here is the proof." That's a field, not a doctype.
>
> **What I didn't miss.** The other eleven doctypes covered most of the durable content by value. I never created a vision because prd genuinely absorbs it for a brownfield project — though now that five research records are committed, form-vision with a real grounded-in edge has become possible where it wasn't before. And milestone-record is for in-flight work, so a shipped milestone correctly had no use for it. The one that reads as out of place is dogfood-record — a doctype for measuring jigc's own instrumented runs, offered to every project that installs it.

## Invocation-log analysis (migration half, 2026-07-10)

**Headline: the loop held mechanically; the friction was all discoverability.** 416 invocations drove the full corpus migration to 20 finalize-rendered commits with **15 non-zero exits (3.6%)** — every one either a usage error (immediately corrected) or a gate doing its declared job; zero blind retries, zero drive-arounds landed (working tree clean, every doc commit a finalize commit).

- **Verb mix:** `set-field` 103 · `set-slot` 88 · `add-item` 37 · `create` 33 · `start` 29 · `validate` 28 · `finalize` 21 · `task validate` 19 · `author` 10 · `discard` 10 · `migrate` 7 · `unmanage` 7 · `ingest` 6 — a write-heavy migration profile, as expected.
- **The output story (the fd-tee's first real catch):** total logged output 4.17 MB; median call **84 B**, p90 3 KB — but the 6 `ingest` runs emitted 618–645 KB each, **≈3.79 MB = 91% of all output** (the raw-walk/no-gitignore finding F8, quantified). Everything except ingest is tiny — the greenfield A7 "mass output" worry is, on this evidence, an ingest problem, not a general one.
- **The F1 signature:** finalize on the ideas batch failed twice with **22× `required-field-present`** (`trigger`, invisible in the create skeleton), then ~22 `set-field` repairs → clean. Plus a visible probe pattern: throwaway tasks/docs minted purely to learn schemas (`describe` used once).
- **The F4 signature:** the run ends in roadmap thrash — finalize-milestone → `doc show` (empty milestones) → unmanage → **3 byte-identical ingest runs in 24 s** → doc show again. An agent hitting the structurally-empty-adoption wall and giving up.
- **Finding-code totals:** `baseline-adopt` 141 · `orphaned-doc` 125 (all mid-migration transients; the F3 trichotomy) · `required-field-present` 66 · `un-baselined` 36 · `title-names-symbol` 3 (the F5 brand-name taxes).
- **Instrumentation gap found:** the record carries no **binary-version field** — version is only knowable from `.jigc/version`, which weakens cross-trial log analysis. (The greenfield analysis's owed output-size field shipped in M39 and worked; this is the next such gap.)

**The roadmap complaint, root-caused:** `docs/roadmap.md` post-migration is (1) an ingest-adopted GSD roadmap that parsed to **zero milestones** — adopted, indexed, baselined, structurally empty (F4) — later hand-repaired to one retrospective v1.0 milestone; plus (2) two stale GSD sections (`## Phases`, `## Progress`) trailing the managed structure, **invisible to both `doc show` and `validate`** (byte-faithful adoption baselined them; the surplus-H2 half of F4). The same v1.0 history is stated three times across the file, duplicating CHANGELOG.md and the completion record. Cleaning the file is dashboard-side work; the two product holes route below.

**What worked (worth naming):** the intentional one-source→many-target splits were done well *manually* — ARCHITECTURE.md → arch-doc + 4 extracted ADRs with `cites` wired back; TODO.md + CONCERNS.md → 22 ideas; the milestone audit → completion-record + verbatim owner artifact. That manual splitting is exactly what F11's re-opened fork would systematize. One dashboard-side follow-up flag: CONVENTIONS/TESTING/STACK/STRUCTURE (~880 lines) were retired with the claim their live content was absorbed — content-level absorption was not verified, and conventions/testing guidance has no obvious home in the migrated corpus (the `reference` doctype gap, D2).

**Honest bounds:** single user, single repo, migration-only — no code task was driven, so the dev-workflow surface (the greenfield trial's strength) went unexercised here. The two halves compose: greenfield proved the authoring loop, this proved the adoption/migration loop; the *implementation-in-a-migrated-repo* combination is the still-owed piece.

## Triage disposition (routed 2026-07-10; fixes land via the rc.4 wave — [decisions-pending.md](../../../implementation/decisions-pending.md) → the rc.4 wave)

Full verification evidence (verdicts, file:line, minimal fix shapes): [findings-verification.md](findings-verification.md).

| # | Finding | Verdict | Route | Status |
|---|---------|---------|-------|--------|
| F1 | Create skeleton hides required meta; no `jigc doc schema` | PARTLY (adr half refuted — `status` defaults) | fix, rc.4 — generalized fillable skeleton (the commit-doc `fillable_form` precedent; also hits completion-record ×2, dogfood-record ×14) + a `doc schema` read surface | **→ rc.4 planning** |
| F2 | No repeatable-item retitle/reorder verb | CONFIRMED | fix, rc.4 — `retitle-item` (retitle-without-reslug is a *declared* invariant with no verb; a blocking route commands it) | **→ rc.4 planning** |
| F3 | Three definitions of "managed"; orphan advisory routes to a guaranteed no-op | CONFIRMED | fix, rc.4 — gate the found-stranded route on actual registration | **→ rc.4 planning** |
| F4 | Ingest adopts structurally-empty required repeatables; surplus foreign H2s invisible | CONFIRMED (worse: surplus-H2 case) | fix, rc.4 — zero-items + surplus-H2 advisories at adopt time and store scope (no new gate) | **→ rc.4 planning** |
| F5 | `title-names-symbol` taxes brand names; anchorless asymmetry | CONFIRMED | fix, rc.4 — case-folded containment match + route must name a real operation (pairs with F2) | **→ rc.4 planning** |
| F6 | Fidelity gate false alarm ("2.0" from the title) | CONFIRMED (changelog-shaped check runs on all doctypes) | fold-in, rc.4 — kept-set scans the whole rewrite | **→ rc.4 planning** |
| F7 | Pre-staged `git rm` → raw git fatal; rollback resurrects the user's deletion | CONFIRMED (reproduced) | fix, rc.4 — **known hole** in the routed-errors invariant; stage on index state, not HEAD | **→ rc.4 planning** |
| F8 | Ingest ignores .gitignore (4,627 candidates, 630 KB, first command) | CONFIRMED | fix, rc.4 — `git ls-files --exclude-standard` candidate set, raw walk as no-git fallback | **→ rc.4 planning** |
| F9 | ingest vs migrate — no up-front cross-routing | PARTLY | fold-in, rc.4 — one cross-pointer sentence each way | **→ rc.4 planning** |
| F10 | No migrate-* for any methodology doctype | CONFIRMED (registry = workflow name; pure pack YAML) | fix, rc.4 — the 6 adoption-relevant migrate workflows (vision/roadmap/decisions-log/deferral-ledger/idea/completion-record); human call 2026-07-10: **in, definitely** | **→ rc.4 planning** |
| F11 | One source → many doctypes impossible (mixed-content sources) | CONFIRMED (deliberate M25 deferral; its re-open trigger **fired** — twice, incl. our own VISION.md) | process — fork re-opened in decisions-pending, keyed to the doctype-completeness milestone | **done 2026-07-10** |
| F12 | `doc show` help doesn't point at `jigc task diff` | CONFIRMED | fold-in, rc.4 — one sentence | **→ rc.4 planning** |
| D1 | `finding` doctype (severity/disposition/evidence + self-invalidating code anchors) | — | idea | **parked 2026-07-10** — [ideas/finding-doctype.md](../../../ideas/finding-doctype.md) |
| D2 | `reference`/handbook doctype | — (independently confirmed by the self-migration audit: ~40 of our own files wait on it) | idea — the doctype-completeness milestone's anchor | **parked 2026-07-10** — [ideas/reference-doctype.md](../../../ideas/reference-doctype.md) |
| D3 | postmortem + runbook doctypes | — | idea | **parked 2026-07-10** — [ideas/postmortem-and-runbook-doctypes.md](../../../ideas/postmortem-and-runbook-doctypes.md) |
| D4 | spec per-criterion status/traceability | — | idea (frozen-v1 gated: schema-version bump + corpus migration) | **parked 2026-07-10** — [ideas/spec-criterion-status.md](../../../ideas/spec-criterion-status.md) |
| D5 | dogfood-record visible in every adopter's describe | — | idea (per-doctype pack-visibility knob) | **parked 2026-07-10** — [ideas/pack-doctype-visibility.md](../../../ideas/pack-doctype-visibility.md) |
| A1 | Methodology schemas unversioned — shape change strands committed corpora silently | from the capability matrix, not the trial | settle at rc.4 planning (one-way-door lens; robust-advocate) | **→ rc.4 planning** (decisions-pending) |
| A2 | Changelog nested repeatables write-only through `doc show` JSON (+ address-grammar asymmetry) | from the capability matrix | settle at rc.4 planning (pinned-contract adjacency) | **→ rc.4 planning** (decisions-pending) |
| A3 | Invocation log lacks a binary-version field | from this log analysis | fold-in, rc.4 (the M39 output-size precedent) | **→ rc.4 planning** |
| A4 | Polish: adr author step under-specified · placement-reslug error message · lowercase `# roadmap`/`# decisions-log`/`# deferral-ledger` H1s (missing `display-title`) · unguarded milestone-record rename desyncs work-unit id | from the capability matrix | fold-in candidates, rc.4 | **→ rc.4 planning** |
