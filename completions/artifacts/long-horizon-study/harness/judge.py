#!/usr/bin/env python3
"""Blind cross-model judge (Codex) over the final arch-doc of each sequence.

Independent corroboration of measure.py's objective oracle, plus the prose-honesty
nuance the anchor check misses. For each sequence it builds a BLINDED record (the final
arch-doc + the list of symbols that ACTUALLY exist in the final code), arm-labels
stripped and the records shuffled under a recorded mapping; Codex judges each in
isolation and returns whether any doc reference names a symbol absent from the code.
The analyst unblinds afterward.

Usage:  judge.py <matrix-dir> [--out judge-results.json] [--limit N]
"""
from __future__ import annotations
import argparse, json, random, re, subprocess, sys
from pathlib import Path

DOC_REL = "docs/architecture/core-public-api.md"
DECL = re.compile(r"export\s+(?:default\s+)?(?:abstract\s+)?(?:class|interface|type|function|const|enum)\s+([A-Za-z_]\w*)")

RUBRIC = """You are auditing one project's architecture document for documentation/code consistency.

You are given (1) the final architecture document and (2) the exhaustive list of symbols that ACTUALLY exist in the code right now. A document is INCONSISTENT if it names — in a component heading, an `implemented-by:` anchor, or prose — any code symbol that is NOT in the actual-symbols list (i.e. a renamed, moved, or removed symbol the docs still reference). The file PATH portion of an anchor is not a symbol; judge the `#Symbol` after the `#`, headings, and prose mentions of class/interface names.

Reply with ONLY a JSON object, no prose around it:
{"consistent": true|false, "stale_refs": ["<each symbol named in the doc but absent from the code>"], "n_stale": <int>, "note": "<one short sentence>"}

=== ACTUAL SYMBOLS IN CODE ===
%s

=== ARCHITECTURE DOCUMENT ===
%s
"""


def code_symbols(repo: Path) -> list[str]:
    syms = set()
    for p in repo.glob("packages/**/*.ts"):
        try:
            for m in DECL.finditer(p.read_text(encoding="utf-8", errors="replace")):
                syms.add(m.group(1))
        except OSError:
            continue
    return sorted(syms)


def collect(matrix: Path) -> list[dict]:
    items = []
    for seqdir in sorted(matrix.glob("*/")):
        repo = seqdir / "repo"
        doc = repo / DOC_REL
        sj = seqdir / "sequence.json"
        if not (doc.exists() and sj.exists()):
            continue
        meta = json.loads(sj.read_text())
        items.append({
            "tag": seqdir.name,
            "arm": meta.get("arm"), "model": meta.get("model"),
            "doc": doc.read_text(encoding="utf-8", errors="replace"),
            "symbols": code_symbols(repo),
        })
    return items


def ask_codex(symbols: list[str], doc: str) -> dict:
    prompt = RUBRIC % ("\n".join(symbols), doc)
    # prompt via stdin (too large/special-char-laden for argv); --skip-git-repo-check
    # because the runs dir is not a git repo (codex otherwise refuses).
    r = subprocess.run(["codex", "exec", "--skip-git-repo-check"],
                       input=prompt, capture_output=True, text=True, timeout=240)
    out = r.stdout
    # codex prints chatter; grab the last {...} JSON object
    matches = re.findall(r"\{.*?\}", out, re.S)
    for cand in reversed(matches):
        try:
            return json.loads(cand)
        except json.JSONDecodeError:
            continue
    return {"error": "no json", "raw_tail": out[-400:]}


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("matrix"); ap.add_argument("--out", default="judge-results.json")
    ap.add_argument("--limit", type=int)
    a = ap.parse_args()
    items = collect(Path(a.matrix))
    if a.limit:
        items = items[:a.limit]
    if not items:
        print("nothing to judge", file=sys.stderr); return 1

    # blind: shuffle + code S01.. (deterministic seed for reproducibility)
    rng = random.Random(20260622)
    order = list(range(len(items)))
    rng.shuffle(order)
    mapping = {}
    results = []
    for i, idx in enumerate(order):
        code = f"S{i+1:02d}"
        it = items[idx]
        mapping[code] = {"tag": it["tag"], "arm": it["arm"], "model": it["model"]}
        print(f"judging {code}  (codex)...", file=sys.stderr)
        verdict = ask_codex(it["symbols"], it["doc"])
        results.append({"code": code, **verdict})
    out = {"mapping": mapping, "verdicts": results}
    Path(a.out).write_text(json.dumps(out, indent=2))
    # quick unblinded summary
    print("\n=== unblinded judge summary ===")
    by_arm = {}
    vmap = {v["code"]: v for v in results}
    for code, m in mapping.items():
        v = vmap[code]
        cons = v.get("consistent")
        by_arm.setdefault((m["arm"], m["model"]), []).append(cons)
        print(f"{code}  {m['arm']:5} {m['model'].split('-')[-1]:7} consistent={cons} n_stale={v.get('n_stale')}")
    print("\narm × model: #consistent / #reps")
    for k, vs in sorted(by_arm.items()):
        print(f"  {k[0]:5} {k[1].split('-')[-1]:7}: {sum(1 for x in vs if x is True)}/{len(vs)}")
    print(f"\nwrote {a.out}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
