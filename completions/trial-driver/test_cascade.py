"""The cascade's own invariants, and the two defects they exist to catch.

Run: python3 completions/trial-driver/test_cascade.py
"""
from __future__ import annotations

import pathlib
import sys
import unittest

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))

from driver import cascade
from driver.cascade import DEFAULT_CASCADE, Rule, _obs, grade


class TheDefaultCascadeIsWellFormed(unittest.TestCase):
    def test_shape(self) -> None:
        self.assertEqual(cascade.check_shape(), [])

    def test_every_row_is_reachable(self) -> None:
        """No row may be shadowed by an earlier one.

        This check found a real defect the first time it ran: row 1's example did
        not set the field row 1's own predicate reads, so it fell through to row 2
        and row 1 was unreachable.
        """
        self.assertEqual(cascade.check_examples(), [])

    #: Void rows that invalidate the WHOLE run: nothing measured is trustworthy, so
    #: they must outrank every scored row. Named explicitly rather than derived from
    #: `void`, because the two kinds of void are genuinely different.
    RUN_INVALIDATING = {1, 2, 3, 4, 5, 6}

    def test_run_invalidating_rows_come_before_every_scored_row(self) -> None:
        """The invariant, in its corrected form.

        It first read "no score may outrank a void", which was too strong and broke
        the moment a second KIND of void arrived. Row 10 voids only the FILESYSTEM
        channel's evidence, and it sits deliberately BELOW the positive rows so that
        a session with a VERB result — measured on the invocation log, which is
        present — is not thrown away because a secondary channel is absent.

        So the real invariant is narrower: a void that invalidates the whole run
        must precede every score; a void scoped to one channel's evidence need not.
        """
        first_score = min(r.n for r in DEFAULT_CASCADE if not r.void)
        for rule in DEFAULT_CASCADE:
            if rule.n in self.RUN_INVALIDATING:
                self.assertTrue(rule.void, f"row {rule.n} should be a void row")
                self.assertLess(rule.n, first_score,
                                f"run-invalidating row {rule.n} sits below a score")

    def test_every_void_row_is_either_run_invalidating_or_evidence_scoped(self) -> None:
        """No third kind may appear without this test being revisited."""
        evidence_scoped = {r.n for r in DEFAULT_CASCADE
                           if r.void and r.n not in self.RUN_INVALIDATING}
        self.assertEqual(evidence_scoped, {10},
                         "a new void row needs a stated kind, not a silent one")


class ThereIsOnlyOneScoringPath(unittest.TestCase):
    """A regression fence, from a defect this package actually shipped.

    `Observation` once carried an `outcome` property that re-implemented the
    cascade's ordering privately. It knew nothing about `authoring_writes`,
    `seed_inherited` or `rc_failed`, so on the first live strict arm it returned
    `NEITHER` — a claim about the worker — while the cascade correctly voided the
    run as `unmeasured — no authoring occasion existed`.

    That is precisely what this module's docstring says cannot happen ("there is no
    second place where a row can be applied"), so the absence is asserted rather
    than trusted.
    """

    def test_an_observation_carries_facts_and_does_not_score(self) -> None:
        self.assertFalse(hasattr(_obs(), "outcome"),
                         "scoring belongs to cascade.grade(), in one list")

    def test_the_shape_that_disagreed_is_voided(self) -> None:
        """13 records, nothing authored — the live strict arm, in miniature."""
        verdict = grade(_obs(records=13, authoring_writes=0))
        self.assertTrue(verdict.void)
        self.assertNotEqual(verdict.outcome, "proceeded without reading")


class NoSilentBranch(unittest.TestCase):
    """`cue-card-postmortem.md` §8 rule 3, asserted rather than intended."""

    def test_a_worker_that_did_nothing_still_scores(self) -> None:
        r = grade(_obs(records=4, verb=0, adjacent=0, filesystem=0))
        self.assertEqual(r.outcome, "proceeded without reading")
        self.assertFalse(r.void, "declining to read is a result, not an apparatus failure")

    def test_a_cascade_without_a_catch_all_is_refused(self) -> None:
        truncated = DEFAULT_CASCADE[:-1]
        self.assertIn("silent branch", " ".join(cascade.check_shape(truncated)))
        with self.assertRaises(RuntimeError):
            grade(_obs(), truncated)


class ApparatusIsNeverScored(unittest.TestCase):
    def test_a_cold_fork_voids_even_with_a_perfect_primary_outcome(self) -> None:
        """The one that manufactures a plausible arm out of a dead fixture.

        `--resume` on a session the CLI has never seen does not fail; it starts a
        fresh conversation. Such a run can produce a textbook VERB result and mean
        nothing, so the void must outrank the score — the shape the eval rig had
        to force for real on a live container turn to prove its own void block.
        """
        o = _obs(seed_expected=True, seed_inherited=False, verb=5, records=90)
        r = grade(o)
        self.assertTrue(r.void)
        self.assertEqual(r.outcome, "apparatus — the fork began cold")

    def test_a_cold_fork_is_only_possible_when_a_seed_was_expected(self) -> None:
        """A cold session has no seed, so the check must not fire on it."""
        r = grade(_obs(seed_expected=False, seed_inherited=False, verb=1))
        self.assertEqual(r.outcome, "read back through the fence's verb")

    def test_jigc_refusals_are_results_not_apparatus_failures(self) -> None:
        """`nonzero_exits` counts jigc's own gates firing — frequently the point."""
        r = grade(_obs(nonzero_exits=7, verb=2))
        self.assertFalse(r.void)


class UnmeasuredIsNotNeither(unittest.TestCase):
    """§3.3: *"The session never reached an authoring step — no occasion existed;
    unmeasured, not NEITHER."*

    Both tests here are regressions from the first live headless arm, which
    truncated on a denied write before authoring anything. Without the row it
    scored `proceeded without reading` — a statement about the worker that the
    evidence does not support.
    """

    def test_a_session_with_nothing_staged_cannot_be_scored_for_read_back(self) -> None:
        r = grade(_obs(records=13, authoring_writes=0, verb=0, adjacent=0, filesystem=0))
        self.assertTrue(r.void)
        self.assertEqual(r.outcome, "unmeasured — no authoring occasion existed")

    def test_the_row_outranks_every_scored_row(self) -> None:
        """A doc cannot be read back before it exists, so this must win."""
        for kw in ({"verb": 3}, {"adjacent": 2}, {"filesystem": 1}):
            with self.subTest(**kw):
                r = grade(_obs(authoring_writes=0, **kw))
                self.assertTrue(r.void, "no staged doc existed to read")

    def test_an_authoring_session_is_scored_normally(self) -> None:
        r = grade(_obs(authoring_writes=4, verb=2))
        self.assertFalse(r.void)
        self.assertEqual(r.outcome, "read back through the fence's verb")


class MissingEvidenceIsNotAVerdict(unittest.TestCase):
    """Findings 1 and 2 of the cross-model review, as fences.

    Both are the same shape: an apparatus failure reaching a row that makes a claim
    about the worker.
    """

    def test_a_dead_cli_voids_even_with_a_clean_looking_log(self) -> None:
        """Finding 1: the CLI exits 1 after one authoring call and one read.

        The log survives and looks perfect. Before the fix, `do_fork` never set
        `rc_failed`, so row 1 was unreachable through the real fork path and this
        scored as `read back through the fence's verb`.
        """
        verdict = grade(_obs(rc_failed=True, records=12, authoring_writes=1, verb=1))
        self.assertTrue(verdict.void)
        self.assertEqual(verdict.outcome, "apparatus — the session did not run")

    def test_a_missing_transcript_cannot_become_neither(self) -> None:
        """Finding 2: NEITHER claims the worker read nothing ANYWHERE.

        That needs both channels looked at, and FILESYSTEM lives only in the
        transcript. With none, `filesystem == 0` because nothing was read, not
        because nothing happened.
        """
        verdict = grade(_obs(authoring_writes=1, verb=0, adjacent=0,
                             filesystem=0, transcript_missing=True))
        self.assertTrue(verdict.void)
        self.assertIn("no transcript", verdict.outcome)

    def test_but_a_verb_result_survives_a_missing_transcript(self) -> None:
        """The claim's own channel is the invocation log, which is present.

        Voiding a good measurement because a secondary channel is absent would
        throw away exactly what the trial is for.
        """
        verdict = grade(_obs(authoring_writes=1, verb=3, transcript_missing=True))
        self.assertFalse(verdict.void)
        self.assertEqual(verdict.outcome, "read back through the fence's verb")


class RegistrationCorrespondence(unittest.TestCase):
    def test_the_code_matches_a_table_generated_from_it(self) -> None:
        table = "\n".join(
            f"| {r.n} | `{r.outcome}` | {'void' if r.void else 'score'} |"
            for r in DEFAULT_CASCADE)
        self.assertEqual(cascade.check_registration(table), [])

    def test_transposed_rows_are_caught_not_smoothed_over(self) -> None:
        """Order is compared, not membership.

        A set comparison stays green while two rows are implemented the wrong way
        round, which is how a void row came to be outranked in the rig this is
        modelled on.
        """
        rows = list(DEFAULT_CASCADE)
        rows[0], rows[1] = rows[1], rows[0]
        table = "\n".join(
            f"| {i} | `{r.outcome}` | {'void' if r.void else 'score'} |"
            for i, r in enumerate(rows, 1))
        problems = cascade.check_registration(table)
        self.assertTrue(problems)
        self.assertTrue(any("position 1" in p for p in problems))

    def test_a_table_inside_a_code_fence_does_not_certify(self) -> None:
        """Finding 10: the regex scans lines, not Markdown.

        An exact copy of an old table inside a ```fence``` was indistinguishable
        from the live one, so the check could pass while the operative table said
        something else — or had been deleted.
        """
        real = "\n".join(f"| {r.n} | `{r.outcome}` | {'void' if r.void else 'score'} |"
                          for r in DEFAULT_CASCADE)
        fenced_only = "Here is how the table used to look:\n\n```\n" + real + "\n```\n"
        self.assertTrue(cascade.check_registration(fenced_only),
                        "a table that is only an example must not certify")
        self.assertEqual(cascade.check_registration(real), [],
                         "the live table still does")

    def test_an_unregistered_cascade_is_refused(self) -> None:
        self.assertIn("registers no outcome table",
                      " ".join(cascade.check_registration("no table here")))


#: Every trial protocol that registers this cascade. `check_registration` existed
#: from the start and was exercised only against synthetic tables — a fence nothing
#: pointed at, which is the same failure mode as a tool nothing references. The
#: RC-1.0-final protocol registers the table in its §3.6, so the comparison is now
#: live: changing a row in either place without the other reddens here.
#:
#: A protocol that does not register a table is skipped, not failed — the earlier
#: trials were written before the cascade existed and back-filling their protocols
#: would be editing a pre-registered instrument after its trial ran.
PROTOCOLS = sorted(
    (pathlib.Path(__file__).resolve().parents[1] / "artifacts").glob(
        "RC-*/protocol.md"))


class TheRegisteredTableMatchesTheShippedCascade(unittest.TestCase):
    """The protocol's own enumeration and the grader must agree, in order."""

    def test_at_least_one_protocol_registers_the_table(self) -> None:
        """Otherwise this whole class is green by vacuity."""
        registering = [p for p in PROTOCOLS
                       if "registers no outcome table" not in
                       " ".join(cascade.check_registration(p.read_text()))]
        self.assertTrue(registering,
                        "no shipped protocol registers the cascade — the fence is "
                        "pointing at nothing, which is how it started")

    def test_every_registering_protocol_agrees_with_the_code(self) -> None:
        for p in PROTOCOLS:
            problems = cascade.check_registration(p.read_text())
            if problems == ["the protocol registers no outcome table "
                            "(no matching rows found)"]:
                continue  # predates the cascade; see PROTOCOLS
            with self.subTest(protocol=str(p.relative_to(p.parents[3]))):
                self.assertEqual(problems, [],
                                 "the protocol's registered outcomes and "
                                 "cascade.py have drifted apart")


if __name__ == "__main__":
    unittest.main(verbosity=2)
