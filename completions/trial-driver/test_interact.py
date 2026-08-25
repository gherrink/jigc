"""The answer-key halt, and the contamination screen that cannot be forgotten.

Run: python3 completions/trial-driver/test_interact.py
"""
from __future__ import annotations

import pathlib
import re
import sys
import tempfile
import unittest

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))

from driver import interact
from driver.interact import Answer, OperatorLog, answer, contaminates, load_answer_key, screen

KEY = """\
# Answer key

## sign-off
match: sign.?off|approv|gate
why: the operator has cleared the docs gate out of band

> Signed off — that is what I wanted recorded. I have cleared the gate my end; go again.

## split the commit
match: split|separate commit
why: the two staged files are mine and were staged before the task began

> Yes, split it. I want that commit to be the ADR and nothing else; leave my two files staged the way they were.
"""


class TheScreenRefusesRatherThanWarns(unittest.TestCase):
    """§3.2's contamination rule is absolute, so it is a refusal, not a log line."""

    def test_a_reply_naming_a_read_surface_is_refused(self) -> None:
        with self.assertRaises(SystemExit) as caught:
            screen("Go ahead, but check the doc first.")
        self.assertIn("contamination rule", str(caught.exception))

    def test_the_words_that_void_a_measurement(self) -> None:
        for bad in ("read it back", "verify the output", "run doc show",
                    "please confirm", "look at the file", "validate it"):
            with self.subTest(reply=bad):
                self.assertTrue(contaminates(bad), bad)

    def test_a_clean_reply_passes(self) -> None:
        clean = "Signed off — that is what I wanted recorded. Go again."
        self.assertEqual(contaminates(clean), [])
        screen(clean)

    def test_an_answer_key_is_screened_at_load_not_at_use(self) -> None:
        """An answer key is written before a trial and read during one.

        A screen that only fires on use fires in the middle of a session, which is
        the worst possible moment to discover the reply was unusable.
        """
        with tempfile.TemporaryDirectory() as d:
            p = pathlib.Path(d) / "key.md"
            p.write_text("## bad\nmatch: x\n\n> Sure — just verify it first.\n")
            with self.assertRaises(SystemExit):
                load_answer_key(p)


class AnswerOrHaltAndNothingElse(unittest.TestCase):
    def setUp(self) -> None:
        self.tmp = tempfile.TemporaryDirectory()
        self.key_path = pathlib.Path(self.tmp.name) / "answer-key.md"
        self.key_path.write_text(KEY)
        self.keys = load_answer_key(self.key_path)

    def tearDown(self) -> None:
        self.tmp.cleanup()

    def test_the_key_parses_both_entries(self) -> None:
        self.assertEqual([k.key for k in self.keys], ["sign-off", "split the commit"])

    def test_a_covered_question_is_answered_from_the_key(self) -> None:
        got = answer("The pre-commit hook rejected my finalize — can you approve "
                     "the docs gate?", self.keys)
        self.assertFalse(got.halted)
        self.assertEqual(got.matched, "sign-off")
        self.assertIn("Signed off", got.text)

    def test_an_uncovered_question_halts_rather_than_improvises(self) -> None:
        """Improvising is how a session gets contaminated, so there is no third branch."""
        got = answer("Should I use a token bucket or a sliding window here?", self.keys)
        self.assertTrue(got.halted)
        self.assertEqual(got.matched, None)
        self.assertEqual(got.text, "")
        self.assertIn("improvising", got.reason)

    def test_every_branch_produces_a_reason(self) -> None:
        """No silent branch: a halt is data, with a why attached."""
        for q in ("approve the gate?", "something nobody anticipated"):
            self.assertTrue(answer(q, self.keys).reason)


class TheLogIsWrittenAsItHappens(unittest.TestCase):
    """The entry that was missed in the 1.0.0-gate trial changed a headline finding."""

    def test_a_reply_and_a_halt_both_land_in_the_log(self) -> None:
        with tempfile.TemporaryDirectory() as d:
            log = OperatorLog(pathlib.Path(d) / "operator-log.md", "B3 stonefly")
            log.record("reply", "Signed off — go again.", "answer-key entry 'sign-off'")
            log.record_halt("Token bucket or sliding window?", "no answer-key entry matches")
            text = log.path.read_text()
            self.assertIn("Signed off", text)
            self.assertIn("HALT — waiting for a human", text)
            self.assertIn("Token bucket or sliding window?", text)
            self.assertIn("*Why:*", text)

    def test_entries_append_rather_than_replace(self) -> None:
        with tempfile.TemporaryDirectory() as d:
            p = pathlib.Path(d) / "operator-log.md"
            OperatorLog(p, "s").record("reply", "first", "a")
            OperatorLog(p, "s").record("reply", "second", "b")
            text = p.read_text()
            self.assertIn("first", text)
            self.assertIn("second", text)
            self.assertEqual(text.count("# Operator log"), 1)


if __name__ == "__main__":
    unittest.main(verbosity=2)
