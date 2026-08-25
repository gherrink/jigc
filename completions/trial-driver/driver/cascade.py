"""Scored outcomes, first match wins — and no "did not fire" branch.

This is `cue-card-postmortem.md` §8 rule 3 made structural:

    Enumerate the branches, and refuse a design with a silent one. For every
    plausible worker behaviour — including *does nothing*, *discards*, *routes
    around* — write down what gets recorded. If any branch records **nothing**,
    the instrument can return no data, and a null is unreadable. The instrument
    must have no "did not fire" branch, only outcomes.

Two properties do the work, and both were bought by someone else's defect:

**One list, and the first match wins.** There is no second place a row can be
applied, because there is no *afterwards*. The eval rig this is modelled on grew
that shape after a void row was outranked by the row above it.

**Apparatus failure is a `void`, never a score.** A dead CLI, a missing log, a
fork that silently began cold — these say nothing about jigc, and folding them
into the denominator protects the product from its most important failures. A
voided run is *data*: it is recorded, counted and reported separately.

The cascade is data, not control flow, so a trial declares its own and the engine
stays ignorant of any particular one. `DEFAULT_CASCADE` is the session-level one
every RC arm needs; a trial with a registered outcome table of its own passes it
to `grade()` instead and pins the correspondence with `check_registration()`.
"""
from __future__ import annotations

import dataclasses
import re
from typing import Callable, Optional, Sequence

from .observe import Observation


@dataclasses.dataclass(frozen=True)
class Rule:
    """One row. `void` marks it an apparatus failure rather than a result.

    `example` is an Observation that must reach this row through the whole engine.
    A row shadowed by an earlier one then fails loudly instead of sitting in the
    table looking implemented — the defect `check_examples()` exists to catch.
    """
    n: int
    outcome: str
    void: bool
    when: Callable[[Observation], bool]
    example: Observation


def _obs(**kw) -> Observation:
    """An Observation with everything healthy, overridden by `kw`.

    Defaults matter: an example that accidentally left `log_missing` true would
    be caught by row 2 and would prove nothing about the row it was written for.
    """
    base = dict(
        session="example", records=10, authoring_writes=1, verb=0, adjacent=0, shipped_adjacent=0,
        filesystem=0, log_missing=False, log_unreadable=False,
        transcript_missing=False, binary_versions=("1.0.0-rc.11",),
        nonzero_exits=0, error_codes=(), verb_lines=(), adjacent_lines=(),
        filesystem_reads=(),
    )
    base.update(kw)
    return Observation(**base)


#: Session-level outcomes for an RC arm, in registered order.
#:
#: Rows 1–4 are apparatus. Row 3 is the one that matters most and is the hardest
#: to see: `--resume` on a session the CLI has never seen does **not** fail — it
#: starts a fresh conversation. Every other channel looks identical, so without
#: this row a dead fixture yields a plausible arm.
DEFAULT_CASCADE: tuple[Rule, ...] = (
    Rule(1, "apparatus — the session did not run", True,
         lambda o: o.rc_failed,
         # A dead CLI usually also leaves no log, so this row must outrank row 2:
         # "the session did not run" is the more informative of the two true
         # statements. The example therefore carries BOTH, and would be caught by
         # row 2 if the order were ever reversed.
         _obs(rc_failed=True, records=0, log_missing=True)),
    Rule(2, "apparatus — no invocation log", True,
         lambda o: o.log_missing,
         _obs(log_missing=True)),
    Rule(3, "apparatus — invocation log unreadable", True,
         lambda o: o.log_unreadable,
         _obs(log_unreadable=True)),
    Rule(4, "apparatus — the fork began cold", True,
         lambda o: o.seed_expected and not o.seed_inherited,
         _obs(seed_expected=True, seed_inherited=False)),
    Rule(5, "apparatus — the session did nothing", True,
         lambda o: o.records == 0,
         _obs(records=0)),
    # §3.3, verbatim: "The session never reached an authoring step — no occasion
    # existed; unmeasured, not NEITHER." Added after the first live headless arm
    # truncated before authoring anything and would have been scored as a worker
    # that declined to read. It must outrank every scored row, because a session
    # with nothing staged cannot have read its staged work back.
    Rule(6, "unmeasured — no authoring occasion existed", True,
         lambda o: o.authoring_writes == 0,
         _obs(authoring_writes=0)),
    Rule(7, "read back through the fence's verb", False,
         lambda o: o.verb > 0,
         _obs(authoring_writes=1, verb=2)),
    Rule(8, "read back through jigc, by another verb", False,
         lambda o: o.adjacent > 0,
         _obs(authoring_writes=1, adjacent=1)),
    # The only row scored from the transcript, and the only one whose evidence is a
    # heuristic. `observe.filesystem_reads` returns what it matched precisely so a
    # human can check the call, and the outcome NAME carries that — a row title is
    # what lands in a trial record, and "read around jigc" stated flatly would give
    # a heuristic the authority of the two channels read from the invocation log.
    # `Observation.filesystem_is_heuristic` exists to make this un-forgettable; this
    # is the thing that reads it.
    Rule(9, "read around jigc, off the filesystem (heuristic — verify the reads)",
         False,
         lambda o: o.filesystem > 0,
         _obs(authoring_writes=1, filesystem=1)),
    # A NEITHER verdict is the claim "the worker read nothing, anywhere". That
    # requires BOTH channels to have been looked at, and the FILESYSTEM channel
    # lives only in the transcript. With no transcript, `filesystem` is 0 because
    # nothing was read, not because nothing happened — so the run would otherwise
    # reach row 11 and assert something no evidence supports.
    #
    # Placed AFTER the three positive rows on purpose: a session with a VERB result
    # is fully measured on the channel that carries the claim, and voiding it for a
    # missing transcript would throw away a good measurement.
    Rule(10, "unmeasured — no transcript, so the filesystem channel is blind", True,
         lambda o: o.transcript_missing,
         _obs(authoring_writes=1, transcript_missing=True)),
    Rule(11, "proceeded without reading", False,
         lambda o: True,
         _obs(authoring_writes=1)),
)


def grade(observation: Observation,
          cascade: Sequence[Rule] = DEFAULT_CASCADE) -> Rule:
    """The first row whose predicate holds. Raises if none does.

    Raising is deliberate. A cascade that silently returned None would reintroduce
    the branch this module exists to forbid.
    """
    for rule in cascade:
        if rule.when(observation):
            return rule
    raise RuntimeError(
        f"no row of the cascade matched {observation.session!r}. A cascade whose "
        "last row is not a catch-all has a silent branch, which is the one thing "
        "it may not have."
    )


def check_examples(cascade: Sequence[Rule] = DEFAULT_CASCADE) -> list[str]:
    """Every row's `example` must reach that row. Returns the failures."""
    bad = []
    for rule in cascade:
        got = grade(rule.example, cascade)
        if got.n != rule.n:
            bad.append(f"row {rule.n} ({rule.outcome!r}) is shadowed by "
                       f"row {got.n} ({got.outcome!r})")
    return bad


def check_shape(cascade: Sequence[Rule] = DEFAULT_CASCADE) -> list[str]:
    """Structural invariants a cascade must satisfy before it is used."""
    bad = []
    if [r.n for r in cascade] != list(range(1, len(cascade) + 1)):
        bad.append("rows are not numbered 1..n in order")
    outcomes = [r.outcome for r in cascade]
    if len(set(outcomes)) != len(outcomes):
        bad.append("two rows share an outcome name")
    if not cascade or not cascade[-1].when(_obs()):
        bad.append("the last row is not a catch-all — the cascade has a silent branch")
    return bad


_ROW = re.compile(r"^\|\s*(\d+)\s*\|\s*`([^`]+)`\s*\|\s*(void|score)\s*\|", re.M)


def check_registration(protocol_text: str,
                       cascade: Sequence[Rule] = DEFAULT_CASCADE) -> list[str]:
    """The registered table and the code must agree — order included.

    Order is compared, not just membership: a set comparison stays green while two
    rows are implemented the wrong way round, which is exactly how a void row came
    to be outranked in the rig this is modelled on.

    Expected table shape, in the trial's own protocol:

        | 1 | `apparatus — the session did not run` | void |
        | 6 | `read back through the fence's verb`  | score |
    """
    # Fenced blocks are stripped first. The regex scans lines, not Markdown, so an
    # exact copy of an old table inside a ```example``` fence was indistinguishable
    # from the live one — and would certify agreement while the operative table said
    # something else, or had been deleted entirely.
    body, fenced, out = protocol_text, False, []
    for line in body.splitlines():
        if line.lstrip().startswith("```"):
            fenced = not fenced
            continue
        if not fenced:
            out.append(line)
    registered = [(int(n), outcome, kind == "void")
                  for n, outcome, kind in _ROW.findall("\n".join(out))]
    if not registered:
        return ["the protocol registers no outcome table (no matching rows found)"]
    coded = [(r.n, r.outcome, r.void) for r in cascade]
    if registered == coded:
        return []
    bad = [f"registered {len(registered)} rows, code has {len(coded)}"] \
        if len(registered) != len(coded) else []
    for i, (reg, cod) in enumerate(zip(registered, coded), 1):
        if reg != cod:
            bad.append(f"position {i}: protocol {reg} != code {cod}")
    return bad
