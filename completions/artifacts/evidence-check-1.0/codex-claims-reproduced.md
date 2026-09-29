<!-- persisted verbatim 2026-09-10 from the evidence-check-1.0 session; agent: Claude Opus 5 (1M context); see VERDICT.md -->
# Live reproduction — review E, two source-trace claims

Binary: `/Users/maurice/projects/gherrink-jigc/target/release/jigc` → `jigc 1.0.0-rc.14`
Repo HEAD: `74627547`. Corpora built with `dev/jigc-rig <state> --binary <that binary>`, evaluated in two steps.
Nothing under the repo was modified. No `cargo` was run.

---

## CLAIM 1 — VERDICT: **TRUE**

`jigc migrate <absolute-path-outside-the-repo> --as <doctype>` is accepted at exit 0, reads the
external file, records that **absolute** path verbatim as the task's `source-path`, and
`jigc task finalize <id> --approve` **permanently deletes the external file** at exit 0 — with no
refusal, no warning, and no route naming the path at any of the four steps. Reproduced on two
doctypes (`changelog`, `vision`), on a file under no repo at all, and on a **tracked file inside a
different git repository**. Two of the four sub-cases the claim bundles (a `../` relative escape and
an in-repo symlink) read outside but do **not** delete — the deletion requires the **absolute**
spelling.

### The minimal deleting sequence (4 commands, no refusal, no warning)

```
$ jigc migrate /tmp/home.Li29kv/notes.md --as vision      # exit 0
$ jigc doc author vision --from-file - --task migrate-vision-…-21258759036d   # exit 0
$ jigc task finalize migrate-vision-…-21258759036d        # exit 4  (the documented review hold)
$ jigc task finalize migrate-vision-…-21258759036d --approve   # exit 0
victim now: DELETED
total 0
drwx------@ 2 maurice staff 64 Sep 10 20:36 .
```

Nothing in any of the four outputs mentions that the path leaves the repository. The exit-4 review
hold — the one human-facing gate — says only:

```
migration review required — nothing committed. Re-run `jigc task finalize … --approve` to write
the canonical doc, retire the foreign original, and commit.
```

It never names the path being retired. The path surfaces only inside the derived task id and in the
commit subject, after the deletion has already happened.

### Repro a/b/c — absolute path, `--as changelog`, full arc

Script (self-contained; `AUTHOR` payload elided in the repeats):

```sh
rig=$(dev/jigc-rig fresh --binary "$JB") || exit; eval "$rig"
EXT=$(mktemp -d "${TMPDIR:-/tmp}/victim.XXXXXX"); V="$EXT/external-victim.md"
printf '# Changelog\n\n## 1.2.0 - 2024-01-01\n- Added a thing\n' > "$V"
$JIGC migrate "$V" --as changelog
```

Output:

```
=== BEFORE ===
-rw-r--r--@ 1 maurice staff 51 Sep 10 20:29 external-victim.md
c58dc2436123dcd378bdf372880fd63cfb1a7cdaa0c2e4a99d9375ca509f0a21  …/victim.G7pXZk/external-victim.md
migrate EXIT=0
TID=migrate-changelog-var-folders-nj-dq5nt8bj72xc0ppkm69y-ckh0000gn-f1d174ea166b
```

The recorded retirement source is the **absolute external path**, unmodified:

```
$ cat .jigc/tasks/<id>/source-path
/var/folders/nj/dq5nt8bj72xc0ppkm69y_ckh0000gn/T/victim.G7pXZk/external-victim.md
```

Then:

```
author EXIT=0
plain finalize EXIT=4         ← the review hold; victim still present, sha unchanged
=== AFTER PLAIN FINALIZE ===
-rw-r--r--@ 1 maurice staff 51 …  external-victim.md
c58dc2436123dcd378bdf372880fd63cfb1a7cdaa0c2e4a99d9375ca509f0a21   (unchanged)

approve EXIT=0
advisory · file-state.staged-copy — staged copy of `CHANGELOG.md` …
  route: no action needed …
finalized cc61343 — docs(changelog): adopt /var/…/victim.G7pXZk/external-victim.md as a managed changelog
  promoted CHANGELOG.md
  1 file committed

=== AFTER APPROVE ===
total 0                       ← directory now empty
exists? NO_DELETED
```

git side (the repo is otherwise untouched; only the promoted doc lands):

```
cc61343 docs(changelog): adopt /var/…/victim.G7pXZk/external-victim.md as a managed changelog
A	CHANGELOG.md
```

The only advisory printed on the destructive call is `file-state.staged-copy` — **"no action needed"** —
about the in-repo `CHANGELOG.md`, unrelated to the external deletion.

### Repro — same behaviour on `--as vision`

```
migrate EXIT=0
source-path: [/var/…/v.f58fnb/VISION-outside.md]
author EXIT=0 ; plain EXIT=4 ; approve EXIT=0
finalized 049e92a — docs(vision): adopt /var/…/v.f58fnb/VISION-outside.md as a managed vision
  promoted VISION.md
VISION-outside.md exists? NO_DELETED
```

Doctype-independent: the retire is planned from `source-path` regardless of target doctype.

### Repro e — deletes a **tracked** file inside a *different* git repository

```
=== e: migrate ANOTHER REPO's TRACKED file (absolute) ===
before: HISTORY.md src
migrate EXIT=0
recorded source-path: [/var/…/otherrepo.L9ah7I/HISTORY.md]
plain EXIT=4
approve EXIT=0
finalized c413a57 — docs(changelog): adopt /var/…/otherrepo.L9ah7I/HISTORY.md as a managed changelog

OTHER HISTORY.md present? NO_DELETED
OTHER git status:
 D HISTORY.md
OUR repo git status:          (clean)
OUR commit:  A	CHANGELOG.md
```

jigc deleted bytes in a repository it has no relationship with. Recoverable there only because that
repo happened to track the file; the earlier probes (a plain file under `mktemp -d`, and a personal
`notes.md`) are **unrecoverable** — `remove_file`, never a `git rm`, never a trash.

### Repro d — the sub-cases that do **not** delete

| spelling | migrate exit | recorded `source-path` | read outside? | external file after `--approve` |
|---|---|---|---|---|
| `/abs/outside/victim.md` | 0 | `/abs/outside/victim.md` (absolute) | yes | **DELETED** |
| `../external-victim.md` | 0 | `external-victim.md` (collapsed into repo) | **yes** | survives |
| `docs/../../external-victim.md` | 0 | `external-victim.md` (collapsed) | **yes** | survives |
| in-repo symlink → outside | 0 | `inside.md` | **yes** (follows link) | target survives; **the symlink is removed** |

The `..` cases:

```
############ d1 RELATIVE ../external-victim.md ############
migrate EXIT=0
task minted: migrate-changelog-external-victim-f69995db531f
recorded source-path: [external-victim.md]
plain finalize EXIT=4 ; approve EXIT=0
finalized 934e1c8 — docs(changelog): adopt external-victim.md as a managed changelog
victim exists? YES
```

The asymmetry is exactly the two lines the review cited. `repo_root.join(path)` (migrate.rs ~252)
lets an **absolute** path win outright, so the read escapes; `repo_relative_source_path`
(migrate.rs ~189) resolves `..` *lexically* with `out.pop()` on an empty buffer, so a relative escape
collapses back inside the repo and the later `repo_root.join(retirement)` in `task.rs::retire`
(~3021) targets a non-existent in-repo path — the retire silently no-ops. An absolute spelling
survives that normalizer untouched (`Component::RootDir` is pushed like any other component), so
`repo_root.join()` in the retire escapes a second time and `remove_file` lands.

**Both halves are still bugs**, even where nothing is deleted: `../` and `docs/../../` read a file
from outside the repository at exit 0, exfiltrate its content into the composed workflow, and commit
a rewrite of it. That is a read-escape the door never adjudicates.

### f — the other path/`from`/`file`/`from_file`/`target` doors

| probe | result |
|---|---|
| `jigc unmanage <abs outside>` | exit 0, `no-op: … is not managed (nothing to drop)` — harmless; never deletes |
| `jigc unmanage ../escape.md` | exit 0, same no-op |
| `jigc relocate vision --from <abs outside>/` | exit 1 — refused *for an unrelated reason*: `vision` is a frozen doctype. **Every shipped doctype in both packs is manifest-governed**, so the freeze-exempt `relocate` path is unreachable without a hand-made pack. `relocate` is a **mover**; its `from` escape is therefore **unprobed, not cleared.** |
| `jigc doc author adr --from-file <abs outside>` | exit 0, accepted (`adr:outside-sourced-decision`) — a caller-named read, defensible |
| `jigc milestone add-from-spec m1 spec:../../../..<abs>/outside` | exit 1, **`store.malformed-slug`** with a route — M50's fix holds |
| `jigc ingest` | takes no path argument (scan-only) — not applicable |
| `jigc migrate-corpus` | takes no path argument — not applicable |

So `migrate` is the outlier, and it is the only path-taking door that **deletes**.

### On the `cli.rs` premise

`ArgToken::Plain` exempts `path` on the stated premise that *"a path argument is a path the caller
means as one, adjudicated by the filesystem and by their own doors."* Driven, `migrate`'s door does
not adjudicate it: it neither confines the read to the repo nor confines the recorded retirement to
the repo, and the doc-comment on the argument itself says `<PATH>` is *"The repo-relative path of the
foreign document"* — a contract the door does not enforce. The premise is false for at least this
one member; the declared bound in `ARG_TOKENS` ("the table can be answered *wrongly*") is the shape
of the miss.

---

## CLAIM 2 — VERDICT: **TRUE**

A hook-rejected `jigc task finalize` leaves `.jigc/version` (and, when it fires, `.jigc/.gitignore`)
**rewritten in the worktree** although no commit landed. The index and HEAD are correctly restored,
so a **clean** tree becomes **dirty** across a transaction that reports `no commit was made`.
`design/finalize.md` does **not** promise otherwise — its config-layer rollback row says
*"worktree untouched"* explicitly — so this is a **design-declared** residue, not a violated promise.
It is still a real leak, and the doc's own framing (*"Before phase 6, all-or-nothing"*) sits in
tension with it.

### Repro — clean tree in, dirty tree out

```sh
rig=$(dev/jigc-rig committed-singletons --binary "$JB") || exit; eval "$rig"
$JIGC start "record a decision about caching" --workflow record-decision
$JIGC doc author adr --from-file - --task "$T" <<'EOF' …  # (adr authored)
$JIGC doc set-field "commit:$T#header/type" --task "$T" --value docs
printf 'record the single-node cache decision' | $JIGC doc set-slot "commit:$T#summary" --task "$T" --from-file -
# simulate a stamp committed by an older build; .gitignore left canonical
printf 'jigc-version: 0.9.0-OLDER-BUILD\n' > .jigc/version
git add .jigc/version && git commit -m "stamp from an older build"
printf '#!/bin/sh\nexit 1\n' > .git/hooks/pre-commit && chmod +x .git/hooks/pre-commit
$JIGC task finalize "$T"
```

Before / after:

```
=== BEFORE ===
status:                        ← blank: clean tree
version worktree: jigc-version: 0.9.0-OLDER-BUILD
version index blob: 69276bb3601d7adb9db7b27bf8750a8aee9f43a5
version HEAD blob:  69276bb3601d7adb9db7b27bf8750a8aee9f43a5

finalize EXIT=1

=== AFTER ===
status:
 M .jigc/version                ← RESIDUE
version worktree: jigc-version: 1.0.0-rc.14      ← rewritten
version index blob: 69276bb3601d7adb9db7b27bf8750a8aee9f43a5   ← restored, correct
version HEAD blob:  69276bb3601d7adb9db7b27bf8750a8aee9f43a5   ← unchanged, correct
HEAD moved? NO

git diff -- .jigc/version
-jigc-version: 0.9.0-OLDER-BUILD
+jigc-version: 1.0.0-rc.14
```

What finalize said — accurate as far as it goes, silent on the residue:

```
`git commit` was rejected (no commit was made):

task record-a-decision-about-caching is intact — nothing was committed, your task's staged docs are
still in `.jigc/tasks/record-a-decision-about-caching/docs/`, and anything you had `git add`-ed is
still in git's index. Fix the hook's complaint, then re-run `jigc task finalize …`.
```

### The wider snapshot (first run, both files perturbed)

```
=== AFTER (index) ===  INDEX IDENTICAL
=== AFTER (status) ===
 M .jigc/.gitignore
 M .jigc/version
=== AFTER (.jigc file shas) — diff vs before ===
< b3b44cc7…  .jigc/.gitignore      > 16a57306…  .jigc/.gitignore     ← changed
< cfbc607a…  .jigc/index/edges.json > 47179e7b… .jigc/index/edges.json ← changed (gitignored cache)
< ada55102…  .jigc/version          > 6a0ef1bd… .jigc/version         ← changed
=== HEAD moved? === HEAD UNCHANGED (no commit landed)
=== docs/ === docs/decisions: No such file or directory      ← promotion correctly rolled back
```

Everything the rollback *claims* to cover is covered: HEAD unmoved, index byte-identical, the
promoted ADR removed from the worktree, the task's staged docs intact. What moves is exactly the
two worktree writes the claim names, plus the gitignored `index/edges.json` cache.

### What the design doc actually says

`design/finalize.md` → *Rollback discipline*, the config-layer row (M45 audit):

> Capture the pre-finalize index entry of every config-layer path before staging; on failure restore
> the present ones (`update-index --cacheinfo`, **worktree untouched**) and **drop** any the stage
> newly added…

and `task.rs::rollback_config_layer_index`'s own doc-comment:

> …**without touching the worktree**, so a `.jigc/version` `refresh_version_stamp` rewrote on disk
> survives while `git show :.jigc/version` == the pre-finalize blob…

So the behaviour is **stated, deliberate and consistent** with the doc. The claim's factual half is
TRUE; its implied "the doc promises otherwise" half is FALSE. The residue is what the design chose.

Two things still worth weighing against that choice:

1. The section header two rows above reads **"Before phase 6, all-or-nothing."** For the config
   layer it is all-or-nothing *in the index only*. A reader of that sentence would not predict a
   dirty tree.
2. The row's own justification — *"a real leak once the running build's stamp differs from the
   committed one"* — is the case that produces the residue. The fix closed the **staged** leak and
   left the **worktree** one, and the surviving half is reachable by the ordinary sequence
   *upgrade jigc → finalize → hook rejects*, with no perturbation needed.

`.jigc` tracked set, for the record (`.jigc/.gitignore` covers `tasks/ index/ state/ milestones/
worktrees/ logs/ displaced/`):

```
.jigc/.gitignore   .jigc/AGENT.md   .jigc/config/.gitkeep   .jigc/config/packs.yaml   .jigc/version
```

---

## Adjacent finding, driven, in neither claim: `gitignore::ensure` destroys user lines and commits it

Not part of either claim, found while snapshotting for Claim 2, so recorded with its evidence.

`crates/cli/src/gitignore.rs::ensure` **replaces the whole file** with the canonical `ENTRIES` set
whenever any one entry is missing. Its own doc-comment says the opposite:

> …so an adapter-written `.gitignore` predating any later entry — `milestones/`, `worktrees/`,
> `logs/` — is **amended once to the union**.

It is not amended. It is overwritten.

Control (all canonical entries present + user lines → correctly a no-op):

```
BEFORE: tasks/ index/ state/ milestones/ worktrees/ logs/ displaced/ # my own note  my-private-dir/
finalize EXIT=0
AFTER:  tasks/ index/ state/ milestones/ worktrees/ logs/ displaced/ # my own note  my-private-dir/
```

Positive case (one canonical entry — `displaced/` — absent, e.g. a `.gitignore` written by a jigc
older than M39, plus user lines):

```
BEFORE:
tasks/ index/ state/ milestones/ worktrees/ logs/
# my own note - I keep secrets here
my-private-dir/

finalize EXIT=0
finalized b3c3aeb — docs: note a decision
  modified .jigc/.gitignore
  promoted docs/decisions/some-decision.md
  2 files committed

AFTER (worktree AND HEAD:.jigc/.gitignore):
tasks/ index/ state/ milestones/ worktrees/ logs/ displaced/
```

`# my own note` and `my-private-dir/` are gone from the worktree and from the commit. The ack prints
`modified .jigc/.gitignore` — so it is *surfaced* — but names no content loss, and the file is
tracked, so the loss is recoverable from history. Severity is low (recoverable, surfaced), but the
doc-comment is a law-1 falsehood about what the writer does.

---

## Summary

| claim | verdict | driven consequence |
|---|---|---|
| 1 — `migrate` reads and deletes outside the repo | **TRUE** | External file **permanently deleted** at exit 0 in 4 commands, no refusal/warning/route. Absolute spelling deletes; `../` and symlink spellings read-escape only. Confirmed on `changelog` and `vision`, on an unversioned file and on a tracked file in another git repo. |
| 2 — finalize rollback leaves worktree bytes changed on a hook rejection | **TRUE** (behaviour) / doc promises nothing else | Clean tree → ` M .jigc/version` (+ ` M .jigc/.gitignore` when `ensure` fires) after `no commit was made`. Index and HEAD correctly restored. `design/finalize.md` says *"worktree untouched"* on purpose. |
| bonus — `gitignore::ensure` overwrite | driven TRUE | User lines in tracked `.jigc/.gitignore` silently replaced and committed; the doc-comment claims "amended to the union". |

Unprobed and explicitly not cleared: `jigc relocate --from` with an escaping path — a **mover** whose
freeze-exempt branch no shipped doctype can reach, so it needs a hand-made pack to drive.
