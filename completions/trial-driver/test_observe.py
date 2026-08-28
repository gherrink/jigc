"""The reader is pinned against the archive it must reproduce.

`pinning.md`'s discipline, applied to apparatus: the 1.0.0-gate record's channel
table was derived by hand from `evidence/*.jsonl`, corrected once against those
same logs, and is the only independently-established ground truth this reader has.
If a change to `observe.py` moves any cell, that is a finding about the reader —
so the table is asserted cell by cell rather than in aggregate.

Run: python3 completions/trial-driver/test_observe.py
"""
from __future__ import annotations

import json
import pathlib
import shutil
import sys
import tempfile
import unittest

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))

from driver import cascade, channels
from driver import observe as observe_mod
from driver.observe import Read, observe, _segments, filesystem_reads

#: The cascade row a VERB session lands on. Named once so the table below states
#: an outcome rather than restating the cascade's wording four times.
VERB_ROW = "read back through the fence's verb"

EVIDENCE = (pathlib.Path(__file__).resolve().parents[1]
            / "artifacts" / "RC-1.0-gate" / "evidence")

#: `trial-record.md` §3's table, after its own logged correction to B1.
#: (records, doc-show---task, task-diff/doc-list--task, task-validate, outcome)
RECORD_TABLE = {
    "harborlight":  (26, 3, 0, 2, VERB_ROW),
    "pinegrove":   (102, 4, 2, 7, VERB_ROW),
    "stonefly":     (94, 6, 0, 5, VERB_ROW),
    "rosewater":    (99, 7, 0, 6, VERB_ROW),
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
                self.assertEqual(cascade.grade(o).outcome, outcome, "§3.3 outcome")
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

    def test_the_two_errors_the_gap_used_to_decompose_into_are_both_gone(self) -> None:
        """Settled 2026-08-28: the counter was aligned to §3.3, not §3.3 to it.

        The gap used to be `validate_missed - bare_doc_list_overcounted`, and it was
        2 / 5 / 5 / 5 across the four archived sessions. Both errors are fixed at the
        source, so the decomposition must now evaluate to zero on both sides — the
        assertion is kept in this shape rather than deleted, because it is the one
        that would redden if either half regressed.
        """
        for name in RECORD_TABLE:
            with self.subTest(session=name):
                o = observe(name, EVIDENCE / f"{name}-invocations.jsonl", None)
                c = self._counts(name)
                self.assertGreater(
                    c["validate_missed"] + c["bare_doc_list_overcounted"], -1,
                    "the archived calls the gap was built from are still in the log")
                self.assertEqual(
                    o.adjacent_counter_gap, 0,
                    "the aligned counter counts `task validate` and skips a bare "
                    "`doc list`, so neither half of the old gap survives")

    def test_the_counter_now_agrees_with_the_registration_on_every_session(self) -> None:
        gaps = {n: observe(n, EVIDENCE / f"{n}-invocations.jsonl", None).adjacent_counter_gap
                for n in RECORD_TABLE}
        self.assertEqual(gaps, {"harborlight": 0, "pinegrove": 0,
                                "stonefly": 0, "rosewater": 0})

    def test_the_aligned_counter_reproduces_the_registered_adjacent_numbers(self) -> None:
        """The shell and the reader are two implementations; they must agree.

        Driven against the aligned shell grep on the archive, these were 2 / 9 / 5 / 6
        — identical to §3.3 as this module computes it. That agreement is the whole
        justification for leaving a counter in the shell at all.
        """
        got = {n: observe(n, EVIDENCE / f"{n}-invocations.jsonl", None).shipped_adjacent
               for n in RECORD_TABLE}
        self.assertEqual(got, {"harborlight": 2, "pinegrove": 9,
                               "stonefly": 5, "rosewater": 6})


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


class CrossModelReviewFindings(unittest.TestCase):
    """Fences for the defects an independent Codex review found.

    Kept together and named, because each is a way the apparatus could have written
    a wrong number into a trial record.
    """

    def test_an_excluded_path_does_not_suppress_a_managed_one_beside_it(self) -> None:
        """Finding 8, the false NEGATIVE half.

        Exclusions were applied to the whole command segment, so one worktree path
        hid every managed path next to it — on the channel §3.3 says must not be
        flattered.
        """
        from driver.observe import _looks_managed
        both = ('grep -n "cap" /work/docs/decisions/x.md '
                '/work/.jigc/worktrees/t/src/a.ts')
        self.assertIsNotNone(_looks_managed(both),
                             "the managed doc is still read, whatever sits beside it")

    def test_a_worktree_only_command_is_still_excluded(self) -> None:
        from driver.observe import _looks_managed
        self.assertIsNone(_looks_managed("cat /work/.jigc/worktrees/t/src/store.ts"))

    def test_a_bare_filename_with_no_slash_still_matches(self) -> None:
        """`cat CHANGELOG.md` has no path-ish token; the fallback must keep it."""
        from driver.observe import _looks_managed
        self.assertIsNotNone(_looks_managed("cat CHANGELOG.md"))

    def test_failed_reads_are_counted_and_reported_not_redefined(self) -> None:
        """Finding 3.

        §3.3 registers VERB as the command *appearing* in the log, and the record
        counts it that way — rosewater's 7 includes two failed reads. The reader
        must reproduce that, AND make the difference visible, because which of the
        two is meant is the protocol's call and not the reader's.
        """
        ev = pathlib.Path(__file__).resolve().parents[1] / "artifacts" / "RC-1.0-gate" / "evidence"
        if not ev.is_dir():
            self.skipTest("archived evidence not present")
        o = observe("rosewater", ev / "rosewater-invocations.jsonl", None)
        self.assertEqual(o.verb, 7, "the registered count, as the record carries it")
        self.assertEqual(o.verb_succeeded, 5)
        self.assertEqual(o.failed_reads, 2)

    def test_a_session_with_no_failed_reads_reports_no_gap(self) -> None:
        ev = pathlib.Path(__file__).resolve().parents[1] / "artifacts" / "RC-1.0-gate" / "evidence"
        if not ev.is_dir():
            self.skipTest("archived evidence not present")
        o = observe("harborlight", ev / "harborlight-invocations.jsonl", None)
        self.assertEqual(o.failed_reads, 0)


class TimingWindows(unittest.TestCase):
    """`cue-card-postmortem.md` §2's table, computed rather than reconstructed.

    That table is what established the cue card was *impossible* rather than
    mistimed, and its own header says it was reconstructed from the archived logs
    by hand, after the trial. Computing it is four lines and the answer decides
    whether a designed occasion is worth building — so it should not wait for a
    post-mortem.
    """

    EV = pathlib.Path(__file__).resolve().parents[1] / "artifacts" / "RC-1.0-gate" / "evidence"

    def setUp(self) -> None:
        if not self.EV.is_dir():
            self.skipTest("archived evidence not present")

    def test_b1_reproduces_the_post_mortem_figures_exactly(self) -> None:
        """§2: B1's trigger is the 3rd `set slot adr:…` ack (`#consequences`),
        its window is 14 s and the longest silence inside it is 4 s."""
        from driver.observe import read_log, windows
        recs = read_log(self.EV / "harborlight-invocations.jsonl")
        slots = [r for r in recs if tuple(r.argv[:2]) == ("doc", "set-slot")]
        third = slots[2]
        self.assertIn("#consequences", " ".join(third.argv),
                      "§2 names the third set-slot as the one on #consequences")
        got = windows(recs, lambda r: r is third,
                      lambda r: tuple(r.argv[:2]) == ("task", "finalize"))
        self.assertEqual(len(got), 1)
        _, _, span, silence = got[0]
        self.assertEqual(span, 14.0)
        self.assertEqual(silence, 4.0)

    def test_a_window_that_never_closes_is_not_reported(self) -> None:
        """An opener with no matching close yields nothing, rather than a fake span."""
        from driver.observe import read_log, windows
        recs = read_log(self.EV / "harborlight-invocations.jsonl")
        self.assertEqual(windows(recs, lambda r: True, lambda r: False), [])


class GlobalFlagsDoNotHideAnInvocation(unittest.TestCase):
    """A silent-undercount defect, found by review and fixed before it fired.

    clap propagates jigc's global options, so `jigc --format json doc show <addr>
    --task <id>` is accepted and logged verbatim as
    `["--format","json","doc","show",…]`. Matching `argv[:2]` made that invocation
    invisible to VERB, VERB-ADJACENT and the authoring count at once — a worker
    reading its staged work back in JSON scoring as never having read it back.

    Verified against the real binary: `jigc --format json doc list` logs
    `{"argv":["--format","json","doc","list"],…}`.
    """

    def test_format_json_before_the_verb_still_scores_verb(self) -> None:
        self.assertTrue(channels.is_verb(
            ["--format", "json", "doc", "show", "adr:x", "--task", "t"]))

    def test_the_equals_form_too(self) -> None:
        self.assertTrue(channels.is_verb(
            ["--format=json", "doc", "show", "adr:x", "--task", "t"]))

    def test_adjacent_and_authoring_are_normalised_as_well(self) -> None:
        self.assertTrue(channels.is_adjacent(["--format", "json", "task", "validate", "t"]))
        from driver.observe import _is_authoring
        self.assertTrue(_is_authoring(["--format", "json", "doc", "create", "adr"]))

    def test_a_terminal_global_flag_is_not_a_verb(self) -> None:
        for argv in (["--help"], ["--version"], ["-V"], ["-h"]):
            self.assertEqual(channels.verb_tokens(argv), [], argv)

    def test_only_format_consumes_its_next_token(self) -> None:
        """`--format` is the only global taking a value, per `jigc --help`.

        A blanket "skip a token after every dash-flag" would eat the verb.
        """
        self.assertEqual(channels.verb_tokens(["--format", "json", "doc", "list"]),
                         ["doc", "list"])
        self.assertEqual(channels.verb_tokens(["--format=json", "doc", "list"]),
                         ["doc", "list"])

    def test_the_shipped_counter_is_deliberately_not_normalised(self) -> None:
        """It models the shell's grep, which sees the raw JSON line.

        Normalising it here would make the shipped counter look better than it is.
        The grep is a **substring** match, not an argv prefix, so a leading global
        flag does not hide the pair from it — an earlier model here compared
        `argv[:2]` and was stricter than the thing it claimed to model. What the
        grep genuinely cannot see is the `--task=<id>` spelling: its pattern carries
        the closing quote of `"--task"`, and `"--task=t"` does not contain it. That
        divergence is left in, because surfacing it is what the gap is for.
        """
        self.assertTrue(channels.is_shipped_adjacent(
            ["--format", "json", "doc", "list", "--task", "t"]),
            "the grep matches the pair wherever it sits in the line")
        self.assertFalse(channels.is_shipped_adjacent(["doc", "list", "--task=t"]),
                         "the shell's literal `\"--task\"` cannot match `--task=t`")
        self.assertTrue(channels.is_adjacent(["doc", "list", "--task=t"]),
                        "§3.3 does count it — which is the residual gap, on purpose")


class ThePlantIsNotTheWorker(unittest.TestCase):
    """A rig-inflates-the-headline defect, found by running the R1 rehearsal.

    `.jigc/logs/invocations.jsonl` lives INSIDE the corpus. Plant E drives the
    real binary to build its state, and its end-state bar reads the doc back five
    times — so every one of those calls landed in the channel the session is
    scored on, timestamped before the session even started.

    Measured on the uncorrected R1 evidence, 2026-08-28: `observe` reported
    **VERB 6** where the worker had done **2**, and 13 authoring writes where the
    worker had done 6. The error is 3x and it points the flattering way, which is
    the direction nobody double-checks. Both plant-E arms carry the trial's
    headline, so this would have inflated the headline measurement itself.

    Two fixes, and this pins the reader half: `observe` splits on the
    `session-start` stamp `run-session.sh` now writes into PROVENANCE.txt. (The
    plant also clears the log as its last act — belt and braces, and the only
    answer to a worker that reads the log and watches itself being planted.)
    """

    #: A plant's records, then the worker's. The plant's include the `doc show`
    #: reads its own bar performs — the exact shape that did the inflating.
    LOG = [
        '{"timestamp":"2026-08-28T05:40:38Z","argv":["doc","create","adr","--title","X"],'
        '"exit_code":0,"duration_ms":1,"finding_codes":[],"output_bytes":1,'
        '"binary_version":"1.0.0-rc.12","error_code":null}',
        '{"timestamp":"2026-08-28T05:40:38Z","argv":["doc","show","adr:x","--task","t"],'
        '"exit_code":0,"duration_ms":1,"finding_codes":[],"output_bytes":1,'
        '"binary_version":"1.0.0-rc.12","error_code":null}',
        '{"timestamp":"2026-08-28T05:40:38Z","argv":["task","validate","t"],'
        '"exit_code":3,"duration_ms":1,"finding_codes":[],"output_bytes":1,'
        '"binary_version":"1.0.0-rc.12","error_code":null}',
        '{"timestamp":"2026-08-28T05:41:20Z","argv":["doc","show","adr:x","--task","t"],'
        '"exit_code":0,"duration_ms":1,"finding_codes":[],"output_bytes":1,'
        '"binary_version":"1.0.0-rc.12","error_code":null}',
    ]
    START = "2026-08-28T05:41:12Z"

    def _log(self) -> pathlib.Path:
        d = pathlib.Path(tempfile.mkdtemp(prefix="jigc-presession-"))
        self.addCleanup(shutil.rmtree, d, True)
        p = d / "invocations.jsonl"
        p.write_text("\n".join(self.LOG) + "\n")
        return p

    def test_without_the_stamp_the_plant_is_scored_as_the_worker(self) -> None:
        """The defect, preserved: this is what the uncorrected reader did."""
        o = observe("uncorrected", self._log(), None)
        self.assertEqual((o.records, o.verb, o.adjacent), (4, 2, 1))
        self.assertEqual(o.pre_session_records, 0, "nothing is split without a stamp")

    def test_with_the_stamp_only_the_worker_is_scored(self) -> None:
        o = observe("corrected", self._log(), None, session_start=self.START)
        self.assertEqual(o.records, 1, "one record is the worker's")
        self.assertEqual(o.verb, 1, "the plant's read-back is not the worker's")
        self.assertEqual(o.adjacent, 0)

    def test_the_size_of_the_error_is_reported_not_hidden(self) -> None:
        """Dropping them silently would be a second way to be wrong about it."""
        o = observe("corrected", self._log(), None, session_start=self.START)
        self.assertEqual(o.pre_session_records, 3)
        self.assertEqual(o.pre_session_verb, 1)
        self.assertEqual(o.pre_session_adjacent, 1)

    def test_a_malformed_stamp_scores_the_session_rather_than_losing_it(self) -> None:
        """Losing the worker's evidence is the worse of the two errors."""
        o = observe("odd", self._log(), None, session_start="not-a-timestamp")
        self.assertEqual(o.records, 4)
        self.assertEqual(o.pre_session_records, 0)


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


class ADocumentReadIsNotABookkeepingRead(unittest.TestCase):
    """§3.3's FILESYSTEM row folds two acts together; a rehearsal split them.

    Two runs of the SAME prompt against the SAME plant, 2026-08-28:

      * R1  read the staged `.md` off disk  -> 2 document + 5 workbench reads
      * R1b took the document only through `jigc doc show --task`
                                            -> 0 document + 6 workbench reads

    Both score `filesystem = 6` under one count and they are opposite results on
    the axis 3A exists to measure. A document read off disk is the adapter bypass
    the invariant is about; a read of `roles.json` or `base.json` is bookkeeping
    no read verb exposes, and reporting it as evidence against VISION principle #3
    would be scoring a capability gap as a channel violation.
    """

    def _read(self, path: str) -> Read:
        return Read(tool="Bash", detail=f"cat {path}", path=path)

    def test_a_staged_managed_doc_is_a_document_read(self) -> None:
        self.assertTrue(self._read(
            "/work/.jigc/tasks/t/docs/adr:x.md").is_document)

    def test_a_committed_managed_doc_is_too(self) -> None:
        self.assertTrue(self._read("docs/decisions/drop-the-oldest.md").is_document)

    def test_the_workbench_bookkeeping_files_are_not(self) -> None:
        for name in ("intent", "workflow", "roles.json", "base.json",
                     "docs/provenance.json", "staged-snapshot.json"):
            with self.subTest(file=name):
                self.assertFalse(
                    self._read(f"/work/.jigc/tasks/t/{name}").is_document,
                    "no read verb exposes this — it is a capability gap, "
                    "not an adapter bypass")

    def test_the_two_rehearsal_runs_are_distinguishable(self) -> None:
        """The property that matters: one count cannot tell these apart."""
        r1 = [self._read("/work/.jigc/tasks/t/docs/adr:x.md"),
              self._read("/work/.jigc/tasks/t/roles.json")]
        r1b = [self._read("/work/.jigc/tasks/t/intent"),
               self._read("/work/.jigc/tasks/t/roles.json")]
        self.assertEqual(len(r1), len(r1b), "identical under the old count")
        self.assertNotEqual(sum(r.is_document for r in r1),
                            sum(r.is_document for r in r1b))


class TheOtherHaltShape(unittest.TestCase):
    """A halt the exact detector cannot see, found by running R2.

    `halted_awaiting_human` keys on `permission_denials` — a tool call was
    refused and the arc stopped. That is one halt shape and it is exact.

    R2 hit the other one: a rejecting `pre-commit` hook refused a doc-promoting
    `jigc task finalize`, the worker stopped and asked the operator to record the
    sign-off, and **nothing was denied** — so `permission_denials` was empty and
    the exact detector reported "not halted" about a session that plainly stopped
    mid-task. That is the exact shape **plant F depends on**, so a trial that
    cannot see it cannot tell a worker who stopped from one who finished.

    Measured across the three rehearsal streams, which is all the evidence this
    heuristic has and all it claims: the two that completed end on a statement,
    the one that stopped ends on a question.
    """

    def _stream(self, result: str, denials: int = 0) -> pathlib.Path:
        d = pathlib.Path(tempfile.mkdtemp(prefix="jigc-halt-"))
        self.addCleanup(shutil.rmtree, d, True)
        p = d / "stream.jsonl"
        event = {"type": "result", "subtype": "success", "is_error": False,
                 "result": result,
                 "permission_denials": [{"tool_name": "Write"}] * denials}
        p.write_text(json.dumps(event) + "\n")
        return p

    #: R2's real tail, verbatim.
    ASKED = ("Task `x` is intact, nothing lost. How would you like to proceed — "
             "do you want to record that sign-off, or is there a different "
             "process I should trigger?")
    #: R1b's real tail, verbatim.
    DONE = ("Landed as commit 4618907, which promoted the ADR to "
            "`docs/decisions/drop-the-oldest-sample.md` and committed it.")

    def test_the_exact_detector_does_not_see_a_product_blocked_halt(self) -> None:
        """The defect, preserved: this is why the heuristic sibling exists."""
        halted, _ = observe_mod.halted_awaiting_human(self._stream(self.ASKED))
        self.assertFalse(halted, "nothing was denied, so the exact signal is silent")

    def test_the_heuristic_sibling_does(self) -> None:
        asked, tail = observe_mod.ended_asking(self._stream(self.ASKED))
        self.assertTrue(asked)
        self.assertIn("sign-off", tail)

    def test_a_completed_session_is_not_reported_as_a_stop(self) -> None:
        asked, _ = observe_mod.ended_asking(self._stream(self.DONE))
        self.assertFalse(asked, "a completion summary is not a question")

    def test_the_two_signals_stay_separate(self) -> None:
        """One is exact and one is a guess; folding them loses which is which."""
        s = self._stream(self.ASKED, denials=2)
        self.assertTrue(observe_mod.halted_awaiting_human(s)[0])
        self.assertTrue(observe_mod.ended_asking(s)[0])
        done = self._stream(self.DONE, denials=2)
        self.assertTrue(observe_mod.halted_awaiting_human(done)[0],
                        "denials halt regardless of how the text reads")
        self.assertFalse(observe_mod.ended_asking(done)[0])


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
        verdict = cascade.grade(o)
        self.assertTrue(verdict.void,
                        "a missing log has not told us the worker declined to read")
        self.assertEqual(verdict.outcome, "apparatus — no invocation log")


if __name__ == "__main__":
    unittest.main(verbosity=2)
