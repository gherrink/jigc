# verify-real — `r1-staged-doc-ids-other-callers-not-driven` (run canary-one, round 1, stage test, attempt 1)

Reporter `verify-p2-r1-staged-doc-ids-other-callers-not-driven`. One finding, handed over: ledger key
`r1-staged-doc-ids-other-callers-not-driven`, door `jigc task discard` and `jigc uninstall` (the other
callers of `staged_doc_ids`), the clause it is said to break `no-lost-files`, triage's grade *unclear*.
It has no block of its own; its plants are those of `Repro V-1` in
`completions/artifacts/canary-one/r1/reports/test/verify-p1-r1-doc-list-staged-arm-unreadable-entry.a1.md`,
which the prompt handed over as this finding's repro and which was read for the plants and for its
`Scope of what was verified` and `Left open`, item 4. No other report and nothing of triage's
reasoning was read.

## Verdict

**`refuted` as a blocker — basis `breaks-no-clause`. The fail-open the finding asks about does not
occur; a narration defect at the same two doors is real, and stays a row of the ledger.**

- **Neither door destroys anything without the consent.** With each plant alone in the live task's
  staging area, `jigc task discard <id>` and `jigc uninstall` each exit 1, print a blocking finding
  that carries a route, and leave `.jigc/` entry for entry as it was — kind, mode, size, link target
  and content hash of every entry, `git status --porcelain --untracked-files=all --ignored` and
  `HEAD` all unchanged. Four cells on the candidate, the same four on the previous release, eight
  of eight fail closed.
- **The forced forms destroy at exit 0, and that is the consent the design names.**
  `jigc task discard <id> --force` removes the task area and `jigc uninstall --force` removes
  `.jigc/`, plant included, in every cell. `design/write-commands.md` → *Abandoning a task*: *`--force`
  is the single consent for both*; `design/project-setup.md` → *Teardown / cleanup*: *the one way
  past it is `--force`, the operator's explicit consent to delete, so the destruction is asked for
  rather than assumed.* The first clause's scope (DECISIONS.md, 2026-10-04, *The exit rule, revised*,
  sharpening 1) is destruction *the user did not ask for*; here it was asked for, in the word the
  refusal printed.
- **The real defect: under the dangling link, the forced doors do not name the staged docs they
  take.** `jigc task discard <id> --force` acks `discarded task ground-the-vision-in-research` and
  nothing more — its `--format json` ack carries `"dropped": []` — while `commit:ground-the-vision-in-research`
  and `vision:vision` go with the area. `jigc uninstall --force` prints no staged-docs warning for
  them, and the candidate's last note then says *Anything you had staged in one is named above*,
  which is not true of that cell. With an ordinary foreign file in place of the link (a control of
  this verifier's), both doors name both docs. So the silence is the probe's error arm, taken as an
  empty list by the two narration callers.
- **It breaks no clause inside its scope.** The bytes are destroyed by consent; the state is a link
  made by hand inside a gitignored workbench, which the scope's last sentence puts among the
  *deliberately planted states* (declared bounds); and the narration is, in the code's own words,
  best-effort (see *Against the design*). What is wrong is what a surface says, not what a command
  destroys. The previous release prints the same ack and the same silence, byte for byte at the
  task door.

`contested: false` — the finding does not argue that a settled decision is wrong.

## The binaries, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` printed
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the hash the
  `BINARY:` line gives (candidate, label c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the previous
  release's (1.0.0-rc.24).
- The binary's directory went first on `PATH` in every driver call, and each call stops unless
  `command -v jigc` prints the binary it was handed. For the candidate that printed
  `<scratch>/bin/c1.a1/jigc`.
- No `cargo build`, nothing under `target/`. Every rig: `SCRATCH=<W>/rigs dev/jigc-rig --binary <binary> refs-post-hoc`,
  stdout captured alone, the construction log to its own file, exit status 0 each time.
- The clone: `git status --porcelain` read `?? completions/artifacts/canary-one/r1/` before and after,
  `HEAD` eeffe347 on `fix/canary-one`. Nothing was edited, staged or committed.

## Platform, and how each cell was measured

**Driven on macOS 26.6.2 (Darwin 25.6.0), git 2.54.0 (Apple Git-157), as a non-root user (uid 501).**
Nothing was driven on Linux, and nothing as root.

One directory of this verifier's own, minted with `mktemp -d` under the scratch root:
`<scratch>/verify-other-callers.LGdYkq` (written `<W>`). **One fresh rig per cell** — 32 cells, 32
rigs, each from the rig tool's own `mktemp -d` under `<W>/rigs/` — because every door here destroys
its subject when it succeeds; a 33rd rig was built first to read the tool's output and was not
driven. Nothing was torn down.

Environment of every invocation: `HOME` the rig's own, `GIT_CONFIG_GLOBAL=/dev/null`,
`GIT_CONFIG_NOSYSTEM=1`, the working directory the rig's repository. Driver `<W>/tools/cell.sh`:
build the rig, make the one plant, snapshot, run the argv with stdin from `/dev/null` and stdout
and stderr to their own files, read the exit status directly (never through a pipe), snapshot
again. The snapshot (`<W>/tools/snap.sh`) is one line per entry under `.jigc/`: kind, mode, size and
the first 16 hex digits of its sha256 — or the link's target, or `UNREADABLE` for the mode-000 file —
plus the porcelain line set and `HEAD`. Follow-on commands in a cell's rig went through
`<W>/tools/again.sh`, the same measurement without a new rig.

The plants, each alone, in `.jigc/tasks/ground-the-vision-in-research/docs/`:

- **dangling** — `ln -s /nonexistent/x.md research:dangling.md`
- **locked** — a 30-byte regular file `research:locked.md`, then `chmod 000`
- **foreign** (this verifier's control, not the finding's) — a regular file `notes.txt`, mode 644

## What was driven — the candidate

Task `ground-the-vision-in-research` (written `T`); the staging area holds
`commit:ground-the-vision-in-research.md`, `vision:vision.md` and `provenance.json` before any plant
(24 entries under `.jigc/`, 25 with a plant). Per-cell files under `<W>/runs/cand-*/`.

### The doors without the consent

| cell | plant | invocation | exit | stdout | stderr | `.jigc/`, porcelain, `HEAD` |
|---|---|---|---|---|---|---|
| cand-none-discard | none | `jigc task discard T` | 1 | empty | `blocking · task-discard.staged-prose` — *stages 2 doc(s)*: `commit:ground-the-vision-in-research, vision:vision`; a route (587 bytes) | identical |
| cand-dangling-discard | dangling | `jigc task discard T` | **1** | empty | `blocking · task-discard.foreign-bytes` — *holds 1 path(s) jigc did not write*: `.jigc/tasks/T/docs/research:dangling.md`; a route (695 bytes) | identical, the link standing |
| cand-locked-discard | locked | `jigc task discard T` | **1** | empty | `blocking · task-discard.staged-prose` — *stages 3 doc(s)*: `commit:ground-the-vision-in-research, research:locked, vision:vision`; a route (604 bytes) | identical, the file standing at mode 000 |
| cand-none-uninstall | none | `jigc uninstall` | 1 | empty | `blocking · uninstall.staged-prose` — 2 staged docs for 1 open task; a route (679 bytes) | identical |
| cand-dangling-uninstall | dangling | `jigc uninstall` | **1** | empty | `blocking · uninstall.foreign-bytes` — the link's path; a route (615 bytes) | identical |
| cand-locked-uninstall | locked | `jigc uninstall` | **1** | empty | `blocking · uninstall.staged-prose` — 3 staged docs, `research:locked` among them; a route (696 bytes) | identical |

The dangling link never reaches the staged-docs probe at either door: the foreign-bytes guard is
asked first and classifies it foreign, which is what the suite header of
`crates/cli/tests/destroying_door_sibling_surfaces.rs` says of it (*the foreign refusal answers first,
every time*). The mode-000 file is not an error to the probe at all: the probe asks for the entry's
shape and not its bytes, so the file is listed as the staged identity `research:locked`.

### The doors with the consent

| cell | plant | invocation | exit | the ack (stdout) | stderr | `.jigc/` after |
|---|---|---|---|---|---|---|
| cand-none-discard-force | none | `jigc task discard T --force` | 0 | `discarded task T — dropped staged edits to: commit:T (transient), vision:vision` (138 bytes) | empty | the task area's 10 entries gone, nothing else |
| cand-dangling-discard-force | dangling | the same | 0 | **`discarded task T`** (45 bytes) — no list | a warning naming `.jigc/tasks/T/docs/research:dangling.md` as work not in git (270 bytes) | the area's 11 entries gone: the link, **and both staged docs, named nowhere** |
| cand-dangling-discard-force-json | dangling | `… --force --format json` | 0 | `"dropped": []`, `"findings": []`, `"op": "task-discard"` (123 bytes; the control's is 190 with both ids in `dropped`) | the same warning | the same |
| cand-locked-discard-force | locked | `jigc task discard T --force` | 0 | `… dropped staged edits to: commit:T (transient), research:locked, vision:vision` (155 bytes) | empty | the area's 11 entries gone |
| cand-foreign-discard-force | foreign | the same | 0 | the control's list, both ids | a warning naming `.jigc/tasks/T/docs/notes.txt` | the area gone |
| cand-none-uninstall-force | none | `jigc uninstall --force` | 0 | the removal ledger, seven lines (417 bytes) | *discards the staged docs of 1 open task(s)*: `T: commit:T, vision:vision`; then the tracked files, jigc's own state, the work unit (1498 bytes) | absent |
| cand-dangling-uninstall-force | dangling | the same | 0 | the same ledger | the working-area warning naming the link; **no staged-docs warning**; the work-unit note still reads *Anything you had staged in one is named above* (1495 bytes) | absent |
| cand-locked-uninstall-force | locked | the same | 0 | the same ledger | the staged-docs warning with three ids, `research:locked` among them (1515 bytes) | absent |
| cand-foreign-uninstall-force | foreign | the same | 0 | the same ledger | the working-area warning naming `notes.txt` **and** the staged-docs warning with both ids | absent |

In every forced cell `HEAD` is unchanged; no commit is made.

### The printed routes, run as printed

Each in the rig of the refusing cell, whose state the refusal had left untouched.

| refusal | the route's command | exit | what it printed |
|---|---|---|---|
| locked, either door | `jigc doc show research:locked --task T` — the read route, with the address the refusal names | **1** | `blocking · store.not-staged — research:locked is not staged in this task and has no committed copy`, with a route to author it first. The refusal says the task stages it; the read it routes at says it does not. |
| locked, either door | `jigc doc show vision:vision --task T` | 0 | the staged doc, 284 bytes |
| locked, task door | `jigc task finalize T` | **1** | `enumerating the task's code-anchor surface at "<abs repo>/.jigc/tasks/T": Permission denied (os error 13)` — a raw OS error, the machine's absolute path, no route. Control with no plant: exit 3, two `schema-conformance` findings, each with its route. |
| locked, uninstall | `jigc task discard T --force`, then `jigc uninstall` | 0, 0 | the three-id ack; then the ledger |
| dangling, task door | the link deleted by hand, then `jigc task discard T` | 1 | `task-discard.staged-prose` over the two docs — byte-identical to cand-none-discard (`cmp`) |
| dangling, uninstall | the link deleted by hand, then `jigc uninstall` | 1 | `uninstall.staged-prose` — byte-identical to cand-none-uninstall (`cmp`) |
| every refusal | its `--force` form | 0 | the cells of the table above |

So the routes of the dangling refusals run as printed and lead to the door's second guard, which is
the stated order (`design/write-commands.md`: the foreign-bytes guard *is asked first*). Two routes of
the locked refusals do not run: the read of the named address, and `finalize`. Neither destroys
anything; both are in *Left open*.

## Against the claim, and against the design that owns the behaviour

**The claim**, as triage put it: does each door fail closed, or destroy the task area or the plant
at exit 0. Each fails closed without `--force`. With it each destroys, as the route that printed
`--force` said it would.

**The design.**

- `design/write-commands.md` → *Abandoning a task*: the task door refuses on two subjects, foreign
  bytes first, and *`--force` is the single consent for both*. Driven as written.
- `design/project-setup.md` → *Teardown / cleanup*, states (b) and (d): staged docs block with
  `uninstall.staged-prose`, a file jigc did not write inside a working area blocks with
  `uninstall.foreign-bytes`, and `--force` is the one way past. Driven as written. The same passage
  holds the rule the forced narration falls short of: *whatever this door takes, it names*.
- The code's own contract for the two narration callers: `crates/cli/src/task.rs`, the comment over
  `dropped_staged_docs` (line 1571 on) — *Best-effort by design: an unreadable `docs/` dir yields the
  empty list … enumeration must never block the discard itself*; and `crates/cli/src/setup.rs`, the
  comment over `pending_staged_prose` (line 6393 on) — *an in-scope area it cannot read yields no
  warning rather than failing a teardown the guards already cleared … the declared bound stays
  visible, not prevented*. Both take the probe's error as an empty list. In the dangling cell that
  bound is not visible: the list is silently empty, and at `uninstall` a sentence beside it says
  the opposite.

This verifier found no dated decision that rules what a forced door says when its own probe errors
on one entry, so the basis is `breaks-no-clause` and not `intended`.

**The clause.** DECISIONS.md, 2026-10-04, *The exit rule, revised*, sharpening 1: *In a healthy
repository used as documented … no jigc command at exit 0 destroys bytes no git object holds or
commits content the user did not ask for. Where jigc cannot tell … it refuses before writing. …
deliberately planted states are declared bounds.* Read against the cells:

- without the consent: exit 1, nothing destroyed, nothing committed — the clause's own wording;
- with the consent: destruction the user asked for, in the word the refusal printed;
- the missing names: a surface that under-reports a consented removal, in a state reached here only
  by a hand-made link. Not a destruction at exit 0 that nobody asked for, and nothing committed.

The verdict does not rest on whether ordinary use can reach the plants. That was not driven here:
both plants were made by hand, and no attempt was made to reach either through jigc's own verbs.

## The same block on the previous release

The verdict is not `confirmed`, so no `regression` field is returned. The same sixteen cells and the
same route drives were run on the previous release for comparison; per-cell files under
`<W>/runs/prev-*/`.

- **Every exit status is the same** in all sixteen cells and in every route drive.
- **The task door is byte-identical**, stdout and stderr, in all eight of its cells (the rig's path
  normalised), the JSON ack with `"dropped": []` among them (`cmp`).
- **`jigc uninstall` without the consent is byte-identical** in all three cells.
- **`jigc uninstall --force` differs in the same way in every cell, the no-plant control included**:
  the candidate lists a sixth tracked file (`.jigc/settings-entries.json`, which the previous release
  does not write) and adds two warnings, for jigc's own rebuildable state and for the work unit. The
  second carries the sentence *Anything you had staged in one is named above*. The staged-docs
  warning is absent under the dangling link on both binaries; that sentence, false in that one cell,
  is on the candidate only.
- The two failing routes of the locked refusals fail identically (`cmp`, the rig's path normalised).

## Scope of what was verified

**Instance.** Driven: two doors (`jigc task discard`, `jigc uninstall`), each bare and forced, two
plants and two controls, on two binaries, on one platform, plus the routes above.

**The callers were counted, not all driven.** `grep -rn staged_doc_ids crates/` gives five call sites
in `crates/cli/src`: `task.rs:1301` (the fail-closed guard probe `staged_task_prose`), `task.rs:1582`
(the task door's ack), `orient.rs:212`, `doc.rs:5023` and `doc.rs:5142`. This report drove the first
two. The two in `doc.rs` are the earlier report's door. **`orient.rs:212` was not driven**, and of
the guard probe's own consumers (`grep -rn "staged_task_prose\|pending_staged_prose(" crates/cli/src`)
the milestone door's two — `milestone.rs:6099` and `milestone.rs:5897` — were not driven either. The
class is not bounded here.

## Repro V-2

```yaml
claim: "with a dangling link or a mode-000 file named as a staged doc alone in a live task's staging area, `jigc task discard <id>` and `jigc uninstall` destroy the task area or the plant at exit 0"
verdict: REFUTED   # basis breaks-no-clause: both doors refuse at exit 1 and change nothing; the forced forms destroy by consent, and under the dangling link they do not name the staged docs they take
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "the same exit status in every cell; the task door byte-identical; the forced uninstall silent about the staged docs under the link there too (sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d)"
platform: "macOS 26.6.2, git 2.54.0, a non-root caller; not driven on Linux"
setup:
  - fixture: refs-post-hoc                  # live task ground-the-vision-in-research staging commit:<id> and vision:vision
  - ["ln", "-s", "/nonexistent/x.md", ".jigc/tasks/ground-the-vision-in-research/docs/research:dangling.md"]
repro:                                      # each from a fresh fixture: a door that succeeds destroys its subject
  - ["jigc", "task", "discard", "ground-the-vision-in-research"]
  - ["jigc", "uninstall"]
  - ["jigc", "task", "discard", "ground-the-vision-in-research", "--force", "--format", "json"]
  - ["jigc", "uninstall", "--force"]
expect:
  - exit: 1
    stdout: ""
    stderr_contains: ["blocking · task-discard.foreign-bytes", ".jigc/tasks/ground-the-vision-in-research/docs/research:dangling.md", "jigc task discard ground-the-vision-in-research --force"]
    tree: "every entry under .jigc/ identical before and after (kind, mode, size, link target, content); porcelain and HEAD unchanged"
  - exit: 1
    stdout: ""
    stderr_contains: ["blocking · uninstall.foreign-bytes", ".jigc/tasks/ground-the-vision-in-research/docs/research:dangling.md", "jigc uninstall --force"]
    tree: "the same"
  - exit: 0
    stdout_json: { "op": "task-discard", "task": "ground-the-vision-in-research", "dropped": [], "findings": [], "commit": null }   # TODAY's answer: two staged docs go with the area and are not in `dropped`
    stderr_contains: [".jigc/tasks/ground-the-vision-in-research/docs/research:dangling.md"]
    tree: ".jigc/tasks/ground-the-vision-in-research/ gone; nothing else under .jigc/ changed; HEAD unchanged"
  - exit: 0
    stderr_contains: [".jigc/tasks/ground-the-vision-in-research/docs/research:dangling.md"]
    stderr_lacks: ["discards the staged docs of"]   # TODAY's answer: the staged-docs warning is absent
    tree: ".jigc/ gone; HEAD unchanged"
variants:        # each alone, in place of the `ln`
  - "a mode-000 research:locked.md  -> bare doors: exit 1, `task-discard.staged-prose` / `uninstall.staged-prose` naming three ids, `research:locked` among them, tree unchanged; forced doors: exit 0, the ack and the staged-docs warning name all three"
  - "a regular notes.txt            -> forced doors: exit 0, the working-area warning names notes.txt AND the ack / the staged-docs warning name commit:<id> and vision:vision"
control: "no plant -> bare doors: exit 1, the staged-prose refusals over two ids; forced task door: exit 0, `dropped` holds both ids"
observed: "<W>/runs/cand-*/ (stdout, stderr, exit, snap.before, snap.after, snap.diff, porcelain.*, head.*); the previous release: <W>/runs/prev-*/; the routes: <W>/runs/routes-discard.log, <W>/runs/routes-uninstall.log"
pinned-by: "UNPINNED: found this round. A name search (`grep -rln 'dropped staged edits' crates/cli/tests tooling-tests`) gives task_lifecycle.rs and staged_prose_consent_axis.rs, neither of which mentions a link; destroying_door_sibling_surfaces.rs states the bare-door half in its header as driven by hand, not as a test. No suite was run by this verifier."
```

**Pinnable as it stands: the first two expectations yes; the last two only as a statement of
today's behaviour.** The fixture is a named state of the shared builder and every step is an argv
or one filesystem call, so the block converts by hand. Conditions: a Unix target (a link, a mode);
the mode-000 variant needs a caller that is not root, though at these two doors nothing reads the
file's bytes, so that variant is expected to hold for root as well — not driven. The refusing half
(expectations 1 and 2) is the half worth a standing test: it is the property the clause asks for.
Expectations 3 and 4 pin a silence that a repair of this row would change on purpose; whoever
converts them should write them as the red test of that repair, with `dropped` holding both ids,
and not as a fact to keep. The comparison with the previous release is not a suite's to hold.

## Left open

Not pursued; each is for triage like any finding.

1. **A refusal names a staged doc that its own read route says is not staged.** Under the mode-000
   plant both doors refuse naming `research:locked`, and route at `jigc doc show <address> --task <id>`;
   that command with that address exits 1 with `store.not-staged`. Identical on the previous
   release. A printed route whose command fails — the second clause's subject, not this finding's.
2. **`jigc task finalize <id>` under the mode-000 plant exits 1 with a raw OS error, the machine's
   absolute path and no route** (`enumerating the task's code-anchor surface at "<abs repo>/…":
   Permission denied (os error 13)`). It is a route both staged-prose refusals print. Identical on
   the previous release.
3. **The forced doors' missing names under the dangling link** — the defect this report's verdict
   leaves on the ledger: the text ack with no list, `"dropped": []` on the JSON ack, no staged-docs
   warning at `uninstall --force`, and on the candidate only the sentence *Anything you had staged in
   one is named above* beside that silence.
4. **Callers not driven:** `orient.rs:212` (orientation's staged list) and the milestone door
   (`milestone.rs:6099`, `milestone.rs:5897`) under these plants.
5. **Reach was not driven here.** Whether any jigc verb can leave a link or an unreadable file in a
   staging area was not tested by this verifier; the verdict does not depend on it.
6. **Linux, and a root caller, were not driven.**

<!-- end of report -->
