# CLI surface corpus — round 2b (jigc 1.0.0-rc.7)

Captured 2026-07-17. Complements round 1 (cli-surface-corpus.md); no re-capture of its sections.

# Part 1 — the remaining composed workflows

## $ jigc start --workflow architecture-documentation "document the payments module architecture"   # fresh setup repo

````
task minted: document-the-payments-module-architecture

Document this part of the system as living architecture. The intent is:
document the payments module architecture

Create the arch-doc, then author its overview:

Run: `jigc doc create arch-doc --title <TITLE> --task document-the-payments-module-architecture`
<<author: arch-doc#overview>>

Cite the decisions this part of the system rests on, so finalize can check each
citation still resolves. `cites` is a list — author **all** of them in ONE
`set-field` as a bracketed list (a second `set-field` replaces, not appends, so
repeated single-value calls would keep only the last). One ADR is the list of one:

jigc doc set-field arch-doc:<slug>#cites --value '[adr:<slug-a>, adr:<slug-b>]' --task document-the-payments-module-architecture

Committed-first ordering: every cited ADR must already be committed before this
task — arch-doc's `allows-create` cannot mint an ADR in-task, so a `cites`
pointing at an uncommitted or absent decision has nothing to resolve against and
a dangling `cites` blocks finalize forever. Cite only committed ADRs; record and
commit the decision first, then cite it here.

Then add one component per architectural piece. For each, mint the item, write
its responsibility in prose, and anchor it to the code that implements it (a
`path#symbol` finalize re-checks against the working tree):

jigc doc add-item arch-doc:<slug>#components --title "<component name>" --task document-the-payments-module-architecture
jigc doc set-slot arch-doc:<slug>#components/<id>/description --from-file - --task document-the-payments-module-architecture
jigc doc set-field arch-doc:<slug>#components/<id>/implemented-by --value <path>#<symbol> --task document-the-payments-module-architecture

Or make it one call — the batch alternative to the whole sequence above (run it
INSTEAD of the create + per-leaf verbs, never after them — an already-staged doc
rejects a second create): the batch verb reads one declarative payload from
stdin (grammar: `jigc doc author --help`) and places every leaf in a single
write; the payload's `title:` mints the slug exactly like the create above. The
`cites` one-list rule and the committed-first ordering above bind unchanged:

jigc doc author arch-doc --from-file - --task document-the-payments-module-architecture

Validate and commit the task as one logical commit. Make sure your code edits
are staged (`git add`) first — finalize commits only the staged set plus the
docs it manages; unstaged edits and untracked files are left out, and with
nothing staged over a dirty tree it refuses. Anything still staged from BEFORE
this task was minted makes finalize refuse too (one blocking finding per
carried path): unstage it, or pass `--carry-staged` to declare the carryover
deliberate.

To see what's left before committing, run `jigc task validate document-the-payments-module-architecture` — it
previews the findings finalize will gate on, without committing anything.

Run: `jigc task finalize document-the-payments-module-architecture`
resume: `jigc start --task document-the-payments-module-architecture`   — re-composes this workflow if context is lost
what's-left: `jigc task validate document-the-payments-module-architecture`   — previews the findings finalize will gate on
task scope: `jigc doc` writes default to the single active task; `--task document-the-payments-module-architecture` is the explicit override and wins when several are active
create-gates: arch-doc
— jigc · run `jigc start` for orientation; all writes through `jigc`.
````

## $ jigc start --workflow plan "plan the notification service"   # fresh setup repo

````
task minted: plan-the-notification-service

Draft the spec for this change. The intent is:
plan the notification service

Create the spec, then author its slots:

Run: `jigc doc create spec --title <TITLE> --task plan-the-notification-service`
<<author: spec#goal>>
<<author: spec#context>>

State the non-goals too — what this change will explicitly NOT do bounds the work
as sharply as what it will; fold them into the `context` slot. Name the concrete
files or modules the change is expected to touch when they are already known, so
a later reader orients without rediscovering them.

Then add one criterion per thing the change must satisfy, each testably phrased.
For each, mint the item and write its statement in prose:

jigc doc add-item spec:<slug>#criteria --title "<criterion>" --task plan-the-notification-service
jigc doc set-slot spec:<slug>#criteria/<id>/statement --from-file - --task plan-the-notification-service

Or make it one call — the batch alternative to the whole sequence above (run it
INSTEAD of the create + per-leaf verbs, never after them — an already-staged doc
rejects a second create): the batch verb reads one declarative payload from
stdin (grammar: `jigc doc author --help`) and places every leaf in a single
write; the payload's `title:` mints the slug exactly like the create above:

jigc doc author spec --from-file - --task plan-the-notification-service

Validate and commit the task as one logical commit. Make sure your code edits
are staged (`git add`) first — finalize commits only the staged set plus the
docs it manages; unstaged edits and untracked files are left out, and with
nothing staged over a dirty tree it refuses. Anything still staged from BEFORE
this task was minted makes finalize refuse too (one blocking finding per
carried path): unstage it, or pass `--carry-staged` to declare the carryover
deliberate.

To see what's left before committing, run `jigc task validate plan-the-notification-service` — it
previews the findings finalize will gate on, without committing anything.

Run: `jigc task finalize plan-the-notification-service`
resume: `jigc start --task plan-the-notification-service`   — re-composes this workflow if context is lost
what's-left: `jigc task validate plan-the-notification-service`   — previews the findings finalize will gate on
task scope: `jigc doc` writes default to the single active task; `--task plan-the-notification-service` is the explicit override and wins when several are active
create-gates: spec
— jigc · run `jigc start` for orientation; all writes through `jigc`.
````

## $ jigc start --workflow project-setup "set up jigc for this project"   # fresh setup repo

````
task minted: set-up-jigc-for-this

Develop the idea into a product direction. The idea is:
set up jigc for this project

Reason about the vision, the requirements it implies, and the context that
shapes it — audience, constraints, and what is out of scope — before authoring
the prd below.

Author every managed document through `jigc` — it owns placement, structure, and
cross-references. Files you write straight to disk are unmanaged: outside `jigc`'s
tracking, validation, and commit — this is a convention, not a sandbox, so a
hand-placed doc is simply invisible to the project.

Create the prd, then author its slots:

Run: `jigc doc create prd --title <TITLE> --task set-up-jigc-for-this`
<<author: brief#vision>>
<<author: brief#context>>

Then add one requirement per thing the product must do. For each, mint the item
and write its requirement in prose:

jigc doc add-item prd:<slug>#requirements --title "<requirement>" --task set-up-jigc-for-this
jigc doc set-slot prd:<slug>#requirements/<id>/statement --from-file - --task set-up-jigc-for-this

Or make it one call — the batch alternative to the whole sequence above (run it
INSTEAD of the create + per-leaf verbs, never after them — an already-staged doc
rejects a second create): the batch verb reads one declarative payload from
stdin (grammar: `jigc doc author --help`) and places every leaf in a single
write; the payload's `title:` mints the slug exactly like the create above:

jigc doc author prd --from-file - --task set-up-jigc-for-this

Validate and commit the task as one logical commit. Make sure your code edits
are staged (`git add`) first — finalize commits only the staged set plus the
docs it manages; unstaged edits and untracked files are left out, and with
nothing staged over a dirty tree it refuses. Anything still staged from BEFORE
this task was minted makes finalize refuse too (one blocking finding per
carried path): unstage it, or pass `--carry-staged` to declare the carryover
deliberate.

To see what's left before committing, run `jigc task validate set-up-jigc-for-this` — it
previews the findings finalize will gate on, without committing anything.

Run: `jigc task finalize set-up-jigc-for-this`
resume: `jigc start --task set-up-jigc-for-this`   — re-composes this workflow if context is lost
what's-left: `jigc task validate set-up-jigc-for-this`   — previews the findings finalize will gate on
task scope: `jigc doc` writes default to the single active task; `--task set-up-jigc-for-this` is the explicit override and wins when several are active
create-gates: prd
— jigc · run `jigc start` for orientation; all writes through `jigc`.
````

## $ jigc start --workflow do-research "research job queue libraries for the worker"   # fresh setup repo

````
task minted: research-job-queue-libraries

Record the investigation on a fresh research doc. Create it — the slug is minted
from the title you give (the question this investigation answers):

Run: `jigc doc create research --title <TITLE> --task research-job-queue-libraries`

Author the three prose slots — the question it set out to answer, what the evidence
showed, and where that evidence came from. The `date` is CLI-set on create:

jigc doc set-slot research:<slug>#question --from-file - --task research-job-queue-libraries
jigc doc set-slot research:<slug>#findings --from-file - --task research-job-queue-libraries
jigc doc set-slot research:<slug>#sources --from-file - --task research-job-queue-libraries

Or make it one call — the batch alternative to the whole sequence above (run it
INSTEAD of the create + per-leaf verbs, never after them — an already-staged doc
rejects a second create): the batch verb reads one declarative payload from
stdin (grammar: `jigc doc author --help`) and places every leaf in a single
write; the payload's `title:` mints the slug exactly like the create above:

jigc doc author research --from-file - --task research-job-queue-libraries

Land the change as exactly one logical commit. finalize commits the git index —
`git add` your code edits before you finalize, because it commits only what you
have staged, plus the docs it manages. Unstaged edits and untracked files are
left out of the commit; with nothing staged over a dirty tree, finalize
refuses. Anything still staged from BEFORE this task was minted makes finalize
refuse too (one blocking finding per carried path): unstage it, or pass
`--carry-staged` to declare the carryover deliberate.

finalize renders the commit doc; it does not fill it, so set its header and prose
first. Inside slot prose, headings must sit at `####` depth or deeper —
`##`/`###` are schema-reserved, and Setext headings are rejected.

Set the Conventional-Commits type:

Run: `jigc doc set-field commit:research-job-queue-libraries#type --value <TYPE> --task research-job-queue-libraries`
The `type` value is one of: feat | fix | docs | style | refactor | perf | test | build | ci | chore | revert

Set the scope — the area this change touches:

Run: `jigc doc set-field commit:research-job-queue-libraries#scope --value <SCOPE> --task research-job-queue-libraries`

Set the subject line — it renders as `<type>(<scope>): <summary>`, so write the
summary without a type or scope prefix of its own (the `type` field already
carries it):

Run: `jigc doc set-slot commit:research-job-queue-libraries#summary --from-file - --task research-job-queue-libraries`

Set the body — why this change:

Run: `jigc doc set-slot commit:research-job-queue-libraries#body --from-file - --task research-job-queue-libraries`

To see what's left before committing, run `jigc task validate research-job-queue-libraries` — it
previews the findings finalize will gate on, without committing anything.

Then validate and commit:

Run: `jigc task finalize research-job-queue-libraries`
resume: `jigc start --task research-job-queue-libraries`   — re-composes this workflow if context is lost
what's-left: `jigc task validate research-job-queue-libraries`   — previews the findings finalize will gate on
task scope: `jigc doc` writes default to the single active task; `--task research-job-queue-libraries` is the explicit override and wins when several are active
create-gates: research
— jigc · run `jigc start` for orientation; all writes through `jigc`.
````

## $ jigc start --workflow form-vision "form the product vision"   # fresh setup repo

````
task minted: form-the-product-vision

No committed research exists yet — a vision grounds in the research it cites, so consider running `do-research` first to gather it. (Advisory, not required: a vision may ground in experience, and grounding can take several research rounds.)
Form the project vision on the `vision` singleton. Create it — the slug is fixed
(`vision`), and its H1 reads `# Vision` regardless of the title:

Run: `jigc doc create vision --title Vision --task form-the-product-vision`

Ground the vision in the committed research it rests on. Set `grounded-in` to the
research this vision is formed from — pass ALL of it in ONE call as an inline list
(the multi-value path). Every element is recorded and resolved at finalize:

jigc doc set-field vision:vision#meta/grounded-in --value "[research:<slug>, …]" --task form-the-product-vision

Now RE-COMPOSE so the grounding research comes into view — the findings below
resolve only after `grounded-in` is set:

jigc start --task form-the-product-vision

The findings of ALL grounding research appear here for reference (nothing appears
until you set `grounded-in` and re-compose; with multiple grounding sources each
one renders under its own labelled `> **type:slug**` blockquote):


Author the three prose slots — the core claim, the invariants it holds, and the
questions it leaves open:

jigc doc set-slot vision:vision#thesis --from-file - --task form-the-product-vision
jigc doc set-slot vision:vision#invariants --from-file - --task form-the-product-vision
jigc doc set-slot vision:vision#open-questions --from-file - --task form-the-product-vision

Land the change as exactly one logical commit. finalize commits the git index —
`git add` your code edits before you finalize, because it commits only what you
have staged, plus the docs it manages. Unstaged edits and untracked files are
left out of the commit; with nothing staged over a dirty tree, finalize
refuses. Anything still staged from BEFORE this task was minted makes finalize
refuse too (one blocking finding per carried path): unstage it, or pass
`--carry-staged` to declare the carryover deliberate.

finalize renders the commit doc; it does not fill it, so set its header and prose
first. Inside slot prose, headings must sit at `####` depth or deeper —
`##`/`###` are schema-reserved, and Setext headings are rejected.

Set the Conventional-Commits type:

Run: `jigc doc set-field commit:form-the-product-vision#type --value <TYPE> --task form-the-product-vision`
The `type` value is one of: feat | fix | docs | style | refactor | perf | test | build | ci | chore | revert

Set the scope — the area this change touches:

Run: `jigc doc set-field commit:form-the-product-vision#scope --value <SCOPE> --task form-the-product-vision`

Set the subject line — it renders as `<type>(<scope>): <summary>`, so write the
summary without a type or scope prefix of its own (the `type` field already
carries it):

Run: `jigc doc set-slot commit:form-the-product-vision#summary --from-file - --task form-the-product-vision`

Set the body — why this change:

Run: `jigc doc set-slot commit:form-the-product-vision#body --from-file - --task form-the-product-vision`

To see what's left before committing, run `jigc task validate form-the-product-vision` — it
previews the findings finalize will gate on, without committing anything.

Then validate and commit:

Run: `jigc task finalize form-the-product-vision`
resume: `jigc start --task form-the-product-vision`   — re-composes this workflow if context is lost
what's-left: `jigc task validate form-the-product-vision`   — previews the findings finalize will gate on
task scope: `jigc doc` writes default to the single active task; `--task form-the-product-vision` is the explicit override and wins when several are active
create-gates: vision
— jigc · run `jigc start` for orientation; all writes through `jigc`.
````

## $ jigc start --workflow park-idea "park the offline-mode idea"   # fresh setup repo

````
task minted: park-the-offline-mode-idea

Park the shaped direction on a fresh idea doc. Create it — the slug is minted from
the title you give (a short name for the direction):

Run: `jigc doc create idea --title <TITLE> --task park-the-offline-mode-idea`

Set the `trigger` — the condition that would bring this idea back into scope — then
author the one `description` slot (the direction, and why it might be worth doing).
The `date` is CLI-set on create:

jigc doc set-field idea:<slug>#trigger --value "<what would resurface it>" --task park-the-offline-mode-idea
jigc doc set-slot idea:<slug>#description --from-file - --task park-the-offline-mode-idea

Or make it one call — the batch alternative to the whole sequence above (run it
INSTEAD of the create + per-leaf verbs, never after them — an already-staged doc
rejects a second create): the batch verb reads one declarative payload from
stdin (grammar: `jigc doc author --help`) and places every leaf in a single
write; the payload's `title:` mints the slug exactly like the create above:

jigc doc author idea --from-file - --task park-the-offline-mode-idea

Land the change as exactly one logical commit. finalize commits the git index —
`git add` your code edits before you finalize, because it commits only what you
have staged, plus the docs it manages. Unstaged edits and untracked files are
left out of the commit; with nothing staged over a dirty tree, finalize
refuses. Anything still staged from BEFORE this task was minted makes finalize
refuse too (one blocking finding per carried path): unstage it, or pass
`--carry-staged` to declare the carryover deliberate.

finalize renders the commit doc; it does not fill it, so set its header and prose
first. Inside slot prose, headings must sit at `####` depth or deeper —
`##`/`###` are schema-reserved, and Setext headings are rejected.

Set the Conventional-Commits type:

Run: `jigc doc set-field commit:park-the-offline-mode-idea#type --value <TYPE> --task park-the-offline-mode-idea`
The `type` value is one of: feat | fix | docs | style | refactor | perf | test | build | ci | chore | revert

Set the scope — the area this change touches:

Run: `jigc doc set-field commit:park-the-offline-mode-idea#scope --value <SCOPE> --task park-the-offline-mode-idea`

Set the subject line — it renders as `<type>(<scope>): <summary>`, so write the
summary without a type or scope prefix of its own (the `type` field already
carries it):

Run: `jigc doc set-slot commit:park-the-offline-mode-idea#summary --from-file - --task park-the-offline-mode-idea`

Set the body — why this change:

Run: `jigc doc set-slot commit:park-the-offline-mode-idea#body --from-file - --task park-the-offline-mode-idea`

To see what's left before committing, run `jigc task validate park-the-offline-mode-idea` — it
previews the findings finalize will gate on, without committing anything.

Then validate and commit:

Run: `jigc task finalize park-the-offline-mode-idea`
resume: `jigc start --task park-the-offline-mode-idea`   — re-composes this workflow if context is lost
what's-left: `jigc task validate park-the-offline-mode-idea`   — previews the findings finalize will gate on
task scope: `jigc doc` writes default to the single active task; `--task park-the-offline-mode-idea` is the explicit override and wins when several are active
create-gates: idea
— jigc · run `jigc start` for orientation; all writes through `jigc`.
````

## $ jigc start --workflow record-change "record the cache eviction change"   # fresh setup repo

````
task minted: record-the-cache-eviction-change

Record the change on the project changelog. Create-or-update the changelog
singleton — safe whether or not it already exists (an existing committed changelog
is copied in for append):

Run: `jigc doc create changelog --title Changelog --task record-the-cache-eviction-change`

Then author the change. To cut a release, mint a release item (its id is minted
from the version title you give — e.g. `1.0.0` mints `100` — and its `date` is
stamped on create). `add-item` PRINTS the new item's address; use that printed
address verbatim for every follow-up verb — never re-spell the version string:

jigc doc add-item changelog:changelog#releases --title "<version>" --task record-the-cache-eviction-change
#   → prints e.g. changelog:changelog#releases/100  (call this <release-addr>)
jigc doc set-field <release-addr>/link --value "<diff-url>" --task record-the-cache-eviction-change

Under that release, add one nested change-group per category and author its notes
(one bullet per change). The category is the group's id (added / changed /
deprecated / removed / fixed / security). Again, drive the printed address verbatim:

jigc doc add-item <release-addr>/changes --title "<category>" --task record-the-cache-eviction-change
#   → prints e.g. changelog:changelog#releases/100/changes/added  (call this <group-addr>)
jigc doc set-slot <group-addr>/notes --from-file - --task record-the-cache-eviction-change

For a change that is not yet cut into a release, add the change-group under the
staged section instead:

jigc doc add-item changelog:changelog#unreleased-changes --title "<category>" --task record-the-cache-eviction-change
#   → prints e.g. changelog:changelog#unreleased-changes/added  (call this <group-addr>)
jigc doc set-slot <group-addr>/notes --from-file - --task record-the-cache-eviction-change

Or make it one call — the batch alternative to the whole sequence above (run it
INSTEAD of the create + per-entry verbs, never after them — an already-staged doc
rejects a second create): the batch verb reads one declarative payload from
stdin (grammar: `jigc doc author --help`) and places every entry in a single
write, no printed-address dance. Over an already-committed changelog it copies
the committed doc in and appends — existing releases are untouched:

jigc doc author changelog --from-file - --task record-the-cache-eviction-change

Validate and commit the task as one logical commit. Make sure your code edits
are staged (`git add`) first — finalize commits only the staged set plus the
docs it manages; unstaged edits and untracked files are left out, and with
nothing staged over a dirty tree it refuses. Anything still staged from BEFORE
this task was minted makes finalize refuse too (one blocking finding per
carried path): unstage it, or pass `--carry-staged` to declare the carryover
deliberate.

To see what's left before committing, run `jigc task validate record-the-cache-eviction-change` — it
previews the findings finalize will gate on, without committing anything.

Run: `jigc task finalize record-the-cache-eviction-change`
resume: `jigc start --task record-the-cache-eviction-change`   — re-composes this workflow if context is lost
what's-left: `jigc task validate record-the-cache-eviction-change`   — previews the findings finalize will gate on
task scope: `jigc doc` writes default to the single active task; `--task record-the-cache-eviction-change` is the explicit override and wins when several are active
create-gates: changelog
— jigc · run `jigc start` for orientation; all writes through `jigc`.
````

## $ jigc start --workflow ingest-existing "ingest the existing docs"   # fresh setup repo

````
Scan the repo for documents jigc can manage:

Run: `jigc ingest`

This discovers candidate documents across the repo, classifies each against the
managed doc-types, and reports a triage verdict per file (adoptable /
needs-reconcile / unmanaged) in deterministic order.

Read the triage verdicts:

- adoptable docs are conformant at their location — jigc has adopted them
  (indexed + baselined, no file moved).
- needs-reconcile docs each carry a routed finding — a non-conformant near-miss
  or a conformant doc at the wrong location. Hand each to a human to resolve;
  jigc never auto-migrates or relocates.
- unmanaged docs parse against no schema — leave them untouched.

Do not rewrite any prose. Route the needs-reconcile findings; adopt nothing that
was not schema-checked.
— jigc · run `jigc start` for orientation; all writes through `jigc`.
````

## $ jigc start --workflow planning "plan the next milestone"   # fresh setup repo

````
task minted: plan-the-next-milestone

Scope the milestone:
plan the next milestone

Restate, from the full-product roadmap, what this milestone proves and its
risk-first place in the sequence — the runnable slice it must deliver and the
worked-example flow(s) that will demonstrate it. That done-picture is the bar
everything below is measured against.

Verify the baseline it builds on, don't trust the roadmap's "shipped" prose:
exercise the real binary to map what is genuinely built versus stubbed, deferred,
or latent. A "design-locked / planning-light" framing must RAISE that audit
intensity, never relax it. Pull this milestone's deferred topics from the
deferral ledger first, and fold them into the done-picture and the gap list.

These phases are a walk, not a liturgy. If you have already scouted this terrain —
baseline verified against the running binary, gaps already mapped — then work them
as a checklist: confirm each phase and spend your effort on what is genuinely still
open, rather than re-performing orientation you have done. Covered means verified,
never assumed. The Settle gate still binds either way: the human owns it, and no
amount of prior scouting self-serves it.

Detect the gaps — a forward-looking, adversarial pass over the milestone's
done-picture against the assembled product: what is missing or unfit to build it
cleanly. Look across decisions still open, design docs that don't exist or have
drifted, doctype schemas no workflow has yet driven, and engine/CLI surface the
milestone assumes.

Reuse is a claim, not a fact: "this already exists, the milestone just reuses it"
is the most dangerous miss — spike the new shape against the real binary before
trusting it, including its cold-start (empty, freshly-created) state. This is
judgment work, not a checklist to tick: produce a ranked gap list, each gap
concrete enough to act on.

Checkpoint: settle
Settle the gaps — the human-in-the-loop gate. For each blocking gap, resolve it
through the right loop and record it: decisions via the design workflow, docs and
doctype schemas elaborated where this milestone's workflows now drive them. Surface
the genuine scope and judgment calls — must-settle-now versus defer-to-a-later
milestone, and whether a gap warrants building machinery at all — rather than
silently deciding them. The human owns this gate.

A gap consciously deferred is logged on the deferral ledger, keyed to the
milestone that will own it. The design Settle writes is itself a claim: pin the
scope of every check it introduces, and spike every acceptance flow — its
invocation commands included — against the real engine before locking it.

Review what Settle produced — an independent pass. A reader who did NOT author the
decisions, docs, or doctype schemas reads them adversarially for coherence, gaps
the first pass missed, over- or under-design, and conflicts with the locked
invariants. Findings are baked back into the docs before anything is decomposed —
a settled doc that survives an adversarial read is a sound basis for increments;
an unreviewed one propagates its flaws into every increment cut from it. The human
owns accepting or rejecting each finding.

Decompose the milestone into ordered, risk-first increments — straight from the
now-reviewed scoped done-picture. Each increment carries its Deliverable, Grouped
scope, and Proves, and names the design docs it builds against — the form the
increment workflow consumes. Keep the cut minimal: the smallest sequence that
delivers the milestone, no creep into a later one. The spine stays linear — each
increment builds on the last.

Record the milestone you decomposed on the running roadmap. Create-or-update the
roadmap singleton — safe whether or not it already exists (an existing committed
roadmap is copied in for append):

Run: `jigc doc create roadmap --title Roadmap --task plan-the-next-milestone`

Then mint this milestone's entry and author it — its `proves` and the prose
`decomposition` of its increments (one level: each increment's deliverable and the
tasks that build it, as prose). The item id is minted from the title you give:

jigc doc add-item roadmap:roadmap#milestones --title "<milestone>" --task plan-the-next-milestone
jigc doc set-slot roadmap:roadmap#milestones/<id>/proves --from-file - --task plan-the-next-milestone
jigc doc set-slot roadmap:roadmap#milestones/<id>/decomposition --from-file - --task plan-the-next-milestone

Or make it one call — the batch alternative to the whole sequence above (run it
INSTEAD of the create + per-entry verbs, never after them — an already-staged doc
rejects a second create): the batch verb reads one declarative payload from
stdin (grammar: `jigc doc author --help`) and places every entry in a single
write. Over an already-committed roadmap it copies the committed doc in and
appends the new milestone — existing entries are untouched:

jigc doc author roadmap --from-file - --task plan-the-next-milestone

Log each gap Settle consciously deferred on the running deferral ledger.
Create-or-update the ledger singleton — safe whether or not it already exists (an
existing committed ledger is copied in for append):

Run: `jigc doc create deferral-ledger --title Deferral-Ledger --task plan-the-next-milestone`

Then, for each deferral, mint an entry and author it: its `kind` (Decision for a
deferred decision, Idea for a parked idea), the `trigger` milestone that resurfaces
it, and the `body` prose for what is owed and why. The `date` is CLI-set on create:

jigc doc add-item deferral-ledger:deferral-ledger#entries --title "<what is owed>" --task plan-the-next-milestone
jigc doc set-field deferral-ledger:deferral-ledger#entries/<id>/kind --value <Decision|Idea> --task plan-the-next-milestone
jigc doc set-field deferral-ledger:deferral-ledger#entries/<id>/trigger --value <milestone> --task plan-the-next-milestone
jigc doc set-slot deferral-ledger:deferral-ledger#entries/<id>/body --from-file - --task plan-the-next-milestone

Or make it one call — the batch alternative to the whole sequence above (run it
INSTEAD of the create + per-entry verbs, never after them — an already-staged doc
rejects a second create): the batch verb reads one declarative payload from
stdin (grammar: `jigc doc author --help`) and places every entry in a single
write. Over an already-committed ledger it copies the committed doc in and
appends — existing entries are untouched:

jigc doc author deferral-ledger --from-file - --task plan-the-next-milestone

Record each decision Settle made on the running decisions log. Create-or-update the
log singleton — safe whether or not it already exists (an existing committed log is
copied in for append):

Run: `jigc doc create decisions-log --title Decisions-Log --task plan-the-next-milestone`

Then, for each decision made, mint an entry and author its `why` — the reasoning,
in a sentence or two. The `date` is CLI-set on create:

jigc doc add-item decisions-log:decisions-log#entries --title "<the decision>" --task plan-the-next-milestone
jigc doc set-slot decisions-log:decisions-log#entries/<id>/why --from-file - --task plan-the-next-milestone

Or make it one call — the batch alternative to the whole sequence above (run it
INSTEAD of the create + per-entry verbs, never after them — an already-staged doc
rejects a second create): the batch verb reads one declarative payload from
stdin (grammar: `jigc doc author --help`) and places every entry in a single
write. Over an already-committed log it copies the committed doc in and appends —
existing entries are untouched:

jigc doc author decisions-log --from-file - --task plan-the-next-milestone

Land the change as exactly one logical commit. finalize commits the git index —
`git add` your code edits before you finalize, because it commits only what you
have staged, plus the docs it manages. Unstaged edits and untracked files are
left out of the commit; with nothing staged over a dirty tree, finalize
refuses. Anything still staged from BEFORE this task was minted makes finalize
refuse too (one blocking finding per carried path): unstage it, or pass
`--carry-staged` to declare the carryover deliberate.

finalize renders the commit doc; it does not fill it, so set its header and prose
first. Inside slot prose, headings must sit at `####` depth or deeper —
`##`/`###` are schema-reserved, and Setext headings are rejected.

Set the Conventional-Commits type:

Run: `jigc doc set-field commit:plan-the-next-milestone#type --value <TYPE> --task plan-the-next-milestone`
The `type` value is one of: feat | fix | docs | style | refactor | perf | test | build | ci | chore | revert

Set the scope — the area this change touches:

Run: `jigc doc set-field commit:plan-the-next-milestone#scope --value <SCOPE> --task plan-the-next-milestone`

Set the subject line — it renders as `<type>(<scope>): <summary>`, so write the
summary without a type or scope prefix of its own (the `type` field already
carries it):

Run: `jigc doc set-slot commit:plan-the-next-milestone#summary --from-file - --task plan-the-next-milestone`

Set the body — why this change:

Run: `jigc doc set-slot commit:plan-the-next-milestone#body --from-file - --task plan-the-next-milestone`

To see what's left before committing, run `jigc task validate plan-the-next-milestone` — it
previews the findings finalize will gate on, without committing anything.

Then validate and commit:

Run: `jigc task finalize plan-the-next-milestone`
resume: `jigc start --task plan-the-next-milestone`   — re-composes this workflow if context is lost
what's-left: `jigc task validate plan-the-next-milestone`   — previews the findings finalize will gate on
task scope: `jigc doc` writes default to the single active task; `--task plan-the-next-milestone` is the explicit override and wins when several are active
create-gates: roadmap, deferral-ledger, decisions-log
— jigc · run `jigc start` for orientation; all writes through `jigc`.
````

## $ jigc start --workflow completion "complete the current milestone"   # fresh setup repo

````
task minted: complete-the-current-milestone

Audit the assembled milestone — a fresh, adversarial pass over the finished whole,
not a re-run of the per-increment validate. The milestone you are closing:
complete the current milestone

Two independent passes: a read-only code review of the whole milestone diff for
correctness, invariant violations, and scope honesty (inert/dead features,
stubs-as-done, tautological tests, creep); and an end-to-end run of the milestone's
worked-example flows over the real binary in throwaway repos — not trusting the
builders' own tests. Produce ranked findings with file:line evidence and a
green/red verdict on whether the deliverable genuinely holds.

This is judgment work, not a checklist to tick. The verdict and the gap-detection
stay pure agent prose — a step that emits a count threshold or a checklist the agent
must satisfy has hollowed the judgment. The genuine, independent audit passes are an
orchestration-level responsibility above this single-agent spine; the spine records
their verdict and findings, it does not certify them.

Checkpoint: triage-gate
Triage each finding — verify it is real (reproduce it as a red test or a source
trace, because an audit is a hypothesis generator, not an oracle) and severity-rank
it. The default is fix-now: a confirmed, bounded finding is repaired immediately,
not parked behind a permission ask.

The human gate fires for exactly two cases, never every finding: a fix too big for
the completion-fix lane (recorded as its own task/increment, keyed to the milestone
that will own it) and a contested finding (the "fix" would revise a settled decision
or change intended behavior). Surface those two; everything else is a tested commit
the human reviews after. The human owns this gate.

Checkpoint: fix-rounds-exhausted
For each blocking finding, run a dev-workflow fix task — a red that fails because
of the defect, a minimal green, the gate, one commit — then re-validate. This is
bounded to three rounds. If blocking findings remain after the fix-round cap,
stop and surface it: thrash is a signal for a human, not for another round.
Advisories are surfaced, not gated.

Re-verify — the project's full gate green (its own format, lint, test, and build
gate, whatever toolchain it uses; do not assume a particular one) AND re-run the
affected audit slice to confirm each closed finding is actually closed. Loop to
clean: fixes regress and audits miss, so repeat the audit-relevant checks until
both the slice and the gate are green.

The milestone is done when every confirmed finding is closed (or consciously
tracked), the gate is green, and the deliverable holds under the re-run end-to-end
flows.

Record the completion audit on the per-milestone completion-record. Create it fresh
— the slug is minted from the title you give (this milestone):

Run: `jigc doc create completion-record --title <TITLE> --task complete-the-current-milestone`

Set the meta header — the audit `verdict` (green/red) and the `owner-artifact`: the
repo-relative path to the genuine-audit artifact the orchestrator recorded. That
path MUST live under the owned artifact home `completions/artifacts/<milestone>/` —
write the artifact file there, then name it here. finalize promotes that file in the
same transaction and the presence gate asserts it is durably staged (it never reads
the artifact's bytes — presence, not content):

jigc doc set-field completion-record:<slug>#verdict --value <green|red> --task complete-the-current-milestone
jigc doc set-field completion-record:<slug>#owner-artifact --value completions/artifacts/<milestone>/<file> --task complete-the-current-milestone

Then, for each triaged finding, mint an entry and author it — its `severity`
(blocking/advisory), its `disposition` (fixed/deferred/contested), and the `evidence`
(a string: the file:line, repro, or trace — not a managed ref). The item id is minted
from the title you give:

jigc doc add-item completion-record:<slug>#findings --title "<the finding>" --task complete-the-current-milestone
jigc doc set-field completion-record:<slug>#findings/<id>/severity --value <blocking|advisory> --task complete-the-current-milestone
jigc doc set-field completion-record:<slug>#findings/<id>/disposition --value <fixed|deferred|contested> --task complete-the-current-milestone
jigc doc set-field completion-record:<slug>#findings/<id>/evidence --value "<the file:line, repro, or trace>" --task complete-the-current-milestone

Or make it one call — the batch alternative to the whole sequence above (run it
INSTEAD of the create + per-leaf verbs, never after them — an already-staged doc
rejects a second create): the batch verb reads one declarative payload from
stdin (grammar: `jigc doc author --help`) and places every leaf in a single
write; the payload's `title:` mints the slug exactly like the create above:

jigc doc author completion-record --from-file - --task complete-the-current-milestone

Record each decision Settle made on the running decisions log. Create-or-update the
log singleton — safe whether or not it already exists (an existing committed log is
copied in for append):

Run: `jigc doc create decisions-log --title Decisions-Log --task complete-the-current-milestone`

Then, for each decision made, mint an entry and author its `why` — the reasoning,
in a sentence or two. The `date` is CLI-set on create:

jigc doc add-item decisions-log:decisions-log#entries --title "<the decision>" --task complete-the-current-milestone
jigc doc set-slot decisions-log:decisions-log#entries/<id>/why --from-file - --task complete-the-current-milestone

Or make it one call — the batch alternative to the whole sequence above (run it
INSTEAD of the create + per-entry verbs, never after them — an already-staged doc
rejects a second create): the batch verb reads one declarative payload from
stdin (grammar: `jigc doc author --help`) and places every entry in a single
write. Over an already-committed log it copies the committed doc in and appends —
existing entries are untouched:

jigc doc author decisions-log --from-file - --task complete-the-current-milestone

Land the change as exactly one logical commit. finalize commits the git index —
`git add` your code edits before you finalize, because it commits only what you
have staged, plus the docs it manages. Unstaged edits and untracked files are
left out of the commit; with nothing staged over a dirty tree, finalize
refuses. Anything still staged from BEFORE this task was minted makes finalize
refuse too (one blocking finding per carried path): unstage it, or pass
`--carry-staged` to declare the carryover deliberate.

finalize renders the commit doc; it does not fill it, so set its header and prose
first. Inside slot prose, headings must sit at `####` depth or deeper —
`##`/`###` are schema-reserved, and Setext headings are rejected.

Set the Conventional-Commits type:

Run: `jigc doc set-field commit:complete-the-current-milestone#type --value <TYPE> --task complete-the-current-milestone`
The `type` value is one of: feat | fix | docs | style | refactor | perf | test | build | ci | chore | revert

Set the scope — the area this change touches:

Run: `jigc doc set-field commit:complete-the-current-milestone#scope --value <SCOPE> --task complete-the-current-milestone`

Set the subject line — it renders as `<type>(<scope>): <summary>`, so write the
summary without a type or scope prefix of its own (the `type` field already
carries it):

Run: `jigc doc set-slot commit:complete-the-current-milestone#summary --from-file - --task complete-the-current-milestone`

Set the body — why this change:

Run: `jigc doc set-slot commit:complete-the-current-milestone#body --from-file - --task complete-the-current-milestone`

To see what's left before committing, run `jigc task validate complete-the-current-milestone` — it
previews the findings finalize will gate on, without committing anything.

Then validate and commit:

Run: `jigc task finalize complete-the-current-milestone`
resume: `jigc start --task complete-the-current-milestone`   — re-composes this workflow if context is lost
what's-left: `jigc task validate complete-the-current-milestone`   — previews the findings finalize will gate on
task scope: `jigc doc` writes default to the single active task; `--task complete-the-current-milestone` is the explicit override and wins when several are active
create-gates: completion-record, decisions-log
— jigc · run `jigc start` for orientation; all writes through `jigc`.
````

## $ jigc start --explain --workflow single-task "add a rate limiter"   # the resolution tree

````
workflow:single-task    (pack-default · dev/v1.0.0-rc.7)
  collision: default-workflow → won by dev/1.0.0-rc.7
  collision: doctype:commit → won by dev/1.0.0-rc.7
  Pack input: dev/1.0.0-rc.7 = <embedded>  (blake3 1b2b2f37a4a2e4a7bd82226d349f98dbd45f763ac8659731fc6c90fc090c07da)
  Pack input: methodology/1.0.0-rc.7 = <embedded>  (blake3 b256c4c33b327db69709861a68fda92455175c9426562c8126ad46be58560b6c)
  overrides applied: none
  includes:
    step:locate    (pack-default)
    step:implement    (pack-default)
    step:record-changelog    (pack-default)
    step:superseded-context    (pack-default)
    step:finalize    (pack-default)
— jigc · run `jigc start` for orientation; all writes through `jigc`.
````

### Reaching implement-from-spec: a committed spec is authored in the `plan` repo first (the minimal drive)

## $ jigc doc author spec --from-file - --task plan-the-notification-service   # in the plan repo; payload = title + goal/context slots + 2 criteria

````
spec:notification-service
````

## $ jigc task finalize plan-the-notification-service   # committing the spec

````
blocking · schema-conformance.field-value-conformant — `commit:plan-the-notification-service`: field `type` in section `header`: "" is not a member of enum "type" (allowed: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert)
  route: `jigc doc set-field commit:plan-the-notification-service#header/type --value <value>` to correct the value
blocking · schema-conformance.required-slot-present — `commit:plan-the-notification-service`: required slot in section `summary` is empty
  route: `jigc doc set-slot commit:plan-the-notification-service#summary --from-file -` to fill the empty slot
— jigc · run `jigc start` for orientation; all writes through `jigc`.
````
(exit code: 3)

## $ jigc task finalize plan-the-notification-service   # after filling the commit doc — the spec lands committed

````
advisory · file-state.staged-copy — staged copy of `docs/specs/notification-service.md` — this task's in-flight version of the doc
  route: no action needed — the staged copy is validated in-task and baselined when its finalize lands
— jigc · run `jigc start` for orientation; all writes through `jigc`.

finalized dc0a617 — feat: spec: plan the notification service with delivery criteria
  promoted docs/specs/notification-service.md
  1 file committed
````

## $ jigc start --workflow implement-from-spec "implement the notification service spec"   # in the plan repo, over the committed spec

````
task minted: implement-the-notification-service-spec

Implement from a committed spec. The intent is:
implement the notification service spec

Pick the spec this work implements from the committed specs below, bind it, then
re-run to read its criteria:

> spec:notification-service

Run: `jigc task bind spec <SPEC_ID> implement-the-notification-service-spec`
Run: `jigc start --task implement-the-notification-service-spec`

The bound spec's criteria — `spec:<slug>#criteria`, one item per criterion, each
carrying the `{#id}` anchor that addresses it:



Wire every criterion you satisfy back to the test that proves it. Once that test
passes, set `maps-to-test` on the criterion, addressed by the id in its `{#id}`
anchor above — never a slug you re-derive from the title (`jigc doc show
spec:<slug> --format json` prints the same ids):

jigc doc set-field spec:<slug>#criteria/<id>/maps-to-test --value <path>#<test-fn> --task implement-the-notification-service-spec

The anchor is `<repo-relative-path>#<symbol>`, it must resolve to a real test, and
it is checked at finalize — so leave it off a criterion this task did not cover
rather than pointing it at a test you have not written.

A spec can leave a decision open — a question its prose raises but does not settle.
When you settle one while implementing, record it before you finalize: this workflow
grants the `adr` gate, so mint a decision record through the `jigc doc create adr`
route below and author its slots. A decision that lives only in the code is one the
next reader cannot find.

Implement the change directly in the working tree. `git add` your code edits
before finalize — it commits only what you have staged. When done, set the
required Conventional-Commits type — your editorial call on what this change
does. The subject renders as `<type>(<scope>): <summary>`, so write the
summary without a type or scope prefix of its own — the `type` field already
carries it. Inside slot prose, headings must sit at `####` depth or deeper —
`##`/`###` are schema-reserved, and Setext headings are rejected. Set the
type, then stage the summary prose:

Run: `jigc doc set-field commit:implement-the-notification-service-spec#type --value <COMMIT_TYPE> --task implement-the-notification-service-spec`
The `type` value is one of: feat | fix | docs | style | refactor | perf | test | build | ci | chore | revert
Run: `jigc doc set-slot commit:implement-the-notification-service-spec#summary --from-file - --task implement-the-notification-service-spec`
<<author: commit:implement-the-notification-service-spec#summary>>

The `scope` and `body` are optional: add a `scope` to name the area touched, or
author a `body` to explain the motivation, only when they earn their place —

jigc doc set-field commit:implement-the-notification-service-spec#scope --value <area> --task implement-the-notification-service-spec
jigc doc set-slot commit:implement-the-notification-service-spec#body --from-file - --task implement-the-notification-service-spec

When the work shares authorship — a co-author, or an agent that wrote it — record
it in a commit trailer. Add one trailer item, then set its value on the address
`add-item` prints:

jigc doc add-item commit:implement-the-notification-service-spec#trailers --title Co-Authored-By --task implement-the-notification-service-spec
jigc doc set-field commit:implement-the-notification-service-spec#trailers/<id>/value --value "Name <email>" --task implement-the-notification-service-spec

If a decision is warranted, create an ADR and author its slots — a line per slot
usually suffices; an ADR earns its keep by capturing the *why*, not by running
long:

Run: `jigc doc create adr --title <TITLE> --task implement-the-notification-service-spec`

Author its three required slots on the address `create` prints — `context` (the
forces at play), `decision` (the call itself), `consequences` (tradeoffs and
follow-on effects):

jigc doc set-slot adr:<slug>#context --from-file - --task implement-the-notification-service-spec
jigc doc set-slot adr:<slug>#decision --from-file - --task implement-the-notification-service-spec
jigc doc set-slot adr:<slug>#consequences --from-file - --task implement-the-notification-service-spec

The `options` slot is optional — fill it only when alternatives were genuinely
weighed; omit it when the call was obvious:

jigc doc set-slot adr:<slug>#options --from-file - --task implement-the-notification-service-spec

Before you finalize, verify the change actually works: build it and run the
tests, and confirm the behaviour you set out to produce. Finalize commits your
staged work; it does not check that the work is correct.

Validate and commit the task as one logical commit. Make sure your code edits
are staged (`git add`) first — finalize commits only the staged set plus the
docs it manages; unstaged edits and untracked files are left out, and with
nothing staged over a dirty tree it refuses. Anything still staged from BEFORE
this task was minted makes finalize refuse too (one blocking finding per
carried path): unstage it, or pass `--carry-staged` to declare the carryover
deliberate.

To see what's left before committing, run `jigc task validate implement-the-notification-service-spec` — it
previews the findings finalize will gate on, without committing anything.

Run: `jigc task finalize implement-the-notification-service-spec`
resume: `jigc start --task implement-the-notification-service-spec`   — re-composes this workflow if context is lost
what's-left: `jigc task validate implement-the-notification-service-spec`   — previews the findings finalize will gate on
task scope: `jigc doc` writes default to the single active task; `--task implement-the-notification-service-spec` is the explicit override and wins when several are active
create-gates: adr
— jigc · run `jigc start` for orientation; all writes through `jigc`.
````

## $ jigc task bind spec notification-service implement-the-notification-service-spec   # the bind the compose demands

````
malformed address `notification-service`: missing ':' between type and slug
````
(exit code: 1)

## $ jigc task bind spec spec:notification-service implement-the-notification-service-spec   # the full-address form

````
bound spec = spec:notification-service (task implement-the-notification-service-spec)
````

## $ jigc start --task implement-the-notification-service-spec   # the resume compose after bind (the spec slice now grounded)

````
Implement from a committed spec. The intent is:
implement the notification service spec

Pick the spec this work implements from the committed specs below, bind it, then
re-run to read its criteria:

> spec:notification-service

Run: `jigc task bind spec <SPEC_ID> implement-the-notification-service-spec`
Run: `jigc start --task implement-the-notification-service-spec`

The bound spec's criteria — `spec:<slug>#criteria`, one item per criterion, each
carrying the `{#id}` anchor that addresses it:

> ### Events reach every subscribed channel  {#events-reach-every-subscribed-channel}
>
> An event published to the service is delivered to each subscribed channel exactly once under normal operation.
>
> ### Failed deliveries retry with backoff  {#failed-deliveries-retry-with-backoff}
>
> A delivery failure is retried with exponential backoff up to five attempts before being parked as dead-lettered.

Wire every criterion you satisfy back to the test that proves it. Once that test
passes, set `maps-to-test` on the criterion, addressed by the id in its `{#id}`
anchor above — never a slug you re-derive from the title (`jigc doc show
spec:<slug> --format json` prints the same ids):

jigc doc set-field spec:<slug>#criteria/<id>/maps-to-test --value <path>#<test-fn> --task implement-the-notification-service-spec

The anchor is `<repo-relative-path>#<symbol>`, it must resolve to a real test, and
it is checked at finalize — so leave it off a criterion this task did not cover
rather than pointing it at a test you have not written.

A spec can leave a decision open — a question its prose raises but does not settle.
When you settle one while implementing, record it before you finalize: this workflow
grants the `adr` gate, so mint a decision record through the `jigc doc create adr`
route below and author its slots. A decision that lives only in the code is one the
next reader cannot find.

Implement the change directly in the working tree. `git add` your code edits
before finalize — it commits only what you have staged. When done, set the
required Conventional-Commits type — your editorial call on what this change
does. The subject renders as `<type>(<scope>): <summary>`, so write the
summary without a type or scope prefix of its own — the `type` field already
carries it. Inside slot prose, headings must sit at `####` depth or deeper —
`##`/`###` are schema-reserved, and Setext headings are rejected. Set the
type, then stage the summary prose:

Run: `jigc doc set-field commit:implement-the-notification-service-spec#type --value <COMMIT_TYPE> --task implement-the-notification-service-spec`
The `type` value is one of: feat | fix | docs | style | refactor | perf | test | build | ci | chore | revert
Run: `jigc doc set-slot commit:implement-the-notification-service-spec#summary --from-file - --task implement-the-notification-service-spec`
<<author: commit:implement-the-notification-service-spec#summary>>

The `scope` and `body` are optional: add a `scope` to name the area touched, or
author a `body` to explain the motivation, only when they earn their place —

jigc doc set-field commit:implement-the-notification-service-spec#scope --value <area> --task implement-the-notification-service-spec
jigc doc set-slot commit:implement-the-notification-service-spec#body --from-file - --task implement-the-notification-service-spec

When the work shares authorship — a co-author, or an agent that wrote it — record
it in a commit trailer. Add one trailer item, then set its value on the address
`add-item` prints:

jigc doc add-item commit:implement-the-notification-service-spec#trailers --title Co-Authored-By --task implement-the-notification-service-spec
jigc doc set-field commit:implement-the-notification-service-spec#trailers/<id>/value --value "Name <email>" --task implement-the-notification-service-spec

If a decision is warranted, create an ADR and author its slots — a line per slot
usually suffices; an ADR earns its keep by capturing the *why*, not by running
long:

Run: `jigc doc create adr --title <TITLE> --task implement-the-notification-service-spec`

Author its three required slots on the address `create` prints — `context` (the
forces at play), `decision` (the call itself), `consequences` (tradeoffs and
follow-on effects):

jigc doc set-slot adr:<slug>#context --from-file - --task implement-the-notification-service-spec
jigc doc set-slot adr:<slug>#decision --from-file - --task implement-the-notification-service-spec
jigc doc set-slot adr:<slug>#consequences --from-file - --task implement-the-notification-service-spec

The `options` slot is optional — fill it only when alternatives were genuinely
weighed; omit it when the call was obvious:

jigc doc set-slot adr:<slug>#options --from-file - --task implement-the-notification-service-spec

Before you finalize, verify the change actually works: build it and run the
tests, and confirm the behaviour you set out to produce. Finalize commits your
staged work; it does not check that the work is correct.

Validate and commit the task as one logical commit. Make sure your code edits
are staged (`git add`) first — finalize commits only the staged set plus the
docs it manages; unstaged edits and untracked files are left out, and with
nothing staged over a dirty tree it refuses. Anything still staged from BEFORE
this task was minted makes finalize refuse too (one blocking finding per
carried path): unstage it, or pass `--carry-staged` to declare the carryover
deliberate.

To see what's left before committing, run `jigc task validate implement-the-notification-service-spec` — it
previews the findings finalize will gate on, without committing anything.

Run: `jigc task finalize implement-the-notification-service-spec`
resume: `jigc start --task implement-the-notification-service-spec`   — re-composes this workflow if context is lost
what's-left: `jigc task validate implement-the-notification-service-spec`   — previews the findings finalize will gate on
task scope: `jigc doc` writes default to the single active task; `--task implement-the-notification-service-spec` is the explicit override and wins when several are active
create-gates: adr
— jigc · run `jigc start` for orientation; all writes through `jigc`.
````

# Part 2 — two more generated migrate templates

## $ jigc migrate CHANGELOG.md --as changelog   # foreign Keep-a-Changelog file, committed — the placement-doctype arm

````
task minted: migrate-changelog-changelog

Migrate the foreign changelog into the managed `changelog` singleton. Below is the
foreign source the CLI staged for you (read-only context — you author the canonical
doc through the write verbs, you never edit this file or place anything yourself):

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- Dark mode toggle in settings

## [1.2.0] - 2026-05-14

### Added

- CSV export for reports
- Keyboard shortcuts for the editor

### Fixed

- Crash when opening an empty project

## [1.1.0] - 2026-03-02

### Changed

- Faster startup by lazy-loading plugins

### Removed

- The legacy XML importer


The target schema and its batch payload, both generated from the resolved `changelog`
schema, follow — the CLI creates-or-updates the singleton and places every release,
change-group and notes slot over a single staged buffer, so migrating dozens of
releases is one call, not hundreds:

The `changelog` schema — the managed singleton at `CHANGELOG.md`.

- `meta` (front-matter fields):
    - `schema-version`: int — CLI-stamped (schema-version) unless authored
- `unreleased-changes`: repeatable items, one per `category`:
    - `category`: enum, one of: added | changed | deprecated | removed | fixed | security — the item's id-source (authored as the item's `title:` payload key)
    - `notes`: prose slot — One bullet per change in this category.
- `releases`: repeatable items, one per `title`:
    - `title`: string — the item's id-source (authored as the item's `title:` payload key)
    - `date`: date — CLI-stamped (on-create) unless authored
    - `link`: string — optional
    - `changes`: repeatable items, one per `category`:
        - `category`: enum, one of: added | changed | deprecated | removed | fixed | security — the item's id-source (authored as the item's `title:` payload key)
        - `notes`: prose slot — One bullet per change in this category.

Author the whole document in ONE `jigc doc author` batch payload — fill each `<…>` value. The `<<…>>` wrapping on slot prose is REQUIRED literal syntax: keep the `<<`/`>>` markers and replace only the text between them (an inline field takes a bare value — wrapping one is rejected). Inside slot prose, headings must sit at `####` depth or deeper — `##`/`###` are schema-reserved, and Setext headings are rejected. An entry marked `# optional` may be omitted entirely. Pipe the payload on stdin:

jigc doc author changelog --from-file - --task migrate-changelog-changelog <<'EOF'
title: Changelog
sections:
  - id: unreleased-changes
    items:
      - title: "<added | changed | deprecated | removed | fixed | security>"
        set:
          notes: |-
            <<One bullet per change in this category.>>
  - id: releases
    items:
      - title: "<the title>"
        set:
          link: "<the link>" # optional — omit if unused
        sections:
          - id: changes
            items:
              - title: "<added | changed | deprecated | removed | fixed | security>"
                set:
                  notes: |-
                    <<One bullet per change in this category.>>
EOF

Re-author every RELEASE from the foreign file, oldest to newest, as an item under
`releases`. The item `title` is the version string (its id is minted from it — e.g.
`1.2.0` mints `120`).

The change-group `title` is the CATEGORY — one of the enum members the schema above
lists. Map each foreign category heading onto exactly one member, and MERGE many-to-one
where two foreign headings land on the same member (e.g. "Improvements" +
"Enhancements" → `changed`): the category is the group's id, so two foreign categories
collapsing onto one member share its SINGLE group — emit one `changes` item for that
member and merge both sets of bullets into its `notes`. A category outside the enum is
rejected at the write.

When a foreign file carries NO category headings at all, infer the member from each
change's commit prefix: a `feat:` change is `added`, a `fix:` change is `fixed`
(fall back to `changed` for anything else).

Set each release's `date` from the foreign file's HISTORICAL date when one is present:
add `date: "<YYYY-MM-DD>"` to the release item's `set:` block, exactly as it appears in
the source — an authored date overwrites the CLI's on-create stamp. When the foreign
source is DATELESS, OMIT the `date` key entirely — no date is stamped, so no history is
fabricated.

If the foreign release carries a compare/diff URL, map it to the optional `link`
field; otherwise omit it (there is no other home for it, and the rest of the foreign
preamble is dropped).

Any foreign change NOT yet cut into a release (an `[Unreleased]` section) is authored
as change-groups under the `unreleased-changes` section — same category map+merge
rule, the staging area rather than a release. When the foreign file has no unreleased
changes, omit the `unreleased-changes` entry entirely.

Finalizing a migration adds a review hold: a plain finalize commits NOTHING —
it renders the foreign source against the canonical rewrite and holds (exit 4),
because the CLI guarantees structure, never content-faithfulness. Review that
fidelity diff, then re-run the same finalize with `--approve` to write the
canonical doc, RETIRE the foreign original, and commit — approval is the one
destructive gate. Retired means removed: the foreign file is deleted from the
worktree and the deletion staged, so the removal lands in the same commit as
the canonical doc (a `git rm` in effect). A source already sitting at its canonical destination is
rewritten in place on approval — the committed file is modified, nothing is
retired. A destination already committed when the doc was authored was copied
in as the edit base (the create said so), so approval updates that committed
file in place — review the fidelity diff before approving. Finalize blocks
(finalize.promote-clobber) only when a file appeared at the destination after
the doc was created: resolve that collision, or retitle the doc so it slugs
differently, then finalize again.
Validate and commit the task as one logical commit. Make sure your code edits
are staged (`git add`) first — finalize commits only the staged set plus the
docs it manages; unstaged edits and untracked files are left out, and with
nothing staged over a dirty tree it refuses. Anything still staged from BEFORE
this task was minted makes finalize refuse too (one blocking finding per
carried path): unstage it, or pass `--carry-staged` to declare the carryover
deliberate.

To see what's left before committing, run `jigc task validate migrate-changelog-changelog` — it
previews the findings finalize will gate on, without committing anything.

Run: `jigc task finalize migrate-changelog-changelog`
resume: `jigc start --task migrate-changelog-changelog`   — re-composes this workflow if context is lost
what's-left: `jigc task validate migrate-changelog-changelog`   — previews the findings finalize will gate on
task scope: `jigc doc` writes default to the single active task; `--task migrate-changelog-changelog` is the explicit override and wins when several are active
create-gates: changelog
— jigc · run `jigc start` for orientation; all writes through `jigc`.
````

## $ jigc migrate docs/VISION.md --as vision   # foreign vision-ish doc, committed — the methodology-pack arm

````
task minted: migrate-vision-docs-vision

Migrate the foreign vision document into the managed `vision` singleton — the project's
charter. Below is the foreign source the CLI staged for you (read-only context — you
author the canonical record through the write verbs, you never edit this file or place
anything yourself):

# Product Vision

We believe small teams drown in coordination overhead. Our product is a
lightweight shared task board that stays out of the way: no accounts to
manage, no notifications by default, one keyboard-driven surface.

## Why now

Remote-first teams have settled on chat + docs, but the task layer is
either a heavyweight tracker or a pile of markdown TODOs. Neither fits a
five-person team shipping weekly.

## What success looks like

A team adopts the board in under ten minutes and still uses it three
months later without ever opening a settings page.

## Open questions

- Should boards sync peer-to-peer or through a hosted relay?
- Is mobile a first-class surface or a companion view?


The target schema and its batch payload, both generated from the resolved `vision`
schema, follow — the CLI creates the fixed-slug singleton (the `title: Vision` line
stays exactly as written; the CLI owns the placement) and places every prose slot over
a single staged buffer:

The `vision` schema — the managed singleton at `VISION.md`.

- `meta` (front-matter fields):
    - `grounded-in`: ref -> research (0..*) — optional
    - `schema-version`: int — CLI-stamped (schema-version) unless authored
- `thesis`: prose slot — The core claim — what this project is and the one idea it descends from.
- `invariants`: prose slot — What must stay true — the boundaries and commitments no change may violate.
- `open-questions`: prose slot — What is deliberately unsettled — the directions still open.

Author the whole document in ONE `jigc doc author` batch payload — fill each `<…>` value. The `<<…>>` wrapping on slot prose is REQUIRED literal syntax: keep the `<<`/`>>` markers and replace only the text between them (an inline field takes a bare value — wrapping one is rejected). Inside slot prose, headings must sit at `####` depth or deeper — `##`/`###` are schema-reserved, and Setext headings are rejected. An entry marked `# optional` may be omitted entirely. Pipe the payload on stdin:

jigc doc author vision --from-file - --task migrate-vision-docs-vision <<'EOF'
title: Vision
sections:
  - id: thesis
    set:
      thesis: |-
        <<The core claim — what this project is and the one idea it descends from.>>
  - id: invariants
    set:
      invariants: |-
        <<What must stay true — the boundaries and commitments no change may violate.>>
  - id: open-questions
    set:
      open-questions: |-
        <<What is deliberately unsettled — the directions still open.>>
EOF

Map the foreign document's headings onto the schema above:

  - the foreign Thesis / Mission / Purpose / What-this-is prose maps to the `thesis` slot;
  - the foreign Principles / Invariants / Non-negotiables / Constraints heading maps to
    the `invariants` slot (keep it one bullet per invariant);
  - the foreign Open questions / Unknowns / Someday / Future-directions heading maps to
    the `open-questions` slot.

OMIT `grounded-in` — always, in migration. The optional `grounded-in` ref points at
managed `research` docs, but a foreign vision has no managed research in the store, so
any authored ref would dangle and block finalize forever. Omitting it is legal (the ref
is `0..*`); when the foreign text cites its evidence or research, fold that citation
into the `open-questions` prose instead — never author it as a structural ref.

Surplus foreign content with NO slot home is usually a live open question worth keeping:
fold it into the `open-questions` slot as one bullet. Drop it ONLY when it is pure
boilerplate (a badge block, a table of contents, a license footer).

Finalizing a migration adds a review hold: a plain finalize commits NOTHING —
it renders the foreign source against the canonical rewrite and holds (exit 4),
because the CLI guarantees structure, never content-faithfulness. Review that
fidelity diff, then re-run the same finalize with `--approve` to write the
canonical doc, RETIRE the foreign original, and commit — approval is the one
destructive gate. Retired means removed: the foreign file is deleted from the
worktree and the deletion staged, so the removal lands in the same commit as
the canonical doc (a `git rm` in effect). A source already sitting at its
canonical destination is
rewritten in place on approval — the committed file is modified, nothing is
retired. A destination already committed when the doc was authored was copied
in as the edit base (the create said so), so approval updates that committed
file in place — review the fidelity diff before approving. Finalize blocks
(finalize.promote-clobber) only when a file appeared at the destination after
the doc was created: resolve that collision, or retitle the doc so it slugs
differently, then finalize again.
Land the change as exactly one logical commit. finalize commits the git index —
`git add` your code edits before you finalize, because it commits only what you
have staged, plus the docs it manages. Unstaged edits and untracked files are
left out of the commit; with nothing staged over a dirty tree, finalize
refuses. Anything still staged from BEFORE this task was minted makes finalize
refuse too (one blocking finding per carried path): unstage it, or pass
`--carry-staged` to declare the carryover deliberate.

finalize renders the commit doc; it does not fill it, so set its header and prose
first. Inside slot prose, headings must sit at `####` depth or deeper —
`##`/`###` are schema-reserved, and Setext headings are rejected.

Set the Conventional-Commits type:

Run: `jigc doc set-field commit:migrate-vision-docs-vision#type --value <TYPE> --task migrate-vision-docs-vision`
The `type` value is one of: feat | fix | docs | style | refactor | perf | test | build | ci | chore | revert

Set the scope — the area this change touches:

Run: `jigc doc set-field commit:migrate-vision-docs-vision#scope --value <SCOPE> --task migrate-vision-docs-vision`

Set the subject line — it renders as `<type>(<scope>): <summary>`, so write the
summary without a type or scope prefix of its own (the `type` field already
carries it):

Run: `jigc doc set-slot commit:migrate-vision-docs-vision#summary --from-file - --task migrate-vision-docs-vision`

Set the body — why this change:

Run: `jigc doc set-slot commit:migrate-vision-docs-vision#body --from-file - --task migrate-vision-docs-vision`

To see what's left before committing, run `jigc task validate migrate-vision-docs-vision` — it
previews the findings finalize will gate on, without committing anything.

Then validate and commit:

Run: `jigc task finalize migrate-vision-docs-vision`
resume: `jigc start --task migrate-vision-docs-vision`   — re-composes this workflow if context is lost
what's-left: `jigc task validate migrate-vision-docs-vision`   — previews the findings finalize will gate on
task scope: `jigc doc` writes default to the single active task; `--task migrate-vision-docs-vision` is the explicit override and wins when several are active
create-gates: vision
— jigc · run `jigc start` for orientation; all writes through `jigc`.
````

# Part 3 — Route inventory (static)

Mechanical extraction from /home/maurice/Projects/gherrink-jigc/crates (read-only) at HEAD e82bae2: every `Route::human(` / `Route::mechanical(` / `Route::informational(` construction tree-wide (test modules excluded), plus finding message format strings in the named engine/cli files (`Finding::graded`, `blocking_conformance`, `blocking_write`, store `block`, `let message = format!` sites). One line per distinct template: `<file>:<line>` · finding code or enclosing context · the template verbatim (format-string placeholders `{x}`/`{}` and route placeholders `<address>`/`<value>` as-is). Mechanical routes render as the constructor composes them: backtick-wrapped argv join + tail. `<expr: …>` marks a non-literal argument (value built elsewhere); the Supplement resolves the multi-variant ones.

### cli/src/cli.rs

- `cli/src/cli.rs:1233` · unknown_subcommand_tip · [mechanical] `jigc task discard <task-id>` abandons the WHOLE task (removes its working area and every staged write); to back out a single external edit, revert that file on disk instead
- `cli/src/cli.rs:1243` · unknown_subcommand_tip · [mechanical] `jigc task list` enumerates the active tasks (id + minting workflow + intent)
- `cli/src/cli.rs:1248` · unknown_subcommand_tip · [mechanical] `jigc task validate <task-id>` previews the finalize gate for one task — what still blocks

### cli/src/doc.rs

- `cli/src/doc.rs:395` · write.machine-maintained · [message] {verb} rejected: `{target}` is a milestone-record — the record is machine-maintained (every leaf is CLI-`set:`, so it carries no author-owned slot or field), and no `jigc doc` write applies to it
- `cli/src/doc.rs:395` · write.machine-maintained · [route] leave the record to the milestone verbs — `jigc milestone create` opens it, `jigc milestone add-task` appends sub-tasks, `jigc task finalize` advances a sub-task's status, `jigc milestone finalize` joins it, and `jigc milestone discard` settles an abandoned one; read it with `jigc doc show`
- `cli/src/doc.rs:706` · write.id-from-field · [message] set-field rejected: `{field}` is the heading-derived id-from field of item `{item_path}` — its value lives in the item heading, not a field bullet
- `cli/src/doc.rs:706` · write.id-from-field · [route] {doc}#{item_path}/{field}
- `cli/src/doc.rs:1007` · <engine::validate::ID_FROM_ENUM_CODE> · [message] add-item rejected: `{slug}` is not an enum member of id-from field `{}`
- `cli/src/doc.rs:1021` · id_from_enum_block · [mechanical] `jigc doc schema <doctype>` to see the declared members, then re-run `add-item` with a member title
- `cli/src/doc.rs:1232` · write.machine-maintained · [message] retitle-item rejected: item `{item_path}` lives in a milestone-record — the record is machine-maintained and an item's heading IS the sub-task's work-unit id, so a retitle would sever the record from its work unit
- `cli/src/doc.rs:1356` · write.identity-change · [message] retitle-item rejected: item `{item_path}` derives its id from enum field `{}` — a member change is an identity change, not a retitle
- `cli/src/doc.rs:1356` · write.identity-change · [route] run `jigc doc remove-item {addr}` then `jigc doc add-item {dest} --title "{title}"` under the target category, moving the prose in the same motion
- `cli/src/doc.rs:1904` · stale_read_hint · [mechanical] `jigc doc show --task`
- `cli/src/doc.rs:2045` · store.unknown-type · [message] unknown doctype `{doctype}`
- `cli/src/doc.rs:2045` · store.unknown-type · [route] list the available doctypes with `jigc describe`
- `cli/src/doc.rs:2087` · store.unknown-type · [message] unknown doctype `{ty}`
- `cli/src/doc.rs:3102` · resolve · [mechanical] `jigc start`
- `cli/src/doc.rs:3233` · workflow_gate · [mechanical] `jigc task discard`
- `cli/src/doc.rs:3234` · workflow_gate · [mechanical] `jigc start`
- `cli/src/doc.rs:3269` · parse_verb_addr · [mechanical] `jigc describe`
- `cli/src/doc.rs:3374` · write.area-barrier · [message] barrier — the staged destination for `{address}` lands outside task `{task_id}`'s area (`tasks/{task_id}/docs/`); a staging write must stay within its own sub-area
- `cli/src/doc.rs:3374` · write.area-barrier · [route] address the doc with a slug inside task `{task_id}`'s own area
- `cli/src/doc.rs:3395` · read_staged · [mechanical] `jigc start`
- `cli/src/doc.rs:3398` · read_staged · [mechanical] `jigc doc create <type> --title "X"`

### cli/src/ingest.rs

- `cli/src/ingest.rs:334` · <finding.code> · [message] <expr: finding.message>
- `cli/src/ingest.rs:365` · ingest.needs-reconcile · [message] `{rel_path}` does not conform to the `{}` schema
- `cli/src/ingest.rs:413` · ingest.wrong-location · [message] conformant `{ty}` at `{rel_path}` sits outside `{location}` — relocate to adopt (jigc never auto-moves)
- `cli/src/ingest.rs:413` · ingest.wrong-location · [route] move {rel_path} into {location}, then re-run `jigc ingest`

### cli/src/locate.rs

- `cli/src/locate.rs:53` · not_set_up · [mechanical] `jigc setup`

### cli/src/migrate.rs

- `cli/src/migrate.rs:113` · ensure_migratable · [mechanical] `jigc migrate <path> --as <doctype>`
- `cli/src/migrate.rs:191` · migrate_in_repo · [mechanical] `jigc migrate <path> --as`

### cli/src/milestone.rs

- `cli/src/milestone.rs:227` · no_such_milestone · [mechanical] `jigc milestone create "<title>"`
- `cli/src/milestone.rs:1181` · run_discard · [mechanical] `jigc milestone list-tasks <milestone-id>`
- `cli/src/milestone.rs:1285` · milestone.dirty-worktree · [message] milestone:{milestone_id} has uncommitted work in {} sub-task worktree(s) — discarding it would destroy that work:\n{}
- `cli/src/milestone.rs:1285` · milestone.dirty-worktree · [route] get the work out of those worktrees first (commit, stash, or copy it), then re-run `jigc milestone discard {milestone_id}` — or re-run with `--force` to abandon the milestone and destroy the uncommitted work

### cli/src/relocate.rs

- `cli/src/relocate.rs:148` · relocate_freeze_exempt · [mechanical] `jigc migrate-corpus`

### cli/src/rename.rs

- `cli/src/rename.rs:100` · run · [mechanical] `jigc describe`
- `cli/src/rename.rs:543` · parse_addr · [mechanical] `jigc describe`

### cli/src/setup.rs

- `cli/src/setup.rs:128` · store-version.binary-mismatch · [message] <expr: message>

### cli/src/start.rs

- `cli/src/start.rs:1598` · resume_in_repo · [mechanical] `jigc task discard`
- `cli/src/start.rs:1613` · resume_in_repo · [mechanical] `jigc start`
- `cli/src/start.rs:1658` · reenter_in_repo · [mechanical] `jigc milestone list-tasks <milestone-id>`
- `cli/src/start.rs:1675` · reenter_in_repo · [mechanical] `jigc task discard`
- `cli/src/start.rs:1693` · reenter_in_repo · [mechanical] `jigc milestone add-task <milestone-id> "<intent>"`
- `cli/src/start.rs:2199` · overrides.project-step-missing · [message] project layer owns step `{id}` but its file {} is unreadable: {e}
- `cli/src/start.rs:2207` · overrides.project-step-missing · [human] restore the project step file {}, or drop the project layer's claim on `{id}` (remove the `deltas:` entry or shadow that references it)

### cli/src/task.rs

- `cli/src/task.rs:478` · no_such_task · [mechanical] `jigc task list`
- `cli/src/task.rs:1392` · workflow_def · [mechanical] `jigc start`
- `cli/src/task.rs:1485` · changelog-recording.gate-granted-unused · [message] workflow `{workflow_id}` grants the `changelog` create-gate and this task recorded no changelog entry
- `cli/src/task.rs:2125` · finalize.nothing-staged · [message] you staged nothing — the working tree has changes but the index is empty
- `cli/src/task.rs:2125` · finalize.nothing-staged · [route] `git add` your changes, then re-run `jigc task finalize`
- `cli/src/task.rs:2151` · finalize.stage-failed · [message] jigc could not stage its own changes — no commit was made and the promotions were rolled back: {git_error}
- `cli/src/task.rs:2151` · finalize.stage-failed · [route] resolve the embedded git failure (e.g. remove a stale `.git/index.lock`), then re-run `jigc task finalize`

### engine/src/cascade.rs

- `engine/src/cascade.rs:158` · <self.code()> · [message] <expr: self.message()>
- `engine/src/cascade.rs:163` · into_finding · [human] correct the target to the delta-target grammar — `workflow:<id>#<step-id>` to name a step position, or `workflow:<id>` with an `after:`/`before:` anchor — in the `jigc config <verb>` argument that passed it or the recorded `deltas:` entry that carries it
- `engine/src/cascade.rs:370` · into_finding · [human] correct the target to the `step:<step-id>#<fill-id>` grammar in the `jigc config fill <target>` argument or the recorded `deltas:` entry that carries it

### engine/src/compose.rs

- `engine/src/compose.rs:39` · blocking_workflow_refs · [human] fix the workflow/step/catalog definition the message names (a definition defect, repaired once at its source), then re-run

### engine/src/file_state.rs

- `engine/src/file_state.rs:139` · file-state.staged-copy · [message] staged copy of `{dest}` — this task's in-flight version of the doc
- `engine/src/file_state.rs:144` · file-state.staged-copy · [informational] no action needed — the staged copy is validated in-task and baselined when its finalize lands
- `engine/src/file_state.rs:645` · file-state.hash-matches · [message] on-disk content of `{path}` differs from the recorded state
- `engine/src/file_state.rs:645` · file-state.hash-matches · [route] review the out-of-band edit to `{path}` and re-author it through the owning workflow
- `engine/src/file_state.rs:668` · file-state.un-baselined · [message] committed doc `{path}` is not yet baselined in the file-state record
- `engine/src/file_state.rs:668` · file-state.un-baselined · [route] no action needed — the doc is baselined on its next author or finalize
- `engine/src/file_state.rs:847` · reconciliation.rename · [message] tracked managed doc {from} ({path}) is missing; {suspect} has the same content hash — likely renamed via `git mv`
- `engine/src/file_state.rs:847` · reconciliation.rename · [route] adopt it as a CLI-owned rename (re-points every referrer atomically): `jigc rename {from} --to "<New Title>"`; or revert the move: `git mv {suspect} {path}`
- `engine/src/file_state.rs:864` · reconciliation.rename · [message] tracked managed doc {from} ({path}) is missing
- `engine/src/file_state.rs:864` · reconciliation.rename · [route] restore {path}, or confirm the deletion by dropping it from the index: `jigc unmanage {path}`
- `engine/src/file_state.rs:881` · reconciliation.absorb · [message] external edit absorbed: `{path}`
- `engine/src/file_state.rs:881` · reconciliation.absorb · [route] no action needed — the external edit was absorbed into the baseline
- `engine/src/file_state.rs:902` · reconciliation.conformance-block · [message] nonconformant edit on `{path}`: {detail}
- `engine/src/file_state.rs:902` · reconciliation.conformance-block · [route] fix the file to restore conformance, or revert the edit
- `engine/src/file_state.rs:927` · reconciliation.conformance-block · [message] unvetted file `{path}` in a managed location is not schema-conformant: {detail}
- `engine/src/file_state.rs:927` · reconciliation.conformance-block · [route] ingest, migrate, or move `{path}` out of the managed location to resolve it
- `engine/src/file_state.rs:953` · reconciliation.conflict-block · [message] conflict on `{path}`: an external edit and this task's staged writes both changed it
- `engine/src/file_state.rs:960` · reconciliation.conflict-block · [mechanical] `jigc task discard <task-id>` to drop this task's staged writes (discard retires the whole task — no per-doc discard exists), or revert the external edit on disk to keep them
- `engine/src/file_state.rs:974` · file-state.baseline-adopt · [message] baseline adopted: `{path}`
- `engine/src/file_state.rs:974` · file-state.baseline-adopt · [route] no action needed — the baseline was adopted on first encounter

### engine/src/finalize.rs

- `engine/src/finalize.rs:433` · finalize.promote-clobber · [message] <expr: message>
- `engine/src/finalize.rs:447` · finalize.provenance-io · [message] could not read the task provenance manifest under `{}`: {err}
- `engine/src/finalize.rs:654` · finalize.carried-staged · [message] <expr: message>
- `engine/src/finalize.rs:659` · finalize.carried-staged · [human] <expr: route>
- `engine/src/finalize.rs:872` · finalize.promote-io · [message] could not read the staged managed doc `{}` to promote it: {err}
- `engine/src/finalize.rs:891` · finalize.migration-no-replacement · [message] this migration recorded the foreign source `{source_path}` but staged no managed doc to replace it — the foreign original will not be retired with nothing to take its place
- `engine/src/finalize.rs:891` · finalize.migration-no-replacement · [route] author the canonical doc (e.g. `jigc doc create <doctype> --task <id>`), then re-run `jigc task finalize <id> --approve`
- `engine/src/finalize.rs:912` · finalize.source-path-io · [message] could not read the recorded migration source path under `{}`: {err}
- `engine/src/finalize.rs:929` · finalize.no-task · [message] no task working area at `{}` — nothing to finalize
- `engine/src/finalize.rs:929` · finalize.no-task · [route] start a task with `jigc start "<intent>"`
- `engine/src/finalize.rs:1036` · finalize.base-mismatch · [message] <expr: message>
- `engine/src/finalize.rs:1061` · finalize.base-mismatch · [message] the task was started at base `{}` but HEAD is now `{head_sha}`, and the moved history overlaps the task's work on `{paths}`
- `engine/src/finalize.rs:1061` · finalize.base-mismatch · [route] resolve the overlap on `{paths}` against the new history, or discard the task with `jigc task discard`
- `engine/src/finalize.rs:1085` · finalize.empty-commit · [message] task validated but produced no diff — nothing to finalize
- `engine/src/finalize.rs:1085` · finalize.empty-commit · [route] make a change, then re-run `jigc task finalize`
- `engine/src/finalize.rs:1097` · finalize.render-io · [message] could not read the staged commit doc `{}`: {err}

### engine/src/finding.rs

- `engine/src/finding.rs:195` · readdress_to_uri · [mechanical] <argv: argv><tail.clone()>
- `engine/src/finding.rs:583` · from · [human] <expr: text>
- `engine/src/finding.rs:614` · deserialize · [human] <expr: String::deserialize(deserializer)?>

### engine/src/index.rs

- `engine/src/index.rs:679` · schema-conformance.mention-resolves · [message] schema-conformance — in-prose mention `#{mention}` in `{from}` resolves to no committed doc (the renamed/deleted-doc case); correct or drop the mention
- `engine/src/index.rs:679` · schema-conformance.mention-resolves · [route] correct or drop the `#{mention}` mention in `{from}`
- `engine/src/index.rs:776` · schema-completeness.inverse-cardinality · [message] schema-completeness — `{target}` has {count} inbound `{relation}` edge(s){inverse_phrase}, below the inverse-card minimum of {min}
- `engine/src/index.rs:776` · schema-completeness.inverse-cardinality · [route] author a `{relation}` referrer of `{target}`
- `engine/src/index.rs:834` · schema-conformance.ref-resolves · [message] forward-ref integrity — `{from}#{relation}` target `{to}` resolves in neither the committed store nor this task's working area; resolution: fix the reference to an existing target, create the target in this task, or drop the `{relation}` field
- `engine/src/index.rs:834` · schema-conformance.ref-resolves · [route] fix the reference, create the target in this task, or drop the field

### engine/src/probe.rs

- `engine/src/probe.rs:336` · pack-probe-integrity.probe-failure · [human] the probe subprocess misbehaved (the message carries the reason) — repair or re-install the probe binary (`jigc setup` reinstalls the shipped probes), then re-run the sweep

### engine/src/state.rs

- `engine/src/state.rs:688` · task.serial-collision · [message] task `{id}` is already active
- `engine/src/state.rs:688` · task.serial-collision · [route] resume with `jigc start --task {id}` or abandon with `jigc task discard {id}`
- `engine/src/state.rs:1047` · create.unknown-doctype · [message] unknown doctype `{type_name}`; known doctypes: [{}]
- `engine/src/state.rs:1047` · create.unknown-doctype · [route] run `jigc describe` to see the doctypes you can author
- `engine/src/state.rs:1075` · create.serial-collision · [message] instance `{address}` already exists in the working area
- `engine/src/state.rs:1075` · create.serial-collision · [route] edit the existing `{address}` instead of re-creating it
- `engine/src/state.rs:1090` · create.empty-title · [message] `jigc doc create {type_name}` needs a title that yields an id, but the given title is empty or slugs to nothing
- `engine/src/state.rs:1090` · create.empty-title · [route] re-run with a non-empty `--title` (its slug becomes the doc id)
- `engine/src/state.rs:1107` · create.gate-blocked · [message] the workflow does not allow `jigc doc create {type_name}` in-task; allowed doctypes: [{}]
- `engine/src/state.rs:1107` · create.gate-blocked · [route] to loosen, add `{type_name}` to `allows-create` in project config
- `engine/src/state.rs:1121` · task.working-area-io · [message] could not {doing} for task `{id}`: {err}

### engine/src/store.rs

- `engine/src/store.rs:123` · store.transient-type · [message] doctype `{type_name}` is transient (no `location:`); `{address_str}` is not committed
- `engine/src/store.rs:129` · store.transient-type · [mechanical] `jigc doc show --task <task-id>` — a transient doc is readable only as a task's staged working copy
- `engine/src/store.rs:150` · store.not-found · [message] could not read `{address_str}` at `{}`: {err}
- `engine/src/store.rs:216` · store.unknown-type · [message] unknown doctype `{type_name}` for `{address_str}`
- `engine/src/store.rs:233` · store.not-found · [message] `{address_str}` names no committed doc: `{type_name}` is a singleton, so its only address is `{type_name}:{}`
- `engine/src/store.rs:274` · store.unparseable · [message] `{address_str}` at `{}` does not parse: {why}
- `engine/src/store.rs:306` · store.not-staged · [message] `{address_str}` is not staged in this task — only its committed copy exists
- `engine/src/store.rs:310` · store.not-staged · [mechanical] `jigc doc show` — the task-less read serves the committed copy
- `engine/src/store.rs:316` · store.not-staged · [message] `{address_str}` is not staged in this task and has no committed copy — nothing to read yet
- `engine/src/store.rs:332` · <code> · [message] <expr: message>
- `engine/src/store.rs:382` · store.no-such-section · [message] `{address}` names no section `{section_id}`
- `engine/src/store.rs:466` · store.no-such-section · [message] `{address}` names no nested section `{bad}` in section `{section_id}`
- `engine/src/store.rs:473` · store.no-such-item · [message] `{address}` names no item `{}` in section `{section_id}`
- `engine/src/store.rs:516` · store.no-such-item · [message] `{address}` names no item `{id}` in {scope}
- `engine/src/store.rs:530` · store.no-such-item · [message] `{address}` carries an empty item path
- `engine/src/store.rs:560` · store.no-such-leaf · [message] `{address}` names no leaf `{leaf}` in section `{}`
- `engine/src/store.rs:647` · store.no-such-leaf · [message] `{address}` names no leaf `{leaf}` on item `{}`

### engine/src/validate.rs

- `engine/src/validate.rs:881` · unadopted_instance · [message] committed file `{rel_key}` sits at the `{ty}` home but was never adopted by jigc — it carries no schema-version stamp and parses against no known `{ty}` schema version
- `engine/src/validate.rs:885` · schema-conformance.unadopted-instance · [message] <expr: message>
- `engine/src/validate.rs:1192` · <SCHEMA_VERSION_CURRENT_CODE> · [message] <expr: message>
- `engine/src/validate.rs:1594` · schema-conformance.unknown-type · [message] staged doc `{identity}` has type `{ty}`, which the resolved cascade does not define
- `engine/src/validate.rs:1714` · owner-artifact.present · [message] owner-artifact `{}` in section `{}` of `{display}`: {why}
- `engine/src/validate.rs:1867` · schema-conformance.repeatable-populated · [message] repeatable section `{}` parses zero items — structurally empty
- `engine/src/validate.rs:1867` · schema-conformance.repeatable-populated · [route] populate the section, or exempt `{token}` via the `validation.schema-conformance.repeatable-populated.exempt` knob
- `engine/src/validate.rs:1947` · schema-conformance.surplus-sections-absent · [message] {} trailing surplus section heading(s) beyond the schema's {} body section(s), starting at `## {first_text}` — the positional parse never visits them, so the content is carried byte-faithful but unmanaged
- `engine/src/validate.rs:1947` · schema-conformance.surplus-sections-absent · [route] fold the surplus content into a schema section or remove it — jigc never reads or splices it
- `engine/src/validate.rs:2080` · schema-conformance.required-slot-present · [message] required slot `{leaf_id}` in item `{item_path}` is empty
- `engine/src/validate.rs:2105` · schema-conformance.field-value-conformant · [message] field `{}` in item `{item_path}`: {why}
- `engine/src/validate.rs:2114` · schema-conformance.required-field-present · [message] required field `{}` is missing from item `{item_path}`
- `engine/src/validate.rs:2184` · <ID_FROM_ENUM_CODE> · [message] id-from field `{}` in item `{item_path}`: `{}` is not an enum member
- `engine/src/validate.rs:2225` · schema-conformance.required-slot-present · [message] required slot in section `{}` is empty
- `engine/src/validate.rs:2258` · schema-conformance.required-field-present · [message] required field `{}` is missing from section `{}`
- `engine/src/validate.rs:2296` · schema-conformance.field-value-conformant · [message] field `{}` in section `{}`: {why}
- `engine/src/validate.rs:2371` · <code> · [message] <expr: message>
- `engine/src/validate.rs:2387` · schema-conformance.required-slot-present · [mechanical] `jigc doc set-slot <address> --from-file -` to fill the empty slot
- `engine/src/validate.rs:2391` · schema-conformance.required-field-present · [mechanical] `jigc doc set-field <address> --value <value>` to supply the missing field
- `engine/src/validate.rs:2404` · schema-conformance.field-value-conformant · [mechanical] `jigc doc set-field <address> --value <value>` to correct the value
- `engine/src/validate.rs:2419` · schema-conformance.unknown-type · [human] check the type against `jigc describe` — restore the doctype's cascade entry if a pack/config change removed it, or remove or re-type the stray staged file
- `engine/src/validate.rs:2426` · owner-artifact.present · [human] place the owned artifact at the recorded path, or correct the field with `jigc doc set-field` to where the file really lives
- `engine/src/validate.rs:5831` · doc-code.symbol-exists · [message] symbol does not resolve: {}
- `engine/src/validate.rs:7130` · doc-code.symbol-exists · [message] dangling: {}
- `engine/src/validate.rs:7296` · <code> · [message] <expr: a.anchor_value.clone()>
- `engine/src/validate.rs:7363` · doc-code.symbol-exists · [message] dangling
- `engine/src/validate.rs:7425` · schema-conformance.required-field-present · [message] required field `owner` is missing

### engine/src/write.rs

- `engine/src/write.rs:5049` · <code> · [message] <expr: message>
- `engine/src/write.rs:5066` · write.malformed-value · [mechanical] `jigc doc schema <doctype>` to see the field's declared type and members, then re-run the write with a conformant value
- `engine/src/write.rs:5074` · write.not-present · [mechanical] `jigc doc schema <doctype>` to see the declared shape, then re-run the write at a declared address
- `engine/src/write.rs:5078` · write.already-present · [human] the target already exists — edit it in place (`set-field`/`set-slot`) instead of re-creating it
- `engine/src/write.rs:5082` · write.unslugable-title · [human] re-run the write with a title carrying at least one word character (the item id is slugged from it)
- `engine/src/write.rs:5086` · write.list-overwrite · [human] re-run `jigc doc set-field` with the inline-list form shown in the message, carrying every value to keep
- `engine/src/write.rs:5090` · write.non-reparseable · [human] nothing was persisted — revise the payload so the result still conforms (the message names the break), then re-run the same write
- `engine/src/write.rs:5094` · write.target-escape · [human] nothing was persisted — re-run the write; a recurring escape is a write-path defect to report
- `engine/src/write.rs:5098` · write.slot-heading-depth · [human] demote the heading to `####` depth or rephrase it as plain prose, then re-run the same write
- `engine/src/write.rs:5102` · write.slot-setext-heading · [human] rewrite the Setext heading as `####` ATX depth or plain prose, then re-run the same write
- `engine/src/write.rs:5338` · write.list-overwrite · [message] write rejected: field {field_key:?} already has {count} value(s); `set-field` replaces the whole list and would silently drop them. To set multiple values, pass them all in one call: --value {suggested:?}
- `engine/src/write.rs:5445` · write.non-reparseable · [message] write rejected: result does not re-parse ({detail})
- `engine/src/write.rs:5459` · write.target-escape · [message] write rejected: the change touched bytes outside the intended target
- `engine/src/write.rs:5488` · write.unknown-field · [message] no field {field_key:?} declared in section {section_id:?}
- `engine/src/write.rs:5495` · write.malformed-value · [message] write rejected: {why}
- `engine/src/write.rs:5609` · write.unknown-field · [message] no field {field_key:?} declared on item {item_ids:?} in section {section_id:?}
- `engine/src/write.rs:5726` · write.not-present · [message] slot in section {section_id:?} is not present
- `engine/src/write.rs:5793` · write.slot-setext-heading · [message] Setext heading in slot prose at line {line}; use `{CEILING_ALLOWED}` ATX depth or rephrase
- `engine/src/write.rs:5809` · write.slot-heading-depth · [message] heading at schema-reserved depth `{depth}` in slot prose at line {line}; use `{CEILING_ALLOWED}` or rephrase

### Supplement — variant templates behind `<expr: message>` sites (read from source, verbatim)

- `engine/src/finalize.rs:401` · finalize.promote-clobber (migration-source arm) · [message] promoting this migration's doc to `{destination}` would overwrite a file already there — and that file is not the migration's recorded source (`{source}`); refusing to clobber it
- `engine/src/finalize.rs:406` · finalize.promote-clobber (migration-source arm) · [route] a file already occupies `{destination}`, and this migration's recorded source is the different file `{source}`: if the occupant is another managed doc, land this migration under a different id — re-author with a title that slugs differently, or `jigc task discard <id>` and re-mint with `jigc migrate {source} --as <doctype> --slug <different-slug>`; if the occupant is itself foreign, adopt it through its own `jigc migrate` task first; then re-run `jigc task finalize <id>` to review the fidelity diff and `jigc task finalize <id> --approve` to land it (`--approve` also retires the recorded source)
- `engine/src/finalize.rs:419` · finalize.promote-clobber (plain arm) · [message] promoting this task's doc to `{destination}` would overwrite a file already there — refusing to clobber it
- `engine/src/finalize.rs:423` · finalize.promote-clobber (plain arm) · [route] a file already occupies `{destination}`: if it is another managed doc, retitle this task's doc so it slugs differently, or re-create it with an explicit `--slug` (`jigc doc create <doctype> --title <title> --slug <slug> --task <id>`); if it is a hand-authored/foreign file, bring it under management with `jigc migrate {destination} --as <doctype>` in its own task; then re-run `jigc task finalize`
- `engine/src/finalize.rs:631` · finalize.carried-staged (task arm; {what} = "staged"|"staged for deletion", {kind} = "change"|"deletion") · [message] `{path}` was already {what} before this task existed — refusing to let a pre-task staged {kind} silently ride this task's commit
- `engine/src/finalize.rs:635` · finalize.carried-staged (task arm) · [route] unstage it (`git restore --staged -- {path}`) if it is not this task's work, or re-run the finalize with `--carry-staged` to declare the carry-over deliberate
- `engine/src/finalize.rs:642` · finalize.carried-staged (milestone arm) · [message] `{path}` was already {what} before this milestone existed — the aggregate commit is built from the sub-task worktrees and cannot carry it, so the {kind} stays staged, undeclared, across this boundary
- `engine/src/finalize.rs:647` · finalize.carried-staged (milestone arm) · [route] unstage it (`git restore --staged -- {path}`) if it is stale, or re-run the finalize with `--carry-staged` to declare it deliberate (it stays staged either way)
- `engine/src/finalize.rs:1019` · finalize.base-mismatch (task pin arm) · [message] the task was started at base `{}` but HEAD is now `{head_sha}`
- `engine/src/finalize.rs:1023` · finalize.base-mismatch (task pin arm) · [route] switch back to `{}` or discard the task with `jigc task discard`
- `engine/src/finalize.rs:1029` · finalize.base-mismatch (milestone pin arm) · [message] the milestone was pinned to base `{}` but HEAD is now `{head_sha}`, and the commits landed since move more than milestone-record bookkeeping — the sub-task worktrees were cut from `{}`, so combining them onto HEAD cannot be proven sound
- `engine/src/finalize.rs:1036` · finalize.base-mismatch (milestone pin arm) · [route] land this milestone's work first (out-of-band git: return HEAD to `{}`, run `jigc milestone finalize`, then re-land the newer commits on top), or re-cut this milestone's work onto the new base (out-of-band git: re-provision the sub-task worktrees from HEAD and re-apply each sub-task's staged changes)
- `cli/src/setup.rs:106` · store-version.binary-mismatch (shared prefix) · [message] store last written by jigc {recorded}; you are running {running}
- `cli/src/setup.rs:109` · store-version.binary-mismatch (stale-corpus arm) · [message] {provenance} — and this store's committed docs are stale against {running}'s schemas: re-stamping alone would clear this advisory while the corpus stayed stale
- `cli/src/setup.rs:113` · store-version.binary-mismatch (stale-corpus arm) · [route] run `jigc migrate-corpus` to upgrade the committed docs, then re-run `jigc setup` to re-stamp the store at {running} (or align the running jigc back to {recorded})
- `cli/src/setup.rs:121` · store-version.binary-mismatch (current-corpus arm) · [message] {provenance} — align versions or re-run `jigc setup`
- `cli/src/setup.rs:122` · store-version.binary-mismatch (current-corpus arm) · [route] align the running jigc to {recorded}, or re-run `jigc setup` to re-stamp the store at {running}
- `engine/src/validate.rs:1180` · schema-conformance.schema-version-current (no-stamp arm) · [message] field `{field}` is absent; the committed doc predates the schema-version stamp (below the current schema-version {current})
- `engine/src/validate.rs:1184` · schema-conformance.schema-version-current (stamped-below arm) · [message] field `{field}` is schema-version {s}, below the current schema-version {current}
- `engine/src/validate.rs:1188` · schema-conformance.schema-version-current · [route] migrate — `{rel_key}` is a managed doc below the current schema-version {current}; run `jigc migrate-corpus` to upgrade it
- `engine/src/validate.rs:1224` · route_schema_conformance (no-stamp arm, rides every schema-conformance.* finding on the doc) · [route] migrate — `{rel_key}` carries no schema-version stamp; run the corpus migration to stamp and upgrade it to schema-version {current}
- `engine/src/validate.rs:1228` · route_schema_conformance (stamped-below arm) · [route] migrate — `{rel_key}` is stamped schema-version {s}, below the current {current}; run the corpus migration to upgrade it
- `engine/src/validate.rs:1232` · route_schema_conformance (at-current arm) · [route] corrupt — `{rel_key}` is at the current schema-version {current} but does not conform; review it by hand
- `engine/src/validate.rs:908` · schema-conformance.unadopted-instance / adoption_route (migratable arm) · [route] adopt — run `jigc ingest` to route it, or `jigc migrate {rel_key} --as {ty}` to rewrite it into the managed `{ty}` shape; it is a foreign file, not an unmigrated managed doc
- `engine/src/validate.rs:915` · schema-conformance.unadopted-instance / adoption_route (non-migratable arm) · [route] adopt — run `jigc ingest` to route it; it is a foreign file, not an unmigrated managed doc
- `engine/src/cascade.rs:134` · TargetParseError::message (structural-op target parse diagnostics, each a distinct message) · [message] structural-op target scheme must be `workflow` · structural-op target needs a `workflow:<id>` scheme · structural-op target has an empty workflow id · structural-op target `#<step-id>` is empty · structural-op `after:` / `before:` anchor step id is empty · structural-op target needs a `#<step-id>` or an `after:` / `before:` anchor · structural-op target cannot carry both `#<step-id>` and an `after:` / `before:` anchor · structural-op target ids must be ASCII
- `engine/src/cascade.rs:349` · SlotFillTargetParseError::message (slot-fill target parse diagnostics, each a distinct message) · [message] slot-fill target scheme must be `step` · slot-fill target needs a `step:<id>` scheme · slot-fill target needs a `#<fill-id>` extension point · slot-fill target has an empty step id · slot-fill target `#<fill-id>` is empty · slot-fill target ids must be ASCII

# Part 4 — `jigc doc schema` for the remaining doctypes (agent listing)

## $ jigc doc schema spec

````
doctype: spec (schema-version 1)
fields (* = author-required):
  - derived-from: ref (section: meta) (set-field: spec:<slug>#meta/derived-from)
  - schema-version: int (section: meta) (set: schema-version)
sections:
  - goal: slot (set-slot: spec:<slug>#goal)
  - context: slot (set-slot: spec:<slug>#context)
  - criteria: repeatable (add-item: spec:<slug>#criteria)
    - title: string *
    - maps-to-test: code-anchor (set-field: spec:<slug>#criteria/<id>/maps-to-test)
    - statement: slot (set-slot: spec:<slug>#criteria/<id>/statement)
````

## $ jigc doc schema prd

````
doctype: prd (schema-version 1)
fields (* = author-required):
  - schema-version: int (section: meta) (set: schema-version)
sections:
  - vision: slot (set-slot: prd:<slug>#vision)
  - requirements: repeatable (add-item: prd:<slug>#requirements)
    - title: string *
    - statement: slot (set-slot: prd:<slug>#requirements/<id>/statement)
  - context: slot (set-slot: prd:<slug>#context)
````

## $ jigc doc schema arch-doc

````
doctype: arch-doc (schema-version 1)
fields (* = author-required):
  - cites: ref (section: meta) (set-field: arch-doc:<slug>#meta/cites)
  - schema-version: int (section: meta) (set: schema-version)
sections:
  - overview: slot (set-slot: arch-doc:<slug>#overview)
  - components: repeatable (add-item: arch-doc:<slug>#components)
    - title: string *
    - implemented-by: code-anchor (set-field: arch-doc:<slug>#components/<id>/implemented-by)
    - description: slot (set-slot: arch-doc:<slug>#components/<id>/description)
````

## $ jigc doc schema changelog

````
doctype: changelog (schema-version 2)
fields (* = author-required):
  - schema-version: int (section: meta) (set: schema-version)
sections:
  - unreleased-changes: repeatable (add-item: changelog:<slug>#unreleased-changes)
    - category: enum [added|changed|deprecated|removed|fixed|security] *
    - notes: slot (set-slot: changelog:<slug>#unreleased-changes/<id>/notes)
  - releases: repeatable (add-item: changelog:<slug>#releases)
    - title: string *
    - date: date (set: on-create)
    - link: string (set-field: changelog:<slug>#releases/<id>/link)
    - changes: repeatable (add-item: changelog:<slug>#releases/<id>/changes)
      - category: enum [added|changed|deprecated|removed|fixed|security] *
      - notes: slot (set-slot: changelog:<slug>#releases/<id>/changes/<id>/notes)
````

## $ jigc doc schema vision

````
doctype: vision (schema-version 1)
fields (* = author-required):
  - grounded-in: ref (section: meta) (set-field: vision:<slug>#meta/grounded-in)
  - schema-version: int (section: meta) (set: schema-version)
sections:
  - thesis: slot (set-slot: vision:<slug>#thesis)
  - invariants: slot (set-slot: vision:<slug>#invariants)
  - open-questions: slot (set-slot: vision:<slug>#open-questions)
````

## $ jigc doc schema research

````
doctype: research (schema-version 1)
fields (* = author-required):
  - date: date (section: meta) (set: on-create)
  - schema-version: int (section: meta) (set: schema-version)
sections:
  - question: slot (set-slot: research:<slug>#question)
  - findings: slot (set-slot: research:<slug>#findings)
  - sources: slot (set-slot: research:<slug>#sources)
````

## $ jigc doc schema idea

````
doctype: idea (schema-version 1)
fields (* = author-required):
  - trigger: string (section: meta) (set-field: idea:<slug>#meta/trigger) *
  - date: date (section: meta) (set: on-create)
  - schema-version: int (section: meta) (set: schema-version)
sections:
  - description: slot (set-slot: idea:<slug>#description)
````

## $ jigc doc schema roadmap

````
doctype: roadmap (schema-version 1)
fields (* = author-required):
  - schema-version: int (section: meta) (set: schema-version)
sections:
  - milestones: repeatable (add-item: roadmap:<slug>#milestones)
    - title: string *
    - proves: slot (set-slot: roadmap:<slug>#milestones/<id>/proves)
    - decomposition: slot (set-slot: roadmap:<slug>#milestones/<id>/decomposition)
````

## $ jigc doc schema decisions-log

````
doctype: decisions-log (schema-version 1)
fields (* = author-required):
  - schema-version: int (section: meta) (set: schema-version)
sections:
  - entries: repeatable (add-item: decisions-log:<slug>#entries)
    - title: string *
    - date: date (set: on-create)
    - why: slot (set-slot: decisions-log:<slug>#entries/<id>/why)
````

## $ jigc doc schema deferral-ledger

````
doctype: deferral-ledger (schema-version 2)
fields (* = author-required):
  - schema-version: int (section: meta) (set: schema-version)
sections:
  - entries: repeatable (add-item: deferral-ledger:<slug>#entries)
    - title: string *
    - kind: enum [Decision|Idea] (set-field: deferral-ledger:<slug>#entries/<id>/kind) *
    - trigger: string (set-field: deferral-ledger:<slug>#entries/<id>/trigger) *
    - date: date (set: on-create)
    - body: slot (set-slot: deferral-ledger:<slug>#entries/<id>/body)
````

## $ jigc doc schema milestone-record

````
doctype: milestone-record (schema-version 2)
fields (* = author-required):
  - base: string (section: meta) (set: on-create)
  - status: enum [active|joined|discarded] (section: meta) (set: on-transition)
  - schema-version: int (section: meta) (set: schema-version)
sections:
  - tasks: repeatable (add-item: milestone-record:<slug>#tasks)
    - task-id: string (set: on-transition)
    - intent: string (set: on-transition)
    - status: enum [active|joined|discarded] (set: on-transition)
````

## $ jigc doc schema completion-record

````
doctype: completion-record (schema-version 1)
fields (* = author-required):
  - verdict: enum [green|red] (section: meta) (set-field: completion-record:<slug>#meta/verdict) *
  - owner-artifact: owned-location (section: meta) (set-field: completion-record:<slug>#meta/owner-artifact) *
  - schema-version: int (section: meta) (set: schema-version)
sections:
  - findings: repeatable (add-item: completion-record:<slug>#findings)
    - title: string *
    - severity: enum [blocking|advisory] (set-field: completion-record:<slug>#findings/<id>/severity) *
    - disposition: enum [fixed|deferred|contested] (set-field: completion-record:<slug>#findings/<id>/disposition) *
    - evidence: string (set-field: completion-record:<slug>#findings/<id>/evidence) *
````

## Capture notes

- Binary: `~/.local/bin/jigc`, verified `jigc 1.0.0-rc.7`. Throwaway repos under `/home/maurice/.claude/jobs/0c0984fb/tmp/r2b-*/` (one fresh `git init` + `jigc setup` repo per compose). No changes to /home/maurice/Projects/gherrink-jigc (crates/ read only, at HEAD e82bae2).
- All eleven remaining workflows composed successfully by name, plus `--explain`. `implement-from-spec` was reached the honest way: a minimal `plan` drive in the same repo (spec authored via the `jigc doc author` batch verb, first finalize blocked on the empty commit doc — captured — then filled and landed as `docs/specs/notification-service.md`), then the compose + `jigc task bind`. The first bind attempt with the compose's literal `<SPEC_ID>` shape (`notification-service`, no type prefix) was rejected — `malformed address` — and is captured verbatim; the full-address form `spec:notification-service` bound. The post-bind `jigc start --task` resume compose is captured with the grounded spec slice.
- Both migrate templates (changelog placement arm, vision methodology arm) captured in full over committed foreign files.
- Route inventory: mechanical static extraction (script: tmp/r2b-extract.py). Coverage = every `Route::human/mechanical/informational(` construction tree-wide excluding `#[cfg(test)]` bodies, plus finding-message format strings in the 16 named files via their real constructors (`Finding::graded`, `blocking_conformance`, `blocking_write`, store `block`, `let message = format!`). Non-literal arguments surface as `<expr: …>`; the Supplement subsection resolves the multi-variant ones (finalize promote-clobber / carried-staged / base-mismatch, setup binary-mismatch, validate version-currency + conformance-route riders + adoption_route, cascade target-parse enums) verbatim from source. Not swept (out of the named scope): `override_default.rs`, `data_value.rs`, `probe.rs` message bodies (their `Route::` constructions ARE included via the tree-wide pass), and plain CLI error `bail!`/`eprintln!` strings that are not finding/route constructions.
- All 12 requested `jigc doc schema` doctypes resolved (spec, prd, arch-doc, changelog, vision, research, idea, roadmap, decisions-log, deferral-ledger, milestone-record, completion-record) — none unreachable.
- Nothing skipped; no unreachable surfaces this round.
