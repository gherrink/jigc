"""The four read-back channels of `RC-1.0-gate/protocol.md` §3.3, as code.

Kept in one module, away from the log reader, because these four predicates are
the *registration* — the thing a protocol pins and a later trial may amend — and
everything else here is plumbing. A channel definition that drifts from its
protocol is not a bug in the reader; it is a different measurement wearing the
same name, which is the failure this module exists to make visible.

§3.3 splits the evidence across two sources, and the split is load-bearing:

  * VERB and VERB-ADJACENT are read from the **invocation log**, a product
    surface with a stable record shape.
  * FILESYSTEM is read from the **session transcript**, which is the only place
    a direct `cat` of a managed doc leaves a trace at all.

`NEITHER` is the residue: the worker proceeded without reading. It is a scored
outcome, never an absence — §3.3 keeps `unmeasured` separate on purpose, and so
does `session_outcome`.
"""
from __future__ import annotations

#: §3.3's names for the four channels, as a shared vocabulary for prose and
#: messages. **Nothing scores with them.** Scoring is `cascade.grade()`, in one
#: list, in the registered order — an earlier version of this package also carried
#: an `Observation.outcome` property that re-implemented that ordering privately
#: and disagreed with the cascade on the first live session it met.
VERB = "VERB"
VERB_ADJACENT = "VERB-ADJACENT"
FILESYSTEM = "FILESYSTEM"
NEITHER = "NEITHER"
UNMEASURED = "unmeasured"

#: Verbs §3.3 names as a staged read-back through jigc that the M48 fence does
#: *not* name. Each entry is (argv prefix, whether a --task scope is required).
#:
#: `task validate` is here because §3.3 lists it verbatim. The counter shipped in
#: `trial-harness/run-session.sh:174` labels itself "(VERB-ADJACENT, §3.3)" and
#: greps only `task diff` and `doc list` — so on the archived `harborlight` log
#: it reports 0 where §3.3 scores 2. The disagreement is reported by
#: `adjacent_counter_gap()` rather than silently resolved: which of the two is
#: right is a protocol question, not a reader question.
ADJACENT_VERBS: tuple[tuple[tuple[str, ...], bool], ...] = (
    (("task", "diff"), False),
    (("doc", "list"), True),
    (("task", "validate"), False),
)

#: What the shipped `run-session.sh` counter actually matches, kept so the gap
#: between the two can be measured instead of argued about.
SHIPPED_ADJACENT_VERBS: tuple[tuple[str, ...], ...] = (
    ("task", "diff"),
    ("doc", "list"),
)


#: jigc's global options, from `jigc --help`. `--format` is the only one taking a
#: VALUE, which is the whole reason this needs enumerating rather than a blanket
#: "skip leading dashes".
GLOBAL_VALUE_FLAGS: frozenset[str] = frozenset(("--format",))


def verb_tokens(argv: list[str]) -> list[str]:
    """`argv` with any leading global options removed, so the verb is at index 0.

    **A silent-undercount fix, found by review rather than by a failing test.**
    clap propagates jigc's global options, so `jigc --format json doc show <addr>
    --task <id>` is accepted and the invocation log records it verbatim:

        {"argv": ["--format","json","doc","show","adr:x","--task","t"], …}

    Every predicate here used to match `argv[:2]`, so that invocation was invisible
    to VERB, VERB-ADJACENT *and* the authoring count — a worker reading its staged
    work back in JSON would have scored as never having read it back at all, which
    is a wrong number in the direction that most flatters a null.

    The archived 1.0.0-gate logs happen to contain only four flag-first records and
    all four are `--version` or `--help`, so no published figure moves. That is
    luck, not design: `--format json` is the machine-output contract the driver
    contract itself encourages.
    """
    i = 0
    while i < len(argv) and argv[i].startswith("-"):
        name = argv[i].split("=", 1)[0]
        # `--format json` consumes the next token; `--format=json` does not.
        i += 2 if (name in GLOBAL_VALUE_FLAGS and "=" not in argv[i]) else 1
    return list(argv[i:])


def _has_task_scope(argv: list[str]) -> bool:
    """Whether this invocation names a task, i.e. reads *staged* work.

    `--task <id>` and `--task=<id>` both count. A bare `jigc doc show <addr>` is
    a committed-store read and is not a read-back of the worker's own in-flight
    work, which is the thing §3 measures.
    """
    return any(a == "--task" or a.startswith("--task=") for a in argv)


def is_verb(argv: list[str]) -> bool:
    """§3.3 VERB: `jigc doc show … --task …`."""
    v = verb_tokens(argv)
    return tuple(v[:2]) == ("doc", "show") and _has_task_scope(v)


def is_adjacent(argv: list[str]) -> bool:
    """§3.3 VERB-ADJACENT: a staged read-back through a verb the fence omits."""
    v = verb_tokens(argv)
    for prefix, needs_task in ADJACENT_VERBS:
        if tuple(v[: len(prefix)]) == prefix:
            if needs_task and not _has_task_scope(v):
                continue
            return True
    return False


def is_shipped_adjacent(argv: list[str]) -> bool:
    """What `run-session.sh:174` counts under the same §3.3 label."""
    # Deliberately NOT normalised: this models what `run-session.sh:174`'s grep
    # actually matches, and that grep sees the raw JSON line. Normalising here
    # would make the shipped counter look better than it is.
    return any(tuple(argv[: len(p)]) == p for p in SHIPPED_ADJACENT_VERBS)
