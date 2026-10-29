# Third re-review: `424e27c3` on `fix/rc24-link-on-the-way` (parent `02699199`)

This review is read-only. I edited nothing in the repository and ran no gate.

- **The binary.** I built a debug `jigc` from `424e27c3` in my own detached worktree (`git worktree add --detach`, under `…/scratchpad/rr4.bSmurF/`), with a private `CARGO_TARGET_DIR` cloned from the previous reviewer's target. The worktree was removed at the end with `git worktree remove`. The binary and target are kept at `…/scratchpad/rr4.bSmurF/target/debug/jigc`.
- **The comparison binary.** The previous reviewer's debug build of `02699199`, copied to `…/scratchpad/rr4.bSmurF/parent/jigc`.
- **How each verdict was reached.** Every verdict marked *driven* comes from running both binaries through `dev/jigc-rig --binary`, in rigs minted by `mktemp -d`. Verdicts marked *by reading* were traced to `source:line` and not driven.
- **What I did not run.** The byte table, the route catalog, or any test target. A grep of the state column of `byte-table-record-cells.tsv` finds no cell this commit can move: no dry-run cell has a link on the way (the four `migrate-corpus-dry-run-…-link-…` cells plant the link at the doc's own place), no `uninstall` cell plants a link, no cell plants inside `.jigc/displaced/`, and route wording is not a column.

## Verdict

**Merge. Nothing I found blocks.**

- **Every item of the second re-review is closed when re-driven:** NS-A, NS-B, NS-C, NS-D, N-2, N-3, N-4 and N-5. N-6, N-7 and N-8 are closed by reading.
- **The commit introduces two should-fix regressions.** Neither loses a byte, and each fix is small:
  - **S-1:** the new "link to nothing" pass in `uninstall` is too wide. A link loop, a link through a file, or a link through an unreadable directory at `CLAUDE.md` now gets a half-done teardown, with a route that cannot be followed. The parent refused these cleanly.
  - **S-2:** the remove-only route N-3 introduced also catches every directory *inside* `.git`. For those its stated reason is false. A route-catalog case still follows the old copy route and stays green, because the catalog does not check an act against the route's words.
- **There are three notes and two leads.**
- **The builder's four left-open items are all ledger keys.** None is a must-fix in the ruled class. One of them (L-3: setup's hook-link route names the outside script as "the link") is a route that would delete the user's script if run as printed. It predates this branch (`55cb29fa`), but it is a one-line fix, and the commit already applied that fix to the sibling it added. I recommend folding it in if there is another fix pass.

## 1. The second re-review's items, re-driven on `424e27c3`

| Item | Status | Evidence |
|---|---|---|
| **NS-A** | **closed**, driven | `dryforecast.sh` (an authored `fix-a-typo`, `README.md` staged), on the parent and then on this commit: <br>• Link at `docs/decisions`: dry run exit 0 → **1**; the act exits 1 on both. <br>• Link at `.jigc/index`: dry run exit 0 → **1**; the act exits 1 on both. <br>• `--format json`: the dry run exits 1, stdout is empty, and stderr is one `{"error": "blocking · store.link-on-the-way …"}`, the same envelope as the act. <br>• `migdry-index.sh` (a research doc below its version, link at `.jigc/index`): `migrate-corpus --dry-run` printed exit 0 and *"1 would migrate"* on the parent; it now exits **1**, the same as the act. <br>• Nothing is written outside (`dryout.sh`: no file under the outside directory is newer than the run). <br>• Mechanism: `Command::forecasts_a_write`, `crates/cli/src/cli.rs:852-860`, read at `:873`. `reads_only()` has no other caller (grep). |
| **NS-B** | **closed** for a link to nothing, driven | `danglingref.sh`, on the parent and then on this commit: <br>• `CLAUDE.md` → `gone.md`: `uninstall` exit 1 → **0**. <br>• `.claude/settings.json` → `gone.md`: `uninstall` exit 1 → **0**. <br>In both, `.jigc` is removed, the link stays, nothing is created where it points, and stderr carries *"note: `…` is a symbolic link to nothing — jigc left it as it is …"*. **The predicate is wider than a link to nothing: see S-1.** |
| **NS-C** | **closed**, driven | `displaced.sh`: `task finalize` exits 0. The note reads *"…NOTES.md — `.jigc/displaced/fix-a-typo` is a symbolic link on the way to its parking place, and jigc writes nothing past a link"*, and the outside directory stays empty. `displaced-after.sh`: `NOTES.md` is still in `.jigc/tasks/fix-a-typo/`, so nothing is lost. A later `start` of the same intent is refused with `task.serial-collision` (leftover area). That is the existing *could not park* arm. Both callers settle through `unwind_settled_area`, which is conditioned on the outcome: `task.rs:8007-8017` and `milestone.rs:7597-7617`. Control (`nolink`): the file is parked inside the repository. `rollback::park`: see note N-1. |
| **NS-D** | **closed**, driven | `hooklink.sh` (a foreign hook wrapped by `setup`, then moved out and linked back): <br>• Parent: `uninstall` exits 0 and rewrites the outside script, 66 lines → 2. <br>• This commit: exits **1** with `uninstall.remove-precommit`, with and without `--force`, and the outside sha is unchanged. <br>• The message names the link as `.git/hooks/pre-commit`; the reason prints the outside path (L-2). |
| **N-2** | **closed**, driven | `task diff --format json` over a link at `docs/decisions` gives `findings == ["store.link-on-the-way"]` and empty stderr. The contract text left stale is note N-2. |
| **N-3** | **closed**, driven | `movein.sh`: the targets `.`, `..`, `../..`, `/` and `../.git` all get *remove the link …, which leads to the repository, a directory that holds the link, or git's own directory*. `nowhere` and `../README.md` get *leads to no directory*. NS2's three shapes are unchanged (copy, copy, move `vendor/dec`). **Over-extended to directories inside `.git`: see S-2.** |
| **N-4** | **closed**, driven | `reflink.sh .jigc/mine.md "# mine"`: <br>• Parent: `setup` exits 0, and `uninstall` deletes the target and leaves `CLAUDE.md` dangling. <br>• This commit: `setup` exits 1 with `setup.inject-reference` (*"…leads to `.jigc/mine.md`, inside jigc's own workbench…"*); `uninstall` exits 1 with `uninstall.unwire-reference`; the target keeps `# mine`. |
| **N-5** | **closed**, driven | `reflink.sh AGENTS.md EMPTY`: <br>• Parent: deletes the user's `CLAUDE.md` link and leaves jigc's section in `AGENTS.md`. <br>• This commit: keeps `CLAUDE.md -> AGENTS.md`, and `AGENTS.md` is empty again. <br>• `reflink.sh AGENTS.md "# mine"`: `# mine`, the link kept (unchanged). <br>• The write-back of the emptied target goes through a link `edited_through_links` already held to a regular file inside the checkout, not `.git/`, not `.jigc/` (`adapter.rs:1031-1043`). |
| N-6, N-7, N-8 | closed | By reading. N-6 goes with NS-A (the dry run refuses once). N-7 is `doc_read.rs:1432-1449`, now `NotBuilt` with the byte-table cell names. N-8: the DECISIONS and `storage.md` counts now say 26. |

## 2. What this commit newly broke

### Should-fix

**S-1: `jigc uninstall` treats any `CLAUDE.md` link whose metadata errs as "a link to nothing". It removes `.jigc`, then refuses at the unwire step with a route that does not work.** This is a regression, new in this commit.

- **Location:**
  - `crates/cli/src/setup.rs:3156`: `if std::fs::metadata(jigc_home.join(path)).is_err() { to_nothing.push(…); continue; }`. The predicate is *any error*, not `NotFound`.
  - The step that then fails: `crates/cli/src/adapter.rs:1006-1010`. `unwire_reference` treats only `NotFound` as absent; every other read error is an `Err`, raised at `setup.rs` step 2 as `uninstall.unwire-reference`, *after* step 1's `remove_dir_all(.jigc)`.
- **Evidence:** `oddref.sh <bin> CLAUDE.md <kind>`, a `fresh` install whose `CLAUDE.md` is replaced by an odd link and committed:

  | kind | `02699199` | `424e27c3` |
  |---|---|---|
  | `loop` (`CLAUDE.md -> CLAUDE.md`) | exit 1, `.jigc` kept, link refusal | exit 1, **`.jigc` removed**, *"cannot unwire … Too many levels of symbolic links (os error 62)"*, route *"ensure `CLAUDE.md` is writable, then re-run"*. The re-run exits 1 again. |
  | `notdir` (`-> README.md/x`) | exit 1, `.jigc` kept | exit 1, **`.jigc` removed**, *"Not a directory (os error 20)"*. The re-run exits 1. |
  | `eacces` (`-> priv/f.md`, `priv` mode 000) | exit 1, `.jigc` kept | exit 1, **`.jigc` removed**, *"Permission denied (os error 13)"* |

- **What is left behind** (`oddref-left.sh … loop`): `.claude/settings.json` keeps jigc's permits and session hook, the guide `.claude/skills/jigc/SKILL.md` stays, and the `pre-commit` hook keeps jigc's block.
  - `.jigc/settings-entries.json` is gone with `.jigc`. It is the settings record that tells a re-run which permits are jigc's. So once the link is repaired, a re-run without `--force` takes none of them (`setup.rs`, step 0b: `(_, false) => default`) and only narrates them as left.
  - The record is still in the index (`git checkout` brings it back, as the warning says), so nothing is lost.
- **Class and count derivation:** the two members `edited_through_links` is given, checked against the reader each one's steps use:
  - `CLAUDE.md` → `unwire_reference` reads `NotFound`-only: **strands**.
  - `.claude/settings.json` → `remove_allowlist` / `remove_hook` / `remove_deny` gate on `target.exists()` (`adapter.rs:1079`, `:1232`, `:1324`), which swallows every error. **Driven:** `loop` and `notdir` exit 0 and write nothing. But the note then calls a loop "a link to nothing".
  - So the class is 1 member × 3 error kinds, all driven. Other error kinds (e.g. `ENAMETOOLONG`) are the same mechanism and were not driven.
- **Severity reasoning:** nothing is lost, and the states are user-made oddities. But a teardown that the parent refused with nothing removed now stops halfway, and its printed route cannot resolve it. That breaks the rule that a route is run as printed.
- **Robust end state:** pass only `NotFound`, i.e. `matches!(std::fs::metadata(..), Err(e) if e.kind() == std::io::ErrorKind::NotFound)`, and let every other end fall to `link_end`'s refusal, as on the parent. Have `axis` drive a loop beside the dangling link.

**S-2: N-3's remove-only route now catches every directory *inside* `.git`, where its reason is false. A route-catalog case still follows the old copy route, and stays green.** This is a regression of a documented, catalog-tested route, new in this commit.

- **Location:**
  - `crates/cli/src/link_way.rs:167-173`: `in_gits` is true for any target whose first component is `.git`, which routes to `WayOut::RemoveOnly`.
  - The clause, `:126-131`: *"which leads to the repository, a directory that holds the link, or git's own directory — no copy of those belongs in its place"*.
  - `design/storage.md:142`: *"because a copy of any of those would hold the repository, or the link, again"*.
- **Evidence:** `gitlink.sh`, parent then this commit:
  - `.claude -> .git/dotclaude` (holding the user's `settings.json`), then `jigc setup`: the route changes from *"replace the link at `.claude` with a real directory holding a copy of what it leads to"* to *"remove the link at `.claude`, which leads to … git's own directory — no copy of those belongs in its place"*.
  - `docs/decisions -> ../.git/mydecisions` (holding a decision), then `jigc start …`: the same change.
  - A copy of `.git/dotclaude` holds neither the repository nor the link, so the stated reason is false for every proper subdirectory of `.git`.
  - Following the new route deletes no bytes. But after the re-run the user's settings, or their decisions, are silently disconnected from the project, and the route says a copy is wrong.
  - The previous review's N-3 named the root and `.git` itself. Nothing asked for `.git/<dir>` to change.
- **Masked by the catalog:**
  - `crates/cli/tests/route_catalog/doc_read.rs:1287` (`link-on-the-way-setup-over-a-committed-link-into-gits-directory-replaced-committed-and-re-run`) still performs `Act::ReplaceWithACopy`, which the printed route no longer asks for.
  - It stays green because an act is checked only against its operand span, never against the route's verb. See `crates/cli/tests/support/route_case.rs:1737-1756` (description) and `:2561-2574` (`replace_with_a_copy`).
  - The reading's derivation, `doc_read.rs:1495-1511`, still says *"setup's member under a link into git's own directory is the cause of a link out of the repository, its way a copy"*.
  - The derivation counts causes "by where the link leads — out, inside, or to no directory". The fourth way, `RemoveOnly`, has no case, and only a text assertion in `link_on_the_way_axis.rs` (`a_link_that_leads_to_what_holds_it_is_never_routed_at_a_copy`).
- **Class and count derivation:** the `WayOut` enum has 4 variants (`link_way.rs:60-81`). `RemoveOnly` is reached by 3 predicates: `holds_the_repository`, `in_gits` and the stands-in arm at `:184`. Only `in_gits` over a proper subdirectory has a false reason. Driven at 2 doors (`setup`'s member, a doc home). Every door that prints `store.link-on-the-way` shares `way_out`, so the class covers all of them by construction.
- **Robust end state:** `RemoveOnly` only where the target holds the link or the repository, or *is* `.git`. A directory inside `.git` stays `CopyIn`, as documented before. Optionally, have the catalog's link acts assert the clause they follow, so a case cannot follow a route the surface no longer prints.

### Notes

- **N-1: `rollback::park` now drops a pre-image where it used to park it past a link.** By reading, not driven: reaching it takes a rollback racing a concurrent edit.
  - `crates/cli/src/rollback.rs:813` returns `None` on a link inside `.jigc/displaced/<door>/…`. `rollback_conflict_finding` then says *"…so that version is gone"* (`:851-853`).
  - On the parent those bytes were written past the user's link, which is outside but kept. Unlike `displace_foreign_area`, `park` has no "leave it where it is": the pre-image exists only in memory.
  - It is the same `None` arm as a failed write, which was already accepted, so this is not new in kind.
  - Robust end state: fall back to a free leaf directly below `displaced/`, which the guard asks, as `relocate::displace_foreign_squatter` does.
- **N-2: `design/command-output-contract.md:113` still says *"`findings` is structurally always empty"* for `task diff`.** That is false since N-2, and it contradicts the same doc's link-on-the-way bullet. Only the `render.rs` comment was updated.
- **N-3: `completions/…/reshape/route-catalog-findings.md`, the `uninstall.remove-precommit` row,** reads *"is a link to a read-only script outside (`-a-link-to-a-read-only-script-inside`; …)"*. The word "outside" is stale beside the renamed case.

### Leads (not pursued)

- **`engine::state::workbench_link_on_the_way` starts its walk at the *first* `.jigc` component of an absolute path** (`crates/engine/src/state.rs:1328-1330`). A repository that lives below an unrelated ancestor directory named `.jigc`, with a link between that ancestor and the repository, would have every workbench write refused. This predates this commit (`02699199`). Not driven.
- **A dangling or looping `pre-commit` hook gets a partial teardown at step 6** (`remove_precommit_hook`'s read error after `.jigc` is removed and `CLAUDE.md` is unwired). `hook_written_through_a_link` deliberately passes `None`-why ends. This predates this commit; the parent had no hook check at all. Not driven.

## 3. The builder's left-open items: in class, or a ledger key?

| Item | Driven | Judgment |
|---|---|---|
| **L-1: `install_precommit_hook` writing through a linked `.git/hooks` directory** | `hooksdir.sh`: `.git/hooks` → an outside directory. `setup` exits 0, the hook lands outside, and the ack prints its absolute path. `uninstall` exits 0 and removes it there. | **Ledger key, not in the class.** The ruling's *why* (git records the link, so what is written past it is in no commit) never applies to hooks, which are never committed. Git follows that link to run the hook, so the write is where git looks, the same as an outside `core.hooksPath`, which jigc already honours. The defect is a doc gap: `storage.md:138`'s headline, *"jigc writes … only inside the repository"*, should name git's hooks directory as the one place it writes that may lie outside. |
| **L-2: `link_end` / `hook_link_end` printing the outside target's absolute path** | `hooklink.sh` (uninstall) and `setuphooklink.sh` (setup) both print `/private/…/outside.*/pre-commit`. | **Ledger key.** Surface-contract law 1 sanctions *"the honest absolute for a path genuinely outside the repository"* (`design/surface-contract.md:61-65`), and `setup.rs` surfaces are declared outside the printed-path fence (`:75`). But the rule's own home says *"an outside target is a host path, which a surface never prints"* (`storage.md:142`, scoped there to `store.link-on-the-way`'s route), and the adjacent bullet's members print one. The two statements need one reconciled rule. This commit adds one more site (`uninstall.remove-precommit`'s reason) to three that already existed. |
| **L-3: setup's hook-link refusal naming the target instead of the link** | `setuphooklink.sh`, the same on both binaries: route *"remove the link at `/private<OUTSIDE>/pre-commit`, or point it at …"*. | **Ledger key, but the highest of the four, and cheap to fix now.** Mechanism: `hook_link_end` builds `shown` with `display_hook_path` (`setup.rs:3218`), which **canonicalizes** the hook (`:960-971`) and so names where the link leads. `hook_link_refusal` (`:3284-3302`) puts that in the route. Run as printed, *remove the link at `<outside script>`* deletes the user's real script, not the link. It predates this branch (`55cb29fa`, before base `0bd8a5b5`) and this commit does not worsen it. But this commit fixed the identical problem in its new sibling (`hook_written_through_a_link`: *"The link itself, never where it leads"*, `:3237-3241`) and left this one. Fix: the same two-line `shown`. Not a merge blocker. |
| **L-4: a link at a workbench file's own place** | `leafconfig.sh`: `.jigc/config/manifest.yaml` → an outside file, then `config set invocation-log true`. Both binaries exit 0, and the outside file's sha goes `d322af9ad82f` → `830f2127af8f`. | **Ledger key, a separate class.** The ruling and `storage.md:140` scope this rule to the way, and declare a link *"in a file's place … that file's own question, under its own code"*. For `config`'s writers no such code exists. The *why* does apply: the manifest is tracked, so git records the link and the knob value is in no commit. **Count:** 6 raw `std::fs::write` production sites in `crates/cli/src/config.rs` (grep: lines 1871, 1948, 2042, 2103, 2332, 2404), 1 driven. `persist`/`write_atomic` replace a leaf link rather than follow it, so the primitives are safe. The workbench-wide count of raw writers that follow a leaf link is **unbounded here**: the previous census lists 22 writer functions into `.jigc/`, and each would need a check of how it writes. |

## Repro blocks

### S-1: an odd `CLAUDE.md` link gets a half teardown

```bash
BIN=<debug jigc of 424e27c3>; WT=<a worktree of it>
rig=$("$WT"/dev/jigc-rig --binary "$BIN" fresh 2>/dev/null) || exit 1; eval "$rig"; cd "$REPO"
rm CLAUDE.md; ln -s CLAUDE.md CLAUDE.md; git add -A; git commit -q -m "a link loop"
$JIGC uninstall; echo "exit=$?"; test -e .jigc && echo kept || echo removed
# exit=1, removed — "cannot unwire … Too many levels of symbolic links", route "ensure `CLAUDE.md` is writable"
# (02699199: exit=1, kept — refused up front)
$JIGC uninstall; echo "exit=$?"   # exit=1 again: the route does not resolve it
```

### S-2: a link into a directory inside `.git` is routed at removal

```bash
rig=$("$WT"/dev/jigc-rig --binary "$BIN" bare 2>/dev/null) || exit 1; eval "$rig"; cd "$REPO"
mkdir -p .git/dotclaude; printf '{"permissions":{"allow":["Bash(make:*)"]}}\n' > .git/dotclaude/settings.json
ln -s .git/dotclaude .claude; git add -A; git commit -q -m l
$JIGC setup 2>&1 | grep route:
# "remove the link at `.claude`, which leads to … git's own directory — no copy of those belongs in its place …"
# (02699199: "replace the link at `.claude` with a real directory holding a copy of what it leads to …")
```

### L-3: setup's hook-link route names the outside script as the link

```bash
rig=$("$WT"/dev/jigc-rig --binary "$BIN" bare 2>/dev/null) || exit 1; eval "$rig"; cd "$REPO"
O=$(mktemp -d "$RIG/outside.XXXXXX"); printf '#!/bin/sh\necho mine\n' > "$O/pre-commit"
H=$(git rev-parse --git-path hooks)/pre-commit; mkdir -p "$(dirname "$H")"; ln -s "$O/pre-commit" "$H"
$JIGC setup 2>&1 | grep route:   # "remove the link at `/private/…/outside.*/pre-commit`" — a regular file, the user's script
```

### L-4: `config set` writes through a linked manifest

```bash
rig=$("$WT"/dev/jigc-rig --binary "$BIN" fresh 2>/dev/null) || exit 1; eval "$rig"; cd "$REPO"
O=$(mktemp -d "$RIG/outside.XXXXXX"); printf 'scalar: {}\n' > "$O/m.yaml"; ln -s "$O/m.yaml" .jigc/config/manifest.yaml
$JIGC config set invocation-log true; echo "exit=$?"; cat "$O/m.yaml"   # exit=0; the outside file now holds the knob
```

## Tools

Everything is in `<scratch>/rr4.bSmurF/tools/`. Each tool is one rig and one question, and takes the binary as an argument or as `BIN=`.

**Copied from `rr3.QkO7ck/tools/`**, with `R=` re-pointed and `--binary ${BIN:-…}` made overridable in `hooklink.sh`, `reflink.sh`, `movein.sh` and `displaced.sh`: `dryforecast.sh`, `migdry.sh`, `danglingref.sh` (its last line now prints the stderr tail and `git status`), `displaced.sh`, `hooklink.sh`, `reflink.sh`, `movein.sh`, and the rest of that set (not re-run).

**New:**

- **`dryout.sh <bin> <place> [json]`**: the dry run's full streams over a link, and whether anything outside changed.
- **`migdry-index.sh <bin>`**: NS-A's `migrate-corpus --dry-run` with the link at `.jigc/index`, not at a prior home. `migdry.sh` only ever hit the prior-homes refusal, which refused on the parent too.
- **`oddref.sh <bin> <CLAUDE.md|.claude/settings.json> <loop|notdir|eacces>`**: does `uninstall` refuse up front over a link whose metadata errs for a reason other than "nothing there", or tear down partly? Answers S-1.
- **`oddref-left.sh`**: the same, then lists what a half teardown left (`git status`, `.claude`, the hook) and whether a commit still works.
- **`displaced-after.sh`**: after NS-C's refused parking, is the user's file still in the area, and what does the next `start` do?
- **`gitlink.sh <bin> <claude|decisions>`**: the route printed for a link into a directory inside `.git`. Answers S-2.
- **`setuphooklink.sh <bin>`**: which path setup's hook-link refusal names as "the link". Answers L-3.
- **`hooksdir.sh <bin>`**: does `setup` and `uninstall` write through a linked `.git/hooks`? Answers L-1.
- **`leafconfig.sh <bin>`**: does `config set` write through a link at `.jigc/config/manifest.yaml`? Answers L-4.

**Re-running them:** the scripts read `$R/wt/dev/jigc-rig`, and the worktree has been removed. Re-create it first with `git worktree add --detach <R>/wt 424e27c3`. The binaries are kept: `<R>/target/debug/jigc` (this commit) and `<R>/parent/jigc` (`02699199`).

**Instruments not run, and why.** The byte table's both-binary arm (`the_cells_are_run_on_the_binary_a_variable_names`, about 800 s): the state-column grep in the header shows no cell this commit can move, and the builder's gate ran its gate cells green. Running it would certify that grep.

**Tool wanted, again:** a `dev/jigc-rig` option to plant a link at a named place, to a named target (inside, outside, nothing, a loop). Every probe here builds one by hand.

**Tool gap found (a test-support defect, reported as part of S-2).** `route_case.rs`'s link acts (`ReplaceWithACopy`, `MoveDirectoryIn`, `ReplaceWithDirectory`, `PutTheFileItself`) check only that their operand span exists. They never check that the printed route asks for that act. So when a route's wording changes, a case can stay green while following a route the surface no longer prints.

<!-- end of report -->
