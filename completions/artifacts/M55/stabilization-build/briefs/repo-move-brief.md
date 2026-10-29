# Brief — everything we will need again goes into the repository (2026-10-29)

## The human's rules, 2026-10-29 (word for word where quoted)

1. "We should track plans in the repository and not somewhere else (general thing). I know agents tend to put things in
   scratchpads or somewhere else on this computer but then the things are gone. That's fine for testing things out but in
   the end it needs to come into the repo and not somewhere on the machine where it will be forgotten or be deleted!"
2. "That will basically be a file that jigc can hold later. So we should use the rules we have there to reformat that
   doc! That's what we always should do — we do something, improve it and use it later for jigc." Applied: a document we
   write takes the shape of the jigc doctype that would hold it (a plan → the `roadmap` doctype's milestone entry,
   progress → `milestone-record`, a plan's gate answers → `planning-record`), checked with jigc itself; where our need
   does not fit a doctype, the document does not bend — the misfit is a `jigc-feedback` row in
   `completions/artifacts/M55/seed-ledger.md` for the port to triage.
3. Placement, ruled A: the readable plan of the work before the stabilization run's opening is an entry in
   `implementation/roadmap.md`, in the `roadmap` doctype's shape (title, what it proves, the decomposition as prose), between
   M55 and M56, with no new milestone number (a blocker is a fix, never a new milestone); the port migrates it with the rest
   of the roadmap. Its detail, its progress record and its history stay plain records under `completions/artifacts/`
   (the port's recorded direction keeps `completions/artifacts/` plain). Another agent writes that entry — not you.

## Your job — one or two commits on `fix/rc24-tier1`, in the main checkout

You work in `~/projects/gherrink-jigc` on `fix/rc24-tier1` (clean, = origin at `7c34a222`). Paths outside the
published crates commit directly on the loop branch behind the gate. Another builder works in parallel in a separate
worktree on `work/rc24-pre-opening-plan`; it writes `implementation/roadmap.md`, `completions/artifacts/M55/seed-ledger.md`
and `completions/artifacts/M55/stabilization-build/pre-opening-plan/{detail,progress,planning-record}.md`. **Do not touch
those files.** You own `DECISIONS.md`, `implementation/working-principles.md`, the build record's `README.md`, and the new
folders below.

**Read first:** `CLAUDE.md` (Conventions, Public hygiene), `implementation/public-hygiene.md`, `implementation/working-principles.md`,
`implementation/tooling.md` (Where tools live; Candidates), `completions/artifacts/M55/stabilization-build/README.md`.

### A. Move what will be needed again into the repository

Sources (all outside the repository, all to be triaged):
- `~/.claude/projects/<project>/rescue-2026-10-29/scratch/` — 786 text files copied out of the
  session scratchpad before a restart (the scratchpad itself is gone after the restart). Many are clones of this
  repository, rigs and probe trees.
- `~/.claude/projects/<project>/stabilization-briefs/` — the build's briefs, reviews and plan
  versions (`pre-opening-plan*.md`, `*-review.md`, `*-brief.md`, `t-rules.md`, …).
- `~/.claude/plans/` — only the files about this project's stabilization work, if any (read each before deciding).

Keep what someone will need again, or what records how the work was steered: plans and their reviews, briefs, fact sheets
(the draft opening fact sheet `opening/opening-facts.md`), measurement tables, and scripts that were run more than once.
Drop clones, rigs, build output, CI logs (record the run ids instead — every log is re-fetchable with `gh run view`),
duplicates and one-off probe output. Homes:
- `completions/artifacts/M55/stabilization-build/pre-opening-plan/history/` — every plan version (v1 … v5, the as-drafted
  v5) and every review of a plan version, in order.
- `completions/artifacts/M55/stabilization-build/briefs/` — the briefs and reviews of the build tasks (B6–B10, the link fix,
  the CI fixes, the opening fact sheet, the tool index, …), plus the draft opening fact sheet.
- scripts: follow `tooling.md` → *Where tools live* and *Candidates*. A script that is not a registered `dev/` tool never
  lands as an executable file; keep its text inside a markdown file beside the briefs with the question it answers, and point
  its Candidates row at it (adding a row where one is owed). If `tooling-tests/tooling_index.rs` or another fence objects,
  follow the fence, never weaken it.
- one `README.md` per new folder: what is kept, and per dropped group one line on why.

**Sanitize every kept file** — the repository is public and so is every commit: no host path (write repository paths
relative to its root, a home path with `~`, a scratch path as `<scratch>`), no session or agent id, no person's name or
email, no employer or client name, no environment variable value. Run `dev/hygiene-scan` over every new file (and the
private denylist as `public-hygiene.md` describes); **a hit stops you** — fix it, never commit around it. No file over
about 1 MB; never a raw transcript.

### B. The two rules, written down

- `implementation/working-principles.md`: one section per rule (rules 1 and 2 above), in that doc's form — the rule as the
  heading, the human's words, what it means in practice, how to apply it. Rule 2 names the three doctype shapes and the
  seed-ledger route for misfits.
- `DECISIONS.md` (newest first, the file's own entry form): the two rules and ruling 3, dated 2026-10-29. **Then every
  ruling already given and not yet recorded** — check each against `DECISIONS.md` first and never duplicate one:
  regression base A (2026-10-27); catalog first, and the hunter stays on top (2026-10-27); the fourth review cycle allowed
  for the link guard (2026-10-28); F-A → B, F-B → A, F-C → A (2026-10-28); T-NR → B (2026-10-29) with its conditions as
  plan v5 and its §9 state them (one subject agent plus three counted step agents, each launched once; the guard binds
  while the probe runs; scoped to probes of one harness mechanism with one subject agent; a stage, a round or a canary
  still runs in the human's own session); the rulings (a)–(e) plan v2/v3 §0 name. The wording of each is in
  `stabilization-briefs/pre-opening-plan-v5.md` §0.1 and §9, `pre-opening-plan-v2.md`/`-v3.md` §0, and the reviews; quote
  the human's words where the sources carry them, and mark the orchestrator's readings as readings. If a ruling's text
  cannot be found in any source, list it under `left_open` instead of inventing it.
- The build record's `README.md`: one row per new folder, in its table's form.

### Stop before the gate

Return `ready-for-gate` with: the files kept per folder (count and total size), the groups dropped and why, the hygiene
scan's result, the `DECISIONS.md` headings added, and `left_open`. The main session tells you when to run the full
`dev/gate` (in the background, unpiped; one gate at a time on this machine). Then `git commit -F <file>`, push
`fix/rc24-tier1` by name.

```
Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
Claude-Session: <session>
```

No stash, no `rm -rf` on a variable path, never print an environment variable's value, no edits to `.claude/` or
`CLAUDE.md`, the human is "they". Your report ends with a **Tools** section.
