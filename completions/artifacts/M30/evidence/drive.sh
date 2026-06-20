#!/usr/bin/env bash
# M30 inc-4 T4 — the recorded-alongside measure: run the THREE measured facts of
# worked-examples flow 32 on the ACTUALLY-PINNED, `cargo install`-built binary (NOT
# cargo test). The installed `jigc` is taken from PATH with the EMBEDDED dev pack
# (NO JIGC_PACK_DIR) and the `doc-code` probe resolved as the sibling beside the
# installed binary (NO JIGC_DOC_CODE_PROBE) — the production probe-resolution path a
# real install hits, exercising the now-G5 pack (the agent-stages contract).
#
# Mirrors flow 32 (crates/cli/tests/flow32_acceptance.rs) on real git repos. M30's
# narrowing logic shipped Inc 1–3 (StagePolicy::IndexHonoring + the left-out manifest +
# the index-validated doc-code gate); this driver shows the closed loop on the binary a
# real install resolves. Two arms over throwaway repos:
#   SCOPE  staged task edit + unrelated untracked + unrelated unstaged-tracked
#          -> finalize commits ONLY the staged edit, both unrelated files REMAIN, and
#             the emitted output NAMES the left-out set (Facts 1 + 2).
#   BLOCK  a doc citing a Rust symbol the agent WROTE but did NOT stage
#          -> finalize BLOCKS on doc-code.symbol-exists (the index-validated gate from
#             Inc 3), HEAD unchanged, nothing promoted (Fact 3).
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

# Stand up a fresh git repo with one commit + the .jigc/config/ project layer (the
# flow32_acceptance init_repo: no `jigc setup`, the embedded pack provides the workflows).
init_repo() { # $1=repo
  local repo="$1"
  git -C "$repo" init -q
  git -C "$repo" config user.email test@example.com
  git -C "$repo" config user.name Test
  printf 'hello\n' > "$repo/README.md"
  git -C "$repo" add .
  git -C "$repo" commit -q -m initial
  mkdir -p "$repo/.jigc/config"
}

fill_commit() { # $1=repo $2=home $3=task $4=type $5=scope
  local repo="$1" home="$2" task="$3" ty="$4" scope="$5"
  ( cd "$repo"
    HOME="$home" jigc doc set-field "commit:$task#type"  --value "$ty"    >/dev/null 2>&1
    HOME="$home" jigc doc set-field "commit:$task#scope" --value "$scope" >/dev/null 2>&1
    HOME="$home" jigc doc set-slot  "commit:$task#summary" --from-file - <<<"scope the declared change set" >/dev/null 2>&1
    HOME="$home" jigc doc set-slot  "commit:$task#body"    --from-file - <<<"An M30 change."                >/dev/null 2>&1 )
}

# ---------------------------------------------------------------------------------------
# ARM SCOPE — Facts 1 + 2: commits only the declared change-set + names the left-out set.
# ---------------------------------------------------------------------------------------
run_scope() {
  local log="$EVID/scope.log"
  local home repo task
  home=$(mktemp -d /tmp/jigc-m30-home-XXXXXX)
  repo=$(mktemp -d /tmp/jigc-m30-scope-XXXXXX)
  init_repo "$repo"
  task="scope-the-set"

  ( cd "$repo" && HOME="$home" jigc start --workflow single-task "scope the set" >/dev/null 2>&1 )

  # The agent's own task edit — written AND staged (the G5 contract: `git add` first).
  printf 'pub fn feature() {}\n' > "$repo/feature.rs"
  git -C "$repo" add feature.rs
  # An unrelated untracked file — must NOT ride the commit, must be surfaced as left out.
  printf 'private WIP\n' > "$repo/scratch.txt"
  # An unrelated unstaged-modified TRACKED file — same.
  printf 'hello\nlocal edit\n' > "$repo/README.md"

  fill_commit "$repo" "$home" "$task" feat cache

  local before
  before=$(git -C "$repo" rev-list --count HEAD)
  {
    echo "=================================================================="
    echo "### ARM: scope (Facts 1 + 2 — commits only the declared change-set, names the rest)"
    echo "repo: $repo   HEAD-before: $before"
    echo "--- git status --porcelain (pre-finalize) ---"
    git -C "$repo" status --porcelain
    echo "--- jigc task finalize $task ---"
  } >> "$log"
  ( cd "$repo" && HOME="$home" jigc task finalize "$task" ) >> "$log" 2>&1
  local exit=$?
  local after
  after=$(git -C "$repo" rev-list --count HEAD)
  {
    echo
    echo "FINALIZE_EXIT=$exit"
    echo "HEAD-after: $after   (delta $((after - before)))"
    echo "--- git show --name-only --format= HEAD (the landed change-set) ---"
    git -C "$repo" show --name-only --format= HEAD
    echo "--- git status --porcelain (post-finalize — the left-out files REMAIN) ---"
    git -C "$repo" status --porcelain
    echo "--- git show HEAD:README.md (the unrelated local edit must NOT be committed) ---"
    git -C "$repo" show HEAD:README.md
  } >> "$log"
  echo "  scope: exit=$exit  head-delta=$((after - before))  log=$log"
  rm -rf "$repo" "$home"
}

# ---------------------------------------------------------------------------------------
# ARM BLOCK — Fact 3: a cited symbol the agent wrote but did NOT stage blocks finalize.
# ---------------------------------------------------------------------------------------
run_block() {
  local log="$EVID/block.log"
  local home repo task item
  home=$(mktemp -d /tmp/jigc-m30-home-XXXXXX)
  repo=$(mktemp -d /tmp/jigc-m30-block-XXXXXX)
  init_repo "$repo"
  task="document-the-gateway"

  # A tracked widget.rs committed WITHOUT the cited symbol.
  printf 'pub fn placeholder() {}\n' > "$repo/widget.rs"
  git -C "$repo" add widget.rs
  git -C "$repo" commit -q -m "track widget"

  ( cd "$repo"
    HOME="$home" jigc start --workflow architecture-documentation "document the gateway" >/dev/null 2>&1
    HOME="$home" jigc doc create arch-doc --title "Gateway" >/dev/null 2>&1
    HOME="$home" jigc doc set-slot arch-doc:gateway#overview --from-file - <<<"The gateway renders widgets." >/dev/null 2>&1 )
  item=$( cd "$repo" && HOME="$home" jigc doc add-item arch-doc:gateway#components --title "Widget" 2>/dev/null )
  ( cd "$repo"
    HOME="$home" jigc doc set-slot  "$item/description"    --from-file - <<<"Renders a widget." >/dev/null 2>&1
    HOME="$home" jigc doc set-field "$item/implemented-by" --value "widget.rs#render_widget"     >/dev/null 2>&1 )

  # The agent writes the cited symbol into the tracked file but does NOT stage it.
  printf 'pub fn placeholder() {}\npub fn render_widget() {}\n' > "$repo/widget.rs"
  # An unrelated staged file so the narrowed set is non-empty — the block is the doc-code
  # gate, never the empty-commit guard.
  printf 'pub fn other() {}\n' > "$repo/other.rs"
  git -C "$repo" add other.rs

  fill_commit "$repo" "$home" "$task" docs arch-doc

  local before
  before=$(git -C "$repo" rev-list --count HEAD)
  {
    echo "=================================================================="
    echo "### ARM: block (Fact 3 — an unstaged cited symbol blocks finalize)"
    echo "repo: $repo   HEAD-before: $before"
    echo "cited item address: $item/implemented-by   anchor: widget.rs#render_widget"
    echo "--- widget.rs (working tree — render_widget present but NOT staged) ---"
    cat "$repo/widget.rs"
    echo "--- git show :widget.rs (the INDEX — render_widget ABSENT) ---"
    git -C "$repo" show :widget.rs
    echo "--- jigc --format json task finalize $task ---"
  } >> "$log"
  ( cd "$repo" && HOME="$home" jigc --format json task finalize "$task" ) >> "$log" 2>&1
  local exit=$?
  local after
  after=$(git -C "$repo" rev-list --count HEAD)
  {
    echo
    echo "FINALIZE_EXIT=$exit"
    echo "HEAD-after: $after   (delta $((after - before)))"
    if [ -f "$repo/docs/architecture/gateway.md" ] || [ -f "$repo/architecture/gateway.md" ]; then
      echo "PROMOTED: gateway.md PRESENT"
    else
      echo "PROMOTED: gateway.md ABSENT (nothing promoted)"
    fi
  } >> "$log"
  echo "  block: exit=$exit  head-delta=$((after - before))  log=$log"
  rm -rf "$repo" "$home"
}

echo "### running the two arms (installed binary, no overrides)"
run_scope
run_block
echo "### done"
