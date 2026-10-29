# Tools used to write the plan in jigc's shapes

Two scripts written while the plan before the opening was shaped into jigc's doctypes and checked with jigc itself (2026-10-29). They are kept here as text, under the human's rule that nothing stays in a scratch folder. Neither is a `dev/` tool: the shape comparison is a candidate for one ([tooling.md](../../../../../implementation/tooling.md) → Candidates), because the port needs the same check document by document.

## 1. The progress-record builder

**The question it answers:** how close can a record this repository writes by hand stay to the `milestone-record` that jigc writes, and where exactly does it have to differ?

It authors the record through jigc, then changes only what jigc's door cannot say, so every difference is visible and each one is a misfit row ([doctype-misfits.md](../../doctype-misfits.md) → M4, M5). Input: `tasks.txt`, one line per task, `<who runs it>|<code> — <intent in plain words>`. Run from the jigc checkout:

```sh
src=$PWD
rig=$("$src"/dev/jigc-rig fresh) || exit; eval "$rig"
$JIGC milestone create "Before the first stabilization run opens"
m=before-the-first-stabilization-run
while IFS='|' read -r who intent; do
  $JIGC milestone add-task "$m" "$intent" >/dev/null || echo "refused: $intent"
done < "$src/tasks.txt"
python3 - "docs/milestone-records/$m.md" "$src/tasks.txt" "$src/progress.md" <<'EOF'
import re, sys
src, tasks, dst = sys.argv[1:]
text = open(src).read()
who = [l.split("|", 1)[0] for l in open(tasks).read().splitlines()]
# the base is the plan's, not the rig's
text = re.sub(r"^base: \S+ \S+$", "base: 7c34a222c926d49de365fc16cf04c157017b7f37 7c34a22", text, flags=re.M)
head, *items = text.split("\n### ")
assert len(items) == len(who)
out = [head]
for item, w in zip(items, who):
    # the door can name only a pack workflow (M5) and writes `active` for a task nobody started (M4)
    out.append(item.replace("- workflow: sub-task\n- status: active", "- workflow: " + w))
open(dst, "w").write("\n### ".join(out))
EOF
cp "$src/progress.md" "docs/milestone-records/$m.md" && git commit -qam "the record as written"
$JIGC validate    # expected: only file-state.hash-matches, the out-of-band edit itself
```

## 2. The shape comparison

**The question it answers:** does a document this repository wrote by hand have exactly the shape jigc gives the same content? It authors the content through jigc in a throwaway repository, finalizes it, and compares the committed bytes with the hand-written file.

The version used here compared a roadmap entry and a planning record. Inputs: the entry's `proves.md` and `decomposition.md`, and one file per planning-record gate in `gates/`, with their order in `gates/order`.

```sh
src=$PWD
rig=$("$src"/dev/jigc-rig fresh) || exit; eval "$rig"
t=$($JIGC start --workflow planning "Plan the pre-opening work" | head -1 | sed 's/task minted: //')
title="Before the first stabilization run opens"; id=before-the-first-stabilization-run
$JIGC doc create roadmap --title Roadmap --task "$t" >/dev/null
$JIGC doc add-item roadmap:roadmap#milestones --title "$title" --task "$t"
$JIGC doc set-slot "roadmap:roadmap#milestones/$id/proves" --from-file "$src/proves.md" --task "$t"
$JIGC doc set-slot "roadmap:roadmap#milestones/$id/decomposition" --from-file "$src/decomposition.md" --task "$t"
$JIGC doc create planning-record --title "$title" --task "$t" >/dev/null
while read -r gate; do
  $JIGC doc set-slot "planning-record:$id#$gate" --from-file "$src/gates/$gate.md" --task "$t" >/dev/null || echo "refused: $gate"
done < "$src/gates/order"
$JIGC doc set-field "commit:$t#header/type" --value docs --task "$t"
echo "plan the work before the first stabilization run opens" | $JIGC doc set-slot "commit:$t#summary" --from-file - --task "$t"
$JIGC task finalize "$t" && $JIGC validate
cmp "docs/planning-records/$id.md" "$src/completions/artifacts/M55/stabilization-build/pre-opening-plan/planning-record.md" && echo "planning record: same bytes"
python3 - docs/roadmap.md "$src/implementation/roadmap.md" <<'EOF'
import sys
jigc, mine = (open(p).read() for p in sys.argv[1:])
entry = jigc.split("### Before the first stabilization run opens", 1)[1].replace("\n#### ", "\n### ")
hand = mine.split("## Before the first stabilization run opens", 1)[1].split("\n## M56")[0]
# this repository's roadmap holds entries one heading level higher than jigc's `## Milestones`
print("roadmap entry: same text" if entry.rstrip() == hand.rstrip() else "roadmap entry: DIFFERS")
EOF
```

Run on 2026-10-29 with the installed `1.0.0-rc.24` and with the debug build from `7c34a222`: the planning record came out byte for byte, and the roadmap entry the same text one heading level lower.
