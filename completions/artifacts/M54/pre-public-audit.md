# M54 — the pre-public audit

Gate-record **row 1** ([planning-gate-record.md](planning-gate-record.md)); the procedure's home is
[release.md](../../../implementation/release.md) → The one-time bootstrap. **This is the short, neutral
record of the audit and of the history rewrite it led to, and this file's first appearance in this
repository.** The working record — which named what it found, with counts and locations — is not in
this repository's history at all: it is kept with the original history in the private archive, because
a catalogue of what was removed, even anonymized, outlines what was removed. The rules that keep this from recurring are
[implementation/public-hygiene.md](../../../implementation/public-hygiene.md).

## Method

1. **Secret scan.** gitleaks 8.30.1 over the full history of every ref (`gitleaks git --log-opts="--all"
   --redact`) and over the working tree (`gitleaks dir --redact`), with the default ruleset.
2. **Targeted greps.** Provider token shapes, secret assignments, email addresses, URLs and hosts, home
   paths and private config paths, person names, and every candidate employer, client and project name,
   over `git log --all -p` (every line any blob ever added), every commit message, and every tracked
   path.
3. **Census.** Every path ever tracked classified as keep · anonymize · remove, with what cites it and
   what executes it, so that nothing a test or a live document depends on was removed blind.
4. **Rewrite into a fresh repository.** `git filter-repo` on a scratch clone, never on the working
   repository: the required files and the working audit record removed from every commit, the
   confirmed names replaced by fixed
   tokens in every blob and every commit message, the affected paths renamed, the one credential value
   redacted. The commit map is [commit-map.txt](commit-map.txt) ([its note](commit-map.README.md)); every
   sha cited in this repository's own files was translated through it.
5. **Archive.** The original repository, with its full history, is kept private and is not published.

## What was found — classes, not names

- **Employer and client project data in trial evidence.** Adoption trials run on copies of real
  internal and client repositories had committed their raw evidence: invocation logs carrying the
  subject repositories' document titles, a client's architecture document and two clients' changelogs
  verbatim (one with the employer's issue-tracker keys), a subject repository's own `git status`/`git
  log` dump, and slugs naming internal strategy work. The trial records themselves — the findings about
  jigc — named the repositories, the employer and an internal service host.
- **One possibly-live credential.** A trial record quoted a hardcoded credential of a subject
  repository by a token that may be its value, beside the titles of that repository's open security
  findings.
- **Personal data.** A work email address in `git log` output captured inside session transcripts.
- **Not found:** no provider-shaped secret, no private key, no other person's name or email, no
  third-party connector traffic in any transcript. gitleaks' one finding in the original history was a
  test fixture's SHA-256 (a false positive, below).

## The human's decisions (2026-09-28)

- **Rewrite, not accept or redact at HEAD.** The findings above go from every commit, not only from the
  tip.
- **Removed from all history:** the 16 files of verbatim third-party and employer content — and, by a
  second decision (2026-09-29), the working audit record itself, because even anonymized it outlined the
  removed material. The commit that recorded it held nothing else and is dropped; this record replaces
  it.
- **Replaced in every blob and message by the fixed vocabulary:** the employer → `acme`; the real
  projects → `project-alpha` (keeping its `-N.0` version shape, which two test fixtures use),
  `project-beta`, `project-gamma`, `project-delta`, `project-kb`; ticket keys → `TKT-<n>`; the trial ids
  derived from a project name → `RC-alpha3` / `RC-alpha4`, their directories renamed with them; the
  internal host → `http://audit-svc`; the subject application's class names → generic names; the work
  email → the author's public address; the credential → `[credential-redacted]`. The open security
  findings are reduced to generic descriptions.
- **Generalized in every blob and message, by a third decision (2026-09-29):** the remaining
  outline descriptions of the removed material — one subject document's size, language, stack and
  subsystem names, one subject application's stack, integrations, internal service name and the
  specifics of its security fixes — are reduced to generic wording (a ~700-line non-English
  architecture document, a PHP/JS monorepo, the CMS, the git host, the audit service, security fixes
  landed with tests). A last pass the same day generalized the remaining regulator name and the
  subject application's tooling and code identifiers in the same way.
- **Kept:** the author's own name, handle and public projects; public vendor and product names; machine
  paths; the `Claude-Session` trailers in commit messages.
- **At HEAD, by ordinary commits:** raw session transcripts and study captures that no tool reads were
  removed (the transcripts a test opens by path stay); the superseded readiness record and the pre-MVP
  review moved under `completions/artifacts/`; every record that pointed at a removed file says so where
  it points.
- **Rotating the credential is the human's**, independent of the rewrite, and is not recorded here.

## The rewrite

| | |
|---|---|
| Commits | **2,535** original → **2,534** kept, every one rehashed (the original commits were GPG-signed, and a rewrite cannot carry a signature over); **1** dropped — the working-audit commit, which maps to forty zeros in the commit map |
| Blob versions rewritten by the content rules | 2,130 of 12,331 |
| Commit messages changed | 27 (plus filter-repo's own abbreviated-sha rewrite in every message) |
| Paths removed from history | 17 (the 16 required files and the working audit record) |
| Paths renamed | 16 (the two trial-id directories) |
| Sha citations translated at HEAD | 2,825 across 437 files |

## Verdict — re-run over the rewritten repository

Scanned at `55557eb8` (2,538 commits: the rewritten history plus the commits that
translated its citations, swept HEAD and added the guard).

| Scan | Result |
|---|---|
| `gitleaks git --log-opts="--all" --redact`, default ruleset | **1** finding — `generic-api-key` on the `crates/engine/src/file_state.rs` insta snapshot, the SHA-256 of a test fixture: **false positive** |
| `gitleaks dir --redact`, default ruleset | **1** finding — the same snapshot line at HEAD: **false positive** |
| Both, with this repository's [.gitleaks.toml](../../../.gitleaks.toml) (that one line allowlisted by file and exact shape) | **0** |
| The private denylist (38 patterns: every confirmed name and variant, the credential, the host, the class names, the finding titles), case-insensitive, over every commit message, every line every commit added, every path and the tracked tree | **0 hits** |
| The same terms normalised (case and punctuation stripped, to catch a term spelled as a regex or split by separators) over every object in the repository — 27,971 commits, trees and blobs, binary included | **0 hits** |

**CLEARED for publication, subject to the human's sign-off below.** No secret and none of the confirmed
names remain in any commit, message, path or blob of the rewritten history.

Signed: ______________________________________________ Date: ______________
