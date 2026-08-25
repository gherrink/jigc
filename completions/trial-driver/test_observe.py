"""The reader is pinned against the archive it must reproduce.

`pinning.md`'s discipline, applied to apparatus: the 1.0.0-gate record's channel
table was derived by hand from `evidence/*.jsonl`, corrected once against those
same logs, and is the only independently-established ground truth this reader has.
If a change to `observe.py` moves any cell, that is a finding about the reader —
so the table is asserted cell by cell rather than in aggregate.

Run: python3 completions/trial-driver/test_observe.py
"""
from __future__ import annotations

import pathlib
import sys
import unittest

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))

from driver import channels
from driver.observe import Read, observe, _segments, filesystem_reads

EVIDENCE = (pathlib.Path(__file__).resolve().parents[1]
            / "artifacts" / "RC-1.0-gate" / "evidence")

#: `trial-record.md` §3's table, after its own logged correction to B1.
#: (records, doc-show---task, task-diff/doc-list--task, task-validate, outcome)
RECORD_TABLE = {
    "harborlight":  (26, 3, 0, 2, channels.VERB),
    "pinegrove":   (102, 4, 2, 7, channels.VERB),
    "stonefly":     (94, 6, 0, 5, channels.VERB),
    "rosewater":    (99, 7, 0, 6, channels.VERB),
}


class ReproducesTheArchive(unittest.TestCase):
    """Every cell of the record's table, from the record's own evidence."""

    def setUp(self) -> None:
        if not EVIDENCE.is_dir():
            self.skipTest(f"archived evidence not present at {EVIDENCE}")

    def test_every_cell_of_the_record_table(self) -> None:
        for name, (recs, verb, diff_list, validate, outcome) in RECORD_TABLE.items():
            with self.subTest(session=name):
                o = observe(name,
                            EVIDENCE / f"{name}-invocations.jsonl",
                            EVIDENCE / f"{name}-transcript.jsonl")
                self.assertEqual(o.records, recs, "record count")
                self.assertEqual(o.verb, verb, "doc show … --task")
                self.assertEqual(o.outcome, outcome, "§3.3 outcome")
                # §3.3's VERB-ADJACENT is the record's two columns summed: it
                # counts `task validate` too, and says so beneath the table.
                self.assertEqual(o.adjacent, diff_list + validate,
                                 "§3.3 VERB-ADJACENT = (task diff / doc list --task) + task validate")

    def test_b1_is_the_corrected_figure_not_the_running_note(self) -> None:
        """B1 is 3, not the 5 `session-findings.md` carried for a day.

        The one cell in this table that was ever wrong, pinned by name so a reader
        that drifts back toward the running note fails loudly.
        """
        o = observe("harborlight",
                    EVIDENCE / "harborlight-invocations.jsonl",
                    EVIDENCE / "harborlight-transcript.jsonl")
        self.assertEqual(o.verb, 3)
        self.assertEqual([l.split()[1] for l in o.verb_lines], ["show", "show", "show"])

    def test_no_managed_doc_filesystem_read_beyond_the_dispositioned_one(self) -> None:
        """The record: *"The FILESYSTEM channel did not fire on a managed doc in any session."*

        The heuristic cannot see registration state, so B3b's read of the planted
        **foreign** ADR still matches. That single hit is the declared residue —
        asserted here so a *new* one would fail the suite rather than pass unnoticed.
        """
        got = {}
        for name in RECORD_TABLE:
            o = observe(name, EVIDENCE / f"{name}-invocations.jsonl",
                        EVIDENCE / f"{name}-transcript.jsonl")
            got[name] = [r.detail for r in o.filesystem_reads]
        self.assertEqual(got["harborlight"], [])
        self.assertEqual(got["pinegrove"], [],
                         "B2's .jigc/worktrees/<id>/src reads are code, not managed docs")
        self.assertEqual(got["stonefly"], [])
        self.assertEqual(len(got["rosewater"]), 1, "only the foreign-ADR read")
        self.assertIn("docs/decisions/", got["rosewater"][0])


class TheShippedCounterDisagrees(unittest.TestCase):
    """`run-session.sh:174` is labelled "(VERB-ADJACENT, §3.3)" and is neither.

    It errs in **both** directions, which is why the net gap is not simply the
    `task validate` count:

      * it **misses** `jigc task validate <id>`, which §3.3 lists verbatim;
      * it **counts** a bare `jigc doc list`, where §3.3 requires `doc list --task`
        — a read of the committed index is not a read-back of staged work.

    The 1.0.0-gate record was not misled: it broke `task validate` into its own
    column and said beneath the table that §3.3 counts it too. So this is an
    apparatus wart the humans compensated for by hand, not a corrupted result —
    and the compensation is exactly what the driver should stop needing.
    """

    def setUp(self) -> None:
        if not EVIDENCE.is_dir():
            self.skipTest("archived evidence not present")

    def _counts(self, name):
        from driver.observe import read_log
        recs = read_log(EVIDENCE / f"{name}-invocations.jsonl")
        argvs = [list(r.argv) for r in recs]
        return {
            "validate_missed": sum(1 for a in argvs if tuple(a[:2]) == ("task", "validate")),
            "bare_doc_list_overcounted": sum(
                1 for a in argvs
                if tuple(a[:2]) == ("doc", "list") and not channels._has_task_scope(a)),
        }

    def test_the_gap_decomposes_into_both_errors(self) -> None:
        for name in RECORD_TABLE:
            with self.subTest(session=name):
                o = observe(name, EVIDENCE / f"{name}-invocations.jsonl", None)
                c = self._counts(name)
                self.assertEqual(
                    o.adjacent_counter_gap,
                    c["validate_missed"] - c["bare_doc_list_overcounted"],
                    "net gap = what §3.3 includes and the counter misses, "
                    "minus what the counter includes and §3.3 excludes")

    def test_the_counter_undercounts_every_blind_session(self) -> None:
        gaps = {n: observe(n, EVIDENCE / f"{n}-invocations.jsonl", None).adjacent_counter_gap
                for n in RECORD_TABLE}
        self.assertEqual(gaps, {"harborlight": 2, "pinegrove": 5,
                                "stonefly": 5, "rosewater": 5})
        self.assertTrue(all(g > 0 for g in gaps.values()),
                        "it is wrong in the same direction in all four sessions")


class FalsePositivesStayDead(unittest.TestCase):
    """Each shape that once scored a FILESYSTEM read and must not again."""

    def test_reader_after_a_pipe_reads_stdin(self) -> None:
        progs = [p for p, _ in _segments('find / -name "*.jigc*" | grep -v "/work/.jigc/worktrees"')]
        self.assertEqual(progs, ["find"], "only the head of a pipeline can read a file")

    def test_a_jigc_invocation_is_the_adapter_not_a_bypass(self) -> None:
        progs = [p for p, _ in _segments('jigc start --workflow implement-from-spec "spec docs/specs/x"')]
        self.assertEqual(progs, ["jigc"])

    def test_compound_statements_are_split(self) -> None:
        progs = [p for p, _ in _segments("cd /work && cat docs/x.md; git status")]
        self.assertEqual(progs, ["cd", "cat", "git"])

    def test_code_in_a_fanout_worktree_is_not_a_managed_doc(self) -> None:
        from driver.observe import _looks_managed
        self.assertIsNone(_looks_managed("/work/.jigc/worktrees/some-task/src/store.ts"))
        self.assertIsNotNone(_looks_managed("/work/.jigc/tasks/some-task/docs/adr.md"))


class AuthoringWrites(unittest.TestCase):
    """What counts as having created a read-back occasion."""

    def test_help_is_not_a_write(self) -> None:
        """`jigc doc author --help` matches the verb pair and writes nothing.

        Counting it reported an authoring occasion for a session that had none,
        turning an honest `unmeasured` into a false `NEITHER`. Caught on the first
        live arm this predicate ever ran against.
        """
        from driver.observe import _is_authoring
        self.assertFalse(_is_authoring(["doc", "author", "--help"]))
        self.assertFalse(_is_authoring(["doc", "author", "-h"]))
        self.assertTrue(_is_authoring(["doc", "author", "adr:x", "--task", "t"]))

    def test_reads_and_orientation_are_not_writes(self) -> None:
        from driver.observe import _is_authoring
        for argv in (["doc", "show", "adr:x"], ["doc", "list"], ["start"],
                     ["doc", "schema", "changelog"], ["validate"]):
            self.assertFalse(_is_authoring(argv), argv)

    def test_a_failed_write_is_not_an_occasion(self) -> None:
        """Only a *successful* authoring write creates something to read back."""
        import json, tempfile, pathlib as pl
        with tempfile.TemporaryDirectory() as d:
            log = pl.Path(d) / "invocations.jsonl"
            log.write_text(json.dumps({
                "timestamp": "2026-08-25T10:00:00Z", "argv": ["doc", "create", "adr", "--task", "t"],
                "exit_code": 1, "duration_ms": 5, "finding_codes": [],
                "output_bytes": 10, "binary_version": "1.0.0-rc.11", "error_code": "x"}) + "\n")
            o = observe("t", log, None)
            self.assertEqual(o.authoring_writes, 0)


class IncrementZero(unittest.TestCase):
    """The validity probe's own numbers, from its own archived evidence.

    `increment-0.md` is the record; this is the fence. Both arms are scored end to
    end — reader plus cascade — so a change to either that would restate the
    probe's conclusion fails here instead of quietly rewriting history.
    """

    EV = pathlib.Path(__file__).resolve().parent / "increment-0-evidence"

    def setUp(self) -> None:
        if not self.EV.is_dir():
            self.skipTest("increment-0 evidence not present")

    def test_headless_bypass_reproduces_the_interactive_channel(self) -> None:
        from driver import cascade
        o = observe("b3-bypass", self.EV / "b3-bypass-invocations.jsonl", None)
        self.assertEqual(o.records, 81)
        self.assertEqual(o.verb, 6, "the archive's interactive B3 arms scored 6 and 7")
        self.assertFalse(cascade.grade(o).void)
        self.assertEqual(cascade.grade(o).outcome, "read back through the fence's verb")

    def test_headless_strict_is_unmeasured_not_neither(self) -> None:
        """0 VERB here is a truncated arc, not a channel inversion."""
        from driver import cascade
        o = observe("b3-strict", self.EV / "b3-strict-invocations.jsonl", None)
        self.assertEqual(o.records, 13)
        self.assertEqual(o.authoring_writes, 0, "it never authored anything")
        verdict = cascade.grade(o)
        self.assertTrue(verdict.void)
        self.assertEqual(verdict.outcome, "unmeasured — no authoring occasion existed")

    def test_the_allowlist_was_honoured_under_strict_permissions(self) -> None:
        """B2 refuted: every denial was a file write, no jigc call was denied.

        The plan predicted `-p` would ignore project allow rules, which would have
        made this arm misrepresent the adopter's asymmetry. It did not.
        """
        import json
        results = json.loads((self.EV / "b3-strict-result.json").read_text())
        denials = [d for r in results for d in (r.get("permission_denials") or [])]
        self.assertEqual(len(denials), 7)
        for d in denials:
            inp = d.get("tool_input") or {}
            target = str(inp.get("command") or inp.get("file_path") or "")
            self.assertNotIn("jigc ", target.split(">")[0],
                             "no jigc invocation was denied")
        o = observe("b3-strict", self.EV / "b3-strict-invocations.jsonl", None)
        self.assertEqual(o.records, 13, "all 13 jigc calls executed")


class ChannelPredicates(unittest.TestCase):
    def test_verb_requires_a_task_scope(self) -> None:
        self.assertTrue(channels.is_verb(["doc", "show", "adr:x", "--task", "t"]))
        self.assertTrue(channels.is_verb(["doc", "show", "adr:x", "--task=t"]))
        self.assertFalse(channels.is_verb(["doc", "show", "adr:x"]),
                         "a committed-store read is not a read-back of staged work")

    def test_doc_list_needs_task_but_task_validate_does_not(self) -> None:
        self.assertFalse(channels.is_adjacent(["doc", "list"]))
        self.assertTrue(channels.is_adjacent(["doc", "list", "--task", "t"]))
        self.assertTrue(channels.is_adjacent(["task", "validate", "t"]))

    def test_unmeasured_is_not_neither(self) -> None:
        o = observe("x", pathlib.Path("/nonexistent/log.jsonl"), None)
        self.assertTrue(o.log_missing)
        self.assertEqual(o.outcome, channels.UNMEASURED,
                         "a missing log has not told us the worker declined to read")


if __name__ == "__main__":
    unittest.main(verbosity=2)
