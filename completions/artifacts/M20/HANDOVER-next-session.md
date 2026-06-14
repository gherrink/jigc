# M20 handover — next session: plan + build M21 (production-readiness for real-project live testing)

**Written 2026-06-14.** Branch `main`, tree clean, HEAD `b9c68aa` (+ this handover's commit on top).
Pinned binary `~/.local/bin/jigc` sha `3f492263…` (M20 release — 6.4MB, embedded-probe + 3-target
validate), `doc-code` sibling beside it; `~/.cargo/bin/jigc` is the same build. **M20 is fully closed.**
**Multi-assistant adapter profiles stay deferred** ([decisions-pending.md](../../implementation/decisions-pending.md))
— a handover that framed it as "next" was written and then **undone**; this replaces it.

## The pivot (read first)

The aim now is **getting jigc production-ready, then proving it on real projects** via **two human-driven
manual live tests** (a *separate* session, nothing automated):

- **Flow A — new project from scratch:** define the **vision → milestones → MVP**, with new docs created
  on the fly. **The human starts here.**
- **Flow B — existing project:** let the LLM **track every doc into jigc**, then **clean up**.

This session did the **readiness check** (two `capability-auditor`s, exercised the real binary in /tmp
repos) and made two product calls. **M21 is the focused production-readiness milestone that closes the
Flow-A blockers** so the new-project test runs on a genuinely clean `jigc setup`. **Planning M21 was
deliberately deferred to you (fresh context).** Decision of record: [DECISIONS.md](../../DECISIONS.md)
→ 2026-06-14 "Direction: pivot to production-readiness".

## Verified readiness findings (don't re-audit — this is the baseline)

### Flow A — new project (vision → milestones → MVP)
- **Vision ✓** — `jigc start --workflow project-setup` mints a task; `jigc doc create prd` → the `prd`'s
  `vision` slot **is** the vision artifact (no separate `vision` doctype); finalize promotes to `prds/`.
- **MVP ✓** — `plan`→`spec` (`specs/`), `single-task`→`adr` (`decisions/`) + real code, finalize lands
  code+doc in **one commit**. Creatable on the fly: **`prd, spec, adr, arch-doc, commit`**. The code-anchor
  fields (`spec.maps-to-test`, `adr.cites-code`) are **not** finalize-required → cold-start friendly.
- **Milestones ✗ — THE HARD WALL.** The `roadmap` doctype + `planning` workflow live **only in
  `packs/methodology/`**, which is **not embedded** (`crates/cli/src/pack.rs:50` `include_dir!`s only
  `crates/cli/pack`) and `jigc setup` **never writes a `packs.yaml` / wires a second pack**. On a clean
  machine a real project gets **no milestone/roadmap authoring**: `jigc doc create roadmap` →
  `create.unknown-doctype`; `jigc start --workflow planning` → `no workflow planning`. (Listed packs are
  read as `FilesystemPack` from absolute paths — `pack.rs:220` — so even staging methodology needs the
  dir present on disk + a hand-written `.jigc/config/packs.yaml`.)
- **Cold-start bugs** (both real, both worked-around-able, both should be fixed for a clean test):
  - `project-setup` **crashes on a zero-commit repo** — runs `git rev-parse HEAD`, dies
    `fatal: ambiguous argument 'HEAD'`. The literal first command of a new project. Workaround: make an
    initial commit first.
  - Composed workflows instruct authoring via **`jigc doc set-slot … --from-file -`** but `set-slot` has
    **no stdin path** (`-` isn't read; `--from-file` requires a real file). Bites *every* doc-author
    command as-emitted. Workaround: write a temp file.
  - Lesser: `planning` is `selectable:false` (won't show in `jigc start` even once wired); address grammar
    is unobvious (`#fieldid`; repeatable items `#section/item/leaf`, slash-separated); commit-doc authoring
    (`type/scope/body`) is mandatory + unscaffolded (discovered via the finalize error).
- **Dev-pack workflow inventory** (from `jigc start`/`describe`): `project-setup`(→prd), `plan`(→spec),
  `single-task`(→adr), `implement-from-spec`(→adr), `architecture-documentation`(→arch-doc),
  `quick-fix`(none), `router` (the `default-workflow`), `ingest-existing`(run-ingest),
  `milestone-execution`/`sub-task`.
- **`jigc setup` creates:** `.claude/settings.json` (allowlist `jigc *`), `.jigc/AGENT.md`,
  `.jigc/.gitignore`, `.jigc/config/.gitkeep`, `CLAUDE.md` (bare `@.jigc/AGENT.md`). Managed-doc dirs
  (`decisions/`, `specs/`, `prds/`, `architecture/`) are created **on first promotion at finalize**, not
  at setup. Idempotent, non-destructive.

### Flow B — existing project (track every doc, cleanup)
- **Triage ✓** — `jigc setup` then `jigc ingest` gives a deterministic, sorted **detect-and-route**
  classification (`unmanaged` / `needs-reconcile` / `adoptable`), with `--format json`. Honest.
- **`adopt` ✓ (register-only)** — a doc **already byte-conformant at its schema location** is adopted
  (writes `.jigc/index/edges.json` + `.jigc/state/file-state.json`, **doesn't move/rewrite**; idempotent).
- **The crux: detect-and-route ≠ adopt.** A *foreign-shaped* doc (the realistic case) is only **flagged**.
  To track it you must **manually re-author its prose** through `doc create + set-slot` in a create-gated
  workflow — and only if a matching doctype exists. **No `jigc adopt <file> --as <type>` / import**
  (auto-migration is the deferred research-grade core).
- **G2 doctype coverage:** only `adr/prd/spec/arch-doc` map to anything — a README / changelog / runbook /
  contributing guide maps to **nothing** and stays `unmanaged` forever.
- **G3 scan breadth:** root (top-level only) + `docs/` (top-level only, **non-recursive**) + location dirs.
  Docs in `wiki/`, nested `docs/sub/`, `rfcs/`, etc. are **invisible** to ingest.
- **G4 trust hazard (real, exercised):** `finalize` silently `baseline-adopt`s foreign docs sitting in
  location dirs with **no schema check** (`crates/engine/src/file_state.rs` UNKNOWN→baseline-adopt,
  ~L118-140; advisory, so finalize proceeds). So "what jigc manages" can quietly exceed what ingest vetted
  — contradicts the safety property project-setup.md asserts.
- **G5 no cleanup/teardown:** no `jigc remove`/uninstall/doc-delete; foreign originals are left on disk;
  cleanup is manual `git rm` / `rm -rf .jigc`.
- **G6:** `ingest-existing` (`creates-task:false`) isn't discoverable from `jigc start`.

## M21 — the scope converged this session (settle the forks at planning)

**Two product calls already made** (DECISIONS 2026-06-14):
1. **Milestones reach a real project by EMBEDDING the methodology pack + AUTO-WIRING it at `jigc setup`**
   (compose dev + methodology by default, on the built **M14 multi-pack** rails) — *not* by forking a
   lighter roadmap into the dev pack.
2. **Scope a focused production-readiness milestone (M21) first**, before the live tests.

**Core M21 deliverable (Flow-A readiness):**
- **Embed the methodology pack in the binary + auto-wire it at setup** so a clean `jigc setup` gives a real
  project vision + **milestones** + MVP out of the box.
- **Fix the two cold-start bugs:** `project-setup` zero-commit crash; the `--from-file -` stdin gap (make
  `-` read stdin, or stop emitting it in composed prose).

**The real design forks to settle at M21 planning (NOT settled yet — don't assume):**
- **KEEP-SUBSUME (the big one).** M14 settled **KEEP-SUBSUME**: the methodology pack *vendors/subsumes*
  the dev surfaces. So does a real project **compose dev + methodology** (two packs) or **use the
  methodology pack alone** (it already contains the dev loops *plus* milestone planning)? This decides what
  "auto-wire at setup" even installs. Spike it — don't reason by analogy.
- **Embedded-pack composition is net-new wiring.** M14 built composition for **`FilesystemPack` from
  absolute paths**, *not* for a second **embedded** pack selected at setup. Embedding a 2nd pack +
  `setup` writing the compose config + the embed-time bloat lesson from M20 (don't `include_dir!` a
  build-tree) are all real work — treat "M14 already does multi-pack" as **`unverified-reuse`** until the
  embedded-pack-at-setup path is spiked.
- **`planning` discoverability** — it's `selectable:false`; for the human to *find* "define milestones",
  either make it selectable/routable or document the `--workflow planning` entry.
- **How much of Flow B to include vs defer.** Recommendation to put to the human: **include G4** (the
  finalize baseline-adopt trust hazard — a real, bounded correctness bug affecting both flows); **defer
  G1/G2** (foreign-doc adoption / doctype coverage = the deferred auto-migration research core — too big,
  its own milestone); **G3/G5 need-driven** (scan breadth, cleanup — fix if the Flow-B test actually
  hurts). Flow B is the *second* test, so it need not fully land in M21.

## How to start (fresh session)
1. Re-read this file + [DECISIONS.md](../../DECISIONS.md) → 2026-06-14 "Direction" entry. That's the whole
   context; you don't need this session's transcript.
2. Run **`/milestone-plan`** for M21 with the description below. The Scope phase should confirm the
   done-picture (Flow-A readiness; Flow-B scope-decision) with the human, re-verify the baseline (the
   findings above are a *map at this sha*, not gospel — spike the embedded-pack composition path
   specifically), then Detect-gaps → Settle (the forks above) → Review → Decompose.
3. The grooved loop (plan → `milestone-build` → completion audit → auto-fix findings) ran clean for M18/
   M19/M20 — reuse it. Re-pin the binary after the build half (M21 changes the embed + `cli`): M20's own
   `cargo install --path crates/cli` + `jigc setup`-extract is now the supported re-pin flow.

**M21 one-liner for `/milestone-plan`:** *"M21 — production-readiness for real-project live testing:
embed the methodology pack + auto-wire it at `jigc setup` (real project composes dev+methodology by
default, vision→milestones→MVP out of the box; settle KEEP-SUBSUME = compose-both vs subsume, and the
embedded-pack composition wiring); fix the Flow-A cold-start blockers (`project-setup` zero-commit crash;
`set-slot --from-file -` stdin gap); settle how much of the Flow-B ingest gap to include (rec: include the
finalize baseline-adopt trust hazard; defer auto-migration/doctype-coverage; need-drive scan-breadth/
cleanup). Goal: the human's manual new-project (Flow A) live test on a clean setup."*

## State at handoff
- `main` clean, HEAD `b9c68aa` (+ the DECISIONS direction entry + this handover). Gate green at M20 close.
- Binary re-pinned to the M20 build (`3f492263…`) at both `~/.local/bin` and `~/.cargo/bin` (+ `doc-code`).
- **No milestone scoped beyond M20.** M21 is created when you run `/milestone-plan` next session.
- Open deferrals untouched ([decisions-pending.md](../../implementation/decisions-pending.md)): multi-assistant
  profiles; the M19 hook warn-surface broadening; override↔default-in-`validate`; the auto-migration core
  (now clearly implicated by Flow B); plus the standing condition-keyed backlog.
