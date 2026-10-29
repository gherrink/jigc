# Scripts the build wrote and ran more than once — kept as text

*Kept 2026-10-29 from the session scratch directory, which a restart removes.* Every script here was written by an agent of the stabilization workflow's build, in the session scratch directory, and run more than once — by its author, or by the next agent, who copied it. **None is a tool of this repository**: a script that is not a registered `dev/` tool never lands as an executable file ([tooling.md](../../../../../implementation/tooling.md) → *Where tools live*), so each is kept here as text, with the question it answers, and has a row in [tooling.md](../../../../../implementation/tooling.md) → *Candidates*, where it is built into `dev/` with its tests or declined with the reason. **Until then nothing here is run as it stands.** The texts are as their authors left them — but the sanitizer, whose own literals were host paths (its section says how it was kept) — with one change: a scratch path is written `<scratch>`, so a script that names one (`R=<scratch>/rr4.bSmurF`) reads its rig tool and binary from a directory its caller must set first — a worktree at `$R/wt` and a debug build at `$R/target/debug/jigc`, as the review that wrote it says.

Why each was kept, and the copies that were not: the [README](README.md) of this directory. The reviews that wrote the link probes say what each probe answered, under their own **Tools** sections: [link-rereview.md](link-rereview.md), [link-rereview2.md](link-rereview2.md), [link-rereview3.md](link-rereview3.md).

## The byte-table record tool — `record-print.py`

**The question.** *Which cells did this change move, and what does the record count now?* It splits a named-binary print of the byte table (the ignored arm `the_cells_are_run_on_the_binary_a_variable_names`, its output saved to a log) into the record's four committed files under `completions/artifacts/M55/stabilization-build/reshape/`, prints every count the record keeps — cells, the gate's and the on-demand share, red here and on the previous release, by class and by command — and, given a base revision's cells file, every cell whose class, key or outcome moved, timing columns left out.

**Raised by.** Written for the link-on-the-way fix (2026-10-28), after the same job had been written four times before it: the `B10` builder's `blocks.py`, the sixth `B10` fix's `split.py`, and two fixers' compare scripts (`fix1-compare.py`, `cmp.py`). The first link review's *timing-insensitive diff* (an `awk` that blanks columns 9 and 12, in [link-review.md](link-review.md) → Tools) asks the third half of its question. Build plan v2 made it a tool, `dev/byte-table-record` (its task `U4`); v3 moved it to Candidates, with the trigger *the first fix that moves a byte-table cell*.

**Called as.** `record-print.py <print.log> <reshape-dir> [<base-rev-cells.tsv>]` — it **writes into `<reshape-dir>`**, so point it at a copy.

`record-print.py`:

```python
#!/usr/bin/env python3
"""Split a byte-table named-binary print into the record's four files and count it.

usage: record-print.py <print.log> <reshape-dir> [<base-rev-cells.tsv>]
Writes byte-table-record-{cells,pairs,needs-and-excused}.tsv and
byte-table-record-what-the-frame-saw.txt under <reshape-dir>, then prints every count
the record keeps, and, given the cells of a base revision, the timing-free diff.
"""
import collections
import sys

RED = ("lost", "git's state moved", "written outside", "refused, and changed",
       "not the listed outcome")
CLASSES = ["lost", "git's state moved outside its allowance", "written outside its allowance",
           "refused, and changed what a refusal may not", "not the listed outcome", "holds",
           "refuses, nothing changed"]


def red(c):
    return any(c.startswith(x) or (" " + x) in c for x in RED)


def strip(lines):
    while lines and not lines[-1].strip():
        lines = lines[:-1]
    return lines


def main():
    log, out = sys.argv[1], sys.argv[2].rstrip("/") + "/"
    L = open(log).read().split("\n")
    at = lambda p: [k for k, l in enumerate(L) if l.startswith(p)][0]
    c, p, e, m, n, s, w = (at("== cells =="), at("== pairs =="), at("== excused =="),
                           at("== marks =="), at("== needs =="), at("== states =="),
                           at("== what the frame saw"))
    end = [k for k, l in enumerate(L) if l.startswith("test byte_table::") and k > w][0]
    files = {
        "byte-table-record-cells.tsv": strip(L[c + 1:p]),
        "byte-table-record-pairs.tsv": strip(L[p + 1:e]),
        "byte-table-record-needs-and-excused.tsv": strip(L[e:m]) + [""] + strip(L[n:s]),
        "byte-table-record-what-the-frame-saw.txt": strip(L[w:end]),
    }
    for name, lines in files.items():
        open(out + name, "w").write("\n".join(lines) + "\n")
    rows = [r.split("\t") for r in files["byte-table-record-cells.tsv"][1:] if r.strip()]
    od = set(l.split("\t")[0] for l in open(out + "byte-table-on-demand-cells.tsv").read().split("\n")
             if l and not l.startswith("#"))
    gate = [r for r in rows if r[0] not in od]
    here = [r for r in rows if red(r[9])]
    there = [r for r in rows if red(r[6])]
    print("cells", len(rows), "gate", len(gate), "on demand", len(rows) - len(gate))
    print("by state (gate)", dict(collections.Counter(r[2] for r in gate)))
    print("red here", len(here), "in gate", len([r for r in here if r[0] not in od]),
          "keys in gate", len({r[5] for r in here if r[0] not in od and r[5] != "-"}))
    print("red there", len(there), "not run", len([r for r in rows if "not run" in r[6]]))
    print("both", len([r for r in rows if red(r[6]) and red(r[9])]),
          "here only", len([r for r in rows if red(r[9]) and not red(r[6])]),
          "there only", len([r for r in rows if red(r[6]) and not red(r[9])]))
    for col, who in ((9, "here"), (6, "there")):
        first, anyc = collections.Counter(), collections.Counter()
        for r in rows:
            parts = [x.strip() for x in r[col].replace("also ", "").split(";")]
            first[parts[0] if not parts[0].startswith("not run") else "not run"] += 1
            for k in CLASSES:
                anyc[k] += any(x == k for x in parts)
        print(who, "first", [first[k] for k in CLASSES], "any", [anyc[k] for k in CLASSES])
    per = collections.defaultdict(lambda: [0, 0, 0, 0])
    for r in rows:
        x = per[r[1].split(" ")[1] if r[1].startswith("jigc ") else r[1]]
        x[0] += 1; x[1] += r[0] not in od; x[2] += red(r[9]); x[3] += red(r[6])
    print("per command (cells, gate, here, there):", dict(per))
    if len(sys.argv) > 3:
        base = {r[0]: r for r in (l.split("\t") for l in open(sys.argv[3]).read().split("\n")[1:] if l.strip())}
        new = {r[0]: r for r in rows}
        for k in sorted(set(base) | set(new)):
            a, b = base.get(k), new.get(k)
            if a is None or b is None or (a[3], a[5], a[6], a[9]) != (b[3], b[5], b[6], b[9]):
                print("MOVED", k, "|", a and (a[5], a[6][:40], a[9][:40]), "->",
                      b and (b[5], b[6][:40], b[9][:40]))


if __name__ == "__main__":
    main()
```

## The raw-write census — `rawwrites.py`, with `enclosing.py`

**The question.** *Which writes bypass the store's write primitives?* `rawwrites.py` lists every raw filesystem or git write call in `crates/cli/src` and `crates/engine/src`, outside `#[cfg(test)]` modules and comments, as `pattern · file:line · text`; `enclosing.py` names the function each `file:line` stands in, so a site can be judged by its caller.

**Raised by.** The second re-review of the link-on-the-way guard (*Tools*: *"a candidate for `dev/`, as the census the brief asked the builder for"*); build plan v2 made it a tool (its task `U5`); v3 moved it to Candidates, with the trigger *the run's fix of the manifest-place key*.

**Called as.** `rawwrites.py <repo-root>` · `enclosing.py <repo-root> <file:line>…`

`rawwrites.py`:

```python
#!/usr/bin/env python3
# rawwrites.py <root>: every raw filesystem/git write call site in crates/{cli,engine}/src,
# outside `#[cfg(test)]` modules and comments. Prints `pattern<TAB>file:line<TAB>text`.
import os, re, sys
root = sys.argv[1]
pats = [r'fs::write\(', r'File::create\(', r'OpenOptions::new\(\)', r'create_dir_all\(', r'fs::rename\(',
        r'fs::copy\(', r'remove_file\(', r'remove_dir_all\(', r'fs::remove_dir\(', r'os::unix::fs::symlink\(',
        r'set_permissions\(', r'\.write_all\(']
gitw = re.compile(r'"(add|commit|mv|rm|worktree|update-index|read-tree|checkout|reset|stash|merge|apply|restore|update-ref|gc|prune|config|branch|tag|cherry-pick|revert|switch|notes|hash-object)"')
for d in ['crates/cli/src', 'crates/engine/src']:
    for dp, _, fs in os.walk(os.path.join(root, d)):
        for f in sorted(fs):
            if not f.endswith('.rs'): continue
            p = os.path.join(dp, f); rel = os.path.relpath(p, root)
            lines = open(p, encoding='utf-8').read().split('\n')
            cut = len(lines)
            for i, l in enumerate(lines):
                if l.strip() == '#[cfg(test)]' and i + 1 < len(lines) and re.match(r'\s*mod \w+', lines[i+1]):
                    cut = i; break
            for i, l in enumerate(lines[:cut]):
                s = l.strip()
                if s.startswith('//'): continue
                for pat in pats:
                    if re.search(pat, l): print(f"{pat}\t{rel}:{i+1}\t{s[:140]}")
                m = gitw.search(l)
                if m and ('git' in l.lower() or '.arg' in l or '[' in l):
                    print(f"git:{m.group(1)}\t{rel}:{i+1}\t{s[:140]}")
```

`enclosing.py`:

```python
#!/usr/bin/env python3
# enclosing.py <root> <file:line>...: the enclosing `fn` name of each site.
import re, sys
root = sys.argv[1]
for site in sys.argv[2:]:
    f, l = site.rsplit(':', 1); l = int(l)
    lines = open(f"{root}/{f}", encoding='utf-8').read().split('\n')
    name = '?'
    for i in range(l - 1, -1, -1):
        m = re.match(r'\s*(pub(\([^)]*\))?\s+)?(async\s+)?fn\s+(\w+)', lines[i])
        if m: name = m.group(4); break
    print(f"{site}\t{name}")
```

## The production call counter — `callcount.py`

**The question.** *How many production call sites does a pattern have?* It counts the lines matching a regex in `crates/{cli,engine}/src`, outside `#[cfg(test)]` modules, comments and `fn` signatures.

**Raised by.** The second re-review of the link-on-the-way guard; build plans v3–v5 (*harvested tools*).

**Called as.** `callcount.py <repo-root> <regex>`

`callcount.py`:

```python
#!/usr/bin/env python3
# callcount.py <root> <regex>: call sites of a pattern in crates/{cli,engine}/src outside
# `#[cfg(test)] mod` blocks and comments.
import os, re, sys
root, pat = sys.argv[1], re.compile(sys.argv[2]); n = 0
for d in ['crates/cli/src', 'crates/engine/src']:
    for dp, _, fs in os.walk(os.path.join(root, d)):
        for f in fs:
            if not f.endswith('.rs'): continue
            lines = open(os.path.join(dp, f), encoding='utf-8').read().split('\n'); cut = len(lines)
            for i, l in enumerate(lines):
                if l.strip() == '#[cfg(test)]' and i + 1 < len(lines) and re.match(r'\s*mod \w+', lines[i+1]): cut = i; break
            for i, l in enumerate(lines[:cut]):
                s = l.strip()
                if s.startswith('//') or re.match(r'(pub(\([^)]*\))?\s+)?fn ', s): continue
                if pat.search(l): n += 1
print(n)
```

## The link read-form probes — `readforms.sh`, `readforms-all.sh`, `readforms2.sh`, `jsonreject.sh`, `dryforecast.sh`

**The question.** Each plants a symbolic link on the way, by hand, and asks one question of a binary. `readforms.sh` / `readforms-all.sh`: *does a read form write past a linked workbench directory?* — fifteen read forms, the outside tree hashed before and after. `readforms2.sh`: the same with the whole `.jigc` linked and an ordinary commit made after the link, so every cache is stale. `jsonreject.sh`: *does a read form that rejects under `--format json` still print exactly one JSON document on stderr while a link stands?* `dryforecast.sh`: *does `task finalize --dry-run` forecast what `task finalize` does, over a link?*

**Raised by.** The three re-reviews of the link-on-the-way guard (2026-10-28), each copying the last one's set; each review names them a candidate for a mode of the axis suite `link_on_the_way_axis` or of the JSON-purity suite. Build plans v3–v5 name *the `readforms-all`, `jsonreject` and `dryforecast` modes*. Every one of the four link reviews also names the tool none of them had: a `dev/jigc-rig` option that plants a link at a named place, to a named target.

**Called as.** `readforms.sh <workbench-dir> <jigc args…>` · `readforms-all.sh <workbench-dir>…` · `readforms2.sh <state> <workbench-dir|.> <jigc args…>` · `jsonreject.sh` · `dryforecast.sh <binary> <docs/decisions|.jigc/index> [json]` — each reads `R`, as said above.

`readforms.sh`:

```bash
#!/bin/bash
# readforms: for a workbench dir X linked to an outside copy, does a read form write past the link?
# usage: readforms.sh <X> <jigc args...>
R=<scratch>/rr4.bSmurF
X=$1; shift
rig=$($R/wt/dev/jigc-rig --binary $R/target/debug/jigc committed-singletons 2>/dev/null) || exit 1; eval "$rig"; cd "$REPO" || exit 1
$JIGC start --workflow quick-fix "fix a typo" >/dev/null 2>&1
O=$(mktemp -d "$RIG/outside.XXXXXX")
mkdir -p ".jigc/$X"; cp -R ".jigc/$X/." "$O"/; mv ".jigc/$X" "$RIG/orig-x"; ln -s "$O" ".jigc/$X"
snap() { (cd "$O" && find . -type f -o -type d | sort | while read -r f; do if [ -f "$f" ]; then echo "$(shasum "$f" | cut -c1-12) $f"; else echo "dir $f"; fi; done); }
snap > "$RIG/before"
"$JIGC" "$@" > "$RIG/out" 2> "$RIG/err"; rc=$?
snap > "$RIG/after"
echo "[$X] jigc $* -> exit=$rc stdout=$(wc -c < "$RIG/out") stderr=$(wc -c < "$RIG/err") first-err: $(head -c 90 "$RIG/err")"
diff "$RIG/before" "$RIG/after" | sed 's/^/    /'
```

`readforms-all.sh`:

```bash
#!/bin/bash
# run readforms.sh over every read form, for the workbench dirs given
R=<scratch>/rr4.bSmurF
forms=("start" "start --explain" "start --workflow router" "workflow single-task --preview" "upgrade" "task validate fix-a-typo" "validate" "doc list" "doc show roadmap" "doc schema adr" "task list" "task diff fix-a-typo" "describe" "config get docs-root" "config list")
for X in "$@"; do for c in "${forms[@]}"; do read -ra a <<< "$c"; "$R/tools/readforms.sh" "$X" "${a[@]}"; done; done
```

`readforms2.sh`:

```bash
#!/bin/bash
# readforms2: like readforms.sh, but X may be "." (the whole .jigc linked), and an ordinary
# commit is made AFTER the link so HEAD moves (every cache is stale). Snapshot is the
# outside tree's files+dirs with shas; also reports any new file in the repo's status.
# usage: readforms2.sh <state> <X> <jigc args...>
R=<scratch>/rr4.bSmurF
ST=$1; X=$2; shift 2
rig=$($R/wt/dev/jigc-rig --binary $R/target/debug/jigc "$ST" 2>/dev/null) || exit 1; eval "$rig"; cd "$REPO" || exit 1
$JIGC start --workflow quick-fix "fix a typo" >/dev/null 2>&1
O=$(mktemp -d "$RIG/outside.XXXXXX")
if [ "$X" = "." ]; then cp -R .jigc/. "$O"/; mv .jigc "$RIG/orig-x"; ln -s "$O" .jigc
else mkdir -p ".jigc/$X"; cp -R ".jigc/$X/." "$O"/; mv ".jigc/$X" "$RIG/orig-x"; ln -s "$O" ".jigc/$X"; fi
echo x >> README.md 2>/dev/null || echo x > README.md; git add README.md; git commit -q -m "an ordinary commit"
snap() { (cd "$O" && find . | sort | while read -r f; do if [ -f "$f" ]; then echo "$(shasum "$f" | cut -c1-12) $f"; else echo "d $f"; fi; done); }
snap > "$RIG/before"
"$JIGC" "$@" > "$RIG/out" 2> "$RIG/err"; rc=$?
snap > "$RIG/after"
echo "[$ST $X] jigc $* -> exit=$rc out=$(wc -c < "$RIG/out") err=$(wc -c < "$RIG/err") | $(head -c 100 "$RIG/err" | tr '\n' ' ')"
diff "$RIG/before" "$RIG/after" | sed 's/^/    WROTE /'
```

`jsonreject.sh`:

```bash
#!/bin/bash
# jsonreject: a read form that REJECTS under --format json, with a link on the way:
# does stderr still parse as exactly one JSON document?
R=<scratch>/rr4.bSmurF
rig=$($R/wt/dev/jigc-rig --binary $R/target/debug/jigc committed-singletons 2>/dev/null) || exit 1; eval "$rig"; cd "$REPO" || exit 1
O=$(mktemp -d "$RIG/outside.XXXXXX"); mkdir -p docs; ln -s "$O" docs/decisions
for args in "doc show no-such-doc" "task validate no-such-task" "task diff no-such-task" "doc schema no-such-type" "workflow no-such-wf --preview" "config get no-such-key"; do
  read -ra a <<< "$args"
  $JIGC --format json "${a[@]}" >"$RIG/o" 2>"$RIG/e"; rc=$?
  so=$(wc -c <"$RIG/o"); if python3 -I -c 'import json,sys; json.load(open(sys.argv[1]))' "$RIG/e" 2>/dev/null; then pe=parses; else pe=NOT-ONE-DOC; fi
  echo "[$args] exit=$rc stdout=$so stderr-json=$pe; stderr head: $(head -c 70 "$RIG/e" | tr '\n' ' ')"
done
echo "== control, no link:"; rm docs/decisions
$JIGC --format json doc show no-such-doc >"$RIG/o" 2>"$RIG/e"; echo "exit=$? $(python3 -I -c 'import json,sys; json.load(open(sys.argv[1])); print("parses")' "$RIG/e" 2>&1 | tail -1)"
```

`dryforecast.sh`:

```bash
#!/bin/bash
# dryforecast: an authored task ready to land, and a link on the way (a home, or the
# workbench's index). Does `task finalize --dry-run` forecast what `task finalize` does?
# usage: dryforecast.sh <binary> <docs/decisions|.jigc/index> [json]
R=<scratch>/rr4.bSmurF
rig=$($R/wt/dev/jigc-rig --binary "$1" committed-singletons 2>/dev/null) || exit 1; eval "$rig"; cd "$REPO" || exit 1
$JIGC start --workflow quick-fix "fix a typo" >/dev/null 2>&1
$JIGC doc set-field commit:fix-a-typo#header/type --task fix-a-typo --value fix >/dev/null
printf 'Fix a typo in the readme.\n' | $JIGC doc set-slot commit:fix-a-typo#summary --task fix-a-typo --from-file - >/dev/null
echo "typo fixed" >> README.md; git add README.md
O=$(mktemp -d "$RIG/outside.XXXXXX"); mkdir -p "$(dirname "$2")"
if [ -d "$2" ]; then cp -R "$2/." "$O"/; mv "$2" "$RIG/orig"; fi; ln -s "$O" "$2"
F=; [ "$3" = json ] && F="--format json"
$JIGC $F task finalize fix-a-typo --dry-run > "$RIG/o" 2> "$RIG/e"; echo "[$2] dry-run exit=$? $(head -c 160 "$RIG/o" | tr '\n' ' ')"
$JIGC task finalize fix-a-typo > "$RIG/o2" 2> "$RIG/e2"; echo "[$2] real    exit=$? $(head -c 100 "$RIG/e2" | tr '\n' ' ')"
```

## The two-binary timing — `n4-measure.py`

**The question.** *What does a change cost a command, cold and warm?* It copies each of two binaries fresh, runs four commands once (cold) and then `N` times (the warm median), in one corpus, and prints both per binary.

**Raised by.** The link-on-the-way fix, for its review's `N4` (the guard's cost: +5 to +8 ms a debug command — `DECISIONS.md` → *2026-10-28 — The review of the link-on-the-way guard*); build plans v3–v5 (*harvested tools*).

**Called as.** `n4-measure.py <repo> <old-binary> <new-binary> <N>` — it writes `payload.yaml` into `<repo>`.

`n4-measure.py`:

```python
#!/usr/bin/env python3
"""Time a few commands on two jigc binaries in one corpus: the first run of a fresh copy
of each binary (cold) and the median of N runs after (warm). usage: n4-measure.py <repo> <old> <new> <N>"""
import shutil, statistics, subprocess, sys, tempfile, time, os
repo, old, new, n = sys.argv[1], sys.argv[2], sys.argv[3], int(sys.argv[4])
CMDS = [
    ("doc set-slot (guarded, refused after the guard: no such task)",
     ["doc", "set-slot", "adr:nope#context", "--from-file", "payload.yaml", "--task", "nope"]),
    ("start (orientation: read form)", ["start"]),
    ("task list (read leaf)", ["task", "list"]),
    ("config get docs-root (read leaf)", ["config", "get", "docs-root"]),
]
open(os.path.join(repo, "payload.yaml"), "w").write("title: X\n")
def run(binary, argv):
    t = time.perf_counter()
    subprocess.run([binary] + argv, cwd=repo, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    return (time.perf_counter() - t) * 1000
for label, binary in (("old", old), ("new", new)):
    d = tempfile.mkdtemp()
    for name, argv in CMDS:
        fresh = os.path.join(d, name.split()[0] + "-" + str(abs(hash(name))))
        shutil.copy(binary, fresh)
        cold = run(fresh, argv)
        warm = statistics.median(run(fresh, argv) for _ in range(n))
        print(f"{label}\t{name}\tcold {cold:.0f} ms\twarm median {warm:.1f} ms")
```

## The flipping cells, as shell repros — `link-cells-repro.sh`

**The question.** *Do the two byte-table cells CI failed on `0bd8a5b5` flip from run to run, or by git version?* It drives both cells' commands `N` times on one binary and prints each run's exit and what stands in the linked directory outside. Run 30 times per environment, it showed them steady within a git and different across gits — macOS git 2.54 and Ubuntu's stock 2.43 write outside every time, the git-core PPA's 2.56 refuses every time: not a flake ([link-on-the-way-brief.md](link-on-the-way-brief.md); `DECISIONS.md` → *2026-10-28 — No door writes through a link on the way*).

**Raised by.** The fixer of [ci-flaky-byte-table-brief.md](ci-flaky-byte-table-brief.md), whose tool the link-on-the-way brief hands on to the next fixer by name; build plans v3–v5 (*harvested tools*).

**Called as.** `link-cells-repro.sh <repo-root> <jigc-binary> <N>`

`link-cells-repro.sh`:

```bash
#!/usr/bin/env bash
# The two byte-table cells that flip, as shell repros, run N times each.
# usage: link-cells-repro.sh <repo-root> <jigc-binary> <N>
# Prints one line per run: cell, run, the door's exit, and what stands in the linked directory outside.
set -u
root=$1 bin=$2 n=$3
git --version
for i in $(seq 1 "$n"); do
    # config-set-a-link-to-a-directory-outside-at-the-decisions-new-directory
    (
        rig=$("$root"/dev/jigc-rig --binary "$bin" fresh 2>/dev/null) || { echo "rig failed"; exit 1; }
        eval "$rig"; cd "$REPO" || exit 1
        mkdir -p docs/decisions notes
        printf -- '---\nschema-version: 2\nstatus: accepted\n---\n\n# One way\n\n## Context\n\nWe had to choose.\n\n## Options\n\n\n\n## Decision\n\nOne way.\n\n## Consequences\n\nIt holds.\n' > docs/decisions/one-way.md
        git add -- docs && git commit -q -m "a decision"
        O=$(mktemp -d "$RIG/outside.XXXXXX"); ln -s "$O" notes/decisions
        "$JIGC" config set docs-root notes >/dev/null 2>&1; rc=$?
        echo "config-set-new-dir run=$i exit=$rc outside=[$(ls -A "$O" | tr '\n' ' ')]"
    )
    # relocate-a-link-to-a-directory-outside-at-the-notes-new-directory
    (
        rig=$("$root"/dev/jigc-rig --binary "$bin" fresh 2>/dev/null) || { echo "rig failed"; exit 1; }
        eval "$rig"; cd "$REPO" || exit 1
        mkdir -p note-pack/schemas docs/legacy-notes
        printf 'type: note\nlocation: notes/\nid-from: title\nsections:\n  - id: body\n    slot: { hint: "The note." }\n' > note-pack/schemas/note.yaml
        printf 'packs:\n  - note-pack\ncompose-embedded-methodology: true\n' > .jigc/config/packs.yaml
        printf '# Cache benchmarks\n\n## Body\n\nA single node caps throughput.\n' > docs/legacy-notes/cache-benchmarks.md
        git add -- note-pack .jigc/config/packs.yaml docs && git commit -q -m "a note pack, and a note at its old home"
        O=$(mktemp -d "$RIG/outside.XXXXXX"); ln -s "$O" docs/notes
        out=$("$JIGC" relocate note --from docs/legacy-notes/ 2>&1); rc=$?
        tally=$(printf '%s\n' "$out" | command grep -o '[0-9]* moved, [0-9]* displaced, [0-9]* blocked')
        echo "relocate-new-dir run=$i exit=$rc [$tally] outside=[$(ls -A "$O" | tr '\n' ' ')]"
    )
done
```

## The `git mv` probe — `gitmv-probe.sh`

**The question.** *Does this machine's git carry a file through a link at `git mv`'s destination?* git 2.55 and older follow the link; 2.56 refuses — the fact behind CI's red on `0bd8a5b5`.

**Raised by.** The diagnosis of that red; named by no brief, review or plan, and how often it ran is not recorded. Kept because it asks the question without jigc, in one throwaway repository, and the question returns whenever the runner's git moves.

**Called as.** `gitmv-probe.sh` — it builds its repository under `mktemp -d`.

`gitmv-probe.sh`:

```sh
#!/bin/sh
# git mv to a destination whose parent directory is a symbolic link to a directory outside the repository
set -e
T=$(mktemp -d)
mkdir "$T/outside" "$T/repo"
cd "$T/repo"
git init -q .
git -c user.name=t -c user.email=t@example.invalid commit -q --allow-empty -m init
mkdir -p docs/legacy && echo hi > docs/legacy/a.md
git add docs && git -c user.name=t -c user.email=t@example.invalid commit -q -m a
ln -s "$T/outside" docs/new
git --version
if git mv docs/legacy/a.md docs/new/a.md; then echo "MV: exit 0"; else echo "MV: exit $?"; fi
echo "outside holds: $(ls "$T/outside")"
```

## The sanitizer — `sanitize.py`

**The question.** *Make this file safe for a public commit*: it copies one file and replaces every absolute host path — a path into this repository by its repo-relative form, a scratch path by `<scratch>`, another temporary path by `<tmp>`, a home path by `~`, the project's directory under `~/.claude/projects/` by `<project>` — and every session link or id by `<session>`, printing how many each rule replaced. [`dev/hygiene-scan`](../../../../../dev/hygiene-scan) finds what must not be committed; this removes the host-path half of it before the scan.

**Raised by.** The move of 2026-10-29 that made this directory: every file here went through it. Fifty-one sources were sanitized by it.

**Called as.** `HOME=… SANITIZE_TMP=<the sessions' temporary root> [SANITIZE_REPO=<repo-root>] sanitize.py <src> <dst>`. **Kept with its literals made parameters:** as it ran, it named this machine's home, temporary root and project directory in its own text — the very host paths it removes, so that text cannot be committed. This form reads them from the environment; run over the same fifty-one sources with this machine's values, it wrote every file byte for byte as the form that ran.

`sanitize.py`:

```python
#!/usr/bin/env python3
"""Copy one file, replacing host paths, session ids and the login name.

usage: sanitize.py <src> <dst>
The machine's own values come from the environment, never from this text: HOME (the
home directory), and SANITIZE_REPO (the repository's root), SANITIZE_TMP (the temporary
root sessions keep their scratch directories under) when they differ from the defaults.
Prints the number of replacements per rule to stderr.
"""
import os
import re
import sys

src, dst = sys.argv[1], sys.argv[2]
home = os.environ["HOME"].rstrip("/")
repo = os.environ.get("SANITIZE_REPO", os.getcwd()).rstrip("/")
tmp = os.environ["SANITIZE_TMP"].rstrip("/")
# a project's directory under ~/.claude/projects is its repository path, `/` as `-`
project = repo.replace("/", "-")
s = open(src, encoding="utf-8").read()

UUID = r"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}"
E = re.escape
RULES = [
    # the session scratchpad, and anything else under the session's temp root
    (E(tmp + "/" + project + "/") + UUID + "/scratchpad", "<scratch>"),
    (E(tmp + "/" + project), "<tmp>"),
    (E(tmp), "<tmp>"),
    # macOS spells the same root without its `/private` prefix too
    (E(tmp.removeprefix("/private")), "<tmp>"),
    # this project's own home under ~/.claude
    (E(home + "/.claude/projects/" + project), "~/.claude/projects/<project>"),
    # a path into this repository: its repo-relative form
    (E(repo + "/"), ""),
    # a sibling worktree, or the repository root named bare: a home path
    (E(home + "/"), "~/"),
    (E(project), "<project>"),
    # the session's link and its id
    (r"https://claude\.ai/code/session_[A-Za-z0-9]+", "<session>"),
    (r"session_[A-Za-z0-9]{16,}", "<session>"),
    (UUID, "<session-id>"),
]
counts = []
for pat, rep in RULES:
    s, n = re.subn(pat, rep, s)
    counts.append((pat, n))
open(dst, "w", encoding="utf-8").write(s)
for pat, n in counts:
    if n:
        print(f"{n:4d}  {pat}", file=sys.stderr)
```

## The landed check — `landed.sh`

**The question.** *Has this draft landed in the repository?* For each file, how many of its lines of 60 characters or more stand verbatim in a tracked markdown file; short lines are skipped, because they match anywhere. A draft whose every long line is found has landed; a low count says it was rewritten or never committed — read it before dropping it.

**Raised by.** The same move: the drafts in the scratch directory — of the byte table's record sections, the opening sheet's §0 — were checked with it before they were dropped (the [README](README.md) → *Not kept*).

**Called as.** `landed.sh <repo-root> <file>…`

`landed.sh`:

```bash
#!/usr/bin/env bash
# landed.sh <repo-root> <file>... — has a draft landed in the repository?
# For each file: how many of its lines of 60 characters or more stand verbatim in a
# tracked markdown file of <repo-root>. Short lines are skipped: they match anywhere.
root=$1; shift
for f in "$@"; do
    n=0; m=0
    while IFS= read -r l; do
        [ ${#l} -lt 60 ] && continue
        n=$((n + 1))
        git -C "$root" grep -qF -e "$l" -- '*.md' && m=$((m + 1))
    done < "$f"
    printf '%s\t%d/%d\n' "$f" "$m" "$n"
done
```
