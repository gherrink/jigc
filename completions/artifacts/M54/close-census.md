# The M54 marker census — M54 Increment 12, T5

This file records the close's census ([planning-gate-record.md](planning-gate-record.md) → the census row and row 24). It counts every live-doc line that names `M54` on three trees: the gate-record's baseline, the increment's base, and the tree this record is committed in. Each matching line in the last tree gets one disposition. **Flipped** means the line spoke of M54 in the future tense, or stated as pending something that has now happened, and was reworded; the new wording is given. **Kept** means the line is true today, and the reason is given.

## The set and the count

**The census set** is the tracked `*.md` files minus `DECISIONS.md`, `implementation/roadmap.md`, `implementation/project-history.md` and everything under `completions/artifacts/`. Those are dated records, and their mentions of M54 are history by construction. **The count** is matching **lines** per file, not occurrences.

```sh
# a committed tree: the baseline 5f730417, the base f7db572a
git grep -c 'M54' <rev> -- '*.md' ':!DECISIONS.md' ':!implementation/roadmap.md' \
  ':!implementation/project-history.md' ':!completions/artifacts/'

# after: the working tree this record is committed with, over the same tracked set
git ls-files -z -- '*.md' ':!DECISIONS.md' ':!implementation/roadmap.md' \
  ':!implementation/project-history.md' ':!completions/artifacts/' \
  | xargs -0 command grep -c 'M54' | command grep -v ':0$'
```

- **`5f730417`** is the Settle's commit, which is the tree the gate-record's census baseline was taken on (*M54 appears in 16 live docs*). This method reproduces that baseline's 16 files, and 15 of the 16 counts match. The 16th is `release.md`, which reads **8** here against the record's **7**. The planning entry already noted this off-by-one, and it is left as found.
- **`f7db572a`** is the base, the increment's planning commit, at **114 lines in 19 files**. The three files new since the baseline are `design/bootstrap.md`, `implementation/pinning.md` and `implementation/public-hygiene.md`.
- **after** is **115 lines in 19 files**. The one added line is `release.md:108`, which T1 (`0313fda4`) rewrote to the verified yank fact with a citation of `completions/artifacts/M54/publish-proof.md`. `release.md` reads 30 at `0313fda4^` and 31 at `0313fda4`. T5's flips change no count, because every flipped line still names M54.

| File | `5f730417` (baseline) | `f7db572a` (base) | after |
|---|--:|--:|--:|
| `CLAUDE.md` | 7 | 5 | 5 |
| `VISION.md` | 1 | 1 | 1 |
| `completions/trial-harness/README.md` | 3 | 4 | 4 |
| `design/assistant-adapter.md` | 4 | 4 | 4 |
| `design/bootstrap.md` | — | 1 | 1 |
| `design/project-setup.md` | 3 | 3 | 3 |
| `design/storage.md` | 1 | 1 | 1 |
| `design/validation.md` | 2 | 3 | 3 |
| `design/worked-examples.md` | 4 | 4 | 4 |
| `ideas/symbol-mention-sweep.md` | 4 | 4 | 4 |
| `implementation/decisions-pending.md` | 25 | 29 | 29 |
| `implementation/dev-workflow.md` | 3 | 3 | 3 |
| `implementation/language-runtime.md` | 2 | 2 | 2 |
| `implementation/machine-setup.md` | 1 | 1 | 1 |
| `implementation/milestone-completion-workflow.md` | 2 | 3 | 3 |
| `implementation/module-layout.md` | 10 | 14 | 14 |
| `implementation/pinning.md` | — | 1 | 1 |
| `implementation/public-hygiene.md` | — | 1 | 1 |
| `implementation/release.md` | 8 | 30 | 31 |
| **Total** | **80 lines, 16 files** | **114 lines, 19 files** | **115 lines, 19 files** |

## The forward-tense markers

```sh
git ls-files -z -- '*.md' ':!DECISIONS.md' ':!implementation/roadmap.md' \
  ':!implementation/project-history.md' ':!completions/artifacts/' \
  | xargs -0 command grep -n -E 'until M54 lands|true when it lands|built by M54|once M54 lands'
```

At the base (`git grep -n -E '<same pattern>' f7db572a -- <same set>`) it matches five lines: `release.md:3` (*built by M54 … until M54 lands*), `milestone-completion-workflow.md:24` (*true when it lands*), `completions/trial-harness/README.md:56` (*built by M54*), `CLAUDE.md:73` (*built by M54*) and `decisions-pending.md:155` (*once M54 lands*). The first three are flipped below. **After, it matches two lines, and the record keeps both:**

- `CLAUDE.md:73` matches *settled at M54's planning, built by M54*. That is now a completed fact, since CLAUDE.md:7 states M54 built and not audited. T4 owned CLAUDE.md and left the line.
- `implementation/decisions-pending.md:155` matches *So once M54 lands, M55 must not seed this row*. That Settle-time note is followed on the same line by *[Landed 2026-09-30 (M54 Increment 6, T2)]*, which states the fact. Its instruction to M55 still applies.

## Every line, disposed

**106 kept and 9 flipped**, over the 115 lines of the after tree. Two of the kept lines are T1's and T2's own flips (`release.md:108`, `dev-workflow.md:42`), which are already in their final wording. *past* means dated or past-tense history that is true today.

| Line | Disposition | New wording, or the reason it is kept |
|---|---|---|
| `CLAUDE.md:7` | **kept** | current truth T4 wrote: M54 built and not audited, rc.22 published, what comes next |
| `CLAUDE.md:10` | **kept** | past — dated or past-tense history, true today (the probe prebuild *retired at M54*) |
| `CLAUDE.md:14` | **kept** | current shape (*since M54*: the guides live in the crate) |
| `CLAUDE.md:16` | **kept** | current shape (*since M54* the packs' one home; *M54 also moved* the probe) |
| `CLAUDE.md:73` | **kept** | marker *built by M54* — kept: *settled at M54's planning, built by M54* is now a completed fact (line 7 states M54 built and not audited), and T4 owned CLAUDE.md |
| `VISION.md:214` | **kept** | idea index, dated *extended 2026-09-28*; *M54's restructure as its evidence* is history |
| `completions/trial-harness/README.md:56` | **flipped** | heading *(M54 — settled 2026-09-28, built by M54)* → *(M54 — settled 2026-09-28, built in Increment 7)* |
| `completions/trial-harness/README.md:58` | **flipped** | *M54 changes … the probe moves … the version moves … build-image.sh gains a mode … learns the layout* → *changed … moved … moved … gained … learned* |
| `completions/trial-harness/README.md:64` | **kept** | landed record (*built as described (M54 Increment 7)*, the Increment 11 positive registry run) plus a citation path |
| `completions/trial-harness/README.md:83` | **kept** | current behaviour (*every image built before M54's harness* fails outright) |
| `design/assistant-adapter.md:33` | **kept** | current shape (*since M54* the guides and packs live inside the crate) |
| `design/assistant-adapter.md:44` | **kept** | current rule (a range reaching back past M54 adds the guides' pre-move paths) |
| `design/assistant-adapter.md:56` | **kept** | past — dated or past-tense history, true today (*M54 removed it by construction … landed 2026-09-29, Increment 2*) |
| `design/assistant-adapter.md:60` | **kept** | past — dated or past-tense history, true today (*Until M54 … since M54 Increment 2*) |
| `design/bootstrap.md:108` | **kept** | current shape (*since M54 put the probe inside jigc*) |
| `design/project-setup.md:144` | **kept** | current behaviour, landed at M54 Increment 4 (*It holds after a failed run too (M54, S22)*) |
| `design/project-setup.md:158` | **kept** | past — dated or past-tense history, true today (*settled 2026-09-28, landed 2026-09-29 in Increment 2*; the superseded bracket kept as the record) |
| `design/project-setup.md:160` | **kept** | past — dated or past-tense history, true today (*Since M54 Increment 2 (2026-09-29)*) |
| `design/storage.md:268` | **kept** | past — dated or past-tense history, true today (*Corrected 2026-09-28 (M54 Settle, S3)*; *From M54 the engine is versioned independently* is true — jigc-engine 0.1.0-rc.1) |
| `design/validation.md:230` | **kept** | past — dated or past-tense history, true today (*Revised 2026-09-28 …, landed 2026-09-29 (M54 Increment 2)*) |
| `design/validation.md:320` | **kept** | current shape (the probe's sources *live at crates/cli/src/doc_code_probe/ since M54*) |
| `design/validation.md:352` | **kept** | past — dated or past-tense history, true today (*M54 (settled 2026-09-28, landed 2026-09-29 in Increment 2) removed the question*) |
| `design/worked-examples.md:2024` | **kept** | past — dated or past-tense history, true today (*Since M54 (settled …, landed 2026-09-29 in Increment 2)*) |
| `design/worked-examples.md:2073` | **kept** | past — dated or past-tense history, true today (same bracket, flow 30) |
| `design/worked-examples.md:2122` | **kept** | past — dated or past-tense history, true today (same bracket, flow 31) |
| `design/worked-examples.md:2172` | **kept** | past — dated or past-tense history, true today (same bracket, the production path) |
| `ideas/symbol-mention-sweep.md:3` | **kept** | past — dated or past-tense history, true today (*Extended 2026-09-28 … from M54's restructure*) |
| `ideas/symbol-mention-sweep.md:23` | **kept** | section heading dated *(added 2026-09-28, M54's Settle)* |
| `ideas/symbol-mention-sweep.md:27` | **flipped** | *M54 moves both packs* → *M54 moved both packs*; the rest of the line (the Settle's census, *M54 answers it … with a gate test*, the S15 citation) is kept — the link fence stands |
| `ideas/symbol-mention-sweep.md:31` | **kept** | current (*whatever of it M54's repo test does not cover* is M56's to find) |
| `implementation/decisions-pending.md:22` | **kept** | section heading naming the four milestones of the road |
| `implementation/decisions-pending.md:24` | **kept** | current pointer to the roadmap's M54 charter |
| `implementation/decisions-pending.md:26` | **kept** | graduated section head (*All seven rows graduated 2026-09-28 at M54's Settle*; *Each row's text is kept as recorded*) |
| `implementation/decisions-pending.md:28` | **kept** | graduated row, kept as recorded under :26's convention (its *Trigger: M54, its first act* is the row as recorded) |
| `implementation/decisions-pending.md:29` | **kept** | graduated row, kept as recorded (dissolved by S4; *Trigger: M54* as recorded) |
| `implementation/decisions-pending.md:30` | **kept** | graduated row; its head bracket *M54 closes with jigc 1.0.0-rc.22 … on crates.io* is the Settle's decision, now fact (publish-proof.md, 2026-10-01) |
| `implementation/decisions-pending.md:31` | **kept** | graduated row, kept as recorded (S2 · S11 · S16; landed at Increment 3) |
| `implementation/decisions-pending.md:32` | **kept** | graduated row, kept as recorded (S9; *Trigger: M54* as recorded) |
| `implementation/decisions-pending.md:33` | **kept** | graduated row; its head bracket *the root README.md lands in M54* is the Settle's decision, landed 2026-09-30 at Increment 6 T3 |
| `implementation/decisions-pending.md:34` | **kept** | graduated row, kept as recorded (S10; dev/runner-faithful committed at Increment 7) |
| `implementation/decisions-pending.md:38` | **kept** | past — dated or past-tense history, true today (*Recorded 2026-10-01 (M54 Increment 11, T4)*) plus a citation path; the fork it keys is open |
| `implementation/decisions-pending.md:50` | **kept** | past — dated or past-tense history, true today (*Noted 2026-09-28 (M54 Settle, S18)*) |
| `implementation/decisions-pending.md:59` | **kept** | dated record (S9); *From M54 on, release-plz writes a changelog per crate* is true — PR #1 carried both |
| `implementation/decisions-pending.md:60` | **kept** | dated record (S15); the link fence it names exists (Increment 3 T9); the product question stays open |
| `implementation/decisions-pending.md:64` | **kept** | dated record (S3); *M54 versions jigc-engine independently at 0.x* is true (0.1.0-rc.1) |
| `implementation/decisions-pending.md:65` | **kept** | dated record (I7); the deny list it names landed at Increment 9 T5 |
| `implementation/decisions-pending.md:69` | **kept** | dated record (S22); *M54 fixes the setup wedge* landed at Increment 4; the uninstall half stays keyed to M57 |
| `implementation/decisions-pending.md:74` | **kept** | graduated row, kept as recorded (S1) |
| `implementation/decisions-pending.md:75` | **kept** | past — dated or past-tense history, true today (*Recorded 2026-09-30, between M54 Increments 9 and 10*) |
| `implementation/decisions-pending.md:77` | **kept** | past — dated or past-tense history, true today (*M54 half LANDED 2026-09-30 (M54 Increment 7, T2)*; the trailing trigger is the row as recorded) |
| `implementation/decisions-pending.md:97` | **kept** | superseded bracket, dated 2026-09-27, kept as the record |
| `implementation/decisions-pending.md:155` | **kept** | marker *once M54 lands* — kept: the Settle-time note (2026-09-28) is followed on the same line by *[Landed 2026-09-30 (M54 Increment 6, T2)]*, which states the fact |
| `implementation/decisions-pending.md:166` | **kept** | past — dated or past-tense history, true today (*Corrected 2026-09-29 (M54 Increment 2, T5)*) |
| `implementation/decisions-pending.md:317` | **kept** | past — dated or past-tense history, true today (*Dissolved 2026-09-29 by M54 Increment 2*) |
| `implementation/decisions-pending.md:398` | **kept** | settled D3: the blind trial runs on the release candidate that carries M54 and M55 — still owed, since M55 is not built |
| `implementation/decisions-pending.md:739` | **kept** | current path (*since M54 crates/cli/src/doc_code_probe/*) |
| `implementation/decisions-pending.md:818` | **kept** | graduated row; the *REVERSED 2026-09-27* bracket is dated and kept as recorded; the landing is stated at :971 |
| `implementation/decisions-pending.md:942` | **kept** | superseded bracket (*SUPERSEDED 2026-09-28 (M54 Settle, S4)*), kept as the record |
| `implementation/decisions-pending.md:971` | **flipped** | *`README.md` and the `publish` flag are what M54 still takes.* kept as recorded, followed by *[Both landed 2026-09-30 (M54 Increment 6): the root README.md at T3, the publish flag at T4, and the first publish followed on 2026-10-01 (release.md → Publishing).]* |
| `implementation/dev-workflow.md:19` | **kept** | past — dated or past-tense history, true today (*Retired 2026-09-29 (M54 Increment 2)*) |
| `implementation/dev-workflow.md:42` | **kept** | flipped by T2 (the measured after run, 430 s on four CPUs); M54 S8 and two citation paths |
| `implementation/dev-workflow.md:54` | **kept** | the struck candidate is kept as recorded; its *[Decided 2026-09-28 …, landed at M54 Increment 7]* bracket states the fact (*the install acceptance M54 adds* ran at Increments 7 and 11) |
| `implementation/language-runtime.md:18` | **kept** | past — dated or past-tense history, true today (*Held literally again since M54 — … landed 2026-09-29 in Increment 2*) |
| `implementation/language-runtime.md:44` | **kept** | past — dated or past-tense history, true today (*Since M54 — landed 2026-09-29, Increment 2*) |
| `implementation/machine-setup.md:32` | **kept** | past — dated or past-tense history, true today (*Retired 2026-09-29 (M54 Increment 2)*) |
| `implementation/milestone-completion-workflow.md:24` | **flipped** | *[From M54 — settled 2026-09-28, true when it lands — the stamp is no longer a hand edit.* → *[Since M54 — settled 2026-09-28, true since the first release PR was merged on 2026-10-01 (PR #1, `jigc 1.0.0-rc.22`) — the stamp is no longer a hand edit.* |
| `implementation/milestone-completion-workflow.md:44` | **kept** | past — dated or past-tense history, true today (*one since M54 Increment 2, 2026-09-29*) |
| `implementation/milestone-completion-workflow.md:48` | **kept** | current rule (*It names no installed version (since M54)*) |
| `implementation/module-layout.md:10` | **kept** | current shape (*since M54* the probe's module tree lives in the bin) |
| `implementation/module-layout.md:12` | **kept** | current shape, dated (*current since Increment 2 … since Increment 5*) |
| `implementation/module-layout.md:33` | **kept** | current: the seam is prepared, the pack-builtin crate is not (its extraction stays triggered by a second frontend); *moved there at M54* is history |
| `implementation/module-layout.md:41` | **kept** | past — dated or past-tense history, true today (*until M54 moved it into the jigc bin*) |
| `implementation/module-layout.md:44` | **kept** | past — dated or past-tense history, true today (*M54 moved it there*; the include_bytes! pattern *retires at M54* — it did, at Increment 2) |
| `implementation/module-layout.md:75` | **kept** | current rule (*since M54* the bundled probe is the exception) |
| `implementation/module-layout.md:76` | **kept** | past — dated or past-tense history, true today (*retired 2026-09-29 (M54 Increment 2)*) |
| `implementation/module-layout.md:77` | **kept** | past — dated or past-tense history, true today (*settled 2026-09-28, landed 2026-09-29 in Increment 2*) |
| `implementation/module-layout.md:81` | **kept** | past — dated or past-tense history, true today (*until M54 kept stdout*; *the stale-probe silent pass the M54 baseline drove*) |
| `implementation/module-layout.md:82` | **kept** | citation of *M54 settled*, S1 |
| `implementation/module-layout.md:83` | **kept** | past — dated or past-tense history, true today (*the 39 suites that each built a probe of their own until M54*) |
| `implementation/module-layout.md:84` | **kept** | past — dated or past-tense history, true today (*the M54 baseline's S3* went by construction) |
| `implementation/module-layout.md:86` | **kept** | superseded bracket (*landed 2026-09-29 (Increment 2)*), kept as the record |
| `implementation/module-layout.md:90` | **kept** | past — dated or past-tense history, true today (*Since M54 Increment 5 … Since M54 Increment 2*) |
| `implementation/pinning.md:18` | **kept** | current rule (*since M54, when the release pipeline took the version bump*) |
| `implementation/public-hygiene.md:3` | **kept** | citation path (completions/artifacts/M54/pre-public-audit.md) |
| `implementation/release.md:3` | **flipped** | the header: *and **built by M54** (roadmap.md → M54): until M54 lands, nothing on this page runs — the version stamp is still the hand edit the milestone close performs, and no crate has been published from this repository.* → *and **built in M54** (roadmap.md → M54): the version stamp is the release PR's, no longer the hand edit the milestone close performed (→ The release PR), and the first versions its release pipeline published are `jigc 1.0.0-rc.22` and `jigc-engine 0.1.0-rc.1`, on 2026-10-01 (publish-proof.md).*; and *`dev/runner-faithful`, which M54 commits beside dev/gate* → *`dev/runner-faithful`, committed beside dev/gate at M54 Increment 7*. The citation of *M54 settled* is kept |
| `implementation/release.md:5` | **flipped** | *an implementation pick M54 makes and records here when it lands* → *an implementation pick M54's build made and recorded here* |
| `implementation/release.md:14` | **kept** | past — dated or past-tense history, true today (*Corrected 2026-09-30, M54 Increment 5*) |
| `implementation/release.md:25` | **kept** | past — dated or past-tense history, true today (*pinned 2026-09-30 (M54 Increment 5)*) |
| `implementation/release.md:32` | **kept** | past — dated or past-tense history, true today (*Pinned by the build (M54 Increment 9, 2026-09-30)*) |
| `implementation/release.md:37` | **kept** | past — dated or past-tense history, true today (*pinned by the build (M54 Increment 9, 2026-09-30)*) |
| `implementation/release.md:52` | **kept** | past — dated or past-tense history, true today (*Driven on a scratch clone (M54 Increment 9 T2)*) |
| `implementation/release.md:60` | **kept** | past — dated or past-tense history, true today (*Corrected 2026-09-30, M54 Increment 9*) |
| `implementation/release.md:61` | **kept** | past — dated or past-tense history, true today (*verified by the crates.io API on 2026-09-30 (M54 Increment 9)*) plus a citation path |
| `implementation/release.md:64` | **kept** | past — dated or past-tense history, true today (*Rehearsed 2026-09-30 … (M54 Increment 10)*) plus a citation path |
| `implementation/release.md:65` | **flipped** | *M54 closes with a real publish of jigc 1.0.0-rc.22 plus jigc-engine 0.1.0-rc.1; M55 publishes the next.* → *M54's pipeline made its first real publish, jigc 1.0.0-rc.22 plus jigc-engine 0.1.0-rc.1, on 2026-10-01 (publish-proof.md); M55 publishes the next.* |
| `implementation/release.md:68` | **kept** | past — dated or past-tense history, true today (*pinned by the build (M54 Increment 9, 2026-09-30)*) |
| `implementation/release.md:80` | **kept** | past — dated or past-tense history, true today (*Observed … on 2026-09-30*) — citation path only |
| `implementation/release.md:81` | **kept** | past — dated or past-tense history, true today (*Measured by the rehearsal (2026-09-30)*) — citation path only |
| `implementation/release.md:92` | **kept** | past — dated or past-tense history, true today (*pinned 2026-09-30 (M54 Increment 6)*) |
| `implementation/release.md:94` | **kept** | past — dated or past-tense history, true today (*verified at the first publish (2026-10-01, M54 Increment 11)*) plus a citation path; the fix stays keyed |
| `implementation/release.md:98` | **kept** | past — dated or past-tense history, true today (*Corrected 2026-09-30, M54 Increment 6*) |
| `implementation/release.md:104` | **kept** | past — dated or past-tense history, true today (*Proved 2026-10-01 (M54 Increment 11)*; *Pinned by the build (M54 Increment 6)*) plus a citation path |
| `implementation/release.md:105` | **kept** | past — dated or past-tense history, true today (*landed 2026-09-30 (M54 Increment 6), inside M54's one guide batch*) |
| `implementation/release.md:107` | **kept** | current behaviour, landed at Increment 4 (*Until M54 … M54 records the install footprint on every failure path*) |
| `implementation/release.md:108` | **kept** | flipped by T1 (*Verified 2026-10-01 (M54 Increment 12)*) plus a citation path |
| `implementation/release.md:112` | **flipped** | *the two installs M54 runs prove different things.* → *the two installs prove different things. M54 ran both: the tarball install before the publish (Increment 7) and the registry install after it (Increment 11).* |
| `implementation/release.md:114` | **kept** | past — dated or past-tense history, true today (*Pinned by the build (M54 Increment 7)*) |
| `implementation/release.md:120` | **kept** | past — dated or past-tense history, true today (*Pinned by the build (M54 Increment 7)*) |
| `implementation/release.md:124` | **kept** | past — dated or past-tense history, true today (*Driven after the first publish (2026-10-01 …)*) — citation path only |
| `implementation/release.md:130` | **kept** | past — dated or past-tense history, true today (*pinned by the build (M54 Increment 9, 2026-09-30)*) |
| `implementation/release.md:161` | **kept** | the settled order inside M54, stated as the decision (S5–S9); every step of it has run |
| `implementation/release.md:165` | **kept** | past — dated or past-tense history, true today (*Driven, and the finding is "none" (M54 Increment 10 T2)*) plus citation paths |
| `implementation/release.md:167` | **kept** | past — dated or past-tense history, true today (*when Increment 10 closed*) — citation path only |
| `implementation/release.md:176` | **kept** | known gap, *Acknowledged at M54's planning, not fixable by design* |
| `implementation/release.md:177` | **kept** | known gap, citation path only |

## Beyond the grep

One forward-tense line about M54 does not name `M54`, so the census does not count it. It was flipped in the same commit: **`implementation/release.md:121`**, in *Verifying a publish*. It read *the install Increment 12's entry expects to resolve nothing once the placeholders are yanked*, and it now reads *which resolves nothing now that both placeholders are yanked (→ Installing)*, where the yank is verified (`release.md:108`). A search of the set for `Increment 11` and `Increment 12` found no other forward-tense line about M54. The other hits are dated history or another milestone's increments.

## Bounds

- **The pattern is the literal `M54`.** A line about M54 that names only an increment or a settle id (S1–S22) is not counted. The section above is the one such line found by the increment search, and it is not a full sweep of the S-ids.
- **A kept line is judged true today, not re-proved here.** Each reason names the dated bracket or the landed increment it rests on. The facts behind them are the increments' own records in `DECISIONS.md` and this directory.
