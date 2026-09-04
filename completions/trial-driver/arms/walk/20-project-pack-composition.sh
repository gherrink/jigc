#!/usr/bin/env bash
# 20-project-pack-composition.sh — protocol.md §5 arm 20 (M49 Increment 6, PB-1).
#
# WHICH SET THIS ARM ITERATES: the three-pack composition
# `[listed ▸ dev ▸ methodology]` — the pack-set `design/multi-pack.md` → "A project
# pack composes with the pair, under one demotion (M49)" declares, driven over its
# two cells: an id the freeze does NOT govern (the listed pack wins — extension)
# and an id it DOES (the embedded pair wins — demotion). The governed set is the
# two embedded manifests' `doctypes:` lists (15 ids), read at assembly, never
# hand-listed; this arm drives one member of each side.
#
# THE FLIP, so the arm reads as a change:
#   rc.12  the marker (`compose-embedded-methodology: true`, which `jigc setup`
#          writes into EVERY project) and a `packs:` list were a loud REFUSAL at
#          pack-load — the M42 repair of an even worse silent drop — so declaring
#          one house doctype cost the entire methodology surface.
#   rc.13  the pack-set the operator declared is the pack-set that loads, with the
#          freeze binding by CHECKING the listed pack (a governed doctype it ships
#          is demoted below the embedded pair, and `--explain` names the loss).
#
# DECLARED BOUNDS, quoted verbatim from `design/multi-pack.md` → Declared bounds
# of a project pack:
#   1 — "A project pack is a fork, not a subclass: every reused base step and
#        command-ref must be vendored." … "So reuse of a base workflow is
#        vendoring, and a vendored copy does not track the base pack's upgrades."
#        Driven below: the house pack composes only after it vendors
#        `config/commands.yaml`, even though its one step emits no `{{cli.*}}`.
#   2 — "The demotion protects the marker path only; the M14 listed-pack path is
#        unchanged." … "a manifest-less listed pack still shadows a frozen doctype
#        with nothing checking it." Measured below on the un-markered list, and
#        restored; the bound is recorded, not adjudicated.
#
# EXPECTED RESULT ON 1.0.0-rc.13, stated up front: this arm FAILS on exactly one
# bar — a driven observation, not a probe bug. The missing-catalog fault a listed
# pack raises reads "the embedded pack is missing `commands`" (a fixed literal at
# `crates/cli/src/start.rs` → `read_pack`), naming the pack that HAS the catalog
# rather than the listed pack that lacks it — a law-1 wording defect at the one
# fault bound 1 says an adopter will meet first. Every other bar is expected OK.
#
# PASS CONDITION, stated before the run:
#   (a) with marker + list, `describe` lists `note` and `record-note`, `doc schema
#       note` projects the house schema, `--explain` names the house pack as the
#       workflow's provider with its path + content-hash, and a task can create,
#       author, read back and finalize a `note` into the committed store;
#   (b) a house `adr` with a section dropped is DEMOTED: `doc schema adr` renders
#       the frozen shape (options present), `--explain` names the collision and
#       its winner, `validate` stays clean, and the ungoverned `note` still wins;
#   (c) the methodology pack is still there (`doc schema idea` resolves).
set -uo pipefail
cd /work

say()  { printf '\n=== %s\n' "$1"; }
FAIL=0
step() { printf '\n$ %s\n' "$*"; "$@" 2>&1; printf '[exit %s]\n' "$?"; }
bar()  { if eval "$2" >/dev/null 2>&1; then echo "  OK    $1"; else echo "  FAIL  $1"; FAIL=1; fi; }
newtask() { jigc task list | awk '/^  [a-z0-9]/{print $1; exit}'; }

say "0 · adopt — setup writes the marker into every project"
jigc setup >/dev/null 2>&1
jigc config set invocation-log true >/dev/null 2>&1
jigc --version
cat .jigc/config/packs.yaml
bar "setup wrote the compose marker" "grep -q '^compose-embedded-methodology: true' .jigc/config/packs.yaml"

# ---------------------------------------------------------------------------
say "1 · THE HOUSE PACK — one new doctype, one workflow that creates it, one step"
H="$(mktemp -d /tmp/housepack-XXXXXX)"
mkdir -p "$H/config" "$H/schemas" "$H/workflows" "$H/steps"
printf 'pack-id: house\n' > "$H/config/defaults.yaml"
cat > "$H/schemas/note.yaml" <<'EOF'
type: note
location: notes/
id-from: title
description: A short house-local note.
usage: you want to jot a house-local note that jigc manages.
sections:
  - id: meta
    header: true
    fields:
      - { id: topic, type: string }
  - id: body
    slot: { hint: "The note." }
EOF
cat > "$H/workflows/record-note.yaml" <<'EOF'
---
when: jot down a short house note
description: Records one house note and commits it.
usage: a house-local note is worth keeping under jigc.
creates-task: true
allows-create: [{type: note, as: note}]
---
{{ include: step:author-note }}
EOF
cat > "$H/steps/author-note.yaml" <<'EOF'
---
states-constraints: [read.staged-read-back]
---
Record the note. The intent is:
{{ task.intent }}

Create it, then set its topic and author its body:

jigc doc create note --title "<title>" --task {{task.id}}
jigc doc set-field note:<slug>#meta/topic --value "<topic>" --task {{task.id}}
jigc doc set-slot note:<slug>#body --from-file - --task {{task.id}} <<'NOTE'
<the note>
NOTE

Read the staged note back before finalizing — `jigc doc show note:<slug> --task {{task.id}}` serves the staged copy.
Then author the commit doc and finalize with `jigc task finalize {{task.id}}`.
EOF
echo "  house pack: $H"
find "$H" -type f | sort | sed 's/^/    /'
printf 'compose-embedded-methodology: true\npacks:\n  - %s\n' "$H" > .jigc/config/packs.yaml
cat .jigc/config/packs.yaml

say "1 · BOUND 1, driven — the pack composes only after it vendors config/commands.yaml"
step jigc start --workflow record-note "note the cache policy"
M="$(jigc start --workflow record-note "note the cache policy" 2>&1)"; MRC=$?
bar "without a vendored catalog the door refuses (exit non-zero)"      "test $MRC -ne 0"
bar "…naming the missing resource"                                      "printf '%s' \"\$M\" | grep -q 'missing `commands`'"
bar "no task was minted by the refused door"                            "test -z \"\$(jigc task list 2>/dev/null | awk '/^  [a-z0-9]/{print \$1}')\""
# THE DRIVEN OBSERVATION (expected FAIL on rc.13 — see the header). The pack that
# lacks `config/commands.yaml` is the LISTED house pack; the message names the
# embedded one. Captured verbatim above in the step output.
bar "the fault names the pack that lacks the catalog (house), not 'the embedded pack' — law 1" \
    "printf '%s' \"\$M\" | grep -q 'house' && ! printf '%s' \"\$M\" | grep -q 'the embedded pack is missing'"
printf 'commands: []\n' > "$H/config/commands.yaml"
# `jigc workflow <id> --preview` is compose-minus-mint: it proves the pack now
# composes without spending a task on it (the task cell A mints its own).
step jigc workflow record-note --preview
bar "with an (empty) vendored catalog the same pack composes (workflow --preview)" "jigc workflow record-note --preview >/dev/null 2>&1"

# ---------------------------------------------------------------------------
say "A · EXTENSION — the ungoverned \`note\` resolves from the listed pack"
step jigc describe --doctypes
# No line anchors on the menu: `describe` prints the FIRST entry of each kind on
# the same line as its lead-in ("The doc-types you can author. adr is …"), so an
# anchored grep silently misses whichever entry sorts first.
DD="$(jigc describe --doctypes 2>&1)"; DW="$(jigc describe --workflows 2>&1)"
bar "describe --doctypes lists note"                 "printf '%s' \"\$DD\" | grep -q 'note is A short house-local note'"
bar "describe --workflows lists record-note"         "printf '%s' \"\$DW\" | grep -q 'record-note is Records one house note'"
step jigc doc schema note
bar "doc schema note projects the house schema"      "jigc doc schema note 2>&1 | grep -q 'doctype: note' && jigc doc schema note 2>&1 | grep -q 'topic: string'"
NJ="$(jigc doc schema note --format json 2>/dev/null)"
bar "…and its --format json parses with the house field" \
    "printf '%s' \"\$NJ\" | node -e 'const j=JSON.parse(require(\"fs\").readFileSync(0,\"utf8\")); process.exit(JSON.stringify(j).includes(\"topic\")?0:1)'"

say "A · the provider is named — --explain carries the house pack, its path and its content-hash"
step jigc start --workflow record-note --explain
X="$(jigc start --workflow record-note --explain 2>&1)"
bar "the workflow line names house as the pack that provided record-note" \
    "printf '%s' \"\$X\" | grep -q '^workflow:record-note .*house/'"
bar "a Pack input line names the house pack's resolving directory"  "printf '%s' \"\$X\" | grep -q \"Pack input: house/.* = $H \""
bar "…with a blake3 content-hash"                                   "printf '%s' \"\$X\" | grep 'Pack input: house/' | grep -q 'blake3 [0-9a-f]\{64\}'"
bar "…and both embedded packs are still in the set"                 "printf '%s' \"\$X\" | grep -q 'Pack input: dev/' && printf '%s' \"\$X\" | grep -q 'Pack input: methodology/'"
# Recorded, not asserted: the house pack's version sentinel renders as
# `house/fs-local` on its Pack-input line and `house/vfs-local` on the workflow
# line (the workflow line prefixes every version with `v`, as `dev/v1.0.0-rc.13`).
echo "  MEASURED · house labels on one --explain surface: $(printf '%s' "$X" | grep -o 'house/[a-z-]*' | sort -u | tr '\n' ' ')"
# `describe` prints no origin pack per entry — its json carries {kind,id,prose,
# router_hidden} only; the origin surface is `--explain`. Recorded so the next
# reader does not look for it on the menu.
echo "  MEASURED · describe --format json keys per definition: $(jigc describe --doctypes --format json 2>/dev/null | node -e 'const j=JSON.parse(require("fs").readFileSync(0,"utf8")); console.log(Object.keys(j.definitions[0]).join(","))')"

say "A · a task creates, authors, reads back and finalizes a note into the committed store"
step jigc start --workflow record-note "note the cache policy"
T="$(newtask)"
echo "  task: $T"
bar "a task was minted under the house workflow" "test -n \"\$T\" && test -d .jigc/tasks/$T"
step jigc doc create note --title "Cache policy" --task "$T"
step jigc doc set-field note:cache-policy#meta/topic --value caching --task "$T"
step bash -c "printf 'Evict cold entries first.\n' | jigc doc set-slot note:cache-policy#body --from-file - --task $T"
step jigc doc show note:cache-policy --task "$T" --format json
R="$(jigc doc show note:cache-policy --task "$T" --format json 2>/dev/null)"
bar "the staged read-back carries the field, the slot and the staging task" \
    "printf '%s' \"\$R\" | node -e 'const j=JSON.parse(require(\"fs\").readFileSync(0,\"utf8\")); process.exit(j.type===\"note\"&&j.fields.topic===\"caching\"&&j.sections.body===\"Evict cold entries first.\"&&j.staged===\"$T\"?0:1)'"
jigc doc set-field "commit:$T#header/type" --value docs --task "$T" >/dev/null 2>&1
printf 'record the cache policy note\n' | jigc doc set-slot "commit:$T#summary" --from-file - --task "$T" >/dev/null 2>&1
step jigc task finalize "$T"
bar "finalize landed"                                   "git log -1 --format=%s | grep -q 'docs: record the cache policy note'"
bar "…promoting the note to its house location"         "test -f docs/notes/cache-policy.md"
bar "doc list note reports it managed"                  "jigc doc list note 2>&1 | grep -q 'note:cache-policy .*docs/notes/cache-policy.md .*managed'"
bar "the committed read serves it"                      "jigc doc show note:cache-policy --format json 2>/dev/null | grep -q '\"topic\": \"caching\"'"
bar "validate is clean over the composed set"           "jigc validate 2>&1 | grep -q 'validates clean'"

say "C · the methodology pack is not lost — the point of the combination"
bar "doc schema idea resolves"              "jigc doc schema idea 2>&1 | grep -q 'doctype: idea'"
bar "doc schema milestone-record resolves"  "jigc doc schema milestone-record 2>&1 | grep -q 'doctype: milestone-record'"

# ---------------------------------------------------------------------------
say "B · DEMOTION — the same listed pack ships an \`adr\` with \`options\` dropped"
cat > "$H/schemas/adr.yaml" <<'EOF'
type: adr
location: decisions/
id-from: title
description: A house ADR with no options section.
usage: the house drops options.
sections:
  - id: status
    header: true
    fields:
      - { id: status, type: enum, of: [proposed, accepted, superseded], default: proposed }
      - { id: date, type: date, set: on-create }
  - id: context
    slot: { hint: "Why." }
  - id: decision
    slot: { hint: "What." }
  - id: consequences
    slot: { hint: "So what." }
EOF
step jigc doc schema adr
S="$(jigc doc schema adr 2>&1)"; SRC=$?
bar "doc schema adr still loads (exit 0)"                              "test $SRC -eq 0"
bar "…at the frozen shape: options present"                            "printf '%s' \"\$S\" | grep -q 'options: slot'"
bar "…and the frozen version"                                          "printf '%s' \"\$S\" | grep -q 'doctype: adr (schema-version 2)'"
bar "…with none of the house copy's loss (supersedes still there)"     "printf '%s' \"\$S\" | grep -q 'supersedes: ref'"
DD2="$(jigc describe --doctypes 2>&1)"
bar "describe still describes adr in the embedded pack's words"        "printf '%s' \"\$DD2\" | grep -q 'adr is A dated architectural decision record'"
bar "…and not in the house copy's"                                     "! printf '%s' \"\$DD2\" | grep -q 'A house ADR with no options section'"
step jigc start --explain
X2="$(jigc start --explain 2>&1)"
bar "--explain names the adjudicated collision and its winner"         "printf '%s' \"\$X2\" | grep -q 'collision: doctype:adr → won by dev/'"
bar "…and still names the house pack as a composed input"              "printf '%s' \"\$X2\" | grep -q \"Pack input: house/.* = $H \""
bar "validate stays clean under the demotion"                          "jigc validate 2>&1 | grep -q 'validates clean'"
bar "the ungoverned note still resolves from the listed pack (per-id demotion, not per-pack)" \
    "jigc doc schema note 2>&1 | grep -q 'topic: string'"

say "B · BOUND 2, measured — the un-markered M14 path has no demotion"
printf 'packs:\n  - %s\n' "$H" > .jigc/config/packs.yaml
U="$(jigc doc schema adr 2>&1)"; URC=$?
printf '%s\n[exit %s]\n' "$U" "$URC"
if [ "$URC" -eq 0 ] && ! printf '%s' "$U" | grep -q 'options: slot'; then
  echo "  MEASURED · un-markered list: the house fork renders at exit 0 with options ABSENT — bound 2 as declared"
else
  echo "  MEASURED · un-markered list: exit $URC, options $(printf '%s' "$U" | grep -q 'options: slot' && echo present || echo absent) — NOT the declared bound; read the output above"
fi
printf 'compose-embedded-methodology: true\npacks:\n  - %s\n' "$H" > .jigc/config/packs.yaml
bar "marker + list restored: the frozen adr is back"  "jigc doc schema adr 2>&1 | grep -q 'options: slot'"

say "SUMMARY"
echo "  cells: extension (note wins) · demotion (adr loses, named) · methodology retained · bound 1 driven · bound 2 measured"
echo "  the one expected FAIL is the missing-catalog fault's wording (see the header) — a driven observation"
if [ "$FAIL" -eq 0 ]; then echo "ARM 20 PASS"; else echo "ARM 20 FAIL"; fi
exit "$FAIL"
