Honest pass over the session. I'll separate where jigc misled me from where I misread it.

1. What confused me

Slug minting. The rule as stated: "capped at the first 5 words / 50 chars; then a leading or trailing filler word (a/an/the/of/to/in/on/at/by/for) is dropped unless a hyphen glues it to its neighbour." I titled the ADR "Keep the sample store in-memory" specifically expecting in-memory to survive as one hyphen-glued token. It didn't — the hyphen split it, giving keep, the, sample, store, in, memory, truncating at five, then dropping the trailing in → keep-the-sample-store. The phrase "a hyphen glues it to its neighbour" describes the opposite of the observed behaviour for word-counting. The rule explains that _, /, . become separators but never says a literal hyphen already in the title does too.

jigc describe is one undifferentiated wall. ~30 workflows and doctypes as continuous prose, most carrying a paragraph explaining why they're hidden from the router — irrelevant when invoking by name. I truncated it with head -80 and got a partial catalog. There's no --workflows, no describe <name>.

Which docs a task may create isn't visible until you've already minted the task. create-gates prints at the bottom of the composed workflow. Deciding which workflow to start partly depends on what it will let you author, which you learn only after starting one.

2. What jigc told me that was wrong or misleading

jigc doc author silently ignored a corrected title. I re-ran author over an already-staged doc with title: changed from "in-memory" to "in memory". It printed adr:keep-the-sample-store — indistinguishable from success — and left the H1 as # Keep the sample store in-memory. The author help says doc-level leaves "overwrite in place"; title is neither called out as create-only nor reported as ignored. I caught it only by dumping the staged file, and the fix was task discard plus re-minting the entire migration. A one-line "title is create-only; ignored" would have saved that.

"Optional — omit this entry if unused" doesn't omit the section. For the migrated ADR I omitted the options entry as instructed. The renderer emitted an empty ## Options heading anyway, and it shipped into the committed doc at ac08936. That committed ADR now has a hollow section in it. Either omission should drop the heading, or the guidance shouldn't say "omit".

The "fidelity" block oversells itself. The migration review hold printed:

fidelity (heuristic version-scan — fuzzy, advisory; feeds no gate...):
  package@version absent from the rewrite: (none)
  version-like token absent from the rewrite: (none)

For an ADR containing no version strings this is pure noise under a heading that implies content verification. The real fidelity check was me reading the diff. Calling a version-scanner "fidelity" invites exactly the trust the parenthetical then disclaims.

A stale advisory attached itself to unrelated tasks. reconciliation.conformance-block for the unmigrated 0002-...md fired on validate and finalize of the spec task, which never touched that file. It's also named -block while being advisory-severity, which reads as blocking on first encounter.

implement-from-spec printed an empty criteria list as though the spec had none. First invocation showed the "bind the spec" instruction and the criteria header with nothing beneath it, in one output. Criteria only appeared after task bind + re-running. Ordering makes an unbound spec look like an empty one.

Wrong suggestion on a typo. jigc doc read → "tip: a similar subcommand exists: 'create'". show was the verb I wanted; create is a write.

3. What I looked for and didn't find

- A way to read a doc in the active task's working area. jigc doc show "serves the committed store" by design, so there's no supported way to review your own in-flight authoring before finalize. This is the gap that pushed me out of the tool most often. In fairness: QUICKSTART mentions jigc task diff <id>, and I never tried it — the composed workflows only ever point at task validate, so it wasn't in front of me. That one's partly on me.
- --slug on doc create / doc author. It exists on jigc rename. Had it existed on create, the whole discard-and-redo cycle wouldn't have happened.
- A way to change a doc-level title in-task. retitle-item covers repeatable items only; rename operates on committed docs via git mv. For a staged doc the only route was discarding the task.
- A flat listing of criterion ids. The workflow says --format json "prints the same ids" — true, but nested, so I wrote a recursive Python walker to extract eight strings.
- Any project-history view. Nothing in jigc answers "what has this project landed"; I used git log.

4. What I did around jigc, and why

No managed doc was ever written outside jigc — every create/set-slot/set-field/add-item went through the CLI. The workarounds were all reads:

┌─────────────────────────────────────────────┬───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│                    What                     │                                                              Why                                                              │
├─────────────────────────────────────────────┼───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ cat/head on .jigc/tasks/<id>/docs/*.md (4×) │ The only way to see staged output before finalize. This is how I caught both the silent title no-op and the empty ## Options. │
├─────────────────────────────────────────────┼───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ find .jigc/tasks -type f                    │ To learn the working-area layout so I could do the above.                                                                     │
├─────────────────────────────────────────────┼───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ cat docs/decisions/0002-...md               │ Read ADR 0002 before migrating. Unregistered at the time, so doc show wouldn't serve it.                                      │
├─────────────────────────────────────────────┼───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ git log / show / status / remote -v         │ Locating ADR 0002 in history, checking for a diff URL for the changelog link field, confirming a clean tree.                  │
├─────────────────────────────────────────────┼───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ python3 over doc show --format json         │ Extracting criterion ids.                                                                                                     │
└─────────────────────────────────────────────┴───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘

Everything else that touched the filesystem directly was outside jigc's remit by design: src/*.ts, test/wal.test.ts, README.md, .gitignore via Write/Edit, then git add before finalize — the documented flow.

The one-line summary: the write surface held up well and the finalize gates caught real things. The read surface is where it leaked — doc show being committed-only meant that every time I wanted to verify my own work in progress, the supported answer was "finalize and find out," so I went to the filesystem instead. Both defects I found were found that way.