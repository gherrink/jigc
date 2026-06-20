#!/usr/bin/env bash
# M28 inc-2 T4 — the F4/F5 recorded-alongside measure: run the THREE measured facts of
# worked-examples flow 30 on the ACTUALLY-PINNED, `cargo install`-built binary (NOT
# cargo test). The installed `jigc` is taken from PATH with the EMBEDDED dev pack
# (NO JIGC_PACK_DIR) and the `doc-code` probe resolved as the sibling beside the
# installed binary (NO JIGC_DOC_CODE_PROBE) — the production probe-resolution path a
# real install hits, exercising the M28 SEVEN-grammar set (M27's six + CSS).
#
# Mirrors flow 30 (crates/cli/tests/flow30_acceptance.rs) on a CSS-bearing repo. Both
# components anchor the SAME stylesheet at different selectors — the sharpest per-item
# disambiguation fixture, since the only lever that picks A from B is the per-selector
# CSS extractor over the same styles.css:
#   component A → a real CSS class (styles.css#card,  a class_selector → class_name)
#   component B → a real CSS class (styles.css#title, a class_selector → class_name)
# Three arms over the SAME authored arch-doc fixture (the per-anchor pass<->block flip
# is the masking guard — a resolving doc-code check emits no finding):
#   BLOCK-CARD  drop .card rule, .title stays   -> block names A's addr, never B's (== DISAMBIG)
#   BLOCK-TITLE drop .title rule, .card stays    -> block names B's addr, never A's
#   PASS        both classes present             -> one docs(arch-doc): commit, promoted
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

# --- the shared stylesheet: .card (component A) + .title (component B) -------------
CARD_RULE=$'.card {\n  border: 1px solid black;\n}\n'
TITLE_RULE=$'.title {\n  font-weight: bold;\n}\n'

# Compose styles.css from whichever class rules are present. Dropping a rule renames its
# selector away (the file stays, the selector vanishes) — a selector-absent block, never
# a file-absent one. The arms below never drop both, so the sheet is never empty.
stylesheet() { # $1=card(1/0) $2=title(1/0)
  local out=""
  [ "$1" = 1 ] && out+="$CARD_RULE"$'\n'
  [ "$2" = 1 ] && out+="$TITLE_RULE"
  printf '%s' "$out"
}

# Stand up a fresh CSS-bearing scratch repo with the flow-30 fixture, set up + the cited
# adr committed. Echoes the repo path. $1=HOME, $2=card present, $3=title present.
make_repo() {
  local home="$1" card="$2" title="$3"
  local repo
  repo=$(mktemp -d /tmp/jigc-m28-flow30-XXXXXX)
  git -C "$repo" init -q
  git -C "$repo" config user.email t@e.com
  git -C "$repo" config user.name T
  stylesheet "$card" "$title" > "$repo/styles.css"
  git -C "$repo" add -A
  git -C "$repo" commit -q -m initial

  ( cd "$repo" && HOME="$home" jigc setup >/dev/null 2>&1 )

  # The committed adr the arch-doc cites (committed-first ordering).
  ( cd "$repo" && HOME="$home" jigc start --workflow single-task "decide the cache strategy" >/dev/null 2>&1
    HOME="$home" jigc doc create adr --title "Use a cache" >/dev/null 2>&1
    HOME="$home" jigc doc set-slot adr:use-a-cache#context      --from-file - <<<"Lookups must stay fast." >/dev/null 2>&1
    HOME="$home" jigc doc set-slot adr:use-a-cache#decision     --from-file - <<<"Cache the index."        >/dev/null 2>&1
    HOME="$home" jigc doc set-slot adr:use-a-cache#consequences --from-file - <<<"A cold node re-warms."   >/dev/null 2>&1
    HOME="$home" jigc doc set-field commit:decide-the-cache-strategy#type  --value docs   >/dev/null 2>&1
    HOME="$home" jigc doc set-field commit:decide-the-cache-strategy#scope --value adr    >/dev/null 2>&1
    HOME="$home" jigc doc set-slot  commit:decide-the-cache-strategy#summary --from-file - <<<"decide the cache strategy" >/dev/null 2>&1
    HOME="$home" jigc doc set-slot  commit:decide-the-cache-strategy#body    --from-file - <<<"Pick the cache strategy."   >/dev/null 2>&1
    HOME="$home" jigc task finalize decide-the-cache-strategy >/dev/null 2>&1 )

  echo "$repo"
}

# Author the two-component arch-doc (A->styles.css#card, B->styles.css#title) in $1=repo, $2=HOME.
author_arch_doc() {
  local repo="$1" home="$2"
  ( cd "$repo"
    HOME="$home" jigc start --workflow architecture-documentation "document the gateway" >/dev/null 2>&1
    HOME="$home" jigc doc create arch-doc --title "Gateway" >/dev/null 2>&1
    HOME="$home" jigc doc set-slot  arch-doc:gateway#overview --from-file - <<<"The gateway renders the card surface and its heading." >/dev/null 2>&1
    HOME="$home" jigc doc set-field arch-doc:gateway#cites --value adr:use-a-cache >/dev/null 2>&1
    HOME="$home" jigc doc add-item  arch-doc:gateway#components --title "Card" >/dev/null 2>&1
    HOME="$home" jigc doc set-slot  arch-doc:gateway#components/card/description --from-file - <<<"The card surface container." >/dev/null 2>&1
    HOME="$home" jigc doc set-field arch-doc:gateway#components/card/implemented-by --value "styles.css#card" >/dev/null 2>&1
    HOME="$home" jigc doc add-item  arch-doc:gateway#components --title "Title" >/dev/null 2>&1
    HOME="$home" jigc doc set-slot  arch-doc:gateway#components/title/description --from-file - <<<"The card heading." >/dev/null 2>&1
    HOME="$home" jigc doc set-field arch-doc:gateway#components/title/implemented-by --value "styles.css#title" >/dev/null 2>&1
    HOME="$home" jigc doc set-field commit:document-the-gateway#type  --value docs     >/dev/null 2>&1
    HOME="$home" jigc doc set-field commit:document-the-gateway#scope --value arch-doc >/dev/null 2>&1
    HOME="$home" jigc doc set-slot  commit:document-the-gateway#summary --from-file - <<<"document the gateway" >/dev/null 2>&1
    HOME="$home" jigc doc set-slot  commit:document-the-gateway#body    --from-file - <<<"Living architecture doc for the gateway." >/dev/null 2>&1 )
}

run_arm() {
  local tag="$1" card="$2" title="$3"
  local log="$EVID/$tag.log"
  local home repo
  home=$(mktemp -d /tmp/jigc-m28-home-XXXXXX)
  repo=$(make_repo "$home" "$card" "$title")
  author_arch_doc "$repo" "$home"
  local before
  before=$(git -C "$repo" rev-list --count HEAD)
  {
    echo "=================================================================="
    echo "### ARM: $tag"
    echo "repo: $repo   HEAD-before: $before"
    echo "styles.css selectors present:$([ "$card" = 1 ] && echo ' .card')$([ "$title" = 1 ] && echo ' .title')"
    echo "--- styles.css ---"
    cat "$repo/styles.css"
    echo "--- jigc --format json task finalize document-the-gateway ---"
  } >> "$log"
  ( cd "$repo" && HOME="$home" jigc --format json task finalize document-the-gateway ) >> "$log" 2>&1
  local exit=$?
  local after
  after=$(git -C "$repo" rev-list --count HEAD)
  {
    echo
    echo "FINALIZE_EXIT=$exit"
    echo "HEAD-after: $after   (delta $((after - before)))"
    if [ -f "$repo/docs/architecture/gateway.md" ]; then
      echo "PROMOTED: docs/architecture/gateway.md PRESENT"
      echo "--- git log -1 --format=%s ---"
      git -C "$repo" log -1 --format=%s
    else
      echo "PROMOTED: docs/architecture/gateway.md ABSENT (nothing promoted)"
    fi
  } >> "$log"
  echo "  $tag: exit=$exit  head-delta=$((after - before))  log=$log"
  rm -rf "$repo" "$home"
}

echo "### running the three arms (installed binary, no overrides)"
run_arm block-card  0 1   # BLOCK-CARD  + DISAMBIG (A blocks, B silent)
run_arm block-title 1 0   # BLOCK-TITLE (B blocks, A silent)
run_arm pass        1 1   # PASS        (one commit, promoted)
echo "### done"
