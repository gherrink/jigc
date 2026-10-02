# doc list field filter — server-side `--where` on the index read

**Status: parked 2026-10-02.** Parked by the M55 Settle, S10 option (c) ([DECISIONS.md](../DECISIONS.md) → 2026-10-02 M55 settled; design of record [design/findings-channel.md](../design/findings-channel.md) → 5). Indexed from [VISION.md](../VISION.md) → Open questions.

## The gap

M55 makes a populated findings store triageable by adding `title` and the header `fields` to every `jigc doc list --format json` row, with an absent defaulted field projected as its default. Sorting, grouping and filtering — *all open feedback, grouped by `found-in`* — then happen client-side, over the json, with `jq` or the agent's own code. The CLI itself filters by doctype only.

## The direction

A filter on the index read: `jigc doc list <doctype> --where status=open` (and perhaps `--where tier=tier-1,tier-2`, `--sort date`), applied by the CLI to the same rows it already projects, the output shape unchanged. The plain listing gains the same filter, so the two arms stay in parity.

## Why parked

Not needed while the S10 keys plus a client-side filter cover the reads M57's triage will make, and it is **additive after 1.0**: a new flag that narrows rows changes no key on the pinned row shape, so it does not need the pre-1.0 window the keys themselves used ([doc-read-surface.md](../design/doc-read-surface.md) → Evolution posture). (a), reading rules only, was not taken either.

## What it would cost

- A small predicate grammar — equality, enum-member sets, absence — whose semantics for a defaulted field agree exactly with the default projection, or a filter and a read disagree about the same row.
- Plain/json parity and help truth for a new flag, and its goldens.
- A line to hold: a filter is not a query language. Anything past field equality — full text, ranking — belongs to [doc-search](doc-search.md)'s layers, and judgment stays with the agent.

## Trigger

Agents measurably avoiding the json-plus-filter path — reading files directly instead of `doc list` — or an agent harness that refuses shell pipelines (the author steps already route payloads through heredocs because some harnesses statically refuse `cat … | jigc …`), which would leave `doc list --format json | jq …` unrunnable exactly where it is needed.
