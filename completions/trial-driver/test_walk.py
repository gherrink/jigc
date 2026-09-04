"""`walk.py` — an arm run in an earlier pass keeps what it saw, not only how it exited.

Found on the M50 trial's first walk (2026-09-04): seven arms driven one `--only` at a
time produced a record whose exit column was complete and whose blocks were empty —
`run_arm` returned the stdout to the pass that ran it and nothing wrote it down, so
`earlier()` could only say the arm ran. An arm's PASS/FAIL lines are the evidence;
the exit code is the summary of it.
"""
import pathlib
import sys
import tempfile
import unittest

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import walk  # noqa: E402


class AnEarlierPassKeepsItsOutput(unittest.TestCase):
    def test_persisted_output_is_rendered_on_a_later_pass(self) -> None:
        with tempfile.TemporaryDirectory() as td:
            root = pathlib.Path(td)
            arm = root / "07-example.sh"
            arm.write_text("#!/bin/sh\n")
            out = root / "07-example"
            out.mkdir()
            (out / "PROVENANCE.txt").write_text("exit-code    0\n")
            (out / walk.ARM_OUTPUT).write_text("  OK    the bar\nARM 07 PASS\n")
            record = walk.render([], [arm], root)
        self.assertIn("ARM 07 PASS", record, "the bar lines must survive the pass boundary")
        self.assertIn("earlier pass", record)

    def test_a_pass_that_predates_persistence_says_so(self) -> None:
        with tempfile.TemporaryDirectory() as td:
            root = pathlib.Path(td)
            arm = root / "07-example.sh"
            arm.write_text("#!/bin/sh\n")
            out = root / "07-example"
            out.mkdir()
            (out / "PROVENANCE.txt").write_text("exit-code    0\n")
            record = walk.render([], [arm], root)
        self.assertIn("was not persisted", record,
                      "an unrecoverable block must say it is one, not render as empty")


if __name__ == "__main__":
    unittest.main()
