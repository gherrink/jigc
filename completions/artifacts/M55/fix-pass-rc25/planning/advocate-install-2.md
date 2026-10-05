# Robust-case advocacy for fork install-2

## robust_case

VERDICT: robust-now. One honest correction to the planner's scope: its robust arm closes four of five driven cells of this class; the fifth is at `task finalize` (see costs).

**1. The narrow option does not meet the pass's own contract.** The plan's section 1 says neither install door "exits 0 having destroyed a byte that no git object or index entry holds". I drove the committed-link cell on the installed `jigc 1.0.0-rc.24` in `dev/jigc-rig bare` roots. In every row the mark was present before and absent afterwards from the target and from every blob.

| Cell | Result |
|---|---|
| `.jigc/AGENT.md` link, target untracked | exit 0, `findings: []`, target overwritten |
| `.jigc/version` link | same |
| `.jigc/config/.gitkeep` link | same |
| `.jigc/config/packs.yaml` link | exit 0, comment gone |
| `--force` on the `AGENT.md` cell | exit 0, `findings: []` — the consent names nothing |
| target outside the repository (absolute link) | exit 0, file outside the repo overwritten, `git status` empty |
| target gitignored | exit 0, mark gone |

**2. The refusal's own route leads into the loss.** On a born repo with an uncommitted link at `.jigc/AGENT.md`, rc.24 exits 1 with `setup.dirty-install-path` and the route "commit or stash the work at those path(s) ... then re-run `jigc setup`". I followed the first-named act: `git add .jigc/AGENT.md`, commit, re-run. Result: exit 0, `findings: []`, target overwritten, mark in no object.
- The narrow fix extends this to an unborn `HEAD`. Its new refusal carries a new route that offers "move the file out of the install path, or commit it" (plan section 2).
- Its planned test `an_untracked_symlink_at_a_whole_rewrite_path_on_an_unborn_head_refuses_and_its_target_is_untouched` goes green one step before the target is destroyed.
- The plan's section 3 drove five next steps, but not this one.
- `design/surface-contract.md` → The route fence, P6, calls a route that runs but repairs the wrong thing "law 2 failing one level down".

**3. It is the door's own stated rule.** `crates/cli/src/gitignore.rs::ensure` refuses a symlink because "following it would append to a file outside `.jigc/`"; its module doc says "reject a non-regular, symlinked or undecodable file rather than replace it". Driven on rc.24: exit 1 `setup.init-project-layer`, "it is a symlink".
- Two lines above that call, `adapter::init_project_layer` does a bare `fs::write` of `.gitkeep` that follows a link.
- `adapter::write_bootstrap_file`, `setup::write_version_stamp` and `setup::write_compose_marker` do the same.
- The same lesson is already recorded twice: `setup.rs::workbench_paths` ("a symlink is a leaf rather than a door out of the tree", M49) and `engine/src/finalize.rs` at `plan_promotions` (a symlink "read through" at exit 0, M53).

**4. The "refuses where it succeeded" cost is mostly not a cost.** With a tracked target, rc.24 exits 0 and leaves ` M notes/agent.md`: jigc rewrote a tracked user file outside its install set, unnamed. `HEAD` holds only the mode-120000 link, so a clone gets the user's notes (or a dangling link) at `.jigc/AGENT.md`, not the bootstrap. That exit 0 is a law-1 falsehood, so the refusal replaces a false success.

**5. The standing lessons point one way.**
- `VISION.md` line 51: "discard is explicit, never silent. No silent data loss".
- `design/validation.md`, the `setup.dirty-install-path` row: "never a silent sweep".
- `design/worked-examples.md` flow 52: "none takes a path it never looked at".
- `DECISIONS.md`: "the fix is owed over the sites that exist"; the auditors find "the *class* only when someone drives the axis afterwards".
- The R1-F1 verification already lists the symlink variant as class member 2. Its claim that the guard holds on a born `HEAD` (V18'') tested only an uncommitted link.

**6. Survival of the re-review.** The exit rule sends a tier-1 inside a fix pass's own new code to another fix pass. Narrow ships a new refusal whose route reaches an exit-0 loss in one step, at a cell the planner has already written down. The reviewer who sees V18' closed will try the committed link next.

**7. Reach is very low, and I am not claiming otherwise.** It needs a committed link at a jigc-generated path whose target holds unique bytes. The tier-1 predicate has no likelihood term, which the R1-F1 verifier applied to the parent row.

Spike scripts and outputs: `<scratch>/adv-install2.sh`, `adv-install2b.sh`, `adv-install2c.sh`, `adv-install2g.sh` and their `.out` files.

## what_the_narrow_option_leaves

All of the following were driven on `jigc 1.0.0-rc.24`; the narrow fix touches none of them.

- **State:** a born repository with a committed symlink at `.jigc/AGENT.md`, `.jigc/version`, `.jigc/config/.gitkeep` or `.jigc/config/packs.yaml`, whose target is untracked, gitignored or outside the repository.
- **Door:** `jigc setup`, with or without `--force`.
- **What is lost:** the target's whole content at the first three paths, and its comments and layout at `packs.yaml`. Exit 0, `findings: []`. `git status` is clean when the target is ignored or outside the repo, and nothing is recoverable from any object.
- **Outside-repo case:** the destroyed file is one no git operation in the repository could ever restore.
- **`--force`:** the link is clean against `HEAD`, so the consent names nothing.
- **The laundering route:** an uncommitted link is refused, and the refusal's first-named act (commit) turns it into the committed cell. The re-run then destroys the target at exit 0. This holds on a born `HEAD` today and on an unborn `HEAD` after the narrow fix, through that fix's own new route.
- **Tracked-target case:** no loss, but `setup` exits 0 having rewritten a tracked user file outside its install set, unnamed and left dirty. The install commit holds a link rather than the bootstrap.
- **A known row in the 1.0.0 binary:** the pass's record would grade this "tier-1-by-letter". The exit rule takes the call only when the re-review finds no tier-1 row, so narrow asks the human to take the call over a row the pass itself names, or to run another pass.

## what_the_robust_option_really_costs

**Build.** One `symlink_metadata` check per path, the same one `gitignore::ensure` already has, mapped to the four existing `setup.*` codes. Add repo-relative message text and a true route, about five red tests, and one sentence in the `design/validation.md` row and `setup --help`. That is one commit, and it drops out cleanly if the human holds the boundary.

**Behaviour change.** `setup` goes from exit 0 to exit 1 for any repository with a symlink at one of the four generated paths, including on an upgrade re-run. The common pattern of a symlinked `CLAUDE.md` is not among the four; it is a merged-into member whose writer preserves bytes (read from the code, not driven).

**Three things the planner's account understates:**

- **It closes four of five cells.** I drove the fifth on rc.24: after install, with a committed `.jigc/version` link to an untracked file, `jigc task finalize` exits 0 and the target's bytes are gone (`task.rs::refresh_version_stamp` → `setup::write_version_stamp`). Putting the check at the `setup` door only leaves this one, at the most-used committing door. Its reach is lower than the setup cells', since the user must replace jigc's own stamp with a link to their own unique bytes. The human should choose between one more guard in the shared writer and a recorded row. The guard would make `finalize` fail through its existing rollback path with a bare error and no route.
- **A mid-span refusal is messier than a pre-write one.** I drove the `.jigc/.gitignore` precedent: it refuses after `AGENT.md`, `.gitkeep` and `CLAUDE.md` are written and staged. A user who then follows "remove the link and commit" with a plain `git commit` sweeps those staged install files into their own commit. Nothing is lost, but checking all four paths before the first write is cleaner and matches the plan's "refuses before the first write". The fixer must drive the route at a member that is not the first write.
- **New message and route strings are new surface.** The re-review will grade them. A refusal cannot lose bytes, so the worst outcome is tier 3, which never blocks the call. The precedent's absolute path and "ensure `.jigc/` is writable" route must not be copied.

**What deferral costs.** If the re-review grades the cell tier 1, the price is another fix pass: a fix, an independent review, a record, a new rc stamp and a re-drive of the setup axis. If it is deferred past 1.0.0, a 1.0 `setup` success becomes a refusal in 1.x. That is technically reversible, so the one-way-door tell is weak; the real cost is the exit-rule loop.

## would_it_be_new_mechanism

No.

- It adds no capability, knob, store, doctype or finding code. The four codes exist in `setup.rs::write_install_span`.
- It moves no frozen schema, pinned JSON key, contract version or compose golden.
- It is a guard condition inside an existing door, repeating a rule that door already applies to a sibling file through `gitignore::ensure`.
- `DECISIONS.md` defines the fix-pass boundary as "registry rows and guard conditions only", and the human's later revision adds "Guards inside existing CLI doors are fine regardless."

Two things would cross the line, and neither is needed:
- a general symlink policy or registry across all writers;
- a new finding code for the refusal.

Extending the guard to `finalize` through the shared `write_version_stamp` is still a guard condition, but it changes behaviour at a second door and is a separate choice for the human.

## verdict_for_the_human

You are choosing between a pass that fixes the six verified rows and knowingly ships a driven exit-0 loss at the `setup` door that its own refusal route leads into, and one more commit applying the door's existing symlink rule to four sibling writers, which turns a false success into a refusal for the few repositories that link those paths. Robust-now is the minimal-correct cut, but it closes four of five driven cells: the `.jigc/version` write-through at `task finalize` needs either one more guard or a recorded row, and that is yours to decide.

