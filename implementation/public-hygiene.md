# Public hygiene — what may enter this repository

This repository is public, and so is every commit it has ever carried: a blob that reaches `main` stays readable by sha long after a later commit deletes it, so the only fix for a leak that landed is a history rewrite. **These rules keep that from being needed again.** They were written when the history was rewritten before publication ([completions/artifacts/M54/pre-public-audit.md](../completions/artifacts/M54/pre-public-audit.md) records the method and the verdict); the guard at the end enforces the mechanical half.

## The rules

1. **Trials and evals run on synthetic or public corpora.** The trial tooling builds synthetic corpora for exactly this reason ([trial-corpus-template](../completions/trial-corpus-template/), [`dev/jigc-rig`](../dev/jigc-rig)); a public open-source project is the other acceptable subject.
2. **A trial on real work keeps its raw evidence outside the repository.** Transcripts, invocation logs, `git log` dumps and copied documents from a real employer or client repository stay on the machine that ran the trial. What is committed is the aggregated, anonymized finding: the defect in jigc, its repro against a synthetic corpus, and the counts — never the subject repository's content.
3. **No raw transcripts, unless a tool or a test reads them as a named fixture.** A transcript that a test opens by path (the `completions/trial-driver/` suites read a small, named set) is a fixture and may stay; a transcript kept "for the record" is raw evidence under rule 2. A synthetic-corpus transcript still passes through rule 4 before it is committed — a `git log` inside a session prints the machine's own identity.
4. **Never commit, in any file or commit message:**
   - a credential, token, key or password — **including a revoked one**, and including one quoted inside a record or a transcript;
   - an internal hostname or URL;
   - an employer's, client's or internal project's name, or a name derived from one (a repository name, a slug, a trial id, a directory name, a code identifier);
   - an issue tracker's ticket keys from a non-public tracker;
   - another person's name or email address.

   The author's own public identity (the git author line, the public GitHub handle, the author's own public projects) is not covered by this rule.
5. **When a real subject has to be referred to at all, use the fixed token vocabulary** — never an ad-hoc abbreviation, which leaks by being guessable:

   | Token | Stands for |
   |---|---|
   | `acme` | the employer |
   | `project-alpha`, `project-beta`, `project-gamma`, `project-delta` | a real internal or client project (a version suffix keeps its shape: `project-alpha-2.0`) |
   | `project-kb` | a real internal knowledge-base project |
   | `client-1`, `client-2`, … | a real client, when the project itself need not be named |
   | `TKT-<n>` | a real ticket key |
   | `[credential-redacted]` | a credential value |

   Existing tokens keep their meaning; a new subject takes the next free one.

## The guard

The guard is mechanical, and it is a floor, not a review: it catches the names someone has already written down and the secret shapes gitleaks knows. Rules 1–3 are judgment and stay with the author.

- **The denylist** is the private list of terms that must not appear — one POSIX extended regex per line, matched case-insensitively; blank and `#` lines are ignored. **It never lives in this repository**, because a committed denylist is itself the leak. [`dev/hygiene-scan`](../dev/hygiene-scan) runs it over a revision range — every commit message and every *added* line, a merge's own conflict resolution included (`--remerge-diff`), plus, with `--tree`, the tracked working tree — and reports a hit by commit, path and line number only, never by the matching text, because a CI log is public. A pattern a matcher rejects is a setup error (exit 2, named by its denylist line, never by its text), never a clean scan: one typo in the secret would otherwise switch the whole guard off while CI stays green.
- **CI** ([.github/workflows/ci.yml](../.github/workflows/ci.yml)) opens its `hygiene` job with the **branch-name rule** — not a leak check, but the same job because it is just as cheap and just as mechanical: a push to, or a pull request from, a branch outside the closed set of [CLAUDE.md](../CLAUDE.md) → Branches fails in seconds with the allowed patterns listed ([`dev/branch-name`](../dev/branch-name), held to jigc's slug grammar by `crates/cli/tests/branch_name_fence.rs`; a tag push is not checked, nor a pull request from a fork — [CLAUDE.md](../CLAUDE.md) → Branches states the exemption, and the steps below run on a fork's pull request unchanged). It then runs two steps over the pushed range (the push's `before..HEAD`, a pull request's `base..HEAD`, and the whole history on a branch's first push), in its own `hygiene` job, beside the cargo jobs: **gitleaks** (pinned version, `--redact`, with this repository's [.gitleaks.toml](../.gitleaks.toml) allowlisting the one known false positive), and **the denylist**, read from the `JIGC_DENYLIST` Actions secret. With the secret unset — a fork's pull request, or a repository that never set it — the denylist step skips with a notice rather than passing silently.
- **Locally**, [`dev/gate`](../dev/gate) prints a hygiene advisory before its steps: the denylist from `~/.config/jigc/denylist` (or `$JIGC_DENYLIST_FILE`) over the unpushed commits and the working tree, and gitleaks over the unpushed commits when it is installed. It is an advisory like the deps count — it warns and names the file, and never joins the step list or the verdict — so the hard stop is CI's, and the advisory is how a hit is found before the push that CI would refuse.
- **A stabilization run's own pushes are scanned before they are made.** The run's step tool makes every push of a stage, and before each it runs the two scanners above over every commit no branch of the remote holds yet — the same two commands, over the same kind of range, as the `hygiene` job — and refuses the push on a hit, or when a scanner cannot run ([stabilization-workflow.md](stabilization-workflow.md) → The record step, *What is vetted, and where*, which also says what a run's own records are held to beside the scan). **A push made by hand with plain git is outside it**: there the advisory is what comes before the push, and CI what comes after.
- **A hit is fixed before the push, not after.** An unpushed commit is rewritten (`git commit --amend`, an interactive rebase); a pushed one needs a history rewrite and a force-push, and the old objects stay reachable on the host until it garbage-collects them — which is why the local advisory exists.
