# verify-real — `r1-p3-discard-uninstall-plants-linux-and-root-not-driven` (run canary-one, round 1, stage test, attempt 1)

Reporter `verify-p3-r1-p3-discard-uninstall-plants-linux-and-root-not-driven`. One finding, handed
over: ledger key `r1-p3-discard-uninstall-plants-linux-and-root-not-driven`, door `jigc task discard`
and `jigc uninstall`, the clause it is said to break `no-lost-files`, triage's grade *unclear*. It has
no block of its own: it is item 6 under `Left open` of
`completions/artifacts/canary-one/r1/reports/test/verify-p2-r1-staged-doc-ids-other-callers-not-driven.a1.md`
(*Linux, and a root caller, were not driven*), and what triage asks is that report's `Repro V-2`
driven as root and on Linux. That report was read because the prompt handed it over as this
finding's repro. No other report and nothing of triage's reasoning was read.

## Status: halted — no verdict

**This finding is neither confirmed nor refuted. It stays unverified.**

The question is what the two doors do under the plants *on Linux* and *for a root caller*. This
verifier had neither, with the binaries it was handed:

- **Linux.** Both handed binaries are macOS images (`file` on each: `Mach-O 64-bit executable
  arm64`). The machine is macOS 26.6.2 (Darwin 25.6.0, arm64). A container runtime is present and
  its server is `linux/aarch64`, which cannot execute either binary. The `BINARY:` line names no
  trial image, so there is no Linux binary whose hash this verifier could assert, and the contract
  is that a verifier drives the binary it is handed and builds none.
- **Root.** The caller is uid 501. `sudo -n true` printed `sudo: a password is required` and exited 1.
  No other way to uid 0 was tried.

A finding that could not be driven is not refuted, and nothing red was observed, so it is not
confirmed either. What could be driven on this platform was driven and is recorded below, because
it narrows what the next attempt has to establish; **none of it is a verdict about Linux or about
root.**

## The binaries, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` printed
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the hash the
  `BINARY:` line gives (candidate, label c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the previous
  release's (1.0.0-rc.24). **The previous release was hashed and not driven**: step 4 runs with a
  `confirmed` verdict only, and there is none.
- The candidate's directory went first on `PATH` in every driver call, and each call stops unless
  `command -v jigc` prints the binary it was handed. It printed `<scratch>/bin/c1.a1/jigc` in all
  sixteen cells.
- No `cargo build`, nothing under `target/`. Every rig:
  `SCRATCH=<W>/rigs dev/jigc-rig --binary <candidate> refs-post-hoc`, stdout captured alone, the
  construction log to its own file, exit status 0 each time.
- The clone: `git status --porcelain` read `?? completions/artifacts/canary-one/r1/` before and
  after, `HEAD` eeffe347 on `fix/canary-one`. Nothing was edited, staged or committed.

## What was driven — the candidate, on macOS, as a non-root caller

One directory of this verifier's own, minted with `mktemp -d` under the scratch root:
`<scratch>/verify-p3-linux-root.wV0gkI` (written `<W>`). **One fresh rig per cell** — sixteen cells,
sixteen rigs, none of them the earlier verifier's. Nothing was torn down.

Environment of every invocation: `HOME` the rig's own, `GIT_CONFIG_GLOBAL=/dev/null`,
`GIT_CONFIG_NOSYSTEM=1`, the working directory the rig's repository, stdin from `/dev/null`, stdout
and stderr to their own files, the exit status read directly. The tree was read before and after
each invocation: one line per entry under `.jigc/` (kind, mode, size, a content hash — or the
link's target, or `UNREADABLE`), `git status --porcelain --untracked-files=all --ignored`, and
`HEAD`. Per-cell files are under `<W>/runs/cand-*/`; the two scripts are `<W>/tools/cell.sh` and
`<W>/tools/snap.sh`.

Task `ground-the-vision-in-research` (written `T`). The plants, each alone, in
`.jigc/tasks/T/docs/`:

- **dangling** — `ln -s /nonexistent/x.md research:dangling.md` (as `Repro V-2` writes it)
- **locked** — a 32-byte regular file `research:locked.md`, then `chmod 000` (`Repro V-2`'s variant;
  the earlier report's file was 30 bytes, this one's text is two bytes longer)
- **readable** — the same 32 bytes under the same name, mode 600. **This plant is this verifier's
  reconstruction and not the finding's**: see the next section.

| cell | plant | invocation | exit | stdout | stderr | `.jigc/`, porcelain | `HEAD` |
|---|---|---|---|---|---|---|---|
| cand-none-discard | none | `jigc task discard T` | 1 | empty | `blocking · task-discard.staged-prose`, two ids (587 bytes) | identical | same |
| cand-none-uninstall | none | `jigc uninstall` | 1 | empty | `blocking · uninstall.staged-prose`, two ids (679 bytes) | identical | same |
| cand-dangling-discard | dangling | `jigc task discard T` | 1 | empty | `blocking · task-discard.foreign-bytes`, naming the link's path, with a route (695 bytes) | identical, the link standing | same |
| cand-dangling-uninstall | dangling | `jigc uninstall` | 1 | empty | `blocking · uninstall.foreign-bytes`, naming the link's path, with a route (615 bytes) | identical | same |
| cand-locked-discard | locked | `jigc task discard T` | 1 | empty | `blocking · task-discard.staged-prose`, three ids, `research:locked` among them (604 bytes) | identical, the file standing at mode 000 | same |
| cand-locked-uninstall | locked | `jigc uninstall` | 1 | empty | `blocking · uninstall.staged-prose`, three ids (696 bytes) | identical | same |
| cand-readable-discard | readable | `jigc task discard T` | 1 | empty | the same refusal, three ids (604 bytes) | identical | same |
| cand-readable-uninstall | readable | `jigc uninstall` | 1 | empty | the same refusal, three ids (696 bytes) | identical | same |
| cand-none-discard-force-json | none | `jigc task discard T --force --format json` | 0 | `"dropped"` holds both ids (190 bytes) | empty | the task area gone, nothing else | same |
| cand-dangling-discard-force-json | dangling | the same | 0 | `"dropped": []` (123 bytes) | a warning naming the link as work not in git (270 bytes) | the task area gone: the link and both staged docs | same |
| cand-locked-discard-force-json | locked | the same | 0 | `"dropped"` holds three ids (213 bytes) | empty | the task area gone, the mode-000 file with it | same |
| cand-readable-discard-force-json | readable | the same | 0 | `"dropped"` holds three ids (213 bytes) | empty | the task area gone | same |
| cand-none-uninstall-force | none | `jigc uninstall --force` | 0 | the removal ledger (417 bytes) | the staged-docs warning, two ids (1498 bytes) | `.jigc/` absent | same |
| cand-dangling-uninstall-force | dangling | the same | 0 | the same ledger (417 bytes) | the working-area warning naming the link; no staged-docs warning (1495 bytes) | `.jigc/` absent | same |
| cand-locked-uninstall-force | locked | the same | 0 | the same ledger (417 bytes) | the staged-docs warning, three ids (1515 bytes) | `.jigc/` absent | same |
| cand-readable-uninstall-force | readable | the same | 0 | the same ledger (417 bytes) | the staged-docs warning, three ids (1515 bytes) | `.jigc/` absent | same |

**So `Repro V-2` reproduces here from nothing, on its own platform**: every exit status and every
byte count the earlier report gives for the candidate is the one measured in these rigs. Both doors
refuse at exit 1 without `--force` under every plant and change nothing; the forced forms remove
at exit 0, which is the consent `design/write-commands.md` → *Abandoning a task* (*`--force` is the
single consent for both*) and `design/project-setup.md` → *Teardown / cleanup* name. That is a
fact about macOS and a non-root caller, and it was already on record.

## The reconstruction for a root caller, and what it does not reach

Triage's hypothesis for root is that the mode-000 variant *binds nobody there*: a root caller can
open and read the file a non-root caller cannot. **What was changed:** the caller stayed uid 501,
and the file was made readable to it instead (mode 600) — the same name, the same bytes, the same
directory. That is the smallest setup this machine allows in which the permission bits do not bind
the caller.

**What it showed:** in all four invocations the readable file's stdout and stderr are
byte-identical to the mode-000 file's (`cmp`, the rig's path normalised — eight comparisons, eight
identical), with the same exit status and the same tree outcome. At these two doors, whether the
caller can read the plant's bytes changed nothing that was measured.

**What it does not show**, and why it is no verdict about root:

- The caller was not uid 0. Anything that depends on the caller's identity and not on what one
  file's mode allows was not exercised: git's own ownership check on a repository another user
  owns, the handling of a repository reached through `sudo`, removal inside a directory the caller
  could not otherwise write.
- A root caller still reads mode `000` from `stat`; this caller read `600`.
- Read from the source, not driven: `grep -rn "geteuid\|getuid\|is_root\|\.uid()" crates/cli/src crates/engine/src`
  prints nothing, and the staged-docs probe (`crates/cli/src/task.rs`, `staged_doc_ids`, line 1215
  on) asks `read_dir` and `metadata` and never opens an entry. That is an argument that a root
  caller meets the same code path. It is not a drive.

## What is on record about Linux

Nothing was driven on Linux, and nothing here stands in for it.

- Read from the source, not driven: `grep -rn "cfg(target_os" crates/cli/src crates/engine/src`
  prints one site, `crates/cli/src/invoke.rs:114`, which picks the image the `doc-code` probe child is
  spawned from (`/proc/self/exe` on Linux). It is not in either door's guards as far as this
  verifier read; that reading was not completed to an end and is not offered as a bound.
- **The coverage half, from the suites and not from the diff.** CI's test legs run on
  `ubuntu-latest` (`.github/workflows/ci.yml`, every `runs-on`), so a suite that planted these
  states at these doors would be the Linux drive. In the seven suites nearest the two doors —
  `destroying_door_sibling_surfaces`, `staged_prose_consent_axis`, `task_lifecycle`, `uninstall`,
  `uninstall_workbench_subject`, `uninstall_worktree_guard`, `subtask_discard_record` — a search for
  `symlink(`, `set_mode(` and `from_mode(` finds no call that plants a link or a mode-000 file
  inside a task's `docs/` directory: the one link (`uninstall_workbench_subject.rs:629`) is at
  `.jigc/state/link`, and every mode call sets an executable bit or locks a hooks directory.
  `destroying_door_sibling_surfaces.rs` names the dangling link in its header (lines 31 to 35) as
  driven, and holds no test that makes one. So within those seven the plants are pinned on no
  platform. **59 suite files name one of the two verbs; the other 52 were not read** — this is a
  count of what was searched, not a bound on the class. No suite was run by this verifier, and CI's
  result for the candidate's commit was not read.

## Scope of what was verified

**Instance, unbounded.** Driven: two doors, each bare and forced, three plants and a no-plant
control, on the candidate only, on macOS, as a non-root caller. Not driven: Linux; a root caller;
the previous release; the milestone door and `orient` (the earlier report's own open items, not
this finding's).

## Repro V-3

The block the next attempt drives. It is `Repro V-2` with the platform and the caller made
explicit, and with what was measured here as the expectation to hold or to break.

```yaml
claim: "on Linux, or for a root caller, with a dangling link or a mode-000 file named as a staged doc alone in a live task's staging area, `jigc task discard <id>` or `jigc uninstall` destroys the task area or the plant at exit 0 without `--force`"
verdict: UNVERIFIED   # halted: neither Linux nor a root caller was available for the handed binaries
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc   # a macOS arm64 image; a Linux drive needs a Linux build of the same commit, with its own hash
platform-driven: "macOS 26.6.2, arm64, git 2.54.0, uid 501 — the expectations below are what was measured there"
platform-owed: ["linux, a non-root caller", "linux, uid 0", "macOS, uid 0"]
setup:
  - fixture: refs-post-hoc                  # live task ground-the-vision-in-research staging commit:<id> and vision:vision
  - ["ln", "-s", "/nonexistent/x.md", ".jigc/tasks/ground-the-vision-in-research/docs/research:dangling.md"]
repro:                                      # each from a fresh fixture: a door that succeeds destroys its subject
  - ["jigc", "task", "discard", "ground-the-vision-in-research"]
  - ["jigc", "uninstall"]
expect:
  - exit: 1
    stdout: ""
    stderr_contains: ["blocking · task-discard.foreign-bytes", ".jigc/tasks/ground-the-vision-in-research/docs/research:dangling.md", "jigc task discard ground-the-vision-in-research --force"]
    tree: "every entry under .jigc/ identical before and after (kind, mode, size, link target, content); porcelain and HEAD unchanged"
  - exit: 1
    stdout: ""
    stderr_contains: ["blocking · uninstall.foreign-bytes", ".jigc/tasks/ground-the-vision-in-research/docs/research:dangling.md", "jigc uninstall --force"]
    tree: "the same"
variants:        # each alone, in place of the `ln`
  - "a regular file research:locked.md, then chmod 000 -> exit 1 at both doors, `task-discard.staged-prose` / `uninstall.staged-prose` naming three ids, `research:locked` among them; tree unchanged, the file still mode 000"
  - "the same file at mode 600 (this verifier's stand-in for a caller the mode does not bind) -> stdout and stderr byte-identical to the mode-000 variant at both doors"
control: "no plant -> exit 1 at both doors, the staged-prose refusals over two ids; tree unchanged"
confirmed-if: "on an owed platform either bare door exits 0, or exits non-zero with any entry under .jigc/ changed"
refuted-if: "on every owed platform both bare doors exit 1 under both plants with the tree unchanged"
root-notes: "create the fixture as the caller that drives it, so that git's ownership check is not what answers; if the fixture is another user's and git refuses, that is the clause's `where jigc cannot tell it refuses before writing` and is recorded as such, not as this finding"
observed: "<W>/runs/cand-*/ (argv, stdout, stderr, exit, snap.before, snap.after, snap.diff, rig.log, command-v)"
pinned-by: "UNPINNED. Within the seven suites named above no test plants a link or a mode-000 file in a task's docs/ directory; the remaining 52 suite files that name either verb were not read."
```

**Pinnable as it stands: yes, for the two bare-door expectations and both variants, on a Unix
target.** The fixture is a named state of the shared builder and every step is an argv or one
filesystem call. Pinned as a suite test it would run on Linux in CI's test legs, which is the
cheapest standing answer to the Linux half. Two conditions: the mode-000 variant must not assume
the caller cannot read the file (the measured behaviour does not depend on it), and a root-caller
case is not a suite's to hold — CI's runner and the local gate are both non-root.

## Halt

- **Root cause.** The finding asks for a drive on Linux and a drive as root. The handed candidate
  and the handed previous release are macOS arm64 images; the `BINARY:` line names no trial image;
  root is not reachable without a password. The block can be run on this platform as written — and
  was — but that is not the question the finding asks, and no reconstruction on macOS reaches
  Linux.
- **Evidence.** `file <scratch>/bin/c1.a1/jigc` → `Mach-O 64-bit executable arm64`; the same for
  `<scratch>/bin/previous-91834b5e011d/jigc`. `sudo -n true` → `sudo: a password is required`, exit 1.
  `id -u` → `501`. `docker info --format '{{.OSType}}/{{.Architecture}}'` → `linux/aarch64`.
- **Tree state.** Branch `fix/canary-one` at eeffe347324f83a51d1ae83d5f254e73c3f1ea3a;
  `git status --porcelain` is `?? completions/artifacts/canary-one/r1/`, as it was at the start. No
  commit of this verifier's; this report is its one file.
- **Recommendation.** Three ways forward, the choice not this verifier's:
  1. *Hand the next verifier a Linux build of the candidate's commit with its hash.* Ruling 11 has
     the preflight build one trial image from the candidate's commit, and a local image tagged
     `jigc-gate:eeffe347324f83a51d1ae83d5f254e73c3f1ea3a` (linux/arm64) is present on this machine. It
     was not on this verifier's `BINARY:` line, the hash of the binary inside it is in nothing this
     verifier was given, and it was **not driven**. A container's default caller is uid 0, so that
     one image would give Linux and root in the same drive; `Repro V-3` is the block.
  2. *Pin the bare-door half as a suite test* and let CI's Linux legs answer the Linux half. That
     leaves the root half open.
  3. *The human rules it* — a platform or a root caller as a declared bound with its reach, or a
     row for a later release. The opening record declares no bound today, so no grade can cite one.

## Left open

Not pursued; each is for triage like any finding.

1. **The finding itself: Linux, and a root caller, at the two doors under the two plants.** Still
   unverified. `Repro V-3` is the block.
2. **The plants at these two doors are pinned by no suite this verifier read**, on any platform
   (seven suites read, 52 not).
3. **`"dropped": []` and the missing staged-docs warning under the dangling link, at the forced
   doors**, re-measured here in fresh rigs (cells cand-dangling-discard-force-json and
   cand-dangling-uninstall-force). It is the earlier report's open item 3, seen again and not a new
   row.

<!-- end of report -->
