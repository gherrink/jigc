# Re-review — `26474b80` on `fix/rc24-link-on-the-way` (parent `63ce9ca5`)

This is a read-only review. I edited nothing in the repository and ran no gate. I built a debug `jigc` from `26474b80` in a detached worktree of my own, with a private `CARGO_TARGET_DIR` under the session scratchpad. Every verdict below comes from driving that binary through `dev/jigc-rig --binary`, in rigs minted by `mktemp -d`. I ran no test suite. The worktree was removed at the end with `git worktree remove`.

## Verdict

**Every earlier finding but one is closed when driven. S8 is only partly closed.**

The fix opened new problems, and one of them is blocking:

- **A write past a link, at the session hook's own command (NB1).** S3's change from *refuse* to *report* lets two read forms write past a link at exit 0: bare `jigc start` and `jigc task validate`. Both rebuild the edge index `.jigc/index/edges.json` past a linked `.jigc` or `.jigc/index`. The advisory they print meanwhile says *"This one only reads, and wrote nothing"*.
  - On `63ce9ca5` both forms refused, so this regression is new in this commit.
  - It breaks the rule's own home sentence (`storage.md`): *"never past a link on the way"*.
  - The test suite does not see it, because of how its fixtures are built.
- **A broken `--format json` contract (NS1).** When a read form rejects under `--format json` with a link on the way, stderr carries the advisory line plus the JSON document. The pinned rule says that stream holds exactly one document.

The rest of what was asked:

- **The read-form classification.** No writing form slips into the read set. Two read forms of writing leaves still refuse: the `--dry-run` forms of `migrate-corpus` and `task finalize` (N-a).
- **The `make_pack_ungated` homes.** I found no case where the guard reads homes that differ from what the command then resolves (N-f).
- **The byte table.** The ten moved cells are exactly the class's. The regressions fell from 8 to 6 because two were fixed, not because a listing absorbed them.
- **The retired key.** It was retired honestly. Its state now meets the guard, and the guard's route, followed, provisions.
- **The newly red stale-key "unmanage" case.** Blaming the existing stale-key finding is fair, and S1 is working as intended there. But the case's classification is imprecise (N-d).
- **(3) `migrate.source-untrackable` over a symlinked source.** This is **not** the same state as `store.link-on-the-way` (see §3).

## 1. The earlier findings, re-driven on `26474b80`

| Finding | Status | Evidence (driven) |
|---|---|---|
| B1 | **closed** | With `.claude` a link to an outside directory, `jigc uninstall` and `jigc uninstall --force` both exit 1 with `store.link-on-the-way` naming `.claude`. The outside `settings.json` keeps its sha (`8e60bcfd…`) and `SKILL.md` keeps its sha (`db43b74d…`). The repository's bootstrap section is still there. |
| S1 | **closed** | With `notes/roadmap.md` a link to a directory, `config set placement-root notes` exits 1 with `config.repoint-failed` wrapping `store.home-not-regular-file`. Nothing is parked (`.jigc/displaced` is absent), and the next `start --workflow quick-fix` mints. Following the new route (*move the link out of the doc's home*) and re-running lands the re-point at exit 0. |
| S2 | **closed** | `config set docs-root notes` over a linked `notes/decisions` exits 1, and nothing is written (only the user's own `notes/` is untracked). In the real revert case (knob set to `notes`, then a link at `notes/decisions`), `config set docs-root docs` exits 0. |
| S3 | **closed as asked; opened NB1 and NS1** | Bare `start` exits 0 with 2880 bytes on stdout, the advisory carried there. `upgrade`, `task validate` and `task list` report, and run. |
| S4 | **closed** | Run from a user's own linked worktree, the message and route name `docs/decisions`, with no host path. |
| S5 | **closed** (at `setup` and the root knobs) | `setup` over a linked `.claude` gives `store.link-on-the-way`, and nothing lands outside (0 entries). `config set docs-root lnk/sub`, with `lnk` a link, gives `store.link-on-the-way`. |
| S6 | **closed** for the shapes it named | Over `docs/decisions -> ../decisions`, the route reads *remove the link at `docs/decisions` and move the directory `decisions` into its place*. Followed as printed: `setup` exits 0, git tracks `docs/decisions/keep.md`, and one copy remains. A broken edge shape is in NS2. |
| S7 | **closed** | A dangling `.jigc/tasks` gives `store.link-on-the-way` with *remove the link …, which leads to no directory*, and nothing is written outside. |
| S8 | **partly open** (NS3) | Fixed for the dev pack's homes. A link at a methodology home passes `setup` at exit 0, and the next command refuses. |
| N1 | **closed** | A link at `.jigc/index` and one at `docs/decisions` are named in one refusal, each with its own route clause. |
| N2 | closed (as a doc) | The bound is stated at the rule's home in `storage.md`. |
| N3 | closed (read) | `Why::NotBuilt` is present, and the `config.unusable-root` reading drops its link cause (`config.rs` reading, causes 8 → 7). |
| N4 | not re-measured | `make_pack_ungated` is read in §2 (N-f). |

## 2. New findings, ranked

### Blocking

**NB1 — Bare `jigc start` and `jigc task validate` rebuild `.jigc/index/edges.json` past a link at exit 0, while their advisory says "wrote nothing".**
- **Location:**
  - `crates/cli/src/cli.rs:741-763` — the `LinkGuard::Reports` arm runs the command.
  - `crates/engine/src/index.rs:242-256` — `load_committed` persists through `EdgeIndex::save`.
  - It is reached through `crates/engine/src/validate.rs:430`: the task-scope sweep that orientation runs for each active-task row (`crates/cli/src/orient.rs:185-204`) and that `task validate` runs.
  - The false claim: `crates/cli/src/link_way.rs:228-241` (*"This one only reads, and wrote nothing"*).
  - The rule it breaks: `design/storage.md` → *No write through a link on the way* (*"never past a link on the way"*).
- **Evidence:**
  - A `committed-singletons` rig, `start --workflow quick-fix "fix a typo"`, then `.jigc/index` turned into a link to an outside directory. `jigc start` exits 0 and a new `./edges.json` appears outside. `jigc task validate fix-a-typo` exits 3 and also writes `./edges.json` outside.
  - Dominant shape: the whole `.jigc` a link out of the repository, one active task, then an ordinary commit. `jigc start` (the session hook's command) exits 0. On `fresh` it creates `index/`, `index/edges.json` and `index/edges.json.lock` in the outside directory; on `committed-singletons` it creates `index/edges.json`.
  - On `63ce9ca5` both forms refused (the earlier review's S3 evidence), so this is new in this commit.
- **Masked:** both suite tests miss it.
  - `link_on_the_way_axis.rs`'s `sweep(".jigc")` runs on `State::Fresh` with no active task. Orientation then sweeps no row and builds no index.
  - `every_read_form_reports_the_link_at_exit_0` does mint a task, but it links `docs/decisions` (a home), so `edges.json` is written into the real `.jigc/index`.
  - This is fixture-topology masking: the dominant shape (a session with an open task, after a commit) is never built.
- **Robust end state:** where the guard reports instead of refusing, a read form persists no cache past the link. The invocation log already does this ("stands down past a link"), so the edge index should be built in memory and not saved there. Alternatively, the read form does not run its sweep, and says so. Then the advisory's *"wrote nothing"* is true.
- **Class and count derivation:**
  - I drove 15 read forms × 3 linked workbench places (`index`, `state`, `tasks`). I listed the read forms myself: the 12 `VERB_KINDS` `Read` leaves, plus `start --explain`, `start --workflow router` and `workflow --preview`; bare `start` makes 16. Probe: `readforms-all.sh`, 46 runs.
  - Exactly 2 forms wrote past the link (bare `start` and `task validate`), both at `index`. Nothing wrote past `state` or `tasks`.
  - There is one persisting mechanism: `load_committed`, of which `grep -rn 'load_committed('` gives 7 callers. The other 6 are writing doors (`rename`, `unmanage`, `ingest`, `milestone join`, `milestone finalize`, compose with a task), which the guard refuses.

### Should-fix

**NS1 — Under `--format json`, a read form that rejects with a link on the way prints the advisory and the reject document together on stderr, so stderr no longer parses as one document.**
- **Location:** `crates/cli/src/cli.rs:759-764`. The advisory is `eprintln!`-ed before dispatch, whatever the outcome.
- **The rule it breaks:** `design/command-output-contract.md:448` (*"the JSON-bearing stream parses as exactly one document"*) and `:460` (*"on the arm whose document is stderr's, nothing else is written there"*).
- **Evidence** (`committed-singletons`, with `docs/decisions` a link):
  - All six of `jigc --format json` `doc show no-such-doc` · `task validate no-such-task` · `task diff no-such-task` · `doc schema no-such-type` · `workflow no-such-wf --preview` · `config get no-such-key` exit 1 with an empty stdout, and stderr is **not** one JSON document: the advisory line comes first.
  - Control, without the link: `doc show no-such-doc` exits 1 and stderr parses.
  - New in this commit: on `63ce9ca5` four of these leaves were silent exceptions and two refused with one document.
- **Precedent and robust end state:** orientation carries a posture breach inside its JSON `findings` array (`crates/cli/src/orient.rs:159-204`), with no schema move. Under json the advisory should fold into the document — the adjudication's `findings`, or the reject document's `findings` — or be withheld there. Even on its adjudication arm, orientation's JSON carries no in-contract signal today (N-b).
- **Class and count derivation:** one emission site, which reaches every reject of every read form under `--format json` while a link stands. I drove 6 of 6.

**NS2 — The *move it in* route, for a link whose in-repository target lies past another link, moves the user's outside directory into the repository.**
- **Location:** `crates/cli/src/link_way.rs:139-160`. `real_directory` uses `symlink_metadata(&target)`, which follows every component but the last, so an in-repository spelling under a linked component reads as a real directory.
- **It contradicts its own doc** at `link_way.rs:61-63`: *"(or to one inside it only through another link): put a copy … What stands outside is the user's and stays."*
- **Evidence** (`fresh`):
  - Set up `vendor -> <outside>` and `docs/decisions -> ../vendor/dec`, with `<outside>/dec/shared.md`. The refusal's route reads *remove the link at `docs/decisions` and move the directory `vendor/dec` into its place*.
  - Followed as printed, the outside directory loses `dec/` entirely: `find <outside>` lists only the directory itself.
- **Class:** instance. There is one site, `way_out`, reached by every link whose lexical target sits inside the repository below a linked component.
- **Robust end state:** take the *move it in* way only when no component of the lexical target is a link (walk it with `regular_file::link_on_the_way`), and *copy it in* otherwise.

**NS3 — S8 is only partly closed: `setup` asks the dev pack's default homes, but not the methodology pack's, which `setup` itself switches on.**
- **Location:**
  - `crates/cli/src/link_way.rs:417-428` (`schemas_at` with `Homes::CurrentOrDefault`).
  - `crates/cli/src/pack.rs:2156-2161`: `make_pack_ungated` reads the compose marker, which is absent before setup.
  - `crates/cli/src/setup.rs:2234`: setup writes the marker.
- **Evidence** (`bare`, `docs/research` a link to an outside directory): `jigc setup` exits 0 with nothing on stderr, and `packs.yaml` then carries the methodology marker. The next `start --workflow quick-fix` refuses with `store.link-on-the-way` naming `docs/research` (*"on the way to the home of `research`"*).
- **Count derivation:** I grepped `location:` across `crates/cli/packs/methodology/schemas/`. Of its 8 hits, `inconsistencies/`, `ideas/`, `completions/`, `research/`, `planning-records/`, `jigc-feedback/`, `milestone-records/` and `dogfood/` are homes setup does not ask. A link at a placement doctype's directory (`docs/`) is still caught, through dev's `docs/decisions`.
- **Robust end state:** `CurrentOrDefault` assembles the pack-set setup is about to compose — the marker as setup will write it — not the pre-setup floor.

**NS4 — `jigc uninstall --force` rewrites an outside file through a link at the settings file's own place, at exit 0.** This one is older than the commit: it is B1's class at the leaf, which B1 bounded to directories.
- **Location:** `crates/cli/src/setup.rs:4899-4922` asks only the way to each member. The edits go through `adapter::remove_allowlist`, `remove_hook` and `remove_deny` (`setup.rs` uninstall steps 3-5).
- **Evidence** (`fresh`, with `.claude/settings.json` a link to an outside copy):
  - `jigc uninstall --force` exits 0, prints `removed jigc allowlist permit ← .claude/settings.json`, and the outside file's sha moves from `8e60bcfd6fa6` to `4a4057bc4bf6`.
  - Plain `jigc uninstall` leaves that file alone.
  - Through a link at `.claude/skills/jigc/SKILL.md`, the guide's removal takes the link only, so the outside `SKILL.md` stays.
  - `jigc setup` refuses the same state (`setup.inject-allowlist`), so `setup` and `uninstall` still disagree at the leaf.
- **Class:** instance. I did not drive a link at `CLAUDE.md` (the bootstrap line's file, which `unwire_reference` edits).

**NS5 — S1's new refusal has no route-catalog case and is not counted.**
- **Location:** the new route is at `crates/cli/src/relocate.rs:922-944` (*"move the link at `…` out of the doc's home — it is yours, and jigc moves no link …"*). The site's reading is `crates/cli/tests/route_catalog/doc_read.rs:1360-1445`: still `causes: 32`, with no `relocate.rs::displace_foreign_squatter` among the files read and no row in `without_case` for a link at a mover's destination.
- **Why it matters:** the brief requires that every new refusal site gets its route-catalog case.
- **Evidence:** I followed the route by hand (`committed-singletons`, `notes/roadmap.md` a link to a directory): the link moved aside, then `config set placement-root notes` exits 0. So the route works; only the catalog does not show it.
- **Count derivation:** 1 new cause, reached by 3 doors (`config set placement-root`, `config set docs-root`, `relocate`), all through `relocate_stranded` or `move_doc` → `displace_foreign_squatter`.

### Notes

- **N-a — Two read forms of writing leaves still refuse.** `migrate-corpus --dry-run` ("writes nothing at all", per its help) and `task finalize --dry-run` refuse over a link. `Command::reads_only` (`cli.rs:816-829`) names only the forms of `start` and `workflow`.
  - *Count:* I grepped `dry_run: bool|preview: bool|explain: bool` across `crates/cli/src`. That gives 4 read-form flags on writing leaves; 2 are handled.
  - Refusing errs on the safe side; the gap is consistency with S3.
- **N-b — Orientation's `--format json` carries no in-contract signal for the link**, which goes to stderr. Compare the posture breach, which rides the `findings` array.
- **N-c — A doc comment is attached to the wrong function** (`crates/cli/src/pack.rs:2131-2156`). The new `make_pack_ungated` was inserted between `discover_project_config`'s doc comment and its `fn`. That function lost its doc, and the new function's doc opens with the CWD-discovery text.
- **N-d — The stale-key "unmanage" case** (`crates/cli/tests/route_catalog.rs:858-866`, classified `SameRefusalAgain` under `TWO_MANAGED_DOCS_OVER_A_STALE_KEY`). Driven:
  - The first refusal calls the link *"a managed instance"* (`write.already-present`), which is the existing stale-key finding's defect.
  - After `unmanage`, the re-run refuses with `config.repoint-failed` wrapping a **different** finding, `store.home-not-regular-file`, whose route is right and works when followed.
  - Blaming the existing finding is fair, and S1 is behaving as intended. But "the same refusal again" matches only the outer code; reading the inner finding, it is *another refusal, routed correctly*.
- **N-e — A route tail that does not fit the workbench.** Every route ends *"committing the change where git tracks the link"* (`link_way.rs:183-192`), including for links under `.jigc/`, which is gitignored (for example, the retired key's `.jigc/worktrees/do-a-thing`).
- **N-f — `make_pack_ungated` versus the gated pack.** I found no divergence in what gets written.
  - With a project-layer whole-file shadow that re-homes `adr` (which the freeze gate refuses), the guard asks the shadow's home and refuses first over a link there. Without the link, the door gives the freeze refusal; with a link at the pack's own home, which the shadow moved away, the door also gives the freeze refusal.
  - The only cost is a two-round fix: the link first, then the freeze.
  - Any error assembling the pack-set makes the guard stand down, and the door then refuses under its own code.
- **N-g — Edge shapes of *move it in*, read and not driven:**
  - a link to an ancestor (`docs/decisions -> .` gives *move `docs` into its place*);
  - two links to one directory inside the repository;
  - an absolute link that names the repository through an aliased prefix (`/var` vs `/private/var`), which falls to *copy it in* and duplicates.
- **Leads, from before this change:**
  - `setup.inject-allowlist`'s message over a link at the settings file's place prints the host path of its outside target.
  - `uninstall`'s "removes N tracked file(s)" note prints `git -C /private/var/…`.
  - Both are law 1, and neither is this commit's.

## 3. `migrate.source-untrackable` over a symlinked source — not the same state

Driven on `fresh`:

1. **An ancestor link out of the repository** (`notes -> <outside>`; `migrate notes/HISTORY.md`) is refused with *"resolves outside the repository"*. The subject there is where the bytes are, not a write past a link.
2. **An ancestor link inside the repository** (`inrepo -> realnotes`) is **accepted**. `trackable::place_inside` (`crates/cli/src/trackable.rs:534-552`) canonicalizes every component but the last, so the task is minted over `realnotes/HISTORY.md`, the real path. Nothing is read in order to write past a link: retiring the file later removes the real path.
3. **A link at the source itself** is the `symlinked_component` check (`trackable.rs:324`), a question about the source path itself (the `store.home-not-regular-file` family).

So the path through a link is resolved to the real path or refused for being outside the repository; it is never written through. The only wrinkle is wording, and it predates this commit: the route (`crates/cli/src/migrate.rs:233-237`) says *"not reached through a symlink"*, but an in-repository ancestor link is accepted.

## 4. The byte table, the regressions and the retired key

- **The ten moved cells are exactly the class's.** I diffed `byte-table-record-cells.tsv` between `63ce9ca5` and `26474b80` with the two seconds columns blanked. The cells that changed:
  - `setup`'s 3 link cells (listing changed to `store.link-on-the-way`; still *refuses, nothing changed*);
  - `config-set-placement-root-…-at-the-roadmaps-new-directory` (listing changed);
  - `config-set-placement-root`'s 3 links at the roadmap's new home (they left the parking key, now *refuses, nothing changed*, exit 1);
  - `relocate`'s 3 links at the notes' new home (moved to `r1-relocate-refuses-a-doc-and-exits-0`, still *not the listed outcome* at exit 0).

  Nothing else changed.
- **Regressions 8 → 6: both removed because they were fixed.**
  - `config-set-placement-root-a-link-to-nothing-at-the-roadmaps-new-home` is *refuses, nothing changed* on both binaries.
  - `relocate-a-link-to-nothing-at-the-notes-new-home` is *not the listed outcome* on both — the same class, so not a regression by the record's definition.
  - The relocate cell stays red under an existing key: S1's new refusal at `relocate` exits **0** with the doc blocked, which is that key's defect.
- **The retired key `r1-leftover-consent-over-a-link-to-nothing-draws-provision-failed` was retired honestly.** Driven: with a link to nothing at `.jigc/worktrees/do-a-thing`, `milestone provision ship-it` refuses `store.link-on-the-way` (way out: *remove*). After removing the link, provision exits 0.

## Repro blocks

### NB1 — orientation rebuilds the edge index past a linked workbench

```bash
WT=<a worktree of 26474b80>; BIN=<a debug jigc built from it>
rig=$("$WT"/dev/jigc-rig --binary "$BIN" committed-singletons 2>/dev/null) || exit 1; eval "$rig"; cd "$REPO"
$JIGC start --workflow quick-fix "fix a typo" >/dev/null 2>&1
O="$RIG/outside-jigc"; mv .jigc "$O"; ln -s "$O" .jigc
echo x >> README.md; git add README.md; git commit -q -m "an ordinary commit"
ls "$O/index"                      # before: edges.json.lock only
$JIGC start | head -1              # exit 0; "advisory · store.link-on-the-way … This one only reads, and wrote nothing"
ls "$O/index"                      # after: edges.json written outside the repository
# same with only .jigc/index linked, and with `jigc task validate fix-a-typo` (exit 3)
```

### NS1 — the JSON reject arm

```bash
rig=$("$WT"/dev/jigc-rig --binary "$BIN" committed-singletons 2>/dev/null) || exit 1; eval "$rig"; cd "$REPO"
O=$(mktemp -d "$RIG/outside.XXXXXX"); mkdir -p docs; ln -s "$O" docs/decisions
$JIGC --format json doc show no-such-doc >/dev/null 2>"$RIG/e"; echo "exit=$?"   # exit=1, stdout empty
head -c 80 "$RIG/e"                                                              # "advisory · store.link-on-the-way …" then the {…} document
python3 -I -c 'import json,sys; json.load(open(sys.argv[1]))' "$RIG/e"           # fails: not one document
```

### NS2 — *move it in* through a second link

```bash
rig=$("$WT"/dev/jigc-rig --binary "$BIN" fresh 2>/dev/null) || exit 1; eval "$rig"; cd "$REPO"
O=$(mktemp -d "$RIG/outside.XXXXXX"); mkdir -p "$O/dec"; printf '# Shared\n' > "$O/dec/shared.md"
ln -s "$O" vendor; mkdir -p docs; ln -s ../vendor/dec docs/decisions; git add -A; git commit -q -m links
$JIGC start --workflow quick-fix "fix a typo"   # route: "… move the directory `vendor/dec` into its place"
rm docs/decisions; mv vendor/dec docs/decisions; find "$O"   # the outside dec/ is gone from where it stood
```

### NS3 — setup over a linked methodology home

```bash
rig=$("$WT"/dev/jigc-rig --binary "$BIN" bare 2>/dev/null) || exit 1; eval "$rig"; cd "$REPO"
O=$(mktemp -d "$RIG/outside.XXXXXX"); mkdir -p docs; ln -s "$O" docs/research; git add -A; git commit -q -m link
$JIGC setup; echo "exit=$?"                               # exit=0, nothing on stderr
$JIGC start --workflow quick-fix "fix a typo"             # store.link-on-the-way naming docs/research
```

### NS4 — `uninstall --force` through a link at the settings file's place

```bash
rig=$("$WT"/dev/jigc-rig --binary "$BIN" fresh 2>/dev/null) || exit 1; eval "$rig"; cd "$REPO"
O=$(mktemp -d "$RIG/outside.XXXXXX"); cp .claude/settings.json "$O/settings.json"
rm .claude/settings.json; ln -s "$O/settings.json" .claude/settings.json; git add -A; git commit -q -m l
shasum "$O/settings.json"; $JIGC uninstall --force; echo "exit=$?"; shasum "$O/settings.json"   # exit 0; sha changed
```

### NS5 — S1's route, followed (it works; only the catalog lacks it)

```bash
rig=$("$WT"/dev/jigc-rig --binary "$BIN" committed-singletons 2>/dev/null) || exit 1; eval "$rig"; cd "$REPO"
O=$(mktemp -d "$RIG/outside.XXXXXX"); mkdir -p notes; ln -s "$O" notes/roadmap.md
$JIGC config set placement-root notes; echo "exit=$?"     # 1, store.home-not-regular-file: "move the link … out of the doc's home"
mv notes/roadmap.md "$RIG/aside"; $JIGC config set placement-root notes; echo "exit=$?"   # 0
```

### N-a — a dry run refused

```bash
rig=$("$WT"/dev/jigc-rig --binary "$BIN" committed-singletons 2>/dev/null) || exit 1; eval "$rig"; cd "$REPO"
O=$(mktemp -d "$RIG/outside.XXXXXX"); mkdir -p docs; ln -s "$O" docs/decisions
$JIGC migrate-corpus --dry-run   # blocking · store.link-on-the-way
```

## Tools

The probes are in `<scratch>/rr.ix8f9W/tools/`. Each is one rig and one question against a named binary, and their bodies are the repro blocks above.

- **`readforms.sh <workbench-dir> <jigc args…>` and `readforms-all.sh <dir>…`** answer *"does a read form write past a linked workbench directory?"*. They build a rig and mint a task, move `.jigc/<dir>` to an outside copy behind a link, snapshot the outside tree (sha of each file), run the form, and diff. The `-all` runner covers 15 forms. It is a candidate for a mode of `link_on_the_way_axis`: run the read sweep **with a task minted, after a commit**, over a linked workbench.
- **`jsonreject.sh`** answers *"does stderr parse as exactly one JSON document on a read form's reject arm while a link stands?"*. It is a candidate for the JSON-purity suite, run with a link planted.
- **The earlier reviewer's `probe1…probe10.sh`**, re-pointed at my binary by a `sed` of their `WT=`/`BIN=` lines, for the re-runs of B1–S8.
- **The timing-insensitive diff of a byte-table record** (the earlier review's tool, re-used): blank columns 9 and 12, sort, diff. It is still a candidate for `dev/`.
- **Missing tool, again:** a rig option to plant a link at a named place, with its target inside or outside. Every probe here built one by hand.
- **A trap in my own harness, recorded:** in zsh an unquoted `$c` is not word-split, so my first sweep passed `"start --explain"` as one argument (clap exit 2). Split into an array (`read -ra`) or use `${=c}`.

<!-- end of report -->
