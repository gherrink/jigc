Honest pass over the session, including where the fault was mine.

1. What confused me

Whether workflow prose is description or enforcement. record-decision says "with no code to write." I could not tell if that was the situation it fits or a constraint jigc applies. create-gates: adr gates doc types, and nothing in the composed text said code edits would be refused. That ambiguity is the only reason I previewed three workflows.

No workflow fits "record a decision and point the code at it." That shape is common — an ADR usually wants a citation in the code it governs. record-decision excludes the code, decided-task writes a decisions-log rather than a decision record, so I took single-task, which is built for implementing a change. The cost was concrete: single-task grants the changelog gate, so the finalize handed me changelog-recording.gate-granted-unused on a change that by construction has no user-facing behavior. A guaranteed-noise advisory produced by a routing gap.

jigc validate's exit code. The hook that setup generated says, in its own comments: "Keys on the findings, never on the exit code (the exit code is wrong-way-round)." jigc ships a workaround for its own exit status. After reading that I stopped trusting exit codes from the validate family and read stdout instead.

Composed output density. The minted single-task ran ~200 lines and embedded the entire changelog schema — every field, the full batch payload grammar — for a task with no changelog. The heading-depth warning about reserved Setext depths appeared four times, verbatim. For a tool whose stated job is compiling exactly the context a task needs, that spends attention on the one artifact I didn't need.

2. What jigc told me that was wrong or misleading

setup implies it commits the pre-commit hook. It doesn't. The output lists four installed things — bootstrap, allowlist, SessionStart hook, pre-commit hook → .githooks/pre-commit — then install commit → e80e0c2. QUICKSTART reinforces it: "commits its own install... (only the files above, never your working tree)." That commit carried 7 files and .githooks/pre-commit was not among them. The hook sat modified-uncommitted through the whole session and turned up in my finalize's left-out list. A clone gets no drift backstop, and nothing says so.

setup states a principle that config set then breaks. Setup isolates its install "so the install doesn't land in your first feature commit." One command later, jigc config set invocation-log true left .jigc/config/manifest.yaml untracked — and finalize swept it into my docs(ingest) ADR commit. To be precise: --dry-run did disclose it as added, and no documented rule was violated (the carryover gate keys on staged-before-mint, and this was untracked). But the workflow text says finalize commits "the staged set plus the docs it manages" — config is not a doc, so the rule that pulled it in was never stated. A setup step from before the task existed rode a feature commit, which is exactly what setup went out of its way to prevent.

The finalize header asserts the commit before it happens. The rejected run opened with finalize — committing the index; leaving out: and only then the rejection. It reads as done until the next line contradicts it.

Worth crediting, since it sharpens the rest: the carried-staged gate caught two genuinely unrelated changes that would otherwise have ridden a docs commit, and the hook-rejection recovery was exemplary — task intact, staged set preserved, hook output verbatim, re-run line. Both earned their keep.

3. What I looked for and didn't find

- A config read verb. jigc config get and jigc config list both don't exist; the surface is write-only. This bit concretely — I was hardcoding docs/decisions/... into a source comment and could not verify docs-root. I inferred it from QUICKSTART's documented default.
- A way to ask a doctype's schema. The changelog step printed a generated schema block; nothing did that for adr. I learned its slots from workflow prose. jigc describe is one undifferentiated prose dump with no --doctype filter, and grepping it for adr returns a wall of hidden migrate-* workflows first.
- Hooks in the preflight. task validate is explicit that it previews only part of the gate, and --dry-run "stops, changing nothing" — so neither runs your hooks. A dry-run that printed promoted docs/decisions/... could have flagged that my pre-commit hook keys on ^docs/. Instead I found out by attempting the commit.
- --preview where it's needed. jigc workflow <W> --preview was the most useful command in the session. It's absent from QUICKSTART and, more importantly, absent from the router menu — the exact output whose job is helping you choose. The menu says "pick... then re-run with that choice" and never mentions you can read a workflow first.
- A path filter on jigc task diff. I wanted one file; it emits everything vs base, dominated by the 57-line pre-commit hook diff.

4. What I did around jigc rather than through it

The real one: I read an in-flight managed doc off disk. To show you the ADR for sign-off I ran cat .jigc/tasks/**/*ingest-overflow*, then find .jigc/tasks -name '*ingest-overflow*' -exec cat {}. That goes straight around the channel jigc claims to be the sole owner of.

Part of that is mine to own: I never ran jigc doc --help, which jigc --help describes as "Read and write managed docs" — a read verb likely exists. And jigc task diff does surface staged docs; I had grepped its output down to headings and reached for cat instead of re-running it unfiltered.

The part that isn't mine: nothing in the composed workflow ever mentions reading a doc back. Every verb it teaches is create/set-slot/set-field/author, and those echo only set slot ... (450 chars) — a character count, never content. Across six writes I had no confirmation of what actually landed in a slot, and the workflow's own "before you finalize, verify" step offers no way to look. Review-before-commit is an obvious need in a write-only-feeling surface.

git diff --cached -- src/router.ts. The carryover findings name paths but not what changed, and I had to judge relatedness to decide unstage-vs---carry-staged. Available through jigc task diff, but not per-path, so raw git was faster.

Raw git status/log/show for orientation, and Read/cat on src/*.ts, package.json, .githooks/pre-commit — source and hooks, outside jigc's remit. touch .githooks/docs-approved likewise, correctly. git add and git restore --staged were both done on jigc's own instruction.