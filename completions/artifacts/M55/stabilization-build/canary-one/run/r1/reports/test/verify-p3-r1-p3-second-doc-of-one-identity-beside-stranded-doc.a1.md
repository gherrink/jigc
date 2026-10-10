# verify-real — `r1-p3-second-doc-of-one-identity-beside-stranded-doc`

Run `canary-one`, round 1, stage `test`, attempt 1. One finding: doors `jigc doc create`
and `jigc task finalize`, the clause it is said to break `no-lost-files`, triage's grade
*unclear*.

## Verdict in one paragraph

**REFUTED as a blocker — `breaks-no-clause`.** The behaviour is real and reproduces as
the finding states it, on both binaries: with `research:context-loss` committed at
`docs/research/context-loss.md` and `docs-root` resolving to `notes`, `jigc doc create
research --title "Context Loss"` exits 0 and acks a bare `research:context-loss`, and its
task's finalize exits 0 and commits `notes/research/context-loss.md` beside the first
doc. `jigc doc list` then lists the new doc alone and `jigc validate` names the first as
an advisory. But neither term the first clause is measured by is met at either door. **No
byte is destroyed:** the first doc's blob is the same before and after, in the work tree
and at `HEAD`, in all four cells. **No content the user did not ask for is committed:**
the commit holds two paths — the doc the user created by a typed command, carrying the
three slots the user wrote, at the home the cascade resolves, and the `docs-root` knob
the user set through `jigc config set`, which a finalize carries by design. That holds
whether or not the stranded state is inside the clause's scope, so the verdict does not
rest on a reading of the scope. **It does rest on reading *nothing incorrect is written*
by the sharpening's own two terms** — see *What this verdict does not settle*.

## The binary, asserted before anything was driven

| which | call | `content_sha256` printed | matches the prompt |
|---|---|---|---|
| candidate, label c1, commit `eeffe347` | `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` | `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` | yes |
| previous release, 1.0.0-rc.24 | `dev/stabilize-step hash --scratch <scratch> --file bin/previous-91834b5e011d/jigc` | `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` | yes |

Both calls ran before the first driven command. In every candidate cell the candidate's
directory was first on `PATH` and `command -v jigc` printed `<scratch>/bin/c1.a1/jigc`;
in the previous release's cells its own directory was first instead and `command -v jigc`
printed `<scratch>/bin/previous-91834b5e011d/jigc`. Nothing was built; nothing under
`target/` was driven. Every rig was built with `dev/jigc-rig refs-post-hoc --binary <that
path>`, one fresh rig per cell, stdout alone captured.

## What was read

The finding as the prompt hands it, and the fourth cell (F) of the block `Repro VR-DS-1`
in the report that carries it, with that report's own account of cell F. No other
finding's report and no other verifier's. The clause:
`completions/artifacts/canary-one/opening.md` → The closing condition, and `DECISIONS.md`
→ *2026-10-04 — The exit rule, revised*, the first sharpening. The design that owns the
behaviour: `design/write-commands.md` → *Create-only — the `new` key* (which states
create-or-update), `design/storage.md` → *Changing `docs-root` can orphan docs at the old
root*, `design/validation.md` → *Orphan detection* and its M40 two-tier route, and
`design/finalize.md` on the pending config delta an ordinary finalize commits. Of the
code: the `do-research` workflow's create gate
(`crates/cli/packs/methodology/workflows/do-research.yaml`) and the call sites of
`orphan::orphaned_docs`.

## What was driven

Five rigs under `<scratch>/verify-p3-second.Vrn7IZ`. Every exit status was read bare, on
the line after its command; stdout and stderr went to separate files.

| cell | binary | how the strand was reached | then |
|---|---|---|---|
| F | candidate | the block as written: plant · `config set docs-root notes` · unplant | `start --workflow do-research` · `doc create` · three slots · commit doc · `task validate` · `task finalize` |
| FP | previous | the same | the same |
| N | candidate | **no plant at any point:** `config set docs-root notes` · `git reset --hard` | the same |
| NP | previous | the same as N | the same |
| K | candidate | no strand: the doc at its resolved home | `start --workflow do-research` · `doc create` |

The plant and its removal, cells F and FP only, as the block gives them:

    blob=$(printf 'x\n' | git hash-object -w --stdin)
    git update-index --add --cacheinfo "100644,$blob,bad<byte 0xFF>name.txt"     # exit 0
    git update-index --force-remove -- "bad<byte 0xFF>name.txt"                  # exit 0

### The strand, before the create

In all four strand cells, on both binaries: `jigc config get docs-root` prints `docs-root
= notes  (project)`; `git ls-files -- docs/research notes` prints
`docs/research/context-loss.md` alone; `git status --short` prints one line, `??
.jigc/config/manifest.yaml`; and `jigc doc show research:context-loss` exits 1 with
`store.not-found` and the route that names the doc's path — *is committed at
`docs/research/context-loss.md`, outside the home this read resolves*. So the read door
knows where the doc is.

The two routes to the strand leave one difference, in the gitignored registry
`.jigc/state/file-state.json`:

- **F, FP** — `config set` relocated nothing (`"relocated": []`), so the registry still
  holds the first doc at `docs/research/context-loss.md`.
- **N, NP** — `config set` relocated the doc (`"relocated"` holds the one pair, staged as
  a rename) and re-keyed its record to `notes/research/context-loss.md`; `git reset
  --hard` undid the rename and not the record. The registry holds the first doc's hash
  under a path that does not exist, and no record at the path where the doc is.

### The create and its finalize — every exit 0, in all four strand cells

| step | exit | what it printed |
|---|---|---|
| `jigc start --workflow do-research "re-create the research the read could not find"` | 0 | `task minted: re-create-the-research`; stderr empty |
| `jigc doc create research --title "Context Loss" --task re-create-the-research` | 0 | `research:context-loss` — one bare line, stderr empty |
| `jigc doc set-slot research:context-loss#question` / `#findings` / `#sources` | 0, 0, 0 | `set slot …` each |
| the commit doc: `#type` `docs`, `#scope` `research`, `#summary`, `#body` | 0, 0, 0, 0 | `set …` each |
| `jigc task validate re-create-the-research` | 0 | advisory `file-state.staged-copy` at `notes/research/context-loss.md` (and, in N and NP, one more — below) |
| `jigc task finalize re-create-the-research` | 0 | `finalized <sha> — docs(research): re-create the context-loss research` · `added .jigc/config/manifest.yaml` · `promoted notes/research/context-loss.md` · `2 files committed` |

In cells F and FP **no door on that path names the first doc or says that the identity
has a committed doc**: the mint, the create's ack, the gate preview and the finalize say
nothing of `docs/research/context-loss.md`. In cells N and NP the gate preview and the
finalize each carry one more advisory, `reconciliation.rename` — *tracked managed doc
research:context-loss (notes/research/context-loss.md) is missing, has no history at
HEAD, and no branch carries it — the baseline outlived the doc* — which names the
identity, as a stale baseline, and not the committed doc at `docs/`.

### What stands afterwards

Identical in all four strand cells unless a row says otherwise.

| what | observed |
|---|---|
| `git ls-files -- docs/research notes` | exit 0: `docs/research/context-loss.md` and `notes/research/context-loss.md` — **both** |
| the commit, `git show --stat HEAD` | two paths: `.jigc/config/manifest.yaml` (2 lines) and `notes/research/context-loss.md` (18 lines) |
| `git status --short` | empty |
| the first doc's bytes | `git hash-object docs/research/context-loss.md` and `git rev-parse HEAD:docs/research/context-loss.md` both print `3507022f2299fc2746a813610ea9732edd8d58ff`, the value read before the strand |
| `jigc doc list` | exit 0, five rows; the one `research` row is `research:context-loss  notes/research/context-loss.md  managed`. The first doc has no row |
| `jigc doc show research:context-loss` | exit 0, serves the new doc — the three slots this drive wrote |
| `jigc validate`, of the first doc — **F, FP** | exit 0, advisory `file-state.orphaned-doc` at `docs/research/context-loss.md`: *sits outside the resolved doctype roots — a `docs-root` change likely stranded it*; route: *move it under the current resolved root (re-point `docs-root` to cover it) or drop it with `jigc unmanage`* |
| `jigc validate`, of the first doc — **N, NP** | exit 0, advisory `file-state.unregistered-doc` at the same path: *looks managed … but was never adopted*; route: *adopt it with `jigc migrate <scratch>/…/repo/docs/research/context-loss.md --as research`, or ignore it*. The same advisory stood before the create |
| the registry, `.jigc/state/file-state.json` — **F, FP** | holds both paths: the first doc's record unchanged, the new doc's added |
| the registry — **N, NP** | the record at `notes/research/context-loss.md` now holds the new doc's hash in place of the first doc's; still none at `docs/research/context-loss.md` |
| the edge index, `.jigc/index/` | holds `edges.json.lock` and no `edges.json`, before and after, in every cell |

### The control, cell K

With the doc at its resolved home, the same two commands: `jigc start --workflow
do-research …` exits 0, and `jigc doc create research --title "Context Loss" --task
re-create-the-research` exits 0 and acks **`research:context-loss (already existed —
copied in for update)`**; `jigc doc show research:context-loss --task
re-create-the-research` then serves the committed doc's own three slots. So the create
door does tell a fresh mint from an identity that is taken — by the resolved home alone.

### The two binaries, compared

Byte for byte (`cmp` exit 0) between F and FP: the `config set` ack, the read's refusal,
the create's stdout and stderr, the gate preview, the `ls-files` listing, `doc list`'s
stdout and stderr, `jigc validate`'s stdout, and the served doc. The same between N and
NP for the outputs that carry no rig path; the two that do — the gate preview and
`jigc validate` — are equal once the rig's directory name is masked. The finalize
receipts are equal once the commit hash is masked. The previous release does everything
the candidate does here.

## Is it what the finding says, read against the design that owns it?

Yes for the behaviour, and nothing in the design states it as intended.

- `design/write-commands.md` gives a create gate without `new: true` the rule
  **create-or-update**: *a create whose minted id is already a doc copies that doc in
  for update*. `do-research` declares `{ type: research, as: record }`, no `new`. Cell K
  is that rule, driven. Under the strand the same id is minted fresh, because the
  occupancy the create consults is the home the cascade resolves, and that home is free.
- `design/storage.md` says what the first doc is in that state: *committed managed docs
  already at the prior resolved root no longer resolve under any doctype location — they
  become orphans*, detected by the `file-state.orphaned-doc` advisory. `design/validation.md`
  holds that detector to *pure read, findings only*, and its M40 revision to
  *discriminate on registration, never gate on it*. So the advisory at exit 0 that
  `jigc validate` prints after the second doc lands is the design's stated answer to an
  orphan; and by the design's own words the first doc holds no identity in the resolved
  store.
- What the design does not say is that a create consults the strand walk. The read door
  does (the path-naming route above); `grep` finds three call sites of
  `orphan::orphaned_docs` outside tests under `crates/cli/src` — `cli.rs` once and
  `doc.rs` twice — and none on the create path. So the finding's sentence holds as
  driven: the read knows the doc is committed at `docs/`, and the create that follows
  mints over the same id without saying so.

That is a real gap between two doors over one state. It is not behaviour a settled
decision intends, and it is not behaviour a settled decision forbids.

## Does it break `no-lost-files`, inside that clause's scope?

The clause, as the run's opening names it: *no file is lost and nothing incorrect is
written or updated, in the scope that entry's first sharpening gives it*. The sharpening:
*In a healthy repository used as documented … no jigc command at exit 0 destroys bytes no
git object holds or commits content the user did not ask for. Where jigc cannot tell (git
fails, the index is unreadable) it refuses before writing. … deliberately planted states,
are declared bounds*.

**Destroys bytes no git object holds — no.** Neither door removed or rewrote any file
but the one it promoted to a path that did not exist. The first doc's blob is unchanged
in the work tree and at `HEAD` in all four cells, and it was a committed blob before the
doors ran.

**Commits content the user did not ask for — no.** The commit's two paths:

1. `notes/research/context-loss.md` — the doc the user asked for by `jigc doc create
   research --title "Context Loss"`, holding exactly the three slots written through
   `jigc doc set-slot`, at the home `docs-root = notes` resolves.
2. `.jigc/config/manifest.yaml` — the knob the user set with `jigc config set docs-root
   notes`, acked `"committed": false`. `design/finalize.md` states that an ordinary
   code-less finalize commits a pending `.jigc/config` delta beside the doc.

**Where jigc cannot tell it refuses — not engaged.** Git did not fail and the index was
readable at both doors; in cells F and FP the entry had been removed before either ran.

**The scope.** Cells F and FP reach the strand through a plumbing plant, removed before
the doors run; cells N and NP reach it by two ordinary commands with no plant. Whether
`config set docs-root` followed by `git reset --hard` is *a healthy repository used as
documented* was not decided here and does not need to be: the two terms above are unmet
in all four cells, inside the scope or outside it.

A real defect, then, that breaks no clause inside its scope: `breaks-no-clause`, never
`does-not-reproduce`. It stays a row of the ledger.

### What this verdict does not settle

The opening's short form of the clause says *nothing incorrect is written or updated*;
the sharpening it points at measures that as *commits content the user did not ask for*.
This verdict uses the sharpening's term. A reader who holds that a second committed doc
under an id whose first doc is still committed is itself *an incorrect thing written* —
whoever typed the create — reads the clause wider than its sharpening states, and under
that reading this row is confirmed, with `regression: false`: the previous release is
identical on the same block (cells FP and NP). That reading is the human's to give. No
decision is argued to be wrong by the finding, so nothing here is contested.

## The regression fact

Not owed with this verdict. The fact, since both binaries were driven on fresh rigs: the
block gives the same exits and the same output on the previous release (the comparison
above). Not a regression.

## Class

**instance, unbounded.** One doctype (`research`, a `location:` home), one knob
(`docs-root`), one workflow (`do-research`, a create-or-update gate), one title, one
host, one git. Not driven: `jigc doc author` as the minting door, a `new: true` gate, a
`placement-root` strand, the fan-out boundary. The three call sites of
`orphan::orphaned_docs` were read from a `grep`, not derived as the mechanism's consumer
set.

## Repro VR-SD-1

```yaml
claim: "beside a doc a docs-root re-point left at its prior home, `jigc doc create` and its task's finalize commit a second doc of the same id at exit 0, and no door names the first — and that breaks the clause no-lost-files"
verdict: "REFUTED as a blocker — breaks-no-clause. The second doc lands as claimed on both binaries; no byte is destroyed and the commit holds only what the user typed."
binary: "candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc, commit eeffe347; previous release 1.0.0-rc.24, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"
task: "re-create-the-research"     # minted from the intent below
fill:                              # every step exit 0; payloads on stdin
  - ["jigc", "doc", "set-slot", "research:context-loss#question", "--from-file", "-", "--task", "<task>"]
  - ["jigc", "doc", "set-slot", "research:context-loss#findings", "--from-file", "-", "--task", "<task>"]
  - ["jigc", "doc", "set-slot", "research:context-loss#sources", "--from-file", "-", "--task", "<task>"]
  - ["jigc", "doc", "set-field", "commit:<task>#type", "--value", "docs", "--task", "<task>"]
  - ["jigc", "doc", "set-field", "commit:<task>#scope", "--value", "research", "--task", "<task>"]
  - ["jigc", "doc", "set-slot", "commit:<task>#summary", "--from-file", "-", "--task", "<task>"]
  - ["jigc", "doc", "set-slot", "commit:<task>#body", "--from-file", "-", "--task", "<task>"]
cells:
  - cell: "the strand with no plant — the second doc, and nothing lost (N, NP)"
    setup:
      - fixture: refs-post-hoc          # dev/jigc-rig refs-post-hoc --binary <binary>
      - "first=$(git hash-object docs/research/context-loss.md)"
      - ["jigc", "config", "set", "docs-root", "notes", "--format", "json"]      # exit 0, relocated: one pair
      - ["git", "reset", "--hard"]                                              # exit 0
    repro:
      - ["jigc", "doc", "show", "research:context-loss"]
      - ["jigc", "start", "--workflow", "do-research", "re-create the research the read could not find"]
      - ["jigc", "doc", "create", "research", "--title", "Context Loss", "--task", "<task>"]
      - fill
      - ["jigc", "task", "finalize", "<task>"]
      - ["git", "ls-files", "--", "docs/research", "notes"]
      - ["git", "show", "--name-only", "--format=", "HEAD"]
      - ["git", "rev-parse", "HEAD:docs/research/context-loss.md"]
      - ["git", "hash-object", "docs/research/context-loss.md"]
      - ["jigc", "doc", "list"]
      - ["jigc", "validate"]
    expect:
      - exit: 1
        stderr_contains: ["store.not-found", "is committed at `docs/research/context-loss.md`, outside the home this read resolves"]
      - exit: 0
        stdout_contains: "task minted: re-create-the-research"
      - exit: 0
        stdout: "research:context-loss"          # the defect: a fresh mint, the first doc unnamed
        stderr: ""
      - exit: 0
      - exit: 0
        stdout_contains: ["promoted notes/research/context-loss.md", "2 files committed"]
      - exit: 0
        stdout: "docs/research/context-loss.md\nnotes/research/context-loss.md"
      - exit: 0
        stdout: ".jigc/config/manifest.yaml\nnotes/research/context-loss.md"     # the clause fact: nothing else is committed
      - exit: 0
        stdout: "<first>"                        # the clause fact: the first doc's bytes stand in git
      - exit: 0
        stdout: "<first>"                        # … and in the work tree
      - exit: 0
        stdout_contains: "research:context-loss  notes/research/context-loss.md  managed"
        stdout_not_contains: "docs/research/context-loss.md"
      - exit: 0
        stdout_contains: ["file-state.unregistered-doc", "docs/research/context-loss.md"]
    on-previous: "identical — every output compared, byte for byte or with the rig's directory name and the commit hash masked"
  - cell: "the block's cell F as written — plant, re-point, unplant (F, FP)"
    setup:
      - fixture: refs-post-hoc
      - "first=$(git hash-object docs/research/context-loss.md)"
      - "blob=$(printf 'x\n' | git hash-object -w --stdin)"
      - ["git", "update-index", "--add", "--cacheinfo", "100644,<blob>,bad<byte 0xFF>name.txt"]   # exit 0
      - ["jigc", "config", "set", "docs-root", "notes", "--format", "json"]                       # exit 0, relocated: []
      - ["git", "update-index", "--force-remove", "--", "bad<byte 0xFF>name.txt"]                 # exit 0
    repro: "the first cell's eleven steps"
    expect: "the first cell's, with one change: `jigc validate` names the first doc as `file-state.orphaned-doc` in place of `file-state.unregistered-doc`"
    on-previous: "identical — every output compared byte for byte, the finalize receipt with the commit hash masked"
  - cell: "the control — the same create with the doc at its resolved home (K, candidate only)"
    setup:
      - fixture: refs-post-hoc
      - ["jigc", "start", "--workflow", "do-research", "re-create the research the read could not find"]   # exit 0
    repro:
      - ["jigc", "doc", "create", "research", "--title", "Context Loss", "--task", "<task>"]
    expect:
      - exit: 0
        stdout: "research:context-loss (already existed — copied in for update)"
        stderr: ""
    on-previous: "not driven"
observed: "<scratch>/verify-p3-second.Vrn7IZ — f.* (cell F), fp.* (FP), n.* (N), np.* (NP), k.* (K); rig-<cell>.env and rig-<cell>.err are each rig's assignments and construction log; the rigs are the five jigc-rig-refs-post-hoc-* directories beside them"
pinned-by: "UNPINNED: the first two cells hold a defect as observed, and what a fix should make the create do is not ruled; the third states the design's rule and was not checked against the suites — see Pinnable"
```

**Pinnable as it stands: no.** The first two cells, pinned green, would hold in place a
create that mints beside a committed doc of the same id without naming it. Their three
clause assertions — the commit's path set and the first doc's blob in git and in the work
tree — are worth keeping whatever a fix does, but they sit behind a create and a finalize
that a fix may turn into a refusal or a copy-in, and which of those is right is not
decided: the block is a fix's red test in waiting once that is, with the third `expect`
changed. The third cell converts as it is — plain argv, one exit, one exact line — and is
the design's create-or-update rule; whether a suite already asserts it was not checked,
and the finding makes no coverage claim, so none was verified. The plant in the second
cell passes a name as raw bytes, so a test that uses it is Unix-only; the first cell
needs no plant.

## Left open

1. **The reading this verdict rests on.** Whether a second committed doc under an id
   whose first doc is still committed is *an incorrect thing written* in the first
   clause's sense, when the user typed the create. The sharpening's term is *content the
   user did not ask for*, and by it the answer is no. Read wider, this row is confirmed,
   `regression: false`.
2. **After `config set docs-root` and `git reset --hard`, `jigc validate` says of a doc
   jigc itself created and relocated that it *was never adopted*** —
   `file-state.unregistered-doc`, in place of the strand advisory — because the gitignored
   registry kept the record under the path the reset removed. Both binaries, before and
   after the create (cells N, NP). Its route was not run.
3. **That advisory's route prints an absolute host path** — `jigc migrate <absolute path
   of the repository>/docs/research/context-loss.md --as research` — where every other
   route in the same output names a repository-relative path. Both binaries (cells N,
   NP).
4. **`reconciliation.rename` at `jigc task validate` and `jigc task finalize` says the
   path *is missing* and routes `jigc unmanage notes/research/context-loss.md`, in the
   same output that reports this task's staged copy of that path** (cells N, NP, both
   binaries). The route was not run, so what it does to the task's staged doc is not
   known.

## Bounds — what this verification did not do

- One macOS host, git 2.54.0.
- The repair routes `jigc validate` prints for the first doc once the second has landed
  — the knob re-pointed back, the move, `jigc unmanage`, `jigc migrate` — were not run.
  What a re-point back to `docs` does when both homes hold a doc is not known from this
  report.
- The rig's open task `ground-the-vision-in-research` stages a reference from the vision
  to `research:context-loss`. Which doc that reference reads after the second doc lands,
  and what that task's finalize says, was not driven.
- `jigc doc author` as the minting door, a `new: true` gate, a `placement-root` strand
  and the fan-out boundary were not driven.
- Cell K was not driven on the previous release.
- No coverage was verified; no suite was read.
- Nothing was built, edited, staged or committed. The clone stands at `eeffe347` on
  `fix/canary-one`, its only untracked path the run's own `r1/` directory, as before.

## Where the evidence is

`<scratch>/verify-p3-second.Vrn7IZ`: the five rigs, `rig-*.env` and `rig-*.err`, and the
cells' captured stdout and stderr — `f.*`, `fp.*`, `n.*`, `np.*`, `k.*`.

<!-- end of report -->
