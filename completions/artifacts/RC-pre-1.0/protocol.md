# The pre-1.0.0 trial — protocol (prepped 2026-08-12, pre-trial)

**Provenance:** [decisions-pending.md](../../../implementation/decisions-pending.md) → *Acceptance —
the pre-1.0.0 trial* — the single home for this trial's shape, revised 2026-08-09 after an
independent adversarial review of the first widening ([DECISIONS.md](../../../DECISIONS.md) →
2026-08-09 the acceptance entries). This file is the **execution** of that charter, not a restatement
of it: roles, corpora, plants, verbatim prompts, and the operator walk's arm script.

**Binary under trial:** `jigc 1.0.0-rc.10` at `~/.local/bin/jigc`, built after the M47 completion
audit's three LOW fixes ([M47/VERDICT.md](../M47/VERDICT.md)). A second binary, `1.0.0-rc.9`, is
built from `d13d8ac` to a temp prefix for V1 arm 6 only, and is never installed on the PATH.

**Two instruments, deliberately different in kind** — three blind sessions for discoverability, one
operator-scripted walk for the plants and destructive sequences a blind agent cannot be relied on to
reach. The charter's reason, kept in view: the prior protocol probed fan-out *"only if a milestone
arises naturally"*.

## Roles and the clean-room rules

- **Operator (the human):** runs the prep commands, starts the blind sessions, pastes the worker
  prompts verbatim, intervenes never.
- **Observer session (this one, in the jigc repo):** built the corpora and the plants, runs the
  operator walk (V1), and does the post-trial analysis. It **never** edits a blind corpus once its
  session has started, and never feeds a worker anything beyond the verbatim prompt below.
- **Worker sessions (blind):** started fresh with `cwd` set to the corpus, given ONLY a verbatim
  prompt from §Blind probes. They must not be given: this file, the jigc repo path, any
  `completions/artifacts/` content, any prior trial record, or any handover summary.
- **No mid-trial fixes.** Findings are recorded, never repaired while the trial runs.

## The corpora

All prior trial corpora died with the machine, and every `~/ideas` repo is the human's real work —
so all six corpora are **built fresh from one synthetic template** and are disposable. The template
is a small but genuinely structured TypeScript service (9 modules, 3 test files, 23 passing tests
under `node --test`, 7-commit history, **zero managed docs**), chosen so the doc↔code probes have
real symbols to bind: `grammar_for` (`crates/cli/probes/doc-code/src/resolve.rs`) dispatches `.ts` to
the TypeScript grammar, and the test files give `maps-to-test` — 0 writes in its entire history —
somewhere to point.

| Corpus | Instrument | Starting state |
|---|---|---|
| `~/ideas/gaugeline` | G1 — cold start | pristine + the two G1 plants |
| `~/ideas/windowpane` | G2 — design altitude from zero | pristine |
| `~/ideas/tidepool` | G3 — corpus accretes from nothing | pristine; the foreign doc lands mid-stream |
| `~/ideas/rc10-walk` | V1 arms 1–2 | `jigc setup` run by the operator |
| `~/ideas/rc10-fanout` | V1 arms 3–5 | pristine |
| `~/ideas/rc9-legacy` | V1 arm 6 | authored on rc.9 |

**Clean-room naming rule:** the three blind corpora carry product-plausible names with no
`rc10`/`trial`/`probe` token — a worker that reads its own `cwd` must learn nothing. The V1 corpora
are not blind, so their names may say what they are.

**Verified pre-trial, all six:** 7 commits, clean tree, `.jigc` absent, no `.claude/` or `CLAUDE.md`
residue, `README.md` the only tracked `.md`, `npm test` green 23/23.

### The shipped user docs, and an honest note on how they are reached

`jigc setup` installs a bootstrap `CLAUDE.md`, an allowlist, a SessionStart hook and a pre-commit
hook — it does **not** ship `QUICKSTART.md` or `MIGRATING.md` into the corpus. A real adopter reads
those on the project page before installing. To keep that true without handing a worker the jigc repo
path, both files are copied to **`~/ideas/jigc-guide/`** and named in the blind prompts.

Two declared bounds on that stand-in, pre-registered so neither is discovered later as a surprise:
their in-repo relative links (`design/*.md`, `implementation/*.md`) dangle from that location — if a
worker follows one and is stranded, **that is a finding, not a prep defect**; and naming the
directory in the prompt is operator seeding, counted as such in the record's purity statement.

## The environment matrix

Per the process rule (RC-alpha3 findings-verification → process change 7): each corpus property →
the designed probe, and which **declared bounds must be printed before an agent walks into them**.

| Corpus / run property | Probe or watch | Declared bound to have in hand |
|---|---|---|
| TypeScript + `node --test`, no `node_modules` | `symbol-exists` binds against the real TS grammar; `maps-to-test` finally has test files to point at | The is-a-test tier is deferred (decisions-pending → M46 entry 5) |
| Zero managed docs at start | The "from nothing" premise is exact, not approximated — G1/G2/G3 each accrete their own first doc | — |
| Synthetic corpus, authored by the observer | Cannot surprise a worker the way a stranger's codebase does | Declared in the record's honest bounds |
| First trial on rc.10 | M47's surfaces are live: the survivable hook-rejection frame over nine doors, the scoped `task validate` preview, the write-path miss matrix, the record-only door pre-images | — |
| Only `README.md` tracked as `.md` | Watch: whether the ingest funnel treats a plain README sensibly | — |
| `.env`/`node_modules` in `.gitignore`, neither present | Watch only: gitignored trees stay out of the ingest funnel (closed M40) | — |
| `core.hooksPath` points **in-repo** (G1 only) | Watch: `setup` splices its block into the tracked foreign hook and leaves that file **modified and uncommitted** — its install commit does not carry it. Verified in the operator's pre-trial rehearsal; recorded as a watch item, not yet adjudicated | — |
| Bounds an agent may walk into (print if adjacent) | — | `provision` over a **non-registered leftover** destroys it unguarded (M46 entry 11 rider, knowingly unfixed) · item-slot corruption closed over the **write path** only · lost-update semantics under fan-out (M46 entry 9) · `setup --format json` cannot name the hooks dir (M46 entry 12) |

## Blind probes

Each prompt is pasted **verbatim** into a fresh session with `cwd` set to that corpus. Each carries
`jigc config set invocation-log true` by operational necessity — M47's nine per-door error codes have
the invocation log as their only observation surface — exactly as every prior trial seeded it.

### G1 — cold start, carrying both plants

Plants (operator, before the session): a **rejecting `pre-commit` hook under `core.hooksPath`** — not
`.git/hooks`, because `resolve_hooks_dir` (`crates/cli/src/setup.rs:531`) resolves through
`git rev-parse --git-path hooks` and Increment 10 made `SetupSummary.hook_file` print what it
resolved — and **two files staged before the first mint**, so the carryover gate has something to
catch.

**Both plants were rehearsed to firing on a throwaway copy before the corpus was frozen** — a plant
assumed to fire is not a plant. On the copy: the carryover gate raised one blocking
`finalize.carried-staged` per planted path with its own unstage route (exit 3); after unstaging, the
hook rejected the ADR-promoting finalize with its stderr verbatim, the state-truth clause, and the
re-run line, HEAD unmoved (exit 1); after `touch .githooks/docs-approved` the re-run landed the ADR
(exit 0). The rehearsal copy was then destroyed and `~/ideas/gaugeline` verified still naive
(`.jigc` absent, both plants staged, HEAD at the hook-install commit).

> This project has no docs and no tooling around them yet. The `jigc` CLI is installed — set the
> project up with it (start with `jigc setup`, then `jigc config set invocation-log true`), and then
> do this piece of work through it: `IngestQueue` drops the *oldest* sample when it overflows, and
> we've settled that this is right — a recent picture beats a stale one — so that reasoning needs to
> be recorded as a decision record rather than living in someone's head. Add a short comment at the
> overflow branch pointing at it too. Finish with a clean task finalize. The CLI's own guides are at
> `~/ideas/jigc-guide/` if you want them.

### G2 — the design altitude from zero

> This service does one thing today: it rolls up time-series samples in memory. Before we build
> anything else on it I want the thinking captured properly — what's actually out there for this
> problem, what we're betting on, and what the next few chunks of work are. The `jigc` CLI is
> installed and this project isn't set up with it yet; set it up (`jigc setup`, then
> `jigc config set invocation-log true`) and do the whole thing through the tool — research first,
> then the vision, then a roadmap, then open the first milestone. The CLI's own guides are at
> `~/ideas/jigc-guide/` if you want them.

### G3 — the corpus accretes from nothing

Plant (operator, **mid-stream** — after the worker's first finalize lands, not before):

```sh
mkdir -p ~/ideas/tidepool/docs/decisions
cp <jigc-repo>/completions/artifacts/RC-pre-1.0/plants/g3-foreign-adr.md \
   ~/ideas/tidepool/docs/decisions/0002-keep-the-sample-store-in-memory.md
git -C ~/ideas/tidepool add docs/decisions/0002-keep-the-sample-store-in-memory.md
git -C ~/ideas/tidepool commit -m "docs: record the in-memory store decision"
```

**Two signals in one plant, pre-registered here so neither is claimed after the fact:**

1. **Detect-and-route.** The file is a classic Nygard-shaped ADR — right directory, foreign shape,
   `## Status` carrying prose where the schema wants a field. It is committed by hand, never through
   jigc, so it is a never-adopted foreign doc: the managed-vs-foreign discriminator (M42) should
   route it to `jigc ingest`/`jigc migrate`, not to `migrate-corpus`.
2. **The contradiction trap.** Task (2) of the prompt asks the worker to make samples survive a
   restart. This ADR says the store stays in memory *by decision*, and names the escape hatch
   (*"gets superseded rather than quietly worked around"*). The signal is whether the worker **finds**
   the conflicting decision once it is adopted and then either respects it or supersedes it through
   the `supersedes` flow — never silently contradicts it. This is the project-alpha-4.0 P4 ADR trap
   re-run against a doc the worker did not author.

> The `jigc` CLI is installed but this project doesn't use it yet. Set it up (`jigc setup`, then
> `jigc config set invocation-log true`) and then build out the project's documentation through it,
> in this order: (1) a changelog entry for what's already shipped; (2) a spec for the one thing
> missing from the store — samples are never persisted, so a restart loses everything — and then
> implement that spec; (3) an architecture document covering how ingest, store and rollup fit
> together. Finish each piece with a clean finalize. The CLI's own guides are at
> `~/ideas/jigc-guide/` if you want them.

## The operator walk (V1)

Not blind. Six arms, each justified by a behaviour M47 changed that no blind probe reaches. Commands
and verbatim output are captured into [v1-walk.md](v1-walk.md) as each arm runs.

**Before adding a seventh arm: check the suites, not the artifact in hand.** That failure hit three
times in two days, always the same shape — `commit_rejected_axis.rs` already drives `rename`, all
four milestone record-only doors and `migrate-corpus` under a rejecting hook, and its door set is
derived from `ERROR_CODE_REGISTRY`, so a door added without a code is a red test.
`cargo test -p cli verb_suite_coverage -- --ignored --nocapture` prints the verb → suite map — and
its green means *named by a suite*, never *fenced*.

1. **`jigc migrate <path> --as <doctype>` end to end** (`rc10-walk`) — the transform, not G3's
   detect-and-route. Watch the review hold, the fidelity diff, and that `--approve` is the sole
   destructive gate.
2. **`migrate-corpus`'s N2 recovery signature** (`rc10-walk`) — **both plants or the arm is
   vacuous**: a committed `schema-version:` stamp hand-downgraded **and** a rejecting hook. No
   schema-version moved in M47, so an ordinary corpus answers `already current`
   (`migrate_corpus.rs:582`) and a green would prove nothing. Expect the MIGRATING.md gate-5
   contract: *"an earlier run's migration was written but never landed"*, and a repaired re-run that
   **lands** the staged bytes.
3. **Re-`provision` over a non-registered leftover** (`rc10-fanout`) — unguarded in the code:
   `crates/cli/src/milestone.rs:1646-1649` `remove_dir_all`s an existing-but-unregistered dir at the
   worktree path with no warning and no refusal. Plant staged **and** unstaged **and** untracked work
   plus an authored doc there first, so a destroyer shows as **visible loss rather than inference**.
4. **The teardown matrix — cheap re-check only** (`rc10-fanout`). `uninstall` over a live abort is
   fenced by `uninstall_worktree_guard.rs` (all three rejection causes) and `discard` ±`--force` by
   `milestone_discard.rs`; the M47 e2e drove all three rejection causes through this binary. This arm
   confirms, it does not sweep.
5. **Fresh clone → `milestone finalize` FIRST** (`rc10-fanout`) — expect the zero-contribution
   refusal → follow its route → `provision` → execute the emitted `Spawn:` line → land. One arm
   covers Increment 3(b)(i), the reseed, and route-followability. Continuing the work directly would
   skip the refusal that motivates the reseed.
6. **An rc.9-authored corpus continued on rc.10** (`rc9-legacy`) — author ~5 managed docs of mixed
   doctypes plus a milestone record on the temp-prefix rc.9, commit; then switch to rc.10 and
   continue: `validate` · `doc show` · a task → finalize · a `rename` · `migrate-corpus`. The most
   common real 1.0.0 upgrade path, the state Increment 1's one-way doors are about, and **covered by
   nothing today** — not even the fixture builder, which constructs every state by driving the
   *current* binary.

## Per-session feedback (after every blind run)

At the end of each worker session — **inside that session, while fresh** — collect verbatim feedback
with the standard prompt:

> About the `jigc` CLI specifically: (1) what confused you; (2) what did jigc tell you that turned
> out to be wrong or misleading; (3) what did you look for — a command, a flag, a way to read or
> write something — and not find; (4) what did you do around jigc rather than through it (any direct
> file read/edit, raw git command, or other workaround touching managed docs), and why?

Save it to this directory as `feedback-G<n>.md`, **never into the corpus**, and never feed it into a
later worker's prompt — feedback is retrospective record for the post-trial triage only. The
discoverability-lens findings of the last four trials came from these reports, not the log.

## After the trial

Trial record + per-corpus invocation-log analysis land here; every claim adversarially verified with
live repros — **CONFIRMED and REFUTED alike ship repro blocks**
([milestone-completion-workflow.md](../../../implementation/milestone-completion-workflow.md) →
Audit); triage through the known-hole and discoverability lenses; the coverage rule applied against
the M47 diff (`9e6cb17..HEAD`) in **three** columns — trial-reached · test-fenced (naming the suite) ·
neither — because the two-way split is what left ~27% of changed lines unexamined last time.

Then the **conversion ledger**: every repro block carries `pinned-by:` or `UNPINNED: <why>`
([pinning.md](../../../implementation/pinning.md) §3). That ledger is the human's own gate — **the
1.0.0 call is not taken until it is closed**, because the trial is a report, not a second close, and
no wave follows it to inherit the obligation.

Also owed before the call, and deliberately **not** part of this trial: the **latent-surface sweep**,
a chartered increment whose deliverable is standing tests.
