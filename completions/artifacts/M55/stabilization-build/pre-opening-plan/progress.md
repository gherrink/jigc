---
base: 7c34a222c926d49de365fc16cf04c157017b7f37 7c34a22
status: active
schema-version: 3
---

# before-the-first-stabilization-run

## Tasks

### d0-write-down-the-rulings  {#d0-write-down-the-rulings}

<!-- fields -->
- intent: D0 — write down the rulings, the owed rows and the tool candidates
- workflow: builder
- status: joined

### d0-def-add-what-is  {#d0-def-add-what-is}

<!-- fields -->
- intent: D0-def — add what is owed before the opening to the opening's fact-sheet agent
- workflow: main-session edit
- status: joined

### m-defs-fix-the-sentences  {#m-defs-fix-the-sentences}

<!-- fields -->
- intent: M-defs — fix the sentences repeated across the agent definitions
- workflow: main-session edit
- status: joined

### pc-count-the-codes-each  {#pc-count-the-codes-each}

<!-- fields -->
- intent: PC — count the codes each dynamic refusal site can produce
- workflow: read-only agent
- status: joined

### fork-cut-the-four-lane  {#fork-cut-the-four-lane}

<!-- fields -->
- intent: Fork — cut the four lane branches from one commit
- workflow: git step

### p0-generate-the-catalogs-overview  {#p0-generate-the-catalogs-overview}

<!-- fields -->
- intent: P0 — generate the catalog's overview of families, parts, sites and answers
- workflow: builder

### p1-stop-counting-the-finding  {#p1-stop-counting-the-finding}

<!-- fields -->
- intent: P1 — stop counting the finding constructors as sites, and tie each standing reason to its ruling
- workflow: builder

### m-p0-merge-the-catalog  {#m-p0-merge-the-catalog}

<!-- fields -->
- intent: M-P0 — merge the catalog lane into the loop branch
- workflow: git step

### t0-give-agents-a-channel  {#t0-give-agents-a-channel}

<!-- fields -->
- intent: T0 — give agents a channel to report the tools they wrote
- workflow: builder
- status: joined

### t0-c2-review-sf1-the-tools  {#t0-c2-review-sf1-the-tools}

<!-- fields -->
- intent: T0 review, cycle 2, SF-1 — the tools text is normalised before anything is asked of it: CRLF and a lone CR each a line end, so a sealed rendering keeps no CR a converting checkout could rewrite; the arm that pinned the CR now pins its absence
- workflow: fixer
- status: joined

### t0-c2-review-n2-one-line  {#t0-c2-review-n2-one-line}

<!-- fields -->
- intent: T0 review, cycle 2, N-2 — fence_check counts lines by the newline, as the host-path check and the fences do; held by a fenced line after a U+2028 and a host path after a lone CR
- workflow: fixer
- status: joined

### t0-c2-review-leads-control-characters  {#t0-c2-review-leads-control-characters}

<!-- fields -->
- intent: T0 review, cycle 2, the leads — control characters but the tab are spaces, as an entry's texts have them, and a leading byte-order mark is dropped, so a mark alone is blank and refused by --tools
- workflow: fixer
- status: joined

### t0-c2-review-sf2-the-principles  {#t0-c2-review-sf2-the-principles}

<!-- fields -->
- intent: T0 review, cycle 2, SF-2 — working-principles.md scopes the rendered triage report to a run with a queue, and says a run without an order writes its report as every agent does
- workflow: fixer
- status: joined

### t0-c2-review-sf4-every-refusal  {#t0-c2-review-sf4-every-refusal}

<!-- fields -->
- intent: T0 review, cycle 2, SF-4 — every refusal reachable from an entry names it: ledger_row and hold_kinds take the entry's name, the key's slug check and the field check say entry N of the file; held for no kind, crossed evidence, a lead's route, a bad doctype, a door that is no text, a key that is no slug
- workflow: fixer
- status: joined

### t0-c2-review-sf5-scope-set  {#t0-c2-review-sf5-scope-set}

<!-- fields -->
- intent: T0 review, cycle 2, SF-5 — scope-set's call is typed as it stands, so its host-path refusal names the door and the text and says to reword it; verdict and report may pass roots, and state, untriaged and apply carry no text of the agent's
- workflow: fixer
- status: joined

### t0-c2-review-n1-the-nul  {#t0-c2-review-n1-the-nul}

<!-- fields -->
- intent: T0 review, cycle 2, N-1 — the rendering's NUL clause dropped rather than armed: every path that refuses a file refuses a NUL first, and the state's read takes a forged rendering by design
- workflow: fixer
- status: joined

### t0-c2-review-n3-the-combined  {#t0-c2-review-n3-the-combined}

<!-- fields -->
- intent: T0 review, cycle 2, N-3 — the combined hygiene refusal names the file --tools names and says to hand both files over again
- workflow: fixer
- status: joined

### t0-c2-review-n4-found-again  {#t0-c2-review-n4-found-again}

<!-- fields -->
- intent: T0 review, cycle 2, N-4 — a found-again entry that brings a row's cell is refused for that before its texts are asked, so it is refused once
- workflow: fixer
- status: joined

### t0-c2-review-sf3-what-is  {#t0-c2-review-sf3-what-is}

<!-- fields -->
- intent: T0 review, cycle 2, SF-3 — what is left open is two clauses of wf-tools-no-road in decisions-pending.md: the per-part hygiene detail by the file's own line, and the --tools refusal naming the file in a form no writer has to replace
- workflow: fixer
- status: joined

### t0-c3-review-sf4-the-door  {#t0-c3-review-sf4-the-door}

<!-- fields -->
- intent: T0 review, cycle 3, SF4 — scope-set's host-path refusal names the door by its place, included door N, and never by its text, which may be the very path refused; the class is the refusals reworded builds from the refused text, two callers, this one site
- workflow: fixer
- status: joined

### t0-c3-review-sf5-the-tools-text  {#t0-c3-review-sf5-the-tools-text}

<!-- fields -->
- intent: T0 review, cycle 3, SF5 — the Tools text does what its sentence says, consistently with cell by one definition both read: every character a cell takes as a line break is a line end, every other C0 character but the tab and DEL a space, a C1 character other than NEL kept; a report's own fence keeps the newline-only line count armed
- workflow: fixer
- status: joined

### rb1-judge-a-regression-against  {#rb1-judge-a-regression-against}

<!-- fields -->
- intent: RB1 — judge a regression against every earlier candidate of the run, in the record script
- workflow: builder
- status: joined

### rb1-review-s1-the-candidates  {#rb1-review-s1-the-candidates}

<!-- fields -->
- intent: RB1 review S1 — an earlier candidate that is the candidate's own binary again counts towards the list when named false, and anything else said of it is a flake: the fact is unknown, a green previous release included (earlier-is-the-candidate); the guard is in the one definition the filing and every read share, which also reads a previous release beside the candidate's hash as no fact at a read
- workflow: fixer
- status: joined

### rb1-review-s2-green-with-no  {#rb1-review-s2-green-with-no}

<!-- fields -->
- intent: RB1 review S2 — an arm for an earlier candidate green with no previous release behind it: a regression, known, sent to a fixer; red against the mutant that keeps the no-previous-hash reason
- workflow: fixer
- status: joined

### rb1-review-s3-the-sealed  {#rb1-review-s3-the-sealed}

<!-- fields -->
- intent: RB1 review S3 — an arm for the read's shape check of earlier: a resealed file with two entries for round 2 in round 3 is no filing; red against the mutant without the check
- workflow: fixer
- status: joined

### rb1-review-n1-the-two  {#rb1-review-n1-the-two}

<!-- fields -->
- intent: RB1 review N1 — stabilization-workflow.md and decisions-pending.md say what the code does: unknown in a run with a queue, for a fact that is not true; a run without an order takes triage entries, which carry no list, the old path T12 deletes
- workflow: fixer
- status: joined

### rb1-review-n2-the-vet  {#rb1-review-n2-the-vet}

<!-- fields -->
- intent: RB1 review N2 — the vet's another-binary message and filing_of's docstring say an earlier candidate's binary too
- workflow: fixer
- status: joined

### rb1-review-n3-the-view  {#rb1-review-n3-the-view}

<!-- fields -->
- intent: RB1 review N3 — the stop's view says a false or not-comparable finding is green on no earlier candidate of the run
- workflow: fixer
- status: joined

### rb1-review-n4-a-round  {#rb1-review-n4-a-round}

<!-- fields -->
- intent: RB1 review N4 — a round whose record names no binary leaves every later fact that is not true unknown, never no, and the filing now says why (earlier-unrecorded); preventing the state at round-set is not built, since the harness writes candidate and binary together
- workflow: fixer
- status: joined

### rb1-review-left-open-a-triage  {#rb1-review-left-open-a-triage}

<!-- fields -->
- intent: RB1 review, left open 1 — a triage entry of round 2 or later says no fact that is not a regression: triage-set writes its no and its not-comparable as unknown and prints why, entry-no-earlier; its yes stands and round 1 is unchanged; held in a run with a queue and in one without an order
- workflow: fixer
- status: joined

### rb1-review-left-open-the-shape  {#rb1-review-left-open-the-shape}

<!-- fields -->
- intent: RB1 review, left open 2 — the read holds the shape of previous, the sha256 of a binary or nothing, as it holds the shape of earlier; a resealed file with a hash that is none is no filing
- workflow: fixer
- status: joined

### rb1-review-left-open-why-a  {#rb1-review-left-open-why-a}

<!-- fields -->
- intent: RB1 review, left open 3 — why a fact is unknown stands beside its key in state, regression_unknown_why, and in the stop's view, read through one definition, why_unknown, which the filing's print uses too; a run without an order answers as it did
- workflow: fixer
- status: joined

### rb1-c3-review-sf1-two-drives  {#rb1-c3-review-sf1-two-drives}

<!-- fields -->
- intent: RB1 review, cycle 3, SF1 — any two drives of one binary hash that disagree make the fact unknown, never a regression: candidate, previous and every earlier binary, pairwise, by one predicate, drives_disagree, used at the filing and at every read; the word is drives-disagree
- workflow: fixer
- status: joined

### rb1-c3-review-sf2-a-fix-stage  {#rb1-c3-review-sf2-a-fix-stage}

<!-- fields -->
- intent: RB1 review, cycle 3, SF2 — triage-set takes --stage test|fix, required, and the harness passes its stage: a fix stage's entries, driven on the round's tip, say no fact that is not a regression in round 1 too; in a run with a queue the round record counts them as entry-no-earlier, so the state says why beside them
- workflow: fixer
- status: joined

### rb1-c3-review-sf3-not-said  {#rb1-c3-review-sf3-not-said}

<!-- fields -->
- intent: RB1 review, cycle 3, SF3 — arms for not-said, for the filing's print of a flake beside no said regression, for the read's guard on a resealed previous null, and for the vet's another-binary words
- workflow: fixer
- status: joined

### rb1-c3-review-sf5-the-docs  {#rb1-c3-review-sf5-the-docs}

<!-- fields -->
- intent: RB1 review, cycle 3, SF5 — the docs held to the code: the N1 clause superseded, the A_STAGE_AS_IT_WAS claim round 1 only, triage-set's Prints line, the state header's reason for a filing that said no regression, and the unknown condition of wf-regression-base
- workflow: fixer
- status: joined

### rb1-c3-review-n2-the-flake-reading  {#rb1-c3-review-n2-the-flake-reading}

<!-- fields -->
- intent: RB1 review, cycle 3, N2 — the rule that a flake overrides a green previous release is marked in DECISIONS.md as the builders' reading, open to being overturned, for the main session to put to the human
- workflow: fixer
- status: joined

### rb1-c3-review-n4-the-dead-assignment  {#rb1-c3-review-n4-the-dead-assignment}

<!-- fields -->
- intent: RB1 review, cycle 3, N4 — the dead assignment of unknown in verdict is removed
- workflow: fixer
- status: joined

### rb1-c3-review-lead-never-driven  {#rb1-c3-review-lead-never-driven}

<!-- fields -->
- intent: RB1 review, cycle 3, its lead — an earlier round whose record names no binary leaves every later fact that is not true unknown: recorded as a clause of wf-regression-base; the harness writes candidate and binary in one call and neither on a re-run, a drop or a part
- workflow: fixer
- status: joined

### u0-give-cis-slowest-legs  {#u0-give-cis-slowest-legs}

<!-- fields -->
- intent: U0 — give CI's slowest legs room, so none runs above 80 % of its limit
- workflow: builder

### m-u0-merge-the-small  {#m-u0-merge-the-small}

<!-- fields -->
- intent: M-U0 — merge the small-tools lane into the loop branch
- workflow: git step

### u5-list-the-commits-no  {#u5-list-the-commits-no}

<!-- fields -->
- intent: U5 — list the commits no review has read, from a registry of past reviews
- workflow: builder

### k7-give-the-trial-image  {#k7-give-the-trial-image}

<!-- fields -->
- intent: K7 — give the trial image a cached base layer
- workflow: builder

### h2-ask-how-to-clear  {#h2-ask-how-to-clear}

<!-- fields -->
- intent: H2 — ask how to clear the secret scanner's flag on an old line of the decisions log
- workflow: human question

### p-f0-classify-the-reason  {#p-f0-classify-the-reason}

<!-- fields -->
- intent: P-F0 — classify the reason texts, build the shared test pieces, write the not-built cases
- workflow: builder

### t-nr-build-build  {#t-nr-build-build}

<!-- fields -->
- intent: T-NR build — build the probe for rewritten returns, and the guard that binds while it runs
- workflow: builder
- status: active

### t-nr-review-b1-no-script-written  {#t-nr-review-b1-no-script-written}

<!-- fields -->
- intent: T-NR review B1 — no script written under the scratch root runs: the probe tool holds each copy it runs to this repository's own by sha256, its dev directory to the copies alone and the throwaway repository's git configuration to the keys git init writes, immediately before it runs one, and refuses altered; under the marker a subagent writes only the batch and the obeyed file, never a copy, a module beside one or a .git
- workflow: fixer
- status: joined

### t-nr-review-b2-each-act-its  {#t-nr-review-b2-each-act-its}

<!-- fields -->
- intent: T-NR review B2 — under the marker each act of the probe tool is the type's the harness launches for it, begin and verdict a build-git's and record a build-executor's, under the marker's own scratch root; the record script is off the list; outside() refuses a scratch root inside or holding any worktree or the common git directory; the refusals name the main session as the one that lifts a marker and hand out no path
- workflow: fixer
- status: joined

### t-nr-review-b3-a-writer-that  {#t-nr-review-b3-a-writer-that}

<!-- fields -->
- intent: T-NR review B3 — a record writer that halts with no line is judged halted, b:r:none unsent or missing, never garbled; the neutralized verdict lifts the marker and writes its result whatever it decides; a simulation arm has the writer halt
- workflow: fixer
- status: joined

### t-nr-review-s1-the-verdicts-line  {#t-nr-review-s1-the-verdicts-line}

<!-- fields -->
- intent: T-NR review S1 — the verdict's line carries hashes, never the planted text; a simulation arm has the runtime's stand-in rewrite the verdict's return where it holds each shape, and the probe is judged, not halted
- workflow: fixer
- status: joined

### t-nr-review-s2-a-root-that  {#t-nr-review-s2-a-root-that}

<!-- fields -->
- intent: T-NR review S2 — a scratch root that holds the repository is refused, inside; the harness cannot check it, having no path of the checkout, and says so beside probeFault
- workflow: fixer
- status: joined

### t-nr-review-s3-who-changed-the  {#t-nr-review-s3-who-changed-the}

<!-- fields -->
- intent: T-NR review S3 — record takes --sha256 and refuses a batch that is not the bytes the harness composed; the cell and the report are judged exactly, against what was handed, with as_planted beside; where the verdict cannot tell who changed a text it says so, by
- workflow: fixer
- status: joined

### t-nr-review-s4-a-cut-runs  {#t-nr-review-s4-a-cut-runs}

<!-- fields -->
- intent: T-NR review S4 — the marker names when it was begun; the guard calls one stale whose root is gone or that is older than three hours, and says when it was begun; the halt note has the main session check the scratch root and the time before it removes one; the harness has no identity but its arguments
- workflow: fixer
- status: joined

### t-nr-review-s5-the-docs-held  {#t-nr-review-s5-the-docs-held}

<!-- fields -->
- intent: T-NR review S5 — the docs held to the code: at most four agents, the observers of (i) the subject and the record writer, the corrections in the log and the plan, the halt's no tracked file, the generated harness's true size
- workflow: fixer
- status: joined

### t-nr-review-s6-every-other-tool  {#t-nr-review-s6-every-other-tool}

<!-- fields -->
- intent: T-NR review S6 — under the marker every tool but the one act, a named write and a read is refused, MCP and web tools among them, with the hook's matcher every tool, a line of the main session's; what never reaches a PreToolUse hook is a declared bound
- workflow: fixer
- status: joined

### t-nr-review-n1-fail-closed-for  {#t-nr-review-n1-fail-closed-for}

<!-- fields -->
- intent: T-NR review N1 — fail closed for the writers too: where the guard cannot tell whether a marker stands, every subagent's call is refused
- workflow: fixer
- status: joined

### m-tnr-merge-the-hunter  {#m-tnr-merge-the-hunter}

<!-- fields -->
- intent: M-TNR — merge the hunter lane into the loop branch, then the loop branch back into it
- workflow: git step

### u1-let-the-local-stand  {#u1-let-the-local-stand}

<!-- fields -->
- intent: U1 — let the local stand-in for CI's runner declare its git setup in one place
- workflow: builder

### u2-add-a-ci-step  {#u2-add-a-ci-step}

<!-- fields -->
- intent: U2 — add a CI step that turns red when the runner's git setup drifts
- workflow: builder

### u3-read-ci-runs-time  {#u3-read-ci-runs-time}

<!-- fields -->
- intent: U3 — read CI runs: time, limit, share of the limit, image and git version
- workflow: builder

### u4-list-the-pending-rows  {#u4-list-the-pending-rows}

<!-- fields -->
- intent: U4 — list the pending rows due before the opening, as a floor
- workflow: builder

### m-u-merge-the-small  {#m-u-merge-the-small}

<!-- fields -->
- intent: M-U — merge the small-tools lane into the loop branch, then the loop branch into the hunter lane
- workflow: git step

### j2-0-first-listing-list  {#j2-0-first-listing-list}

<!-- fields -->
- intent: J2-0 first listing — list what no review covers since the last review of the test stage
- workflow: main-session tool run

### j2a-frame-wave-review  {#j2a-frame-wave-review}

<!-- fields -->
- intent: J2a frame wave — review the route catalog's and the byte table's test frames
- workflow: review

### t-nr-run-run  {#t-nr-run-run}

<!-- fields -->
- intent: T-NR run — run the probe once with nothing else running, and record its verdict
- workflow: probe run

### p-f1-commit-1-write  {#p-f1-commit-1-write}

<!-- fields -->
- intent: P-F1 commit 1 — write the table of acts and count the routes that leave an act or operand to guess
- workflow: builder

### h0-ask-whether-to-carry  {#h0-ask-whether-to-carry}

<!-- fields -->
- intent: H0 — ask whether to carry on if the count is far above the pilot's
- workflow: human question

### h1-ask-the-human  {#h1-ask-the-human}

<!-- fields -->
- intent: H1 — ask the human to accept or challenge the nine standing reasons and the act table
- workflow: human question

### t1-make-a-hunt-name  {#t1-make-a-hunt-name}

<!-- fields -->
- intent: T1 — make a hunt name its family and part, and hold every part of a family hunted
- workflow: builder

### j2a-frame-fixes-fix-what  {#j2a-frame-fixes-fix-what}

<!-- fields -->
- intent: J2a frame fixes — fix what the frame reviews found, in the lanes that own the files
- workflow: builder

### p-f1-commit-2-make  {#p-f1-commit-2-make}

<!-- fields -->
- intent: P-F1 commit 2 — make a route that leaves an act or operand to guess fail, and list each one
- workflow: builder

### loop-to-p-merge  {#loop-to-p-merge}

<!-- fields -->
- intent: Loop to P — merge the loop branch into the catalog lane before its first family
- workflow: git step

### t2-store-a-hunts-rows  {#t2-store-a-hunts-rows}

<!-- fields -->
- intent: T2 — store a hunt's rows per site and compute the count, in the record script
- workflow: builder

### j2a-second-wave-review  {#j2a-second-wave-review}

<!-- fields -->
- intent: J2a second wave — review the catalog's cases, the byte table's first families, two CI fixes, the guard, the regression set, the hygiene scan and the settings
- workflow: review

### c1-answer-the-refusal-sites  {#c1-answer-the-refusal-sites}

<!-- fields -->
- intent: C1 — answer the refusal sites of task discard, task bind and doc read (10)
- workflow: builder

### c2-answer-the-refusal-sites  {#c2-answer-the-refusal-sites}

<!-- fields -->
- intent: C2 — answer the refusal sites of migrate, validate and ingest (7), then re-price
- workflow: builder

### c3-answer-the-refusal-sites  {#c3-answer-the-refusal-sites}

<!-- fields -->
- intent: C3 — answer the refusal sites of start (9)
- workflow: builder

### o-def-edit-five-agent  {#o-def-edit-five-agent}

<!-- fields -->
- intent: O-def — edit five agent definitions for the route hunt
- workflow: main-session edit

### j2a-second-fixes-fix-what  {#j2a-second-fixes-fix-what}

<!-- fields -->
- intent: J2a second fixes — fix what the second wave found, in the lanes that own the files
- workflow: builder

### c4-answer-the-first-seven  {#c4-answer-the-first-seven}

<!-- fields -->
- intent: C4 — answer the first seven refusal sites of config (7)
- workflow: builder

### c5-answer-the-rest  {#c5-answer-the-rest}

<!-- fields -->
- intent: C5 — answer the rest of config, relocate and two of start (7)
- workflow: builder

### c6-answer-the-doc-write  {#c6-answer-the-doc-write}

<!-- fields -->
- intent: C6 — answer the doc-write refusal sites of the doc module (11)
- workflow: builder

### t8-build-the-route-hunter  {#t8-build-the-route-hunter}

<!-- fields -->
- intent: T8 — build the route hunter: its role, its prompt and its fence
- workflow: builder

### m-u-again-merge  {#m-u-again-merge}

<!-- fields -->
- intent: M-U again — merge the small-tools lane's review fixes into the loop branch, and on into the hunter lane
- workflow: git step

### c7-answer-the-other-doc  {#c7-answer-the-other-doc}

<!-- fields -->
- intent: C7 — answer the other doc-write refusal sites (9)
- workflow: builder

### c8-answer-the-first-milestone  {#c8-answer-the-first-milestone}

<!-- fields -->
- intent: C8 — answer the first milestone refusal sites (10)
- workflow: builder

### h-r1-ask-about-any  {#h-r1-ask-about-any}

<!-- fields -->
- intent: H-r1 — ask about any new reason a site has no test, from the first batch
- workflow: human question

### m3-merge-the-trial-image  {#m3-merge-the-trial-image}

<!-- fields -->
- intent: M3 — merge the trial image lane into the hunter lane
- workflow: git step

### k7b-make-the-trial-images  {#k7b-make-the-trial-images}

<!-- fields -->
- intent: K7b — make the trial image's build a held command
- workflow: builder

### c9-answer-the-next-milestone  {#c9-answer-the-next-milestone}

<!-- fields -->
- intent: C9 — answer the next milestone refusal sites (7)
- workflow: builder

### c10-answer-the-last-milestone  {#c10-answer-the-last-milestone}

<!-- fields -->
- intent: C10 — answer the last milestone refusal sites (6)
- workflow: builder

### c11-answer-the-task-finalize  {#c11-answer-the-task-finalize}

<!-- fields -->
- intent: C11 — answer the task-finalize refusal sites of the parser (7)
- workflow: builder

### k6-make-the-scripted-trial  {#k6-make-the-scripted-trial}

<!-- fields -->
- intent: K6 — make the scripted trial arm a held check
- workflow: builder

### t9-make-the-small-canarys  {#t9-make-the-small-canarys}

<!-- fields -->
- intent: T9 — make the small canary's setup follow, with its two remaining arms
- workflow: builder

### c12-answer-the-rest  {#c12-answer-the-rest}

<!-- fields -->
- intent: C12 — answer the rest of the parser's task-finalize sites (7)
- workflow: builder

### c13-answer-the-task-finalize  {#c13-answer-the-task-finalize}

<!-- fields -->
- intent: C13 — answer the task-finalize sites of finalize, the index and the target surface (10)
- workflow: builder

### t11-move-the-simulations-remaining  {#t11-move-the-simulations-remaining}

<!-- fields -->
- intent: T11 — move the simulation's remaining scenarios to the new path
- workflow: builder

### c14-answer-the-task-finalize  {#c14-answer-the-task-finalize}

<!-- fields -->
- intent: C14 — answer the task-finalize sites of the CLI modules (10)
- workflow: builder

### c15-answer-the-task-finalize  {#c15-answer-the-task-finalize}

<!-- fields -->
- intent: C15 — answer the task-finalize sites of file state, state, validate and milestone (8)
- workflow: builder

### h-r2-ask-about-any  {#h-r2-ask-about-any}

<!-- fields -->
- intent: H-r2 — ask about any new reason a site has no test, from the second batch
- workflow: human question

### t12-delete-the-old-path  {#t12-delete-the-old-path}

<!-- fields -->
- intent: T12 — delete the old path
- workflow: builder

### o5-edit-the-definitions  {#o5-edit-the-definitions}

<!-- fields -->
- intent: O5 — edit the definitions the new path changed
- workflow: main-session edit

### m-t-merge-the-hunter  {#m-t-merge-the-hunter}

<!-- fields -->
- intent: M-T — merge the hunter lane into the loop branch
- workflow: git step

### c16a-give-every-code  {#c16a-give-every-code}

<!-- fields -->
- intent: C16a — give every code of the dynamic sites a test, first of four tasks cut from the count
- workflow: builder

### c16b-give-every-code  {#c16b-give-every-code}

<!-- fields -->
- intent: C16b — give every code of the dynamic sites a test, second of four
- workflow: builder

### t10a-bring-the-workflow-doc  {#t10a-bring-the-workflow-doc}

<!-- fields -->
- intent: T10a — bring the workflow doc up to what the stage does as built
- workflow: builder

### j2-0-partition-split  {#j2-0-partition-split}

<!-- fields -->
- intent: J2-0 partition — split the review range into covered, reviewed and to-be-reviewed files
- workflow: main-session tool run

### j2b-review-the-areas-that  {#j2b-review-the-areas-that}

<!-- fields -->
- intent: J2b — review the areas that moved, nine reviewers and one pass across them
- workflow: review

### c17-give-every-code  {#c17-give-every-code}

<!-- fields -->
- intent: C17 — give every code of the dynamic sites a test, third of four
- workflow: builder

### c18-give-every-code  {#c18-give-every-code}

<!-- fields -->
- intent: C18 — give every code of the dynamic sites a test, last of four
- workflow: builder

### h-r3-ask-about-any  {#h-r3-ask-about-any}

<!-- fields -->
- intent: H-r3 — ask about any new reason a site has no test, from the third batch
- workflow: human question

### j2b-fixes-fix-what  {#j2b-fixes-fix-what}

<!-- fields -->
- intent: J2b fixes — fix what the review of the moving areas found
- workflow: builder

### p-nc2-sweep-the-reason  {#p-nc2-sweep-the-reason}

<!-- fields -->
- intent: P-NC2 — sweep the reason texts the families wrote
- workflow: builder

### p-z-close-the-catalog  {#p-z-close-the-catalog}

<!-- fields -->
- intent: P-Z — close the catalog
- workflow: builder

### m-p-merge-the-catalog  {#m-p-merge-the-catalog}

<!-- fields -->
- intent: M-P — merge the catalog lane into the loop branch
- workflow: git step

### j2r-re-review-the-fixes  {#j2r-re-review-the-fixes}

<!-- fields -->
- intent: J2r — re-review the fixes, at most three cycles
- workflow: review

### j1-hold-what-the-two  {#j1-hold-what-the-two}

<!-- fields -->
- intent: J1 — hold what the two lanes owe each other
- workflow: builder

### t10b-discharge-the-rows-and  {#t10b-discharge-the-rows-and}

<!-- fields -->
- intent: T10b — discharge the rows and write the build's record
- workflow: builder

### g1-add-the-secret-scanners  {#g1-add-the-secret-scanners}

<!-- fields -->
- intent: G1 — add the secret scanner's allow entry
- workflow: builder

### phase-3-end-clean  {#phase-3-end-clean}

<!-- fields -->
- intent: Phase 3 end — clean the build directories, push every branch and read CI
- workflow: main-session tool run

### h3-ask-whether-to-run  {#h3-ask-whether-to-run}

<!-- fields -->
- intent: H3 — ask whether to run the whole canary after the parts
- workflow: human question

### canary-the-whole-canary  {#canary-the-whole-canary}

<!-- fields -->
- intent: Canary — the whole canary, in the human's own session, if ruled
- workflow: human session

### j3-review-what-the-canary  {#j3-review-what-the-canary}

<!-- fields -->
- intent: J3 — review what the canary changed, in the build session
- workflow: review

### opening-the-fact-sheet-its  {#opening-the-fact-sheet-its}

<!-- fields -->
- intent: Opening — the fact sheet, its questions, the version sentence and the opening commit
- workflow: human session

### h4-ask-which-release-is  {#h4-ask-which-release-is}

<!-- fields -->
- intent: H4 — ask which release is being prepared
- workflow: human question
