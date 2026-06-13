# Appendix — the seed code, the citations, and the frozen per-step edit sequence

**Frozen with the pre-registration.** This fully specifies the experiment: the toy Rust project both
arms start from, the exact doc↔code citations, and the ordered per-step code changes. Both twins are
built from this; both arm-runs get the identical step list.

## The seed Rust project (`ratelimit` — a toy token-bucket lib)

A small, compiling, test-green Cargo project. File contents (verbatim — the seed builder uses these):

### `Cargo.toml`
```toml
[package]
name = "ratelimit"
version = "0.1.0"
edition = "2021"

[lib]
path = "src/lib.rs"
```

### `src/lib.rs`
```rust
//! A toy token-bucket rate limiter.
pub mod bucket;
pub mod clock;
pub mod limiter;
```

### `src/bucket.rs`
```rust
use crate::clock::Clock;

/// A classic token bucket: capacity tokens, refilled at a fixed rate.
pub struct TokenBucket {
    capacity: u32,
    tokens: u32,
    refill_per_tick: u32,
}

impl TokenBucket {
    pub fn new(capacity: u32, refill_per_tick: u32) -> Self {
        TokenBucket { capacity, tokens: capacity, refill_per_tick }
    }

    /// Add `refill_per_tick` tokens, saturating at capacity.
    pub fn refill(&mut self) {
        self.tokens = (self.tokens + self.refill_per_tick).min(self.capacity);
    }

    /// Try to take `n` tokens; returns true and deducts them if available.
    pub fn try_acquire(&mut self, n: u32) -> bool {
        if self.tokens >= n {
            self.tokens -= n;
            true
        } else {
            false
        }
    }
}

/// Advance a bucket one clock tick, then attempt to take one token.
pub fn tick_and_take<C: Clock>(bucket: &mut TokenBucket, clock: &C, n: u32) -> bool {
    let _ = clock.now();
    bucket.refill();
    bucket.try_acquire(n)
}
```

### `src/clock.rs`
```rust
/// A source of monotonically non-decreasing tick counts (injected for testability).
pub trait Clock {
    fn now(&self) -> u64;
}

/// A clock the tests drive by hand.
pub struct ManualClock {
    pub tick: u64,
}

impl Clock for ManualClock {
    fn now(&self) -> u64 {
        self.tick
    }
}
```

### `src/limiter.rs`
```rust
use crate::bucket::{tick_and_take, TokenBucket};
use crate::clock::Clock;

/// The facade callers use: a bucket + a clock + a per-call decision.
pub struct RateLimiter<C: Clock> {
    bucket: TokenBucket,
    clock: C,
}

impl<C: Clock> RateLimiter<C> {
    pub fn new(capacity: u32, refill_per_tick: u32, clock: C) -> Self {
        RateLimiter { bucket: TokenBucket::new(capacity, refill_per_tick), clock }
    }

    /// Allow this call? (advances one tick, then takes one token)
    pub fn allow(&mut self) -> bool {
        tick_and_take(&mut self.bucket, &self.clock, 1)
    }
}
```

### `tests/burst.rs`
```rust
use ratelimit::clock::ManualClock;
use ratelimit::limiter::RateLimiter;

#[test]
fn burst_is_rejected() {
    // capacity 2, refill 1/tick: a 3-call burst at the same tick exhausts the bucket.
    let mut rl = RateLimiter::new(2, 1, ManualClock { tick: 0 });
    assert!(rl.allow());
    assert!(rl.allow());
    // third within the same window: refill adds 1 but capacity was 2 -> only 1 spare; rejected after.
    let third = rl.allow();
    let fourth = rl.allow();
    assert!(!(third && fourth), "a sustained burst must eventually be rejected");
}

#[test]
fn steady_rate_passes() {
    // one call per tick stays under the refill rate forever.
    let mut rl = RateLimiter::new(2, 1, ManualClock { tick: 0 });
    for _ in 0..10 {
        assert!(rl.allow());
    }
}
```

*(The seed builder confirms `cargo test` is green before authoring docs. If the burst arithmetic
needs a tweak to be genuinely green, adjust the test bodies — NOT the cited symbol names — and note it.)*

## The doc↔code citations (the starting honest state — identical content in both twins)

**One `arch-doc`** (`architecture/ratelimit-overview.md`, title *"Rate limiter architecture"*):
overview prose + **5 components**, each a description + an `implemented-by` anchor:

| Component | `implemented-by` anchor | fate in the sequence |
|---|---|---|
| Token bucket | `src/bucket.rs#TokenBucket` | **STABLE** (control — never mutated) |
| Refill scheduling | `src/bucket.rs#refill` | step 1 (renamed) |
| Acquire path | `src/bucket.rs#try_acquire` | step 2 (deleted/folded) |
| Clock abstraction | `src/clock.rs#Clock` | step 3 (moved) |
| Rate limiter facade | `src/limiter.rs#RateLimiter` | **STABLE** (control) |

**Two `adr`s** (`decisions/`):
- `0001` *"Token bucket over a fixed window"* — `cites-code: src/bucket.rs#TokenBucket` — **STABLE**.
- `0002` *"Inject the clock for testability"* — `cites-code: src/clock.rs#Clock` — step 3 (moved).

**One `spec`** (`specs/rate-limiting.md`, title *"Rate limiting"*): overview + **2 criteria**, each a
statement + a `maps-to-test` anchor:
- *"A sustained burst is rejected"* — `maps-to-test: tests/burst.rs#burst_is_rejected` — step 4 (renamed).
- *"A steady rate passes"* — `maps-to-test: tests/burst.rs#steady_rate_passes` — **STABLE** (control).

**9 citations total. 5 are broken by the sequence; 4 are stable controls** (an arm that over-edits
or that jigc false-flags would touch a stable one). All 9 resolve clean at the start.

## The frozen per-step edit sequence (identical, ordered — given to both arm subagents verbatim)

Each step is a **code task**; none mentions documentation. The task list handed to the subagent is
exactly the **bold imperative** lines below (the parenthetical "breaks" notes are for the judge/
orchestrator, **not** shown to the arms). One commit per step.

1. **Rename `try_acquire` is NOT this step — rename the refill method: rename `TokenBucket::refill` to
   `TokenBucket::replenish` throughout the crate, updating all call sites; keep `cargo test` green.**
   *(Breaks `arch-doc … Refill scheduling → src/bucket.rs#refill`.)*

2. **Remove the `try_acquire` method: inline its take-n-tokens logic directly into `tick_and_take` (so
   `tick_and_take` decrements the bucket itself) and delete `TokenBucket::try_acquire`; keep
   `cargo test` green.**
   *(Breaks `arch-doc … Acquire path → src/bucket.rs#try_acquire`.)*

3. **Move the clock module: relocate the `Clock` trait and `ManualClock` from `src/clock.rs` to a new
   file `src/time.rs` (module `time`), update `lib.rs` and all `use` paths; keep `cargo test` green.**
   *(Breaks BOTH `arch-doc … Clock abstraction → src/clock.rs#Clock` AND `adr 0002 cites-code →
   src/clock.rs#Clock` — one move, two stale citations, the realistic multi-doc break.)*

4. **Rename the burst test: rename the test function `burst_is_rejected` to `rejects_sustained_burst`
   in `tests/burst.rs`; keep `cargo test` green.**
   *(Breaks `spec … A sustained burst is rejected → tests/burst.rs#burst_is_rejected`.)*

5. **Add a capacity accessor: add a `pub fn capacity(&self) -> u32` method to `TokenBucket` returning
   its capacity, and add a one-line unit test for it; keep `cargo test` green.**
   *(Control step — touches NO cited symbol. Tests over-editing: a faithful arm changes no doc here;
   `jigc validate` must stay quiet. An arm that rewrites docs here is over-editing.)*

**Expected end-state for a perfect arm:** all 5 broken citations updated to their new `path#symbol`
(refill→replenish; Acquire-path component removed or re-pointed since the symbol is gone; clock path
→ `src/time.rs#Clock`; test → `rejects_sustained_burst`), the 4 stable citations untouched, code
green → **0 stale citations**. Every stale citation in the final state is a drift the arm shipped.

## Amendment 1 (pre-run, no arm has run — 2026-06-13): the burst-test body

The `tests/burst.rs#burst_is_rejected` body as first written was **not green** (with capacity 2 /
refill 1, every `allow()` refills before it takes, so a burst is never rejected). The test **body**
was corrected — **byte-identically in both twins** — to use a non-refilling bucket
(`RateLimiter::new(2, 0, …)`), drain the 2 starting tokens, then assert the 3rd and 4th calls are
both rejected. The cited function **names** (`burst_is_rejected`, `steady_rate_passes`) are unchanged,
so every `maps-to-test` citation is unaffected. This is the seed's only deviation from the verbatim
code above; both twins remain byte-identical in code and carry the identical 9-citation set.

**Seeds verified (orchestrator, pre-run):** code byte-identical across twins; the 9 `path#symbol`
citations identical (same multiplicities); twin J `jigc validate` → 0 findings; twin S has no `.jigc`
and no jigc mention in `CLAUDE.md`; `tests/burst.rs` green (2 passed) in both. Seeds at
`/home/maurice/jigc-dogfood/mdt-seed-{J,S}`. **De-anonymization note for judging:** twin J's ADR
filenames are title-slugs (`token-bucket-over-a-fixed-window.md`), twin S's are numbered
(`0001-…md`) — a residual tell to neutralize when de-identifying for the judge.

## Step-2 judging note (a deleted symbol has no valid re-point)

Step 2 deletes `try_acquire` entirely (folded into `tick_and_take`). There is no symbol to re-point
the "Acquire path" citation at — the honest repair is to **remove that component** (or re-point it to
`tick_and_take` if the arm judges the documented capability moved there). The judge counts the
citation **stale** only if it still names `src/bucket.rs#try_acquire` (a symbol that no longer
exists); removing the component or re-pointing to a real symbol both count as **not stale**.
