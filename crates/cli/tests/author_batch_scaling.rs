//! M49 Increment 5, T6 — the batch-apply path stops re-reading the whole document
//! once per leaf per parse site.
//!
//! **What was wrong.** `jigc doc author` is the one door every `migrate-*` workflow
//! authors a whole document through (`design/corpus-migration.md`: the twelve migrate
//! workflows each hand the CLI one payload), and its cost grew as roughly **n²·⁹** in
//! the number of items — 50 items in 0.08 s, 800 in 77 s, 1500 never finishing. The
//! batch chains its leaves over one in-memory buffer and persists once
//! (`design/write-commands.md` → Batch authoring; `design/auto-migration.md` →
//! Hardening #1), so the *write* was already single-shot; the cost was in the
//! **read**. Each leaf re-parses the growing buffer three or four times (the presence
//! probe, the splice's own locate, `validate_after`), which is quadratic by
//! construction — but `engine::parse` was itself quadratic *inside one parse*, so the
//! product came out cubic. A `sample(1)` profile of the 800-item run at `e0b53bd`
//! attributed **3495 of 3554 samples — 98 % — to `engine::parse::line_of`**, which
//! answered "which line is this byte on?" by counting the newlines in the whole
//! prefix, once per heading in `scan_blocks` and again per item in `parse_items`.
//!
//! **What changed.** Two indexes, both inside `engine::parse`, both built once per parse
//! and neither visible at any surface: newline offsets, so a byte's line number is a
//! binary search rather than a prefix rescan; and a start-ordered permutation of the
//! block list, so *"which blocks lie in this byte range?"* is a binary search too — that
//! second one was the same O(n²)-per-parse shape a layer down, sitting behind the first
//! (the item heads, the field-group sentinel, the slot ceiling and the nested-region
//! boundary each filtered the **whole** block list, once per item). Neither touches what
//! the parser decides, which is what the golden arm below is for.
//!
//! **What this suite pins.** Not a wall-clock ceiling — that is a fact about the
//! machine, and it would redden on a loaded CI box and green on a fast one for
//! reasons that have nothing to do with the code. It pins the **growth ratio**
//! between two payload sizes one octave apart, which is machine-speed-invariant: a
//! constant-factor speedup moves both measurements together and leaves the ratio
//! alone. Only a change of *shape* moves it.
//!
//! **Measured, one payload, release build, `spec` with three leaves per item
//! (`add-item` + a `statement` slot + a `maps-to-test` field):**
//!
//! | items | before (`e0b53bd`) | after   | speedup |
//! |-------|--------------------|---------|---------|
//! |    50 |             0.08 s |  0.07 s |    1.2× |
//! |   100 |             0.27 s |  0.14 s |    2.0× |
//! |   200 |             1.48 s |  0.40 s |    3.7× |
//! |   400 |            10.24 s |  1.54 s |    6.6× |
//! |   800 |            77.26 s |  6.69 s |   11.6× |
//! |  1500 |    (never finished) | 25.94 s |       — |
//!
//! Ratio per doubling **before**: 3.3 → 5.5 → 6.9 → 7.5, i.e. approaching **n²·⁹**.
//! **After**: 2.0 → 3.0 → 3.8 → 4.3, i.e. **n²·¹** — the quadratic residual and no more.
//! Every cell but one was measured for this task — "before" at `e0b53bd`, this
//! increment's HEAD before it, and "after" at the commit that lands it. The exception is
//! the 1500 row's "before": that is the wave's own recorded observation at the earlier
//! `352993a`, where the run was killed rather than timed, so it carries no speedup.
//!
//! The debug build the assertion below actually runs on tells the same story, ~14× slower
//! throughout: **before** 200 → 52.3 s, 400 → 410.4 s (ratio **7.85**); **after** 200 →
//! 3.6 s, 400 → 14.1 s (ratio **3.96**).
//!
//! **The residual bound is quadratic, and is stated in `MIGRATING.md`.** Each leaf still
//! re-reads the whole buffer, so n leaves over an O(n) document remain O(n²); what this
//! task removes is the extra factor of n *inside* each read. A batch that authors
//! thousands of items in one payload is still super-linear, and an adopter migrating a
//! very large corpus document should know that before they hit it.
//!
//! The 800-item golden lives under `tests/fixtures/` rather than `tests/goldens/`
//! deliberately: the compose-golden root is regenerated wholesale from an emptied
//! directory at the close of a wave, and this capture is not part of that set.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

use crate::support;

/// The smaller of the two measured payloads.
const SMALL_ITEMS: usize = 200;
/// The larger — one octave up, so the expected quadratic residual is a ratio of 4.
const LARGE_ITEMS: usize = 400;
/// The payload the byte-fidelity golden was captured from.
const GOLDEN_ITEMS: usize = 800;

/// The most the run time may grow across one doubling of the payload.
///
/// The residual shape is quadratic, so the honest expectation is **4**; the bound is
/// set at 5.5 to absorb per-run noise and the fixed process-startup cost that inflates
/// the denominator, while staying decisively under the **7.85** the cubic path measured
/// at `e0b53bd`. It is a shape assertion with slack, not a stopwatch.
const MAX_GROWTH_RATIO: f64 = 5.5;

/// How many times each size is measured; the **minimum** is taken, which rejects
/// scheduler noise from the eleven other test binaries running beside this one.
const ROUNDS: usize = 2;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-batch-scaling-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
        fs::create_dir_all(&path).expect("create temp dir");
        TempDir(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn git(repo: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

fn run_jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("spawn the jigc binary")
}

fn ok(out: std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The declarative whole-document payload for `items` criteria — the shape every
/// `migrate-*` workflow hands `doc author`: one document, every leaf in one payload.
/// Three leaves per item exercise all three lowered leaf kinds (`add-item`, a slot
/// write, a field write).
fn payload(items: usize) -> String {
    let mut out = String::from(
        "title: \"Batch Scaling Bench\"\n\
         sections:\n\
         \x20 - id: goal\n\
         \x20   set:\n\
         \x20     goal: \"<<A generated spec that exists to be measured, nothing else.>>\"\n\
         \x20 - id: criteria\n\
         \x20   items:\n",
    );
    for i in 1..=items {
        out.push_str(&format!(
            "      - title: \"Criterion {i}\"\n\
             \x20       set:\n\
             \x20         statement: \"<<Criterion {i} holds.>>\"\n\
             \x20         maps-to-test: \"tests/bench.rs#c{i}\"\n"
        ));
    }
    out.push_str(
        "  - id: context\n\
         \x20   set:\n\
         \x20     context: \"<<Generated by the batch-scaling suite.>>\"\n",
    );
    out
}

/// Drive one whole-document `doc author` of `items` criteria through the real binary
/// in a fresh throwaway repo, returning how long **the author invocation alone** took
/// and the staged document it produced. The repo setup is outside the clock.
fn author_once(items: usize, tag: &str) -> (Duration, String) {
    let repo = TempDir::new(&format!("repo-{tag}"));
    let home = TempDir::new(&format!("home-{tag}"));
    let (r, h) = (repo.path(), home.path());

    git(r, &["init", "-q"]);
    git(r, &["config", "user.email", "test@example.com"]);
    git(r, &["config", "user.name", "Test"]);
    fs::write(r.join("README.md"), "hello\n").expect("write file");
    git(r, &["add", "."]);
    git(r, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(r.join(".jigc").join("config")).expect("create project layer");

    ok(run_jigc(r, h, &["setup"]), "jigc setup");
    ok(
        run_jigc(
            r,
            h,
            &["start", "--workflow", "plan", "batch scaling bench"],
        ),
        "jigc start --workflow plan",
    );

    // Outside the repo: the payload is an input to the run, not a file under test, and
    // an untracked file in the corpus is a state some jigc probe could legitimately have
    // an opinion about.
    let payload_path = h.join("payload.yaml");
    fs::write(&payload_path, payload(items)).expect("write the payload");

    let started = Instant::now();
    let out = run_jigc(
        r,
        h,
        &[
            "doc",
            "author",
            "spec",
            "--from-file",
            payload_path.to_str().expect("utf-8 path"),
            "--task",
            "batch-scaling-bench",
        ],
    );
    let elapsed = started.elapsed();
    ok(out, "jigc doc author spec --from-file");

    let staged = r
        .join(".jigc")
        .join("tasks")
        .join("batch-scaling-bench")
        .join("docs")
        .join("spec:batch-scaling-bench.md");
    let text = fs::read_to_string(&staged)
        .unwrap_or_else(|e| panic!("the batch-authored spec is staged at {staged:?}: {e}"));
    (elapsed, text)
}

/// The captured 800-item document — the byte contract this task must not move.
fn golden_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/author-batch-scaling/spec-800-items.md")
}

#[test]
fn the_batch_apply_growth_ratio_stays_within_its_stated_bound() {
    // The two sizes are measured **alternately**, so a burst of load from the eleven
    // other test binaries (or this suite's own golden arm, running on a sibling
    // thread) lands on both rather than on one; the minimum of each round is then the
    // reading, which discards the loaded round outright.
    let mut small = Duration::MAX;
    let mut large = Duration::MAX;
    for round in 0..ROUNDS {
        small = small.min(author_once(SMALL_ITEMS, &format!("s{round}")).0);
        large = large.min(author_once(LARGE_ITEMS, &format!("l{round}")).0);
    }

    let ratio = large.as_secs_f64() / small.as_secs_f64();
    assert!(
        ratio <= MAX_GROWTH_RATIO,
        "the batch-apply path grows super-quadratically: {SMALL_ITEMS} items took \
         {small:?}, {LARGE_ITEMS} items took {large:?} — a growth ratio of {ratio:.2} \
         across one doubling, over the stated bound of {MAX_GROWTH_RATIO}. The whole \
         cost is in the read side: each lowered leaf re-parses the growing buffer, so \
         a parse that is itself super-linear in document size makes the batch cubic. \
         See this suite's module doc for the measured table."
    );
}

#[test]
fn the_eight_hundred_item_document_is_byte_identical_to_the_captured_golden() {
    let (_, produced) = author_once(GOLDEN_ITEMS, "g");
    let path = golden_path();

    if support::goldens::update_mode(
        std::env::var("UPDATE_GOLDENS").ok().as_deref(),
        std::env::var("CI").ok().as_deref(),
    ) {
        fs::create_dir_all(path.parent().expect("golden has a parent"))
            .expect("create the golden directory");
        fs::write(&path, &produced).expect("write the golden");
        return;
    }

    let expected = fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "the captured 800-item golden is missing at {path:?}: {e} — regenerate it \
             with `UPDATE_GOLDENS=1 cargo test -p cli author_batch_scaling::` and review \
             the diff before committing"
        )
    });

    // The document is ~89 KB, so a bare `assert_eq!` would dump both copies and say
    // nothing. Report the first divergence and a window around it instead — a red
    // golden here means bytes moved, and the reader needs to see *which*.
    if produced != expected {
        let at = produced
            .bytes()
            .zip(expected.bytes())
            .position(|(a, b)| a != b)
            .unwrap_or_else(|| produced.len().min(expected.len()));
        // Byte-sliced and lossy-decoded rather than `&str`-sliced: `at` is a byte
        // offset and need not sit on a char boundary, and a panic path must not panic.
        let window = |text: &str| {
            let bytes = text.as_bytes();
            let lo = at.saturating_sub(80).min(bytes.len());
            let hi = (at + 80).min(bytes.len());
            String::from_utf8_lossy(&bytes[lo..hi]).into_owned()
        };
        panic!(
            "the 800-item batch-authored document is no longer byte-identical to the \
             capture taken before the speedup — a changed byte here is a failed task, \
             not a tuning decision.\n  lengths: produced {} / golden {}\n  first \
             divergence at byte {at}\n  produced: {:?}\n  golden:   {:?}",
            produced.len(),
            expected.len(),
            window(&produced),
            window(&expected),
        );
    }
}
