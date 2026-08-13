# M48 handover — start here, then run `/milestone-plan`

Written 2026-08-13 while the context was live, deliberately: the last two handovers were written
after the fact and one of them followed a machine loss. **Nothing below needs reconstructing from a
transcript.**

## State you're inheriting

| | |
|---|---|
| HEAD | `c941fde`, pushed, tree clean |
| Gate | **2561 passed / 0 failed**, fmt + clippy clean — cargo's own exit code, measured unpiped |
| Binary | `1.0.0-rc.10` installed at `~/.local/bin/jigc` |
| M47 | Complete and audited ([VERDICT](../M47/VERDICT.md)) |
| Pre-1.0.0 trial | **Run, verified, recorded** ([RC-pre-1.0/](../RC-pre-1.0/trial-record.md)) |
| Latent-surface sweep | **Discharged** (`20800f7`) — one arm, and the reason there is only one is recorded |
| Refuted set | **Closed** (`fa343dc`) — R1–R3 carry `pinned-by:` citations |

## What you're running

**`/milestone-plan` for M48**, then build. The charter has one home and is **not** restated anywhere —
read it, don't reconstruct it from this file:

**[implementation/decisions-pending.md](../../../implementation/decisions-pending.md) → *The rc.11
wave (M48)***

It carries the provenance, the razor, three tiers (F1–F16 with the now-or-never split), and **four
Settle forks**. Read the trial's [findings-verification.md](../RC-pre-1.0/findings-verification.md)
alongside it — every finding has a live repro block, so nothing needs re-deriving.

## The four things that will shape the Settle

1. **The razor is generous but must still refuse.** *In scope if it makes an existing surface more
   reliable, more discoverable, or cheaper to use correctly — including **new** surface where
   discoverability requires it. Out of scope if it opens a **new domain capability**.* The human
   sequenced it deliberately generous (2026-08-13) on the reasoning that a discoverability defect in
   a *context compiler* is a tax on every future session forever. **Do not quietly narrow it** — but
   do not treat it as unbounded either: a wave that cannot refuse cannot halt, and M47's planners
   halted twice on false premises precisely because they had a boundary to check against.

2. **M46 is adjudicated inside this Settle, entry by entry** (fork 4) — twelve entries, each ending
   with **absorb** / **drain-then-re-count** / **stay-deferred-re-counted**. This was decided rather
   than assumed: several M46 entries defer *because a cheaper drain had not been tried*, and M48 is
   that drain — entry 3 says a ledger over a floor a precision fix removes is capability for a state
   that no longer exists; entry 4 says a guard over a gap punishes the gap. **F1's fix may drain
   entry 4's evidence base before its own Settle runs.** Sequencing M46 first would invert its own
   recorded conditions.

3. **F3 is data loss and its fix shape is a real fork** (fork 1), owed an independent
   `robust-advocate` — M46 entry 11 pre-emptively warned that *"add a `--force` flag"* is the cheap
   framing, so the proposer must not self-frame it.

4. **F1 is the wave's centre of gravity, and its fence is the one genuine design question** (fork 3).
   The gap is countable — **1 of 69 pack step files names `doc show`**, and it is not an authoring
   step. Precedent for the fence is the `states-constraints:` stated-at mold (M43 fork 3 / M44 D5).

## Three things that will bite you

1. **Check the suites before classifying anything as uncovered.** This failure has now hit **five**
   times — twice by agents, once by an adversarial reviewer, and twice by me in this session — always
   the same shape: classifying from the artifact in hand instead of from the code.
   `cargo test -p cli verb_suite_coverage -- --ignored --nocapture` prints the verb → suite map, and
   its green means *named by a suite*, never *fenced*. The rule is `pinning.md` §5; the fence is
   `verb_suite_coverage.rs`. **The sweep's own most useful output came from applying it**: the
   leftover re-`provision` left the sweep because a standing test over its behaviour would have
   pinned data loss as expected output.

2. **Never read an exit code through a pipe.** `cmd | tail` reports *tail's* status. This cost four
   near-misses this session: a false finding about `jigc validate`'s exit code, a false finding about
   a missing changelog item (`head` truncating a render), a `cargo fmt --check` failure nearly
   recorded as clean, and a **full gate reported green that had never been measured**. Redirect to a
   file, capture `$?` directly, then inspect the file. Two of these are in the trial record as
   died-in-verification claims.

3. **The full gate is ~30–50 min, past the 10-minute Bash ceiling.** Run it with
   `run_in_background`, unpiped, appending cargo's own exit code — never a foreground poll loop.

## Owed at wave close

- **The conversion ledger closes, and it is the human's gate.** Every row of
  [findings-verification.md](../RC-pre-1.0/findings-verification.md) → The conversion ledger must
  carry `pinned-by:` or a stated `UNPINNED: <why>`. **The 1.0.0 call is not taken until it does.**
  F1–F16 are uniformly UNPINNED today because the trial ran under *no mid-trial fixes*.
- **M46's twelve entries each carry a disposition** — that is what closes it.
- **Flow 48** in `worked-examples.md`, each arm **iterating its class's axis** from a code-side
  registry rather than pinning the reported repro.
- **1.0.0-rc.11** built and installed *after* the completion audit's fixes, not before (M47's audit
  caught the owed bump unshipped — do not repeat it).
- The usual fold-back: `DECISIONS.md`, the roadmap, `CLAUDE.md`.

## What I'd watch for

The trial's own recommendation, unchanged: **if the fix wave lands and a later probe finds the
discoverability lens a seventh time, that is a signal about the product's shape, not about the
trial.** F1 is the first fix aimed at the *mechanism* (the composed surface never names the read
verb) rather than at another instance of it. Whether that generalises is the real question M48
answers.
