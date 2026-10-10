# verify-real — `r1-git-capture-other-callers-not-enumerated`

Run `canary-one`, round 1, stage `test`, attempt 1. One finding, handed to this verifier: the
`Class` paragraph and `Left open` item 7 of
`completions/artifacts/canary-one/r1/reports/test/verify-p1-r1-non-utf8-path-other-orphan-walk-consumers.a1.md`
— *the other callers of `task::git_capture`, each of which shares the UTF-8 step; not
enumerated, not driven*. No door named, no block of its own; the plant is that of that report's
`Repro VR-LD-2`. Clause it is said to break: `no-lost-files`. Triage's grade: *unclear*.

## Verdict in one paragraph

**`refuted` — as a blocker, with the basis `breaks-no-clause`. It is not `does-not-reproduce`:
the shared UTF-8 step is real and was reached at five call sites other than the one already
driven.** `task::git_capture` has 34 production call sites at `eeffe347`. With the index entry
of `Repro VR-LD-2` exactly as written — one root-level name holding the byte 0xFF, default git
configuration — **one** of the 34 receives output it cannot decode: `orphan.rs:149`, the site
the handed report already drove. At every other door that writes, commits or removes, that
entry behaves as its ASCII control does, or the door refuses at exit 1 for a reason that is not
this function. Where I changed the plant so that another site's capture *is* undecodable (the
name under `.jigc/`, the name ending `.md`, `core.quotePath=false`, a staged file holding a
Latin-1 byte), the door **refused with a non-zero exit before writing** at four sites, and at
the two sites that swallow the error nothing was destroyed without consent and nothing was
committed that the control does not commit: `jigc rename` lands the same commit and prints an
empty advisory list, and `jigc uninstall --force` removes what `--force` consents to and prints
neither of its warning blocks. No cell, on either binary, shows a capture that could not be
decoded letting a door destroy bytes no git object holds or commit content nobody asked for at
exit 0. The candidate and the previous release agree in every one of 58 cells on exit status,
on whether `HEAD` moved and on what stands afterwards. **`orphan.rs:149`'s own three writing
doors are another row's and are not graded here** — see *Left open*, item 5.

## The binaries, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` printed
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the
  hash the prompt gives for the candidate (commit `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`,
  label c1).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the
  previous release's. **It was driven**, through the same script and the same cells: triage
  asked for both binaries.
- The candidate's directory went first on `PATH` and `command -v jigc` printed the candidate's
  path. The driver repeats that check per cell with the binary it was handed and stops if it
  fails; for the previous release's cells the previous release's directory is the one put
  first, inside that cell's process only. Every rig was built with
  `dev/jigc-rig <state> --binary <that binary>`, and the driver stops unless the rig's `$JIGC`
  is that path. The installed `pre-commit` hook names its binary by absolute path, so a hook
  run in a rig is the rig's binary.
- git on this host: `git version 2.54.0 (Apple Git-157)`. One macOS host.

## The enumeration

`grep -rn "git_capture(" crates/` at `eeffe347`: 58 lines. 21 are three test files' own local
helpers of the same name (`crates/cli/tests/adapter_artifact.rs`, `fixed_identity_axis.rs`,
`setup.rs`). 37 are in `crates/cli/src` (the engine crate has none): the definition
(`task.rs:10087`), 2 calls inside `task.rs`'s `#[cfg(test)]` module (lines 11242, 11329), and
**34 production call sites**. A second grep for the name without a call's parenthesis finds one
import line and one doc comment, so no site takes the function by reference.

Each site was read with its argv, and classed by **what git's standard output can hold** —
which is what decides whether the capture can fail to decode at all:

- **A — an object id only** (`rev-parse`, `hash-object`, `rev-list`): no name, no text. 12 sites.
- **B — commit message text** (`log --format=%s`, `%B`, `show --format=%h%n%s`). 5 sites.
- **C — path names, line-oriented, no `-z`.** git C-quotes a byte at or above 0x80 by default,
  so the capture is ASCII and decodes; it is raw only under `core.quotePath=false`. 14 sites.
  One of them, `task.rs:3755` (`git diff --cached`), also carries file **content**.
- **D — path names under `-z`**, raw under every configuration. 3 sites.

| site | enclosing function, and the door | argv | class | on an error |
|---|---|---|---|---|
| `rename.rs:1181` | `apply_and_commit` — `jigc rename` | `rev-parse --short HEAD` | A | swallowed (the sha reads `HEAD`) |
| `rename.rs:1303` | `scan_prose_mentions` — `jigc rename` | `ls-files` | C | **swallowed: an empty list** |
| `ingest.rs:312` | `git_candidates` — `jigc ingest` | `ls-files -z --cached --others --exclude-standard -- *.md` | D, scoped to `*.md` | propagated |
| `task.rs:3026` | `amend_index_findings` — `jigc task finalize`, amend arm, and its preview | `diff --cached --name-only HEAD` | C | propagated |
| `task.rs:3577` | `finalize` | `rev-parse --short HEAD` | A | propagated |
| `task.rs:3755` | `finalize`, ordinary arm | `diff --cached` | C, and content | propagated |
| `task.rs:3762` | `finalize`, ordinary arm | `status … -- .jigc/config .jigc/.gitignore` | C, scoped | propagated |
| `task.rs:3956`, `3957` | `finalize`, the amend forecast | `rev-parse --short` · `log -1 --pretty=format:%s` | A · B | propagated |
| `task.rs:4195`, `4196`, `4228` | `finalize`, after the commit | `rev-parse --short` · `log -1 …%s` · `rev-parse --short` | A · B · A | propagated |
| `task.rs:6221` | `migration_source_edited_since_mint` — `finalize`, migration arm | `hash-object` | A | swallowed |
| `task.rs:6239`, `6254` | `promotion_changes_head` — `finalize` | `rev-parse --verify` · `hash-object` | A · A | swallowed, toward *changed* |
| `task.rs:6270` | `owner_artifacts_change_head` — `finalize` | `status … -- <one literal path>` | C, scoped | swallowed, toward *changed* |
| `task.rs:6397` | `capture_owner_artifact_index` — `finalize`'s stage (3 calls) and the milestone record commit (`milestone.rs:1120`) | `ls-files --stage -- <one literal path>` | C, scoped | propagated |
| `task.rs:6678` | `capture_config_layer_index` — `finalize`'s stage | `ls-files --stage -- <the config layer>` | C, scoped | propagated |
| `task.rs:6740` | `rollback_config_layer_index` — `finalize`'s rollback | the same | C, scoped | swallowed |
| `task.rs:8362` | `git_changed_paths` — `jigc start --task` (`start.rs:2837`), `finalize` and its preview | `diff --no-renames --name-only <base> <head>` | C | propagated |
| `task.rs:9108` | `git_head` — 13 textual hits in `crates/cli/src`, the definition among them; not traced door by door | `rev-parse HEAD` | A | propagated |
| `task.rs:9191` | `co_author_for` — the commit seam | `log -1 --format=%B HEAD` | B | swallowed |
| `task.rs:9793` | `git_commit_files` — the file-state advance after a commit | `show --name-only … HEAD` | C | propagated, after the commit |
| `task.rs:9809` | `git_commit_name_status` — `finalize`'s landed manifest | `show --name-status … HEAD` | C | propagated, after the commit |
| `orphan.rs:149` | `committed_markdown` — 5 consumers: the listing's two, `jigc config set docs-root`, `jigc config set placement-root`, `jigc relocate` | `ls-files -z` | **D, unscoped** | **swallowed: an empty list** |
| `milestone.rs:1026` | `landed_record_sha` — the record commits of `milestone create`, `add-task` and a sub-task's discard | `rev-parse --short HEAD` | A | swallowed |
| `milestone.rs:1820` | `record_only_range` — `jigc milestone finalize` | `log --format= --name-only --no-renames <base>..<head>` | C | propagated |
| `milestone.rs:9473`, `9474`, `9475` | `milestone_landed_summary` — `milestone finalize`, after the boundary | `rev-parse --short` · `log -1 …%s` · `diff --name-status --no-renames` | A · B · C | propagated, after the commit |
| `milestone.rs:9609`, `9616`, `9619` | `boundary_commits` — the same | `rev-list` · `show …%h%n%s` · `show --name-only` | A · B · C | propagated, after the commit |
| `setup.rs:5794` | `classify_workbench_paths` — `jigc uninstall`: its guard (`setup.rs:5854`) and its narration (`setup.rs:6172`) | `ls-files -z --cached --full-name -- .jigc` | D, scoped to `.jigc` | guard: propagated into a refusal · narration: **swallowed** |

12 + 5 + 14 + 3 = 34. By door: `jigc rename` 2 · `jigc ingest` 1 · `jigc task finalize` and
the helpers it shares with `jigc start --task` 21 · `committed_markdown` 1 · the `jigc
milestone` verbs 8 · `jigc uninstall` 1.

**What the table says before anything is driven.** The entry of `Repro VR-LD-2` is a
root-level name ending `.txt`. Under default git configuration it reaches a raw capture at one
site only — the unscoped `-z` listing at `orphan.rs:149`. The other two `-z` sites are scoped
past it (`*.md`, `.jigc`), and the 14 class-C sites receive the name C-quoted in ASCII. The
cells below hold that reading to the binary.

## What was driven

58 cells per binary, each on a fresh rig of its own under `<scratch>/verify-gc.TXNMKK/runs/`;
every command whose exit status is read ran bare, its standard output and standard error each
to a file of its own. The plant, as in `Repro VR-LD-2`:

    blob=$(printf 'x\n' | git hash-object -w --stdin)
    git update-index --add --cacheinfo "100644,$blob,<name>"        # exit 0

read back in every cell: after an `ff` plant `git ls-files -z` exits 0, holds one byte 0xFF and
is not valid UTF-8; after an `ascii` plant it is valid UTF-8.

The states:

- **none** — the rig as built.
- **ascii** / **ff** — the plant of `Repro VR-LD-2`: `badXname.txt` / `bad<byte 0xFF>name.txt`
  at the root, index only. *The cells triage asked for.*
- **asciiq** / **ffq** — the same, then `git config core.quotePath false`.
- **asciimd** / **ffmd** — the name ending `.md`; **asciij** / **ffj** — the name under `.jigc/`.
- **hc-** — the entry committed by a plain `git commit --no-verify`, so `HEAD` holds the name.
- **sw-** — the entry committed and then marked `git update-index --skip-worktree`.
- **wt-** — the plant made in the index of the sub-task's worktree.
- **latin1** — a real file `legacy.txt` holding one byte 0xE9 and no NUL, `git add`ed.

**What I changed from the block as handed.** It named no door and no corpus. The `ascii` and
`ff` rows are its plant unchanged. Every other state is mine, built because the handed plant
reaches no capture at these doors: each is the smallest change that makes one named site's
capture undecodable. `sw-` exists because this filesystem cannot hold the name (`touch` of it
fails, per the handed report), so a committed entry always reads as a deleted file and
`jigc rename` refuses a dirty tree before its scan; the flag gives the clean tree a host that
can hold the name has without one.

### The plant is live, on both binaries — `jigc config set docs-root notes --format json`

Rig `refs-post-hoc`. This is the handed report's Door 1, re-driven as the control that the
plant empties a capture on each binary — not to grade it.

| state | candidate | previous release |
|---|---|---|
| ascii | exit 0, `relocated` holds `docs/research/context-loss.md` to `notes/research/context-loss.md` | the same |
| **ff** | **exit 0, `"relocated": []`**, the knob file written, no rename staged | **the same** |

### `jigc task finalize` — rig `fresh --start single-task`, one staged `code.txt`, one untracked `keep.txt`

| state | exit (both binaries) | `HEAD` | working area | the line on standard error |
|---|---|---|---|---|
| none | 0 | moved; commit holds `code.txt` | gone | — |
| ascii | 0 | moved; commit holds `badXname.txt`, `code.txt` | gone | — |
| **ff** | **1** | same | standing | `git checkout-index -a --prefix=<tmp>/` failed: `error: unable to create file <tmp>/bad?name.txt: Illegal byte sequence` |
| asciiq | 0 | moved | gone | — |
| **ffq** | **1** | same | standing | `git status` produced non-UTF-8 output |
| **latin1** | **1** | same | standing | `git` produced non-UTF-8 output: invalid utf-8 sequence of 1 bytes from index 256 |

`keep.txt` hashes the same blob after every one of the twelve cells. The `ff` refusal is this
filesystem's, not this function's: the door materialises the index into a scratch tree and the
name cannot be created here. The `ffq` refusal comes from another capture helper, which runs
first. **The `latin1` line is `git_capture`'s own** — its context string, at `task.rs:3755` —
so that site was reached with an undecodable capture, and it refused before anything was
written (see *Left open*, item 1).

### `jigc task finalize`, amend arm — rig `fresh`, then `jigc task amend`

| state | exit (both) | `HEAD` | what it said |
|---|---|---|---|
| none | 0 | rewritten | — |
| ascii | 3 | same | `finalize.amend-index-dirty`, naming `badXname.txt` |
| ff | 3 | same | `finalize.amend-index-dirty`, naming `"bad\377name.txt"` |
| hc-ascii · hc-asciiq | 0 | rewritten | — |
| hc-ff | 1 | same | the `git checkout-index` line above |
| hc-ffq | 1 | same | `git status` produced non-UTF-8 output |

An amend over a staged entry is refused by design in the control too. `task.rs:3026` decoded
in every cell (the name arrives C-quoted).

### `jigc start --task <id>`, then `jigc task finalize` — rig `fresh --start single-task`

| state | `start --task` exit (both) | `finalize` exit (both) | what it said |
|---|---|---|---|
| ascii · ff | 0 · 0 | not run | the composed workflow |
| hc-ascii · hc-asciiq | 1 | 3 | `finalize.base-mismatch`: the moved history overlaps the task's work on `badXname.txt` |
| hc-ff | 1 | 3 | the same, on `"bad\377name.txt"` |
| **hc-ffq** | **1** | **1** | `git` produced non-UTF-8 output: invalid utf-8 sequence of 1 bytes from index 3 |

`hc-ffq` is `task.rs:8362` reached with an undecodable capture: both doors exit 1, `HEAD` is
unmoved, the working area stands. The `hc-` controls are refused as well — on this host the
committed entry has no file, which the door reads as the task's own work — so these cells show
the site refusing, and do not show the door working beside such a name.

### `jigc rename spec:padding --to "Padding Helper Two"` — rig `vendored`, plus a committed `NOTES.txt` that mentions the slug

| state | exit (both) | `HEAD` | the advisory list |
|---|---|---|---|
| none | 0 | moved | names `NOTES.txt:1` and `docs/architecture/padding-layer.md:9` |
| ascii · ff · asciiq | 1 | same | — `rename.dirty-tree`, naming the staged entry |
| ffq | 1 | same | — `git status` produced non-UTF-8 output |
| sw-ascii · sw-ff · sw-asciiq | 0 | moved | the same two lines as `none` |
| **sw-ffq** | **0** | **moved** | **absent — no advisory line at all** |

`sw-ffq` is `rename.rs:1303` reached with an undecodable capture, at a site that swallows the
error. The commit it lands is the control's: one path, the rename of
`docs/specs/padding.md` to `docs/specs/padding-helper-two.md`, `git status` empty afterwards.
What is lost is the advisory — two mentions of the old slug are not reported.

### `jigc ingest` — rig `committed-singletons`, its file-state record moved out first

| state | exit (both) | file-state record afterwards | rows adopted |
|---|---|---|---|
| none · ascii · ff | 0 | written | 4 |
| asciimd, a file on disk and staged | 0 | written | 4 |
| asciimd, index only | 1 | absent | 0 — *could not read the candidate at … badXname.md: No such file or directory* |
| **ffmd** | **1** | **absent** | **0** — *could not enumerate the `.md` candidate set via `git ls-files`: `git` produced non-UTF-8 output* |

`ffmd` is `ingest.rs:312` reached with an undecodable capture: exit 1, nothing adopted, nothing
written.

### `jigc uninstall` — rig `fresh`

| state | exit (both) | `.jigc/` | what it said |
|---|---|---|---|
| none · ascii · ff · asciij | 0 | gone | the teardown, naming the tracked files it takes |
| **ffj** | **1** | **standing** | `uninstall.untracked-workbench-file` — *cannot check `.jigc/` for files no index has a copy of, so removing it could destroy them: `git` produced non-UTF-8 output* |
| none, plus an untracked `.jigc/config/note.txt` | 1 | standing | the same code, naming `note.txt` |
| asciij, plus that file | 1 | standing | the same, naming `note.txt` |
| **ffj, plus that file** | **1** | **standing** | the *cannot check* refusal; `note.txt` stands |
| asciij, that file, `--force` | 0 | gone | two warning blocks on standard error: the 1 file no index has a copy of, by name, and the tracked files |
| **ffj, that file, `--force`** | **0** | **gone** | **standard error empty** — neither warning block |

`ffj` is `setup.rs:5794` reached with an undecodable capture. Through the guard the door
**refuses before removing**. Under `--force` — which the help text calls *the explicit consent
to destroy work no commit has a copy of* — the removal is the control's and the two warning
blocks that name what went are missing (see *Left open*, item 3).

### The `jigc milestone` verbs — rig `fresh`

| door, state | exit (both) | `HEAD` | what it said |
|---|---|---|---|
| `milestone finalize`: none · ascii · asciiq | 0 | moved; commit holds `code.txt` and the record | — |
| `milestone finalize`: ff · ffq (plant in the main index) | 1 | same; the worktree stands | `git diff --cached` produced non-UTF-8 output |
| `milestone finalize`: wt-ascii · wt-asciiq | 0 | moved; commit also holds `badXname.txt` | — |
| `milestone finalize`: wt-ff | 1 | same; the worktree stands | candidate: `git diff --name-status` produced non-UTF-8 · previous release: `git read-tree --reset -u <sha>` failed, *Illegal byte sequence* |
| `milestone finalize`: wt-ffq | 1 | same; the worktree stands | `git diff --name-status` produced non-UTF-8 (both) |
| `milestone create`, `add-task`, a sub-task's `task discard`, `milestone discard`: none · ascii | 0 · 0 · 0 · 0 | four record commits | — |
| the same four, ff | 1 at `create`, then 1 at each for want of the milestone | no commit | `git diff --cached` produced non-UTF-8 output |

Every refusal here is another capture helper's (see *Left open*, item 6), which runs before any
`git_capture` site of class C. No `git_capture` site of these doors was reached with an
undecodable capture.

### Candidate against previous release

Exit status, whether `HEAD` moved, the landed commit's file list at the door and what stands
afterwards agree in all 58 cells. Standard output and standard error, with shas and paths
normalised, differ in 6 of the 332 stream files compared, in two ways only: five uninstall
narrations count one more tracked file on the candidate (its install tracks
`.jigc/settings-entries.json`), and the `wt-ff` refusal's reason is the one in the table above.

## Is it what the finding says?

The finding says the other callers share the UTF-8 step, and by its clause that a door of
theirs may lose files or write wrongly over it. **The first half is so; the second did not
happen at any site I reached.** To the question triage put — *does a capture that cannot be
decoded read as empty and let the door write, commit or destroy at exit 0, or does the door
refuse before writing*:

| site reached with an undecodable capture | how | the door |
|---|---|---|
| `task.rs:3755` — `task finalize` | latin1 | **refuses**, exit 1, nothing written |
| `task.rs:8362` — `start --task`, `task finalize` | hc-ffq | **refuses**, exit 1, nothing written |
| `ingest.rs:312` — `ingest` | ffmd | **refuses**, exit 1, nothing written |
| `setup.rs:5794` — `uninstall`, the guard | ffj | **refuses**, exit 1, nothing removed |
| `setup.rs:5794` — `uninstall --force`, the narration | ffj | reads as empty; removes what `--force` consents to, as the control does; names none of it |
| `rename.rs:1303` — `rename` | sw-ffq | reads as empty; lands the control's commit; prints no advisory |
| `orphan.rs:149` — `config set docs-root` | ff | reads as empty; writes the knob, moves nothing — **the other row's** |

**Read against the design that owns each.** `design/write-commands.md` → `jigc rename` →
*Boundary* owns the report: it is *advisory (it never blocks and never rewrites)*, and the
function's own comment says a git failure is skipped *because the report must never change the
verb's exit status*. That is a design for the report not blocking; it is not a design for the
report vanishing while git exited 0, so the empty list is a defect and not intended behaviour.
`jigc uninstall`'s guard is documented as failing closed (`setup.rs`, the doc comment of
`untracked_workbench_files`), and it does. The one declared bound about a non-UTF-8 name,
`DECISIONS.md` → *2026-09-17 — M52 Increment 4 / T3* — *a non-UTF-8 entry name is narrated, not
kept* — is about a **file in a task's working area** whose name the complement derivation
reads through a lossy conversion; it says nothing of git's output or of an index entry, and no
cell here reaches it (this filesystem cannot hold such a file). Nothing found says a refusal
over undecodable git output is intended or unintended; nothing here is contested.

## Does it break `no-lost-files`, inside that clause's scope?

No. **Basis: `breaks-no-clause`.**

- **The clause and its scope.** `DECISIONS.md` → *2026-10-04 — The exit rule, revised*, first
  sharpening: *in a healthy repository used as documented — which includes ordinary git
  configuration … — no jigc command at exit 0 destroys bytes no git object holds or commits
  content the user did not ask for. Where jigc cannot tell (git fails, the index is unreadable)
  it refuses before writing.* The run's opening names it `no-lost-files`.
- **No cell destroys such bytes at exit 0 because of an undecodable capture.** The only
  exit-0 removal over one is `uninstall --force`, and it removes exactly what the ASCII control
  removes under the same flag, which is the door's documented consent.
- **No cell commits content nobody asked for.** The only exit-0 commit over one is
  `jigc rename`'s, and its commit is the control's, path for path.
- **Where these sites cannot tell, they refuse before writing** — at four of the five sites
  reached here, with `HEAD` unmoved, the working area or `.jigc/` standing and the untracked
  file intact. The two callers that swallow the error instead are an advisory and a narration:
  neither decides what is written, committed or removed.
- This holds without leaning on the scope's exclusions. I did not need to decide whether a
  tracked name that is not UTF-8 is a *healthy repository* or a *planted state*, nor whether
  `core.quotePath=false` is *ordinary git configuration*: the cells break nothing under the
  reading least favourable to the binary.

**What this verdict does not say.** It does not grade `orphan.rs:149`. It does not say these
doors *work* beside such a name — several refuse where a user would expect a result, and that
is the second clause's question, not this one's (*Left open*, items 1 and 6). And it rests on
the sites reached: see *Class* and *Bounds*.

## The regression fact

**Not owed by this verdict**: step 4 runs with `confirmed` only. Both binaries were driven
through every cell because triage asked for it, and what that showed is under *Candidate
against previous release*: no cell is green on one and red on the other.

## Class

**Enumerated: the callers of `task::git_capture` — 34 production call sites, by the grep and
the table above; the count is derived from that grep.** Of the 34, 6 were reached with an
undecodable capture (the table under *Is it what the finding says?* — seven rows, because
`setup.rs:5794` is reached through its two callers). The other 28: 12 are class A and cannot
receive undecodable output from any path name; 5 are class B and were not driven with an
undecodable commit message; 11 are class C sites not reached raw — 5 scoped to jigc's own
paths (`task.rs:3762`, `6270`, `6397`, `6678`, `6740`); `task.rs:3026`, which decoded in every
amend cell and was not driven raw (in the one amend cell under `core.quotePath=false` another
helper refused first); 4 after a commit that this host never lets land with the name in the
index (`task.rs:9793`, `9809`, `milestone.rs:9475`, `9619`); and `milestone.rs:1820`, behind
the milestone door's earlier refusal.

**`instance, unbounded` beyond that function.** The same UTF-8 step lives in other capture
helpers that are not `git_capture` — the refusals quoted above as *`git status` produced…*,
*`git diff --cached` produced…* and *`git diff --name-status` produced…* come from them, and
`setup.rs` has `git_capture_untrimmed`. I enumerated none of them and give no count.

## Coverage

The finding makes no coverage claim, and none is verified here.

## Repro VR-GC-1

The two cells that state a fact this verdict wants to stay true: an undecodable capture at a
scoped `-z` site makes its door refuse before it writes or removes.

```yaml
claim: "the other callers of task::git_capture share the UTF-8 step, so a door of theirs writes, commits or destroys at exit 0 over a capture it could not decode — breaking no-lost-files"
verdict: "REFUTED as a blocker — breaks-no-clause. The shared step reproduces; at the sites reached the door refuses before writing, or the swallowed error decides nothing that is written, committed or removed."
binary: "candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc, commit eeffe347; the previous release, sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d, gives the same exits"
plant:                      # needs no file on disk; <name> is passed as raw bytes
  - "blob=$(printf 'x\n' | git hash-object -w --stdin)"
  - ["git", "update-index", "--add", "--cacheinfo", "100644,<blob>,<name>"]   # exit 0
doors:
  - door: "jigc ingest"                 # ingest.rs:312
    setup:
      - fixture: committed-singletons   # dev/jigc-rig committed-singletons --binary <binary>
      - "move .jigc/state/file-state.json out of the repository"
      - plant: "bad<byte 0xFF>name.md"
    repro:
      - ["jigc", "ingest"]
    expect:
      exit: 1
      stdout: ""
      stderr_contains: "could not enumerate the `.md` candidate set via `git ls-files`"
      file_absent: ".jigc/state/file-state.json"
    control: "with no plant, or with a staged badXname.md that is a file on disk: exit 0, four rows adopted, the record written"
  - door: "jigc uninstall"              # setup.rs:5794, through the guard at setup.rs:5854
    setup:
      - fixture: fresh                  # dev/jigc-rig fresh --binary <binary>
      - plant: ".jigc/bad<byte 0xFF>name.txt"
    repro:
      - ["jigc", "uninstall"]
    expect:
      exit: 1
      stderr_contains: "uninstall.untracked-workbench-file"
      dir_present: ".jigc"
    control: "with no plant, or with the plant under the name .jigc/badXname.txt: exit 0 and .jigc/ is gone"
observed: "<scratch>/verify-gc.TXNMKK — cand.log and prev.log (every command, exit, stdout, stderr), cand.facts and prev.facts, runs/<binary>/<door>.<state>/"
pinned-by: "UNPINNED: a verifier writes no test — the two cells convert as they stand, Unix-only"
```

**Pinnable as it stands: yes, these two cells.** The plant is two git plumbing calls and needs
no file on disk, so the refusal does not depend on what the filesystem can hold; the name has
to be passed as raw bytes, so the test is Unix-only.

### Observed, and not to be pinned green

Three cells hold a defect as observed; pinned green they would hold it in place. Each is a
fix's red test in waiting, with its `expect` inverted, if triage schedules one.

```yaml
- door: "jigc rename"                   # rename.rs:1303
  setup:
    - fixture: vendored
    - "write NOTES.txt = `see padding for the helper`; git add; git commit --no-verify"
    - plant: "bad<byte 0xFF>name.txt"
    - ["git", "commit", "-q", "--no-verify", "-m", "foreign"]
    - ["git", "update-index", "--skip-worktree", "--", "bad<byte 0xFF>name.txt"]
    - ["git", "config", "core.quotePath", "false"]
  repro:
    - ["jigc", "rename", "spec:padding", "--to", "Padding Helper Two"]
  observed: { exit: 0, stdout_lacks: "prose/unmanaged mention(s)" }
  control: "the same with badXname.txt: exit 0 and `2 prose/unmanaged mention(s) of spec:padding remain`, naming NOTES.txt:1 and docs/architecture/padding-layer.md:9"
- door: "jigc uninstall --force"        # setup.rs:5794, through the narration at setup.rs:6172
  setup:
    - fixture: fresh
    - "write .jigc/config/note.txt, untracked"
    - plant: ".jigc/bad<byte 0xFF>name.txt"
  repro:
    - ["jigc", "uninstall", "--force"]
  observed: { exit: 0, stderr: "", dir_absent: ".jigc" }
  control: "the same with .jigc/badXname.txt: exit 0 and standard error names .jigc/config/note.txt as not recoverable"
- door: "jigc task finalize"            # task.rs:3755
  setup:
    - fixture: fresh
    - ["jigc", "start", "--workflow", "single-task", "probe the finalize door"]
    - "write legacy.txt = the bytes `caf`, 0xE9, ` au lait`, newline; git add legacy.txt"
    - "fill the commit doc: set-field #header/type = chore; set-slot #summary"
  repro:
    - ["jigc", "task", "finalize", "<task>"]
  observed: { exit: 1, stderr_contains: "`git` produced non-UTF-8 output", head: "unmoved" }
  control: "without legacy.txt: exit 0 and the commit lands"
```

## Left open

1. **`jigc task finalize` refuses a task that stages a text file holding a byte that is not
   UTF-8** — exit 1, `git` produced non-UTF-8 output, on both binaries (cell `latin1`). One
   byte 0xE9 in a 13-byte file, default git configuration, nothing planted by plumbing. It
   loses nothing; it is a door that does not run, which is the second clause's question
   (`working-product`), and it was not graded. Not pursued beyond the one cell: the doc-only,
   amend and migration arms do not take `task.rs:3755` by reading, and were not driven.
2. **`jigc rename` prints no advisory when its listing cannot be decoded** (cell `sw-ffq`,
   both binaries): two mentions of the old slug go unreported at exit 0. And, read and not
   driven: the same scan uses each `git ls-files` line as a path, so under default quoting a
   tracked file whose name git C-quotes is never opened and a mention inside it is never
   reported.
3. **`jigc uninstall --force` names nothing it takes when that listing cannot be decoded**
   (cell `ffj`, that file, `--force`; both binaries): the untracked file is destroyed under
   consent and neither warning block is printed. The refusal that precedes it without
   `--force` routes to *make sure `git` is on PATH and the `.jigc/` tree is readable*; git was
   on `PATH` and the tree was readable. The route was not run.
4. **A printed route that spells a C-quoted name as a path.** The amend refusal for the
   0xFF entry prints *unstage it (`git -C <repo> restore --staged -- '"bad\377name.txt"'`)*.
   Not run.
5. **`orphan.rs:149`'s three writing doors against `no-lost-files` — not graded here.** They
   are row `r1-non-utf8-path-other-orphan-walk-consumers`'s, and that report's own first open
   item. One fact added for whoever grades it: `jigc config set docs-root notes --format json`
   in the `ff` state gives the same exit 0, the same `"relocated": []` and the same written
   knob file on the previous release.
6. **Other capture helpers refuse where the ASCII control works, under default git
   configuration.** With the entry of `Repro VR-LD-2` unchanged: `jigc milestone create` exits
   1 (control 0), `jigc milestone finalize` exits 1 (control 0), both on *`git diff --cached`
   produced non-UTF-8 output*; both binaries. None is a `git_capture` site, none loses
   anything, none was enumerated.
7. **A host whose filesystem holds the name.** There `task finalize` would not stop at the
   index materialisation, and the four class-C sites that run after a commit could be reached
   under `core.quotePath=false`. Each propagates its error, so what is open is a door exiting
   non-zero *after* its commit landed. Read, not driven: one macOS host.
8. **A commit message that is not UTF-8** — the 5 class-B sites. Not driven.

## Bounds — what this verification did not do

- One macOS host, one git. This filesystem refuses the name, so every `ff` state is an index
  entry with no file, and two refusals in the tables are the filesystem's.
- `sw-` uses a flag a user of a host that can hold the name would not need; it stands in for
  that host's clean tree, and is not that host.
- One corpus per door and one value per argument. `jigc task finalize` was driven on its
  ordinary and amend arms only; `jigc milestone finalize` on one sub-task with one staged file.
- The callers of `git_head` were classed by its argv and not traced to their doors.
- `--format json` was driven at the `docs-root` door only.
- It read `DECISIONS.md`'s two entries named above, `design/write-commands.md` → `jigc rename`,
  `design/project-setup.md` on the ingest candidate set, and the source at each site. It read
  the one report it was handed, and no other finding's or verifier's.

## Where the evidence is

- `<scratch>/verify-gc.TXNMKK/scripts/drive.sh` — one cell: rig, plant, door, read-back.
  `matrix.sh` — the 53 cells of the first pass; the five added afterwards (`ren/sw-*`,
  `ing/asciimdfile`) were run by the same driver, by hand. `summ.py` — the side-by-side read.
- `<scratch>/verify-gc.TXNMKK/cand.log`, `prev.log` — every command, its exit status, its
  standard output and standard error, in order. `cand.facts`, `prev.facts` — one line per fact.
- `<scratch>/verify-gc.TXNMKK/runs/cand/`, `runs/prev/` — one directory per cell: a file per
  stream per step, and the rig itself.
- `<scratch>/verify-gc.TXNMKK/try.log` and the `explore-*` files — the driver's trial run and
  the exploration that fixed each door's command line; nothing in the tables is read from them.

<!-- end of report -->
