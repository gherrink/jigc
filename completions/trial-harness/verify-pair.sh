#!/usr/bin/env bash
# verify-pair.sh [rc10-tag] [rc11-tag]
#
# Prove the two images are DIFFERENT TREES, behaviourally — and that both binaries
# actually run.
#
# Why this exists: TWO commits stamp `1.0.0-rc.10` — 8979f16 (the genuine pre-M48
# binary) and 4fd7fbc (rc.10-stamped, but containing all of M48 because the bump landed
# late at 9cb9b78). `jigc --version` cannot tell them apart, so an upgrade arm built
# from the wrong one compares rc.11 against itself and nothing in the output reveals it.
#
# Each verb probe below is a verb M48 SHIPPED, so a genuine pre-M48 tree must lack all
# three. But absence is ALSO what a dead binary produces — a `jigc` that cannot resolve
# its libc answers "absent" to every probe and used to score a clean 3/3. So liveness
# and the version stamps are asserted FIRST, and the sha is checked directly.
set -uo pipefail

OLD="${1:-jigc-gate:rc10}"
NEW="${2:-jigc-gate:rc11}"

# Trial-specific, and therefore overridable — the harness outlives any one trial.
# Defaults are the 1.0.0-gate trial's: 8979f16 is the genuine pre-M48 rc.10.
EXPECT_OLD_SHA="${EXPECT_OLD_SHA:-8979f163d628c72aa2b05821b0059606e2f8267a}"
EXPECT_OLD_VERSION="${EXPECT_OLD_VERSION:-jigc 1.0.0-rc.10}"

FAIL=0
ok()  { echo "  OK    $1"; }
bad() { echo "  FAIL  $1"; FAIL=$((FAIL+1)); }

stamp_of() { docker run --rm --entrypoint /usr/local/bin/jigc "$1" --version 2>&1; }
sha_of()   { docker inspect "$1" --format '{{range .Config.Env}}{{println .}}{{end}}' 2>/dev/null \
               | sed -n 's/^JIGC_SHA=//p' | head -1; }

echo "liveness and identity — asserted, not printed:"
OLD_STAMP="$(stamp_of "$OLD")"
NEW_STAMP="$(stamp_of "$NEW")"
[ "$OLD_STAMP" = "$EXPECT_OLD_VERSION" ] && ok "old runs and reports '$OLD_STAMP'" \
  || bad "old binary did not report '$EXPECT_OLD_VERSION' — got: $OLD_STAMP"
case "$NEW_STAMP" in
  "jigc 1.0.0-rc."*) ok "new runs and reports '$NEW_STAMP'" ;;
  *) bad "new binary did not run or reports something unexpected: $NEW_STAMP" ;;
esac

# The decisive check, and the cheapest: the image records the sha it was built from.
OLD_SHA="$(sha_of "$OLD")"
[ "$OLD_SHA" = "$EXPECT_OLD_SHA" ] && ok "old built from $EXPECT_OLD_SHA" \
  || bad "old built from '${OLD_SHA:-<none>}', expected $EXPECT_OLD_SHA — rebuild with ./build-image.sh 8979f16"
[ -n "$(sha_of "$NEW")" ] && ok "new records its sha ($(sha_of "$NEW"))" \
  || bad "new carries no JIGC_SHA — not built by build-image.sh"

echo
echo "M48 verbs — absent in a genuine pre-M48 tree, present after:"
probe() { # <label> <shell test inside the image>
  local label="$1" test="$2" a b
  a="$(docker run --rm --entrypoint sh "$OLD" -c "$test" 2>/dev/null)"
  b="$(docker run --rm --entrypoint sh "$NEW" -c "$test" 2>/dev/null)"
  printf '  %-22s old=%-8s new=%-8s ' "$label" "$a" "$b"
  if [ "$a" = "absent" ] && [ "$b" = "PRESENT" ]; then echo "OK"; else echo "MISMATCH"; FAIL=$((FAIL+1)); fi
}
probe "doc rename"           'jigc doc --help 2>&1 | grep -qw rename && echo PRESENT || echo absent'
probe "config get"           'jigc config --help 2>&1 | grep -qw get && echo PRESENT || echo absent'
probe "describe --workflows" 'jigc describe --help 2>&1 | grep -q -- "--workflows" && echo PRESENT || echo absent'

echo
if [ "$FAIL" -eq 0 ]; then
  echo "both binaries run, the old one is the pre-M48 tree — the rc.10 arm is not vacuous"
else
  echo "$FAIL problem(s) — DO NOT run the upgrade arm on these images"
  exit 1
fi
