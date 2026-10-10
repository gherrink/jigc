# verify-real — `r1-p3-validate-unreadable-orphan-platforms-and-variants`

Run `canary-one`, round 1, stage `test`, attempt 1. One finding, handed over by triage with the
grade *unclear*: door `jigc validate`, clause said to be broken `working-product`. It has no
block of its own: it is the *Bounds* section of
`completions/artifacts/canary-one/r1/reports/test/verify-p2-r1-validate-unreadable-orphan-variant-not-driven.a1.md`,
which says that `Repro RC-9u` was driven with one kind of *unreadable* (the file's mode), by one
uid, on one platform. Triage asks for that block again with the file unreadable by an ACL, by
ownership and by an unreadable parent directory; as root; and on Linux where available. I read
that one report, which the prompt hands me as the finding's source, and no other finding's
report and no other verifier's.

## Verdict

**`refuted` as a blocker — basis `breaks-no-clause` — and it covers what was driven, which is
two of the five things triage asked for.** It is **not** `does-not-reproduce`: on every variant
I could plant, the false green of `Repro RC-9u` reproduces, byte for byte, on the candidate and
on the previous release. It stays a row of the ledger.

- **Driven, and reproducing (macOS, uid 501, both binaries).** A stamped committed file no
  doctype claims, made unreadable by **an ACL on the file** (`deny read`), by **its parent
  directory's mode** (000, and 600 — listable, not searchable) and by **an ACL on the parent
  directory** (`deny search`): `jigc validate` exits 0 and prints *no findings — the committed
  store validates clean*, stderr empty; `--format json` exits 0 with `"findings": []`. Each is
  `cmp`-identical to the mode-000 cell of `Repro RC-9u`, which I drove again beside them as the
  anchor. Each plant removed, the exit 1 and the blocking `schema-conformance.orphaned-instance`
  are back, `cmp`-identical to the control.
- **NOT driven: ownership, root, Linux.** Each for a reason that is not the finding's:
  - *ownership* — a file of another uid inside a directory of mine cannot be made without
    privilege elevation on the host, and I attempted none: nothing granted it to this role, and
    a computed task text is not that grant;
  - *as root* — the same reason;
  - *Linux* — both binaries the prompt hands me are Mach-O arm64 executables, the prompt hands
    no trial image, and I build nothing.
  **Nothing below is a statement about those three.** They are under *Left open*, and the basis
  line of my return names them, so that `refuted` is not read as covering them.
- **Why it breaks no clause, as far as driven.** `working-product` is the second clause of the
  exit rule of 2026-10-04 (`DECISIONS.md` → *The exit rule, revised*), and that entry gives it
  one instrument: *no command that works on rc.24 in a supported layout stops working, and every
  refusal's route works as printed*. First limb: 67 output files of 67 are identical between the
  candidate and the previous release, across fifteen cells — nothing stopped working. Second
  limb: the false-green cells print no refusal, so no printed route fails; the one cell that
  does refuse (below, the docs root itself unreadable) prints no route either, and does so
  identically on both binaries.
- **What a reader could object to, and it is the same thing as before.** The clause is read
  here by its instrument's two limbs. If *rely on* is read to reach a false green that the
  previous release prints too, this row re-grades — on the driven variants as much as on the
  mode-000 one. That reading is the human's. Under *Left open*.
- **Not `intended`.** No ruling I found says an exit-0 clean is the intended answer when a
  tracked file in a managed home cannot be read. `design/validation.md` → the
  `schema-conformance.orphaned-instance` row states the three legs of the condition and no
  posture for an unreadable file; a text search of `design/validation.md`, `design/storage.md`
  and `design/reconciliation.md` for *unreadable*, *cannot be read* and *permission denied*
  found nothing about this sweep. The one statement is a source comment on the stamp reader
  (`crates/cli/src/orphan.rs`, `carries_stamp`), about an **absent** file. The opening record
  declares no bound.
- **`contested`: false.** The finding does not argue that a settled decision is wrong.
- **Regression fact.** Not owed with `refuted`. The previous release was driven because the
  instrument's first limb cannot be answered without it.

## The binaries, asserted before anything was driven

- `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` printed
  `"content_sha256": "dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc"` — the
  hash the prompt gives for the candidate (label c1, commit
  eeffe347324f83a51d1ae83d5f254e73c3f1ea3a).
- The same call with `bin/previous-91834b5e011d/jigc` printed
  `"content_sha256": "accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d"` — the
  previous release's (1.0.0-rc.24).
- The candidate's directory went first on `PATH`; `command -v jigc` printed
  `<scratch>/bin/c1.a1/jigc`, exit 0, before the first command, and again at the head of the
  candidate's drive.
- No `cargo build`, nothing under `target/`. The clone: `git status --porcelain` read
  `?? completions/artifacts/canary-one/r1/`, `HEAD` eeffe347324f83a51d1ae83d5f254e73c3f1ea3a,
  branch `fix/canary-one`; nothing was edited, staged or committed there.

## How it was driven

Host: macOS 26.6.2, arm64, git 2.54.0 (Apple Git-157), an APFS volume. The caller's uid is 501,
not root. One directory of my own under the scratch root, `<scratch>/verify-p3-variants.EKbEOQ`
(written `<W>`), and under it one fresh root per binary, each from `mktemp -d`:
`<W>/cand.dpW4O0` and `<W>/prev.jJfo9Y`. Nothing of the reporter's or of an earlier verifier's
was reused. Every plant was taken off again before the next one, and at the end of each drive
`git status --porcelain` was empty and the modes and ACLs were the ones the setup left, so
nothing in those roots resists removal.

Environment of every invocation: `HOME=<root>/home` (empty before `setup`),
`GIT_CONFIG_GLOBAL=/dev/null`, `GIT_CONFIG_NOSYSTEM=1`, a synthetic identity in the four
`GIT_AUTHOR_*` / `GIT_COMMITTER_*` variables. Every jigc call: stdin from `/dev/null`, stdout
and stderr to their own files, the exit status read directly — no pipe in front of any status.

**What I changed from `Repro RC-9u` as written.** The setup: nothing — the repository is built
by hand, command for command (`git init`, an empty commit, `jigc setup`, the orphan committed
with `--no-verify`). The plant: the block's one `chmod 000` on the file is kept as the anchor
cell, and each variant replaces it with its own plant. Every plant is followed by the block's
plant-control (`cat docs/zzz/orphan.md` must fail, or the cell says nothing about an unreadable
file) and, once removed, by a restored control. On the previous release every jigc call is the
binary's absolute path, and that binary's directory was first on `PATH` inside that one shell,
so that nothing the previous release might start by a bare name could reach the candidate.

### The cells — candidate (`<W>/cand.dpW4O0`)

`init`, the base commit, `jigc setup`, `git add` and the `--no-verify` commit: exit 0 each;
porcelain empty afterwards. Sizes are bytes of stdout; stderr is 0 bytes unless said.

| cell | the plant | `cat` of the file | `jigc validate` | `--format json` | against |
|---|---|---|---|---|---|
| control | none | - | **exit 1**, 1191 | exit 1, 1243 | - |
| anchor (`RC-9u` as written) | `chmod 000 docs/zzz/orphan.md` | exit 1, `Permission denied` | **exit 0**, 125 | exit 0, 112 | - |
| **ACL on the file** | `chmod +a "user:<caller> deny read" docs/zzz/orphan.md` (mode still `-rw-r--r--`) | exit 1, `Permission denied` | **exit 0**, 125 | exit 0, 112 | `cmp`-identical to the anchor |
| **parent directory, mode 000** | `chmod 000 docs/zzz` | exit 1, `Permission denied` | **exit 0**, 125 | exit 0, 112 | `cmp`-identical to the anchor |
| **parent directory, mode 600** (listable, not searchable) | `chmod 600 docs/zzz` | exit 1, `Permission denied` | **exit 0**, 125 | exit 0, 112 | `cmp`-identical to the anchor |
| **ACL on the parent directory** | `chmod +a "user:<caller> deny search" docs/zzz` | exit 1, `Permission denied` | **exit 0**, 125 | exit 0, 112 | `cmp`-identical to the anchor |
| parent directory, mode 300 (searchable, not listable) | `chmod 300 docs/zzz` | **exit 0** — the file is still readable | **exit 1**, 1191 | exit 1, 1243 | `cmp`-identical to the control |
| the docs root, mode 000 | `chmod 000 docs` | exit 1, `Permission denied` | **exit 1**, stdout 0, stderr 244 | exit 1, stdout 0, stderr 263 | neither — below |
| each plant removed (`chmod 644`, `chmod -N`, `chmod 755`) — seven restored controls | - | - | **exit 1**, 1191 | exit 1, 1243 | `cmp`-identical to the control, all seven, text and JSON |

The false green's stdout, whole, the same bytes in all five exit-0 cells:

```text
no findings — the committed store validates clean
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

Its JSON, whole:

```json
{
  "blocking_probes": [],
  "findings": [],
  "report_only": true,
  "schema_version": 3,
  "scope": "store"
}
```

What git says of the tree in those cells, read with its stderr folded in and its exit status
**not** read: under a mode or an ACL on the file, ` M docs/zzz/orphan.md`; under an unsearchable
parent, `docs/zzz/orphan.md: Permission denied`, with `warning: could not open directory
'docs/zzz/': Permission denied` where the directory cannot be listed either.

**The one cell that refuses.** With the docs root itself at mode 000, `jigc validate` exits 1
with an empty stdout and one line on stderr, and the JSON form carries the same sentence under
`"error"`:

```text
validating the committed store at "<root>/repo": Permission denied (os error 13)
```

It names the repository's root, not the directory that cannot be opened, and it prints no
route. It is a refusal, not a false green, and it is the same on both binaries. Noted under
*Left open*; not pursued.

### Attempts to refute it, each of which failed

| attempt | what was done | result |
|---|---|---|
| stale state | a restored control after every plant | exit 1 seven times, each `cmp`-identical to the first control — the plant is the cause each time, and the store did not drift between cells |
| the plant did not take | `cat` of the file under every plant | exit 1, `Permission denied`, in the six cells that claim an unreadable file |
| an unreadable parent, or merely an unlistable one | mode 300 on the parent: the listing fails, the file opens | exit 1, identical to the control — what makes the green is that the file cannot be opened, not that its directory cannot be enumerated |
| the ACL as such, not the denial | the ACL removed with `chmod -N`, the file's mode never touched | exit 1, identical to the control |
| read through a pipe, or cut | every status read bare; outputs sized with `wc -c`, compared with `cmp` | 1191 / 125 bytes, the sizes the source report gives for the two states |
| another binary | `command -v jigc` at the head of the drive | the candidate |
| behaviour a settled decision intends | the design row, three design docs and the exit rule's entry read | no ruling — see *Not `intended`* above |

### Previous release (`<W>/prev.jJfo9Y`, driven by absolute path)

The same fifteen cells on a fresh root, every jigc call
`<scratch>/bin/previous-91834b5e011d/jigc`. Exit statuses, cell for cell, the candidate's:
control 1 / 1; anchor 0 / 0; ACL on the file 0 / 0; parent 000 0 / 0; parent 600 0 / 0; ACL on
the parent 0 / 0; parent 300 1 / 1; docs root 000 1 / 1 with an empty stdout; all seven
restored controls 1 / 1. `cat` failed in the same six cells and succeeded under mode 300.

`cmp` of every output file against the candidate's — stdout and stderr of the text form and of
the JSON form for fifteen cells, and the seven `cat` messages — after replacing each root's own
path with one placeholder, which only the two stderr files of the refusing cell carry:
**67 files of 67 identical.**

One fact from the repository, read and not driven: `crates/cli/src/orphan.rs` has no change
between the previous release's commit and the candidate's (`git diff --stat 91834b5e..eeffe347`
lists `crates/engine/src/validate.rs` only, and none of its hunks touches
`schema_version_from_front_matter`). The stamp reader is the same code on both sides.

## Does it break the clause, inside the clause's scope

The question an *unclear* grade sends a verifier to answer. For what was driven: no, on the
reading stated in the verdict.

- **The clause is in the closing condition.** `completions/artifacts/canary-one/opening.md`
  names `working-product` as the second clause of the entry of 2026-10-04, with the regression
  set and, in this run, the gate as its instrument.
- **First limb — nothing that works on rc.24 stops working.** Answered by the 67 `cmp`s: on
  every variant I could plant, the candidate does exactly what rc.24 does.
- **Second limb — every refusal's route works as printed.** Five cells print no refusal. The
  sixth prints a refusal with no route. No printed route was there to fail.
- **A fact for whoever routes this row, not a grade of mine.** The opening record names
  `jigc doc list` as the round's one door; this finding's door is `jigc validate`.
- **No other clause was asked of me, and I grade none.** One fact that bears on the first
  clause, because I observed it: `jigc validate` wrote nothing in any cell — porcelain was empty
  at the end of both drives.

## The class

`instance, unbounded`. Driven: one state (one tracked, stamped, unclaimed file in a managed
home), five ways of making it unopenable without privilege (file mode, file ACL, parent mode
000, parent mode 600, parent ACL), one door (`jigc validate`, store scope, text and
`--format json`), one finding code, both binaries, one platform, one uid. I enumerated no
consumers. `orphaned_instances(` has two production call sites by a text search of
`crates/cli/src` — `cli.rs` and `doc.rs` — and I drove the first only; that is a text search of
one function's name, not a derivation.

What the five have in common is a reading of the source, not something a drive can show:
`carries_stamp` reads with `read_to_string(...).ok()`, so a read that fails for any reason is
`None` and the file is filtered out as unstamped. A file of another uid would fail the same
call the same way — **which I say as a reading, and did not drive**.

## The coverage claim

The finding makes none of its own. For the block it re-drives, checked by a text scan and not
by reading every test: fifteen files under `crates/cli/src`, `crates/cli/tests`,
`crates/engine/src` and `tooling-tests` name `orphaned-instance`, `orphaned_instance` or
`ORPHANED_INSTANCE`; one of them, `crates/cli/tests/flow52_acceptance.rs`, sets a file mode or
names `chmod` at all, and none names an ACL. So nothing I found plants an unreadable orphan
under this finding, by any of the variants. A scan of names is what this is.

## Repro RC-9u-variants — the block, in the pipeline's schema

```yaml
claim: "the false green of Repro RC-9u — `jigc validate` exit 0, `no findings — the committed store validates clean`, over a stamped committed file no doctype claims — also appears when the file is unreadable by an ACL, and when its parent directory is unsearchable by mode or by an ACL"
verdict: "REFUTED as a blocker (breaks-no-clause) — the behaviour itself REPRODUCES on every variant driven, on both binaries; ownership, root and Linux NOT driven"
binary: candidate c1, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
also-on-previous-release: "sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d — fifteen cells, 67 output files of 67 identical"
requires: "macOS for the two ACL plants (`chmod +a` / `chmod -N`); any Unix host for the mode plants; a caller that is not root"
env:
  HOME: "<root>/home"
  GIT_CONFIG_GLOBAL: "/dev/null"
  GIT_CONFIG_NOSYSTEM: "1"
setup:
  - ["git", "init", "-q", "repo"]
  - ["git", "-C", "repo", "commit", "-q", "--allow-empty", "-m", "base"]
  - ["jigc", "setup"]                                   # in repo; exit 0
  - "write docs/zzz/orphan.md = `---\nschema-version: 1\n---\n# An orphan nobody claims\n\nbody\n`"
  - ["git", "add", "docs/zzz/orphan.md"]
  - ["git", "commit", "-q", "--no-verify", "-m", "an orphan"]
  - control: ["jigc", "validate"]                       # exit 1; stdout opens `blocking · schema-conformance.orphaned-instance — committed doc `, 1191 bytes; stderr empty
plants:                                                 # one at a time; each followed by plant-control, repro, undo, and the control again
  - acl-file:    { do: ["chmod", "+a", "user:<caller> deny read", "docs/zzz/orphan.md"],   undo: ["chmod", "-N", "docs/zzz/orphan.md"] }
  - dir-000:     { do: ["chmod", "000", "docs/zzz"],                                        undo: ["chmod", "755", "docs/zzz"] }
  - dir-600:     { do: ["chmod", "600", "docs/zzz"],                                        undo: ["chmod", "755", "docs/zzz"] }
  - acl-dir:     { do: ["chmod", "+a", "user:<caller> deny search", "docs/zzz"],            undo: ["chmod", "-N", "docs/zzz"] }
plant-control: ["cat", "docs/zzz/orphan.md"]            # exit 1, `Permission denied` — else the cell is vacuous
repro:
  - ["jigc", "validate"]
  - ["jigc", "validate", "--format", "json"]
expect:                                                 # under each of the four plants
  - exit: 0
    stdout: "no findings — the committed store validates clean\n— jigc · run `jigc start` for orientation; all writes through `jigc`.\n"
    stderr: ""
  - exit: 0
    stdout_json: { "blocking_probes": [], "findings": [], "report_only": true, "schema_version": 3, "scope": "store" }
    stderr: ""
controls:
  - "each plant undone -> `jigc validate` exit 1 and `--format json` exit 1, each stdout identical to the control"
  - "chmod 300 docs/zzz (searchable, not listable; `cat` succeeds) -> exit 1, identical to the control"
  - "chmod 000 docs/zzz/orphan.md (Repro RC-9u as written) -> exit 0, identical to the four plants"
  - "chmod 000 docs -> exit 1, stdout empty, stderr `validating the committed store at \"<root>/repo\": Permission denied (os error 13)`; a refusal with no route, not this claim"
observed: "<W>/cand.dpW4O0/out and <W>/prev.jJfo9Y/out — control, d000, acl, dir000, dir600, dir300, diracl, gp000 and a `-restored` cell for each plant, as .out / .err / .json / .jerr, plus a .cat.err per plant"
pinned-by: "UNPINNED: the `expect` above is the behaviour as it stands, which no ruling has made intended"
```

**Pinnable as it stands: no.** Three reasons, none of them mechanical difficulty with the mode
plants.

1. **The `expect` is the behaviour in question.** A standing test over this block would fence
   the false green in. Once the row is ruled the block is ready either way: fixed, it is the
   fix's red test with `expect` turned to a non-zero exit that names the file; bounded, it pins
   the bound as written.
2. **The ACL plants are one platform's.** `chmod +a` is the macOS spelling; a test that carries
   them is a macOS-only test, and the Linux equivalent was not driven here. The two mode plants
   on the parent directory are portable and are the ones a suite could carry.
3. **Every cell is vacuous as root.** Neither a mode nor an ACL stops a read by uid 0, so a test
   needs the `plant-control` step as a guard and must skip, announced, where the read succeeds.

## Left open

- **Ownership — not driven.** A file of another uid needs privilege elevation on the host; none
  was attempted. The reading of the source above says it would fail the same read; nobody has
  shown it.
- **As root — not driven.** Same reason. What a mode or an ACL does to uid 0 suggests the cell
  is simply the control there; nobody has shown that either.
- **Linux — not driven.** No Linux build of either binary was handed to me, and no trial image.
  A verifier handed one could run the two mode plants on the parent directory unchanged.
- **The reading of the `working-product` clause.** Read by its instrument's two limbs. Whether
  *rely on* reaches a false green that the previous release prints too is the human's to say;
  if it does, this row and the one it came from are not `refuted`.
- **The refusal under an unreadable docs root — hit on the way, not pursued.** `chmod 000 docs`:
  exit 1, `Permission denied (os error 13)` said of the repository's root, the directory that
  failed named nowhere, no route. Identical on both binaries.
- **Two answers to one condition.** A file that cannot be opened is skipped in silence at exit
  0; a docs root that cannot be opened stops the sweep at exit 1. Which of the two the sweep
  means is ruled nowhere I read.
- **`jigc doc list` under these plants.** The other production caller of `orphaned_instances(`,
  and the round's one door. Not driven here.

## Bounds — what this verification did not do

- One platform, one git, one uid, one file system. No ownership variant, nothing as root,
  nothing on Linux — the three under *Left open*, each with its reason.
- No ACL other than `deny read` on the file and `deny search` on the directory; no inherited
  ACL; no immutable flag; no extended-attribute or sandbox denial.
- No other consumer of the stamp reader, no other finding code of the sweep, no other door.
- `git status`'s exit status under the plants was not read; its text was.
- The route the control's finding prints was not followed; the finding makes no claim about it.
- One file of mine outside the scratch root: a list of fifteen file names from the coverage
  scan, written under the system temp directory and removed by its literal path.

<!-- end of report -->
