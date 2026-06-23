#!/usr/bin/env python3
"""Arm-agnostic doc<->code drift oracle for ONE edit of a long-horizon sequence.

The same measurement runs on every arm (jigc and static alike) — it is the oracle,
not enforcement. It reads the arch-doc's current `implemented-by:` anchors and:

1. drives the real `doc-code` probe (tree-sitter AST resolution — the floor's exact
   predicate) over them against the working tree → DANGLING ANCHORS (the surface the
   blocking hook governs);
2. greps the whole doc for any CUMULATIVE DEAD NAME — the old name of every
   rename/delete edit up to and including this one — → STALE PROSE/TITLE references
   (the prose blind spot the anchor check misses; the pilot's headline grep).

Drift for the edit = a dangling anchor OR a dead name present anywhere in the doc.

The probe speaks the engine's stdin protocol: a ProbeRequest pointing at a snapshot
file ({anchors, working_tree_root}); it returns {findings,...}. Pure stdlib.

Usage:
    measure.py --repo DIR --seq sequence.json --edit N [--probe PATH]
    measure.py --selftest
"""
from __future__ import annotations
import argparse, json, re, subprocess, sys, tempfile
from pathlib import Path

DEFAULT_PROBE = str(Path.home() / ".local/bin/doc-code")
_ANCHOR_RE = re.compile(r"^\s*-\s*implemented-by:\s*(\S+)\s*$", re.M)


def doc_anchors(doc_text: str) -> list[str]:
    """Every `implemented-by: <value>` anchor in the doc, in order."""
    return _ANCHOR_RE.findall(doc_text)


def run_probe(anchors: list[str], repo: Path, probe: str) -> list[dict]:
    """Drive the doc-code probe over `anchors` against `repo`; return its findings."""
    if not anchors:
        return []
    snap = {
        "anchors": [
            {"address": f"doc#{i}", "anchor_value": a, "check_id": "symbol-exists"}
            for i, a in enumerate(anchors)
        ],
        "working_tree_root": str(repo),
    }
    with tempfile.NamedTemporaryFile("w", suffix=".json", delete=False) as f:
        json.dump(snap, f)
        snap_path = f.name
    req = {
        "probe_id": "doc-code", "target": "measure",
        "effective_state": {"snapshot_path": snap_path},
        "config": {}, "schema_version": 2,
    }
    out = subprocess.run([probe], input=json.dumps(req), capture_output=True, text=True)
    Path(snap_path).unlink(missing_ok=True)
    if out.returncode != 0 or not out.stdout.strip():
        raise RuntimeError(f"probe failed (rc={out.returncode}): {out.stderr[:200]}")
    return json.loads(out.stdout).get("findings", [])


def dead_candidates(edits: list[dict], upto: int) -> list[str]:
    """Names INTENDED to be absent from code after edit `upto`: the `old` of every
    rename/delete edit so far (a move keeps its symbol, so it is excluded). A name that
    was a rename target and later renamed away (compounding) is captured as a later
    edit's `old`."""
    out = []
    for e in edits:
        if e["n"] > upto:
            break
        if e["type"] in ("rename", "delete"):
            out.append(e["old"])
    return list(dict.fromkeys(out))


_DECL_RE = "(?:abstract\\s+)?(?:export\\s+)?(?:default\\s+)?(?:class|interface|type|function|const|enum)\\s+{name}\\b"


def code_has_symbol(repo: Path, name: str) -> bool:
    """True if `name` is DECLARED anywhere in the package sources — the guard that
    makes a dead-name candidate count only when the symbol is genuinely gone (so a
    no-op edit, or a name the agent legitimately kept, is not falsely scored as drift)."""
    pat = _DECL_RE.format(name=re.escape(name))
    rx = re.compile(pat)
    for p in repo.glob("packages/**/*.ts"):
        try:
            if rx.search(p.read_text(encoding="utf-8", errors="replace")):
                return True
        except OSError:
            continue
    return False


def dead_names(repo: Path, edits: list[dict], upto: int) -> list[str]:
    """Candidates that are GENUINELY absent from the code (declaration gone). A doc
    reference to one of these is a real doc<->code drift; a reference to a still-declared
    symbol is not."""
    return [n for n in dead_candidates(edits, upto) if not code_has_symbol(repo, n)]


def measure(repo: Path, seq: dict, edit_n: int, probe: str = DEFAULT_PROBE) -> dict:
    doc_path = repo / seq["doc"]
    rec: dict = {"edit": edit_n, "doc": seq["doc"]}
    if not doc_path.exists():
        # The doc was deleted/moved by the agent — itself a (severe) drift outcome.
        return {**rec, "doc_exists": False, "n_dangling": None, "n_stale_names": None,
                "drift": True, "note": "arch-doc file absent"}
    text = doc_path.read_text(encoding="utf-8", errors="replace")
    anchors = doc_anchors(text)
    findings = run_probe(anchors, repo, probe)
    dangling = [f["message"] for f in findings if f.get("severity") == "blocking"]
    advisories = [f["code"] for f in findings if f.get("severity") == "advisory"]

    # Prose-honesty grep over TITLES + DESCRIPTIONS only — exclude the
    # `implemented-by:` anchor lines, whose full `path#symbol` correctness the probe
    # already judges (an old name surviving only in a file PATH of a resolving anchor
    # is not a prose drift). This isolates the prose/title blind spot the anchor check
    # misses (e.g. a `### OldName` heading left behind — the pilot's Opus failure).
    prose = "\n".join(l for l in text.splitlines()
                      if not re.match(r"\s*-\s*implemented-by:", l))
    deads = dead_names(repo, seq["edits"], edit_n)
    stale_hits = {}
    for name in deads:
        n = len(re.findall(rf"(?<![\w]){re.escape(name)}(?![\w])", prose))
        if n:
            stale_hits[name] = n

    control_sym = seq.get("control_symbol")
    control_ok = None
    if control_sym:
        # the control anchor must still resolve (a never-edited symbol) — a sanity tripwire
        ctrl = run_probe([seq["control_anchor"]], repo, probe)
        control_ok = len(ctrl) == 0

    return {
        **rec,
        "doc_exists": True,
        "anchors_total": len(anchors),
        "n_dangling": len(dangling),
        "dangling": dangling,
        "advisories": advisories,
        "dead_names_tested": deads,
        "stale_name_hits": stale_hits,
        "n_stale_names": len(stale_hits),
        "control_ok": control_ok,
        "drift": (len(dangling) > 0) or (len(stale_hits) > 0),
    }


# --------------------------------------------------------------------------- selftest
def _selftest() -> int:
    fails = 0
    seq = {
        "doc": "doc.md",
        "edits": [
            {"n": 1, "type": "rename", "old": "Foo", "new": "Bar"},
            {"n": 2, "type": "delete", "old": "Baz", "new": None},
            {"n": 3, "type": "move", "old": "Qux", "new": "Qux"},
            {"n": 4, "type": "rename", "old": "Bar", "new": "Zap"},
        ],
    }
    # dead_candidates: cumulative old of rename/delete, excluding move; compounding captured
    assert dead_candidates(seq["edits"], 1) == ["Foo"], dead_candidates(seq["edits"], 1)
    assert dead_candidates(seq["edits"], 2) == ["Foo", "Baz"]
    assert dead_candidates(seq["edits"], 3) == ["Foo", "Baz"], "move excluded"
    assert dead_candidates(seq["edits"], 4) == ["Foo", "Baz", "Bar"], "compounding old captured"
    # anchor parse
    doc = ("### Foo\n\n<!-- fields -->\n- implemented-by: a/b.ts#Foo\n"
           "### Q\n- implemented-by: a/q.ts#Qux\n")
    assert doc_anchors(doc) == ["a/b.ts#Foo", "a/q.ts#Qux"], doc_anchors(doc)
    # word-bound dead-name grep: "Bar" must not match "Barn" / "ZBar"
    txt = "Bar Barn ZBar Bar."
    assert len(re.findall(r"(?<![\w])Bar(?![\w])", txt)) == 2, "word-bound match"
    # end-to-end against a temp repo with a real .ts file + the probe
    probe = DEFAULT_PROBE
    if Path(probe).exists():
        with tempfile.TemporaryDirectory() as d:
            r = Path(d)
            (r / "a").mkdir()
            (r / "a/b.ts").write_text("export class Bar {}\n")  # Foo was renamed to Bar
            (r / "doc.md").write_text("### Foo\n- implemented-by: a/b.ts#Foo\n")  # anchor stale
            m = measure(r, {**seq, "doc": "doc.md", "control_symbol": None}, 1, probe)
            if m["n_dangling"] != 1:
                print(f"FAIL: expected 1 dangling, got {m['n_dangling']}"); fails += 1
            if m["stale_name_hits"].get("Foo") != 1:  # title "### Foo" only (anchor line excluded)
                print(f"FAIL: expected stale Foo x1, got {m['stale_name_hits']}"); fails += 1
            if not m["drift"]:
                print("FAIL: expected drift=True"); fails += 1
    else:
        print(f"(skipped probe e2e — no probe at {probe})")
    if fails:
        print(f"{fails} failure(s)"); return 1
    print("measure.py selftest OK")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--repo"); ap.add_argument("--seq"); ap.add_argument("--edit", type=int)
    ap.add_argument("--probe", default=DEFAULT_PROBE)
    ap.add_argument("--selftest", action="store_true")
    a = ap.parse_args()
    if a.selftest:
        return _selftest()
    if not (a.repo and a.seq) or a.edit is None:
        ap.error("need --repo --seq --edit (or --selftest)")
    seq = json.loads(Path(a.seq).read_text())
    rec = measure(Path(a.repo), seq, a.edit, a.probe)
    json.dump(rec, sys.stdout, indent=2); sys.stdout.write("\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
