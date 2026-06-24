#!/usr/bin/env bash
# drive.sh — M33 freeze-gate measured facts, observed on the ACTUALLY-PINNED `jigc`
# binary (NOT `cargo test`). Three arms, each logged into ./ alongside this script:
#
#   A · conformant  — the production path: PATH `jigc`, EMBEDDED dev pack, NO
#                      JIGC_PACK_DIR / JIGC_DOC_CODE_PROBE. The freeze gate runs over
#                      the embedded six-doctype manifest and PASSES → compose, exit 0.
#   B · blocked     — a JIGC_PACK_DIR on-disk dev-pack copy whose `adr` schema SHAPE
#                      drifted (location: decisions/ → adr-records/) with NO
#                      schema-version bump: the pack-load freeze gate FAILS LOUDLY,
#                      `jigc start` exits non-zero, stderr names the schema-hash
#                      mismatch on `adr`. (A shape mutation can only be expressed on
#                      an on-disk copy — the embedded pack is conformant by build.)
#   C · manifest-less — the omitting context: the SAME drift with the manifest
#                      dropped is UNCHECKED → exit 0 (the gate is the manifest, and a
#                      seeded/composed pack with no manifest stays inert, never errors).
#
# The runtime sibling of crates/cli/tests/freeze_enforcement.rs, run on the pinned
# binary with the production probe resolution. Design: corpus-migration.md → The
# freeze, declared and enforced; worked-examples.md → flow 34.
set -u

HERE="$(cd "$(dirname "$0")" && pwd)"
# The embedded dev pack's on-disk source (the faithful tree the embed mirrors).
PACK_SRC="$(cd "$HERE/../../../../crates/cli/pack" && pwd)"

# Resolve the PINNED binary from PATH; print its identity for the record.
JIGC="$(command -v jigc)"
echo "jigc on PATH: $JIGC"
sha256sum "$JIGC"

scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT

# A throwaway git repo with one commit (composition reads HEAD) + the project layer.
mk_repo() {
  local r="$1"
  mkdir -p "$r"
  git -C "$r" init -q
  git -C "$r" config user.email t@e.com
  git -C "$r" config user.name T
  echo hi > "$r/README.md"
  git -C "$r" add .
  git -C "$r" commit -qm init
  mkdir -p "$r/.jigc/config"
}

# Drift the `adr` schema SHAPE without bumping its schema-version — the un-migrated
# change the freeze forbids.
drift_adr() {
  local pack="$1"
  sed -i 's#location: decisions/#location: adr-records/#' "$pack/schemas/adr.yaml"
}

# ---- Arm A · conformant (embedded pack, no JIGC_PACK_DIR) -------------------
repoA="$scratch/repoA"; homeA="$scratch/homeA"; mkdir -p "$homeA"; mk_repo "$repoA"
( cd "$repoA" && env -u JIGC_PACK_DIR -u JIGC_DOC_CODE_PROBE HOME="$homeA" \
  "$JIGC" start --workflow single-task "freeze-gate probe" ) \
  > "$HERE/conformant.log" 2>&1
echo "ARM A (conformant, embedded pack) exit: $?" | tee -a "$HERE/conformant.log"

# ---- Arm B · blocked (drifted on-disk copy via JIGC_PACK_DIR) --------------
repoB="$scratch/repoB"; homeB="$scratch/homeB"; mkdir -p "$homeB"; mk_repo "$repoB"
packB="$scratch/packB"; cp -r "$PACK_SRC" "$packB"; drift_adr "$packB"
( cd "$repoB" && env -u JIGC_DOC_CODE_PROBE HOME="$homeB" JIGC_PACK_DIR="$packB" \
  "$JIGC" start --workflow single-task "freeze-gate probe" ) \
  > "$HERE/blocked.log" 2>&1
echo "ARM B (drifted copy, JIGC_PACK_DIR) exit: $?" | tee -a "$HERE/blocked.log"

# ---- Arm C · manifest-less (same drift, manifest dropped) ------------------
repoC="$scratch/repoC"; homeC="$scratch/homeC"; mkdir -p "$homeC"; mk_repo "$repoC"
packC="$scratch/packC"; cp -r "$PACK_SRC" "$packC"; drift_adr "$packC"
rm -f "$packC/config/schema-manifest.yaml"
( cd "$repoC" && env -u JIGC_DOC_CODE_PROBE HOME="$homeC" JIGC_PACK_DIR="$packC" \
  "$JIGC" start --workflow single-task "freeze-gate probe" ) \
  > "$HERE/manifest-less.log" 2>&1
echo "ARM C (manifest-less, same drift) exit: $?" | tee -a "$HERE/manifest-less.log"
