"""The seed/fork chain's refusals, which are the whole safety story.

Every test here is about the same failure: `--resume` on a session the CLI has
never seen does **not** error — it silently starts a fresh conversation, and the
resulting run looks exactly like a real fork in every channel except one. So the
chain is built out of refusals, and these assert that each one actually refuses.

No API spend: the live end-to-end chain is proven separately and recorded in
`increment-1.md`.

Run: python3 completions/trial-driver/test_session.py
"""
from __future__ import annotations

import json
import pathlib
import sys
import tempfile
import unittest

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))

from driver import cascade, session
from driver.cascade import _obs, grade
from driver.observe import observe


def _write_transcript(path: pathlib.Path, session_id: str, first_turn: str) -> None:
    """A minimally realistic transcript: the record shapes the CLI actually writes."""
    rows = [
        {"parentUuid": None, "isSidechain": False, "type": "user",
         "message": {"role": "user", "content": first_turn},
         "uuid": "u1", "sessionId": session_id, "cwd": "/work", "version": "2.1.233"},
        {"parentUuid": "u1", "isSidechain": False, "type": "assistant",
         "message": {"role": "assistant", "content": [{"type": "text", "text": "ok"}]},
         "uuid": "a1", "sessionId": session_id, "cwd": "/work", "version": "2.1.233"},
    ]
    path.write_text("\n".join(json.dumps(r) for r in rows) + "\n")


def _make_frozen(root: pathlib.Path, *, marker: str = "first turn text",
                 corrupt_sha: bool = False,
                 drop_marker: bool = False) -> session.Frozen:
    root.mkdir(parents=True, exist_ok=True)
    sid = "11111111-2222-3333-4444-555555555555"
    _write_transcript(root / "session.jsonl", sid,
                      "something else" if drop_marker else marker)
    (root / "project").mkdir(exist_ok=True)
    sha = session._sha16((root / "session.jsonl").read_bytes())
    (root / "manifest.json").write_text(json.dumps({
        "session_id": sid, "project_slug": "-work",
        "session_sha": "deadbeefdeadbeef" if corrupt_sha else sha,
        "seed_marker": marker, "turns": 1, "transport": "container",
    }))
    return session.Frozen(root)


class TheSlugIsDerivedNotGuessed(unittest.TestCase):
    def test_container_workdir_slug(self) -> None:
        self.assertEqual(session.project_slug("/work"), "-work")
        self.assertEqual(session.CONTAINER_SLUG, "-work")

    def test_an_absolute_host_path(self) -> None:
        self.assertEqual(session.project_slug("/Users/x/proj"), "-Users-x-proj")


class StagingRefusesRatherThanBeginsCold(unittest.TestCase):
    """The one apparatus failure that manufactures a plausible arm out of nothing."""

    def test_a_missing_transcript_is_refused_not_staged(self) -> None:
        with tempfile.TemporaryDirectory() as d:
            d = pathlib.Path(d)
            with self.assertRaises(SystemExit) as caught:
                session._stage_home(d / "stage", "some-id", d / "nope.jsonl")
            self.assertIn("resume nothing", str(caught.exception))

    def test_a_staged_home_lands_where_the_cli_looks(self) -> None:
        with tempfile.TemporaryDirectory() as d:
            d = pathlib.Path(d)
            tr = d / "session.jsonl"
            _write_transcript(tr, "abc", "hello")
            stage = session._stage_home(d / "stage", "abc", tr)
            landed = stage / ".claude" / "projects" / "-work" / "abc.jsonl"
            self.assertTrue(landed.is_file(),
                            "the transcript must sit at ~/.claude/projects/-work/<id>.jsonl")


class AFixtureIsVerifiedNotAssumed(unittest.TestCase):
    def test_a_good_fixture_verifies(self) -> None:
        with tempfile.TemporaryDirectory() as d:
            self.assertEqual(_make_frozen(pathlib.Path(d) / "f").verify(), [])

    def test_a_drifted_transcript_is_caught(self) -> None:
        """Existence was never enough — a stray edit resumes a different conversation."""
        with tempfile.TemporaryDirectory() as d:
            problems = _make_frozen(pathlib.Path(d) / "f", corrupt_sha=True).verify()
            self.assertTrue(any("sha" in p for p in problems))

    def test_a_fixture_without_its_own_marker_is_caught(self) -> None:
        """If the marker is not in the seed, `seed_inherited` can never be true."""
        with tempfile.TemporaryDirectory() as d:
            problems = _make_frozen(pathlib.Path(d) / "f", drop_marker=True).verify()
            self.assertTrue(any("seed marker" in p for p in problems))

    def test_a_missing_transcript_reports_one_clear_reason(self) -> None:
        with tempfile.TemporaryDirectory() as d:
            f = _make_frozen(pathlib.Path(d) / "f")
            f.transcript.unlink()
            self.assertEqual(len(f.verify()), 1)
            self.assertIn("nothing to resume", f.verify()[0])


class SeedRefusals(unittest.TestCase):
    def test_a_seed_with_no_turns_is_refused(self) -> None:
        with tempfile.TemporaryDirectory() as d:
            with self.assertRaises(SystemExit) as caught:
                session.seed(pathlib.Path(d), [], pathlib.Path(d) / "frozen")
            self.assertIn("not a conversation", str(caught.exception))

    def test_reseeding_over_a_fixture_needs_an_explicit_force(self) -> None:
        with tempfile.TemporaryDirectory() as d:
            frozen = pathlib.Path(d) / "frozen"
            frozen.mkdir()
            with self.assertRaises(SystemExit) as caught:
                session.seed(pathlib.Path(d), ["t"], frozen)
            self.assertIn("force", str(caught.exception))


class AColdForkIsVoidedNotScored(unittest.TestCase):
    """The end-to-end consequence: a cold fork must not become a result."""

    def _log(self, d: pathlib.Path) -> pathlib.Path:
        rows = [
            {"timestamp": "2026-08-25T10:00:00Z", "argv": ["doc", "create", "adr", "--task", "t"],
             "exit_code": 0, "duration_ms": 5, "finding_codes": [], "output_bytes": 10,
             "binary_version": "1.0.0-rc.11", "error_code": None},
            {"timestamp": "2026-08-25T10:00:05Z",
             "argv": ["doc", "show", "adr:x", "--task", "t"],
             "exit_code": 0, "duration_ms": 5, "finding_codes": [], "output_bytes": 10,
             "binary_version": "1.0.0-rc.11", "error_code": None},
        ]
        p = d / "invocations.jsonl"
        p.write_text("\n".join(json.dumps(r) for r in rows) + "\n")
        return p

    def test_a_fork_that_began_cold_voids_despite_a_perfect_verb_result(self) -> None:
        with tempfile.TemporaryDirectory() as d:
            d = pathlib.Path(d)
            tr = d / "forked.jsonl"
            _write_transcript(tr, "new-id", "a completely different opening")
            o = observe("cold", self._log(d), tr,
                        seed_expected=True, seed_marker="the seed's first turn")
            self.assertFalse(o.seed_inherited)
            verdict = grade(o)
            self.assertTrue(verdict.void, "a cold fork must never be scored")
            self.assertEqual(verdict.outcome, "apparatus — the fork began cold")

    def test_a_real_fork_carrying_the_seed_is_scored_normally(self) -> None:
        with tempfile.TemporaryDirectory() as d:
            d = pathlib.Path(d)
            tr = d / "forked.jsonl"
            _write_transcript(tr, "new-id", "the seed's first turn")
            o = observe("warm", self._log(d), tr,
                        seed_expected=True, seed_marker="the seed's first turn")
            self.assertTrue(o.seed_inherited)
            self.assertFalse(grade(o).void)

    def test_a_cold_session_is_not_judged_on_a_seed_it_never_had(self) -> None:
        with tempfile.TemporaryDirectory() as d:
            d = pathlib.Path(d)
            o = observe("plain", self._log(d), None, seed_expected=False)
            self.assertFalse(grade(o).void)


class ForkedIdRecovery(unittest.TestCase):
    def test_the_new_id_is_read_off_the_stream(self) -> None:
        """`--fork-session` mints a new id, so the log to read is not the seed's."""
        with tempfile.TemporaryDirectory() as d:
            s = pathlib.Path(d) / "stream.jsonl"
            s.write_text(json.dumps({"type": "system", "session_id": "brand-new"}) + "\n")
            self.assertEqual(session.forked_id(s), "brand-new")

    def test_a_missing_stream_yields_none_rather_than_raising(self) -> None:
        self.assertIsNone(session.forked_id(pathlib.Path("/nonexistent/stream.jsonl")))

class CarryForwardKeepsTheTreeAndDropsTheRig(unittest.TestCase):
    """A turn's out-dir becomes the next turn's corpus, so the rig must not travel.

    Without this, `.session-transcript/`, `PROVENANCE.txt`, `stream.jsonl` and
    `stderr.txt` are copied into `/work` as corpus content on every turn after the
    first — apparatus artefacts planted in the tree under test, in every arm.
    """

    def test_evidence_is_dropped_and_history_is_kept(self) -> None:
        with tempfile.TemporaryDirectory() as d:
            d = pathlib.Path(d)
            out = d / "out"
            (out / ".session-transcript" / "projects").mkdir(parents=True)
            (out / ".session-transcript" / "projects" / "s.jsonl").write_text("{}")
            for name in ("PROVENANCE.txt", "stream.jsonl", "stderr.txt"):
                (out / name).write_text("x")
            (out / "src").mkdir()
            (out / "src" / "a.ts").write_text("code")
            (out / ".git").mkdir()
            (out / ".git" / "HEAD").write_text("ref: refs/heads/main")

            carried = session.carry_forward(out, d / "next")
            names = sorted(p.name for p in carried.iterdir())
            self.assertEqual(names, [".git", "src"])
            self.assertTrue((carried / ".git" / "HEAD").is_file(),
                            "the next turn must see the history")


class IncrementOne(unittest.TestCase):
    """The live chain's own numbers, from its own archived evidence."""

    EV = pathlib.Path(__file__).resolve().parent / "increment-1-evidence"

    def setUp(self) -> None:
        if not self.EV.is_dir():
            self.skipTest("increment-1 evidence not present")

    def test_both_forks_minted_new_ids_distinct_from_the_seed(self) -> None:
        seed = json.loads((self.EV / "seed-manifest.json").read_text())["session_id"]
        ids = set()
        for name in ("forkA", "forkB"):
            results = json.loads((self.EV / f"{name}-result.json").read_text())
            got = results[-1]["session_id"]
            self.assertNotEqual(got, seed, "--fork-session must mint a new id")
            ids.add(got)
        self.assertEqual(len(ids), 2, "the two forks must not share an id")

    def test_both_forks_knew_what_only_the_seed_said(self) -> None:
        """The behavioural proof: 500 and the eviction policy exist only in turn 2.

        A fork that began cold could not produce them, so this is a stronger check
        than the marker — which says the file was carried, not the conversation.
        """
        turns = (self.EV / "seed-turns.txt").read_text()
        self.assertIn("500", turns, "the fact must be in the seed, or this proves nothing")
        for name in ("forkA", "forkB"):
            results = json.loads((self.EV / f"{name}-result.json").read_text())
            answer = str(results[-1]["result"])
            self.assertIn("500", answer, name)
            self.assertIn("evict", answer.lower(), name)

    def test_a_cold_resume_refuses_loudly_on_this_build(self) -> None:
        """The harvested warning said a bad resume starts a fresh conversation.

        On this build it refuses instead — exit 1, `error_during_execution`, zero
        turns. Pinned so that a future CLI reverting to the silent behaviour is
        visible rather than assumed. `seed_inherited` stays regardless: the failure
        it guards is silent by construction wherever this refusal does not happen.
        """
        stderr = (self.EV / "cold-resume-stderr.txt").read_text()
        self.assertIn("No conversation found with session ID", stderr)
        result = json.loads((self.EV / "cold-resume-result.json").read_text())[-1]
        self.assertEqual(result["subtype"], "error_during_execution")
        self.assertTrue(result["is_error"])
        self.assertEqual(result["num_turns"], 0)


if __name__ == "__main__":
    unittest.main(verbosity=2)
