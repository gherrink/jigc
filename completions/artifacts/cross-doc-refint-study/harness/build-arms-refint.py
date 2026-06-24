#!/usr/bin/env python3
"""Build the cross-doc study's dilution-ladder rules files (C40 / C160 / C550).

The dilution ladder reuses the doc<->code study's REAL-OSS-DERIVED bulk (arms/C160,
arms/C550 under ~/lh-study) as genuine dilution, swapping ONLY:

  (1) the rule section body -> the byte-identical CROSS-REFERENCE INTEGRITY rule (below);
  (2) one intro sentence -> establishes the decision records the rule governs.

The RULE BODY is byte-identical across C40/C160/C550 (only the surrounding file grows and
the rule's heading-number/placement differ). C40 is purpose-built (small, prominent); C160
and C550 are the reused ladder with the section swap. Honest bound (recorded in the
pre-registration amendments): the cross-doc twin contains only the `docs/` subtree, so the
surrounding engineering guidance is NON-ACTIONABLE dilution for the decision-record tickets
— exactly the attention-competition the ladder measures, not a strawman.

Usage: build-arms-refint.py <lh-study-arms-dir> <out-dir>
  reads <arms>/C160-CLAUDE.md, <arms>/C550-CLAUDE.md
  writes <out>/C40-CLAUDE.md, <out>/C160-CLAUDE.md, <out>/C550-CLAUDE.md
"""
from __future__ import annotations
import re, sys
from pathlib import Path

# ----- the byte-identical rule BODY (everything after the section heading line) -----
RULE_BODY = """\
This project keeps a set of **decision records** under `docs/decisions/` (one Architecture
Decision Record per file) and a **`docs/architecture/`** overview that ties them together.
These documents **reference each other**, and the references must always resolve:

- An ADR may record that it **supersedes** an earlier decision, via a
  `supersedes: adr:<slug>` field in its front-matter.
- The architecture overview **cites** the decisions that govern the system, via a
  `cites: [adr:<slug>, …]` field in its front-matter.

Each `adr:<slug>` names another decision file (`docs/decisions/<slug>.md`). A reference that
names a decision which no longer exists is a defect — treat a dangling cross-reference with
the same seriousness as a failing test.

### The rule

Whenever you **delete, rename, split, or otherwise restructure any decision record**, keep
**every cross-reference across all documents valid** — no `supersedes:` and no `cites:`
entry, in any file, may point at a decision slug that no longer exists. **This includes
references in documents you were not asked to touch.**

### How to be sure

After any such change, **search the whole `docs/` tree for the old slug** and confirm no
`supersedes:` or `cites:` field anywhere still names a decision that is now absent. Update or
remove every reference you find. A cross-reference that does not resolve to an existing
decision is the failure mode this discipline exists to prevent.
"""

RULE_TITLE = "Cross-reference integrity discipline"

# one intro sentence establishing the records (appended to the first paragraph)
INTRO_SENTENCE = (
    " Among other things, this repository keeps its architectural **decision records** under "
    "`docs/decisions/` and an architecture overview under `docs/architecture/`; the "
    "cross-reference integrity discipline below is load-bearing."
)

C40 = f"""\
# Project working agreement

This repository keeps its architectural **decision records** (ADRs) under
`docs/decisions/`, one per file, plus an architecture overview under
`docs/architecture/` that ties them together. Read this file before changing
any decision record.

## {RULE_TITLE}

{RULE_BODY}"""


def swap_section(text: str, out_heading_word: str) -> str:
    """Replace the `## N. Documentation-consistency discipline` section (heading + body,
    up to the next `## ` heading) with `## N. <RULE_TITLE>` + the byte-identical RULE_BODY,
    preserving the original section NUMBER N (placement is the ladder's variable)."""
    m = re.search(r"^## (\d+[a-z]?)\. Documentation-consistency discipline.*?(?=^## )",
                  text, re.M | re.S)
    if not m:
        raise SystemExit(f"rule section not found ({out_heading_word})")
    num = m.group(1)
    replacement = f"## {num}. {RULE_TITLE}\n\n{RULE_BODY}\n"
    return text[:m.start()] + replacement + text[m.end():]


def add_intro_sentence(text: str) -> str:
    """Append the decision-records sentence to the end of the first paragraph (the lines
    after the H1 up to the first blank line)."""
    lines = text.splitlines(keepends=True)
    # find H1, then the end of the first non-empty paragraph after it
    i = 0
    while i < len(lines) and not lines[i].startswith("# "):
        i += 1
    i += 1  # past H1
    while i < len(lines) and lines[i].strip() == "":
        i += 1
    # i = start of first paragraph; advance to its last line
    while i < len(lines) and lines[i].strip() != "":
        i += 1
    # i-1 is the paragraph's last line; splice the sentence onto it
    j = i - 1
    lines[j] = lines[j].rstrip("\n") + INTRO_SENTENCE + "\n"
    return "".join(lines)


def main() -> int:
    if len(sys.argv) != 3:
        raise SystemExit(__doc__)
    arms, out = Path(sys.argv[1]), Path(sys.argv[2])
    out.mkdir(parents=True, exist_ok=True)

    (out / "C40-CLAUDE.md").write_text(C40)

    for arm in ("C160", "C550"):
        src = (arms / f"{arm}-CLAUDE.md").read_text()
        src = add_intro_sentence(src)
        src = swap_section(src, arm)
        (out / f"{arm}-CLAUDE.md").write_text(src)

    # verify the rule body is byte-identical across all three
    bodies = []
    for arm in ("C40", "C160", "C550"):
        t = (out / f"{arm}-CLAUDE.md").read_text()
        mm = re.search(re.escape(RULE_BODY.rstrip()), t)
        bodies.append(bool(mm))
    if not all(bodies):
        raise SystemExit(f"rule body not byte-identical across arms: {bodies}")
    sizes = {arm: len((out / f'{arm}-CLAUDE.md').read_text().splitlines())
             for arm in ("C40", "C160", "C550")}
    print(f"built ladder (lines): {sizes}; rule body byte-identical across all three: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
