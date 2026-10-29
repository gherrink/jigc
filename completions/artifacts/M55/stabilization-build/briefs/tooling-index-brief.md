# Brief — the tool index, *deterministic first*, and the harvest of agents' tools (2026-10-27)

One builder, one commit on `fix/rc24-tier1` in `~/projects/gherrink-jigc`, from the HEAD you find
(`git log -3`; a dirty tree stops you). **`implementation/working-principles.md` first.**

## The human's ruling, 2026-10-27 → A, with their addition

**Why.** The route hunter was put to the human as owed, though the route catalog — our own deterministic tool — had
since grown to cover most of what the hunter was for; the fact gatherer and the planner each found the tools by grep,
from their own task's angle, and nobody asked what the catalog already establishes. `CLAUDE.md` already carries the
same lesson for one case (*"A tool nothing points at is a tool that gets rebuilt"*, the trial corpus template); nothing
holds it in general. There are 19 tools under `dev/`, the trial tooling's four homes, the deterministic instruments
(the byte table, the route catalog, the regression set, the site list) and the stabilization harness, and no index.

**Ruled (A):**
1. **One index, `implementation/tooling.md`**: one row per tool and per deterministic instrument — the question it
   answers, how it is called, the doc that owns it, the test or fence that holds it true. Points; restates nothing a
   tool's own `--help` or doc owns.
2. **A fence**: every file under `dev/` and every instrument the index's own registry names has a row, and every row's
   paths exist — so a tool cannot be added unlisted, and a row cannot outlive its tool. Put it where the repository's
   tooling tests live (`tooling-tests/`, registered in exactly one group root) and say why that place.
3. **The human's principle, in `implementation/working-principles.md`**, a section of its own, in their words as far as
   they go: *jigc is deterministic, so many things can be checked by a script or tool; whatever a script or test can
   establish is checked there first, before an agent judges, hunts or builds. The tools and scripts exist to support the
   agents in their work. But tools can be incorrect: an agent that suspects a bug in a tool investigates it and reports
   it as a finding with its evidence — it neither works around it nor trusts it blindly.*
4. **The human's addition — harvesting agents' tools:** *agents tend to create scripts in the scratchpad, and the next
   agent builds a similar tool — total waste.* Every agent says, at the end of its work, what script it wrote and kept
   using, or what tool it needed and did not have; the orchestrator triages each into the index's **candidates** list
   (to be built into `dev/` with its tests, or declined with the reason); a candidate built is a row like any other.
   The rule goes into the same principles section; the index carries the candidates list and says who adds to it.
5. **Pointers** — written by the main session, not by you: one line in `CLAUDE.md`, and one **shared paragraph** in
   every agent definition under `.claude/agents/` (the text is below). You build the fence that holds that paragraph
   byte for byte in every definition, as `release_pipeline_fence` arm (q) holds the *Never* paragraph — read how that
   arm is built and follow its pattern (a new arm there, or a suite of its own; say which and why).

The shared paragraph, as the main session will apply it (you may propose a correction, byte for byte, in your first
return):

```
**Tools: use them, doubt them, hand yours on.** jigc is deterministic, so much of what you establish a script or test can establish too — before you judge, hunt or build, look in [tooling.md](../../implementation/tooling.md) for the tool that answers your question, and use it rather than rebuilding it ([working-principles.md](../../implementation/working-principles.md) → *Deterministic first*). A tool can be wrong: if you suspect a bug in one, investigate it and report it as a finding with its evidence — never work around it silently. And if you wrote a script or probe you ran more than once, or needed a tool that does not exist, say so under **Tools** at the end of your report, or in your return where you write no report: what it does, the question it answers, and its text if it is short — a scratch path is gone when you return.
```

(The section title *Deterministic first* is a working name — if you title the principles section otherwise, return the
corrected paragraph.)

## Check before you write

- The harness's return schemas (`.claude/workflow-sources/stabilize.js`): does a harness-launched agent's report or
  return have room for **Tools** without a schema change? If a schema change is needed, say what and stop — the harness
  is the remaining `test`-stage tasks' ground and must not move under them without the orchestrator knowing.
- `planning_gate_home.rs`'s rule that no paragraph under `.claude/` restates a majority of the planning gates, and any
  other fence that reads every definition: run them with the paragraph applied before the gate.

## Process

1. Build the index (enumerate every tool yourself; say the command and the count), the fence (red first: an unlisted
   `dev/` file and a dangling row each named), the principles section, the candidates list, `DECISIONS.md` at the top
   (newest first, dated 2026-10-27: the ruling, its why in a line, the addition).
2. **Stop before the gate** and return: the paragraph (confirmed or corrected), the `CLAUDE.md` line you propose (one
   line, pointing at the index), and the fence's arm name. The main session applies both and resumes you.
3. Then the fences that read the definitions, the full `dev/gate` in the background, read unpiped (never two gates at
   once — the main session tells you if one is running), `git commit -F <file>` with the trailers below, push by name.

```
Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
Claude-Session: <session>
```

Message ends "(the tool index: deterministic first, and agents' tools handed on)". No stash, no `rm -rf` on a variable
path, never print an environment variable's value, no host path / name / id in a file, the human is "they", no edits to
`.claude/` or `CLAUDE.md` yourself.

## Return

Compact: the sha, the gate's `passed=` line, the pushed head, the tool count and how derived, anything left open.
