#!/usr/bin/env bash
# check-base-layer.sh <case> [args…]
#
# The harness's own checks of the kept base layer: the toolchain and the fetched
# dependencies, built once for one toolchain file and one lockfile and kept as the image
# `jigc-gate-base:<key>`, so that a second candidate with the same two files compiles on it
# instead of syncing the toolchain and downloading every crate again (build-image.sh, the
# Dockerfile's `toolchain` and `base` stages; README → The kept base layer). One case per
# call, named as the task that built the layer names it. Each prints PASS/FAIL lines and
# exits 1 on any FAIL, 2 on a usage error.
#
#   a_changed_lockfile_or_toolchain_file_builds_a_new_layer
#       No docker. In a throwaway repository: a commit that changes code keeps the key; a
#       commit that changes Cargo.lock, changes rust-toolchain.toml or removes it gets a new
#       key; so does an edit to the Dockerfile above its build stage, and an edit below it,
#       or to a comment, does not.
#   a_second_candidate_with_the_same_lockfile_builds_on_the_kept_layer <first> <second>
#       Two images build-image.sh built, in that order, from two commits with one lockfile
#       and one toolchain file: both record the same base — its key and the layer's own
#       mark — that base is still the image tagged by the key, and the key is the one
#       base_key_of computes for each commit with this harness's Dockerfile.
#   the_binary_in_the_image_is_the_commits <first> <second>
#       Each image records the commit it was built from, the same in its source record and
#       in its JIGC_SHA (what verify-pair.sh reads), and the two binaries differ by sha256
#       where the two commits' sources differ — so neither binary came from the kept base,
#       which holds no tree and no build output.
#   the_image_is_built_from_the_repository_the_harness_sits_in
#       No docker. With JIGC_REPO empty, build-image.sh reads its commits from the
#       repository it sits in, and JIGC_REPO still overrides it.
#
# The repository the commits are read from is JIGC_REPO, else the one this script is in.
set -uo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
CASE="${1:-}"
FAIL=0
ok()  { echo "  PASS  $1"; }
bad() { echo "  FAIL  $1"; FAIL=$((FAIL+1)); }

# The functions build-image.sh defines; sourcing it runs nothing else.
# shellcheck source=build-image.sh
. "$HERE/build-image.sh"
set +e
REPO="$(declare -F default_repo >/dev/null && default_repo || git -C "$HERE" rev-parse --show-toplevel)"

have_key_fn() {
  if declare -F base_key_of >/dev/null; then return 0; fi
  bad "build-image.sh defines no base_key_of — the image has no base layer keyed by the toolchain file and the lockfile"
  return 1
}

# The image's records, read off the image itself.
record_of() { docker run --rm --entrypoint cat "$1" "/usr/local/share/jigc-image/$2" 2>/dev/null; }
field_of()  { sed -n "s/^$2 //p" <<<"$1" | head -1; }

case "$CASE" in
  a_changed_lockfile_or_toolchain_file_builds_a_new_layer)
    echo "== a changed lockfile or toolchain file builds a new layer"
    if have_key_fn; then
      R="$(mktemp -d "${TMPDIR:-/tmp}/base-key.XXXXXX")"
      g() { git -C "$R" -c user.email=t@t.t -c user.name=T -c commit.gpgsign=false "$@"; }
      g init -q
      printf 'version = 4\n[[package]]\nname = "a"\nversion = "1.0.0"\n' > "$R/Cargo.lock"
      printf '[toolchain]\nchannel = "1.95.0"\n' > "$R/rust-toolchain.toml"
      printf 'fn main() {}\n' > "$R/main.rs"
      g add -A; g commit -qm base;                      C0="$(g rev-parse HEAD)"
      printf 'fn main() { println!("x"); }\n' > "$R/main.rs"
      g commit -qam code;                               C1="$(g rev-parse HEAD)"
      printf '[[package]]\nname = "b"\nversion = "2.0.0"\n' >> "$R/Cargo.lock"
      g commit -qam lock;                               C2="$(g rev-parse HEAD)"
      g checkout -q "$C1" -- Cargo.lock
      printf '[toolchain]\nchannel = "1.96.0"\n' > "$R/rust-toolchain.toml"
      g commit -qam toolchain;                          C3="$(g rev-parse HEAD)"
      g rm -q rust-toolchain.toml; g commit -qm no-toolchain; C4="$(g rev-parse HEAD)"

      DF="$HERE/Dockerfile"
      DF_ABOVE="$R/Dockerfile.above"; DF_BELOW="$R/Dockerfile.below"; DF_COMMENT="$R/Dockerfile.comment"
      awk '/^FROM \$\{JIGC_BASE\} AS build$/ && !d { print "RUN true"; d=1 } { print }' "$DF" > "$DF_ABOVE"
      { cat "$DF"; echo 'RUN true'; } > "$DF_BELOW"
      awk 'NR==1 { print "# a comment edit"; print "" } { print }' "$DF" > "$DF_COMMENT"

      K0="$(base_key_of "$R" "$C0" "$DF")"; K1="$(base_key_of "$R" "$C1" "$DF")"
      K2="$(base_key_of "$R" "$C2" "$DF")"; K3="$(base_key_of "$R" "$C3" "$DF")"
      K4="$(base_key_of "$R" "$C4" "$DF")"
      KA="$(base_key_of "$R" "$C0" "$DF_ABOVE")"; KB="$(base_key_of "$R" "$C0" "$DF_BELOW")"
      KC="$(base_key_of "$R" "$C0" "$DF_COMMENT")"
      echo "  keys: base=$K0"
      [[ "$K0" =~ ^[0-9a-f]{64}$ ]] && ok "the key is one sha256" || bad "the key is not a sha256: '$K0'"
      [ "$K1" = "$K0" ] && ok "a code change keeps the layer"            || bad "a code change made a new key ($K1)"
      [ "$K2" != "$K0" ] && ok "a lockfile change builds a new layer"     || bad "a lockfile change kept the key"
      [ "$K3" != "$K0" ] && ok "a toolchain change builds a new layer"    || bad "a toolchain change kept the key"
      [ "$K4" != "$K0" ] && [ "$K4" != "$K3" ] && ok "a removed toolchain file builds a new layer" \
        || bad "a removed toolchain file kept a key ($K4)"
      [ "$KA" != "$K0" ] && ok "an edit above the Dockerfile's build stage builds a new layer" \
        || bad "an edit above the build stage kept the key"
      [ "$KB" = "$K0" ] && ok "an edit below the build stage keeps the layer" || bad "an edit below the build stage made a new key"
      [ "$KC" = "$K0" ] && ok "a comment keeps the layer"                     || bad "a comment made a new key"
      NOMARK="$R/Dockerfile.nomark"; sed '/^FROM \${JIGC_BASE} AS build$/d' "$DF" > "$NOMARK"
      if base_key_of "$R" "$C0" "$NOMARK" >/dev/null 2>&1; then
        bad "a Dockerfile with no build-stage line still gave a key"
      else
        ok "a Dockerfile with no build-stage line gives no key"
      fi
    fi
    ;;

  a_second_candidate_with_the_same_lockfile_builds_on_the_kept_layer)
    FIRST="${2:?usage: $0 $CASE <first-tag> <second-tag>}"; SECOND="${3:?usage: $0 $CASE <first-tag> <second-tag>}"
    echo "== a second candidate with the same lockfile builds on the kept layer ($FIRST, then $SECOND)"
    {
      B1="$(record_of "$FIRST" base)"; B2="$(record_of "$SECOND" base)"
      K1="$(field_of "$B1" key)"; L1="$(field_of "$B1" layer)"
      K2="$(field_of "$B2" key)"; L2="$(field_of "$B2" layer)"
      S1="$(field_of "$(record_of "$FIRST" source)" sha)"; S2="$(field_of "$(record_of "$SECOND" source)" sha)"
      if [ -z "$K1" ] || [ -z "$L1" ] || [ -z "$K2" ] || [ -z "$L2" ]; then
        bad "an image records no base ('$B1' / '$B2') — it was not built on a kept layer"
      else
        [ "$K1" = "$K2" ] && ok "both record the base key $K1" || bad "the keys differ: $K1 / $K2"
        [ "$L1" = "$L2" ] && ok "both record the layer $L1 — the second built on the first's" \
          || bad "the layers differ: $L1 / $L2 — the second rebuilt the base"
        KEPT="$(docker run --rm --entrypoint cat "jigc-gate-base:$K1" /usr/local/share/jigc-base/record 2>/dev/null)"
        [ "$(field_of "$KEPT" layer)" = "$L1" ] && ok "jigc-gate-base:$K1 is still that layer" \
          || bad "jigc-gate-base:$K1 is not that layer ('$KEPT')"
        if [ -n "$S1" ] && [ -n "$S2" ] && [ "$S1" != "$S2" ]; then
          ok "two candidates: $S1, $S2"
          have_key_fn && for S in "$S1" "$S2"; do
            K="$(base_key_of "$REPO" "$S" "$HERE/Dockerfile" 2>/dev/null)"
            [ "$K" = "$K1" ] && ok "base_key_of $S is the recorded key" \
              || bad "base_key_of $S is '$K', not the recorded $K1"
          done
        else
          bad "the two images do not record two commits ('$S1' / '$S2')"
        fi
      fi
    }
    ;;

  the_binary_in_the_image_is_the_commits)
    FIRST="${2:?usage: $0 $CASE <first-tag> <second-tag>}"; SECOND="${3:?usage: $0 $CASE <first-tag> <second-tag>}"
    echo "== the binary in the image is the commit's ($FIRST, $SECOND)"
    declare -a SHAS=() BINS=()
    for T in "$FIRST" "$SECOND"; do
      S="$(field_of "$(record_of "$T" source)" sha)"
      E="$(docker image inspect "$T" --format '{{range .Config.Env}}{{println .}}{{end}}' 2>/dev/null | sed -n 's/^JIGC_SHA=//p' | head -1)"
      if [ -n "$S" ] && [ "$S" = "$E" ] && git -C "$REPO" cat-file -e "${S}^{commit}" 2>/dev/null; then
        ok "$T records the commit $S, in its source record and its JIGC_SHA"
      else
        bad "$T records '$S' as its source and '$E' as its JIGC_SHA — not one commit of $REPO"
      fi
      SHAS+=("$S")
      BINS+=("$(docker run --rm --entrypoint sha256sum "$T" /usr/local/bin/jigc 2>/dev/null | cut -d' ' -f1)")
    done
    if git -C "$REPO" diff --quiet "${SHAS[0]}" "${SHAS[1]}" -- crates/engine/src crates/cli/src 2>/dev/null; then
      bad "the two commits' sources do not differ, so their binaries cannot tell whose each is — pick a pair whose do"
    elif [ -n "${BINS[0]}" ] && [ "${BINS[0]}" != "${BINS[1]}" ]; then
      ok "the sources differ and so do the binaries: ${BINS[0]} / ${BINS[1]}"
    else
      bad "the sources differ but the binaries do not ('${BINS[0]}' / '${BINS[1]}') — one of them is not its commit's"
    fi
    ;;

  the_image_is_built_from_the_repository_the_harness_sits_in)
    # The commits a build archives, and the key it reads, come from one repository. With no
    # JIGC_REPO that is the repository build-image.sh sits in — never a path of one machine.
    echo "== the image is built from the repository the harness sits in"
    if ! declare -F default_repo >/dev/null; then
      bad "build-image.sh defines no default_repo — its repository is not derived from where it sits"
    else
      GOT="$(JIGC_REPO='' default_repo)"; WANT="$(git -C "$HERE" rev-parse --show-toplevel)"
      [ "$GOT" = "$WANT" ] && ok "with no JIGC_REPO, the repository is $WANT" \
        || bad "with no JIGC_REPO, the repository is '$GOT', not $WANT"
      GOT="$(JIGC_REPO=/elsewhere default_repo)"
      [ "$GOT" = /elsewhere ] && ok "JIGC_REPO still names it" || bad "JIGC_REPO=/elsewhere gave '$GOT'"
    fi
    ;;

  *)
    echo "usage: $0 a_changed_lockfile_or_toolchain_file_builds_a_new_layer" >&2
    echo "       $0 a_second_candidate_with_the_same_lockfile_builds_on_the_kept_layer <first-tag> <second-tag>" >&2
    echo "       $0 the_binary_in_the_image_is_the_commits <first-tag> <second-tag>" >&2
    echo "       $0 the_image_is_built_from_the_repository_the_harness_sits_in" >&2
    exit 2 ;;
esac

echo
[ "$FAIL" -eq 0 ] && echo "== passed" || { echo "== $FAIL failed"; exit 1; }
