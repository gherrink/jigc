#!/usr/bin/env bash
# M29 inc-2 T4 — the F4/F5 recorded-alongside measure: run the THREE measured facts of
# worked-examples flow 31 on the ACTUALLY-PINNED, `cargo install`-built binary (NOT
# cargo test). The installed `jigc` is taken from PATH with the EMBEDDED dev pack
# (NO JIGC_PACK_DIR) and the `doc-code` probe resolved as the sibling beside the
# installed binary (NO JIGC_DOC_CODE_PROBE) — the production probe-resolution path a
# real install hits, exercising the M29 EIGHT-grammar set (M28's seven + YAML).
#
# Mirrors flow 31 (crates/cli/tests/flow31_acceptance.rs) on a YAML/docker-compose
# repo. Both components anchor the SAME compose file at different service keys — the
# sharpest per-item disambiguation fixture, since the only lever that picks A from B is
# the per-key YAML extractor (block_mapping_pair -> key) over the same compose.yaml:
#   component A -> a real compose service key (compose.yaml#web, a mapping key)
#   component B -> a real compose service key (compose.yaml#db,  a mapping key)
# Component B's item slug is `database` while its anchor symbol is `db`, so the block is
# proven keyed on the ITEM ADDRESS, not on a name coincidence with the symbol.
# Three arms over the SAME authored arch-doc fixture (the per-anchor pass<->block flip
# is the masking guard — a resolving doc-code check emits no finding):
#   BLOCK-WEB  drop web: service, db: stays  -> block names A's addr, never B's (== DISAMBIG)
#   BLOCK-DB   drop db: service,  web: stays  -> block names B's addr, never A's
#   PASS       both service keys present      -> one docs(arch-doc): commit, promoted
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

# --- the shared compose file: web: (component A) + db: (component B) ----------------
WEB_SERVICE=$'  web:\n    image: nginx\n    ports:\n      - "8080:80"\n'
DB_SERVICE=$'  db:\n    image: postgres\n    environment:\n      POSTGRES_PASSWORD: secret\n'

# Compose compose.yaml from whichever service blocks are present. Dropping a block
# removes its service key (the file stays, the key vanishes) — a key-absent block, never
# a file-absent one. The arms below never drop both, so `services:` is never empty.
compose() { # $1=web(1/0) $2=db(1/0)
  local out="services:"$'\n'
  [ "$1" = 1 ] && out+="$WEB_SERVICE"
  [ "$2" = 1 ] && out+="$DB_SERVICE"
  printf '%s' "$out"
}

# Stand up a fresh YAML-bearing scratch repo with the flow-31 fixture, set up + the cited
# adr committed. Echoes the repo path. $1=HOME, $2=web present, $3=db present.
make_repo() {
  local home="$1" web="$2" db="$3"
  local repo
  repo=$(mktemp -d /tmp/jigc-m29-flow31-XXXXXX)
  git -C "$repo" init -q
  git -C "$repo" config user.email t@e.com
  git -C "$repo" config user.name T
  compose "$web" "$db" > "$repo/compose.yaml"
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

# Author the two-component arch-doc (A->compose.yaml#web, B->compose.yaml#db) in $1=repo,
# $2=HOME. Component B's title "Database" mints slug `database`, distinct from symbol `db`.
author_arch_doc() {
  local repo="$1" home="$2"
  ( cd "$repo"
    HOME="$home" jigc start --workflow architecture-documentation "document the gateway" >/dev/null 2>&1
    HOME="$home" jigc doc create arch-doc --title "Gateway" >/dev/null 2>&1
    HOME="$home" jigc doc set-slot  arch-doc:gateway#overview --from-file - <<<"The gateway runs a web tier backed by a database service." >/dev/null 2>&1
    HOME="$home" jigc doc set-field arch-doc:gateway#cites --value adr:use-a-cache >/dev/null 2>&1
    HOME="$home" jigc doc add-item  arch-doc:gateway#components --title "Web" >/dev/null 2>&1
    HOME="$home" jigc doc set-slot  arch-doc:gateway#components/web/description --from-file - <<<"The web tier container." >/dev/null 2>&1
    HOME="$home" jigc doc set-field arch-doc:gateway#components/web/implemented-by --value "compose.yaml#web" >/dev/null 2>&1
    HOME="$home" jigc doc add-item  arch-doc:gateway#components --title "Database" >/dev/null 2>&1
    HOME="$home" jigc doc set-slot  arch-doc:gateway#components/database/description --from-file - <<<"The database service." >/dev/null 2>&1
    HOME="$home" jigc doc set-field arch-doc:gateway#components/database/implemented-by --value "compose.yaml#db" >/dev/null 2>&1
    HOME="$home" jigc doc set-field commit:document-the-gateway#type  --value docs     >/dev/null 2>&1
    HOME="$home" jigc doc set-field commit:document-the-gateway#scope --value arch-doc >/dev/null 2>&1
    HOME="$home" jigc doc set-slot  commit:document-the-gateway#summary --from-file - <<<"document the gateway" >/dev/null 2>&1
    HOME="$home" jigc doc set-slot  commit:document-the-gateway#body    --from-file - <<<"Living architecture doc for the gateway." >/dev/null 2>&1 )
}

run_arm() {
  local tag="$1" web="$2" db="$3"
  local log="$EVID/$tag.log"
  local home repo
  home=$(mktemp -d /tmp/jigc-m29-home-XXXXXX)
  repo=$(make_repo "$home" "$web" "$db")
  author_arch_doc "$repo" "$home"
  local before
  before=$(git -C "$repo" rev-list --count HEAD)
  {
    echo "=================================================================="
    echo "### ARM: $tag"
    echo "repo: $repo   HEAD-before: $before"
    echo "compose.yaml service keys present:$([ "$web" = 1 ] && echo ' web')$([ "$db" = 1 ] && echo ' db')"
    echo "--- compose.yaml ---"
    cat "$repo/compose.yaml"
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
run_arm block-web 0 1   # BLOCK-WEB + DISAMBIG (A blocks, B silent)
run_arm block-db  1 0   # BLOCK-DB  (B blocks, A silent)
run_arm pass      1 1   # PASS      (one commit, promoted)
echo "### done"
