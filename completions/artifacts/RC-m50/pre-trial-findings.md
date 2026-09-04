# Pre-trial findings — found while verifying the handover, kept out of the blind yield

An operator's discovery is not a blind session's. These are recorded before any session runs so
that a worker meeting one of them is scored as **reaching a known state**, never as discovering
it — and so that none of them is quietly fixed on the trial binary between now and the sessions.
Same rule as [RC-1.0-final/pre-trial-findings.md](../RC-1.0-final/pre-trial-findings.md).

## PT-1 · `jigc task validate ""` is a false green on the release binary, a panic on debug

Handed over as *"panics at exit 101 on `1.0.0-rc.13`"* and routed to M50 on that description.
Driven on both build postures, 2026-09-04, on `dev/jigc-rig` states `committed-singletons` (no
active task) and `refs-post-hoc` (one live task):

| door | release `~/.local/bin/jigc` (rc.13) | debug `target/debug/jigc` (rc.13) |
|---|---|---|
| `jigc task validate ""` | *"no findings — the task validates clean"*, **exit 0**; `--format json` → `{"schema_version": 2, "findings": []}` | panic at `crates/engine/src/finding.rs:741`, **exit 101** — `Route::mechanical` refuses argv `["jigc","task","discard",""]` |
| `jigc task discard ""` | *"discarded task "*, **exit 0** | same |

The assert is the M43 route fence's parse half, installed debug-only (`finding.rs:737`,
`#[cfg(debug_assertions)]` — *"a release binary never pays or panics"*). So the class the fence
catches in the suite — a `Route::mechanical` built from a caller token the door never validated —
is **invisible on the binary an adopter runs**, and the door underneath reports a task that does
not exist as validating clean. Under [protocol.md](protocol.md) §1 that is the *wrong result on a
non-destructive path* row; it is not a false green **over managed state**, so it ships recorded,
but the M50 entry must describe the release observable, not the debug one.

**The axis, as the handover sized it, is unchanged:** *doors that build a mechanical route from an
unvalidated caller token*. Walk arm 17 drives all seven id-taking doors with `""` on the release
binary and records the observable per door, so M50 receives a measured table rather than this one
cell.

**Not fixed here**, on purpose: the trial's yield is what a blind worker meets on this binary, and
fixing product surface while building the instrument would remove findings the trial could
legitimately produce.

## PT-2 · the installed rc.13 binary predates the gate fix it was said to carry

`~/.local/bin/jigc` and `target/release/jigc` are both dated Sep 1 01:01 — the rc.13 bump
(`21b6236`). Commit `1799a2d` (Sep 4) changed `crates/cli/src/milestone.rs` after that. Both trees
stamp `1.0.0-rc.13`, so `jigc --version` cannot tell them apart. The handover's *"both on the
binary this trial runs"* is true of an image built from HEAD and false of the installed binary.

Not a product finding — it is the fourth consecutive instance of the version-stamp shape the
completion audits keep catching, this time on the *other* side (a tree changed after the stamp
rather than a stamp left unbumped). Consequence: the trial image is built from `979baca` and the
gate record carries the sha; the installed binary is rebuilt from HEAD only **after** the trial,
so the walk's `--binary`-less host probes cannot drift from the image mid-trial.
