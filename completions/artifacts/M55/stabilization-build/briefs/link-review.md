# Review — `fix/rc24-link-on-the-way` (`0bd8a5b5..63ce9ca5`)

Two commits: `22f06d31` (`dev/runner-faithful` made faithful to the CI runner's git) and `63ce9ca5` (no door writes through a link on the way). This is a read-only review. I edited nothing and ran no gate. I drove a debug binary that I built from `63ce9ca5` in a private target directory, and I ran one suite, `g_migrate link_on_the_way_axis::` (3 passed).

## Verdict

**The deliverable mostly holds, but not fully.** The dispatch guard is a sound chokepoint, and it does what §28 asked at every cell the byte table holds. Every one of the class's 22 recorded cells, plus the two new ones, now comes out *refuses, nothing changed*. No cell's expectation depends on git's version. The byte table's records moved only those cells (point 9 below).

Three claims made in the commit do not hold when driven:

- **"every door of the class"** is false at `uninstall`. Its exception reason says it writes nothing past a link, but over a linked `.claude` it removes and rewrites files outside the repository at exit 0 (B1).
- **"jigc makes no link anywhere in it [the workbench]"** is false. A mover parks a link to a directory into `.jigc/displaced/`, and after that every guarded door refuses over a link that jigc put there (S1).
- **`config set docs-root`** still lands the store's homes past a link at exit 0 when nothing moves. After that, the guard refuses the revert (S2).

There are also two surface problems:

- Bare `jigc start` is what the session hook runs. Over a link it now exits 1 with empty stdout, so a session starts with no orientation and no word about the link (S3).
- A host path is printed for a link in a user's own linked worktree (S4).

## Findings, ranked

### Blocking

**B1 — `jigc uninstall`, exempt from the guard, removes and rewrites files past a linked `.claude` at exit 0.**
- **Location:** `crates/cli/src/cli.rs:785-789` (the exception and its reason); `crates/cli/src/setup.rs:5002` (allowlist / hook / deny edits) and `:5080` (guide removal).
- **Evidence:** I committed `.claude` as a link to a directory outside the repository holding `settings.json` and `skills/jigc/SKILL.md`.
  - `jigc uninstall` exited 0, printed `removed jigc guide artifact`, and deleted the outside `skills/jigc/SKILL.md`.
  - `jigc uninstall --force` also rewrote the outside `settings.json`: its sha changed from `8e60bcfd…` to `4a4057bc…`, and the run printed `removed jigc allowlist permit ← .claude/settings.json`.
- **Why the exception's reason is wrong:** it reasons only about `.jigc` ("a removal takes a link as the entry it is"). The door also edits and removes adapter members under `.claude/`. `setup` refuses exactly this state (`setup.write-guide`), so the pair is asymmetric.
- **Masked:** `link_on_the_way_axis.rs` plants links only at `.jigc` and `docs/decisions`, so the exceptions' own write targets are never planted.
- **Robust end state:** `uninstall` stays exempt from the store-wide guard, so a user can still remove jigc. In its step 0 (before step 1 removes `.jigc`) it asks the way to every adapter member it edits or removes, through `regular_file::link_on_the_way`, as setup does, and refuses before its first removal.
- **Class:** the exceptions that write at all — 2 of 12 (`setup` asks for itself; `uninstall` does not).
- **Count derivation:** I read `LINK_GUARD_EXCEPTIONS` (12 rows). The 10 readers are `VerbKind::Read` in `VERB_KINDS` (`cli.rs:2150`). The paths `uninstall` writes come from reading `setup.rs::uninstall` steps 1-7: `.jigc`, `CLAUDE.md`, the settings file, the pre-commit hook, the guide. Of those, the settings file and the guide sit below a directory, `.claude`. The hook's directory is git's and I did not probe it.

### Should-fix

**S1 — Self-inflicted lockout: a mover parks a link to a directory into `.jigc/displaced/`, then every guarded door refuses over it.**
- **Location:** `crates/cli/src/relocate.rs:884-961` (`displace_foreign_squatter` moves a squatter "as the entry it is" with `fs::rename`); `crates/cli/src/link_way.rs:154` (the false claim) and `:193` (it counts every link to a directory anywhere in `.jigc`, `displaced/` included).
- **Evidence** (`committed-singletons`): with `notes/roadmap.md` a link to a directory, `jigc config set placement-root notes` exits 0 and prints `notes/roadmap.md → .jigc/displaced/roadmap.md (a foreign file at the new home, parked out of the way)`. The next `jigc start --workflow quick-fix "fix a typo"` is refused with `` `.jigc/displaced/roadmap.md` is a symbolic link inside the workbench ``, and the route tells the user to replace it "with a real directory holding what it led to". That copies the user's directory into the gitignored parking home.
- **Why it is wrong:** a link inside `displaced/` is on the way to nothing jigc writes.
- **Robust end state:** the workbench walk asks only places jigc writes through. The `displaced` entry itself is asked; what is parked inside it is not.
- **Class:** displacement sites by grep (`displace_foreign_squatter`, `task::displace_foreign_area`, three `SubtaskComplement::Displace` uses) are 3 mechanisms. One of them can park a link to a directory without the guard refusing first, because the other two move entries that already stand in the workbench. It is reached by 2 doors (grep `relocate_stranded(`: `config.rs:1572` for `placement-root`, and `relocate.rs:689` for `relocate`). I drove one of the two doors.

**S2 — `config set docs-root <root>` lands every location home past a link at exit 0 when nothing moves, and the guard then refuses the revert.**
- **Location:** `crates/cli/src/config.rs:1373-1383`. The preflight asks only the two ends of docs being moved. `config.unusable-root` asks only the root value's own components (`design/storage.md` → What a root knob refuses).
- **Evidence** (`committed-singletons`, which has no ADR to move): with `notes/decisions` a link outside, `jigc config set docs-root notes` exits 0 and prints `config: set docs-root = notes`. After that, `jigc config set docs-root docs` (the revert) is refused with `store.link-on-the-way` naming `notes/decisions`. `jigc config get` reads back `notes`.
- **Robust end state:** before writing the knob, the door asks the way to every doctype's home as the new value would resolve it (`link_way::on_the_way_to_homes` over the re-rooted schemas) and refuses under `store.link-on-the-way`.
- **Class:** of the 2 `ROOT_KNOBS`, only `docs-root` is affected: `placement-root`'s shipped homes are `<root>/<file>`, whose only directory on the way is the root, which `config.unusable-root` / `config.untrackable-root` refuse. This is an instance at 1 door, bounded by the 2 knobs.

**S3 — Bare `jigc start` (the session hook's command), `start --explain`, `jigc upgrade` and `jigc task validate` refuse at exit 1.**
- **Location:** `crates/cli/src/cli.rs:740-756` (the guard is keyed on the leaf, not on the parsed form) and `:776-830` (the exceptions).
- **Evidence:** the installed `.claude/settings.json` hook is `"command": "jigc start"`. Over a linked `docs/decisions`, bare `jigc start` writes 0 bytes to stdout and 486 to stderr, and exits 1. A session hook that exits 1 with empty stdout gives the session nothing.
  - Precedent: the posture guard's orientation *reports* a breach at exit 0 and blocks nothing (`cli.rs:677-686`). I drove a merge in progress: exit 0.
  - `upgrade` and `task validate` are `VerbKind::Read` in `VERB_KINDS` (`cli.rs:2150`) but are not exceptions. The exceptions list is a second hand-written read set that disagrees with the total classification at 2 leaves, and gives no reason for `upgrade`.
- **Robust end state:** orientation (bare `start`) and `--explain` report `store.link-on-the-way` at exit 0, as orientation does for posture. The read exceptions are derived from `VerbKind::Read`, or each read leaf that stays guarded carries its own reason.
- **Count derivation:** `VERB_KINDS` has 12 Read leaves. The exceptions list 10 of them. The 2 guarded read leaves, plus the 2 read forms of the `start` leaf, make 4 surfaces.

**S4 — A host path is printed in the message, route and JSON `error` for a link in a user's own linked worktree outside the main checkout (law 1).**
- **Location:** `crates/cli/src/link_way.rs:273`. `repo_relative(&jigc_home, root.join(link))` falls through to the absolute path. Its canonicalize arm follows the link, so it cannot strip either.
- **Evidence:** `git worktree add $RIG/wt2`, then a link at `wt2/docs/decisions`, then `jigc start --workflow quick-fix …` run from `wt2`, prints `` `/private<RIG>/wt2/docs/decisions` is a symbolic link … `` in both the message and the route.
- **Robust end state:** a link found under the standing checkout is spelled relative to that checkout.
- **Count:** instance; 1 site (the only `repo_relative` call in `link_way.rs`).

**S5 — One state, several codes.**
- **Where:**
  - a link on the way at `setup`: `setup.write-bootstrap` / `setup.write-guide` (the three setup cells, unchanged);
  - a root value through a link: `config.unusable-root`, or `config.untrackable-root` when it leads outside;
  - everywhere else: `store.link-on-the-way`.
- **Location:** `crates/cli/src/regular_file.rs:105-125` (setup's `blocker` now asks through `link_on_the_way` but keeps the member's code); `crates/cli/src/link_way.rs:57-58` (cites "one fault, one code").
- **Robust end state:**
  - setup raises `store.link-on-the-way` when the blocker is a directory on the way (`!at_leaf()`), and keeps `setup.write-*` for the leaf's own shape.
  - The root knobs could stay as they are because the value is the subject, but the registry should say so.
- **Class:** 5 codes for the state. The count comes from reading the callers of `regular_file::link_on_the_way` / `link_way::` (15 non-comment hits outside `link_way.rs`) and the `config.unusable-root` family in `storage.md`.

**S6 — The message's reason is false for a link inside the repository, and the route is wrong when the link's target is not empty or is inside the repository.**
- **Location:** `crates/cli/src/link_way.rs:94` ("where no commit of this repository holds it") and `:99` ("a real directory holding what it led to").
- **Evidence:** `docs/decisions -> ../decisions` (an in-repo directory, committed, holding `keep.md`). After `jigc setup` (exit 0), the first command refuses saying the write lands "where no commit of this repository holds it". A commit does hold `decisions/keep.md`. Following the route copies it, leaving two copies.
- **The route's main clause is never followed with content:** every route-catalog case's link leads to an empty directory. `not_followed` and `without_case` both say the frame cannot construct a non-empty one (`crates/cli/tests/route_catalog/doc_read.rs`, `LINK_ON_THE_WAY_READING`), and the dominant `.jigc` case is "without case". That is fixture-topology masking: the route is certified only where "holding what it led to" means nothing.
- **Robust end state:**
  - The reason should be what is true in both cases: *git records the link, never a path past it*.
  - The route should split on the link's text, read with `read_link`, which follows nothing. For an in-repo target: move the target into the link's place. For an outside target: copy it in.
  - The frame needs an act that fills the directory.

**S7 — A dangling link, or a link to a file, where the workbench keeps a directory is not counted; the door then fails with a misrouted code.**
- **Location:** `crates/cli/src/link_way.rs:193` (`entry.path().is_dir()`).
- **Evidence:** `.jigc/tasks` a dangling link, then `jigc start --workflow quick-fix "fix a typo"` gives `task.working-area-io — … File exists (os error 17)`, routed at "a disk or permissions problem". Nothing was written outside, because `mkdir` does not follow a link.
- **Inconsistent with the home check:** the home check counts a link "whatever it points at" (`regular_file.rs` doc, entry 25).
- **Robust end state:** the walk knows the workbench's directory places and counts any link standing at one.
- **Count:** instance, unbounded over the workbench's directory places; one site.

**S8 — `setup` succeeds over a linked home, then the very next command refuses (law 3: a constraint first met as a block).**
- **Location:** `crates/cli/src/cli.rs:777-783` (setup exempt; its own walk covers only its install members).
- **Evidence:** in a `bare` repository with `docs/decisions` linked, `jigc setup` exits 0 and the next `jigc start --workflow quick-fix …` gives `store.link-on-the-way`.
- **Robust end state:** setup reports the store surface's links in its ack, at least as advisory.
- **Count:** instance.

### Notes

- **N1** — `store_links` returns early on any workbench link (`link_way.rs:251`), so links at a home are named only after the workbench ones are fixed: two fix-and-rerun rounds where one refusal could name all.
- **N2 — The window between the check and the write (TOCTOU).** The dispatch check and the writes run in one process. The check is asked again only at `regular_file::replace` (7 writers), `move_doc`, the two re-point preflights, `relocate_stranded` and `migrate-corpus`. Task areas, caches, the edge index and the record writes are not asked again. A link made by a third party inside that window is followed. This is acceptable under the adapter-enforced, not-sandboxed model; it should be stated as a declared bound in `storage.md`'s section.
- **N3** — The route-catalog reason for `config set placement-root`'s new home names `config.unusable-root`, but the driven cell (`…-at-the-roadmaps-new-directory`) refuses `config.untrackable-root`. Several `Why::NotConstructible` entries are really "not built": the byte table builds the `relocate` and `migrate-corpus` states itself.
- **N4** — The guard loads the pack and resolves the cascade on every guarded command, and the door then does it again. Unmeasured.
- **N5 — Lead, from before this change:** `jigc validate` (exempt) over an in-repo linked home calls `docs/decisions/keep.md` "committed", though git tracks no path past the link. It routes at `ingest` / `migrate`, which then refuse under the new code. `validate` never reports `store.link-on-the-way` (`validation.md`: "nothing sweeps for it"), which is a gap in reporting the state.
- **N6** — The `CLAUDE.md` sentence about `runner-faithful` ("stock git … no git config at any level") is stale until the main session applies the correction.

## The eight points

1. **Store-wide refusal: right for the writing doors; the over-reach is at the read forms.** Every task ends at a finalize that rebuilds the edge index from every home, so refusing at the mint saves the agent's work rather than refusing it at the end. Neither the doc nor the code decides this; what works for users does. The over-reach is at read forms (S3), and at the revert of the knob that caused the link to be on the way (S2).
2. **Bare `jigc start`: yes, a regression.** It is the session hook's command. Exit 1 with empty stdout means the session starts with no orientation. Robust end state: report at exit 0, as posture does (S3).
3. **`setup` keeping its own codes: a defect against the commit's own one-fault-one-code citation.** Setup should raise `store.link-on-the-way` for a directory on the way and keep `setup.write-*` for the leaf (S5).
4. **`uninstall`: yes, it writes past a linked `.claude`.** It removes the guide outside the repository at exit 0, and with `--force` rewrites the outside settings file (B1). `.jigc` itself is safe: `remove_dir_all` removes the link and leaves the outside untouched, which the axis test drives.
5. **Links inside a fan-out worktree's checkout: no defect found.** From the worktree, the link is refused (driven). From the main checkout, no door I found writes into a worktree's checkout: sub-task doors run elsewhere are refused by `sub_task_fan_out_refusal`, and the merges are git's. The printed path is relative to the main checkout while the user stands in the worktree (cosmetic). A user's own linked worktree outside the main checkout prints a host path (S4).
6. **Dangling links in the workbench: a defect (misroute, not data loss).** It is inconsistent with the home walk. Count any link at a place where jigc keeps a directory (S7).
7. **The route: it can be followed for an outside link to an empty directory; it is wrong for an in-repo target and unproven for a non-empty one.** For an in-repo target it duplicates content. For a non-empty target no case follows it (S6). Robust end state: split the route on the link's text — move an in-repo target into place, copy an outside one in.
8. **`runner-faithful`: faithful enough, and the assertion is sound for what it asserts.**
   - The git package must be the PPA's (`*ppa*`), the system config must be exactly `file:/etc/gitconfig\tsafe.directory=*`, and there must be no global, XDG or `GIT_CONFIG_*` config.
   - Bounds:
     - The PPA's newest git can be ahead of, or behind, the runner image's pinned git within a week. The header's `git version` is the only comparison.
     - "Nothing else in the system config" was established from `install-git.sh` alone. Other runner-image scripts (for example a git-lfs install) may add system entries; one CI step printing `git config --system --list --show-origin` settles it.
     - The ISO week is computed on the host with BSD `date +%G-W%V` (supported).
   - I did not re-run the container's "red on exactly CI's two cells" claim.

## Also asked

- **Message and route against `design/surface-contract.md`'s laws:**
  - Law 1 is broken in two places: the false reason for an in-repo link (S6) and the host path (S4).
  - Law 2 holds: a human route; no `jigc` argv fixes this.
  - Law 3: setup's silent success (S8).
- **Route-catalog cases:** 3 cases for 9 causes.
  - Of the 6 left without a case, the move-primitive belt and the placement-home directory are fair.
  - The workbench `.jigc` itself, and every non-empty target, are the dominant real-world shapes. Their absence leaves the route's main clause unproven (S6).
  - The `relocate` / `migrate-corpus` reasons are "not built" rather than "not constructible" (N3).
- **Byte-table frame and records moved honestly.** With the two timing columns removed, the diff of `byte-table-record-cells.tsv` between `0bd8a5b5` and `63ce9ca5` is exactly the class's 20 cells, whose listing changed to name the link and whose outcome on this tree is *refuses, nothing changed*, plus the two new `placement-root` cells. Every other line differs only in its seconds. The `EXPECTED_RED` removals in `byte_table.rs` match: the key's three rows and §17's two cells.
- **TOCTOU:** see N2.

## Repro blocks

### B1 — uninstall past a linked `.claude`

```bash
WT=<worktree>; BIN=<debug jigc of 63ce9ca5>
rig=$("$WT"/dev/jigc-rig --binary "$BIN" fresh 2>/dev/null) || exit 1; eval "$rig"; cd "$REPO"
O=$(mktemp -d "$RIG/outside.XXXXXX"); cp -R .claude/. "$O"/
git rm -r -q --cached .claude; mv .claude "$RIG/claude-orig"; ln -s "$O" .claude; git add .claude; git commit -q -m "link .claude"
$JIGC uninstall; echo "exit=$?"                 # exit=0, "removed jigc guide artifact"
ls "$O/skills/jigc/SKILL.md"                     # gone: removed outside the repository
# with --force instead: "$O/settings.json" is rewritten (its sha changes)
```

### S1 — the parked link locks every door

```bash
rig=$("$WT"/dev/jigc-rig --binary "$BIN" committed-singletons 2>/dev/null) || exit 1; eval "$rig"; cd "$REPO"
O=$(mktemp -d "$RIG/outside.XXXXXX"); mkdir -p notes; ln -s "$O" notes/roadmap.md
$JIGC config set placement-root notes; echo "exit=$?"   # exit=0; notes/roadmap.md → .jigc/displaced/roadmap.md
$JIGC start --workflow quick-fix "fix a typo"              # store.link-on-the-way naming .jigc/displaced/roadmap.md
```

### S2 — docs-root lands homes past a link; the revert is refused

```bash
rig=$("$WT"/dev/jigc-rig --binary "$BIN" committed-singletons 2>/dev/null) || exit 1; eval "$rig"; cd "$REPO"
O=$(mktemp -d "$RIG/outside.XXXXXX"); mkdir -p notes; ln -s "$O" notes/decisions
$JIGC config set docs-root notes; echo "exit=$?"   # exit=0, knob written
$JIGC config set docs-root docs; echo "exit=$?"    # exit=1, store.link-on-the-way naming notes/decisions
```

### S3 — the session hook's command

```bash
rig=$("$WT"/dev/jigc-rig --binary "$BIN" fresh 2>/dev/null) || exit 1; eval "$rig"; cd "$REPO"
O=$(mktemp -d "$RIG/outside.XXXXXX"); mkdir -p docs; ln -s "$O" docs/decisions
$JIGC start 2>/dev/null | wc -c; $JIGC start >/dev/null 2>&1; echo "exit=$?"   # 0 bytes; exit=1
$JIGC upgrade; $JIGC task validate <id>                                        # both refused (VerbKind::Read)
```

### S4 — the host path

```bash
rig=$("$WT"/dev/jigc-rig --binary "$BIN" fresh 2>/dev/null) || exit 1; eval "$rig"; cd "$REPO"
git worktree add -q "$RIG/wt2" -b side; O=$(mktemp -d "$RIG/outside.XXXXXX")
mkdir -p "$RIG/wt2/docs"; ln -s "$O" "$RIG/wt2/docs/decisions"; cd "$RIG/wt2"
$JIGC start --workflow quick-fix "fix a typo"   # message and route name the absolute host path
```

### S6 — the in-repo link

```bash
rig=$("$WT"/dev/jigc-rig --binary "$BIN" bare 2>/dev/null) || exit 1; eval "$rig"; cd "$REPO"
mkdir -p decisions docs; printf '# Keep\n' > decisions/keep.md; ln -s ../decisions docs/decisions; git add -A; git commit -q -m l
$JIGC setup; $JIGC start --workflow quick-fix "x"   # setup exit 0; then "where no commit of this repository holds it"
```

### S7 — a dangling link where the workbench keeps a directory

```bash
rig=$("$WT"/dev/jigc-rig --binary "$BIN" fresh 2>/dev/null) || exit 1; eval "$rig"; cd "$REPO"
O=$(mktemp -d "$RIG/outside.XXXXXX"); ln -s "$O/tasks-not-yet" .jigc/tasks
$JIGC start --workflow quick-fix "fix a typo"   # task.working-area-io, "File exists (os error 17)", disk/permissions route
```

## Tools

- **`rev/tools/probe1.sh` … `probe10.sh`** in `<scratch>/` — throwaway rig probes, each driving one question against a named binary. Their bodies are the repro blocks above.
- **A timing-insensitive diff of a byte-table record**, which answers "did a re-run of the record move only the class's cells?":

  ```bash
  git show <base>:<rec>.tsv | awk -F'\t' 'BEGIN{OFS="\t"}{$9="";$12="";print}' | sort > old
  awk -F'\t' 'BEGIN{OFS="\t"}{$9="";$12="";print}' <rec>.tsv | sort > new; diff old new
  ```

  The whole TSV is rewritten on every re-run (1566 changed lines here), so without this the honest-movement check cannot be done by eye. It is a candidate for `dev/`, or a mode of the byte-table print: `--compare-outcomes <rev>`.
- **Missing tool:** a rig option to plant a link at a named place (`--plant-link <repo-path>[:in-repo-target]`). Every probe above hand-built one, and so do the byte table's `Plant::LinkToADir` and the axis suite's `linked()`.

<!-- end of report -->
