# M49 — handover to the planning session

**Read [charter.md](charter.md) first.** This file is the state of the world, the traps, and what
to do in what order. The charter is *what*; this is *where you are standing*.

## State of the world

| | |
|---|---|
| Branch | `main`, clean, **not pushed** |
| Binary | `1.0.0-rc.12`, built from `5ff85ea` |
| Images | `jigc-gate:rc10`, `rc11`, **`rc12`** — rc12 gated, 7/7, record at [../RC-1.0-final/gate-rc12.json](../RC-1.0-final/gate-rc12.json) |
| The 1.0.0 call | **the human's, unblocked** — the trial found no blocking findings |
| This repo | **not self-hosted.** No `.jigc/`, no `docs/roadmap.md`. Every port claim assumes that migration happens first |

**The trial that chartered this wave:** [../RC-1.0-final/](../RC-1.0-final/) — protocol, corpora,
plants, prompts, answer key, coverage, session findings, per-claim verification, archived evidence.
Six sessions + an 11-arm walk, **118 bars, zero failures, zero blocking findings**.

## Do these in this order

**1 · Baseline against the binary, not this charter.** Every wave since M45 has had charter
premises corrected by a baseline that exercised HEAD — **M48's corrected 19**. This charter was
written by the session that ran the trial, which is exactly the position that produces confident
wrong premises. Two of its rows are already flagged as unverified or borderline.

**2 · Drive the one unverified port claim** before scoping it: does
`milestone add-task --workflow <other>` + `milestone execute` really emit a spawn the W-equality
re-entry guard refuses? Routing dead end → in. Capability gap → out.

**3 · Settle N1 and N2.** Both are one-way, both contract-touching, both parked *for* a deliberate
call. **N2's record explicitly warns that a fix wave must not decide it by accident** — so it is a
fork with an argued decision, never a fix someone slips in.

**4 · Write D1's and D5's rules, or drop both and say so.** They fail the razor's first leg today
**by their own refusals' terms**. The rule is the deliverable, not a formality.

**5 · Then decompose.** Not before.

## Traps

- **The razor must be able to refuse.** M46's refused **nine** items with citations, three carrying
  measured evidence. *If the razor cannot refuse, the claim is wrong* — that is the wave's own
  falsifier, adopted verbatim. Watch for scope creep from Tier 3 and from the port's shape items,
  which fail leg 1 and would make the claim unfalsifiable if admitted.
- **"Prose and routing" will try to swallow the backlog.** Seven consecutive discoverability
  landings say that backlog is effectively unbounded. The claim — *apply each shipped fence over
  its axis* — is what bounds it.
- **A fix that is not swept over its axis will be back.** The trial's own instrument produced
  **an incomplete fix to one of its own fixes**; treat this wave's fixes the same way.
- **An agent's report is a lead, not a measurement.** The two backlog sweeps behind this charter
  were both good and both had to be corrected: one invented a `pinned-by:` symbol that does not
  exist, and one asserted a pre-1.0 cost advantage that this repo's lack of self-hosting removes.

## What is already verified, so you need not re-derive it

Each measured at HEAD during the charter, with the command:

- packs name `doc show` **34×**, `doc schema` **0**, `doc list` **0**
- `doctype-authoring.md` names **none** of the M43–M48 pack-load fences (0 hits)
- `module-layout.md:58` still calls a question settled 2026-05-28 *"an open question"*
- `is_route_exempt` takes a **finding code**, so an anyhow refusal is outside the route floor
- `main.rs`: *"Every other clap error kind still prints clap's own render"* — the seam covers 2 kinds
- `schema-version` is `2` (int) on `doc schema` and `"2"` (str) on `doc show`
- `diff_item_fields` filters item-block leaves to `item_field`, so a nested repeatable classifies
  as nothing and the empty-diff backstop refuses the migration
- `robust-advocate`: **0 hits** across both packs
- `execute.yaml` says *"strictly serially… one working tree"*; `implement-tasks.yaml` fans out over
  per-sub-task worktrees

## Apparatus owed — trial tooling, not product, and not this wave

Kept out of the charter deliberately so the wave's claim stays about the product:

1. The answer key's `scope` pattern missed twice while its **reply was right both times** — widen
   it to cover *deliverable* and the numbered-options shape.
2. Nothing marks invocation-log records made **during** a session by someone other than the worker.
   The `session-start` split handles *before*; an operator probe against a live container is
   invisible. Mitigation today is discipline (reproduce in a copy).
3. `completions/trial-corpus-template`: `IngestQueue` is dead on the live path and **plant E's
   subject sits on it** — wire `tick()` in, and add a `check-corpus.sh` bar that fails when a
   symbol the plants depend on is unreachable.
