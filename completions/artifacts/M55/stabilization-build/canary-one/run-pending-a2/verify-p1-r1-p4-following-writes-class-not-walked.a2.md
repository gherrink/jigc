# verify-real — `r1-p4-following-writes-class-not-walked` (run canary-one, round 1, stage test, attempt 2)

Reporter `verify-p1-r1-p4-following-writes-class-not-walked`. One finding, handed over: ledger key
`r1-p4-following-writes-class-not-walked`, door *unlisted — every write in jigc that follows a
link*, the clause it is said to break `no-lost-files`, triage's grade *unclear*. It has no block of
its own: its source is item 6 of *Left open* in
`completions/artifacts/canary-one/r1/reports/test/verify-p3-r1-p3-milestone-merged-area-writer-follows-a-link.a1.md`
— *the class is not bounded: other following writes in jigc were not walked, and the spawned `git`
processes were not each read as writers into the area*. That report was read because the prompt
hands it over as the finding. No other report was read, and nothing of triage's reasoning beyond the
grade.

## Verdict

**`refuted` as a blocker — basis `breaks-no-clause`.** The class is real and it has members beyond
the one that report drove. It is **not** `does-not-reproduce`, and it stays a row of the ledger.

There was no block to re-drive, so the smallest setups that reach the same door were built: the
write sites of both crates were enumerated from the source, and the ones whose destination git can
deliver a link to were driven with such a link, each on a fresh rig, on the candidate and on the
previous release.

1. **Two more doors write through a link at exit 0, and here git itself delivers the link.**
   - `jigc config fill step:implement#extra-guidance --from-file -`, with a **committed** symbolic
     link at `.jigc/config/fills/extra-guidance.md` pointing at an untracked `notes.md`: exit 0,
     and `notes.md` — 43 bytes no git object holds — is the 27 bytes of the fill afterwards (K1).
     The acknowledgement reads *written to `.jigc/config/`*; nothing was written there, the link
     stands, and nothing on either stream names it. With the link dangling to a path outside the
     repository the file is made there (K1d).
   - `jigc config set invocation-log true`, with a committed link at `.jigc/config/manifest.yaml`
     pointing at the same `notes.md`: exit 0, and `notes.md` is `scalar:` / `invocation-log:
     'true'`, 33 bytes (K4). Dangling: the file is made outside the repository (K4d).
   - The write is `std::fs::write` at `crates/cli/src/config.rs:1913` (the fill) and at `:2203` and
     `:2275` (the manifest's two read-merge-write writers).
2. **Both are the same on `1.0.0-rc.24`**, byte for byte — the config cells of the two logs differ
   in nothing.
3. **Every other git-deliverable destination driven refuses, or merges and keeps the bytes it
   found.** `config fork` and `config insert-step` over a link at the step's path, live or
   dangling, refuse with `config.step-id-collision` and write nothing (K2, K2d, K3, K3d). `jigc
   setup` refuses over a `CLAUDE.md` link whose end is untracked (K5) and over a root `.gitignore`
   link that leads out of the repository (K8) — both of which the previous release wrote through.
   `jigc uninstall` strips its own section through a `CLAUDE.md` link and keeps the user's line
   (K7). A link at an ADR's home is never written through; the boundary blocks at exit 3 (K9). The
   staged-doc writer replaces a link with a regular file and leaves its target alone (K6).
4. **Why it breaks no clause inside its scope.** The first clause's scope, in the closing
   condition's words (DECISIONS.md, 2026-10-04, *The exit rule, revised*, sharpening 1): *in a
   healthy repository used as documented — which includes ordinary git configuration:
   line-ending conversion, `status.showUntrackedFiles`, a symlinked `CLAUDE.md`, linked worktrees
   — no jigc command at exit 0 destroys bytes no git object holds … deliberately planted states
   are declared bounds, written down with their reach.* K1 and K4 are at exit 0 and the bytes are
   ones no git object holds. What takes them out of the scope is the state they need:
   - **Where the link is an arrangement of the fill's or the manifest's own content** — the content
     kept elsewhere and linked in, the shape the scope sentence names for `CLAUDE.md` — the verb
     does what it was asked: `config fill` replaces a fill, and replaces an uncommitted regular one
     the same way (K0, the second call); `config set` merges one key into a mapping.
   - **Bytes the user did not ask to replace are reached only when the link is aimed at a file
     that is not that content.** Nothing found makes such a link but a hand: no production code in
     either crate creates a link (that report's enumeration, re-read here in the scan below), and
     no documented use puts one at a config writer's path. A commit can carry it into a checkout;
     somebody still aimed it. That is the scope sentence's *deliberately planted states*.

**What this verdict does not settle, said plainly.** It rests on reading a link aimed at unrelated
bytes as a planted state **also when git delivered it**. The handed report drew its own line at a
link *in a directory git ignores* and set *links git can deliver* apart; K1 and K4 are on the other
side of that line, and this verdict does not follow it there. **No written bound names either
state**: the run's opening declares none, and the list with reach is still owed
(`implementation/decisions-pending.md` → *The exit rule*). Whether a committed link at one of
jigc's own config paths is inside *healthy repository used as documented* is the human's reading to
give. It is item 1 of *Left open*.

`contested: false` — the finding argues against no settled decision. The settled texts that bear on
it are under *Against the design that owns the behaviour*; none intends a whole-file write through
a link at these two doors.

No `regression` field is returned (the verdict is not `confirmed`). The fact was established
anyway and is in *The same blocks on the previous release*.

## The binaries, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a2/jigc` printed
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the hash
  the `BINARY:` line gives (candidate, label c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the
  previous release's (1.0.0-rc.24).
- With the candidate's directory first on `PATH`, `command -v jigc` printed
  `<scratch>/bin/c1.a2/jigc`. Each driver puts the handed binary's directory first on `PATH` and
  stops (exit 90) unless `command -v jigc` prints the binary it was handed; all four runs passed
  it.
- No `cargo build`, nothing under `target/`. Rigs: `SCRATCH=<dir> dev/jigc-rig --binary <binary>
  <state>`, stdout captured alone, the construction log to its own file, exit 0 each time — one rig
  per scenario, 18 per binary, plus one repository built by hand per binary (K8: `git init`, no
  commit) and one exploration rig on the candidate.
- The clone: `HEAD` 126a8531 on `fix/canary-one`; `git diff --stat eeffe347 HEAD -- crates dev` is
  empty, so the source read here is the candidate's. `git status --porcelain` shows the untracked
  reports of this round's test stage and nothing else, before and after. Nothing was edited,
  staged or committed.

## Platform, and how each cell was measured

**Driven on macOS 26.6.2 (Darwin 25.6.0), git 2.54.0 (Apple Git-157), as a non-root user.** Nothing
was driven on Linux, and nothing as root.

One directory of this verifier's own, minted with `mktemp -d` under the scratch root:
`<scratch>/verify-follow-class.nfb5Mn` (written `<W>`). Under it: `<W>/tools/drive.sh` and
`<W>/tools/drive2.sh`; `<W>/c/` (candidate) and `<W>/p/` (previous release), each holding
`log.txt`, `log2.txt`, `runs/` (one stdout and one stderr file per cell) and `rigs/`. Nothing was
torn down.

Environment of every invocation: `HOME` the rig's own, `GIT_CONFIG_GLOBAL=/dev/null`,
`GIT_CONFIG_NOSYSTEM=1`. Each cell runs with stdin from `/dev/null` or a file, stdout and stderr to
their own files, the exit status read directly and never through a pipe. A *shape* is
`stat -f '%HT|%Sp|%z|->%Y'`, which reads an entry **without following a link**, with the sha256 of
a regular file beside it.

**Two things about the tooling, stated because each is a deviation of a kind.** The two drivers
are scratch files written from the shell with a here-document, not with the file tool; they are in
no repository. And the block could not be *run as written* because the finding has none: what was
changed is everything — the setups below are reconstructions, chosen from the enumeration.

## The enumeration — what the class is, by text

A scan of `crates/engine/src` and `crates/cli/src`, each file cut at its first `#[cfg(test)]`,
comment lines dropped, for `fs::write(`, `File::create(`, `OpenOptions::new(`, `fs::copy(`,
`fs::rename(`, `.write_all(`, `write_atomic`, `regular_file::replace` and `create_new(`:
**67 lines in 17 files.** Read site by site:

| group | sites | follows a link at the last component? | where the destination is |
|---|---|---|---|
| the no-follow writers and their callers | `cli/src/regular_file.rs:168`, `:184`; its callers `adapter.rs:775`, `:1562`, `setup.rs:442`, `:481`, `:1550`, `:6765`; the promote sink `task.rs:5815`; the record's `create_new` at `milestone.rs:947`; `engine/src/store.rs:272` (`rewrite_home`, shape and identity asked) | no | committed paths |
| the atomic writer | `engine/src/state.rs:1328` (a temporary beside the path, then `rename`) and its six callers (`:1050`, `:1103`, `:1153`, `:1319`, `:2273`, `:2362`) | no — the rename replaces the link (driven, K6) | the working area and the caches |
| renames | `doc.rs:3322`, `relocate.rs:926`, `task.rs:7808` | no | the working area, the displacement area |
| **the config layer's writers** | `config.rs:1742`, `:1819`, `:1974` (`steps/<id>.yaml`); **`:1913`** (`fills/<id>.md`); **`:2203`**, **`:2275`** (`manifest.yaml`) | **yes** | `.jigc/config/` — committed, so git can deliver a link |
| the merged-into install members | `adapter.rs:821`, `:1039`, `:1530`; `setup.rs:949`, `:1228`, `:1363`, `:1377` | yes — read, merged, written back | `CLAUDE.md`, `.claude/settings.json`, the `pre-commit` hook, the root `.gitignore` |
| `.jigc/.gitignore` | `gitignore.rs:255` | refused before the write (`ensure`: *it is a symlink*) | committed |
| following writes into directories git ignores | `engine/src/milestone.rs:2323` (the handed report's instance); `engine/src/state.rs:1565` (the save lock), `:1696`, `:1700`, `:1705`, `:1851`, `:2021`, `:2426`; `invocation_log.rs:656`; `milestone.rs:1010`, `:8553`; `rollback.rs:752`, `:810`; `setup.rs:3579`, `:4073`; `task.rs:5371`, `:9312`, `:9591` | yes | `.jigc/tasks/`, `milestones/`, `state/`, `logs/`, `displaced/` — all in `.jigc/.gitignore` |
| scratch trees and the temporary directory | `task.rs:3464`, `:10153`; `rename.rs:1166` | yes | a tree minted for the call; the system temp directory |
| pipes to a child process — no file | `combine.rs:268`, `invoke.rs:230`, `setup.rs:6085`, `task.rs:8627`, `:8831`, `:9410` | — | — |

**The bound of this enumeration.** It is textual and by reading. *After the first `#[cfg(test)]`*
is taken for *in a test module*, not checked brace by brace. A write spelled another way — a
helper that takes the path as an argument and is not named above — is outside it; a second scan
for `io::copy(`, `File::options(`, `set_len(` and a bare `use std::fs` in production code found
the promote sink's and the two no-follow writers' own lines and nothing more. `rollback.rs:752`
restores at a path a door wrote earlier in the same run, which for a few doors is a committed
path; it was read and not driven. **The spawned `git` processes — `apply`, `mv`, `checkout`,
`stash`, `commit` — were not read as writers at all**: that half of the finding is as it was
handed over.

## What was driven — the candidate

Logs `<W>/c/log.txt` and `<W>/c/log2.txt`. 19 scenarios, 29 jigc cells: 19 exit 0, 9 exit 1, 1
exit 3. `notes.md` is an untracked file in the repository's root, 43 bytes, sha256
`c5c99d101776b0f8…`: `user notes - no git object holds this line`. *Committed link* means
`ln -s`, `git add`, `git commit`, after which `git status --porcelain` shows `?? notes.md` alone.
The fill's content is `house style: keep it short` (27 bytes, sha256 `682e42dc7d8a588a…`).

| scenario | the state | what was run | exit | what happened |
|---|---|---|---|---|
| **K0** control | no link | `config fill step:implement#extra-guidance --from-file -`, twice, with different content | 0 · 0 | a regular `fills/extra-guidance.md`, 27 bytes, then 28: the second fill replaces the first. `notes.md` unchanged. `manifest.yaml` holds the same `slot-fill` entry twice |
| **K1** | committed link `.jigc/config/fills/extra-guidance.md` → `../../../notes.md` | the same fill, once | **0** | **`notes.md` is 27 bytes, sha256 `682e42dc7d8a588a…` — the fill; its own line is gone.** The link stands. stdout: `config: filled … — written to .jigc/config/, uncommitted — commit it with your next commit`; stderr empty |
| **K1u** | the same link, not committed | the same | **0** | as K1 |
| **K1d** | committed link at the same path, dangling, to `<rig>/outside/made-elsewhere.md` | the same | **0** | **that file exists**, 27 bytes; `notes.md` unchanged |
| **K2** · **K2d** | committed link at `.jigc/config/steps/implement.yaml` → `notes.md` · dangling to outside | `config fork workflow:single-task#implement` | 1 · 1 | `config.step-id-collision — implement is already forked`; nothing written, nothing made outside |
| **K2c** control | no link | the same | 0 | a regular `steps/implement.yaml`, 2594 bytes |
| **K3** · **K3d** | a committed step source `my-step.yaml`; committed link at `.jigc/config/steps/my-step.yaml` → `notes.md` · dangling to outside | `config insert-step --workflow single-task --after implement my-step.yaml` | 1 · 1 | `config.step-id-collision — my-step is already a step id`; nothing written |
| **K3c** control | no link | the same | 0 | a regular `steps/my-step.yaml`, 2594 bytes |
| **K4** | committed link `.jigc/config/manifest.yaml` → `../../notes.md` | `config set invocation-log true` | **0** | **`notes.md` is 33 bytes, sha256 `cc6aa5737cec6770…`: `scalar:` / `invocation-log: 'true'`.** The link stands |
| **K4r** | no link: `manifest.yaml` a regular untracked file holding the same line | the same | 0 | that file is replaced the same way, 33 bytes — the replacement is the writer's *a manifest that is not a mapping becomes a fresh map*, and needs no link |
| **K4d** | committed link at `manifest.yaml`, dangling to outside | the same | **0** | **the file is made outside**, 33 bytes |
| **K5** | rig `bare`; committed link `CLAUDE.md` → `notes.md` | `jigc setup` | 1 | `setup.dirty-install-path`, naming `notes.md`; nothing installed, `notes.md` unchanged |
| **K6** | a plant in an ignored directory: the staged `adr:alpha-choice.md` moved outside and a link left at its name | `doc set-slot adr:alpha-choice#context --task … --from-file -` | 0 | the staged entry is a **regular file** again, 162 bytes; the file outside is unchanged, 136 bytes |
| **K7** | rig `fresh`; `CLAUDE.md` replaced by a committed link to an untracked `agents.md` holding one line of the user's and jigc's section | `jigc uninstall` | 0 | `agents.md` is 45 bytes — the user's line, kept; jigc's section stripped through the link; the link stands |
| **K8** | a repository with no commit; root `.gitignore` a link to `<rig>/outside/shared-ignore` | `jigc setup` | 1 | `setup.secrets-gitignore — .gitignore is a symbolic link and leads out of this repository`; the file outside unchanged, 0 commits |
| **K9** | committed link `docs/decisions/alpha-choice.md` → `../../notes.md` | `start --workflow single-task …` · `doc create adr --title "Alpha choice" --task …` · three `set-slot` · `task finalize` | 0 · 0 · 1 · 1 · 1 · 3 | the create answers `adr:alpha-choice (already existed — copied in for update)`; each `set-slot` is `write.non-reparseable`; the boundary blocks. `notes.md` unchanged, no commit |
| **K10** | a plant in an ignored directory: `invocation-log` on, then `.jigc/logs/invocations.jsonl` replaced by a link to `notes.md` | `config get invocation-log` | 0 | one record is **appended** to `notes.md` (238 bytes); its own line is kept |

## The same blocks on the previous release

Not the regression step — the verdict is not `confirmed` — but the fact is cheap and triage will
want it. Rigs built with `--binary <scratch>/bin/previous-91834b5e011d/jigc`; the same two
drivers; logs `<W>/p/log.txt` and `<W>/p/log2.txt`. 29 cells: 21 exit 0, 7 exit 1, 1 exit 3.

`diff` of the logs, with the rig names, the abbreviated commit id and the log record's timestamp
and duration normalised:

- **K0 through K4d, K6, K9 and K10 are the same**, every exit, shape, size and hash — the
  invocation log's own hash aside, which carries the clock. **K1, K1u, K1d, K4, K4d: the link is
  written through on `1.0.0-rc.24` in the same way.**
- **K5 differs, in the candidate's favour**: the previous release exits 0, appends its section to
  `notes.md` through the link (82 bytes, the user's line kept) and makes the install commit.
- **K8 differs, in the candidate's favour**: the previous release exits 0 and appends the secrets
  floor to the file outside the repository (192 bytes, its line kept).
- K7 differs only in the count of tracked files the teardown names (the candidate installs one
  more).

## Against the design that owns the behaviour

- **`design/overrides.md` → Authoring deltas, the `config fill` row**: *a `slot-fill` + native fill
  file*, checked at write time for the fill point's presence. It says nothing of the shape of the
  entry at the fill's path, and nothing of a second fill; the command's own help says the content
  *is written to `.jigc/config/fills/<fill-id>.md`*.
- **DECISIONS.md, 2026-10-05, the round-3 entry for `104a7d4b`**: *a file jigc replaces is written
  as a regular file, never through a link* — one writer, `regular_file::replace`, shared with the
  promote sink. Its subject, by the ruling it builds (DECISIONS.md, 2026-10-04, *fork on a symlink
  at `jigc setup`'s replacing writers*), is four install files and the version stamp. `config
  fill` replaces a file whole and is not routed through that writer; the ruling does not name it.
  That same ruling records the symlink cell as *a class member and not graded as its own finding*.
- **The merged-into members' rule**, in the candidate's own refusal (K8): *`jigc setup` merges
  into the file at that path, and it follows a link there only to a regular file inside this
  repository*. `manifest.yaml` is a merged-into file with no such question asked of it — and its
  merge drops what it cannot read as a mapping (K4r).
- **The suite that counts these sites knows them as non-area writes and asks nothing of their
  shape**: `crates/cli/tests/task_area_writer_registry.rs`, `NON_AREA_JOINS`, the `config.rs` row —
  *two `steps/<basename>.yaml` writes, one `fills/<id>.md`, one `steps/<step_id>.yaml`; a config
  dir is no working area*.

None of the four intends a write through a link at the fill's or the manifest's path, so the basis
is not `intended`. None of them puts a link aimed at unrelated bytes inside the first clause's
scope either.

## The coverage claim inside the finding

*Not walked* is a claim about what was driven and what a suite holds. From the enumeration and the
suites, read and not run: 34 files under `crates/cli/tests`, `crates/engine/tests` and
`tooling-tests` call `symlink(`; 8 of them also name a config writer's path (`config/fills`,
`fills/`, `config/steps`, `manifest.yaml`). Read at those lines, **none plants a link at a config
writer's destination**: `step_source_rules.rs` plants one at the step's *source*,
`config_relocation_rollback.rs` and `flow37_rename.rs` at a doc's home, `root_knob_rules.rs` at a
knob's value, `task_area_writer_registry.rs` and `uninstall_workbench_subject.rs` inside the
workbench, `flow53_acceptance.rs` and `path_arg_occurrence_axis.rs` at a path argument. The claim
holds for the two doors found here: nothing tests them.

## Scope of what was verified

**Enumerated: the write sites of both crates** — 67 lines in 17 files, by the scan and with the
bound stated above. That is a count of *sites by text*, not of the mechanism's consumers derived
from a registry; a fixer derives the axis.

**Driven: nine doors** — `config fill`, `config set`, `config fork`, `config insert-step`, `setup`
(two members), `uninstall` (one member), the promote boundary, the staged-doc writer, the
invocation log — on two binaries, on one platform.

**Read, not driven — for each of these the report is `instance, unbounded`:** `config
replace-step`; the `.claude/settings.json` and `pre-commit` members at `setup` and at `uninstall`;
every following write into an ignored directory other than the invocation log and the handed
report's own; `rollback.rs:752`; the scratch trees and the temp-directory message file; **the
spawned `git` processes, not read at all.**

## Repro F-1

```yaml
claim: "other writes in jigc follow a link: a symbolic link at `.jigc/config/fills/<fill-id>.md` is written through by `jigc config fill` at exit 0, and one at `.jigc/config/manifest.yaml` by `jigc config set` — the target's bytes replaced, the acknowledgement naming `.jigc/config/`"
verdict: REFUTED   # as a blocker — basis breaks-no-clause; the behaviour below is what was OBSERVED, and it is a defect
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "the spine and the variants K1u, K1d, K4, K4d, K2, K3, K0 are identical (sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d)"
platform: "macOS 26.6.2, git 2.54.0, a non-root caller; not driven on Linux"
setup:
  - fixture: fresh
  - write: "notes.md in the repository's root, untracked: `user notes - no git object holds this line\n` (43 bytes)"
  - plant: "mkdir .jigc/config/fills; a symbolic link .jigc/config/fills/extra-guidance.md -> ../../../notes.md"
  - ["git", "add", "--", ".jigc/config/fills/extra-guidance.md"]
  - ["git", "commit", "-q", "-m", "a link at the fill's path"]
repro:
  - ["jigc", "config", "fill", "step:implement#extra-guidance", "--from-file", "-"]   # stdin: `house style: keep it short\n`
expect:
  exit: 0
  stdout_contains: "config: filled `step:implement#extra-guidance` — written to `.jigc/config/`, uncommitted"
  stderr: ""
  tree:
    - "notes.md is a regular file of 27 bytes holding `house style: keep it short`; the line it held is gone"
    - ".jigc/config/fills/extra-guidance.md is still the symbolic link"
    - "git status --porcelain: `?? .jigc/config/manifest.yaml` and `?? notes.md`"
variants:
  - "K1u — the link not committed: the same"
  - "K1d — the link dangling, to a path outside the repository: exit 0, a 27-byte file is made there, notes.md unchanged"
  - "K4 — a committed link .jigc/config/manifest.yaml -> ../../notes.md, then [\"jigc\", \"config\", \"set\", \"invocation-log\", \"true\"]: exit 0, notes.md is `scalar:\\n  invocation-log: 'true'\\n` (33 bytes), the link stands"
  - "K4d — that link dangling to outside: exit 0, the 33-byte file is made there"
  - "K4r — no link, manifest.yaml a regular untracked file holding the notes line: exit 0, replaced by the same 33 bytes"
  - "K2 / K2d — a committed link at .jigc/config/steps/implement.yaml, live or dangling, then [\"jigc\", \"config\", \"fork\", \"workflow:single-task#implement\"]: exit 1, config.step-id-collision, nothing written"
  - "K3 / K3d — a committed link at .jigc/config/steps/my-step.yaml, live or dangling, then [\"jigc\", \"config\", \"insert-step\", \"--workflow\", \"single-task\", \"--after\", \"implement\", \"my-step.yaml\"]: exit 1, config.step-id-collision, nothing written"
control: "K0 — no link: exit 0, fills/extra-guidance.md is a regular file of 27 bytes, notes.md unchanged; a second fill with other content replaces it (28 bytes)"
observed: "<W>/c/log.txt with <W>/c/runs/ (K0-K6), <W>/c/log2.txt (K7-K10); the previous release: <W>/p/log.txt, <W>/p/log2.txt"
pinned-by: "UNPINNED: found this round. No suite plants a link at a config writer's destination — read, not run: of the 34 suite files calling symlink(, the 8 that also name a config writer's path plant theirs elsewhere (the step's source, a doc's home, a knob's value, the workbench, a path argument)"
```

**Pinnable as it stands: in part, and not the spine.** The fixture is a named state of the shared
builder and every step but the plant is an argv; the plant is one `std::os::unix::fs::symlink`
call, so the block needs a Unix target. **The control and the variants K2, K2d, K3 and K3d are
true statements about behaviour worth keeping** and convert by hand to a test as they are — the
two refusals over a dangling link are the cells that keep a file from being made outside the
repository. **The spine and K1u, K1d, K4, K4d describe a defect**: converted as written they
would pin the write through the link. They are the red test of a fix, should the row be admitted,
or the reach of a bound, should one be declared. K4r pins a replacement that needs no link and is
its own question (*Left open*, item 3). The comparison with the previous release is not a suite's
to hold.

## Left open

Not pursued; each is for triage like any finding.

1. **Whether a link git delivers is a planted state is not ruled, and this verdict leans on it.**
   K1 and K4 need a committed link at one of jigc's own config paths, aimed at bytes that are not
   that config. The scope sentence excludes *deliberately planted states* as declared bounds
   *written down with their reach*; the run's opening declares none. A reader who holds that a
   state a `git pull` can put in a checkout is not *planted* reads K1 as a break of the first
   clause at exit 0, present on `1.0.0-rc.24` as well and so no regression.
2. **`jigc config fill` and `jigc config set` acknowledge a write into `.jigc/config/` that did not
   happen there** (K1, K4, K1d, K4d): exit 0, the printed sentence names the directory, the bytes
   are at the link's target, and nothing names the link. Separate from where the bytes go.
3. **`jigc config set` replaces a `manifest.yaml` that parses but is not a mapping, whole, at exit
   0 — no link needed** (K4r): 43 bytes no git object held became the 33 bytes of a fresh
   `scalar:` map. The writer says so in a comment (*a present-but-non-mapping manifest becomes a
   fresh map*, `config.rs`, `write_scalar` and its sibling). The same on both binaries.
4. **A second `jigc config fill` of one fill point appends a second, identical `slot-fill` entry
   to `manifest.yaml`** (K0). Seen in the control, on both binaries; what resolution does with the
   pair was not looked at.
5. **`jigc doc create` over a link at a doc's home answers *already existed — copied in for
   update*** and stages the link's target as the doc (K9). It is a read through a link; every
   write after it refused or blocked, and `notes.md` kept its bytes. The same on both binaries.
6. **`jigc uninstall` writes through a `CLAUDE.md` link without the question `jigc setup` asks of
   it** (K7). It strips jigc's own section only and kept the user's line; a link leading out of
   the repository was not driven at this door.
7. **The invocation log is appended through a planted link** (K10) — an append, the target's
   bytes kept, the link in an ignored directory.
8. **Not driven:** Linux; a root caller; `config replace-step`; a link at a directory on the way
   (`.jigc/config/fills` or `.jigc/config` themselves); a link whose target is a tracked file with
   uncommitted changes; the `settings.json` and hook members at both install doors; the
   following writes into ignored directories listed in the enumeration, the invocation log aside;
   `rollback.rs:752`.
9. **The spawned `git` processes were not read as writers** — the second half of the finding as
   handed over, untouched by this report.

<!-- end of report -->
