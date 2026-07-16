# The surface contract

**Status: settled at M43 planning (2026-07-16); built across M43.** The record: [DECISIONS.md](../DECISIONS.md) → 2026-07-16 M43 planning: the Settle. Provenance: the lacon trial ([completions/artifacts/RC-lacon/](../completions/artifacts/RC-lacon/trial-record.md)) — zero correctness bugs, a recurring discoverability class: *the capability exists, no composed surface names it*.

Everything jigc prints — composed output, findings, routes, help, author templates, acks — is a **contracted surface**: it obeys three checkable laws, each **fenced where the surface is generated**, plus a written style guide for the judgment tier no fence can hold. The M42 lesson binds throughout: *a census cannot enforce a predicate* — no fence in this doc is a grep-checklist over prose; every fence is either a type that makes the lie unrepresentable, an assert at the seam where the property is decided, or a structural declaration checked against enumerable data.

This doc owns the **laws, their fences, and the style guide**. It deliberately does *not* restate what its neighbours own: the findings envelope, route kinds, and stable `(code, target)` key ([command-output-contract.md](command-output-contract.md)); the route floor's rule text and per-family route strings ([validation.md](validation.md)); the `when:`-line craft format ([workflow-dialect.md](workflow-dialect.md) → the catalog line); the `description:`/`usage:` field boundary ([introspection.md](introspection.md)); the read-surface postures ([doc-read-surface.md](doc-read-surface.md)); the finalize manifest vocabulary ([finalize.md](finalize.md)); the AGENT.md body ([assistant-adapter.md](assistant-adapter.md)).

## The three laws

**Law 1 — nothing lies.** Every claim a surface makes is generated from the thing it describes, or asserted against it. Templates never hand-enumerate what the schema can project; every printed path is repo-real or a typed identity; behaviour claims match knobs; an ack that says "created" distinguishes created from already-existed. The strongest form is making the lie **unrepresentable**: the string is built from the schema / the `Command` value / the enforcing constant, so drift has no representation.

**Law 2 — nothing hides.** Every affordance that is the designated recovery for a state is named by the surfaces that produce that state — resume (`jigc start --task <id>`), what's-left (`jigc task validate <id>`), the staged read (`jigc doc show <addr> --task <id>`), `task list` on a wrong id. Routes and `Run:` lines parse against the real CLI. Any suppression flag (`selectable: false` and kin) carries a machine-visible reason + expiry, asserted at pack-load — a hidden capability whose hiding condition expired is the `decided-task` lesson made mechanical.

**Law 3 — nothing ambushes.** Every constraint is stated where it binds, *before* it can fail: the slot heading-depth ceiling, the `--from-file -` convention, the `--approve`/clobber/retire contract, the slug word-cap — each named in the template/step/help that solicits the write that would trip it. A block whose contract first appears in the block message is an ambush even when the block is correct.

## The fences

Enforcement posture is **per-class, by the existing records** ([DECISIONS.md](../DECISIONS.md):179; the M42 freeze-assert discharge):

- **Render/serialization-seam asserts** on jigc's own producers are `debug_assert!` — enforced by the test suite exercising the seam, never a release-build panic on a user's finding.
- **Pack-load asserts** are release-real, in the pack factory (`make_pack`), because filesystem packs load in production (the M42 lesson). They require the **eager workflow-front-matter sweep** — net-new at M43; today workflows parse lazily. Scope: the **shipped embedded packs** (the factory sees pack bytes only; project-layer cascade deltas are consciously outside these fences — declared bound, not a hole).

### The route fence (law 2)

`Finding.route` becomes an **internal `Route` value** — `Mechanical { argv, tail } | Human(String) | Informational(String)` — constructed at the producer, **serializing byte-identical to today's string** (the pinned golden in `finding.rs` does not move; the wire tagged-union stays deferred, [decisions-pending.md](../implementation/decisions-pending.md)). The fence: a `Mechanical` route **cannot be constructed** from a command that does not parse against the real CLI (`Cli::try_parse_from` over its argv parts, placeholder args substituted). `From<String>` maps to `Human`, so migration is per-producer. The parse assert lives at the **CLI seam** (the engine cannot see clap); it is the third assert on the M42 finding-key seam.

**The route floor widens to blocking validation/gate findings** (revising [validation.md](validation.md)'s advisory scoping — a blocked finalize is the highest-stakes surface and today prints no recovery). Two exemptions are **re-affirmed on their own rationales**: purely-positional parser conformance diagnostics (the located message *is* the repair) and the hook-rejection route-exempt error identity (git's verbatim stderr *is* the correction signal, [finalize.md](finalize.md):78). With the floor true, [assistant-adapter.md](assistant-adapter.md)'s "every finding carries a route" sentence becomes true rather than fixed down.

Route text that lives in anyhow error strings (outside `Finding`) is in scope for the law-2 rewrite and must either become a `Finding` or construct a `Route` for its command spans — an errorish surface is still a surface.

### The schema projection (law 1)

A new compose placeholder **`{{schema:<doctype>}}`** — CLI-filled read path, the `{{source}}` seam mold — renders the resolved schema's section/field/enum tree **and the author-payload skeleton** into the step that solicits the authoring. **Unfed or dangling blocks**: `workflow-refs.schema-ref-resolves` (blocking; fires at the store-scope sweep per workflow per origin pack *and* at compose — the `command-ref-resolves` mold; never silently empty, which would be a new lie). Joins the dialect grammar + the `workflow-refs` check ([workflow-dialect.md](workflow-dialect.md) owns the grammar row). The renderer reads the *resolved current* schema, so a future schema bump re-renders correctly by construction. Migration **judgment** prose (foreign-status mapping, drop-to-prose rules) stays hand-written — that is the style guide's tier.

### The suppression fence (law 2)

Workflow front-matter gains `suppressed: { reason: <text>, expires: <condition> }` required on every `selectable: false` (and any future hiding flag). Asserted at pack-load over the shipped packs: missing ⇒ fail (required-field-shaped, so the loader's unknown-key tolerance cannot defeat it — a `deny_unknown_fields` flip is consciously not taken; project packs may carry vendor keys). `jigc describe` prints the reason for a hidden workflow, so `describe` and the orient catalog stop contradicting each other.

### The catalog shape fence (style guide's floor)

Pack-load asserts on the shipped packs: `when:`/`description:`/`usage:` **present** on every selectable work-workflow (extending the existing catalog-build assert into the factory) and `when:` **mechanically shaped** (one line, period-less, length-capped — the [workflow-dialect.md](workflow-dialect.md) craft rules' checkable half). The when-NOT-clause assert is **declined** — that is prose semantics, the barred census; the when-NOT discipline lives on `usage:` via the review checklist. [introspection.md](introspection.md):84's skip-on-absent stays intact for project-authored workflows (honored via scoping; its :86 shipped-pack obligation is what these asserts implement).

### The stated-at fence (law 3)

Tiered by what the constraint's source is:

- **Seam-generated** where a code-owned constant exists: the slug rule's statement renders from `MAX_WORDS`/`MAX_CHARS` (the constants the manifest already fingerprints); the heading-depth ceiling's statement is co-located with the enforcing check and rendered via the schema-projection generator. Statement and enforcement share one source; drift is unrepresentable.
- **Structural stated-at** for irreducibly-prose contracts (the `--approve`/clobber/retire contract; the staging contract): the soliciting step's front-matter declares `states-constraints: [<finding-code>, …]`; the pack-load assert checks every member of the declared **ambush-class code set** has at least one shipped-pack declarer. Both sides are structural (codes are enumerable since the M42 key work; the declaration is YAML) — no prose-matching. Honest bound: the assert proves the *obligation* is carried, not that the prose is good; prose quality is the review checklist's job.
- **Guide-only** where the constraint is already demonstrated at every solicit (`--from-file -` appears verbatim in ~20 steps and its `Run:` lines fall under the route-parse fence).

### The carryover gate (law 1 + 3 at the commit boundary)

`jigc start` snapshots the staged set (path + staged blob hash) into the task workbench; finalize preflight computes the **carryover set** — index entries whose (path, blob) match the snapshot, i.e. *staged before this task existed*, deterministically. Non-empty ⇒ finalize **refuses** with a blocking routed finding naming the paths; override with `--carry-staged` (the `--approve` mold — undecidable intent converted to a declared one). The manifest labels the carried entries `carried-over` at all four render sites (dry-run forecast, pre-commit print, landed text, landed JSON — the identical-set invariant holds). This revises [finalize.md](finalize.md)'s "surfaced, not prevented" paragraph on its changed factual basis (the lacon A6/A7/A10 evidence postdates the M42 re-affirmation); the M42 left-out **print** settle is distinguished and untouched. Fan-out worktrees are provisioned clean, so the snapshot is trivially empty there. New finding code: `finalize.carried-staged` (blocking, task/finalize funnel, target = the file path).

### The error-code namespace

Not every non-commit is a `Finding` (the [measurement.md](measurement.md):62 rationale). The **error-code vocabulary** — dotted identities carried on `Outcome` into the invocation log (`finalize.commit-rejected`; M43 adds the exit-4 migration review-hold identity) — is hereby its declared home: error codes are dotted like finding codes, must not collide with any pinned finding family, and each new member is added to this list as it ships. Members: `finalize.commit-rejected` (M42) · `migrate.review-pending` (M43).

## The surface style guide (the judgment tier)

For the prose no fence holds. The milestone-completion audit checks pack-prose changes against this list; it is a review checklist, never a machine gate.

- **Workflow catalog descriptions are routing surfaces.** `when:` does one job: disambiguation against the neighbouring workflows — name the discriminating trigger axes (records a decision? test-first? touches managed docs? spec-bound?), the axes an agent actually decides across. `usage:` carries the fuller story **including when NOT to use it** and its nearest-neighbour handoff ("if it renames a documented symbol, pick single-task"). `description:` says what it is, never mechanism. (Format rules: [workflow-dialect.md](workflow-dialect.md); field boundary: [introspection.md](introspection.md).)
- **Step prose leads with the action; constraints before the solicit.** State what the agent is about to do in the first sentence; state every constraint that gates the solicited write *above* the `Run:`/heredoc that solicits it (law 3's prose form — the M42 staging-contract sentence is the mold).
- **Routes name the exact next command for the state at hand** — one command, real, copy-runnable where mechanical; "no action needed" says so in the first clause. (Route kinds and per-family strings: [command-output-contract.md](command-output-contract.md), [validation.md](validation.md).)
- **Help verbs lead with the one-line common case**; contract detail, posture notes, and design-doc pointers go below the fold, never in the subcommand table line.
- **Advisories say what happened + whether to care, in the first clause.** An advisory an agent should usually ignore must read as ignorable in clause one — habituation is a real, measured cost (three trials).
- **Acks state the effect, not just the target.** Created vs already-existed; what a discard threw away; `task minted: <id>`.
- **Verdict words bless legitimate end-states.** `unmanaged`/`unregistered` describe; they do not imply everything must migrate — staying plain is a correct outcome for off-home files (the at-home squatter keeps the one-story adopt-it rule, [doc-read-surface.md](doc-read-surface.md)).

## Honest bounds

- The seam asserts are debug-posture: they hold via the suite, not in a user's release binary (the recorded class posture).
- The pack-load fences cover the shipped embedded packs; project-layer workflow deltas are outside them by declared scope.
- The stated-at fence proves presence-of-obligation, never prose quality; the style guide has no machine teeth beyond the shape asserts, by design (the A-3 rationale — re-scoped at M43 to what it actually protects: agent runtime judgment — plus [ideas/cost-of-enforcement.md](../ideas/cost-of-enforcement.md)'s calibration).
- ~75 literal `jigc` command lines inside pack step prose (author heredocs) carry compose placeholders and are fence-covered only post-compose (the compose-scope `Run:` emission); pre-compose they are style-guide territory.
