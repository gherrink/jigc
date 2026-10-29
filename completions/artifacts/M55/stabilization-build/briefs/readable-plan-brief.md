# Brief — the plan before the opening, written for the human and in jigc's own shapes (2026-10-29)

## Why

The human read the detailed plan (`stabilization-briefs/pre-opening-plan-v5.md`, 591 lines, with v4 for the parts v5
marks "as in v4") and said: "that plan is really hard to read for a human. Too many cross references, numbers … I'm not
able to read this or it will take me really long to understand." They also cannot run the machine one to three weeks
straight: "we will need breaks in between and do it in steps." And: "That will basically be a file that jigc can hold
later. So we should use the rules we have there to reformat that doc!"

Ruled (A): the readable plan is an **entry in `implementation/roadmap.md`**, in the shape of jigc's `roadmap` doctype
(`crates/cli/packs/methodology/schemas/roadmap.yaml`: an entry is a title, a `proves` slot and a `decomposition` slot —
"the increments, as prose: each increment's deliverable and the tasks that build it, one level"), placed between the M55
and M56 entries, with **no new milestone number** (a blocker is a fix, never a new milestone). The port migrates it with
the rest of the roadmap. Its detail, progress and gate answers are plain records under `completions/artifacts/`.

## Where to work

Your own worktree, persistent (not the scratchpad): `git -C ~/projects/gherrink-jigc fetch origin`, then
`git -C ~/projects/gherrink-jigc worktree add -b work/rc24-pre-opening-plan ~/projects/gherrink-jigc-plan 7c34a222`.
Your own `CARGO_TARGET_DIR` inside it. Another builder works in the main checkout in parallel and owns `DECISIONS.md`,
`implementation/working-principles.md`, the build record's `README.md` and the folders `pre-opening-plan/history/` and
`briefs/`. **You do not touch those.** You write exactly:

1. **`implementation/roadmap.md` — the entry.** Read first: `design/methodology-docs.md` (the roadmap doctype, the
   one-level bound), `roadmap.yaml`, and how jigc writes an entry (`jigc doc schema roadmap`; author one in a throwaway
   repository through jigc with the methodology pack — `dev/jigc-rig` and `crates/cli/guides/QUICKSTART.md` show how — and
   copy the rendered shape). Write our entry in that rendered shape. The readers are the human first and builder agents
   second. Rules for the text:
   - plain words, short sentences; **no internal task codes** (no `P-F1`, `M-TNR`, `J2a`, `H0`…), no line numbers, no
     `file:line` citations, no finding ids; name a thing by what it is ("the route catalog", "the final review of the
     test stage");
   - `proves`: one short paragraph — what is true when this work is done, and why it must be true before the opening;
   - `decomposition`: the work as **steps**, each one a paragraph or a short list: what it gives us, the tasks that build
     it (in words), what the human is asked in it (if anything) with the recommendation, roughly how many hours of
     machine time (an estimate, say so), and that it **ends with everything committed and pushed and nothing running, so
     the machine can be switched off after it**. Cut the steps so each is roughly one working day or less where the
     dependencies allow; a step that cannot end at a safe break says why. Keep v5's order and dependencies exactly — you
     re-cut and re-word, you do not re-plan. One line at the end points at the detail and the progress record.
   - Short. A reader should get through it in ten minutes.
2. **`completions/artifacts/M55/stabilization-build/pre-opening-plan/detail.md`** — the builders' spec: v5 made
   self-contained (the "as in v4" parts filled in from v4, §9's corrections applied in place, the changelogs and review
   tables moved to the end as history). Sanitized. Here the task codes stay, and each step of the roadmap entry names the
   tasks of the detail it covers, so the two cannot drift: one table at the top of `detail.md` maps step → tasks.
3. **`…/pre-opening-plan/progress.md`** — the progress record in the shape of jigc's `milestone-record` doctype
   (`milestone-record.yaml`: `base`, `status`, and an ordered `tasks` list with `task-id`, `intent`, `workflow`, `status`),
   rendered as jigc renders it. Base `7c34a222`; every task of the detail in order, `intent` in plain words; status values
   as the doctype's enum allows. If the enum cannot say "not started" or "waiting for the human", do not bend it — that is
   a misfit (item 5).
4. **`…/pre-opening-plan/planning-record.md`** — the plan's gate answers in the shape of jigc's `planning-record` doctype
   (`planning-record.yaml`, its fourteen gates): each answered briefly from what the reviews established (the plan's
   reviews are in `stabilization-briefs/pre-opening-plan*-review.md` and the checks recorded in v5 §8/§9), or N/A with why.
5. **`completions/artifacts/M55/seed-ledger.md` — one `jigc-feedback` row per misfit** you meet, in that ledger's own
   grammar (read its header and the fence that reads it). Likely candidates: the roadmap's decomposition has no place for
   "what the human is asked here" or "a safe break"; no doctype holds a builder-level task spec; the decomposition is one
   level while this plan is steps with tasks under them (note: the recorded trigger for a deeper level asks for a second
   driver that is not this repository's own planning convenience, so this row alone does not fire it — say so in the row);
   anything the jigc check below turns up. A misfit is a finding, never a requirement.

**Check with jigc itself:** author the roadmap entry, the progress record and the planning record in a throwaway
repository through the installed or dev binary (methodology pack), validate them, and compare the rendered shape with
what you wrote. Put the commands and their outcome in your report. A shape jigc refuses is a misfit row, not a reason to
change the shape by hand.

**Sanitize** every file (public repository): no host path, no session or agent id, no person's or employer's name.
`dev/hygiene-scan` over what you wrote; a hit stops you.

## Stop before the gate

Return `ready-for-gate` with: the roadmap entry's text in full (it goes to the human), the step → tasks map, the jigc check
(commands, outcomes), the seed-ledger rows added, and `left_open`. The main session tells you when to run the full
`dev/gate` (`--private-target`, background, unpiped; one gate at a time). Then `git commit -F <file>` on
`work/rc24-pre-opening-plan` and push it by name. You do not merge; the main session does.

```
Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
Claude-Session: <session>
```

No stash, no `rm -rf` on a variable path, never print an environment variable's value, no edits to `.claude/`,
`CLAUDE.md` or `DECISIONS.md`, the human is "they". Your report ends with a **Tools** section.
