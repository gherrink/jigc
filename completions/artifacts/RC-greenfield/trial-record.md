# RC greenfield trial 1 — project-kb (2026-07-06)

**Status: counts as RC input** (human call, 2026-07-06 — [DECISIONS.md](../../../DECISIONS.md) → 2026-07-06 RC greenfield trial 1). This run is the greenfield arm of the two planned RC trials (keeper corpora); its invocation log joins the RC invocation-log analysis that feeds the 1.0.0 call. The adoption trial (existing project) is still owed.

## Provenance

- **Repo:** `~/ideas/project-kb` — fresh project, human-driven from `jigc setup` onward (keeper corpus, kept in place).
- **Binary:** jigc **1.0.0-rc.2** (installed release build; store stamp `jigc-version: 1.0.0-rc.2`).
- **Composition:** dev pack ▸ embedded methodology (`compose-embedded-methodology: true`) — the full M37/M38 design altitude was exercised (`do-research` → `form-vision` with `grounded-in`, `park-idea`), plus the M38 placement layout.
- **Invocation log:** ON — 72 records; snapshot at [invocations.jsonl](invocations.jsonl) (source: `<repo>/.jigc/logs/invocations.jsonl`).
- **Analysis:** owed — runs with the adoption trial's log in the RC invocation-log analysis before the 1.0.0 call.

## Feedback (verbatim, captured 2026-07-06)

> My honest take on jigc
>
> Overall: the core idea is genuinely good, the ergonomics tax it. And there's a pleasing irony I want to name first: jigc is a miniature of the exact system we're envisioning — it routes you to where work belongs, forces knowledge (research, decisions, ideas) into findable, structured form, and records provenance. Working inside it for a day was a live demo of "placement guidance" working.
>
> What works well:
>
> - The research → vision grounding chain is the killer feature. grounded-in forcing the vision to cite committed research — and quoting that research back during authoring — is real provenance, not ceremony. The vision I wrote is better because the findings were in view.
> - The commit boundary discipline. One task → one validated, well-formed commit. The rendered conventional commits with promoted files are exactly what a reviewer wants.
> - Validation quality. The one blocker I hit (## heading at schema-reserved depth) told me the file, the line, the rule, and the exact re-run route. Exit-code semantics (3 = fix findings, 1 = operational) are agent-friendly.
> - park-idea with a trigger field is a small brilliant design — ideas don't just get stored, they get a re-entry condition.
>
> What grates — critical, in descending order of impact:
>
> 1. No way to read committed docs through the interface. AGENT.md forbids reading managed files directly, but I found no jigc doc show. On re-compose, only the first grounded research's findings appear — I grounded the vision in three docs and could only see one through jigc; I got away with it because the other two were in my session memory. A fresh session doing a vision revision would be forced to either violate the "never read directly" rule or work blind. That's the biggest hole. Recommendation: jigc doc show <ref>, and render all grounded research on compose, not just the first.
> 2. Instructions aren't state-aware. Every re-compose reprints the full command template verbatim — including "Create it" for the vision doc that already existed (I had to silently skip a printed instruction, which an interface should never make necessary), and including set-slot commands for slots I'd already filled. Recommendation: compose output as a checklist with done/pending marks. This would also cut the output size roughly in half, which matters for an agent-facing tool.
> 3. Slug minting truncates mid-thought. verify-whether-rovo-glean-and-onyx-enforce-per, revise-the-vision-ground-it-in-the-two-new, research:how-should-acme-introduce-a-company — hard caps that cut mid-word/mid-phrase produce IDs that are simultaneously too long to type and too truncated to read. Recommendation: mint at word boundaries with a shorter cap (~5 words), and accept --slug/--id overrides.
> 4. The --task <long-id> repetition. Every one of my ~30 write commands carried the full task id. A sticky active-task context (jigc task use <id>, like a git branch) would remove a third of the typed surface and a whole class of copy-paste errors.
> 5. Advisory noise. advisory · file-state.baseline-adopt — baseline adopted: docs/... appeared on every finalize; I still don't know whether it wanted anything from me. Advisories an agent can't act on are output an agent shouldn't see — or they need one clause saying "no action needed."
>
> Smaller observations: the two-step routing (jigc start "<intent>" → prints list → re-run with --workflow) added little beyond what the SessionStart hook already showed — if intent routing doesn't actually pick, letting the first call take --workflow directly (it does) makes the intent-only form mostly redundant. And --from-file - heredoc input works great for an agent, worth keeping stable.
>
> Net: I'd rather work with jigc than without it — the grounding/provenance model changed the quality of the output, not just its format. Fix the read-access hole and make compose state-aware, and it goes from "good discipline with friction" to "genuinely pleasant."

## Additional findings from the same run (verbatim)

- Setup wrote **invalid Claude Code permission rules** — `.claude/settings.json` rejected two allow entries: `"jigc *"` and `"git add *"` skipped ("Wildcard tool name … not supported in allow rules. An allow pattern must name the scope it widens"). Source confirmed: `crates/cli/adapters/claude-code.yaml` `permit:` list.
- When the user has `bypassPermissions` globally, setup should skip writing permissions — they require context and are obsolete under that setting.
- Permissions may be too restrictive — risk of blocking something someone needs.
- `.jigc/AGENT.md` can improve: the status-code explanation is obsolete if the CLI returns a proper message; describe better what jigc does.
- `form-vision` task should include research.
- Consider pulling files out of `.jigc` into the project (e.g. milestones) so others can pick up the work — team-readiness.
- The agent called many commands in one row producing mass output — optimize to prevent unwanted/unrequired data leaking into context.

## Triage disposition (running — updated as items route)

| # | Finding | Route | Status |
|---|---------|-------|--------|
| A1 | Invalid permission allow rules from setup | fix (adapter profile) | **done 2026-07-06** — permits now `Bash(jigc:*)` / `Bash(git add:*)` |
| A2 | Skip permissions under bypassPermissions | idea: adapter permission model | **parked 2026-07-06** — [ideas/adapter-permission-model.md](../../../ideas/adapter-permission-model.md) |
| A3 | Permissions too restrictive | idea: adapter permission model (rebuts the M36 deny-floor — engage its rationale) | **parked 2026-07-06** — same file as A2 |
| A4 | AGENT.md content (status codes, what jigc does) | fix (pack content) | pending |
| A5 | form-vision should include research | design discussion | pending |
| A6 | Team-ready state externalization | idea | **parked 2026-07-06** — [ideas/team-ready-state-externalization.md](../../../ideas/team-ready-state-externalization.md) |
| A7 | Mass command output / context leak | extends composed-context-token-budget | **parked 2026-07-06** — invocation-output face added to [ideas/composed-context-token-budget.md](../../../ideas/composed-context-token-budget.md) |
| F1 | No read path for committed docs (`jigc doc show`; all grounded research on compose) | idea — strongest next-milestone candidate | **parked 2026-07-06** — [ideas/doc-read-surface.md](../../../ideas/doc-read-surface.md) |
| F2 | Compose not state-aware (checklist done/pending; fold in two-step-routing redundancy) | idea | **parked 2026-07-06** — [ideas/state-aware-compose.md](../../../ideas/state-aware-compose.md) |
| F3 | Slug minting truncates mid-word; no `--slug` override | idea | **parked 2026-07-06** — [ideas/slug-minting-ergonomics.md](../../../ideas/slug-minting-ergonomics.md) |
| F4 | Sticky active-task context (`jigc task use`) | idea | **parked 2026-07-06** — [ideas/sticky-task-context.md](../../../ideas/sticky-task-context.md) |
| F5 | Advisory noise (baseline-adopt with no action cue) | fix (output wording) | **done 2026-07-06** — every route-less advisory now renders `(no action needed)` |
| F6 | `--from-file -` heredoc: works, keep stable | no action (positive signal) | closed |
