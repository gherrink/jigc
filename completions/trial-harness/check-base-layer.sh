#!/usr/bin/env bash
# check-base-layer.sh <case> [args…]
#
# The harness's own checks of the kept base layer that need docker: the toolchain and the
# fetched dependencies, built once for one toolchain file, one lockfile, one base recipe and
# one platform and kept as the image `jigc-gate-base:<key>`, so that a second candidate with
# the same key compiles on it instead of syncing the toolchain and downloading every crate
# again (build-image.sh, the Dockerfile's `toolchain` and `base` stages; README → The kept
# base layer). One case per call, run by hand over images build-image.sh built. Each prints
# PASS/FAIL lines and exits 1 on any FAIL, 2 on a usage error.
#
# The halves that need no docker — the key (what moves it and what does not) and the
# repository the commits are read from — are a suite of the gate,
# tooling-tests/trial_image_base_key.rs, and are not repeated here.
#
#   a_second_candidate_with_the_same_lockfile_builds_on_the_kept_layer <first> <second>
#       Two images build-image.sh built, in that order, from two commits with one lockfile
#       and one toolchain file: both record the same base — its key and the layer's own
#       mark — that base is still the image tagged by the key, and the key is the one
#       base_key_of computes for each commit with this harness's Dockerfile and the platform
#       the base records, which is also docker's whole report of each image.
#   the_binary_in_the_image_is_the_commits <first> <second>
#       Each image records the commit it was built from, the same in its source record and
#       in its JIGC_SHA (what verify-pair.sh reads), and the two binaries differ by sha256
#       where the two commits' sources differ — so neither binary came from the kept base,
#       which holds no tree and no build output.
#   a_toolchain_the_kept_base_lacks_fails_loudly <image>
#       The kept base the image records, run as the build stage runs on it, with no network:
#       a toolchain file naming a toolchain the base lacks fails with rustup's own "is not
#       installed" and never starts a sync, and the base's own toolchain file still works.
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
          # The platform the base records, and docker's report of each image, variant and
          # all, compared whole — never a part of either.
          P="$(field_of "$B1" platform)"
          for T in "$FIRST" "$SECOND"; do
            IP="$(docker image inspect "$T" --format '{{.Os}}/{{.Architecture}}{{if .Variant}}/{{.Variant}}{{end}}' 2>/dev/null)"
            [ -n "$P" ] && [ "$IP" = "$P" ] && ok "$T is $IP, the platform its base records" \
              || bad "$T is '$IP', and its base records platform '$P'"
          done
          have_key_fn && for S in "$S1" "$S2"; do
            K="$(base_key_of "$REPO" "$S" "$HERE/Dockerfile" "$P" 2>/dev/null)"
            [ "$K" = "$K1" ] && ok "base_key_of $S for $P is the recorded key" \
              || bad "base_key_of $S for $P is '$K', not the recorded $K1"
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

  a_toolchain_the_kept_base_lacks_fails_loudly)
    IMAGE="${2:?usage: $0 $CASE <image-tag>}"
    echo "== a toolchain the kept base lacks fails loudly ($IMAGE)"
    K="$(field_of "$(record_of "$IMAGE" base)" key)"
    if [ -z "$K" ]; then
      bad "$IMAGE records no base — it was not built on a kept layer"
    else
      BASE="jigc-gate-base:$K"
      # The base's own toolchain file, read off the commit the image records.
      S="$(field_of "$(record_of "$IMAGE" source)" sha)"
      OWN="$(git -C "$REPO" show "${S}:rust-toolchain.toml" 2>/dev/null)"
      try() { # try <toolchain file text> — cargo --version under it, in the base, offline
        docker run --rm --network none --entrypoint sh "$BASE" -c \
          'mkdir /t && cd /t && printf "%s\n" "$1" > rust-toolchain.toml && cargo --version' _ "$1" 2>&1
      }
      OUT="$(try "$(printf '[toolchain]\nchannel = "1.1.0"')")"; RC=$?
      if [ "$RC" -ne 0 ] && grep -q "is not installed" <<<"$OUT" && ! grep -q "syncing channel" <<<"$OUT"; then
        ok "a toolchain $BASE lacks is refused, not synced: $(grep 'is not installed' <<<"$OUT" | head -1)"
      else
        bad "a toolchain $BASE lacks was not refused loudly (exit $RC): $(tr '\n' ' ' <<<"$OUT")"
      fi
      if [ -z "$OWN" ]; then
        bad "the commit $S carries no rust-toolchain.toml to run the base's own toolchain under"
      else
        OUT="$(try "$OWN")"; RC=$?
        [ "$RC" -eq 0 ] && ok "the base's own toolchain file runs: $OUT" \
          || bad "the base's own toolchain file fails (exit $RC): $(tr '\n' ' ' <<<"$OUT")"
      fi
    fi
    ;;

  *)
    echo "usage: $0 a_second_candidate_with_the_same_lockfile_builds_on_the_kept_layer <first-tag> <second-tag>" >&2
    echo "       $0 the_binary_in_the_image_is_the_commits <first-tag> <second-tag>" >&2
    echo "       $0 a_toolchain_the_kept_base_lacks_fails_loudly <image-tag>" >&2
    exit 2 ;;
esac

echo
[ "$FAIL" -eq 0 ] && echo "== passed" || { echo "== $FAIL failed"; exit 1; }
