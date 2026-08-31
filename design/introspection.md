# `describe` — the self-description surface

`jigc describe` — an on-demand **prose self-description** of jigc's resolved surface: *what doc-types / workflows / commands this project has and how they're used*. Any LLM can call it to orient, or be pointed at it. It is the **introspection read-path surface**: a projection of the resolved definitions (`project > team > pack-default`) into human-readable prose.

Promoted from `ideas/describe.md` (direction locked 2026-05-30; open thread settled at M11 planning 2026-06-07 — see [DECISIONS.md](../DECISIONS.md)). This is the home of record.

## Purpose & audience

Orientation for *any LLM that wants to understand the project*. Who calls it doesn't matter — because nothing relies on the output, the earlier "host author" vs "runtime agent" split collapses. Distinct from its neighbours:

| Surface | Audience | Answers |
|---|---|---|
| **bootstrap** (advertise+demonstrate) | runtime agent | "what do I do *now*" |
| **`jigc start --explain`** | composer / debugger | "how does *this composition* resolve through the cascade" |
| **validation** | composer | "is *this* composition correct" |
| **`describe`** | any LLM wanting orientation | "what *can* be composed here, and how is it used" |

describe is the **menu**; validate is the **check**; bootstrap is the **runtime nudge**; `--explain` is the **resolution trace**.

> **Claim-vs-reality note (corrected at M11 planning).** Earlier framing positioned `--explain` as "human, one command-ref — what does *this command* do." No such command-ref `--explain` exists in the code. The only built `--explain` is `jigc start --explain`, which prints the **cascade resolution tree** over a composed workflow (for the composer/debugger). describe is genuinely net-new output, not an extension of an existing single-command surface.

## The load-bearing constraint — facts, not advice

describe emits *what exists and how it's used*, never *"you should fan out / compose these."* Advisory output would hand structural-composition judgment back to an LLM — the determinism thesis inverted one meta-level up. It speaks **usage and intent, never mechanism**: how a thing functions internally stays hidden behind jigc and its definitions. (Reading *filled* prose stays [`jigc doc show`](doc-read-surface.md) — a different verb: the committed-doc read surface, whose slice grammar + pinned 1.0 `--format json` contract are homed in [doc-read-surface.md](doc-read-surface.md).)

**Who enforces which constraint — be honest about the split.** describe carries two constraints, and they have *different* enforcement owners:

- **Facts-not-advice is author/review-enforced, not CLI-enforced.** The prose is human-authored and the CLI makes no LLM call, so the CLI *cannot* mechanically tell "facts" from "advice" — it assembles whatever prose the definition carries. Keeping the prose factual is authoring discipline, caught at review, not a gate the binary runs.
- **Non-contractual is format-enforced** (next section) — this one the binary *can* hold, via the output's shape.

This split is the same adapter-not-sandbox honest boundary the rest of the system makes: the CLI enforces what it structurally can (format), and trusts authoring discipline for what it can't (content).

## Derived from configuration — assembles, does not author

Output is a **projection of the resolved definitions** (`project > team > pack-default`) into prose. Consequences:

- It can't drift from reality — it's generated from the same definitions that drive composition.
- It reflects the cascade for free (see Cascade resolution).
- Because the CLI core makes **no LLM calls**, the prose is **human-authored `description:` / `usage:` fields carried on the definitions**, which describe *assembles* through the cascade — never LLM-generated.

This is the document model applied to jigc describing itself: structure owned by the CLI, prose authored by a human, assembly deterministic.

## Non-contractual by design — enforced by format

This **dissolves** (rather than negotiates) the "not a public API" non-goal: if nothing may depend on the output, there is no stability obligation to violate.

The enforcement lever is **format**: the output stays prose / discursive — *hostile to parsing* — rather than a structured contract that invites dependence. The format choice is the enforcement mechanism, not cosmetics.

**The operational format contract (what makes "hostile to parsing" testable).** "Non-contractual" is only real if it is checkable, otherwise it is intent dressed as enforcement. The acceptance bar asserts **positive format properties** of the describe output — each pinned precisely so a legitimate prose output passes and a parseable catalog fails:

- The **prose surface** is **not valid JSON**, and **does not parse as a top-level YAML mapping or sequence** (a *bare scalar* parse is fine and expected — prose *is* a YAML scalar; what's forbidden is structure a consumer can deserialize into keyed fields/items).
- It has **no key-shaped lines**: no line matches `^\s*[\w-]+:\s` (line-anchored — authored prose **may** contain mid-line colons like "e.g." or "Reach for it:"; the shipped prose must simply not *begin a line* with a colon-term). No bullet rows either (`^\s*[-*]\s`).
- It has **no per-definition extractable key or delimiter** — a consumer cannot programmatically pull `single-task`'s `usage` out as a field. *Light prose section grouping* (an unkeyed transition like "The workflows you can compose here…") is permitted as a non-binding reading aid; what's forbidden is a stable per-definition handle.
- It reads as **prose paragraphs** (a sentence/prose-density floor), not a list.

**The predicate is scoped to the prose surface, and the `--format json` arm states its own posture (M47 Inc 10 / T6).** The four properties above are properties of the default `agent`/`human` output — the surface this doc's enforcement-by-format argument is about. The global `--format json` arm has always routed describe's projection through the generic serde renderer and emitted a keyed object, so reading the predicate as a claim about *every* arm made the doc — and `jigc describe --help`, which said *"don't parse it"* — contradict an arm the same binary serves (RC-alpha4's settle-by-doing gap; [surface-contract.md](surface-contract.md) law 1). The correction is scope, not a surface change: **nothing is added or removed.** The json arm is **not exempt from non-contractual** — it is *unpinned* rather than *unparseable*: no version governs it and any pack edit may move it, so a driver that depends on it has no promise to hold jigc to. A structural read a driver *can* depend on is `jigc doc schema`, the separately-versioned contract named below. The help says exactly this.

**The suppression is a *shape* on that arm, not a substring (M48 Inc 7 / T5).** The projection's per-definition entry carries **`router_hidden`** — the declared `suppressed.reason` for a workflow hidden from the router catalog (below), `null` otherwise — beside the woven `prose` that narrates the same fact. Until M48 the state existed *only* inside that prose sentence, so recovering it meant substring-matching the one surface this doc makes deliberately hostile to parsing: a fact computed and printed, withheld from the arm that is allowed to be read ([command-output-contract.md](command-output-contract.md) → Evolution posture, the M48 discharge). The prose tier is untouched and stays unpinned; the key changes what the *json* arm says, not what describe *is*. The engine still assembles and never generates — the reason is the pack-authored string carried verbatim, exactly like `description:` / `usage:`.

Together these make depending on the output structurally unattractive. (Snapshot tests of the *bytes* are necessary-but-insufficient — a snapshot of a bulleted list passes a byte-snapshot happily while *failing* non-contractual; the format predicate is a separate, positive assertion.) The format predicate is also why the M40 **`jigc doc schema`** verb — a *structural* projection of the resolved schemas, machine-readable the day it ships — **cannot ride describe**: it ships as its own **separately-pinned, explicitly versioned** contract instead ([doc-read-surface.md](doc-read-surface.md) → Why json is a contract here).

## The authored fields (settled M11)

The prose is carried in **authored fields on the definitions themselves** — a genuinely new, third prose category, distinct from both sides of the determinism boundary:

- not a **`<<slot>>`** (write-path, LLM-filled, per *instance*),
- not a **`{{placeholder}}`** (read-path, CLI-resolved at compose),
- but **authored-at-definition-time usage prose, projected on read**.

**Two fields**, with a policed boundary:

- **`description:`** — what the thing *is* (its identity, one or two sentences).
- **`usage:`** — when and why you'd reach for it.

Neither field describes *how it works internally* (the usage-not-mechanism constraint). The boundary must be policed at authoring/review: a `description:`/`usage:` that drifts into mechanism or advice is an authoring defect.

**The two fields are load-bearing, not cosmetic — the renderer weaves them into a structured sentence**, e.g. "*single-task* is one end-to-end scoped change. Reach for it when the work is one coherent change you can hold in your head." The two fields are two semantic slots the projection composes, not one concatenated blob — that is what justifies the split over a single field. **Partial presence:** a definition with only one of the two narrates *that one* (description-only → "X is …"; usage-only → "Reach for X when …"); **both absent → not narrated** (skip-on-absent).

**Which definition types carry them (the floor):**

| Definition type | Carries `description:`/`usage:`? | Notes |
|---|---|---|
| **workflows** | yes | front-matter fields alongside `when` |
| **doctypes** (schemas) | yes | doc-type-level fields (the schema's first authored-prose fields beyond per-slot `hint`) |
| **command-refs** | **no new field** — describe **projects the existing `hint`** | command-refs already carry a required `hint`; minting a parallel `usage` would duplicate it. describe consumes `hint`; it is `hint`'s first projection consumer (`hint` was previously defined-but-unprojected). **Projected pack-only in M11** — the catalog is read pack-only and catalog override-deltas are unbuilt, so a `hint` override is *not* cascade-reflected (see Cascade resolution). |
| **data-value roots** | **out of scope** | roots are hardcoded engine match arms with no definition layer to carry prose; describing them would require inventing a root-registry — a separate, milestone-sized piece. Cut from M11. |

**Optional, skip-on-absent.** The fields are optional; a definition that omits them is simply **not narrated** (describe is a menu, not a gate). This deliberately diverges from the `when`-hint precedent (which *hard-errors* for a selectable work-workflow) — a missing menu line is not a correctness failure, unlike a missing router selection hint.

> **Deliverable scope (not just runtime contract).** skip-on-absent is the *runtime* behavior for a project that omits the fields — it is **not** a license to ship an under-narrated pack. The M11 pack-prose deliverable authors `description:`/`usage:` on **all shipped workflows + doctypes**, so `jigc describe` against the stock pack produces a full menu (not a two-entry stub). The acceptance asserts over the real shipped definitions, not a fixture pack shaped to the renderer.

> **Two distinct schema changes, not one (build note).** The doctype `Schema` struct is `#[serde(deny_unknown_fields)]`: the struct field must land **before** any schema YAML carries the key, or every schema fails to parse, and the change ripples into the schema golden snapshots. Workflow front-matter is *not* `deny_unknown_fields`: a `description:`/`usage:` added to a workflow YAML is **silently ignored** until the struct + loader read it (the "authored it but describe shows nothing" trap). Treat workflow-field and doctype-field as separate tasks.

## Cascade resolution — whole-file definition shadow (settled M11)

describe must **reflect the resolved cascade**: a project override of a definition's authored prose visibly changes the projection. This is the headline proof — *it can't drift, because it's generated from the same definitions that drive composition.* (Constraint: describe shows usage prose, not step structure, so reflecting the cascade *means* reflecting overridden prose — there is no other surface for it to reflect.)

**Scope of the cascade-reflection proof (disclosed, not papered over).** M11 proves cascade-reflection on **workflows + doctypes only** — the definition types whose files describe reads through `file_owner`. **Command-ref `hint` is projected pack-only** — *pack-only* meaning outside the project/team cascade, not "one pack": since M49 every constituent pack's catalog is projected (Command surface, above), but none of them through `file_owner`. The catalog is read outside the cascade today and catalog override-deltas are unbuilt ([command-catalog.md](command-catalog.md) → Cascade override behavior is a design claim, not yet code), so a project cannot override a `hint` and see describe reflect it. This is the M4 "resolver-unwired" shape named openly: describe's headline proof is whole on workflows/doctypes, and the command-ref surface is honestly partial until the catalog cascade path exists. The acceptance override-proof therefore shadows a *workflow or doctype* file, never a command-ref.

The resolution model is **whole-file definition shadow** — the same mechanism steps already use (`file_owner` selects the highest-precedence layer's whole file):

- A project overrides a definition's prose by shadowing the **whole definition file** at the project layer (`.jigc/config/workflows/<id>.yaml`, `.jigc/config/schemas/<id>.yaml`) — new shadow dirs, a faithful extension of the existing `steps/` shadow dir.
- describe reads each definition through `file_owner`, so the winning layer's prose wins (**replace at file granularity**).
- **No field-level merge.** This honors [overrides.md](overrides.md)'s invariant *"shadowing is atomic at file level — no field-level merge across layers."* Authored metadata on a definition resolves like the definition's other fields: from the winning file, whole.

The cost — overriding one `usage:` line means restating that definition's whole YAML at the project layer — is the same price the design already charges for overriding a step, and acceptable for a rare, non-contractual menu override.

**Deferred (logged with a trigger):** *field-granular* per-definition-metadata override (changing one field without restating the file). It would pressure overrides.md's no-field-level-merge invariant and has no precedent; deferred until a real need recurs across milestones (see [decisions-pending.md](../implementation/decisions-pending.md)). append-merge of prose across layers is explicitly **not** pursued (it would be the first value-combining merge in the cascade and contradicts the invariant for a non-contractual aesthetic).

**Team layer** is plumbed but never fed in production today; M11 acceptance proves `project > pack-default` reflection only.

## Command surface (settled M11)

- **`jigc describe`** — whole-menu by default, **no positional argument**. Emits the prose projection of every resolved workflow + doctype (and command-ref `hint`s). A single-item form (`jigc describe <id>`) is **not** built — it would blur the `describe` / `--explain` boundary; revisit only if a real need appears. **The foreclosure stands unchanged after M48's kind filter** (below): a filtered menu is still *the menu*, so the filter does not reach the boundary this bullet draws.
- **The kind filter (M48)** — `--workflows` / `--doctypes` / `--commands` select **which kinds of entry** the menu returns. They **combine**, and selecting none returns the whole menu (today's output, byte-unchanged). The filter selects **membership, never prose**: a returned entry reads exactly as it reads in the whole tour, so the operational format contract above is untouched and its acceptance re-runs over every filtered arm. The selection is applied to the *assembled* projection, so the prose surface and the `--format json` envelope return the same membership by construction. Why it earned a flag: the whole menu had grown to ~24 kB — 33 workflows (18 of them narrating the router-hidden clause, 12 of them `migrate-*`) plus every doctype plus every composed pack's command-refs on one 1,167-char line — so an agent reading it to find one kind of thing paid for all three.
- Enumeration is over the **unfiltered** definition set (every workflow, not the selectable-only catalog `{{catalog}}` surfaces, which filters to `creates-task && selectable`). Since M43, a hidden workflow's entry additionally says it is **hidden from the router catalog** and carries its declared `suppressed.reason` ([surface-contract.md](surface-contract.md) → The suppression fence) — so describe and the orient catalog stop contradicting each other about what exists. **M49 widens that clause to the catalog's whole complement**: a workflow is off the catalog either by `selectable: false` or by `creates-task: false`, and the fence now requires the declaration for both, so `router` / `ingest-existing` / `increment` stopped being absences with no stated reason.
- **The command surface is the union of every declaring pack's catalog, attributed (M49).** describe read `config/commands` through the composite's winner-take-all whole-file `read`, so the `[dev ▸ methodology]` composition an ordinary `jigc setup` project runs under projected the dev catalog and nothing else — while composition resolves each workflow's `{{cli.<id>}}` against **its own** origin pack's catalog ([multi-pack.md](multi-pack.md) → Pack-local body-reference resolution), so eleven methodology command-refs that every methodology workflow really composes appeared on no menu at all. That is law 2 on the surface that exists to name what is available. The projection therefore carries **one entry per declaring pack**, keyed `(id, pack)` and sorted by it: two packs may declare the same id with a different argv and a different `hint`, and both are genuinely reachable, so the entry names the pack whose catalog declares it — in prose (`<id> (<pack> pack) <hint>`) and as a `pack` key in the `--format json` envelope. The enumeration uses `PackSource::origin_packs`, the same accessor the pack-load fences use to reach every constituent rather than the winner. This is **not** a cascade-reflection change: the `hint` is still projected pack-only in the sense below (a *project layer* cannot override one), and the pack-set is not an adjudicated id-space for `config/commands` at all ([multi-pack.md](multi-pack.md) → What the header deliberately does not adjudicate).
- The output is rendered as discursive prose per the format contract above; the routing footer convention applies (agent/human surfaces, not JSON — though describe's whole point is to *not* be JSON-shaped).

**Deferred:** **left-behind pointers** — advertising `jigc describe` from composed-workflow output / the orientation footer. It touches the bootstrap/emit path (scope beyond a standalone read command) and is a secondary affordance; describe works when called directly. Revisit post-M11 (a single orientation-footer line is the likely minimal form).

## On the reading order

Slots among the integration layers: its dependencies are the cascade ([overrides.md](overrides.md)), the definition format ([workflow-dialect.md](workflow-dialect.md)), the doctype schema ([document-type-schema.md](document-type-schema.md)), and the command catalog ([command-catalog.md](command-catalog.md)) — so it reads *after* those, alongside the other read-path/integration surfaces.
