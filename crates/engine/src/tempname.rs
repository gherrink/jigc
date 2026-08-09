//! The workspace's **process-unique disambiguator** for throwaway path names.
//!
//! Every site that mints a scratch path — a test's temp repo, a store-sweep
//! snapshot, an atomic write's temp sibling — needs a name no *other* live mint can
//! produce. The idiom is `pid + nanos`: `pid` separates processes, and the clock is
//! assumed to separate the calls inside one.
//!
//! **The clock does not.** `SystemTime::now()` reports nanosecond *units*, never
//! nanosecond *resolution*: macOS truncates it to microseconds (measured on a build
//! host: 1000 tight calls returned **88** distinct values). So two threads of one
//! process routinely read the same instant, mint the same path, and collide — and the
//! collision is silent at `create_dir_all`, surfacing later as whatever the shared
//! directory breaks first (`git init` dying on `File exists`, or one owner's `Drop`
//! deleting the other's tree out from under it).
//!
//! [`unique_nanos`] is the fix as a value rather than a convention: it returns a
//! nanoseconds-since-epoch reading that is **strictly increasing within the process**,
//! so it can never repeat however coarse the underlying clock is. It stays a real
//! timestamp, so a leaked path is still readable in `$TMPDIR` and still sorts by age.
//!
//! Two production siblings — `state::temp_sibling` (the atomic write's temp) and
//! `validate::store_scratch_path` (the store-sweep snapshot) — reached this conclusion
//! independently (M45 Increment 7, Decision 9), each with its own local sequence counter.
//! Both now draw from here, so the property is proven once and the lesson does not have to
//! be re-learned per site. What they keep is the other axis: each prefixes the value with
//! [`std::process::id`], which is what separates concurrent *processes* — [`unique_nanos`]
//! fences only the intra-process collision, the one a coarse clock causes.

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// The last value handed out by [`unique_nanos`] in this process.
static LAST: AtomicU64 = AtomicU64::new(0);

/// Nanoseconds since the epoch, **strictly increasing within this process**.
///
/// The value is the wall clock reading, except when the clock has not advanced past
/// the previous call — then it is the previous value plus one. So it is monotone and
/// injective per process regardless of the OS clock's resolution, while still tracking
/// real time closely enough to read as a timestamp.
///
/// Combine it with [`std::process::id`] for a name that is unique across processes too;
/// on its own it only fences the intra-process collision (the one a coarse clock causes).
pub fn unique_nanos() -> u128 {
    let clock = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| u64::try_from(d.as_nanos()).unwrap_or(u64::MAX))
        .unwrap_or(0);

    let mut prev = LAST.load(Ordering::Relaxed);
    loop {
        let next = clock.max(prev.saturating_add(1));
        match LAST.compare_exchange_weak(prev, next, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return u128::from(next),
            Err(observed) => prev = observed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::unique_nanos;
    use std::collections::BTreeSet;

    /// A sequential burst never repeats — the property the raw clock fails.
    #[test]
    fn a_sequential_burst_never_repeats() {
        let seen: BTreeSet<u128> = (0..10_000).map(|_| unique_nanos()).collect();
        assert_eq!(seen.len(), 10_000, "every mint in a burst must be distinct");
    }

    /// The burst is also strictly increasing, so callers may rely on the ordering.
    #[test]
    fn a_sequential_burst_strictly_increases() {
        let mut last = 0u128;
        for _ in 0..10_000 {
            let next = unique_nanos();
            assert!(next > last, "{next} must exceed the previous {last}");
            last = next;
        }
    }

    /// Concurrent threads never collide — the shape parallel `#[test]`s actually run.
    #[test]
    fn concurrent_threads_never_collide() {
        const THREADS: usize = 8;
        const PER_THREAD: usize = 2_000;

        let handles: Vec<_> = (0..THREADS)
            .map(|_| {
                std::thread::spawn(|| (0..PER_THREAD).map(|_| unique_nanos()).collect::<Vec<_>>())
            })
            .collect();
        let seen: BTreeSet<u128> = handles
            .into_iter()
            .flat_map(|h| h.join().expect("mint thread must not panic"))
            .collect();

        assert_eq!(
            seen.len(),
            THREADS * PER_THREAD,
            "every mint across every thread must be distinct",
        );
    }
}
