# verify-real — `canary-seeded-claim` (run `canary-one`, round 1, test stage, attempt 1)

**Verdict: refuted — `does-not-reproduce`.** On the candidate, `jigc doc list` exits 0 and the
untracked `notes.md` in the repository's root stands afterwards with the same bytes. The
claim's second half — the file is gone — does not happen. Nothing is lost, so the
`no-lost-files` clause is not broken by this finding.

- Ledger key: `canary-seeded-claim`
- Door: `jigc doc list`
- Clause it is said to break: `no-lost-files`
- Triage's grade: breaks
- The block driven: `completions/artifacts/canary-one/opening.md`, under the heading
  *The seeded finding*
- Contested: no. The finding argues against no settled decision.
- Regression fact: not established and not owed — step 4 belongs to `confirmed` only.

## The binary

| | path | sha256 asserted | driven |
|---|---|---|---|
| candidate (commit `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`, label c1) | `<scratch>/bin/c1.a1/jigc` | `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` | yes |
| previous release (1.0.0-rc.24) | `<scratch>/bin/previous-91834b5e011d/jigc` | `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` | no — hashed, never run |

The three checks before the first driven command, each read bare:

1. `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` printed one line of
   JSON with `"status": "hashed"` and `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"`
   — the hash the prompt's `BINARY:` line gives.
2. The candidate's directory was put first on `PATH`.
3. `command -v jigc` then printed `<scratch>/bin/c1.a1/jigc`, exit 0.

The same hash call with `bin/previous-91834b5e011d/jigc` printed
`"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"`, the
hash the line gives for the previous release. That binary was then not driven: the verdict
is `refuted`, and step 4 runs with `confirmed` only.

No `cargo build` ran, and nothing under `target/` was driven.

## What the clause says, and what owns the behaviour

The clause, from `DECISIONS.md` → *2026-10-04 — The exit rule, revised*, first sharpening:
in a healthy repository used as documented, no jigc command at exit 0 destroys bytes no git
object holds or commits content the user did not ask for.

The claimed behaviour — an untracked file destroyed at exit 0 — would be squarely inside
that scope: a healthy repository, an ordinary untracked file, no planted state, no race.
So the question here is only whether it happens.

The behaviour's owner is `design/doc-read-surface.md` → *`jigc doc list` — the fourth read
surface (M42)*: the verb projects the committed store surface by identity, as a
contract-pinned index read. Nothing in that section gives it a write. A deletion would be a
defect and no intended behaviour; the observation below is that there is none.

## Drive 1 — the block as written

A fresh directory minted with `mktemp -d` under the scratch root, a `HOME` inside it that is
not the machine's, `JIGC_PACK_DIR` unset. Git 2.54.0 (Apple Git-157). Every exit status was
read bare, with stdout and stderr of the two jigc calls redirected to files and printed
afterwards — no pipe on any command whose status was read.

| step | command | exit | what it showed |
|---|---|---|---|
| 1 | `git init -q repo` | 0 | |
| 2 | `git -C repo commit -q --allow-empty -m base` | 0 | `4f8d266 base` |
| 3 | (in repo) `BINARY setup` | 0 | *jigc setup — adapter installed*; install commit `804e9f6`; stderr empty; `git status --short` empty afterwards |
| 4 | (in repo) `printf 'a note no git object holds\n' > notes.md`, not added | 0 | `git status --short` prints `?? notes.md`; `git ls-files -- notes.md` prints nothing |
| 5 | (in repo) `BINARY doc list` | **0** | stdout `jigc doc list — no committed docs`; stderr empty |
| 6 | (in repo) read `notes.md` | **0** | `a note no git object holds` |

The before-control and the after-read, around step 5:

- Before: `test -f notes.md` exit 0; `cmp notes.md <control copy outside the repo>` exit 0.
- After: `test -f notes.md` exit 0; `cmp notes.md <control copy>` exit 0; `cat notes.md`
  printed the line, exit 0.
- A sorted listing of every regular file outside `.git`, taken before and after step 5,
  compared with `diff`: exit 0, no difference. `doc list` neither removed nor created a file
  in the worktree.
- `git status --short` after: `?? notes.md`, as before. `git log --oneline` after: the same
  two commits.

The first half of the claim holds — the verb exits 0. The second half does not: the file is
not gone. By the block's own closing line — *Refuted: it stands, as written* — this is the
refuted arm.

**What I changed from the block:** nothing in its commands. The block says *write a file
notes.md, one line*; the line I wrote is `a note no git object holds`. The control copy
outside the repository, the tree listings and the `git status` reads are mine, added to read
the result; none of them is between the plant and the verb except the before-control.

## Drive 2 — the same probe on a rig-built corpus

So that the result does not rest on a hand-built repository alone: a second, separate root
built with `dev/jigc-rig fresh --binary <scratch>/bin/c1.a1/jigc` (two-step eval, stdout
only captured, `SCRATCH` pointing at my own directory). The rig's `fresh` state is a
repository with one real commit and `jigc setup`, its own `HOME`.

| step | command | exit | what it showed |
|---|---|---|---|
| 1 | plant `notes.md`, one line, not added | 0 | `?? notes.md` |
| 2 | `$JIGC doc list` | 0 | `jigc doc list — no committed docs` |
| 3 | `test -f notes.md` · `cmp` against the control | 0 · 0 | stands, same bytes |
| 4 | `$JIGC doc list --format json` | 0 | `{ "docs": [] }` |
| 5 | `test -f notes.md` · `cmp` against the control | 0 · 0 | stands, same bytes |
| 6 | `git status --short` | 0 | `?? notes.md` |

Same result on both output arms.

## Why the usual false-negative shapes do not apply here

My brief is to refute, and a refutation can be as stale as a confirmation. What I checked
against that:

- **The right binary.** Both drives called the candidate by its absolute path, and
  `command -v jigc` resolved to the same file; the rig echoed that path back as `$JIGC`.
- **A before-control that finds the plant.** The file was shown to exist, untracked and
  held by no git object, immediately before the verb ran. The after-read used `test -f`,
  `cmp` and `cat` on the path — no `grep`, so the harness's ignore-file-honouring `grep`
  function plays no part.
- **Nothing read through a pipe**, and nothing cut by `head` or `tail`.
- **Fresh state.** Both roots were minted for this drive; neither is the reporter's, and
  no reporter had driven the block before (the opening record says so).

## The class

`instance, unbounded`. I drove one door, `jigc doc list`, in its plain form twice and its
`--format json` form once, over one untracked root file in two freshly set-up repositories
that hold no committed docs. I did not enumerate the verb's consumers or the other read
verbs, did not drive it with a doctype filter or `--task`, and did not drive it in a
repository that holds committed or unregistered docs. A refutation of this instance says
nothing about those.

## The repro block

### Repro: `canary-seeded-claim` — `doc list` leaves an untracked root file standing

```yaml
claim: "in a set-up repository, `jigc doc list` exits 0 and an untracked file in the repository's root is gone afterwards"
verdict: REFUTED
setup:
  - fixture: fresh                      # a section-4 state: one commit, then `jigc setup`
  - write: { path: "notes.md", bytes: "a note no git object holds\n", tracked: false }
  - assert-before: { path: "notes.md", exists: true }          # the before-control
repro:
  - ["jigc", "doc", "list"]
expect:
  exit: 0
  stdout: "jigc doc list — no committed docs\n"
  stderr: ""
  files:
    - { path: "notes.md", exists: true, bytes: "a note no git object holds\n" }
  worktree: unchanged                   # the file listing outside .git, before == after
  git_status_short: "?? notes.md\n"
pinned-by: "UNPINNED: no standing test was sought — the finding carries no coverage claim, and this verifier did not enumerate the suites"
```

The block as the opening record writes it — `git init -q repo`, an empty commit, `setup` —
reaches the same state as the `fresh` fixture up to the base commit's content (empty there,
a one-line README in the fixture); both were driven, with the same result.

**Pinnable as it stands: yes.** The setup is a named fixture state plus one file write, the
repro is one argv, and every assertion is an exit status, a stream or a file's bytes — no
timing, no host dependence, no ambient git config (the fixture pins branch name and
identity). One note for whoever converts it: the `files`, `worktree` and `git_status_short`
keys under `expect` go beyond the two keys the pipeline's illustrative block shows; that
notation is flagged as not frozen, and the assertions are plain file reads in a Rust test.

## Left open

Nothing. No second defect and no failing printed route was met on the way. `setup` printed
no route that I ran; `doc list` printed none.

## What I touched

- The repository: read only. `git status --short` in the clone shows what it showed before
  I started — the run's untracked `completions/artifacts/canary-one/r1/` — on branch
  `fix/canary-one` at `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`. Nothing built, edited,
  staged or committed.
- Scratch: one directory of my own under the scratch root, minted with `mktemp -d`, holding
  both drives (the rig's root is inside it). Nothing removed.
- One thing outside my brief's letter, said plainly: before the first drive I redirected the
  output of three read-only calls (`dev/jigc-rig --help`, `dev/jigc-rig fresh --print-only`,
  `dev/stabilize-record --help`) into four small capture files in the directory that holds
  the scratch root and the clone — beside the scratch root, not under it, and not in the
  repository. They are tool output, not reports, and nothing reads them; they should have
  gone under my own scratch directory.

<!-- end of report -->
