# verify-real — `r1-doc-list-prints-id-doc-show-refuses` (run canary-one, round 1, attempt 1)

- **key:** `r1-doc-list-prints-id-doc-show-refuses`
- **door:** `jigc doc list`
- **clause it is said to break:** `working-product` (triage's grade: *unclear*)
- **verdict:** `refuted`
- **basis:** `intended` — the listing's `unregistered` row for a name that is not a doc id, and the `store.malformed-slug` refusal routed at `jigc doc list`, are the state DECISIONS.md → *2026-09-05 — M50 Inc 2 / T1* settled and a standing suite holds; and the refusal's route, run as printed, exits 0 and prints the listing its sentence names, so no clause is broken either.
- **contested:** `false`
- **regression:** not established — step 4 runs with `confirmed` only. The previous release's binary was hashed and not driven.
- **class:** `instance, unbounded` (what was driven is listed under *What I drove*; I enumerated no consumer set of the mechanism).

`<W>` below is my own scratch directory, `<scratch>/verify-p1.zXHXC0`, minted with `mktemp -d` under the scratch root the prompt names.

## The binary

| check | result |
|---|---|
| `dev/stabilize-step hash --scratch <scratch> --file bin/c1.a1/jigc` | `content_sha256` = `dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc` — the hash the prompt gives |
| the binary's directory first on `PATH`, then `command -v jigc` | printed `<scratch>/bin/c1.a1/jigc`, exit 0 — in every shell that drove anything |
| `dev/stabilize-step hash --scratch <scratch> --file bin/previous-91834b5e011d/jigc` | `content_sha256` = `accf3996ae4a66d13adb4bb03253606e0037b5ddb7be653a314a908f6b88ab5d` — the hash the prompt gives; **not driven** |

Candidate: commit `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`, label c1. No build ran; nothing under `target/` was driven.

## What I drove

Two fresh rigs, both `dev/jigc-rig bare --binary <scratch>/bin/c1.a1/jigc` with `SCRATCH=<W>`, both followed by `jigc setup` (exit 0) as the block writes it. The rig's `bare` state stands in for the block's `git init` + one empty commit: a git repository with one commit and no setup, under a home directory that is not the machine's. That is the one change to the block's setup.

**Rig 1 — the block as written, then its route.** Every exit status read bare; stdout and stderr captured to separate files.

| step | argv | exit | stdout | stderr |
|---|---|---|---|---|
| plant | `docs/research/UPPER.md` = `# Upper\n`, `docs/research/has space.md` = `# Spaced\n`; neither added (`git status --short` shows both `??`) | - | - | - |
| 1 | `jigc doc list` | 0 | the three lines below | empty |
| 2 | `jigc doc show research:UPPER` | 1 | empty | the two lines below |
| 3 | `jigc doc list` — the route of step 2, as printed | 0 | byte-identical to step 1 (`cmp` exit 0) | empty |

Step 1 and step 3, stdout:

```
id  path  state
research:UPPER  docs/research/UPPER.md  unregistered
research:has space  docs/research/has space.md  unregistered
```

Step 2, stderr:

```
blocking · store.malformed-slug — "UPPER" is not a valid doc slug — the `<slug>` head of address `research:UPPER`
  route: `jigc doc list` lists the committed docs and the identity each one carries; use lowercase letters, digits, and single hyphens (no leading, trailing, or doubled `-`)
```

Both were compared byte for byte (`cmp`, exit 0) with the strings the reporter's block expects. `git status --short --untracked-files=all` was the same two `??` lines before step 1 and after step 3.

**Rig 2 — the neighbours, so the verdict does not rest on one cell.** The same two plants plus a control, `docs/research/lower.md` = `# Lower\n`.

| label | argv | exit | what it printed |
|---|---|---|---|
| show-space | `jigc doc show 'research:has space'` | 1 | `store.malformed-slug` for `"has space"`, the same route |
| show-lower-control | `jigc doc show research:lower` | 1 | `store.unparseable`, route *adopt — run `jigc ingest` to route it, or `jigc migrate <path> --as research`* |
| show-upper-json | `jigc doc show research:UPPER --format json` | 1 | `{ "error": … }` on stderr, carrying the same two lines |
| validate | `jigc validate` | 1 | three `schema-conformance.unadopted-instance` rows; for the two planted names: *its name is not a doc id, so no `<type>:<slug>` address reaches it*, each with the adoption route |
| ingest | `jigc ingest` | 0 | the two planted names `needs-reconcile`, `conformance.section-missing`, route `jigc migrate <path> --as research` |
| committed | the two files added and committed, then steps 1 to 3 again | 0 · 1 · 0 | the same rows and the same refusal as untracked |
| list-json | `jigc doc list research --format json` | 0 | `"id": "research:UPPER"`, `"state": "unregistered"`, `"item-count": 0`, `"fields": null` |

## Step 2 — is it what the finding says?

**It reproduces, whole.** An untracked `docs/research/UPPER.md` is listed as `research:UPPER`; `jigc doc show research:UPPER` exits 1 with `store.malformed-slug`; its route names `jigc doc list`. I tried the four ways a repro like this is usually wrong and none applied: the rig and the scratch root are mine and new; no command whose status I read ran through a pipe; nothing was cut; the binary is the candidate by hash and by `command -v`.

**It is the behaviour a settled decision intends.** DECISIONS.md → *2026-09-05 — M50 Inc 2 / T1: the slug head of a caller-typed address is checked before it names a path*:

- the refusal and its route are that entry's: *one blocking finding (`store.malformed-slug`) … one route (`jigc doc list`, the family's discovery verb …)*;
- the entry met exactly the pair this finding describes, and ruled on it. Its *disposition arm* paragraph: a hand-dropped file at a managed home whose name is not a doc id answered `research:OddName … managed` — *a `managed` row for an identity every door refuses, and the sharpest form of the lie, since the refusal's own route names `jigc doc list`*. The repair it chose was that the listing answers **`unregistered`** for such a file, through the one discriminator (`is_unadopted_foreign`'s identity leg), and that the store sweep routes it to adoption. So what the route's reader finds back at the listing — the same row, in state `unregistered` — is what that entry decided the reader should find.
- the suite holds it: `address_slug_head_axis::no_surface_calls_an_unaddressable_identity_managed` plants `docs/research/OddName.md` and `docs/decisions/OldDoc.md`, finds each listing row **by its `research:OddName` / `adr:OldDoc` prefix**, and asserts it ends `unregistered`; `address_slug_head_axis::every_address_door_refuses_a_malformed_slug_head` holds the refusal's code, its one route and the grammar sentence at every address door.

The design section that owns the row says the same: design/doc-read-surface.md → the `doc list` contract — a row is emitted for every instance the enumerator yields, `unregistered` is *a foreign file squatting at a canonical home, or a never-ingested doc*, and `id` is `null` on an `orphaned` row only. design/validation.md's `ingest.unaddressable-identity` row names the class (*no `<type>:<slug>` address reaches the file*) and puts its repair at the doors that adopt or move a file, never at the listing.

**The finding does not argue that decision wrong.** It files `clause: none as read` and calls the pair *a loop all the same*, without naming the decision. So `contested` is `false`. What the finding adds to the decision's own record is two cells its fence did not drive — a name with a space, and a plant that is untracked — and both behave as the fenced cell does (rig 2, *committed*).

## Step 3 — does it break the clause, inside its scope?

**No.** The clause is DECISIONS.md → *2026-10-04 — The exit rule, revised*, second clause, as the opening record's `working-product`: *no command that works on rc.24 in a supported layout stops working, and every refusal's route works as printed*.

- **The route works as printed.** It is one command, `jigc doc list`. Typed as printed in the repository that printed it, it exits 0 with an empty stderr and prints what its sentence says it prints: the docs at the managed homes and the identity each row carries. It needs no placeholder filled, no quoting, no other directory.
- **This is the sense the record uses the words in.** A route fails *as printed* there when its command does not run or dead-ends on the door it names: a step's `jigc doc show adr:<slug> --task <id>` *unrunnable as printed* because nothing supplies `<slug>`; a `Spawn:` line that answers *no task*; a linked-worktree guard *with every printed route a dead end*. This route is none of those.
- **What is true and is no part of the clause:** the route does not, by its own command, move this reader forward. The row it returns them to repeats the identity that was refused, and the plain listing prints no route on a row. The way out is on two other printed surfaces of the same binary — `jigc validate` (exit 1) names the file, says *no `<type>:<slug>` address reaches it* and routes to adoption, and `jigc ingest` (exit 0) routes it to `jigc migrate <path> --as research`. A reading of the clause under which a route must also *advance* the reader would be a new reading of the human's rule; I did not apply one.
- **Scope.** The state is ordinary — a hand-written file at a doctype's home — so the scope excludes nothing here. It is the clause's content the finding does not reach.

Had step 2 not settled it, the basis would have been `breaks-no-clause`.

## Step 5 — the coverage claim (`pinned-by: "UNPINNED: found this round"`)

Half true, read from the suites and not from any diff. Method: the suite files under `crates/cli/tests` and `tooling-tests` that spell the argv `"doc", "list"` — **43** files; of those, the ones that also name `store.malformed-slug` — **4** (`address_slug_head_axis.rs`, `blocked_finding_log_axis.rs`, `ingest.rs`, `relocate.rs`); and, among the 43, a literal `docs/<home>/<name>.md` whose name carries an upper-case letter, a space or `_` — **2** files (`address_slug_head_axis.rs`: `OddName`, `OldDoc`; `ingest.rs`: `My Decision`, `___`). A suite that composes the argv or the path some other way is outside this count.

- **Held:** the listing row's state for a committed, space-free, mixed-case name, found by its id prefix (`no_surface_calls_an_unaddressable_identity_managed`); that `ingest`'s refused file is not listed `managed` (`ingest::ingest_refuses_a_conformant_doc_whose_name_is_not_a_doc_id`); the refusal's code, single route and grammar sentence (`every_address_door_refuses_a_malformed_slug_head`, cells `../..`, an absolute path and `Odd Name`).
- **Not held by any test I found:** the three steps in sequence with the route run after the refusal; an **untracked** plant; a name with a **space** in the listing (the fence's fixture is space-free on purpose, its doc comment says why); the literal `jigc doc list` inside the route's text.

## Repro V-1

```yaml
claim: "an untracked docs/research/UPPER.md is listed `research:UPPER … unregistered`; `jigc doc show research:UPPER` exits 1 `store.malformed-slug` routed at `jigc doc list`; that route, run as printed, exits 0 and returns the same listing — the state M50 Inc 2 / T1 settled, and no clause broken"
verdict: REFUTED
basis: intended
binary: candidate c1, commit eeffe347324f83a51d1ae83d5f254e73c3f1ea3a, sha256 dded1facfd8986baa3c0bcb8c2389d873fdc0b5806f0e1d148a916c1b8beadfc
setup:
  - fixture: fresh            # the rig's `bare` + ["jigc", "setup"] is what was driven
  - "write docs/research/UPPER.md = `# Upper\n` and docs/research/has space.md = `# Spaced\n`; add neither"
repro:
  - ["jigc", "doc", "list"]
  - ["jigc", "doc", "show", "research:UPPER"]
  - ["jigc", "doc", "list"]          # the route step 2 printed, as printed
expect:
  - exit: 0
    stdout: "id  path  state\nresearch:UPPER  docs/research/UPPER.md  unregistered\nresearch:has space  docs/research/has space.md  unregistered\n"
    stderr: ""
  - exit: 1
    stdout: ""
    stderr: "blocking · store.malformed-slug — \"UPPER\" is not a valid doc slug — the `<slug>` head of address `research:UPPER`\n  route: `jigc doc list` lists the committed docs and the identity each one carries; use lowercase letters, digits, and single hyphens (no leading, trailing, or doubled `-`)\n"
  - exit: 0
    stdout: "byte-identical to step 1"
    stderr: ""
    assert: "no row ends `managed`"
  tree: "`git status --short --untracked-files=all` is the same two `??` lines before step 1 and after step 3"
observed: "<W>/jigc-rig-bare-JCAeAm (c1, c2, c3); neighbours in <W>/jigc-rig-bare-tpcYA3 (d-*, e-*)"
pinned-by: "UNPINNED: no test runs the route after the refusal, plants the file untracked, or lists a name with a space; the halves are held by address_slug_head_axis::no_surface_calls_an_unaddressable_identity_managed and address_slug_head_axis::every_address_door_refuses_a_malformed_slug_head"
```

**Pinnable as it stands: yes.** Three argv steps on the `fresh` fixture and two file writes; every expected byte is free of a host path and of a task id; nothing in it depends on the volume's case sensitivity. It fits `address_slug_head_axis.rs` as a second arm beside the disposition arm.

## Left open — hit on the way, not pursued

1. **A lower-cased address reaches the upper-cased file on this volume, and the refusal prints a path no file has.** Rig 2, `jigc doc show research:upper` — the route's own grammar hint taken literally — exits 1 with `store.unparseable`, saying that `research:upper` at `docs/research/upper.md` does not parse; the only file there is `UPPER.md`. The volume is case-insensitive. One cell, one volume; a case-sensitive one was not driven.
2. **`jigc validate` calls an untracked file a *committed file*.** Rig 2, before anything was added: *committed file `docs/research/UPPER.md` sits at the `research` home but was never adopted*, while `git status` showed it `??`. Wording only, as far as I looked.
3. **The adoption route for a name that is not a doc id was not run.** `jigc validate` and `jigc ingest` both hand `docs/research/UPPER.md` and `docs/research/has space.md` the route `jigc migrate <path> --as research`; over these non-conformant files `ingest` answers `conformance.section-missing`, and `ingest.unaddressable-identity` with its `git mv` route did not appear. Whether that `migrate` lands such a file under an addressable name is undriven — the reporter left the same cell open.
4. **A design question the finding sits next to, raised by nobody yet.** design/doc-read-surface.md gives the `orphaned` row an unpasteable identity so that the listing never prints *a synthesized `<type>:<slug>`, which would be an address `doc show` refuses*; the `unregistered` row for a name that is not a doc id prints exactly such an address, in the plain arm and as the pinned json's `id`, while `jigc validate` says of the same file that no address reaches it. Under the M50 entry this is as decided; whether the later rule should reach this row is a design fork, not a verdict of mine.

## Tree state

Repository: branch `fix/canary-one` at `eeffe347324f83a51d1ae83d5f254e73c3f1ea3a`; `git status --short` shows the run's untracked `completions/artifacts/canary-one/r1/` and nothing else, before and after. I built nothing, edited nothing, staged nothing and committed nothing in it. The only commits made are inside my two rigs under `<W>`: the install commit `jigc setup` makes in each, and one of mine in rig 2 for the *committed* row.

<!-- end of report -->
