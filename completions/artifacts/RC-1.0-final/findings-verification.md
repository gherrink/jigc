# Findings verification

[protocol.md](protocol.md) §7: **every CONFIRMED *and* REFUTED verdict arrives with a repro
block**, and each carries `pinned-by: <suite>::<test>` or a stated `UNPINNED: <why>`. A refuted
claim's obligation is that the refuting fact **has** a standing test, not that a duplicate is
minted.

**Citations were verified by reading what the cited test asserts.** Not by the name looking apt —
[pinning.md](../../../implementation/pinning.md) §3 refuses a symbol parser by name, on the
grounds that *a grep is not a fence*. Where a suite pins something adjacent but not the claim, the
row says so and is marked UNPINNED rather than borrowing the citation.

**Product findings are separated from instrument and fixture findings**, because folding them
together would inflate the trial's yield with work the trial did not do.

> **This file committed its own warned-against error once, and the correction is left visible.**
> R-1 was first cited as `doc_read_surface::malformed_address_names_the_grammar` — a plausible
> suite paired with a test name that **does not exist**, invented because it sounded like what the
> claim needed. Checking turned up the real one in a different suite entirely
> (`anyhow_route_spans::malformed_doc_address_routes_to_describe`, which asserts the exact error
> bytes *including* the route). A citation that is never opened is decoration, and this is what
> that failure looks like from the inside.

---

# Product findings

## S-1 · The `--from-file -` idiom is unreachable by the idioms an agent reaches for

```yaml
claim: "under the default permission set, an agent cannot feed a payload to
        `jigc doc author --from-file -` by any idiom it tries first"
verdict: CONFIRMED (as a discoverability finding, after the severity was corrected — see below)
setup:
  - corpus: adopted, plant E staged
  - session: run-session.sh --headless --strict-permissions   # --permission-mode default
repro:
  - ["cat", "<<EOF", "|", "jigc", "doc", "author", "changelog", "--from-file", "-"]   # DENIED
  - ["Write", "/tmp/changelog-payload.yaml"]                                          # DENIED
  - ["Bash", "cat > /work/.scratch-changelog-payload.yaml <<EOF"]                     # DENIED
  - ["Bash", "printf '%s\\n' 'title: Changelog' ..."]                                 # DENIED x3
expect:
  denials: 11
  bare_jigc_denials: 0
  outcome: "the worker exhausts its idioms and halts"
pinned-by: "stdin_form_naming::every_site_bearing_step_names_the_permitted_form +
  ::every_composed_workflow_names_the_form_above_its_sites +
  ::the_named_heredoc_form_runs_verbatim_through_a_real_shell — the NAMING half, which is the
  half M49 shipped (increment 11 / T8, b5680ba). The `--from-file -` site set is DERIVED from
  both loaded packs rather than listed; every site-bearing step is disposed — it names the
  permitted heredoc-on-the-command form, or it is out with a stated reason; the assertion is
  made on the COMPOSED bytes through the real binary, not on the step file; and the named form
  is then EXECUTED through a real shell, so the words the pack now prints are runnable rather
  than plausible. UNPINNED: the DENIAL half — no suite drives the binary under a restricted
  permission set; the permission layer is the harness's, not jigc's, and nothing in
  crates/cli/tests models it."
```

**The severity was corrected on evidence, and the correction is the useful part.** The first
write-up flagged this as possibly a §1 **blocking dead end**. A four-command probe settles it:

```yaml
claim: "a heredoc attached directly to the jigc command is permitted"
verdict: CONFIRMED — so the route CAN run, and S-1 is not a dead end
repro:
  - ["jigc", "--version"]                                                    # permitted
  - ["jigc", "doc", "create", "adr", "--title", "Probe one", "--task", "probe"]  # permitted
  - ["jigc", "doc", "set-slot", "adr:probe-one#context", "--from-file", "-", "<<EOF"]  # PERMITTED
  - ["cat", "<<EOF", "|", "jigc", "doc", "set-slot", "adr:probe-one#decision", "--from-file", "-"]  # DENIED
expect:
  denial_reason: "Contains shell syntax (pipeline) that cannot be statically analyzed"
evidence: evidence/stdin-probe-invocations.jsonl, evidence/stdin-probe-result.json
```

**Disposition: surface/discoverability — SHIPS RECORDED.** The block is the *pipeline*, not `cat`
and not a too-narrow allowlist. A permitted form exists; **nothing names it.** The pack prints
`--from-file -` in `author-roadmap.yaml` (×3), `author-completion-record.yaml`,
`author-research.yaml` and more, and no step, help text or guide shows how to supply stdin in a
shape the default permission set allows.

**The seventh consecutive landing of the discoverability lens, and the purest instance yet** — not
a missing capability and not a wrong route, but a **permitted syntax no composed surface names**.

## S-4 · The identity verb pair misdirects on both wrong turns

```yaml
claim: "`jigc rename <addr> --to X --task <id>` draws a clap error whose tip points away from
        the answer, and nothing in it names `jigc doc rename`"
verdict: CONFIRMED
setup:
  - fixture: an adopted corpus with a STAGED, never-committed adr bound to an open task
repro:
  - ["jigc", "rename", "adr:drop-the-oldest-sample", "--to", "Shed the oldest sample on ingest overflow", "--task", "<id>"]
expect:
  exit: 2
  stderr: |
    error: unexpected argument '--task' found
      tip: to pass '--task' as a value, use '-- --task'
  absent: "any mention of `jigc doc rename`"
observed_recovery: "the worker then tried the task-less top-level form (exit 1), read
  `jigc doc rename --help`, and only then reached the right verb"
pinned-by: "clap_error_kind_axis::the_rename_identity_pair_names_its_in_task_sibling — it
  drives THIS repro's argv (`jigc rename adr:x --to X --task t1`) through the real binary and
  asserts four things about the emitted stderr: the exit stays `EXIT_USAGE`, clap's own
  `unexpected argument '--task' found` line is preserved, the tip now names `jigc doc rename`,
  and the misdirecting `-- --task` value tip is REPLACED rather than printed above the answer
  (the M48 lesson that a correction below a contradicting suggestion is not a correction).
  ::every_producible_kind_is_reached_by_driving_its_probe holds the surrounding axis: every
  clap `ErrorKind` a jigc argv can produce is reached by driving a probe that produces it,
  which is the completeness proof available where `ErrorKind` is `#[non_exhaustive]`."
```

**Disposition: surface/discoverability — SHIPS RECORDED.** T9 was reached and no work was lost;
the cost is one help read. **But it is the class M46 Increment 8 T5 already fixed once** — for
`jigc describe <positional>`, where clap printed a bare error and the repair was a tip that
*routes*. Here the tip exists **and misdirects**, which is worse than absent, on precisely the
confusable pair the identity split is built around.

## PT-A · A blocking refusal with no route, whose sibling on the same verb has one

```yaml
claim: "`jigc rename` onto an occupied identity refuses with no route line, while the
        unknown-doc refusal on the same verb carries one"
verdict: CONFIRMED
setup:
  - fixture: two committed adrs
repro:
  - ["jigc", "rename", "adr:cap-distinct-series", "--to", "Drop the oldest sample on overflow"]
  - ["jigc", "rename", "adr:no-such-doc", "--to", "Something else"]
expect:
  first:  { exit: 1, stderr_contains: "a different doc already exists at", route: absent }
  second: { exit: 1, route: "check the id (or run `jigc describe` for the doctype surface)" }
  invocation_log: { error_code: null, finding_codes: [] }   # both
pinned-by: "flow37_rename::every_rename_refusal_carries_an_identity_and_an_exit — it iterates
  `RefusalKind::ALL` (the nine arms `run_rename` declares, `OccupiedDestination` among them),
  drives each through the real binary, and asserts per arm: exit 1, the declared code identity
  in stderr, EXACTLY ONE route line, and then — for a `Command` repair — that the emitted argv
  leads with `jigc` and RUNS VERBATIM at exit 0, or — for a `Judgment` repair — that prose
  survives outside the backticked spans, so a bare command cannot wear a judgment label. It
  also asserts the refusal's code reaches the invocation log's `finding_codes`, which is the
  identity half. ::every_refusal_kind_is_declared is the enumeration fence: a tenth arm cannot
  ship without joining `ALL` and gaining a driven scene.
  `flow37_rename::rename_onto_a_different_existing_slug_blocks` still pins the block, the
  message and the incumbent's surviving H1, and is deliberately NOT the citation here — it says
  nothing about a route, which is why it was refused as a pin before this wave."
```

**Disposition: surface — SHIPS RECORDED.** The message names the occupied path, so the user is not
stranded. It is an anyhow error rather than a `Finding`, so M43's route floor does not formally
reach it — but its sibling one argument away does route, which is the incomplete-sweep shape.

## PT-C · The router's closing claim is true only under a definition the surface never states

```yaml
claim: "'each hidden one carries the reason it is hidden' is checkable only if 'hidden' means
        `selectable: false`, and nothing states that"
verdict: CONFIRMED as a measurement; NOT adjudicated as a defect
repro:
  - ["jigc", "start", "add a rate limiter to the ingest path"]   # the catalog
  - ["jigc", "describe", "--workflows"]                          # the full set
derivation: "absent-from-catalog minus those carrying 'It is hidden from the router catalog'"
expect:
  catalog: 12
  absent: 21
  without_a_stated_reason: 3      # increment, ingest-existing, router
note: "those three are exactly the `creates-task: false` set M46's own audit named"
pinned-by: "describe::describe_states_why_every_off_catalog_workflow_is_absent — the population
  it iterates IS this claim's: `shipped_off_catalog_workflows()` is the complement (absent from
  the router catalog), not M43's `selectable: false` set, and the arm asserts a derived
  NON-VACUITY first — the shipped packs must carry at least one off-catalog member the hidden
  set does not contain, or the arm would silently re-prove M43. For every member it then
  asserts the declared reason on BOTH surfaces: `router_hidden` on the `--format json`
  definition, and the reason string inside the emitted prose.
  ::describe_projects_the_suppression_reason_for_hidden_workflows keeps the `selectable: false`
  half separate, so the two populations cannot be conflated again."
```

**Disposition: not a defect on the charitable reading, and unreadable on the natural one.** The
honest finding is that **the claim cannot be checked by the person it addresses.**

---

# Refuted

## R-1 · B3's failed read is not a defect

```yaml
claim: "a `doc show` that exits 1 indicates a read-surface defect"
verdict: REFUTED
repro:
  - ["jigc", "doc", "show", "decisions/reject-the-newest-sample-when", "--task", "<id>", "--format", "json"]
expect:
  exit: 1
  error: |
    malformed address `decisions/reject-the-newest-sample-when`: missing ':' between type and
    slug — a doc is addressed as `<type>:<slug>`, e.g. `adr:single-node-cache` (a singleton
    doctype like `changelog` or `vision` may be named bare)
      route: run `jigc describe` for the doctype surface
observed: "the worker corrected to `adr:reject-the-newest-sample-when` on its next call"
pinned-by: anyhow_route_spans::malformed_doc_address_routes_to_describe
```

The worker used a path-shaped address. The refusal names the malformation, gives the grammar, an
example, the singleton exception **and** a route, and was recovered from in one step. **This is
the surface working**, and it is why Q1's settlement reports attempts *and* effective separately:
7 and 6 here, and one number would have hidden both.

## R-2 · B3-strict's managed-document read is permitted, not a bypass

```yaml
claim: "the one managed-document filesystem read is an adapter bypass"
verdict: REFUTED
repro:
  - ["cat", "/work/CHANGELOG.md"]
context: "the invocation log shows NO ingest and NO migrate before it — the file was still
          foreign and never adopted"
expect: "`.jigc/AGENT.md` explicitly permits reading an unregistered doc before adopting it"
pinned-by: "cli::adapter::tests::bootstrap_file_states_the_read_rule_and_scopes_the_slices_promise
  — and the earlier `UNPINNED` was wrong on its facts, which is what reading the test rather
  than its name buys. The permission is not merely stated in the adapter's text: the test
  asserts it on the RENDERED `.jigc/AGENT.md` body, which must carry the `unregistered` row
  clause (not yet managed — readable directly until adopted) alongside the managed-set
  discriminator, the `jigc doc show` read path, the staged `--task <id>` read and its inverse.
  A refuted claim's obligation is that the refuting fact HAS a standing test; it does."
```

This is the heuristic's declared bound firing in the wild: registration state is not in the
transcript, the reader flags the read, and a human makes the call.

---

# Instrument findings — found by running the rig, fixed, and pinned

These are **not** product findings and are kept out of the trial's yield. Every one was found by
driving something, none by reading, and each is pinned by a test that preserves the defect.

| # | what was wrong | disposition |
|---|---|---|
| I-1 | the plant wrote into the channel it is scored on — `observe` read **VERB 6** where the worker had done **2** | pinned-by: `test_observe::ThePlantIsNotTheWorker` |
| I-2 | a managed-document read and a workbench-bookkeeping read were one number; two runs of the same prompt diverged on exactly that seam | pinned-by: `test_observe::ADocumentReadIsNotABookkeepingRead` |
| I-3 | `halted_awaiting_human` could not see a **product-blocked** halt — the shape plant F depends on | pinned-by: `test_observe::TheOtherHaltShape` |
| I-4 | …and the fix for I-3 was itself incomplete: `endswith("?")` missed B1, which asked and then added a closing sentence | pinned-by: `test_observe::TheOtherHaltShape::test_a_question_followed_by_a_closing_sentence_still_counts` |
| I-5 | the aligned §3.3 counter and its model — `is_shipped_adjacent` compared an argv **prefix** where the grep is a **substring** match | pinned-by: `test_observe::TheShippedCounterDisagrees` |

**I-4 is the M45 complete-fix lens turned on my own patch**, and it was found the same way I-3
was: by running it.

## S-5 · The operator contaminated the evidence channel

```yaml
claim: "operator commands run against a LIVE session's container land in the measurement channel"
verdict: CONFIRMED
repro:
  - ["docker", "exec", "-u", "node", "<cid>", "jigc", "rename", "..."]   # during a live session
expect:
  invocation_log: "+2 records, timestamped inside the session"
impact:
  scored_channels: unaffected      # VERB 5 and adjacent 6 are all the worker's
  record_count: "42 -> 40"
pinned-by: "UNPINNED: no fence was built and none is owed from the product — the `session-start`
  split handles records made BEFORE a session (`test_observe::ThePlantIsNotTheWorker`), nothing
  marks records made DURING one by someone other than the worker, and M49 changed no file under
  `completions/` outside its own artifacts, so nothing here moved. The mitigation stays
  discipline — reproduce in a copy — which is what plant-f-correction.md already requires for
  the slug check."
```

Recorded because **the rule broken was one written hours earlier in this trial's own
apparatus**, obeyed for the slug derivation and violated minutes later for a refusal repro.

---

# Fixture findings

## PT-D · The trial corpus's `IngestQueue` is dead on the live path

```yaml
claim: "IngestQueue is never fed by anything on a live path, and plant E's whole subject is
        that dead code"
verdict: CONFIRMED — a finding about the FIXTURE, not the product
repro:
  - ["grep", "-rn", "queue\\.", "src/"]        # exactly one hit, inside a dead tick()
  - ["sed", "-n", "/private ingest(/,/^  }/p", "src/router.ts"]   # calls store.put() directly
expect:
  router_ingest: "bypasses the queue"
  tick: "drains the queue and prunes — and nothing calls it"
found_by: "B3, unprompted, while writing its architecture doc; verified against the template
  before being believed"
pinned-by: "UNPINNED: the owed fix lives in trial-fixture tooling, and M49 touched no file under
  `completions/trial-corpus-template/` — `check-corpus.sh` still asserts the doc-code anchor
  symbols EXIST but not that they are REACHED. Carried rather than closed: the bar belongs to
  the next trial's corpus gate, not to a product wave."
```

**Not fixed during the trial**: B3 ran on this fixture and B1/B2 had to match it, so changing the
template mid-trial would have been worse than the wart.
