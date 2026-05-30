# `describe` — self-description surface

**Status: parked, post-MVP.** A shaped design direction, not built and not yet a `design/` part-doc. Direction locked in a 2026-05-30 design conversation; see [DECISIONS.md](../DECISIONS.md) for the *why* and [VISION.md](../VISION.md#open-questions) for the index entry. Notation illustrative.

A self-contained, on-demand **prose self-description** of jigc's resolved surface — *what doc-types / workflows / commands this project has and how they're used* — that an LLM can call when it wants more, or be pointed at via left-behind pointers in composed output.

## Purpose & audience

Orientation for *any LLM that wants to understand the project*. This deliberately collapses the earlier "host author" vs "runtime agent" split — because nothing relies on the output, who calls it doesn't matter. Distinct from neighbours:

| Surface | Audience | Answers |
|---|---|---|
| **bootstrap** (advertise+demonstrate) | runtime agent | "what do I do *now*" |
| **`--explain`** | human, one command-ref | "what does *this* command do" |
| **validation** | composer | "is *this* composition correct" |
| **`describe`** | any LLM wanting orientation | "what *can* be composed here, and how is it used" |

describe is the **menu**; validate is the **check**; bootstrap is the **runtime nudge**.

## The load-bearing constraint — facts, not advice

Emits *what exists and how it's used*, never *"you should fan out / compose these."* Advisory output would hand structural-composition judgment back to an LLM — the determinism thesis inverted one meta-level up. It speaks **usage and intent, never mechanism**: how it functions stays hidden behind jigc and its definitions. (Reading filled prose stays `jigc doc show`, a different verb.)

## Derived from configuration — assembles, does not author

Output is a **projection of the resolved definitions** (`project > team > pack-default`) into prose. Consequences:

- It can't drift from reality — it's generated from the same definitions that drive composition.
- It reflects the cascade for free.
- Because the CLI core makes **no LLM calls**, the prose must be **human-authored `description:` / `usage:` fields carried in the definitions**, which describe *assembles* through the cascade — not LLM-generated.

This is the document model applied to jigc describing itself: structure owned by the CLI, prose authored by a human, assembly deterministic.

## Non-contractual by design, prose-shaped to stay that way

This **dissolves** (rather than negotiates) the "not a public API" non-goal: if nothing may depend on it, there is no stability obligation to violate.

"Nothing relies on its consistency" is intent, not enforcement — the same adapter-not-sandbox honest-boundary bet the rest of the system makes. The enforcement lever is **format**: keep the output prose / discursive — *hostile to parsing* — rather than a structured contract that invites dependence. The format choice is the enforcement mechanism, not cosmetics.

## Open thread — the authored description field

The substantive post-MVP design work hiding here:

- **Which definitions carry `description:` / `usage:`** — doc-types, workflows, command-refs, data-value roots?
- **Required vs optional** per definition type.
- **Cascade merge semantics** — replace vs append when a project overrides a pack's authored prose.

## When de-parked

Promote to a `design/introspection.md` part-doc (new audience + new read-path surface; not folded into `bootstrap.md`), wire it into the `design/` reading order in [CLAUDE.md](../CLAUDE.md), and move the VISION index entry from "parked" to "resolved → design/".
