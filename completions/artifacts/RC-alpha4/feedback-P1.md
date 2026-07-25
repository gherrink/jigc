1. What confused me

The owner-artifact home. This cost the most. The migrate-completion-record guidance printed literal shell: mkdir -p completions/artifacts/<milestone>. Every other managed path in the corpus lives under docs/, so I read that as doc-root-relative and put the file at docs/completions/artifacts/v1.0/source.md with the field set to completions/artifacts/v1.0/source.md. Two consecutive finalize blocks to learn it's repo-root-relative and literal: first names no file under the repository, then after I "corrected" it, is not under the owned artifact home. The ladder text was right; it just never said which root it was relative to, in a corpus where the surrounding convention points the other way.

Two code-anchor fields, two different accepted shapes. ADR cites-code took a bare path (apps/audit-api/index.js) and validated clean. I generalised, and arch-doc implemented-by turned out to want path#symbol. Both are typed code-anchor in jigc doc schema, so the schema output gave me no way to tell them apart.

"Advisory" is scope-dependent, not severity. jigc validate said 7 finding(s) — report-only at store scope (exit 0); each gates nowhere. Later, structurally similar advisories said these gate at jigc task validate / jigc task finalize. So the same word covers both "ignore this forever" and "this will block your next task on this doc." I only worked that out by comparing the two trailer lines.

--preview. Orientation lists Preview: jigc workflow <id> --preview — read a workflow's step text without minting a task as a general affordance, directly above a Run: line for ingest-existing. Doing exactly that returned mints no task, so there is nothing to preview. The preview verb only works on the workflows you least need to preview — the ones that mint.

2. What jigc told me that was wrong or misleading

jigc task validate does not preview what finalize gates on. It's described as "previews the findings finalize will gate on, without committing anything." On the completion-record task it returned only a benign staged-copy advisory. finalize --approve then blocked on owner-artifact.present. It did this twice, across both wrong paths. This is the one I'd call a broken contract rather than a rough edge — it's the verb whose entire purpose is to stop you finding out at finalize.

title-names-symbol misdescribes what it found. For a component titled "WordPress.org public APIs" anchored to CmsGlobalClient, it reported: names symbol(s) ["WordPress", "APIs"] but its anchor implements CmsGlobalClient — the heading still names a renamed/removed symbol. Nothing was renamed or removed. APIs is not a symbol. It's a capitalized-token heuristic, and phrasing it as rename/removal detection sent me looking for a code change that didn't exist.

Setup reported the wrong hook path. Output said pre-commit hook → .git/hooks/pre-commit. This repo has core.hooksPath=.husky/_. jigc actually did the right thing — installed to .husky/_/pre-commit and chained . "$(dirname "$0")/h" so lint-staged still runs — but the message named a path that doesn't exist here, so I checked, found nothing, and briefly concluded the backstop was inert.

The drift hook cried wolf. Four consecutive commits printed jigc: doc<->code drift detected in committed docs. The findings were the three cosmetic title-names-symbol advisories. The hook message doesn't distinguish "your docs describe code that no longer exists" from "a heading contains a capital letter."

migrate-corpus had no route for its own half-failure. The husky hook rejected its commit. The schema-version write had already landed and was staged. Re-running reported 0 would migrate, 1 already current — correct, and also a dead end: the verb considered itself done while nothing was committed. No finding, no route, no --retry-commit.

3. What I looked for and didn't find

- A way to clear an optional field. implemented-by blocked finalize; there's set-field but no unset/--clear, and I couldn't set it empty. Worked around with remove-item + add-item + re-authoring the description by hand.
- Item reordering. Consequence of the above — the re-added component lands at the end of the section. Harmless here because it was already last; there's no recourse if it hadn't been.
- Idempotent re-author. doc author appends to repeatables once a doc is committed, so a payload is not a declaration of intended state. Getting one item wrong in a 34-item PRD would have meant surgical patching, not re-running the corrected file. The same-path-migration blank-skeleton carve-out is the one exception and it's narrow.
- A dry-run for doc author — payload shape and anchor resolution checked before staging.
- A task-scope diff — "what will this task change against the committed store." Migrations render a fidelity diff at finalize; non-migration tasks appeared to have no equivalent.

One of these was my fault, not jigc's: I scraped task ids with grep -o and got a mangled two-line id, because task list puts the id and its [workflow] label on one line. --format json exists globally and I never tried it there.

4. What I did around jigc rather than through it

- git mv .planning/ROADMAP.md docs/roadmap.md — raw git on a doc jigc had just classified as conformant-but-misplaced. Sanctioned (jigc says it never auto-moves and routes to a human) but still a manual move of a to-be-managed doc.
- A hand-written git commit (a9731ef) for migrate-corpus's writes. Because of the half-failure above, the first commit of a managed doc did not go through finalize. This is the one I'm least comfortable with — I staged and committed a jigc-owned schema-version stamp myself.
- git rm -r .planning (202 files), then find -type d -empty -delete. The deletion was raw; the commit went through a jigc task, which picked up the staged removals at finalize.
- git rm --cached + git mv to relocate the owner-artifact between the two finalize blocks.
- Read-out → external patch → set-slot write-back, twice (the roadmap's "phases 3, 9, and 11" fix and its .planning pointer). Nominally through jigc's verbs, but the actual editing was a Python string replace on a temp file, because there's no partial-edit verb for slot prose.
- Direct Write to CLAUDE.md. Unmanaged, so legitimate — but it carries jigc's @.jigc/AGENT.md bootstrap reference, which I preserved by hand. Nothing enforces that; a careless rewrite silently unwires the adapter, and only jigc setup would restore it.

I read .planning/** directly throughout — those were unmanaged foreign files, which is the intended path.

Not exercised, so no opinion: rename, unmanage, relocate, upgrade's remediation path, milestone, uninstall.
