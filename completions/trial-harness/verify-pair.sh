#!/usr/bin/env bash
# verify-pair.sh [old-tag] [new-tag]
#
# Prove the two images are DIFFERENT TREES, behaviourally — and that both binaries
# actually run.
#
# Why this exists: TWO commits stamp `1.0.0-rc.10` — 8979f16 (the genuine pre-M48
# binary) and 4fd7fbc (rc.10-stamped, but containing all of M48 because the bump landed
# late at 9cb9b78). `jigc --version` cannot tell them apart, so an upgrade arm built
# from the wrong one compares a binary against itself and nothing in the output reveals it.
#
# ── The probe set is chosen, not assumed (2026-08-28) ────────────────────────────
# The original probes were three verbs M48 SHIPPED, asserted absent-then-present. That
# set is **vacuous on an rc.11/rc.12 pair**: rc.11 has all three, because M46 shipped no
# new verb at all. A wave that changes only behaviour needs behavioural probes, so the
# set is now selected with $PAIR_PROBES and each probe declares what it expects on BOTH
# sides rather than hard-coding absent/PRESENT.
#
#   PAIR_PROBES=m48   the 1.0.0-gate set: rc.10 -> rc.11
#   PAIR_PROBES=m46   the 1.0.0 set:      rc.11 -> rc.12
#   PAIR_PROBES=m49   the pre-v1 set:     rc.12 -> rc.13   (default)
#
# The m49 set (2026-09-04): M49 shipped one new verb flag (`doc add-item --slug`) and
# many behaviour changes; the three probes below are picked from the changes an adopter
# meets FIRST and that a driver keys on — a mint door that used to accept an unknown
# workflow, a pinned read contract that gained a top-level key, and a catalog projection
# that gained a `pack` key. Each is a difference in bytes a driver already parses.
#
# The default tracks the CURRENT trial, and so do the default tags, because a default
# that is wrong for the trial in front of you is a trap wearing a convenience. Running
# an old pair is one env var; running the current one must not be.
#
# Absence is ALSO what a dead binary produces — a `jigc` that cannot resolve its libc
# answers "absent" to every probe and would score a clean sweep. So liveness and the
# version stamps are asserted FIRST, the sha is checked directly, and an old==new pair
# is refused outright.
set -uo pipefail

OLD="${1:-jigc-gate:rc12}"
NEW="${2:-jigc-gate:rc13}"
PAIR_PROBES="${PAIR_PROBES:-m49}"

# Trial-specific, and therefore overridable — the harness outlives any one trial.
case "$PAIR_PROBES" in
  m48) DEF_SHA=8979f163d628c72aa2b05821b0059606e2f8267a; DEF_VER="jigc 1.0.0-rc.10" ;;
  m46) DEF_SHA=9a37f0152744f0cba5f9140483e1ca1b1c453c46; DEF_VER="jigc 1.0.0-rc.11" ;;
  m49) DEF_SHA=314f59ecc1c32c0ccf16685b83f2797fd2e13fc2; DEF_VER="jigc 1.0.0-rc.12" ;;
  *)   echo "refusing: unknown PAIR_PROBES='$PAIR_PROBES' (want m48, m46 or m49)" >&2; exit 2 ;;
esac
EXPECT_OLD_SHA="${EXPECT_OLD_SHA:-$DEF_SHA}"
EXPECT_OLD_VERSION="${EXPECT_OLD_VERSION:-$DEF_VER}"

FAIL=0
ok()  { echo "  OK    $1"; }
bad() { echo "  FAIL  $1"; FAIL=$((FAIL+1)); }

stamp_of() { docker run --rm --entrypoint /usr/local/bin/jigc "$1" --version 2>&1; }
sha_of()   { docker inspect "$1" --format '{{range .Config.Env}}{{println .}}{{end}}' 2>/dev/null \
               | sed -n 's/^JIGC_SHA=//p' | head -1; }
id_of()    { docker image inspect "$1" --format '{{.Id}}' 2>/dev/null; }

echo "probe set: $PAIR_PROBES   old=$OLD   new=$NEW"
echo
echo "liveness and identity — asserted, not printed:"

# The cheapest way to make this arm vacuous is to point both names at one image.
# Nothing downstream would reveal it: every probe would agree with itself.
OLD_ID="$(id_of "$OLD")"; NEW_ID="$(id_of "$NEW")"
if [ -n "$OLD_ID" ] && [ "$OLD_ID" = "$NEW_ID" ]; then
  bad "old and new are the SAME image ($OLD_ID) — this pair cannot show a difference"
else
  ok "old and new are distinct images"
fi

OLD_STAMP="$(stamp_of "$OLD")"
NEW_STAMP="$(stamp_of "$NEW")"
[ "$OLD_STAMP" = "$EXPECT_OLD_VERSION" ] && ok "old runs and reports '$OLD_STAMP'" \
  || bad "old binary did not report '$EXPECT_OLD_VERSION' — got: $OLD_STAMP"
# Any `jigc <digit>…` stamp is a live binary. The earlier `"jigc 1.0.0-rc."*` pattern sent
# a released `jigc 1.0.0` to the failure arm — it would have fired on the very next
# release (decisions-pending → the harness-surface wave, time-boxed before 1.0.0).
case "$NEW_STAMP" in
  "jigc "[0-9]*) ok "new runs and reports '$NEW_STAMP'" ;;
  *) bad "new binary did not run or reports something unexpected: $NEW_STAMP" ;;
esac

# The decisive check, and the cheapest: the image records the sha it was built from.
OLD_SHA="$(sha_of "$OLD")"
[ "$OLD_SHA" = "$EXPECT_OLD_SHA" ] && ok "old built from $EXPECT_OLD_SHA" \
  || bad "old built from '${OLD_SHA:-<none>}', expected $EXPECT_OLD_SHA — rebuild with ./build-image.sh ${EXPECT_OLD_SHA:0:7}"
[ -n "$(sha_of "$NEW")" ] && ok "new records its sha ($(sha_of "$NEW"))" \
  || bad "new carries no JIGC_SHA — not built by build-image.sh"

# ── probes ───────────────────────────────────────────────────────────────────────
# Each probe declares the token it expects from EACH side. A probe whose two sides
# agree is reported MISMATCH: it discriminates nothing, and a pair that discriminates
# nothing is the failure this script exists to prevent.
probe() { # <label> <expect-old> <expect-new> <shell snippet echoing one token>
  local label="$1" want_a="$2" want_b="$3" test="$4" a b
  a="$(docker run --rm --entrypoint bash "$OLD" -c "$test" 2>/dev/null | tail -1)"
  b="$(docker run --rm --entrypoint bash "$NEW" -c "$test" 2>/dev/null | tail -1)"
  printf '  %-26s old=%-10s new=%-10s ' "$label" "${a:-<none>}" "${b:-<none>}"
  if [ "$a" = "$want_a" ] && [ "$b" = "$want_b" ]; then echo "OK"
  else echo "MISMATCH (wanted old=$want_a new=$want_b)"; FAIL=$((FAIL+1)); fi
}

# A brownfield repo: real code, a foreign Keep-a-Changelog at the `changelog` home,
# `jigc setup` and nothing else. This is the state an adopter is in on day one, and
# both M46 changes below are things they meet in their first minutes.
read -r -d '' BROWNFIELD <<'SNIP' || true
export HOME=/tmp/h; mkdir -p $HOME; d=$(mktemp -d); cd "$d"
git init -q -b main . 2>/dev/null
git config user.email a@b.c; git config user.name A
printf '# Changelog\n\n## [0.1.0]\n- first\n' > CHANGELOG.md
printf 'export const x = 1;\n' > x.ts
git add -A >/dev/null; git commit -qm init
jigc setup >/dev/null 2>&1
git add -A >/dev/null 2>&1; git commit -qm setup >/dev/null 2>&1
SNIP

read -r -d '' TASKREPO <<'SNIP' || true
export HOME=/tmp/h; mkdir -p $HOME; d=$(mktemp -d); cd "$d"
git init -q -b main . 2>/dev/null
git config user.email a@b.c; git config user.name A
printf 'export const x = 1;\n' > x.ts
git add -A >/dev/null; git commit -qm init
jigc setup >/dev/null 2>&1
git add -A >/dev/null 2>&1; git commit -qm setup >/dev/null 2>&1
jigc start --workflow single-task "add a rate limiter" >/dev/null 2>&1
# `[a-z0-9]`, not `[a-z]`: jigc mints digit-leading slugs, and the narrower class dropped
# them and fed "" into the probe below — the reader that found T1-a (`task validate ""`
# false-greens on a release binary). Time-boxed before 1.0.0; taken here.
ID=$(jigc task list 2>/dev/null | awk '/^  [a-z0-9]/{print $1; exit}')
SNIP

echo
case "$PAIR_PROBES" in
  m48)
    echo "M48 verbs — absent in a genuine pre-M48 tree, present after:"
    probe "doc rename"           absent PRESENT 'jigc doc --help 2>&1 | grep -qw rename && echo PRESENT || echo absent'
    probe "config get"           absent PRESENT 'jigc config --help 2>&1 | grep -qw get && echo PRESENT || echo absent'
    probe "describe --workflows" absent PRESENT 'jigc describe --help 2>&1 | grep -q -- "--workflows" && echo PRESENT || echo absent'
    ;;
  m46)
    echo "M46 behaviour — no new verb shipped, so these are the differences that exist:"
    # Increment 3, and the trial protocol's declared change 0.1.
    probe "validate over a squatter" exit0 nonzero \
      "$BROWNFIELD"'
       jigc validate >/dev/null 2>&1 && echo exit0 || echo nonzero'
    # Increment 3's paired half: the file leaves `blocked` for its own declared key.
    probe "migrate-corpus unadopted" absent PRESENT \
      "$BROWNFIELD"'
       jigc migrate-corpus --dry-run --format json > /tmp/mc.json 2>&1
       node -e "const o=require(\"/tmp/mc.json\");process.stdout.write(o.unadopted?\"PRESENT\":\"absent\")" ; echo'
    # Increment 6, and the trial protocol's declared change 0.2.
    probe "changelog gate at validate" absent PRESENT \
      "$TASKREPO"'
       jigc task validate "$ID" 2>&1 | grep -q "gate-granted-unused" && echo PRESENT || echo absent'
    ;;
  m49)
    echo "M49 behaviour — the differences a driver meets first:"
    # Increment 2: an unknown --workflow used to mint at exit 0; it now blocks BEFORE the mint.
    probe "add-task unknown workflow" exit0 nonzero \
      "$TASKREPO"'
       jigc milestone create "bound the store" >/dev/null 2>&1
       jigc milestone add-task bound-the-store "cap distinct series" --workflow no-such-workflow >/dev/null 2>&1 \
         && echo exit0 || echo nonzero'
    # Increment 8 (N2): the pinned whole-doc read gains a top-level `schema-version` key.
    probe "doc show schema-version key" absent PRESENT \
      "$TASKREPO"'
       jigc doc show "commit:$ID" --task "$ID" --format json > /tmp/ds.json 2>&1
       node -e "const o=require(\"/tmp/ds.json\");process.stdout.write(Object.prototype.hasOwnProperty.call(o,\"schema-version\")?\"PRESENT\":\"absent\")" ; echo'
    # Increment 11 (T6): the catalog projection is a per-(id, pack) union carrying `pack`.
    probe "describe --commands pack key" absent PRESENT \
      "$TASKREPO"'
       jigc describe --commands --format json > /tmp/dc.json 2>&1
       node -e "const o=require(\"/tmp/dc.json\");process.stdout.write((o.commands||[]).some(c=>\"pack\" in c)?\"PRESENT\":\"absent\")" ; echo'
    ;;
esac

echo
if [ "$FAIL" -eq 0 ]; then
  echo "both binaries run, the pair is two distinct trees — the upgrade arm is not vacuous"
else
  echo "$FAIL problem(s) — DO NOT run the upgrade arm on these images"
  exit 1
fi
