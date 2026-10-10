# Route registry facts — what the code enumerates about refusals and their routes

Read-only fact-finding on `fix/rc24-tier1` at `95f77b57` (product tree last changed at `1c7d391c`
for `crates/cli/src/setup.rs`). Nothing was built, run, or driven. Every count below is a static
scan of source text; the method is stated with each number.

## How the counts were made

One Python scan over every `.rs` under `crates/cli/src` and `crates/engine/src`:

- **Production code only.** Every `#[cfg(test)]` item is blanked by brace matching (string- and
  comment-aware). The workspace interleaves test modules with production code
  (`crates/cli/tests/support/rust_source.rs:14` says so), so "first `#[cfg(test)]` to end of file"
  is wrong; a first pass that used it undercounted by 10 sites and was redone.
- **Line comments blanked**, so a code named in a doc comment is not counted.
- **A construction site** is a call to `Finding::graded(`, `Finding::block(`, `Finding::blocking(`,
  or a `Finding { … }` literal carrying `severity:` and `code:`. Arguments are split by balanced
  parentheses. The code argument is read as a string literal, a constant (resolved by a second scan
  for `const NAME: &str = "…"`), or an expression (not resolved).
- **The route argument** is classified by its text: `Route::mechanical(…)`, `Route::human(…)`,
  `Route::informational(…)`, a bare string / `format!` / `.into()` (which `From<String>` turns into
  a human route), a variable or helper call (kind not read), or `None`.
- **Code-like literal**: any `"a.b"` or `"a.b.c"` string in production code made of lowercase words
  and hyphens, minus file extensions. An upper bound on the code vocabulary.

Reliability: site counts are exact for the four constructor shapes and blind to a finding built any
other way. The route kind of the 34 "variable" sites is not known. Two scan helpers were
spot-checked against source by hand; no per-site audit was done.

---

## 1. Is there a registry of refusals / findings?

**No single one. A finding code is a free `String`.**

- `Finding` is `crates/engine/src/finding.rs:1584`; its `code: String` is `:1602`. There is no enum
  of codes. Constructors: `blocking` `:1771` (no route), `block` `:1804`, `graded` `:1828`.
- **257 production construction sites**: 161 `graded`, 79 `block`, 13 `blocking`, 4 struct literals.
- **Codes at those sites**: 186 are string literals, 61 are constants, 10 are expressions
  (`kind.code()`, `refusal.code()`, `self.member.code()`, `door.code` — at
  `crates/cli/src/author.rs:210`, `rename.rs:286`, `relocate.rs:182`, `repo.rs:875`,
  `rollback.rs:862`, `crates/engine/src/cascade.rs:158`, `:365`, `finalize.rs:2621`).
  After resolving constants: **175 distinct codes**. 24 sites keep a dynamic code.
- **289 distinct code-like literals** in production overall. The gap to 175 is codes that reach a
  constructor through a helper or a `code()` method (`workflow-refs.*` 33 literals against 6 at a
  constructor; `migrate-corpus.*` 13 against 1; `rename.*`, `relocate.*`, `repo.*`,
  `structural-target.*`, `slot-fill-target.*` all 0 at a constructor). The 289 includes at least
  five identifiers that are not finding codes (`finalize.fan-out`, `finalize.fan-out.squash`,
  `finalize.left-out`, `read.staged-read-back`, `create.singleton-copy-in`) and the 12 error
  identities below.

**Partial registries that do exist** (each a `const` table, each covering one family):

| table | where | rows | what a row carries |
|---|---|---|---|
| `CHECK_INVENTORY` | `crates/engine/src/result.rs:331` | 31 | `(probe, check)` — the severity-tunable checks only; private, projected by `check_inventory_codes()` |
| `ERROR_CODE_REGISTRY` | `crates/cli/src/invocation_log.rs:275` | 12 | error identities that are *not* findings |
| `COMMITTING_DOORS` | `crates/cli/src/invocation_log.rs:196` | 11 | **door → code** → commit clause |
| `FINALIZE_FAMILY` | `crates/cli/src/render.rs:1582` | 21 | code, producer module, subject |
| `AMBUSH_CONTRACTS` | `crates/cli/src/pack.rs:989` | 8 | **code, door, producer site**, disposition |
| `CONFORMANCE_ROUTE_CODES` | `crates/engine/src/validate.rs:3421` | 5 | the gate codes `conformance_route` answers |
| `CONFORMANCE_PARSE_DIAGNOSTICS` | `crates/engine/src/finding.rs:328` | 13 | the codes exempt from carrying a route |
| `ENVELOPE_OWED_CODES` | `crates/cli/src/render.rs:6444` | 5 | codes owed the findings envelope |
| `STORE_EXIT_FLIPS` | `crates/cli/src/render.rs:1092` | 7 | store-scope findings that flip the exit |
| `ROLLBACK_DOORS` | `crates/cli/src/rollback.rs:527` | 5 | door → `*.rollback-conflict` code |

**One family is derived from the source, not hand-listed.**
`crates/cli/tests/finalize_family_registry.rs` scans production source for `Finding` constructors
in the `finalize.` namespace and requires the `(code, module)` set to equal `FINALIZE_FAMILY`
exactly. Its header records that four hand counts of that family were all wrong before it. It
carries its own scanner (`scan_producers`, `:201`); a shared source reader also exists,
`crates/cli/tests/support/rust_source.rs`, used by 22 other suites.

**Code → door.** By data only in `COMMITTING_DOORS` (11), `AMBUSH_CONTRACTS` (8), `ROLLBACK_DOORS`
(5), and the two crossed axes in section 3. Everywhere else a code's doors are known only by
reading call sites.

**Doors are enumerable.** The clap tree; `VERB_KINDS` `crates/cli/src/cli.rs:2033` (48 leaf verbs);
`BEHALF_DOORS` `cli.rs:2203` (61 rows, every acting row carrying a runnable argv);
`crates/cli/tests/support/leaf_argv.rs` (`MINIMAL_ARGV`, one runnable argv per leaf).

*Implication for enumerating routes:* the door axis exists as data; the refusal axis does not. It
can be derived by a constructor scan — the repository already does that for one namespace — but a
scan sees 175 of roughly 270 codes directly and needs the helper and `code()` paths followed for
the rest.

---

## 2. Is there a registry of routes?

**There is a type and a seam, not a list.**

- `Route` — `crates/engine/src/finding.rs:717`, private fields. `RouteKind` — `:731`, **three
  kinds**: `Mechanical { argv, tail }`, `Human`, `Informational`.
- Constructors: `mechanical` `:774`, `human` `:801`, `human_echoing_caller_token` `:823`,
  `informational` `:833`. `From<String>` `:1527` and `From<&str>` `:1534` make a `Human` route.
- Text: a mechanical route's text is composed once at `:792` — backtick, `argv.join(" ")`,
  backtick, tail. A human or informational route's text is the producer's string verbatim. Shared
  renderers for spans: `shell_token` `:874`, `shell_operand` `:900`, `git_at` `:939`, `migrate_at`
  `:969`.
- The kind never reaches the wire: `Serialize` `:1541` writes the flat string. Printed as a
  `  route: ` line at `crates/cli/src/render.rs:4513` (and `:5279`, `:5295`, `:5311`).
- Placeholders a mechanical argv may carry: `ROUTE_PLACEHOLDERS` `finding.rs:401` and
  `DUMMY_SUBSTITUTIONS` `crates/cli/src/route_fence.rs:22` — 11 tokens, fenced equal in order.

**Is every refusal forced through it?** Every `Finding` route is a `Route` (`Finding.route:
Option<Route>`, `:1612`), so every one passes the constructor. That is weaker than it sounds:

| route at a `Finding` construction site | sites |
|---|---|
| bare string or `format!` → `Human` by conversion | 148 |
| explicit `Route::human` | 39 |
| explicit `Route::mechanical` | 16 |
| explicit `Route::informational` | 4 |
| a variable or helper result — kind not read | 34 |
| no route | 16 |
| **total** | **257** |

So of 241 routed sites, **187 (78%) are prose with no argv**, 16 (7%) carry an argv, 34 are
unknown. Of the 186 prose routes whose text is at the call (one of the 187 is a variable), 87
contain a literal `` `jigc …` `` or `` `git …` `` span and **99 contain none** — "ensure the repo's
git hooks directory is writable", "remove the link at `<path>`".

Production calls to the constructors, wherever they sit: 101 `mechanical`, 89 `human`, 6
`informational`. Of the 101 mechanical: 16 directly in a `Finding` constructor, 51 bound to a
variable or returned, **34 interpolated into a `format!` / `anyhow!` / `bail!` / `eprintln!`
string** — a checked span inside free text, with no finding and no code.

**Refusals outside the seam** (the operational-error funnel, exit 1, free text):

- 106 `bail!` / `anyhow!` / `ensure!` sites and 336 `.context(` / `.with_context(` sites.
- Of the 106: **43 are "`` `git …` `` failed: <git's stderr>" and name no next step at all**
  (mostly `crates/cli/src/task.rs` 23 and `milestone.rs` 14). About 17 carry a next step as free
  text — 4 with a literal command span (`doc.rs:2897`, `start.rs:2684`, `:2718`, `pack.rs:1639`)
  and 13 with direction words only. Three hand-write their own `\n  route: …` line into an error
  string: `crates/cli/src/migrate.rs:191`, `:439`, `crates/cli/src/doc.rs:7253`.
- clap usage errors print tips from three tables: `CURATED_SIBLING_TIPS` `cli.rs:3863` (5 rows),
  `FORECLOSED_ARGUMENT_TIPS` `cli.rs:4147` (2), `PARENT_READ_ANSWERS` `cli.rs:3815` (5).

*Implication:* "run its route as printed" is mechanical for 16 to 50 sites. For the other four
fifths the printed route is a sentence, and following it means deciding what the sentence says to
do — which is where the confirmed defect in section 6 slipped past an existing test.

---

## 3. What already checks that a route works?

### Fences at construction — all shape, none executes, all debug-only

Every one is under `#[cfg(debug_assertions)]` or `debug_assert!`; a release binary carries none.
`main` installs the validator at `crates/cli/src/main.rs:43`.

| fence | where | what it asserts |
|---|---|---|
| parse | `finding.rs:782`, `route_fence.rs:84` | a mechanical argv leads with `jigc`, every token is shell-safe, and it parses through clap with placeholders swapped for dummy values. It never runs and never sees the finding |
| span | `finding.rs:1463` on all three constructors | every backticked `git`/`jigc` span has shell-safe tokens; a `git` span with a path operand leads with `-C <absolute>` (`unaimed_git_span` `:1077`); a `jigc migrate` span has an absolute operand (`unbased_migrate_span` `:1216`) |
| subject | `finding.rs:1870`, in `graded` only | a span naming the finding's own address names it quoted |
| floor | `finding.rs:1654`, at `Serialize` | a blocking finding carries a route, unless its code is one of 13 exempt `conformance.*` |
| followability | `finding.rs:1666`, at `Serialize` | no derivable placeholder (`<address>`, `<doctype>`) is left in a mechanical argv |

`design/surface-contract.md:220` states the limit itself: the asserts hold "via the suite", and
their enforcement "is exactly as strong as the suite's traffic through the seam" — the M43 audit
found four producers violating the floor under a green suite.

### Suites that execute a printed route against a real state

There is no shared harness. `crates/cli/tests/support/` offers only the split
(`support::shell_words`, `support/mod.rs:142`). Each suite defines its own lifter and runner:
**36 private definitions** of `run_emitted`, `run_route`, `run_span`, `route_argv`,
`route_command`, `follow_the_routes` and similar, and **30** of `backticked`, `jigc_spans`,
`route_lines`, `spans` and similar (counted by `grep` over function definitions).

Suites whose own header says they lift the emitted bytes and run them verbatim:

| suite | tests | what it runs |
|---|---|---|
| `route_followability.rs` | 3 | `write.wrong-shape`, `write.not-present`, and the 3 mechanical gate repair codes × the 3 `BOUNDARY_DOORS` — both sides read from code; asserts exit 0 and the finding cleared |
| `no_such_task_route.rs` | 6 | `finalize.no-task` at three doors, two shapes |
| `write_not_present_route.rs` | 5 | `write.not-present` → the emitted `doc show` |
| `migrate_route_family.rs` | 6 | every production mention of `migrate-corpus` must carry a row; **2 rows driven**, the rest argued |
| `pre_guard_repair_route.rs` | 9 | `conformance.item-heading-unanchored` at three carriers, repair lifted from the message |
| `posture_door_axis.rs`, `flow53_acceptance.rs` | 7, – | `repo.*` × every acting `BEHALF_DOORS` row × `GitState::ALL`; the route concludes the operation |
| `commit_rejected_axis.rs` | 9 | the 11 `COMMITTING_DOORS` under a rejecting hook; one test re-runs the printed command through a real shell |
| `file_state_history_gate.rs`, `finalize_render_io_absent.rs`, `create_only_gate.rs`, `copy_in_baseline.rs`, `fixed_identity_axis.rs`, `milestone_promote_guards.rs` | – | one family each |

**Heuristic count** (a test function whose within-file call closure mentions `route`, lifts from
output — `["route"]`, `route:`, a backtick split — and splits what it lifted through
`shell_words`, `shell_split`, `split_whitespace` or `sh`): **170 of 2,796 test functions, in 58 of
468 suite files**. Codes named inside those functions: **43**. Codes named anywhere in those 58
files: 117. Neither bound is tight: the function-level count misses codes a test reads from a
table (the 11 committing-door codes, the 3 posture codes); the file-level count credits a suite
for codes it only mentions; and "lifts and splits" includes tests that run a composed `Run:` line
rather than a refusal's route.

Of those 43, **none is a `setup.*` code and one is an `uninstall.*` code**. The canary's two
blockers and three more of its confirmed findings are at `jigc setup` and `jigc uninstall`.

### Suites that check shape only

`anyhow_route_spans.rs` (16 tests) asserts emitted stderr bytes; `error_remediation.rs` (5) asserts
a route is present; `not_in_repo_axis.rs` drives every leaf outside a repository and asserts one
text; `repo_relative_paths.rs`, `git_span_aim.rs` scan spans. `verb_suite_coverage.rs` proves only
that each leaf verb is *named* by some suite, and says so.

**83 of the 289 code-like literals are named in no file under `crates/cli/tests/`** — among them
`uninstall.remove-precommit` (a confirmed canary finding), six other `uninstall.*`, five
`setup.*`. Unit tests inside `src` were not scanned for this, so "untested" does not follow.

*Implication:* running a printed route is an established practice here, done per suite with
copied helpers, covering somewhere between 15% and 40% of the codes by two loose bounds, and
thinnest exactly where the canary found the regressions.

---

## 4. Provoking states

- **Named states**: `State::ALL` at `crates/cli/tests/support/trial_corpus.rs:176` — six (`fresh`,
  `committed-singletons`, `migrated`, `refs-post-hoc`, `chatty-hooks`, `vendored`). `dev/jigc-rig`
  builds the same six plus `bare`. Each is a state an adoption trial hit; none is keyed to a
  refusal. `dev/jigc-rig` contains no finding code at all.
- **No table maps a code to a state and a command.** A search for a must-refuse matrix or a
  refusal census found none.
- **What exists instead is per-suite, private door tables**, each with its own planted states:
  `repo_relative_paths.rs:258` (`DOORS`), `:399` (`READ_DOORS`), `:651` (`PIN_DOORS`);
  `blocked_finding_log_axis.rs:67`; `text_json_parity_axis.rs:128`.
- **Registry-crossed axes that do provoke refusals from data**: `BEHALF_DOORS` × `GitState::ALL`
  (`support/git_state.rs`) → the three `repo.*` codes; `COMMITTING_DOORS` × a rejecting hook → 11
  codes; `conformance_repair_codes()` × `BOUNDARY_DOORS` → 3 codes × 3 doors;
  `WORK_UNIT_ID_DOORS` (27), `SLUG_DOORS` (6), `PATH_ARG_OCCURRENCES` (14) for argument-shape
  refusals; `MINIMAL_ARGV` for the not-in-a-repository precondition.
- **`implementation/pinning.md`** (`:15`–`:31`) enumerates *workflows and doctypes* from the
  loaded pack registry for the compose goldens, *advertised addresses* from `doc schema` for the
  address round-trip, and *outcome classes per verb family* against one exit-code table. It
  enumerates no refusals. `:139`–`:143` warns that a test asserting emitted route text "pins a
  route a later wave wants to change".

*Implication:* the provoking state for a given refusal lives, where it exists, inside the test
that happens to cover it. A walk over refusals has no state table to read and would have to build
one — and the registry-crossed axes above show the shape the repository already accepts for
that.

---

## 5. The regression set that exists

`dev/regression-set` builds the previous release's own test suite at its commit, runs it once on
that release's binary, swaps the candidate's binary into the path the tests compiled in, and runs
the same test binaries again — 4,393 tests in the recorded runs
(`completions/artifacts/M55/stabilization-build/regression-set/README.md`). It compares the two
`nextest` reports: a test that passed on the old binary and fails on the new one must be a row of
a committed list of intended changes, or the check is red. It defines no test, plants no state and
follows no route of its own; it follows a printed route only where an rc.24 test already did, in
the states that test built — and its record says about half the old suite never reaches the binary
and that the second part, success paths across configurations and layouts, is not built.

*Implication:* it cannot see a refusal that the previous release did not have. Both blockers are
that case.

---

## 6. The two confirmed regressions

Source: `completions/artifacts/M55/stabilization-build/canary-one/findings.md:9`–`:10` and the two
reports under `canary-one/run/r1/reports/test/` (`verify-p2-r1-setup-install-hook-route-other-causes-unexamined.a1.md`,
`verify-p3-r1-p3-setup-install-hook-undriven-producers-and-causes.a1.md`). Source lines checked
against the current tree; they match the reports.

**The refusal.** Code `setup.install-hook`. Three producers, all in `crates/cli/src/setup.rs`:

| | site | fires when | route |
|---|---|---|---|
| P1 | `:2379`–`:2387` | `current_exe()` fails | re-run `jigc setup` |
| P2 | `:2391`–`:2397` | `install_precommit_hook` (`:907`) returns **any** I/O error | fixed string: "ensure the repo's git hooks directory is writable, then re-run `jigc setup`" |
| P3 | `:3120`–`:3155`, `hook_link_refusal` | the hook is a symlink leading outside the repository and its hooks directory, or to nothing | `format!("remove the link at `{shown}`, or point it at a script inside this repository, then re-run `jigc setup`. …")` at `:3150`–`:3154` |

All three routes are bare strings — `Human` routes with no argv. No fence in section 3 has
anything to say about them: P2 and P3 contain one backticked span, `jigc setup`, which is
shell-safe and parses.

**What made it destructive.** `{shown}` comes from `display_hook_path` (`setup.rs:960`), which
calls `std::fs::canonicalize` on the hook path. Canonicalizing a symlink follows it. So when the
link's target **exists**, `{shown}` is the target — the user's script outside the repository — and
the route reads "remove the link at `<the script>`". Done as printed, the script is deleted, the
re-run exits 1 again (the link now dangles), and only the second refusal names the real link. When
the link **dangles**, `canonicalize` fails, the function falls back to the link's own path, and
the route is correct. It is a regression because `1.0.0-rc.24` had no refusal there: it exited 0
(and wrote through the link).

The second key is the same mechanism with a relative link, plus a non-regression half on P2: a
hook path that is a directory, or a hook of mode 000 or 200, draws the writability route while the
hooks directory is already writable, and the re-run is byte-identical.

**The suite already provoked this exact state.**
`crates/cli/tests/replacing_writers_never_follow.rs:1210` loops `for end in ["outside",
"dangling"]`, plants the link, asserts exit 1, the code, nothing installed and the outside file
untouched. Then at `:1235` it takes the route's step as
`fs::remove_file(repo.join(".git/hooks/pre-commit"))` — the path the route *means*, written by
hand, not the path the route *printed*. The re-run exits 0 and the test is green in both states.

**Would a registry-driven walk have caught it?**

- **One state per code: no, not reliably.** `setup.install-hook` has three producers. The most
  natural single state — a read-only hooks directory, the state `setup_failed_first_run.rs`
  asserts this code under six times — hits P2, and P2's route works for that cause.
- **One state per producer site: only by luck of the state.** P3 has two arms (the `why` match at
  `setup.rs:3127`). The dangling arm passes; only the arm with an existing target outside
  the repository is destructive. A walk that picked one state for P3 had even odds.
- **One state per branch of the producer: yes — on one further condition.** The walk must take
  the route's operand from the printed bytes. The existing test had the right state and passed
  because it did not.
- **The P2 half is catchable only by a walk over causes.** The refusal site is a single
  `map_err`; the five failure points are inside the callee (`resolve_hooks_dir`, `create_dir_all`,
  `read_to_string`, `fs::write`, `make_executable`). The verifier drove 17 rigs on the candidate
  to separate the three causes where the route works from the four where it loops.

*Implication:* the unit a walk needs is not the code and not the construction site but the
*cause* — the distinct state that reaches the route text with distinct interpolated values. Nothing
in the code enumerates causes. And for a prose route the walk needs an interpretation step that
uses the printed operand, since that is the step the existing test replaced with a constant.

---

## 7. Size

No data maps a code to a command, so this table groups by **code prefix**, with the door a prefix
mostly belongs to. A prefix is a family name, not a reachability fact: `store.*` is printed by
every door that takes an address, `repo.*` by every acting door, the gate families by both
`task validate` and `task finalize` and by `milestone finalize`.

| command (by code prefix) | code-like literals | construction sites | blocking with a route | advisory with a route | route kinds at those sites |
|---|---|---|---|---|---|
| `setup` (`setup`, `adapter-guide`) | 22 | 28 | 23 | 5 | prose 26, variable 2 |
| `uninstall` | 13 | 16 | 16 | 0 | prose 14, variable 2 |
| `start` / `workflow` compose (`workflow-refs`, `workflow`, `pack`) | 35 | 10 | 10 | 0 | prose 8, mechanical 2 |
| `task finalize` + `task validate` gate families (`finalize`, `schema-conformance`, `conformance`, `reconciliation`, `file-state`, `doc-code`, `owner-artifact`, `commit-recording`, `changelog-recording`, `amend`, `task`) | 75 | 83 | 45 | 24 | prose 54, variable 8, informational 4, mechanical 3 |
| `task discard` | 4 | 4 | 4 | 0 | prose 4 |
| `task bind` | 2 | 2 | 2 | 0 | mechanical 1, variable 1 |
| `doc` write verbs (`write`, `create`) | 28 | 23 | 23 | 0 | prose 15, mechanical 5, variable 3 |
| `doc` read verbs and every address-taking door (`store`, `read`, `work-unit`) | 13 | 5 | 5 | 0 | mechanical 3, prose 2 |
| `config *` (`config`, `overrides`, `slot-fill-target`, `structural-target`) | 28 | 19 | 19 | 0 | prose 17, variable 2 |
| `milestone *` (`milestone`, `join`, `combine`, `milestone-*`) | 32 | 32 | 31 | 1 | prose 25, variable 6, mechanical 1 |
| `migrate` | 4 | 3 | 2 | 1 | prose 2, mechanical 1 |
| `ingest` | 3 | 3 | 3 | 0 | prose 2, variable 1 |
| `migrate-corpus` | 13 | 1 | 0 | 1 | variable 1 |
| `rename` | 4 | 0 | 0 | 0 | built through `kind.code()` |
| `relocate` | 2 | 0 | 0 | 0 | built through `refusal.code()` |
| `validate`, store scope only (`schema-completeness`, `store-version`, `pack-probe-integrity`, `override-default`) | 8 | 4 | 2 | 2 | prose 4 |
| every acting door (`repo`) | 3 | 0 | 0 | 0 | built through `self.member.code()` |
| **total** | **289** | **233** + 24 dynamic | **185** + 20 dynamic | **34** | |

How reliable: the *construction sites* and *blocking with a route* columns are exact for
literal-coded sites and are the floor of the work — **about 205 blocking routed sites**. They
undercount wherever a family builds its findings through a helper (`rename`, `relocate`,
`migrate-corpus`, `repo`, most of `workflow-refs`), where the *code-like literals* column is the
better guide. Neither column counts causes; section 6 shows one site carrying five.

Outside this table: the ~17 free-text next steps and 43 route-less git failures in the operational
funnel, and the 12 clap tip rows.

---

## Not established

- **Which door prints which code.** Not derived. The table in section 7 is by prefix, not by
  reachability.
- **The route kind at 34 construction sites** where the route is a variable or a helper result.
- **Whether each of the 170 candidate tests really runs a refusal's route.** Two helpers were read
  by hand; the rest is a pattern match. Some run a composed `Run:` line or an ack's next step.
- **How many refusals have a route-running test, exactly.** Bounded between 43 and 117 codes by
  two imperfect counts; no per-code check was made.
- **How many of the 289 literals are finding codes.** At least five are not, plus the 12 error
  identities; the remainder was not separated one by one.
- **Unit tests inside `src`.** Not scanned for which codes they name or which routes they run.
- **Causes per site.** Not counted anywhere; only `setup.install-hook` was traced.
- **The other sixteen canary findings.** Not traced to their producers.
- **Any fence firing.** Nothing was run. The debug-only posture is read from the source; no
  release binary was inspected.
