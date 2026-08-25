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

    def test_apparatus_rows_come_before_scored_rows(self) -> None:
        """A voided run must never be scored, so no score may outrank a void."""
        seen_score = False
        for rule in DEFAULT_CASCADE:
            if not rule.void:
                seen_score = True
            elif seen_score:
                self.fail(f"void row {rule.n} sits below a scored row")


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

    def test_an_unregistered_cascade_is_refused(self) -> None:
        self.assertIn("registers no outcome table",
                      " ".join(cascade.check_registration("no table here")))


if __name__ == "__main__":
    unittest.main(verbosity=2)
