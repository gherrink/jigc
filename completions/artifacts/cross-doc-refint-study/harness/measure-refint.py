#!/usr/bin/env python3
"""Arm-agnostic CROSS-DOC FORWARD-REF integrity oracle for ONE committed HEAD.

The cross-doc analog of the doc<->code study's measure.py. Where that oracle walked
`implemented-by:` anchors into the *code* via tree-sitter, this one walks the managed
store's own forward-edge graph (`supersedes`: adr->adr, `cites`: arch-doc->adr) and asks
the question a static CLAUDE.md rule cannot make a cold agent ask: **does every committed
forward reference still resolve to a doc that exists in the store?**

It is the oracle, not enforcement — the SAME measurement runs on every arm (jigc and static
alike). Two independent paths:

  (a) EDGE-WALKER (arm-agnostic, the headline) — parse each managed doc's front-matter
      `supersedes`/`cites` values, resolve each `<type>:<slug>` target against the store's
      canonical path (`docs/<location>/<slug>.md`), and count edges whose target file is
      ABSENT. Runs on jigc, static, and plain arms identically (pure file existence).
  (b) JIGC CROSS-CHECK (arm A only, when `.jigc/` exists) — drive `jigc validate
      --format json` and count `schema-conformance.ref-resolves` findings. Required to
      AGREE with (a) on arm A; a disagreement is recorded as an oracle integrity flag.

Drift for the HEAD = count of live dangling cross-doc edges (path a). Pure stdlib.

On-disk forms this parses (confirmed against the real binary, 2026-06-24):
  - single:  `supersedes: adr:single-node-cache`
  - bracket: `cites: [adr:single-node-cache, adr:shared-redis-cache]`
  - YAML list (defensive — jigc emits inline, but handle it):
        supersedes:
          - adr:a
          - adr:b

Usage:
    measure-refint.py --repo DIR [--jigc PATH] [--no-jigc]
    measure-refint.py --selftest
"""
from __future__ import annotations
import argparse, json, re, subprocess, sys, tempfile
from pathlib import Path

DEFAULT_JIGC = str(Path.home() / ".local/bin/jigc")

# The store's doctype -> managed location, under the `docs/` root (the canonical-path rule
# `docs/<location>/<slug>.md`, confirmed against the real binary). The seed corpus uses
# exactly these two persisted doctypes; extend this map if the corpus grows a third.
TYPE_LOCATIONS = {
    "adr": "docs/decisions",
    "arch-doc": "docs/architecture",
}
# The forward-ref fields the corpus carries (both `to: adr`). Field id -> nothing; the set
# of keys is what matters. `cites-code` / `implemented-by` are CODE anchors, not doc refs —
# deliberately excluded (this study is cross-doc refs only; the seed carries no code anchor).
REF_FIELDS = {"supersedes", "cites"}

# A `<type>:<slug>` reference token: a lowercase-hyphen type, a colon, a lowercase-hyphen
# slug. Matches `adr:single-node-cache`, `arch-doc:session-subsystem`; never matches a bare
# field key like `supersedes:` (no slug immediately after the colon).
_REF_TOKEN = re.compile(r"\b([a-z][a-z0-9-]*:[a-z0-9-]+)\b")


def _front_matter(text: str) -> str:
    """The YAML front-matter block (between the first two `---` fences), or "" if none."""
    m = re.match(r"\A﻿?---\n(.*?)\n---\n", text, re.S)
    return m.group(1) if m else ""


def doc_edges(text: str) -> list[tuple[str, str]]:
    """Every forward edge a doc contributes, as `(relation, target_identity)` pairs.

    Parses ONLY the front-matter, ONLY the `REF_FIELDS` keys, handling inline-bare,
    inline-bracket, and YAML-list value forms. The target identity is the raw
    `<type>:<slug>` token (already type-qualified by jigc)."""
    fm = _front_matter(text)
    edges: list[tuple[str, str]] = []
    lines = fm.splitlines()
    i = 0
    while i < len(lines):
        line = lines[i]
        m = re.match(r"^([a-z][a-z0-9-]*):\s*(.*)$", line)
        if not m:
            i += 1
            continue
        key, val = m.group(1), m.group(2)
        if key not in REF_FIELDS:
            i += 1
            continue
        if val.strip():
            # inline value (bare `adr:x` or bracket `[adr:a, adr:b]`)
            for tok in _REF_TOKEN.findall(val):
                edges.append((key, tok))
            i += 1
        else:
            # YAML-list continuation: consume following `- <token>` lines (defensive)
            i += 1
            while i < len(lines) and re.match(r"^\s*-\s+", lines[i]):
                for tok in _REF_TOKEN.findall(lines[i]):
                    edges.append((key, tok))
                i += 1
    return edges


def canonical_path(repo: Path, identity: str) -> Path | None:
    """The store path a `<type>:<slug>` identity resolves to, or None for an unknown type."""
    ty, _, slug = identity.partition(":")
    loc = TYPE_LOCATIONS.get(ty)
    if loc is None or not slug:
        return None
    return repo / loc / f"{slug}.md"


def walk_edges(repo: Path) -> dict:
    """Path (a): every forward edge in the store + which ones dangle (target file absent)."""
    edges: list[dict] = []
    dangling: list[dict] = []
    for ty, loc in TYPE_LOCATIONS.items():
        d = repo / loc
        if not d.is_dir():
            continue
        for path in sorted(d.glob("*.md")):
            frm = f"{ty}:{path.stem}"
            text = path.read_text(encoding="utf-8", errors="replace")
            for relation, target in doc_edges(text):
                tgt_path = canonical_path(repo, target)
                resolves = bool(tgt_path and tgt_path.exists())
                rec = {"from": frm, "relation": relation, "to": target, "resolves": resolves}
                edges.append(rec)
                if not resolves:
                    dangling.append(rec)
    return {"edges_total": len(edges), "n_dangling": len(dangling),
            "dangling": dangling, "edges": edges}


def jigc_ref_findings(repo: Path, jigc: str) -> int | None:
    """Path (b): `schema-conformance.ref-resolves` finding count from `jigc validate`, or
    None when this arm is not a jigc project (no `.jigc/`) or the binary is unavailable."""
    if not (repo / ".jigc").is_dir():
        return None
    try:
        out = subprocess.run([jigc, "validate", "--format", "json"], cwd=repo,
                             capture_output=True, text=True)
    except OSError:
        return None
    if not out.stdout.strip():
        return None
    try:
        findings = json.loads(out.stdout).get("findings", [])
    except json.JSONDecodeError:
        return None
    return sum(1 for f in findings
               if f.get("probe") == "schema-conformance" and f.get("check") == "ref-resolves")


def measure(repo: Path, jigc: str = DEFAULT_JIGC, use_jigc: bool = True) -> dict:
    a = walk_edges(repo)
    rec = {
        "edges_total": a["edges_total"],
        "n_dangling": a["n_dangling"],
        "dangling": [f"{e['from']}#{e['relation']} -> {e['to']}" for e in a["dangling"]],
    }
    jigc_n = jigc_ref_findings(repo, jigc) if use_jigc else None
    rec["jigc_ref_findings"] = jigc_n
    # Agreement is only defined on a jigc arm (path b ran). A mismatch flags an oracle
    # integrity problem (the walker and jigc disagree about the same store) — never
    # silently reconciled.
    rec["oracle_agrees"] = (jigc_n is None) or (jigc_n == a["n_dangling"])
    return rec


# --------------------------------------------------------------------------- selftest
def _write(p: Path, body: str) -> None:
    p.parent.mkdir(parents=True, exist_ok=True)
    p.write_text(body)


def _adr(title: str, supersedes: str | None = None) -> str:
    sup = f"supersedes: {supersedes}\n" if supersedes else ""
    return (f"---\nstatus: accepted\ndate: 2026-06-24\n{sup}---\n\n"
            f"# {title}\n\n## Context\nc\n\n## Decision\nd\n\n## Consequences\ne\n")


def _archdoc(title: str, cites: str) -> str:
    return (f"---\ncites: {cites}\n---\n\n# {title}\n\n## Overview\no\n\n## Components\n")


def _selftest() -> int:
    fails = 0

    def check(cond, msg):
        nonlocal fails
        if not cond:
            print(f"FAIL: {msg}")
            fails += 1

    # edge parsing — bare, bracket, list, and a non-ref key ignored
    e1 = doc_edges(_adr("B", "adr:a"))
    check(e1 == [("supersedes", "adr:a")], f"bare supersedes: {e1}")
    e2 = doc_edges(_archdoc("Sys", "[adr:a, adr:b]"))
    check(e2 == [("cites", "adr:a"), ("cites", "adr:b")], f"bracket cites: {e2}")
    e3 = doc_edges("---\nsupersedes:\n  - adr:a\n  - adr:b\n---\n# X\n")
    check(e3 == [("supersedes", "adr:a"), ("supersedes", "adr:b")], f"yaml-list: {e3}")
    e4 = doc_edges(_adr("Standalone"))
    check(e4 == [], f"no edges when no ref field: {e4}")
    # the field key itself is never mistaken for a target
    check(all(t != "supersedes:" for _, t in e1), "field key leaked as token")

    with tempfile.TemporaryDirectory() as d:
        repo = Path(d)
        # clean store: B supersedes A, arch cites A+B — all resolve
        _write(repo / "docs/decisions/a.md", _adr("A"))
        _write(repo / "docs/decisions/b.md", _adr("B", "adr:a"))
        _write(repo / "docs/architecture/sys.md", _archdoc("Sys", "[adr:a, adr:b]"))
        m = measure(repo, use_jigc=False)
        check(m["edges_total"] == 3, f"3 edges total: {m}")
        check(m["n_dangling"] == 0, f"clean store 0 dangling: {m}")

        # delete A -> B#supersedes dangles AND arch#cites->a dangles = 2
        (repo / "docs/decisions/a.md").unlink()
        m = measure(repo, use_jigc=False)
        check(m["n_dangling"] == 2, f"deleting A dangles 2 edges: {m}")
        check(any("b.md".replace(".md", "") in s or "adr:a" in s for s in m["dangling"]),
              f"dangling names adr:a: {m['dangling']}")

        # rename b -> b2 (slug change): arch still cites adr:b -> now dangles too
        (repo / "docs/decisions/b.md").rename(repo / "docs/decisions/b2.md")
        m = measure(repo, use_jigc=False)
        # edges now: b2#supersedes->adr:a (dangle), arch#cites->adr:a (dangle),
        #            arch#cites->adr:b (dangle, b renamed)  = 3
        check(m["n_dangling"] == 3, f"rename compounds dangling to 3: {m}")

    if fails:
        print(f"{fails} failure(s)")
        return 1
    print("measure-refint.py selftest OK")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--repo")
    ap.add_argument("--jigc", default=DEFAULT_JIGC)
    ap.add_argument("--no-jigc", action="store_true",
                    help="skip the jigc cross-check (path b) even if .jigc exists")
    ap.add_argument("--selftest", action="store_true")
    a = ap.parse_args()
    if a.selftest:
        return _selftest()
    if not a.repo:
        ap.error("need --repo (or --selftest)")
    rec = measure(Path(a.repo), a.jigc, use_jigc=not a.no_jigc)
    json.dump(rec, sys.stdout, indent=2)
    sys.stdout.write("\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
