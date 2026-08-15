#!/usr/bin/env bash
# verify-pair.sh [rc10-tag] [rc11-tag]
#
# Prove the two images are DIFFERENT TREES, behaviourally.
#
# Why this exists rather than a version check: TWO commits stamp `1.0.0-rc.10` —
# 8979f16 (the genuine pre-M48 binary) and 4fd7fbc (rc.10-stamped, but containing all
# of M48 because the bump landed late at 9cb9b78). `jigc --version` cannot tell them
# apart, so an upgrade arm built from the wrong one compares rc.11 against itself and
# nothing in the output reveals it.
#
# Each probe below is a verb M48 SHIPPED, so a genuine pre-M48 tree must lack all three.
# If any row reads PRESENT/PRESENT the rc.10 image is the wrong tree — rebuild it with
# `./build-image.sh 8979f16`.
set -uo pipefail

OLD="${1:-jigc-gate:rc10}"
NEW="${2:-jigc-gate:rc11}"
FAIL=0

probe() { # <label> <shell test inside the image>
  local label="$1" test="$2" a b
  a="$(docker run --rm --entrypoint sh "$OLD" -c "$test" 2>/dev/null)"
  b="$(docker run --rm --entrypoint sh "$NEW" -c "$test" 2>/dev/null)"
  printf '  %-26s old=%-8s new=%-8s ' "$label" "$a" "$b"
  if [ "$a" = "absent" ] && [ "$b" = "PRESENT" ]; then echo "OK"; else echo "MISMATCH"; FAIL=$((FAIL+1)); fi
}

echo "version stamps (expected to be uninformative):"
printf '  old: '; docker run --rm --entrypoint /usr/local/bin/jigc "$OLD" --version
printf '  new: '; docker run --rm --entrypoint /usr/local/bin/jigc "$NEW" --version
echo
echo "M48 verbs — absent in a genuine pre-M48 tree, present after:"
probe "doc rename"          'jigc doc --help 2>&1 | grep -qw rename && echo PRESENT || echo absent'
probe "config get"          'jigc config --help 2>&1 | grep -qw get && echo PRESENT || echo absent'
probe "describe --workflows" 'jigc describe --help 2>&1 | grep -q -- "--workflows" && echo PRESENT || echo absent'

echo
if [ "$FAIL" -eq 0 ]; then
  echo "the two images are different trees — the rc.10 arm is not vacuous"
else
  echo "$FAIL mismatch(es) — DO NOT run the upgrade arm on these images"
  exit 1
fi
