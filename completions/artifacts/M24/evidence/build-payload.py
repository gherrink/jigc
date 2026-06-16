#!/usr/bin/env python3
"""M24 inc-7 T4 — the agent's batch-payload authoring tool (Framing A).

Reads a foreign `CHANGELOG.md` and emits ONE declarative `jigc doc author
changelog --from` payload (YAML) plus a `<slug>.counts` file recording the
per-leaf-equivalent round-trip count for the agent-call-count metric (d).

This script *is* the coding agent rewriting the foreign prose: the
foreign-category/prefix/verb -> `category` enum mapping below is the agent's
editorial judgment, recorded as code so the measured run is reproducible. It
PLACES nothing — it only produces the payload the CLI applies leaf-by-leaf
(`auto-migration.md` -> Hardening #1/#7; the determinism boundary stays intact:
agent authors the payload, CLI owns every splice).

Mapping (map + MERGE many-to-one onto the enum {added, changed, deprecated,
removed, fixed, security}):
  - project-delta category headings: Features->added; Fixes/Hotfix->fixed;
    Changes/Improvements/Others->changed (the merge case — two foreign headings
    collapse onto one member, since the enum is the id-from and two groups
    can't share an id).
  - project-gamma conventional-commit prefixes: feat:->added; fix:->fixed;
    refactor:/reactor:/chore:/docs:/perf:/style:/ci:/build:->changed.
  - plain (headingless, prefixless) bullets: inferred from the leading verb —
    fix*->fixed; add*/new/include*/introduce*->added; remove*->removed; else
    changed.
Bullet PROSE is preserved verbatim (only `* ` normalized to `- `, trailing
whitespace trimmed, blank lines dropped); the original conventional-commit
prefix is kept inside the note so nothing is lost on the merge.
"""
import re
import sys

ENUM_ORDER = ["added", "changed", "deprecated", "removed", "fixed", "security"]
GROUP_TITLE = {
    "added": "Added",
    "changed": "Changed",
    "deprecated": "Deprecated",
    "removed": "Removed",
    "fixed": "Fixed",
    "security": "Security",
}

HEADING_MEMBER = {
    "features": "added",
    "feature": "added",
    "added": "added",
    "fixes": "fixed",
    "fix": "fixed",
    "hotfix": "fixed",
    "bugfix": "fixed",
    "bug fixes": "fixed",
    "changes": "changed",
    "improvements": "changed",
    "others": "changed",
    "other": "changed",
    "removed": "removed",
    "deprecated": "deprecated",
    "security": "security",
}

PREFIX_MEMBER = {
    "feat": "added",
    "feature": "added",
    "fix": "fixed",
    "refactor": "changed",
    "reactor": "changed",
    "chore": "changed",
    "docs": "changed",
    "perf": "changed",
    "style": "changed",
    "test": "changed",
    "ci": "changed",
    "build": "changed",
}


def bullet_member(text):
    """Classify one top-level bullet's prose onto an enum member (the agent's call)."""
    body = text.strip()
    m = re.match(r"^([a-zA-Z]+):", body)
    if m:
        pref = m.group(1).lower()
        if pref in PREFIX_MEMBER:
            return PREFIX_MEMBER[pref]
    first = re.split(r"[\s:]", body.lower(), maxsplit=1)[0]
    if first.startswith("fix") or first.startswith("correct"):
        return "fixed"
    if (
        first.startswith("add")
        or first in ("new", "include", "includes", "introduce", "introduced")
    ):
        return "added"
    if first.startswith("remove"):
        return "removed"
    return "changed"


def is_release_heading(line):
    """`# <version>` (level-1). Returns the version text, or None."""
    m = re.match(r"^#\s+(.+?)\s*$", line)
    if not m:
        return None
    return m.group(1).strip()


def is_subheading(line):
    m = re.match(r"^##\s+(.+?)\s*$", line)
    if not m:
        return None
    return m.group(1).strip()


def is_top_bullet(line):
    return bool(re.match(r"^[-*]\s+", line))


def norm_line(line):
    """Normalize a content line: `* ` -> `- ` at the start, rstrip."""
    line = line.rstrip()
    line = re.sub(r"^(\s*)\*(\s)", r"\1-\2", line)
    return line


def parse_releases(text):
    """-> list of (version, [(member, [lines])]) preserving first-appearance order."""
    lines = text.splitlines()
    releases = []
    i = 0
    n = len(lines)
    while i < n:
        ver = is_release_heading(lines[i])
        if ver is None:
            i += 1
            continue
        i += 1
        body = []
        while i < n and is_release_heading(lines[i]) is None:
            body.append(lines[i])
            i += 1
        if ver.lower() == "new":
            continue  # the empty Unreleased placeholder — accepted drop
        groups = parse_body(body)
        if groups:
            releases.append((ver, groups))
    return releases


def parse_body(body):
    """Body lines -> ordered [(member, [content lines])], merging onto members."""
    # Detect project-delta-style `##` category headings.
    has_subheadings = any(is_subheading(l) is not None for l in body)
    # Collect (member, block-lines) in document order.
    blocks = []
    if has_subheadings:
        current_member = None
        cur = []
        for l in body:
            h = is_subheading(l)
            if h is not None:
                if cur and current_member:
                    blocks.append((current_member, cur))
                current_member = HEADING_MEMBER.get(h.lower(), "changed")
                cur = []
            elif l.strip() == "":
                continue
            elif current_member is not None:
                cur.append(norm_line(l))
        if cur and current_member:
            blocks.append((current_member, cur))
    else:
        # Prefix/plain bullets at the top level; sub-indented lines stay with parent.
        cur_lines = None
        cur_member = None
        for l in body:
            if l.strip() == "":
                continue
            if is_top_bullet(l):
                if cur_lines is not None:
                    blocks.append((cur_member, cur_lines))
                cur_member = bullet_member(re.sub(r"^[-*]\s+", "", l))
                cur_lines = [norm_line(l)]
            else:
                # continuation / nested sub-bullet of the current block
                if cur_lines is not None:
                    cur_lines.append(norm_line(l))
        if cur_lines is not None:
            blocks.append((cur_member, cur_lines))

    # Merge blocks by member, first-appearance order.
    merged = {}
    order = []
    for member, lines in blocks:
        if member not in merged:
            merged[member] = []
            order.append(member)
        merged[member].extend(lines)
    return [(m, merged[m]) for m in order]


def emit_payload(releases):
    out = ["title: Changelog", "sections:", "  - id: releases", "    items:"]
    for ver, groups in releases:
        out.append(f'      - title: "{ver}"')
        out.append("        sections:")
        out.append("          - id: changes")
        out.append("            items:")
        for member, lines in groups:
            out.append(f"              - title: {GROUP_TITLE[member]}")
            out.append("                set:")
            out.append("                  notes: |")
            # literal block; first line opens `<<`, last closes `>>`
            content = list(lines)
            content[0] = "<<" + content[0]
            content[-1] = content[-1] + ">>"
            for cl in content:
                out.append("                    " + cl)
    return "\n".join(out) + "\n"


def main():
    src, out_payload, out_counts = sys.argv[1], sys.argv[2], sys.argv[3]
    with open(src, encoding="utf-8") as f:
        text = f.read()
    releases = parse_releases(text)
    payload = emit_payload(releases)
    with open(out_payload, "w", encoding="utf-8") as f:
        f.write(payload)
    n_releases = len(releases)
    n_groups = sum(len(g) for _, g in releases)
    # per-leaf equivalent: create + add-item(release) + add-item(group) + set-slot(notes)
    per_leaf = 1 + n_releases + 2 * n_groups
    with open(out_counts, "w", encoding="utf-8") as f:
        f.write(f"releases={n_releases}\n")
        f.write(f"change_groups={n_groups}\n")
        f.write(f"per_leaf_round_trips={per_leaf}\n")
        f.write("batch_author_calls=1\n")
    print(f"{src}: {n_releases} releases, {n_groups} change-groups, "
          f"per-leaf={per_leaf}, batch=1")


if __name__ == "__main__":
    main()
