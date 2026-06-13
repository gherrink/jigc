# Appendix — the extended seed, the 16 citations, and the 8-step no-cue sequence (M19)

**Frozen with the pre-registration.** The M18 `ratelimit` toy project, **extended** into a longer,
denser citation surface, with an 8-step no-cue code-evolution sequence. Both arms' seeds are built
from this; both arm-chains get the identical step list. Every step is a **pure code task that never
mentions `architecture/` / `decisions/` / `specs/`** (the no-cue property — a cold agent has no cue
to audit the docs its change silently breaks).

## The extended `ratelimit` project

Starts from the M18 seed ([../../M18/maintenance-test/appendix-sequence.md](../../M18/maintenance-test/appendix-sequence.md)
— `bucket.rs`, `clock.rs`, `limiter.rs`, `tests/burst.rs`, with the M18 Amendment-1 green test bodies)
and **adds**:

### `src/window.rs` (new — a second rate-limit strategy)
```rust
use crate::clock::Clock;

/// A sliding-window counter: timestamps of recent events, evicting those older than the window.
pub struct SlidingWindow {
    window: u64,
    events: Vec<u64>,
}

impl SlidingWindow {
    pub fn new(window: u64) -> Self {
        SlidingWindow { window, events: Vec::new() }
    }

    /// Record an event at the clock's current tick.
    pub fn record<C: Clock>(&mut self, clock: &C) {
        self.events.push(clock.now());
    }

    /// How many recorded events fall within `window` ticks of now (older ones evicted).
    pub fn count_in_window<C: Clock>(&mut self, clock: &C) -> usize {
        let now = clock.now();
        self.events.retain(|&t| now.saturating_sub(t) < self.window);
        self.events.len()
    }
}
```

### `src/policy.rs` (new — strategy selection)
```rust
/// Which rate-limit strategy a limiter uses.
pub enum Policy {
    TokenBucket,
    SlidingWindow,
}

/// Pick a policy from a capacity hint: tiny capacities favor the sliding window.
pub fn select_policy(capacity: u32) -> Policy {
    if capacity <= 1 {
        Policy::SlidingWindow
    } else {
        Policy::TokenBucket
    }
}
```

### `src/clock.rs` — add a second `Clock` impl
Append a `SystemClock` (a real-time impl) alongside `ManualClock`:
```rust
/// A real-time clock (seconds since an arbitrary epoch); used outside tests.
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> u64 {
        0 // a stub for the toy project; the real impl would read a monotonic source
    }
}
```

### `src/bucket.rs` — add a `tokens_remaining` getter
```rust
impl TokenBucket {
    /// Tokens currently available.
    pub fn tokens_remaining(&self) -> u32 {
        self.tokens
    }
}
```

### `src/lib.rs` — declare the new modules
```rust
//! A toy token-bucket + sliding-window rate limiter.
pub mod bucket;
pub mod clock;
pub mod limiter;
pub mod policy;
pub mod window;
```

### `tests/window.rs` (new)
```rust
use ratelimit::clock::ManualClock;
use ratelimit::window::SlidingWindow;

#[test]
fn window_counts_recent() {
    let mut w = SlidingWindow::new(10);
    let clock = ManualClock { tick: 5 };
    w.record(&clock);
    w.record(&clock);
    assert_eq!(w.count_in_window(&clock), 2);
}

#[test]
fn window_evicts_old() {
    let mut w = SlidingWindow::new(10);
    w.record(&ManualClock { tick: 0 });
    // 20 ticks later, the tick-0 event is outside a 10-tick window.
    assert_eq!(w.count_in_window(&ManualClock { tick: 20 }), 0);
}
```

*(The seed builder confirms `cargo test` is green — 4 integration tests + any unit tests — before
authoring docs; adjust only test BODIES if needed, never the cited symbol/test names.)*

## The 16 citations (identical content in both twins)

**`arch-doc` `architecture/ratelimit-overview.md`** — overview + **8 components**, each a description +
an `implemented-by` anchor:

| Component | `implemented-by` | fate |
|---|---|---|
| Token bucket | `src/bucket.rs#TokenBucket` | **STABLE** |
| Refill scheduling | `src/bucket.rs#refill` | step 1 (rename) |
| Acquire path | `src/bucket.rs#try_acquire` | step 2 (delete/fold) |
| Clock abstraction | `src/clock.rs#Clock` | step 3 (move) |
| Rate limiter facade | `src/limiter.rs#RateLimiter` | **STABLE** |
| Sliding window | `src/window.rs#SlidingWindow` | step 5 (rename) |
| Window eviction | `src/window.rs#count_in_window` | step 6 (rename) |
| Policy selection | `src/policy.rs#select_policy` | step 7 (rename) |

**Four `adr`s** (`decisions/`), each `status` header carrying a `cites-code`:
- `0001` *"Token bucket over a fixed window"* — `src/bucket.rs#TokenBucket` — **STABLE**
- `0002` *"Inject the clock for testability"* — `src/clock.rs#Clock` — step 3 (move)
- `0003` *"Support a sliding-window policy"* — `src/window.rs#SlidingWindow` — step 5 (rename)
- `0004` *"Select the policy at construction"* — `src/policy.rs#select_policy` — step 7 (rename)

**`spec` `specs/rate-limiting.md`** — **4 criteria**, each a statement + a `maps-to-test`:
- *"A sustained burst is rejected"* — `tests/burst.rs#burst_is_rejected` — step 4 (rename)
- *"A steady rate passes"* — `tests/burst.rs#steady_rate_passes` — **STABLE**
- *"The window counts recent events"* — `tests/window.rs#window_counts_recent` — **STABLE**
- *"The window evicts old events"* — `tests/window.rs#window_evicts_old` — **STABLE**

**16 citations. 10 are broken by the sequence (across 4 docs); 6 are stable controls.** All resolve
clean at the start (arm J: `jigc validate` → 0 findings; arm S: every cited `path#symbol` greps to a
real definition).

## The 8-step no-cue sequence (identical, ordered — given to each cold per-step subagent verbatim)

Each step is handed to a **fresh cold subagent** as the **bold imperative only** (the parenthetical
"breaks" notes are for the judge/orchestrator, never shown to the arms). One commit per step. **No
step mentions documentation.**

1. **Rename the `TokenBucket::refill` method to `TokenBucket::replenish` throughout the crate, updating
   call sites; keep `cargo test` green.** *(Breaks arch "Refill scheduling".)*
2. **Remove `TokenBucket::try_acquire`: inline its take-n-tokens logic into `tick_and_take` and delete
   the method; keep `cargo test` green.** *(Breaks arch "Acquire path".)*
3. **Move the `Clock` trait, `ManualClock`, and `SystemClock` from `src/clock.rs` to a new
   `src/time.rs` (module `time`); update `lib.rs` and all `use` paths; keep `cargo test` green.**
   *(Breaks arch "Clock abstraction" AND adr 0002 `cites-code` — 2 citations, 2 docs.)*
4. **Rename the test `burst_is_rejected` to `rejects_sustained_burst` in `tests/burst.rs`; keep
   `cargo test` green.** *(Breaks spec "A sustained burst is rejected".)*
5. **Rename the `SlidingWindow` struct to `SlidingLog` throughout the crate, updating all references;
   keep `cargo test` green.** *(Breaks arch "Sliding window" AND adr 0003 `cites-code` — 2 citations,
   2 docs.)*
6. **Rename `SlidingWindow::count_in_window` (now `SlidingLog::count_in_window`) to `count_recent`,
   updating call sites and tests; keep `cargo test` green.** *(Breaks arch "Window eviction".)*
7. **Rename the `select_policy` function to `choose_policy` in `src/policy.rs`, updating call sites;
   keep `cargo test` green.** *(Breaks arch "Policy selection" AND adr 0004 `cites-code` — 2
   citations, 2 docs.)*
8. **Add a `pub fn tokens_remaining(&self) -> u32` accessor to `TokenBucket` (it already exists in the
   seed — if so, instead add `pub fn is_empty(&self) -> bool` returning whether no tokens remain), with
   a one-line unit test; keep `cargo test` green.** *(Control — touches NO cited symbol. Tests
   over-editing: a faithful arm changes no doc here; the backstop must stay quiet.)*

**Perfect end-state:** all 10 broken citations updated to their new `path#symbol` (or the deleted
`try_acquire` component removed/re-pointed), the 6 stable citations untouched, code green → **0 stale
citations**. Every stale citation in a chain's final state is drift that chain shipped. **Steps 3, 5,
7 each break a citation in *two* docs (an `arch-doc` component and an `adr` `cites-code`)** — a cold
agent that notices the arch-doc but not the ADR (or neither) leaves residual drift the backstop (arm
J) surfaces and the static arm (S) does not.

## Step-2 judging note (a deleted symbol has no valid re-point)

Step 2 deletes `try_acquire`. The honest repair is to **remove** the "Acquire path" component or
**re-point** it to a real symbol (`tick_and_take`). The judge counts the citation **stale** only if it
still names `src/bucket.rs#try_acquire`; removal or a real re-point both count as **not stale**.
