#!/usr/bin/env bash
# M31 inc-5 T4 — the recorded-alongside measure: run the FOUR measured facts of the
# M31 fan-out finalize hook story on the ACTUALLY-PINNED, `cargo install`-built binary
# (NOT cargo test). The installed `jigc` is taken from PATH with the EMBEDDED dev pack
# (NO JIGC_PACK_DIR) and the `doc-code` probe resolved as the sibling beside the
# installed binary (NO JIGC_DOC_CODE_PROBE) — the production probe-resolution path a
# real install hits.
#
# Mirrors the M31 acceptance suite (crates/cli/tests/milestone.rs +
# crates/cli/tests/flow33_acceptance.rs) on real git repos. The four arms:
#   SQUASH-TRUE-HOOKS  a squash:true fan-out (2 disjoint-code sub-agents) combines off
#                      the live checkout and runs the user's pre-commit hook against the
#                      combined tree -> ONE aggregate commit, hook FIRES once + relayed,
#                      both sub-agents' code + the merged docs land.
#   SQUASH-FALSE       a squash:false fan-out lays N per-sub-task CODE commits (each
#                      carrying THAT worktree's code, hook fires + relayed per commit) +
#                      the parent aggregate (merged docs) -> N+1 commits, hook relayed
#                      N+1 times.
#   WF2-BLOCK          two sub-agents staging the SAME code path block UP FRONT across
#                      the fan-out, naming the contended path, committing nothing.
#   WIP-SURVIVES       a same-file collision blocks the squash:true combine, and
#                      unrelated MAIN-checkout WIP (untracked + unstaged-tracked) survives
#                      byte-identically (the off-line combine never touches the live tree).
set -u

EVID="$(cd "$(dirname "$0")" && pwd)"

# Guard the production path: NO pack/probe overrides reach the installed binary.
unset JIGC_PACK_DIR
unset JIGC_DOC_CODE_PROBE

JIGC=$(command -v jigc)
echo "### binary under test"
echo "which jigc       : $JIGC"
echo "which doc-code   : $(command -v doc-code)"
echo "jigc sha256      : $(sha256sum "$JIGC" | cut -d' ' -f1)"
echo "doc-code sha256  : $(sha256sum "$(command -v doc-code)" | cut -d' ' -f1)"
echo

HOOK_WARNING="NON-BLOCKING-MILESTONE-HOOK-WARNING"

# A fresh git repo with one commit (the milestone mint pins its base to HEAD). `$2..`
# name extra base files committed into the base (so a sub-agent can rename one).
init_repo() { # $1=repo [extra files...]
  local repo="$1"; shift
  git -C "$repo" init -q
  git -C "$repo" config user.email test@example.com
  git -C "$repo" config user.name Test
  printf 'hello\n' > "$repo/README.md"
  local f
  for f in "$@"; do printf 'shared\n' > "$repo/$f"; done
  git -C "$repo" add .
  git -C "$repo" commit -q -m initial
}

# A non-blocking pre-commit hook: prints HOOK_WARNING to stderr (git folds a hook's own
# stdout onto stderr) and exits 0 — the M19 doc<->code backstop shape. The CLI captures +
# relays its output on EVERY fan-out commit it makes (finalize.md -> never bypass hooks).
install_warning_hook() { # $1=repo
  local hook="$1/.git/hooks/pre-commit"
  printf '#!/bin/sh\necho %s 1>&2\nexit 0\n' "$HOOK_WARNING" > "$hook"
  chmod 0755 "$hook"
}

# Stage a doc body into a sub-task's working area `.jigc/tasks/<sub>/docs/<addr>.md`
# (mirrors milestone.rs stage_doc). Provenance is written separately (write_provenance).
stage_doc() { # $1=repo $2=sub $3=address $4=body
  local docs="$1/.jigc/tasks/$2/docs"
  mkdir -p "$docs"
  printf '%s' "$4" > "$docs/$3.md"
}

# Write the per-sub-task provenance manifest `{"docs": {<addr>: <prov>, ...}}` in one shot
# (the by-task-id join's input alongside the staged bodies).
write_provenance() { # $1=repo $2=sub $3=json-object-of-docs
  local docs="$1/.jigc/tasks/$2/docs"
  mkdir -p "$docs"
  printf '{\n  "docs": %s\n}\n' "$3" > "$docs/provenance.json"
}

# A plain accepted ADR body (no supersedes) — a clean disjoint persisted doc.
adr_plain() { # $1=title
  printf -- '---\nstatus: accepted\ndate: 2026-06-04\n---\n\n# %s\n\n## Context\n\nForces.\n\n## Decision\n\nDo the thing.\n\n## Consequences\n\nTradeoffs.\n' "$1"
}

# A sub-task's authored transient `commit:<sub>` doc (feat header + summary), provenance
# `created`. The prose the squash:false per-sub-task render reads.
commit_doc() { # $1=sub $2=summary
  printf -- '---\ntype: feat\n---\n\n# %s\n\n## Summary\n\n%s\n\n## Body\n\n\n\n## Trailers\n' "$1" "$2"
}

# Write + `git add` a code file IN a provisioned fan-out worktree
# (`.jigc/worktrees/<sub>/<rel>`) — the staged code a fanned-out sub-agent produces in its
# isolated worktree. Stages in the worktree's OWN index, never the main checkout.
stage_worktree_code() { # $1=repo $2=sub $3=rel $4=body
  local wt="$1/.jigc/worktrees/$2"
  mkdir -p "$(dirname "$wt/$3")"
  printf '%s' "$4" > "$wt/$3"
  git -C "$wt" add "$3"
}

# -----------------------------------------------------------------------------------------
# ARM SQUASH-TRUE-HOOKS — a squash:true combine runs the user's pre-commit hook against the
# combined tree (WIP-safe, off-line): ONE aggregate commit, hook fires once + relayed, both
# sub-agents' code + the merged docs land in that one commit.
# -----------------------------------------------------------------------------------------
run_squash_true_hooks() {
  local log="$EVID/squash-true-hooks.log"
  local home repo
  home=$(mktemp -d /tmp/jigc-m31-home-XXXXXX)
  repo=$(mktemp -d /tmp/jigc-m31-sqtrue-XXXXXX)
  init_repo "$repo"
  install_warning_hook "$repo"

  ( cd "$repo"
    HOME="$home" jigc milestone create "Cache rework" >/dev/null 2>&1
    HOME="$home" jigc milestone add-task cache-rework "Area zed" >/dev/null 2>&1
    HOME="$home" jigc milestone add-task cache-rework "Area low" >/dev/null 2>&1 )
  stage_doc "$repo" area-low "adr:low-policy" "$(adr_plain 'Low policy')"
  write_provenance "$repo" area-low '{ "adr:low-policy": "edited-from-base" }'
  stage_doc "$repo" area-zed "adr:zed-policy" "$(adr_plain 'Zed policy')"
  write_provenance "$repo" area-zed '{ "adr:zed-policy": "edited-from-base" }'
  ( cd "$repo" && HOME="$home" jigc milestone provision cache-rework >/dev/null 2>&1 )
  stage_worktree_code "$repo" area-low "src/low.rs" "pub fn low() {}\n"
  stage_worktree_code "$repo" area-zed "src/zed.rs" "pub fn zed() {}\n"

  local before
  before=$(git -C "$repo" rev-list --count HEAD)
  {
    echo "=================================================================="
    echo "### ARM: squash-true-hooks (the squash:true combine runs the user's pre-commit hook)"
    echo "repo: $repo   HEAD-before-commits: $before   (default squash:true — no override)"
    echo "--- jigc milestone finalize cache-rework ---"
  } >> "$log"
  local out
  out=$( cd "$repo" && HOME="$home" jigc milestone finalize cache-rework 2>/dev/null )
  local exit=$?
  local after
  after=$(git -C "$repo" rev-list --count HEAD)
  {
    echo "$out"
    echo
    echo "FINALIZE_EXIT=$exit"
    echo "HEAD-after-commits: $after   (delta $((after - before)) — squash:true lands ONE aggregate)"
    echo "HOOK_WARNING relays in stdout: $(printf '%s' "$out" | grep -c "$HOOK_WARNING") (expect 1 — hook fires once on the combine)"
    echo "has '--- hook output ---' delimiter: $(printf '%s' "$out" | grep -c -- '--- hook output ---')"
    echo "--- git show --name-only --format= HEAD (the one combine commit carries both code + merged docs) ---"
    git -C "$repo" show --name-only --format= HEAD
    echo "--- git status --porcelain (main checkout clean after the combine + teardown) ---"
    git -C "$repo" status --porcelain
  } >> "$log"
  echo "  squash-true-hooks: exit=$exit  head-delta=$((after - before))  hook-relays=$(printf '%s' "$out" | grep -c "$HOOK_WARNING")  log=$log"
  rm -rf "$repo" "$home"
}

# -----------------------------------------------------------------------------------------
# ARM SQUASH-FALSE — N per-sub-task CODE commits (hook fires + relayed each) + the parent
# aggregate (merged docs). N+1 commits, hook relayed N+1 times, each per-sub-task commit
# carries THAT sub-task's code (a real tree, not the retired --allow-empty form).
# -----------------------------------------------------------------------------------------
run_squash_false() {
  local log="$EVID/squash-false.log"
  local home repo
  home=$(mktemp -d /tmp/jigc-m31-home-XXXXXX)
  repo=$(mktemp -d /tmp/jigc-m31-sqfalse-XXXXXX)
  init_repo "$repo"
  install_warning_hook "$repo"

  # Opt into squash:false via the project config layer.
  mkdir -p "$repo/.jigc/config"
  printf 'scalar:\n  finalize.fan-out.squash: false\n' > "$repo/.jigc/config/manifest.yaml"

  ( cd "$repo"
    HOME="$home" jigc milestone create "Cache rework" >/dev/null 2>&1
    HOME="$home" jigc milestone add-task cache-rework "Area zed" >/dev/null 2>&1
    HOME="$home" jigc milestone add-task cache-rework "Area low" >/dev/null 2>&1 )
  # Each sub-task: a clean disjoint persisted ADR + its own authored commit doc.
  stage_doc "$repo" area-low "adr:low-policy" "$(adr_plain 'Low policy')"
  stage_doc "$repo" area-low "commit:area-low" "$(commit_doc area-low 'rework the low cache path')"
  write_provenance "$repo" area-low '{ "adr:low-policy": "edited-from-base", "commit:area-low": "created" }'
  stage_doc "$repo" area-zed "adr:zed-policy" "$(adr_plain 'Zed policy')"
  stage_doc "$repo" area-zed "commit:area-zed" "$(commit_doc area-zed 'rework the zed cache path')"
  write_provenance "$repo" area-zed '{ "adr:zed-policy": "edited-from-base", "commit:area-zed": "created" }'
  ( cd "$repo" && HOME="$home" jigc milestone provision cache-rework >/dev/null 2>&1 )
  stage_worktree_code "$repo" area-low "src/low.rs" "pub fn low() {}\n"
  stage_worktree_code "$repo" area-zed "src/zed.rs" "pub fn zed() {}\n"

  local before
  before=$(git -C "$repo" rev-list --count HEAD)
  {
    echo "=================================================================="
    echo "### ARM: squash-false (N per-sub-task code commits + parent aggregate; hook relayed each)"
    echo "repo: $repo   HEAD-before-commits: $before   (finalize.fan-out.squash: false override)"
    echo "--- jigc milestone finalize cache-rework ---"
  } >> "$log"
  local out
  out=$( cd "$repo" && HOME="$home" jigc milestone finalize cache-rework 2>/dev/null )
  local exit=$?
  local after
  after=$(git -C "$repo" rev-list --count HEAD)
  {
    echo "$out"
    echo
    echo "FINALIZE_EXIT=$exit"
    echo "HEAD-after-commits: $after   (delta $((after - before)) — expect 3 = 2 sub-task + 1 aggregate)"
    echo "HOOK_WARNING relays in stdout: $(printf '%s' "$out" | grep -c "$HOOK_WARNING") (expect 3 — every fan-out commit relays its hook)"
    echo "--- commit subjects (oldest-first; the id-sorted sub-task sequence then the aggregate) ---"
    git -C "$repo" log -3 --reverse --format='%s'
    echo "--- per-sub-task commit trees (each carries THAT worktree's code, not an empty tree) ---"
    echo "[HEAD~2 = first sub-task commit] git show --stat:"
    git -C "$repo" show --stat --format='  %s' HEAD~2
    echo "[HEAD~1 = second sub-task commit] git show --stat:"
    git -C "$repo" show --stat --format='  %s' HEAD~1
    echo "[HEAD = parent aggregate] git show --name-only (the MERGED docs ride here, not per-sub-task) :"
    git -C "$repo" show --name-only --format='  %s' HEAD
    echo "--- git status --porcelain (working tree clean after the boundary) ---"
    git -C "$repo" status --porcelain
  } >> "$log"
  echo "  squash-false: exit=$exit  head-delta=$((after - before))  hook-relays=$(printf '%s' "$out" | grep -c "$HOOK_WARNING")  log=$log"
  rm -rf "$repo" "$home"
}

# -----------------------------------------------------------------------------------------
# ARM WF2-BLOCK — two sub-agents staging the SAME code path block UP FRONT across the
# fan-out, naming the contended path, committing nothing (HEAD unchanged), promoting nothing.
# -----------------------------------------------------------------------------------------
run_wf2_block() {
  local log="$EVID/wf2-block.log"
  local home repo
  home=$(mktemp -d /tmp/jigc-m31-home-XXXXXX)
  repo=$(mktemp -d /tmp/jigc-m31-wf2-XXXXXX)
  init_repo "$repo"

  mkdir -p "$repo/.jigc/config"
  printf 'scalar:\n  finalize.fan-out.squash: false\n' > "$repo/.jigc/config/manifest.yaml"

  ( cd "$repo"
    HOME="$home" jigc milestone create "Cache rework" >/dev/null 2>&1
    HOME="$home" jigc milestone add-task cache-rework "Area zed" >/dev/null 2>&1
    HOME="$home" jigc milestone add-task cache-rework "Area low" >/dev/null 2>&1 )
  stage_doc "$repo" area-low "adr:low-policy" "$(adr_plain 'Low policy')"
  stage_doc "$repo" area-low "commit:area-low" "$(commit_doc area-low 'rework the low cache path')"
  write_provenance "$repo" area-low '{ "adr:low-policy": "edited-from-base", "commit:area-low": "created" }'
  stage_doc "$repo" area-zed "adr:zed-policy" "$(adr_plain 'Zed policy')"
  stage_doc "$repo" area-zed "commit:area-zed" "$(commit_doc area-zed 'rework the zed cache path')"
  write_provenance "$repo" area-zed '{ "adr:zed-policy": "edited-from-base", "commit:area-zed": "created" }'
  ( cd "$repo" && HOME="$home" jigc milestone provision cache-rework >/dev/null 2>&1 )
  # Both sub-agents stage the SAME path src/shared.rs in their isolated worktrees.
  stage_worktree_code "$repo" area-low "src/shared.rs" "fn low() {}\n"
  stage_worktree_code "$repo" area-zed "src/shared.rs" "fn zed() {}\n"

  local before_head before_count
  before_head=$(git -C "$repo" rev-parse HEAD)
  before_count=$(git -C "$repo" rev-list --count HEAD)
  {
    echo "=================================================================="
    echo "### ARM: wf2-block (the WF2 same-file block fires across the fan-out)"
    echo "repo: $repo   HEAD-before: $before_head"
    echo "both area-low and area-zed staged src/shared.rs in their own worktrees"
    echo "--- jigc milestone finalize cache-rework (stdout+stderr) ---"
  } >> "$log"
  local out err exit
  out=$( cd "$repo" && HOME="$home" jigc milestone finalize cache-rework 2>"$EVID/.wf2.err" )
  exit=$?
  err=$(cat "$EVID/.wf2.err"); rm -f "$EVID/.wf2.err"
  local after_head after_count
  after_head=$(git -C "$repo" rev-parse HEAD)
  after_count=$(git -C "$repo" rev-list --count HEAD)
  {
    echo "$out"
    echo "$err"
    echo
    echo "FINALIZE_EXIT=$exit (non-zero — the collision blocks)"
    echo "names the contended path 'src/shared.rs': $(printf '%s\n%s' "$out" "$err" | grep -c 'src/shared.rs')"
    echo "HEAD before==after (nothing committed): $before_head == $after_head"
    echo "commit count before==after: $before_count == $after_count"
    # "promoted nothing" = no ADR doc FILE landed (flow33 idiom — git does not track an empty
    # parent dir, so the rollback's leftover empty docs/decisions/ is invisible + harmless).
    if [ -f "$repo/docs/decisions/low-policy.md" ] || [ -f "$repo/docs/decisions/zed-policy.md" ]; then
      echo "PROMOTED: an ADR doc file PRESENT (UNEXPECTED)"
    else
      echo "PROMOTED: no ADR doc file (nothing promoted)"
    fi
  } >> "$log"
  echo "  wf2-block: exit=$exit  head-unchanged=$([ "$before_head" = "$after_head" ] && echo yes || echo NO)  log=$log"
  rm -rf "$repo" "$home"
}

# -----------------------------------------------------------------------------------------
# ARM WIP-SURVIVES — a same-file collision blocks the squash:true combine, and unrelated
# MAIN-checkout WIP (untracked + unstaged-tracked) survives byte-identically (review S2 —
# the off-line temp-index build never touches the live checkout).
# -----------------------------------------------------------------------------------------
run_wip_survives() {
  local log="$EVID/wip-survives.log"
  local home repo
  home=$(mktemp -d /tmp/jigc-m31-home-XXXXXX)
  repo=$(mktemp -d /tmp/jigc-m31-wip-XXXXXX)
  # `shared.txt` lives in the base tree so a sub-agent can rename it.
  init_repo "$repo" shared.txt

  ( cd "$repo"
    HOME="$home" jigc milestone create "Cache rework" >/dev/null 2>&1
    HOME="$home" jigc milestone add-task cache-rework "Area zed" >/dev/null 2>&1
    HOME="$home" jigc milestone add-task cache-rework "Area low" >/dev/null 2>&1 )
  stage_doc "$repo" area-low "adr:low-policy" "$(adr_plain 'Low policy')"
  write_provenance "$repo" area-low '{ "adr:low-policy": "edited-from-base" }'
  stage_doc "$repo" area-zed "adr:zed-policy" "$(adr_plain 'Zed policy')"
  write_provenance "$repo" area-zed '{ "adr:zed-policy": "edited-from-base" }'
  ( cd "$repo" && HOME="$home" jigc milestone provision cache-rework >/dev/null 2>&1 )
  # area-low EDITS shared.txt; area-zed RENAMES it — the rename old-path collides.
  printf 'shared, edited by low\n' > "$repo/.jigc/worktrees/area-low/shared.txt"
  git -C "$repo/.jigc/worktrees/area-low" add shared.txt
  git -C "$repo/.jigc/worktrees/area-zed" mv shared.txt moved.txt

  # Seed UNRELATED WIP in the MAIN checkout: an untracked file + an unstaged tracked edit.
  printf 'scratch\n' > "$repo/wip-untracked.txt"
  printf 'hello\nlocal WIP\n' > "$repo/README.md"
  local status_before head_before
  status_before=$(git -C "$repo" status --porcelain)
  head_before=$(git -C "$repo" rev-parse HEAD)

  {
    echo "=================================================================="
    echo "### ARM: wip-survives (a blocked squash:true combine leaves main WIP byte-identical)"
    echo "repo: $repo   HEAD-before: $head_before   (default squash:true — no override)"
    echo "--- git status --porcelain (pre-finalize: seeded unrelated WIP) ---"
    echo "$status_before"
    echo "--- jigc milestone finalize cache-rework (stdout+stderr) ---"
  } >> "$log"
  local out exit
  out=$( cd "$repo" && HOME="$home" jigc milestone finalize cache-rework 2>"$EVID/.wip.err" )
  exit=$?
  local err; err=$(cat "$EVID/.wip.err"); rm -f "$EVID/.wip.err"
  local head_after status_after
  head_after=$(git -C "$repo" rev-parse HEAD)
  status_after=$(git -C "$repo" status --porcelain)
  local untracked readme
  untracked=$(cat "$repo/wip-untracked.txt")
  readme=$(cat "$repo/README.md")
  {
    echo "$out"
    echo "$err"
    echo
    echo "FINALIZE_EXIT=$exit (non-zero — the collision blocks the combine)"
    echo "HEAD before==after (nothing committed): $head_before == $head_after"
    echo "--- unrelated MAIN WIP after the blocked combine (must be byte-identical) ---"
    echo "wip-untracked.txt = [$untracked]   (expect [scratch])"
    echo "README.md = [$(printf '%s' "$readme" | tr '\n' '|')]   (expect [hello|local WIP])"
    echo "status before==after byte-identical: $([ "$status_before" = "$status_after" ] && echo yes || echo NO)"
    # "promoted nothing" = no ADR doc FILE landed (the empty leftover docs/decisions/ dir is
    # git-untracked, so it never reaches the byte-identical status above — WIP-safety holds).
    if [ -f "$repo/docs/decisions/low-policy.md" ] || [ -f "$repo/docs/decisions/zed-policy.md" ]; then
      echo "PROMOTED: an ADR doc file PRESENT (UNEXPECTED)"
    else
      echo "PROMOTED: no ADR doc file (nothing promoted)"
    fi
  } >> "$log"
  echo "  wip-survives: exit=$exit  head-unchanged=$([ "$head_before" = "$head_after" ] && echo yes || echo NO)  wip-intact=$([ "$status_before" = "$status_after" ] && echo yes || echo NO)  log=$log"
  rm -rf "$repo" "$home"
}

# -----------------------------------------------------------------------------------------
# ARM WIP-SURVIVES-SUCCESS — a SUCCESSFUL squash:true combine (two disjoint-code sub-agents)
# leaves unrelated MAIN-checkout WIP byte-identical (the success-path twin of WIP-SURVIVES,
# mirroring crates/cli/tests/flow33_acceptance.rs::flow33_unrelated_wip_survives_successful_combine).
# The dec61c7 fix lands the combine via `git merge --ff-only`, NOT `git reset --hard` — so
# unstaged WIP a human is editing in the main checkout rides the fast-forward intact.
# -----------------------------------------------------------------------------------------
run_wip_survives_success() {
  local log="$EVID/wip-survives-success.log"
  local home repo
  home=$(mktemp -d /tmp/jigc-m31-home-XXXXXX)
  repo=$(mktemp -d /tmp/jigc-m31-wipok-XXXXXX)
  init_repo "$repo"
  install_warning_hook "$repo"

  ( cd "$repo"
    HOME="$home" jigc milestone create "Cache rework" >/dev/null 2>&1
    HOME="$home" jigc milestone add-task cache-rework "Area zed" >/dev/null 2>&1
    HOME="$home" jigc milestone add-task cache-rework "Area low" >/dev/null 2>&1 )
  stage_doc "$repo" area-low "adr:low-policy" "$(adr_plain 'Low policy')"
  write_provenance "$repo" area-low '{ "adr:low-policy": "edited-from-base" }'
  stage_doc "$repo" area-zed "adr:zed-policy" "$(adr_plain 'Zed policy')"
  write_provenance "$repo" area-zed '{ "adr:zed-policy": "edited-from-base" }'
  ( cd "$repo" && HOME="$home" jigc milestone provision cache-rework >/dev/null 2>&1 )
  # Disjoint code in each worktree — the combine SUCCEEDS (no collision).
  stage_worktree_code "$repo" area-low "src/low.rs" "pub fn low() {}\n"
  stage_worktree_code "$repo" area-zed "src/zed.rs" "pub fn zed() {}\n"

  # Seed UNRELATED unstaged tracked WIP in the MAIN checkout, the way a human edits.
  printf 'hello\nUNRELATED WIP THE USER IS EDITING\n' > "$repo/README.md"
  local readme_before head_before
  readme_before=$(cat "$repo/README.md")
  head_before=$(git -C "$repo" rev-list --count HEAD)

  {
    echo "=================================================================="
    echo "### ARM: wip-survives-success (a SUCCESSFUL squash:true combine leaves main WIP byte-identical)"
    echo "repo: $repo   HEAD-before-commits: $head_before   (default squash:true — no override)"
    echo "--- README.md (pre-finalize: seeded unrelated unstaged WIP) ---"
    echo "README.md = [$(printf '%s' "$readme_before" | tr '\n' '|')]"
    echo "--- jigc milestone finalize cache-rework ---"
  } >> "$log"
  local out exit
  out=$( cd "$repo" && HOME="$home" jigc milestone finalize cache-rework 2>/dev/null )
  exit=$?
  local head_after readme_after
  head_after=$(git -C "$repo" rev-list --count HEAD)
  readme_after=$(cat "$repo/README.md")
  {
    echo "$out"
    echo
    echo "FINALIZE_EXIT=$exit (zero — the combine SUCCEEDS)"
    echo "HEAD-after-commits: $head_after   (delta $((head_after - head_before)) — squash:true lands ONE aggregate)"
    echo "--- unrelated MAIN WIP after the SUCCESSFUL combine (must be byte-identical) ---"
    echo "README.md = [$(printf '%s' "$readme_after" | tr '\n' '|')]   (expect [hello|UNRELATED WIP THE USER IS EDITING|])"
    echo "README byte-identical before==after: $([ "$readme_before" = "$readme_after" ] && echo yes || echo NO)"
    echo "--- the unrelated WIP stays OUT of the aggregate commit (git show --name-only HEAD) ---"
    git -C "$repo" show --name-only --format= HEAD
  } >> "$log"
  echo "  wip-survives-success: exit=$exit  head-delta=$((head_after - head_before))  wip-intact=$([ "$readme_before" = "$readme_after" ] && echo yes || echo NO)  log=$log"
  rm -rf "$repo" "$home"
}

echo "### running the five arms (installed binary, no overrides)"
run_squash_true_hooks
run_squash_false
run_wf2_block
run_wip_survives
run_wip_survives_success
echo "### done"
