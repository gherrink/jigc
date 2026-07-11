# RC adoption trial — migration-half rerun on rc.4 — project-alpha-2.0 (2026-07-11)

**Status: RC input, migration half rerun** — the full GSD→jigc migration re-run from a fresh copy of the original repo (directory renamed to the corrected spelling `project-alpha-2.0`), this time on the installed **1.0.0-rc.4**. The implementation half of the adoption trial is **in progress** in the same repo (smaller dev workflows, invocation log still ON) and gets its own record + log analysis when done; the 1.0.0 call waits on it.

## Provenance

- **Repo:** `~/ideas/project-alpha-2.0` — a fresh copy of the original GSD-managed project (`.planning/`, ~220 files), agent-driven migration on the human's direction.
- **Binary:** jigc **1.0.0-rc.4** (installed release build; workbench stamp `jigc-version: 1.0.0-rc.4`; every log record carries `binary_version: 1.0.0-rc.4` — the M40 A3 field, first trial where the log self-identifies the binary).
- **Method:** materially different from the first run — Claude Code `ultracode` multi-agent orchestration: parallel read-only assessment (18 agents, claims checked against code), synthesis + an adversarial critic, human-settled forks, then **parallel drafting (26 + 10 agents) with strictly sequential writing in one process** (jigc mints a task + commit per doc; parallel writers would race on tool state and the index). The full method write-up is archived at [migration-method.md](migration-method.md). The human's assessment: this approach was better than the first run's.
- **Invocation log:** ON — **1145 records analyzed** (entries 0–1144, 2026-07-11 04:13–09:13 UTC, ~5 h); snapshot at [invocations.jsonl](invocations.jsonl) (1150 entries — the tail past 1144 is the implementation half starting, analyzed with that record, not this one).
- **Outcome:** 51 conventional commits on the day, `.planning/` fully retired, **41 managed docs** (40 under `docs/` + root `VISION.md`), 11 specs carrying 153 acceptance criteria, store validates clean.
- **Findings verification:** [findings-verification.md](findings-verification.md) — every feedback item + log-discovered finding verified against the rc.4 code with file:line evidence.

## Log analysis

**Volume and shape.** 1145 invocations (vs 416 in the first run — the rerun drove far more granular writes): `doc set-slot` 373 · `doc add-item` 245 · `doc set-field` 223 · `task` 89 · `validate` 57 · `start` 54 · `doc author` 24 · `doc create` 22 · `migrate` 21 · `doc schema` 15 · `ingest` 5 · `doc retitle-item` 5 · `doc show` 4 · `rename` 3. Total output 1.26 MB; durations p50 1 ms / p95 91 ms / max 691 ms (a `task diff`). 50 `task finalize` calls, 45 clean.

**The M40 surface was load-bearing.** The verbs shipped in the rc.4 wave were used in anger and held: `doc schema` **15/15 clean** (the F1 create-path-hides-the-schema fix, confirmed in use), `doc retitle-item` 4/5 (the F2/F5 identity verb — used to clear symbol-drift advisories without reslugging, exactly its design case), `doc show` 3/4, `migrate` 21 calls with the F8 candidate funnel. The new findings sit one layer deeper than the rc.3 wave's — the previous wave's holes did not recur.

**Failures: 15 non-zero exits, all explained, no unexplained crash.**

- 6× `doc author` exit 1 — schema-validation refusals of bad payloads (the gate working; the payloads' badness partly traces to the template-shape findings below).
- 3× `task finalize` exit 3 + 2× exit 4 — blocking findings incl. one `finalize.migration-no-replacement` (the refusal-to-retire gate: declined to delete a foreign source with no staged replacement — praised verbatim in the feedback) and the bad-anchor block.
- 1× `rename` exit 1 — bare slug passed where a `<type>:<slug>` address is required; the error is correct but hint-less (papercut, verified below).
- 1× `doc retitle-item` exit 1 — the no-active-task refusal; retried with `--task` 12 s later, clean. This is the "validate routes omit the task-minting step" dead-end observed live in the log (finding V8).
- 1× `doc show --format json` exit 1 — `store.not-found` on a doc still in-flight/uncommitted at that moment. Correct behavior (`doc show` reads the committed store); the error could route to the in-flight task.
- 1× `start` exit 1 — a `--workflow plan` invocation (workflow-resolution refusal).

**Log-discovered signal the feedback missed:** `doc-code.unsupported-language` fired 209 times across 47 records. Verified: it is **9 distinct `.vue` anchors** re-emitted on every validate/finalize sweep, not 200 — but the underlying hole is real and rated medium (finding V4): the anchor gate is **symbol-blind for Vue SFCs** (file-existence still blocks; `#symbol` degrades to a route-less advisory), which partially undercuts the flagship "a doc cannot claim code that doesn't exist" property on exactly this trial's stack.

**Advisory noise profile:** `file-state.unregistered-doc` 143 · `file-state.baseline-adopt` 102 · `schema-completeness.inverse-cardinality` 47 · `doc-code.title-names-symbol` 45 (the M40 F5 demotion to advisory held — it advised, never blocked) · `write.slot-heading-depth` 2.

## Feedback (verbatim, captured 2026-07-11)

> Confirmed both: --format json on start just wraps the same prose in a "text" field — no structured task_id — and there's no --unset. So the friction below is real, not misuse.
>
> I used jigc hard here: 40 docs, ~55 commits, 8 doctypes, three write paths (migrate, workflow + doc create, doc author). Here's what I'd fold back in.
>
> **What's genuinely excellent — don't touch it**
>
> The code-anchor gate is the whole product. jigc refused to commit an arch-doc citing app.ts#createInertiaApp because the symbol is imported there, not declared. Think about what that means in this specific repo: the entire reason .planning/ had to die is that GSD let 200 documents claim things about code that weren't true, and a self-scored audit blessed them. jigc makes that class of lie structurally impossible to commit. That is a much stronger claim than "we have docs" and I'd lead with it.
>
> The refusal-to-retire gate is the same idea applied to migration. When my deferral-ledger author failed schema validation, finalize declined to delete TODO.md — "recorded the foreign source but staged no managed doc to replace it." A lesser tool eats the source file and you find out later. That gate is why I trusted migrate enough to point it at 220 files.
>
> migrate retiring the source in the same commit as the promoted doc is the right atomic unit. .planning/ drained itself as I went; there was never a window where content existed in neither place.
>
> retitle-item with the anchor frozen — an identity refactor that doesn't reslug — is a genuinely thoughtful piece of design. Most systems would silently break every referrer.
>
> **The one bug that silently produces bad documents**
>
> Your doc author payload templates demonstrate double-quoted YAML scalars, and double-quoted YAML scalars fold newlines.
>
> The migrate-research template literally shows:
>
> question: "<<the question this research set out to answer>>"
>
> I followed it. My bullets merged into one line, my paragraph breaks vanished, and it committed clean — no error, no warning. I only caught it because I happened to diff the staged doc. Every gate you have is about preventing silent falsehood, and this one silently mangles the author's prose.
>
> The fix is one character in every template: show the literal block scalar.
>
> question: |
>   <<the question this research set out to answer>>
>
> I'd treat this as the highest-priority item on the list. It defeats the tool's own purpose, and a less careful agent ships mangled docs and never knows.
>
> **Papercuts, each with an obvious fix**
>
> Task IDs are only available inside backticked prose. The composed workflow says Run: `jigc doc create ... --task foo` . Scraping --task (\S+) captures the trailing backtick, which then blows up the next shell command with EOF in backquote substitution. I lost two cycles to this. --format json doesn't help — it returns {"text": "<the same prose>"}. Put task_id in the JSON as a field. Right now the JSON mode is a wrapper, not a contract.
>
> Enum members aren't discoverable. jigc doc schema deferral-ledger prints kind: enum * and nothing else. I learned the members by being rejected: they're D and I. Two problems — the schema projection should list enum members (it's the single thing you need in order to author), and D/I are cryptic where deferred-decision/parked-idea would be self-documenting. Same gap on adr.status and completion-record.verdict.
>
> Field placement in author payloads varies per doctype with no inferable rule. research puts date under a meta section. adr puts status under a section called status, and date/cites-code aren't in the author template at all — they need separate set-field calls. arch-doc's cites is a field. I discovered each by trial and error. Either make author accept all fields uniformly under a fields: key, or have doc schema emit the author payload shape.
>
> Repeatable sections take items:, not add: — undocumented until the error fires. The error message was good; just say it in the template.
>
> set-field --value '' doesn't clear a field, and there's no --unset. I needed to drop one bad code anchor. The empty write was accepted, finalize still failed on the old value, and I had to remove-item → add-item → rewrite the description to get rid of it. That's a destructive workaround for a trivial need.
>
> validate routes you to commands that can't run. It told me to run jigc doc retitle-item …, which replies no active task — start one with jigc start. The route is a dead end; it should include the task-minting step.
>
> **Two real bugs**
>
> Slug truncation cuts mid-word. I have a committed doc at docs/architecture/cms-integration-contrac.md. "contrac". Also the-cms-rest-api-cannot — truncated mid-clause into something that reads as a sentence fragment. Truncation should be word-boundary-safe at minimum, and ideally drop leading articles before it starts cutting.
>
> Nothing warns you that arch-doc.cites creates a hard ordering dependency on ADRs. The natural authoring order is architecture → decisions; the required order is the reverse, because a citation to a non-existent ADR fails the commit. I caught it by reading the composed workflow carefully. One line in the architecture-documentation workflow — "author the ADRs you'll cite first" — saves a wasted cycle.
>
> **One observation about scale**
>
> jigc ingest on a 220-file tree emitted 32KB and 219 identical unmanaged … parses against no schema lines. The verdict was correct, but I had to grep my own tool output to find the single actionable row. Group the report by directory (168 files under .planning/milestones/**: unmanaged) and the signal is instantly visible.
>
> More broadly: jigc's protocol is prose addressed to an LLM, which is lovely for one doc and expensive for forty. I ended up writing a Python driver around it. That's fine — but it means the JSON mode is the surface that matters for anything at scale, and right now JSON mode is prose-in-a-box. If you want jigc used for migrations like this one rather than just steady-state authoring, make --format json a real contract: task id, doc slug, item id, findings as data.
>
> **The thing I'd most want you to keep**
>
> The gates fired four times in this session and were right every time — the bad anchor, the unstaged replacement, the symbol-drift advisories, the discharged ledger entry. I didn't have to trust myself, and given that I was authoring 40 documents about a codebase whose own documentation had been confidently lying for months, that was worth more than any convenience feature you could add.

**Fold-back priorities (verbatim addendum, same agent, relayed 2026-07-11):**

> The three things I'd carry into the fold-back, in order: fix the folding-YAML templates (silent data corruption), fix doc author's payload discoverability (it's the keystone verb and it's the one that breaks), and fix the advisory noise floor (or the gates die of neglect). Everything else on my list is a papercut by comparison.

Mapped to verified findings in [findings-verification.md](findings-verification.md): V1 · V3+V6+V11 (one problem under the keystone-verb framing) · V15.

**On the machine surface (verbatim addendum, same agent, relayed 2026-07-11):**

> jigc's output protocol is prose written for an LLM to read, which is lovely for one document and expensive for forty. --format json currently returns {"text": "<the same prose>"} — a wrapper, not a contract. If you want jigc used for migrations at scale rather than only steady-state authoring, make JSON emit real data: task id, doc slug, item id, findings. That's a critique of the machine surface, not a claim that jigc is CLI-only.

This is finding V2 / Settle fork 1 of the M41 charter. The human's own trial observation reinforces it: the agent ran `--format json` heavily and ended up writing a Python driver around the prose.

## Related standing item (not this run's finding)

The `~/ideas/project-kb` greenfield repo's `.jigc/AGENT.md` still carries the pre-A4 exit-code table; re-running `jigc setup` there would refresh it but also re-add the allow permits + deny floor into `.claude/settings.json` (deliberately deleted there) — the refresh and the permission write are bundled until `ideas/adapter-permission-model.md` lands. Standing operational note, owed to that idea's de-parking.
