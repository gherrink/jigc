# verify-real — `r1-p3-display-hook-path-other-callers-not-driven` (run `canary-one`, round 1, stage `test`, attempt 1)

One finding, re-driven from nothing. Door: `jigc setup` and `jigc uninstall`. Clause it is said
to break: `working-product`. Triage's grade: *unclear*. The finding came with no block — it is
the second bullet under *Left open — noticed, not pursued* of the report
`verify-p2-r1-setup-install-hook-route-other-causes-unexamined.a1.md` — and triage asked for one
thing: a `pre-commit` hook that is a link to a script of the repository, and one that is a link
to a sibling, with the path the install's ack names and the path the teardown's *left the
`pre-commit` hook in place* warning tells its reader to delete, on both binaries.

## Verdict in one paragraph

**`refuted` as a blocker, basis `breaks-no-clause`; `contested: false`.** What the finding
suspects is true and reproduces on fresh rigs: both other callers of `display_hook_path` name
the **end of the link**, never the link. It breaks neither measure of the clause. **The
install's ack** names `scripts/pre-commit` (or the sibling) at exit 0, byte-identical on the
candidate and on the previous release, and that is the file the install wrote and — for the
tracked script — the file the install commit carries: the meaning the design gives `hook_file`.
**The teardown's warning** exists on the candidate only, is printed at exit 0, and is no
refusal. Of the two acts it names, *delete the file yourself* done at the printed path removes
the file that holds jigc's lines, which is what the sentence promises; *re-run
`jigc uninstall --force` to remove it whole* exits 0 and removes the **link**, and the file the
warning named is left byte-identical with jigc's lines in it. That second arm is a real defect
of the warning in this state — the path it prints and the thing `--force` removes are two
different entries — and it stays a row of the ledger. No command that works on the previous
release stops working, and no refusal's route is involved.

## The binaries, asserted before anything was driven

| binary | path under the scratch root | `content_sha256` printed by `dev/stabilize-step hash` | matches the prompt's line |
|---|---|---|---|
| candidate, label c1, commit eeffe347 | `bin/c1.a1/jigc` | `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` | yes |
| previous release, 1.0.0-rc.24 | `bin/previous-91834b5e011d/jigc` | `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` | yes |

Both hash calls were the plain call the prompt spells, made before the first command was
driven. Every run put one binary's directory first on `PATH` and stopped unless
`command -v jigc` printed that binary's absolute path; where one rig was driven by both
binaries in turn, the check was repeated at the switch. Nothing was built and nothing under
`target/` was driven. Rigs were built with `dev/jigc-rig bare --binary <that path>`.

## The callers of `display_hook_path`

`command grep -n 'display_hook_path'` over `crates/cli/src` finds the definition
(`setup.rs`, line 960), two mentions in doc comments, and **three** call sites, all in
`crates/cli/src/setup.rs`:

| # | line | caller | where its value is printed |
|---|---|---|---|
| C1 | 1240 | `narrate_kept_precommit` | the teardown's `warning: left the `pre-commit` hook `<shown>` in place` on stderr |
| C2 | 2047 | the install span | `SetupSummary.hook_file`: the text line `pre-commit hook → <shown>` and the `hook_file` key of `--format json` |
| C3 | 3126 | `hook_link_refusal` | the `setup.install-hook` refusal — the reporter's `Repro VR-1`, not this finding's subject |

The function canonicalizes the hook's path before it renders it, and canonicalizing a link
follows it. C1 and C2 are the two callers the finding names. C3 was not re-driven here.

## When C1 is reached at all

The warning is the teardown's `Kept` verdict (`classify_precommit_for_teardown`, line 1121),
and that arm is reached only by a hook with **no end marker** that opens as jigc's standalone
hook and does not hold jigc's block verbatim. The candidate writes the end marker into every
hook, so a hook the candidate installed never reaches it: the state needs a standalone hook
written by a build before 2026-10-06 — the previous release is one — whose jigc lines were then
edited. I produced it with the binaries, never by planting bytes: **the previous release's
`jigc setup`** over the linked hook (it follows the link and writes its marker-less hook into
the link's end), **one edit inside jigc's own lines** (a comment appended to line 3 of the
hook), then **the candidate's `jigc uninstall`**. That is an upgrade path — install on rc.24,
uninstall on the candidate — plus one edit.

For the link's end to be a *standalone* jigc hook it has to hold nothing of the adopter's when
the install runs, so both links lead to an empty executable file: a committed, empty
`scripts/pre-commit`, and an empty `.git/hooks/pre-commit.shared`. With adopter lines there the
install wraps, the block is delimited on both binaries, and the warning is unreachable.

## How it was driven

- `<W>` is a directory of my own, minted with `mktemp -d` under the scratch root:
  `<scratch>/verify-dhp.msnTdN`. **One fresh rig per scenario**, 19 in all (one built to read
  the rig's assignments and driven by nothing), each from `dev/jigc-rig bare` with
  `SCRATCH=<W>`: a git repository with one commit and no `jigc setup`. None is a reporter's.
- Environment of every run: `HOME` the rig's own empty directory, `GIT_CONFIG_NOSYSTEM=1`,
  `GIT_CONFIG_GLOBAL=/dev/null`, the identity the rig sets in the repository's own config. Git
  2.54.0, macOS, an unprivileged user.
- Every exit status was read bare: `jigc <args> > out 2> err`, then `$?`, never through a pipe.
- The driver is `<W>/drive.sh`; its transcripts are the `*.txt` files beside it, named
  `<setup binary><teardown binary>.<link kind>.<edit>[.<act>].txt` with `c` the candidate and
  `p` the previous release. Each rig holds `ev/<n>.out` and `ev/<n>.err` per `jigc` call.
- The two links: `script` — `.git/hooks/pre-commit -> ../../scripts/pre-commit`, the script
  committed first; `sibling` — `.git/hooks/pre-commit -> pre-commit.shared`. A third kind,
  `plain`, is the control with no link.
- **What "the act as printed" was taken to mean.** *Delete the file yourself*: remove what is
  at the path the warning prints between its backticks, read off stderr by the driver.
  *Re-run `jigc uninstall --force`*: that argv, as it stands.

## What was observed

### C2 — the install's ack

| link | binary | `jigc setup` | the text line | `jigc setup --format json` (second run) |
|---|---|---|---|---|
| to the tracked script | candidate (`hjmfxI`) | exit 0, stderr empty, porcelain empty, one install commit | `pre-commit hook → scripts/pre-commit` | exit 0; `"hook_file": "scripts/pre-commit"`, `"hook_committed": true` |
| to the tracked script | previous release (`NC2g7I`) | the same | the same | the same |
| to the sibling | candidate (`HD6Y7K`) | exit 0, stderr empty, porcelain empty, one install commit | `pre-commit hook → .git/hooks/pre-commit.shared`, then the *local to this checkout — git cannot track this path* line | exit 0; `"hook_file": ".git/hooks/pre-commit.shared"`, `"hook_committed": false` |
| to the sibling | previous release (`ArjN9a`) | the same | the same | the same |

In all four the link is still a link afterwards and the file at its end holds the hook: 5129
bytes with one end marker on the candidate, 5106 bytes with none on the previous release. The
stdout of the two binaries differs in the install commit's sha and in nothing else I read.

**Is the ack wrong?** The path it prints is the file that was written, the file the hook's
bytes are in, and — where git can track it — the file the install commit names. A reader who
opens the printed path finds the hook. `hook_committed` is true of the printed path and would
be false of `.git/hooks/pre-commit`. Nothing is told to act on the path.

### C1 — the teardown's warning

Setup for every row: the previous release's `jigc setup` over the link (exit 0), the one-line
edit inside jigc's lines (5106 → 5131 bytes; for the script, committed), then the candidate's
`jigc uninstall`.

| link | rig | `jigc uninstall` | the path the warning names | what is at that path |
|---|---|---|---|---|
| to the tracked script, written relatively | `uiSbE9`, `U2UQ9H`, `D4z505` | exit 0 | `scripts/pre-commit` | a regular file, no link; byte-identical to before the teardown |
| to the tracked script, written absolutely | `srROaT` | exit 0 | `scripts/pre-commit` | the same |
| to the sibling | `7ahLtb`, `DK9IhJ` | exit 0 | `.git/hooks/pre-commit.shared` | the same |
| none — the control | `s2AWnl`, `IPVfAj` | exit 0 | `.git/hooks/pre-commit` | a regular file; byte-identical |

stderr, the script link (the two warnings above it, about the tracked files under `.jigc/` and
about the settings entries, cut):

```text
warning: left the `pre-commit` hook `scripts/pre-commit` in place — it opens as a hook jigc wrote before its block carried an end marker, and is not that hook byte for byte: it was edited inside jigc's own lines, or written from another text, so jigc cannot tell its lines from yours and removed none.
  note: jigc's lines in it print nothing once the install is gone. Delete the file yourself, or re-run `jigc uninstall --force` to remove it whole.
```

The warning's own claims hold of the path it prints: that file was left in place, and jigc's
lines are in it. `.git/hooks/pre-commit`, the link, is not named.

**Act A — delete the file at the printed path** (`U2UQ9H`, `7ahLtb`, control `s2AWnl`):

| link | after the delete | `jigc uninstall` again | a `git commit` with hooks on | `jigc setup` afterwards |
|---|---|---|---|---|
| to the tracked script | jigc's lines are gone with the file; porcelain gains ` D scripts/pre-commit`; `.git/hooks/pre-commit` is still a link and leads to nothing | exit 0, *nothing to remove* | exit 0 | **exit 1**, `setup.install-hook`, *is a symbolic link and leads to nothing*; its route names `<rig>/repo/.git/hooks/pre-commit`, which is the link |
| to the sibling | the same, with no porcelain line | exit 0, *nothing to remove* | exit 0 | **exit 1**, the same refusal; on `7ahLtb` I then removed the link the route names and `jigc setup` exits 0, the hook a regular file at `.git/hooks/pre-commit` |
| none | the hook is gone | exit 0, *nothing to remove* | exit 0 | exit 0 |

So the act does what the sentence says — the file holding jigc's lines is deleted — and leaves
one thing the control does not: a link that leads to nothing, which git passes over and which a
later `jigc setup` refuses once, with a route that is right.

**Act B — re-run `jigc uninstall --force`** (`D4z505`, `srROaT`, `DK9IhJ`, control `IPVfAj`):

| link | `jigc uninstall --force` | `.git/hooks/pre-commit` | the path the warning named |
|---|---|---|---|
| to the tracked script | exit 0, stderr empty, `- removed pre-commit hook` | **absent — the link was removed** | **still there, 5131 bytes, byte-identical (`cmp` exit 0), jigc's sentinel line in it** |
| to the sibling | exit 0, stderr empty, `- removed pre-commit hook` | absent — the link was removed | still there, 5131 bytes, byte-identical, jigc's sentinel line in it |
| none | exit 0, stderr empty, `- removed pre-commit hook` | absent | gone — the two are one path |

This is the one place where the printed path misleads: the warning says the hook
`scripts/pre-commit` was left and that `--force` removes *it* whole; `--force` removes another
entry, and the summary then says the hook was removed while the named file stands as it was.
The hook no longer fires, because the link git would have followed is gone.

### The neighbouring states, for what bounds the warning

| scenario | `jigc uninstall` | what happened to the hook |
|---|---|---|
| previous release's install, **unedited**, candidate's teardown (`cO3pRt` script, `um0a7T` sibling) | exit 0, no warning, `- removed pre-commit hook` | the link removed; the link's end byte-identical, 5106 bytes |
| previous release's install, edited, **previous release's** teardown (`bIsETF`, `ZRTAs2`) | exit 0, no warning, `- removed pre-commit hook` | the link removed; the link's end byte-identical, 5131 bytes |
| candidate's install, edited, candidate's teardown (`ZNetQL`, `0jf6cU`) | exit 0, no warning, `- removed pre-commit hook` | the link removed; the link's end byte-identical, 5154 bytes |

The previous release prints no such warning in any state I drove: it has no `Kept` arm.

## What the record holds

**The ack.** `design/command-output-contract.md` → *Every other envelope*, `jigc setup`: "**`hook_file`** is the pre-commit hook's path", and the M48 declaration: "the path of the
`pre-commit` hook the install wrote … The value is **not** the `.git/hooks` literal but the
hooks dir git resolved". `design/assistant-adapter.md`, the paragraph *A file the install
replaces is written as a regular file at exactly its path, never through a link*: "The
`pre-commit` hook is asked the same way where it is itself a link: followed to a script in the
repository or in its own hooks directory". For the other merged-into members that paragraph has
a `note:` say which file was merged into while the summary key stays the link; it says no such
thing of the hook, whose key is declared as the file the install wrote. The source's own
comment on `install_precommit_hook` gives the reason: "so the caller can name the file it
actually wrote". Naming the link's end in the ack is what those sentences describe.

**The warning.** `design/project-setup.md` → Teardown, *Both hooks must come out*: "A hook with
no end marker is treated as it was: the block is cut where it is there exactly as jigc wrote
it, and otherwise the file is left in place with the message and `--force` removes it." It
speaks of *the file* and says nothing of a hook that is a link. No decision says which entry the
message is to name there, and none says `--force` is to remove the link. The source's comment
on `narrate_kept_precommit` says what kind of surface it is: "it is not a finding because
nothing is wrong and nothing blocks".

**`design/surface-contract.md` → The three laws**, law 1: "Every claim a surface makes is
generated from the thing it describes, or asserted against it". The warning's *re-run
`jigc uninstall --force` to remove it whole* is a claim about the printed path that the run
does not bear out in this state. That is the defect, and it is a design law, not the clause.

**The clause.** `DECISIONS.md` → *2026-10-04 — The exit rule, revised*, the second clause's
measure: "no command that works on rc.24 in a supported layout stops working, and every
refusal's route works as printed".

## Does it break the clause, inside its scope

- **The first measure — holds.** `jigc setup` over either link exits 0 on both binaries.
  `jigc uninstall` exits 0 on both binaries in every state driven, and so does
  `jigc uninstall --force`. Nothing that worked stops.
- **The second measure — not engaged.** Neither surface is a refusal: the ack is the summary of
  a run that succeeded, and the warning is printed beside a teardown that exits 0 and blocks
  nothing. No route of a refusal is printed by either caller. Read generously, as if the
  warning's note were a route: act A reaches the end it names, and act B exits 0 at the first
  attempt and leaves no hook firing — it does not loop, it does not exit non-zero, and it takes
  nothing that is not jigc's or the link.

**Why `refuted`, and why this basis.** *Does not reproduce* would be false: it reproduces, each
state on its own fresh rig, and the report says so. *Intended* covers the ack and not the
warning's second arm, for which no decision speaks. *Breaks no clause* covers both: a real,
small defect in an exit-0 warning, reachable only across an upgrade with an edited hook behind
a link, breaking neither measure. **Why not `contested`.** The finding does not argue that a
settled decision is wrong, and I found none that settles what the warning names.

## The regression fact

Not a field of this return, since the verdict is not `confirmed`. The facts, for whoever grades
the row: the ack is byte-identical on both binaries (the install commit's sha apart); the
warning does not exist on the previous release, which removes the link at exit 0 under
*removed pre-commit hook* in the same state — so the warning's path is *not comparable* there.

## Coverage — verified from the suites, for the blocks' `pinned-by`

The finding makes no coverage claim. By `command grep` over `crates/cli/tests` and
`tooling-tests`:

- The warning's text is asserted in one suite, `uninstall_workbench_subject` (three sites,
  lines 1555, 1584 and 1660). Its hook is a regular file; that file's ten `symlink` hits are
  about cache directories under `.jigc/`. No suite reads the path the warning prints where the
  hook is a link.
- `replacing_writers_never_follow` drives the hook as a link to a tracked `scripts/pre-commit`
  (from line 1393): it asserts exit 0 and that jigc's block is in the script beside the
  adopter's line. It reads neither the ack's path nor `hook_file`, and runs no teardown there.
- A suite that reaches either state by another spelling than `hooks/pre-commit` beside
  `symlink` would have been missed.

## Repro VR-A — `jigc setup` over a linked hook: the ack names the file the install wrote

```yaml
claim: "where .git/hooks/pre-commit is a link to a script of the repository or to a sibling in the hooks directory, `jigc setup`'s ack and its `hook_file` key name the link's end and not the hook — said to break working-product"
verdict: REFUTED          # breaks-no-clause; the behaviour reproduces and is what the design declares
binary: candidate c1, commit eeffe347, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
on-previous-release: "the same exits, the same line and the same keys (sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d)"
setup:
  - env: "HOME=<fresh empty dir>, GIT_CONFIG_NOSYSTEM=1, GIT_CONFIG_GLOBAL=/dev/null"
  - fixture: bare                              # dev/jigc-rig bare: one commit, no `jigc setup`
  - write: "scripts/pre-commit = '' (0 bytes), mode 755"
  - ["git", "add", "scripts/pre-commit"]
  - ["git", "commit", "-q", "-m", "chore: a tracked, empty hook script"]
  - symlink: ".git/hooks/pre-commit -> ../../scripts/pre-commit"
repro:
  - ["jigc", "setup"]                          # run 1
  - ["jigc", "setup", "--format", "json"]      # run 2
expect:
  run_1:
    exit: 0
    stderr: ""
    stdout_contains:
      - "  - pre-commit hook → scripts/pre-commit   (warn-only doc↔code drift backstop)"
    files: "porcelain empty; .git/hooks/pre-commit still a link; scripts/pre-commit holds jigc's block and is in the install commit"
  run_2:
    exit: 0
    stdout_json: { "hook_file": "scripts/pre-commit", "hook_committed": true }
variants:
  - "the link a sibling, `.git/hooks/pre-commit -> pre-commit.shared` (empty, mode 755): exit 0; the line names `.git/hooks/pre-commit.shared`, followed by the `local to this checkout` line; `hook_file` the same path, `hook_committed` false"
observed: "candidate <W>/jigc-rig-bare-hjmfxI and HD6Y7K; previous release NC2g7I and ArjN9a; <W>/c.script.ack.txt, c.sibling.ack.txt, p.script.ack.txt, p.sibling.ack.txt"
pinned-by: "UNPINNED: replacing_writers_never_follow drives this link and asserts exit 0 and the splice, and reads neither the ack's line nor `hook_file`"
```

**Pinnable as it stands:** yes. Argv and three file-system acts, exits and substrings, no
machine-dependent value.

## Repro VR-B — `jigc uninstall` over a linked, marker-less, edited hook: the warning names the link's end

```yaml
claim: "the teardown's `left the pre-commit hook in place` warning names the end of the link and tells its reader to delete the file there — said to break working-product"
verdict: REFUTED          # breaks-no-clause; the path reproduces, and `--force` then removes the link and leaves the named file
binary: candidate c1, commit eeffe347, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
on-previous-release: "not comparable — no such warning: `jigc uninstall` exits 0 with `- removed pre-commit hook`, the link removed and the link's end byte-identical (sha256 accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d)"
setup:
  - env: "HOME=<fresh empty dir>, GIT_CONFIG_NOSYSTEM=1, GIT_CONFIG_GLOBAL=/dev/null"
  - fixture: bare
  - write: "scripts/pre-commit = '' (0 bytes), mode 755"
  - ["git", "add", "scripts/pre-commit"]
  - ["git", "commit", "-q", "-m", "chore: a tracked, empty hook script"]
  - symlink: ".git/hooks/pre-commit -> ../../scripts/pre-commit"
  - previous-release: ["jigc", "setup"]        # exit 0; writes its standalone hook, with no end marker, into scripts/pre-commit
  - edit: "scripts/pre-commit line 3: append ' # tweaked by the adopter'"   # inside jigc's own lines
  - ["git", "add", "scripts/pre-commit"]
  - ["git", "commit", "-q", "--no-verify", "-m", "chore: tweak the hook"]
repro:
  - ["jigc", "uninstall"]                      # run 1, the candidate
  - ["jigc", "uninstall", "--force"]           # run 2, the warning's second arm
expect:
  run_1:
    exit: 0
    stderr_contains:
      - "warning: left the `pre-commit` hook `scripts/pre-commit` in place"
      - "Delete the file yourself, or re-run `jigc uninstall --force` to remove it whole."
    files: "scripts/pre-commit byte-identical; .git/hooks/pre-commit still a link to it"
  run_2:
    exit: 0
    stderr: ""
    stdout_contains:
      - "  - removed pre-commit hook"
    files: ".git/hooks/pre-commit absent; scripts/pre-commit still there, byte-identical, jigc's sentinel line in it"
variants:
  - "instead of run 2, delete the file at the printed path: jigc's lines are gone, the link leads to nothing, `jigc uninstall` exits 0 with nothing to remove, a commit with hooks on exits 0, and `jigc setup` exits 1 at setup.install-hook with a route naming the link; removing that link, `jigc setup` exits 0"
  - "the link a sibling, `.git/hooks/pre-commit -> pre-commit.shared`: the warning names `.git/hooks/pre-commit.shared`; run 2 removes the link and leaves the sibling byte-identical"
  - "the link written absolutely to the same tracked script: the warning still names `scripts/pre-commit`"
  - "no link, the control: the warning names `.git/hooks/pre-commit`; run 2 removes that file; after a delete by hand `jigc setup` exits 0"
  - "the same install unedited: no warning, `- removed pre-commit hook`, the link removed and the link's end byte-identical"
observed: "<W>/jigc-rig-bare-uiSbE9, U2UQ9H, D4z505, srROaT (script); 7ahLtb, DK9IhJ (sibling); s2AWnl, IPVfAj (control); cO3pRt, um0a7T (unedited); bIsETF, ZRTAs2 (previous release's teardown); <W>/pc.*.txt and pp.*.txt"
pinned-by: "UNPINNED: uninstall_workbench_subject asserts the warning's text over a regular hook; no suite reads the path it prints where the hook is a link"
```

**Pinnable as it stands:** yes for run 1 and its variants, with one substitution a test has to
make: the block has the previous release's binary write the marker-less hook, which a suite
cannot call, so the test plants that hook's bytes at the link's end from the literal
`uninstall_workbench_subject` already uses for the same state. **Run 2 is written as observed,
not as wanted:** pinned as it stands it would fence the defect in. Whoever takes the row decides
what `--force` is to do to a linked hook, or what the warning is to name, before run 2 becomes
an assertion; until then it is a record of today's bytes.

## The class

**Consumers of `display_hook_path`: enumerated — three**, by `command grep -n` over
`crates/cli/src` (one file, `setup.rs`, lines 1240, 2047 and 3126). C1 and C2 were driven; C3
is the reporter's block. **States: `instance, unbounded`.** I drove two link ends inside the
repository's reach (a tracked script, a sibling in the hooks directory), one of them written
both relatively and absolutely, and the control with no link. Not driven: a link's end that
holds the adopter's own lines beside a marker-less block; a link to a script the repository
does not track or that git ignores; `core.hooksPath` beside a linked hook; a linked worktree;
`jigc uninstall --format json`.

## Left open — noticed, not pursued

- **`jigc uninstall` over a linked hook that is wholly jigc's removes the link and leaves the
  hook's bytes.** In every state where the teardown says `- removed pre-commit hook` over a
  linked hook — the candidate's own install (`ZNetQL`, `0jf6cU`), the previous release's
  unedited install (`cO3pRt`, `um0a7T`), `--force` (`D4z505`, `DK9IhJ`, `srROaT`), and the
  previous release's own teardown (`bIsETF`, `ZRTAs2`) — what was removed is
  `.git/hooks/pre-commit`, the adopter's link, and the file at its end keeps jigc's whole
  block, byte-identical; for the tracked script that is a committed file with jigc's delimited
  block still in it after the summary says the hook was removed. The same on both binaries.
  This is the mechanism under act B above, and wider than the warning.
- A hook the previous release installed **through a link that leads out of the repository** —
  which it follows, per the reporter's block — then edited and torn down by the candidate: by
  the same function the warning would print the outside file's absolute path under *delete the
  file yourself*. Not driven; triage asked for the two ends inside the repository's reach.
- The warning's sentence *jigc's lines in it print nothing once the install is gone* was not
  put to the test: no commit was made through the kept hook after the teardown.
- After act A on the tracked script, `scripts/pre-commit` is a deletion git shows, and
  restoring it from the index brings jigc's lines back with it. Observed in the porcelain, not
  followed.

## Tree state

The repository clone was read and not written: branch `fix/canary-one` at eeffe347, its status
the one untracked directory `completions/artifacts/canary-one/r1/` that the stage's reports
live in. No commit, no stage, no build. With my file tool I wrote this report and nothing else;
the driver script, its transcripts, two dumps of `--help` text and the rigs under `<W>` were
written by the shell, inside the scratch root.

<!-- end of report -->
