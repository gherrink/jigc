# Independent judge verdict — OpenAI Codex (gpt-5.5), blind to arm identity

Sets: A = arm S (static, by hand), B = arm J (jigc) — mapping revealed only after judging.

**Verdict**

Set A has **0 defect classes PRESENT**.  
Set B has **1 defect class PRESENT**: D3.

Set A has fewer committed structural-integrity defects. The only differing class is **D3**: Set B failed to update the superseded ADR’s status.

**Set A**

| Class | Verdict | Evidence |
|---|---:|---|
| D1 doc misplaced | CLEAN | Files are under `/tmp/ngt-judge/set-A/architecture/` and `/tmp/ngt-judge/set-A/decisions/`. |
| D2 supersedes wrong/dangling | CLEAN | New ADR line: `/tmp/ngt-judge/set-A/decisions/absorb-advances-baseline-only-at-a-landed-finalize.md:4:supersedes: absorb-advances-baseline-on-every-sweep`; target exists as `/tmp/ngt-judge/set-A/decisions/absorb-advances-baseline-on-every-sweep.md`. |
| D3 superseded ADR not updated | CLEAN | Old ADR line: `/tmp/ngt-judge/set-A/decisions/absorb-advances-baseline-on-every-sweep.md:2:status: superseded`. |
| D4 cites dangling | CLEAN | Arch cites line: `/tmp/ngt-judge/set-A/architecture/reconciliation-and-file-state.md:2:cites: out-of-band-edits-are-detected-and-routed, absorb-advances-baseline-only-at-a-landed-finalize, drift-hash-is-over-raw-bytes`; all three corresponding files exist in `decisions/`. |
| D5 implemented-by unanchored | CLEAN | All component anchors exist; grep evidence below. |
| D6 missing required structure | CLEAN | Overview exists at line 7; every component has prose plus `implemented-by` at lines 50, 56, 62, 70, 77, 85, 93. |
| D7 other dangling/internal inconsistency | CLEAN | None found. |

Set A D5 symbol evidence:

| Component | Claimed symbol | Exists |
|---|---|---|
| Raw-byte drift hash | `hash_bytes` | Yes: `crates/engine/src/file_state.rs:41:pub fn hash_bytes(bytes: &[u8]) -> String {` |
| File-state baseline record | `FileStateRecord` | Yes: `crates/engine/src/file_state.rs:54:pub struct FileStateRecord {` |
| File-state probe | `file_state` | Yes: `crates/engine/src/file_state.rs:133:pub fn file_state(...)` |
| OOB reconciliation classifier | `reconcile_committed` | Yes: `crates/engine/src/file_state.rs:178:pub fn reconcile_committed(` |
| Committed-store sweep | `reconcile_committed_store` | Yes: `crates/engine/src/file_state.rs:251:pub fn reconcile_committed_store(` |
| Rename detection | `detect_rename` | Yes: `crates/engine/src/file_state.rs:439:pub fn detect_rename(` |
| Landed-finalize baseline persistence | `advance_file_state` | Yes: `crates/cli/src/task.rs:865:fn advance_file_state(` |

**Set B**

| Class | Verdict | Evidence |
|---|---:|---|
| D1 doc misplaced | CLEAN | Files are under `/tmp/ngt-judge/set-B/architecture/` and `/tmp/ngt-judge/set-B/decisions/`. |
| D2 supersedes wrong/dangling | CLEAN | New ADR line: `/tmp/ngt-judge/set-B/decisions/absorb-advances-the-file-state-baseline-only-at-a.md:2:supersedes: adr:absorb-advances-baseline-on-every-sweep`; resolving the `adr:` id qualifier gives existing target `/tmp/ngt-judge/set-B/decisions/absorb-advances-baseline-on-every-sweep.md`. |
| D3 superseded ADR not updated | **PRESENT** | Old ADR still says accepted: `/tmp/ngt-judge/set-B/decisions/absorb-advances-baseline-on-every-sweep.md:2:status: accepted`. |
| D4 cites dangling | CLEAN | Arch cites line: `/tmp/ngt-judge/set-B/architecture/reconciliation-and-file-state-subsystem.md:2:cites: [adr:out-of-band-edits-are-detected-and-routed, adr:absorb-advances-baseline-on-every-sweep, adr:absorb-advances-the-file-state-baseline-only-at-a]`; all resolve to existing files after stripping `adr:`. |
| D5 implemented-by unanchored | CLEAN | All component anchors exist; grep evidence below. |
| D6 missing required structure | CLEAN | Overview exists at line 7; every component has prose plus `implemented-by` at lines 50, 61, 73, 86, 100, 114, 128. |
| D7 other dangling/internal inconsistency | CLEAN | None found. |

Set B D5 symbol evidence:

| Component | Claimed symbol | Exists |
|---|---|---|
| Raw-byte drift hash | `crates/engine/src/file_state.rs#hash_bytes` | Yes: `crates/engine/src/file_state.rs:41:pub fn hash_bytes(bytes: &[u8]) -> String {` |
| File-state baseline record | `crates/engine/src/file_state.rs#FileStateRecord` | Yes: `crates/engine/src/file_state.rs:54:pub struct FileStateRecord {` |
| The file-state probe | `crates/engine/src/file_state.rs#file_state` | Yes: `crates/engine/src/file_state.rs:133:pub fn file_state(...)` |
| OOB reconciliation classifier | `crates/engine/src/file_state.rs#reconcile_committed` | Yes: `crates/engine/src/file_state.rs:178:pub fn reconcile_committed(` |
| Committed-store sweep | `crates/engine/src/file_state.rs#reconcile_committed_store` | Yes: `crates/engine/src/file_state.rs:251:pub fn reconcile_committed_store(` |
| Rename detection | `crates/engine/src/file_state.rs#detect_rename` | Yes: `crates/engine/src/file_state.rs:439:pub fn detect_rename(` |
| Baseline persistence at landed finalize | `crates/cli/src/task.rs#advance_file_state` | Yes: `crates/cli/src/task.rs:865:fn advance_file_state(` |

**Defect Counts**

Set A: **0 / 7 defect classes PRESENT**.  
Set B: **1 / 7 defect classes PRESENT**: D3. No multiple-instance defect class found.

Direct comparison: **Set A is structurally cleaner**. The sets differ only on **D3**: Set A correctly marks the old ADR `superseded`; Set B leaves it `accepted`.
tokens used
27,774
**Verdict**

Set A has **0 defect classes PRESENT**.  
Set B has **1 defect class PRESENT**: D3.

Set A has fewer committed structural-integrity defects. The only differing class is **D3**: Set B failed to update the superseded ADR’s status.

**Set A**

| Class | Verdict | Evidence |
|---|---:|---|
| D1 doc misplaced | CLEAN | Files are under `/tmp/ngt-judge/set-A/architecture/` and `/tmp/ngt-judge/set-A/decisions/`. |
| D2 supersedes wrong/dangling | CLEAN | New ADR line: `/tmp/ngt-judge/set-A/decisions/absorb-advances-baseline-only-at-a-landed-finalize.md:4:supersedes: absorb-advances-baseline-on-every-sweep`; target exists as `/tmp/ngt-judge/set-A/decisions/absorb-advances-baseline-on-every-sweep.md`. |
| D3 superseded ADR not updated | CLEAN | Old ADR line: `/tmp/ngt-judge/set-A/decisions/absorb-advances-baseline-on-every-sweep.md:2:status: superseded`. |
| D4 cites dangling | CLEAN | Arch cites line: `/tmp/ngt-judge/set-A/architecture/reconciliation-and-file-state.md:2:cites: out-of-band-edits-are-detected-and-routed, absorb-advances-baseline-only-at-a-landed-finalize, drift-hash-is-over-raw-bytes`; all three corresponding files exist in `decisions/`. |
| D5 implemented-by unanchored | CLEAN | All component anchors exist; grep evidence below. |
| D6 missing required structure | CLEAN | Overview exists at line 7; every component has prose plus `implemented-by` at lines 50, 56, 62, 70, 77, 85, 93. |
| D7 other dangling/internal inconsistency | CLEAN | None found. |

Set A D5 symbol evidence:

| Component | Claimed symbol | Exists |
|---|---|---|
| Raw-byte drift hash | `hash_bytes` | Yes: `crates/engine/src/file_state.rs:41:pub fn hash_bytes(bytes: &[u8]) -> String {` |
| File-state baseline record | `FileStateRecord` | Yes: `crates/engine/src/file_state.rs:54:pub struct FileStateRecord {` |
| File-state probe | `file_state` | Yes: `crates/engine/src/file_state.rs:133:pub fn file_state(...)` |
| OOB reconciliation classifier | `reconcile_committed` | Yes: `crates/engine/src/file_state.rs:178:pub fn reconcile_committed(` |
| Committed-store sweep | `reconcile_committed_store` | Yes: `crates/engine/src/file_state.rs:251:pub fn reconcile_committed_store(` |
| Rename detection | `detect_rename` | Yes: `crates/engine/src/file_state.rs:439:pub fn detect_rename(` |
| Landed-finalize baseline persistence | `advance_file_state` | Yes: `crates/cli/src/task.rs:865:fn advance_file_state(` |

**Set B**

| Class | Verdict | Evidence |
|---|---:|---|
| D1 doc misplaced | CLEAN | Files are under `/tmp/ngt-judge/set-B/architecture/` and `/tmp/ngt-judge/set-B/decisions/`. |
| D2 supersedes wrong/dangling | CLEAN | New ADR line: `/tmp/ngt-judge/set-B/decisions/absorb-advances-the-file-state-baseline-only-at-a.md:2:supersedes: adr:absorb-advances-baseline-on-every-sweep`; resolving the `adr:` id qualifier gives existing target `/tmp/ngt-judge/set-B/decisions/absorb-advances-baseline-on-every-sweep.md`. |
| D3 superseded ADR not updated | **PRESENT** | Old ADR still says accepted: `/tmp/ngt-judge/set-B/decisions/absorb-advances-baseline-on-every-sweep.md:2:status: accepted`. |
| D4 cites dangling | CLEAN | Arch cites line: `/tmp/ngt-judge/set-B/architecture/reconciliation-and-file-state-subsystem.md:2:cites: [adr:out-of-band-edits-are-detected-and-routed, adr:absorb-advances-baseline-on-every-sweep, adr:absorb-advances-the-file-state-baseline-only-at-a]`; all resolve to existing files after stripping `adr:`. |
| D5 implemented-by unanchored | CLEAN | All component anchors exist; grep evidence below. |
| D6 missing required structure | CLEAN | Overview exists at line 7; every component has prose plus `implemented-by` at lines 50, 61, 73, 86, 100, 114, 128. |
| D7 other dangling/internal inconsistency | CLEAN | None found. |

Set B D5 symbol evidence:

| Component | Claimed symbol | Exists |
|---|---|---|
| Raw-byte drift hash | `crates/engine/src/file_state.rs#hash_bytes` | Yes: `crates/engine/src/file_state.rs:41:pub fn hash_bytes(bytes: &[u8]) -> String {` |
| File-state baseline record | `crates/engine/src/file_state.rs#FileStateRecord` | Yes: `crates/engine/src/file_state.rs:54:pub struct FileStateRecord {` |
| The file-state probe | `crates/engine/src/file_state.rs#file_state` | Yes: `crates/engine/src/file_state.rs:133:pub fn file_state(...)` |
| OOB reconciliation classifier | `crates/engine/src/file_state.rs#reconcile_committed` | Yes: `crates/engine/src/file_state.rs:178:pub fn reconcile_committed(` |
| Committed-store sweep | `crates/engine/src/file_state.rs#reconcile_committed_store` | Yes: `crates/engine/src/file_state.rs:251:pub fn reconcile_committed_store(` |
| Rename detection | `crates/engine/src/file_state.rs#detect_rename` | Yes: `crates/engine/src/file_state.rs:439:pub fn detect_rename(` |
| Baseline persistence at landed finalize | `crates/cli/src/task.rs#advance_file_state` | Yes: `crates/cli/src/task.rs:865:fn advance_file_state(` |

**Defect Counts**

Set A: **0 / 7 defect classes PRESENT**.  
Set B: **1 / 7 defect classes PRESENT**: D3. No multiple-instance defect class found.

Direct comparison: **Set A is structurally cleaner**. The sets differ only on **D3**: Set A correctly marks the old ADR `superseded`; Set B leaves it `accepted`.
