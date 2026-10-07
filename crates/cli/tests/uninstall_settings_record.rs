//! **`jigc uninstall` takes back the settings entries `jigc setup` added, and no entry
//! that merely equals one** (the human's ruling of 2026-10-06 on the rc.24 fix pass's item
//! 21 — `DECISIONS.md` → *Ahead of the run's opening*, 5b, 10, and the entry that built it).
//!
//! The loss, driven on `1.0.0-rc.24` and on the pass's own tree: `.claude/settings.json`
//! already holds entries identical to ones jigc installs — the adopter's own
//! `Bash(git add:*)` permit, their own `Bash(rm -rf:*)` and `Read(./.env)` deny rules,
//! their own `SessionStart` hook running `jigc start`. `setup` finds them present and adds
//! only what is missing; `uninstall` then removed every entry that *equals* one jigc
//! installs, the adopter's included — three removals, one per kind. Where git tracks the
//! settings file that is a modified file; where git ignores it the entries are in no git
//! object.
//!
//! **The rule.** `setup` records the entries it added in one small committed file,
//! `.jigc/settings-entries.json`, a member of the install commit. `uninstall` consults it
//! **only where git tracks the settings file** — the record and the file it describes then
//! travel together — and removes exactly the entries it lists. Everywhere else it removes
//! none of the three kinds and names what it left: a settings file git ignores or does not
//! track, and an install with no record. `--force` is the operator's consent, and takes
//! every entry identical to one jigc installs, as the door did before.
//!
//! The cells, each driving the real binary (`CARGO_BIN_EXE_jigc`) over a throwaway
//! `git init`:
//!
//! 1. tracked — exactly what `setup` added goes, per kind, and the identical entries stay;
//! 2. tracked, only jigc's entries — all removed, as before;
//! 3. ignored — nothing is recorded, nothing removed, everything named;
//! 4. an install with no record, and the upgrade over it — never a claim over an entry no
//!    run added;
//! 5. tracked ↔ ignored between `setup` and `uninstall`, and the re-run that takes a
//!    stale record away;
//! 6. the must-keep cells beside the removals — one character's difference, entries added
//!    after `setup`, a twin of one jigc added;
//! 7. `--force`;
//! 8. a second clone — the record travelled with the file;
//! 9. the record itself: its form, its place in the install commit, a byte-stable re-run.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The settings file the Claude Code profile merges into.
const SETTINGS: &str = ".claude/settings.json";

/// The record of the entries `jigc setup` added to it.
const RECORD: &str = ".jigc/settings-entries.json";

/// An adopter's own settings, written before jigc ever ran: one entry of each kind that is
/// **identical** to one jigc installs, beside entries that are theirs alone.
const OWN: &str = r#"{
  "permissions": {
    "allow": ["Bash(git add:*)", "Bash(make:*)"],
    "deny": ["Bash(rm -rf:*)", "Read(./.env)", "Bash(sudo:*)"]
  },
  "hooks": {
    "SessionStart": [ { "hooks": [ { "type": "command", "command": "jigc start" } ] } ]
  }
}
"#;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-settings-record-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
        fs::create_dir_all(&path).expect("create temp dir");
        TempDir(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Run `git` in `repo`, asserting success, returning trimmed stdout.
fn git(repo: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

/// A real `git init` with a per-repo identity and one seed commit.
fn born_repo(tag: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new(tag);
    let home = TempDir::new(&format!("{tag}-home"));
    git(repo.path(), &["init", "-q"]);
    git(repo.path(), &["config", "user.email", "test@example.com"]);
    git(repo.path(), &["config", "user.name", "Test"]);
    write(repo.path(), "README.md", "hello\n");
    git(repo.path(), &["add", "README.md"]);
    git(repo.path(), &["commit", "-q", "-m", "initial"]);
    (repo, home)
}

/// Write `contents` at `relative` under `repo`, creating parent directories.
fn write(repo: &Path, relative: &str, contents: &str) {
    let path = repo.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent");
    }
    fs::write(path, contents).expect("write file");
}

/// Read `relative` under `repo`.
fn read(repo: &Path, relative: &str) -> String {
    fs::read_to_string(repo.join(relative)).unwrap_or_default()
}

/// Run `jigc <args>` with `cwd = repo` and a temp `$HOME`, composing the embedded packs.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run jigc")
}

/// Run `jigc <args>` and hold it to exit 0, returning the output.
fn jigc_ok(repo: &Path, home: &Path, args: &[&str], what: &str) -> std::process::Output {
    let out = jigc(repo, home, args);
    assert_eq!(
        out.status.code(),
        Some(0),
        "{what}: `jigc {}` exits 0: {}",
        args.join(" "),
        said(&out)
    );
    out
}

/// stdout + stderr of an invocation, joined for message assertions.
fn said(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

/// The settings file, parsed.
fn settings(repo: &Path) -> serde_json::Value {
    serde_json::from_str(&read(repo, SETTINGS)).expect("the settings file parses")
}

/// The record, parsed — or `None` where no record stands.
fn record(repo: &Path) -> Option<serde_json::Value> {
    repo.join(RECORD)
        .exists()
        .then(|| serde_json::from_str(&read(repo, RECORD)).expect("the record parses"))
}

/// The strings of one `permissions` array of a parsed settings file.
fn listed(settings: &serde_json::Value, key: &str) -> Vec<String> {
    settings["permissions"][key]
        .as_array()
        .map(|entries| {
            entries
                .iter()
                .filter_map(|entry| entry.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

/// How many `SessionStart` commands in a parsed settings file run `jigc start`.
fn start_hooks(settings: &serde_json::Value) -> usize {
    settings["hooks"]["SessionStart"]
        .as_array()
        .map(|matchers| {
            matchers
                .iter()
                .filter_map(|matcher| matcher["hooks"].as_array())
                .flatten()
                .filter(|command| command["command"] == "jigc start")
                .count()
        })
        .unwrap_or(0)
}

/// The entries the shipped profile installs, read off the profile rather than restated.
fn installs() -> cli::adapter::SettingsEntries {
    cli::adapter::SettingsEntries::installed_by(
        &cli::adapter::load_profile("claude-code").expect("the shipped profile loads"),
    )
}

/// How many entries of a parsed settings file are identical to one jigc installs.
fn jigc_shaped(settings: &serde_json::Value) -> usize {
    let installs = installs();
    listed(settings, "allow")
        .iter()
        .filter(|entry| installs.allow.contains(entry))
        .count()
        + listed(settings, "deny")
            .iter()
            .filter(|entry| installs.deny.contains(entry))
            .count()
        + start_hooks(settings)
}

/// The `removed` flags of a `jigc --format json uninstall` for the three settings kinds:
/// `(allowlist, hook, deny)`.
fn removed_flags(out: &std::process::Output) -> (bool, bool, bool) {
    let envelope: serde_json::Value =
        serde_json::from_slice(&out.stdout).expect("the uninstall envelope parses");
    let flag = |key: &str| envelope["removed"][key].as_bool().expect("a removed flag");
    (flag("allowlist"), flag("hook"), flag("deny"))
}

/// A born repository whose settings file holds [`OWN`], committed — or, with `ignored`,
/// present on disk and covered by a committed ignore rule.
fn repo_with_own_settings(tag: &str, ignored: bool) -> (TempDir, TempDir) {
    let (repo, home) = born_repo(tag);
    write(repo.path(), SETTINGS, OWN);
    if ignored {
        write(repo.path(), ".gitignore", ".claude/\n");
    }
    git(repo.path(), &["add", "-A"]);
    git(repo.path(), &["commit", "-q", "-m", "our own settings"]);
    (repo, home)
}

/// (1) **Tracked: `setup` then `uninstall` removes exactly the entries jigc added and
/// leaves every entry the adopter had, the identical ones included — for all three
/// kinds.** The red cell: on `1.0.0-rc.24` the adopter's `Bash(git add:*)`, both deny
/// rules and their own hook were gone afterwards.
#[test]
fn tracked_uninstall_removes_exactly_what_setup_added() {
    let (repo, home) = repo_with_own_settings("tracked", false);
    let (repo, home) = (repo.path(), home.path());
    let before = settings(repo);

    jigc_ok(repo, home, &["setup"], "setup");
    let installed = settings(repo);
    // The before-control: the install really did add what the adopter lacked.
    assert!(
        listed(&installed, "allow").contains(&"Bash(jigc:*)".to_string())
            && listed(&installed, "deny").contains(&"Bash(curl:*)".to_string())
            && start_hooks(&installed) == 1,
        "premise: jigc's entries were merged in, and the hook the adopter already had was \
         not doubled: {installed}"
    );

    // The record lists what the run added — and none of the four entries it found.
    let record = record(repo).expect("a run that added entries leaves a record");
    assert_eq!(record["format"], 1, "the record names its format: {record}");
    assert_eq!(record["file"], SETTINGS, "and the file it describes");
    let installs = installs();
    assert_eq!(
        record["added"]["allow"],
        serde_json::json!(["Bash(jigc:*)"]),
        "the permit jigc added; `Bash(git add:*)` was the adopter's"
    );
    let added_deny: Vec<String> = installs
        .deny
        .iter()
        .filter(|entry| !["Bash(rm -rf:*)", "Read(./.env)"].contains(&entry.as_str()))
        .cloned()
        .collect();
    assert_eq!(
        record["added"]["deny"],
        serde_json::json!(added_deny),
        "the floor patterns jigc added, without the two the adopter had"
    );
    assert_eq!(
        record["added"]["hooks"],
        serde_json::json!([]),
        "and no hook: the adopter's own already ran `jigc start`"
    );
    // It is a member of the install commit, beside the file it describes.
    let committed = git(repo, &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        committed.lines().any(|path| path == RECORD)
            && committed.lines().any(|path| path == SETTINGS),
        "the record and the settings file ride one install commit: {committed}"
    );
    assert_eq!(git(repo, &["status", "--porcelain"]), "", "committed whole");

    let out = jigc_ok(repo, home, &["--format", "json", "uninstall"], "uninstall");
    assert_eq!(
        settings(repo),
        before,
        "what the adopter had before `jigc setup` is what they have after `jigc uninstall` \
         — jigc removes the entries it added and no others"
    );
    assert_eq!(
        removed_flags(&out),
        (true, false, true),
        "a permit and floor patterns went; no hook did — jigc added none"
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("note: left 4 entries in `.claude/settings.json`")
            && stderr.contains("permissions.allow: `Bash(git add:*)`")
            && stderr.contains("permissions.deny: `Bash(rm -rf:*)`, `Read(./.env)`")
            && stderr.contains("hooks.SessionStart: `jigc start`")
            && stderr.contains(RECORD),
        "and the four it left are named, with the record that says they are not jigc's: \
         {stderr}"
    );
}

/// (2) **Tracked, with only jigc's entries: all of them are removed, as before.** The
/// must-not-keep cell — the record must not make the ordinary teardown leave anything.
#[test]
fn tracked_with_only_jigcs_entries_removes_them_all() {
    for own in [None, Some("{\n  \"model\": \"sonnet\"\n}\n")] {
        let (repo, home) = born_repo("only-jigcs");
        let (repo, home) = (repo.path(), home.path());
        if let Some(own) = own {
            write(repo, SETTINGS, own);
            git(repo, &["add", "-A"]);
            git(repo, &["commit", "-q", "-m", "our own settings"]);
        }
        jigc_ok(repo, home, &["setup"], "setup");
        let installs = installs();
        assert_eq!(
            jigc_shaped(&settings(repo)),
            installs.len(),
            "premise: every entry the profile installs is in the file"
        );
        let record = record(repo).expect("the record");
        assert_eq!(
            record["added"],
            serde_json::to_value(&installs).expect("the entries serialize"),
            "and the record lists every one of them"
        );

        let out = jigc_ok(repo, home, &["--format", "json", "uninstall"], "uninstall");
        assert_eq!(
            removed_flags(&out),
            (true, true, true),
            "all three kinds went: {}",
            said(&out)
        );
        let after = settings(repo);
        assert_eq!(
            jigc_shaped(&after),
            0,
            "no entry of jigc's is left: {after}"
        );
        if own.is_some() {
            assert_eq!(after["model"], "sonnet", "and the adopter's own key stays");
        }
        assert!(
            !String::from_utf8_lossy(&out.stderr).contains("left "),
            "nothing was left, so nothing is named: {}",
            said(&out)
        );
    }
}

/// (3) **Ignored: nothing is recorded, `uninstall` removes none of the three kinds, and
/// it names what it left** — jigc's entries and the adopter's alike, because in a file
/// git holds no copy of nothing can tell them apart, and the entries are in no git object.
#[test]
fn an_ignored_settings_file_keeps_every_entry_and_they_are_named() {
    let (repo, home) = repo_with_own_settings("ignored", true);
    let (repo, home) = (repo.path(), home.path());

    jigc_ok(repo, home, &["setup"], "setup");
    let installed = settings(repo);
    let installs = installs();
    assert_eq!(
        jigc_shaped(&installed),
        installs.len(),
        "premise: the merge ran — an ignored settings file is merged into"
    );
    assert!(
        record(repo).is_none(),
        "no record where the install commit cannot carry the settings file: there is \
         nothing to record that a clone could read against the same file"
    );

    let out = jigc_ok(repo, home, &["--format", "json", "uninstall"], "uninstall");
    assert_eq!(
        settings(repo),
        installed,
        "not one entry was removed — the adopter's four are where they were"
    );
    assert_eq!(removed_flags(&out), (false, false, false));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains(&format!(
            "warning: left {} entries in `.claude/settings.json`",
            installs.len()
        )) && stderr.contains("git does not track `.claude/settings.json`"),
        "the teardown says how many it left and why: {stderr}"
    );
    for entry in installs.allow.iter().chain(&installs.deny) {
        assert!(
            stderr.contains(&format!("`{entry}`")),
            "`{entry}` is named, so it can be removed by hand: {stderr}"
        );
    }
    assert!(
        stderr.contains("hooks.SessionStart: `jigc start`")
            && stderr.contains("jigc uninstall --force"),
        "the hook is named too, and the consent that takes them all: {stderr}"
    );

    // A second teardown is still a no-op for the file, and still says what is there.
    let again = jigc_ok(repo, home, &["uninstall"], "second uninstall");
    assert_eq!(settings(repo), installed, "byte-for-byte the same entries");
    assert!(said(&again).contains("warning: left "), "{}", said(&again));
}

/// Take the record out of a committed install, the way an install made by a jigc that
/// kept none stands: every entry in the settings file, no record in git.
fn forget_the_record(repo: &Path) {
    git(repo, &["rm", "-q", "--", RECORD]);
    git(
        repo,
        &["commit", "-q", "-m", "an install from before the record"],
    );
}

/// (4) **An install with no record is left in place and named — and a later `setup` over
/// it claims nothing it did not add.**
///
/// The upgrade, decided on purpose: the older install's entries are in the settings file
/// and no record lists them, so the new `setup` finds them present, adds nothing and
/// writes no record. Where the newer build's floor has a pattern the older one lacked,
/// that one pattern is added, recorded, and is the one `uninstall` takes back.
#[test]
fn an_install_without_a_record_is_left_and_an_upgrade_claims_only_what_it_adds() {
    // (a) No record: nothing removed, everything named.
    {
        let (repo, home) = born_repo("no-record");
        let (repo, home) = (repo.path(), home.path());
        jigc_ok(repo, home, &["setup"], "setup");
        forget_the_record(repo);
        let installed = settings(repo);

        let out = jigc_ok(repo, home, &["--format", "json", "uninstall"], "uninstall");
        assert_eq!(settings(repo), installed, "no entry was removed");
        assert_eq!(removed_flags(&out), (false, false, false));
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            stderr.contains("warning: left ")
                && stderr.contains("no record git tracks (`.jigc/settings-entries.json`)")
                && stderr.contains("`Bash(jigc:*)`"),
            "left, and named with the reason: {stderr}"
        );
    }

    // (b) The upgrade over it, with nothing new to add: no record is written, no commit
    //     is made, and the teardown is cell (a)'s.
    {
        let (repo, home) = born_repo("upgrade-same");
        let (repo, home) = (repo.path(), home.path());
        jigc_ok(repo, home, &["setup"], "the older install");
        forget_the_record(repo);
        let head = git(repo, &["rev-parse", "HEAD"]);
        let installed = settings(repo);

        jigc_ok(repo, home, &["setup"], "the upgrade");
        assert!(
            record(repo).is_none(),
            "the upgrade added no entry, so it records none — it never claims the older \
             install's"
        );
        assert_eq!(
            git(repo, &["rev-parse", "HEAD"]),
            head,
            "and commits nothing"
        );
        assert_eq!(git(repo, &["status", "--porcelain"]), "");

        jigc_ok(repo, home, &["uninstall"], "uninstall");
        assert_eq!(
            settings(repo),
            installed,
            "the older install's entries are left"
        );
    }

    // (c) The upgrade adds one pattern the older install lacked: that one is recorded and
    //     taken back; the rest are left and named.
    {
        let (repo, home) = born_repo("upgrade-adds");
        let (repo, home) = (repo.path(), home.path());
        jigc_ok(repo, home, &["setup"], "the older install");
        let mut older = settings(repo);
        older["permissions"]["deny"]
            .as_array_mut()
            .expect("the deny floor")
            .retain(|entry| entry != "Bash(wget:*)");
        write(
            repo,
            SETTINGS,
            &format!("{}\n", serde_json::to_string_pretty(&older).unwrap()),
        );
        git(repo, &["add", "--", SETTINGS]);
        forget_the_record(repo);

        jigc_ok(repo, home, &["setup"], "the upgrade");
        let record = record(repo).expect("the upgrade added an entry, so it records it");
        assert_eq!(
            record["added"],
            serde_json::json!({ "allow": [], "deny": ["Bash(wget:*)"], "hooks": [] }),
            "exactly the pattern this run added"
        );
        assert!(
            git(repo, &["show", "--name-only", "--format=", "HEAD"])
                .lines()
                .any(|path| path == RECORD),
            "committed with the install"
        );

        let out = jigc_ok(repo, home, &["--format", "json", "uninstall"], "uninstall");
        assert_eq!(
            settings(repo),
            older,
            "the one pattern the upgrade added is gone; the older install's entries stand"
        );
        assert_eq!(removed_flags(&out), (false, false, true));
        assert!(
            String::from_utf8_lossy(&out.stderr).contains("note: left "),
            "and they are named: {}",
            said(&out)
        );
    }
}

/// (5) **The settings file changes sides between `setup` and `uninstall`.** The record is
/// read only where git tracks the file *now*, and is written only where the install commit
/// can carry it:
///
/// - (a) tracked at `setup`, untracked and ignored at `uninstall` ⇒ the record is there
///   and is not read: nothing removed, everything named;
/// - (b) ignored at `setup`, tracked at `uninstall` ⇒ there is no record: nothing removed,
///   everything named;
/// - (c) as (a), then `setup` again ⇒ the record no longer describes a file that travels
///   with it, so the run removes it, says so, and its install commit carries the removal.
#[test]
fn a_settings_file_that_changes_sides_is_never_read_against_a_stale_record() {
    let untrack = |repo: &Path| {
        git(repo, &["rm", "-q", "--cached", "--", SETTINGS]);
        write(repo, ".gitignore", ".claude/settings.json\n");
        git(repo, &["add", "--", ".gitignore"]);
        git(repo, &["commit", "-q", "-m", "settings are private now"]);
    };

    // (a)
    {
        let (repo, home) = repo_with_own_settings("leaves-git", false);
        let (repo, home) = (repo.path(), home.path());
        jigc_ok(repo, home, &["setup"], "setup");
        assert!(record(repo).is_some(), "premise: tracked, so recorded");
        untrack(repo);
        let installed = settings(repo);

        let out = jigc_ok(repo, home, &["uninstall"], "uninstall");
        assert_eq!(settings(repo), installed, "(a) no entry was removed");
        assert!(
            said(&out).contains("git does not track `.claude/settings.json`"),
            "(a) and the reason is the file's, not the record's: {}",
            said(&out)
        );
    }

    // (b)
    {
        let (repo, home) = repo_with_own_settings("joins-git", true);
        let (repo, home) = (repo.path(), home.path());
        jigc_ok(repo, home, &["setup"], "setup");
        assert!(record(repo).is_none(), "premise: ignored, so not recorded");
        write(repo, ".gitignore", "");
        git(repo, &["add", "--", ".gitignore", SETTINGS]);
        git(repo, &["commit", "-q", "-m", "settings are shared now"]);
        let installed = settings(repo);

        let out = jigc_ok(repo, home, &["uninstall"], "uninstall");
        assert_eq!(settings(repo), installed, "(b) no entry was removed");
        assert!(
            said(&out).contains("no record git tracks"),
            "(b) the file is tracked and nothing describes it: {}",
            said(&out)
        );
    }

    // (c)
    {
        let (repo, home) = repo_with_own_settings("stale-record", false);
        let (repo, home) = (repo.path(), home.path());
        jigc_ok(repo, home, &["setup"], "setup");
        untrack(repo);
        let installed = settings(repo);

        let out = jigc_ok(repo, home, &["setup"], "the re-run");
        assert!(record(repo).is_none(), "(c) the stale record is gone");
        assert!(
            said(&out).contains("note: removed `.jigc/settings-entries.json`")
                && said(&out).contains("git does not hold that file here"),
            "(c) and the run says it took it, and why: {}",
            said(&out)
        );
        assert_eq!(
            git(repo, &["status", "--porcelain"]),
            "",
            "(c) the removal is in the install commit, not left lying in the tree"
        );
        assert!(
            git(repo, &["show", "--name-status", "--format=", "HEAD"])
                .lines()
                .any(|line| line.starts_with('D') && line.ends_with(RECORD)),
            "(c) as a deletion of the record"
        );
        assert_eq!(
            settings(repo),
            installed,
            "(c) the settings file is untouched"
        );

        // Tracked again later: there is no record to read against whatever the file
        // holds by then.
        write(repo, ".gitignore", "");
        git(repo, &["add", "--", ".gitignore", SETTINGS]);
        git(repo, &["commit", "-q", "-m", "settings are shared again"]);
        jigc_ok(repo, home, &["uninstall"], "uninstall");
        assert_eq!(
            settings(repo),
            installed,
            "(c) and nothing is removed on its word"
        );
    }
}

/// (6) **What must stay, beside what goes** — in a tracked settings file with a record:
///
/// - an entry of the adopter's that differs from jigc's by one character;
/// - entries the adopter adds **after** `setup`, committed or not;
/// - a twin of an entry jigc added, written beside it afterwards: jigc added one, so one
///   goes.
#[test]
fn the_adopters_near_and_later_entries_stay() {
    let (repo, home) = born_repo("must-keep");
    let (repo, home) = (repo.path(), home.path());
    write(
        repo,
        SETTINGS,
        r#"{ "permissions": { "allow": ["Bash(jigc :*)"], "deny": ["Bash(rm -rf :*)"] } }"#,
    );
    git(repo, &["add", "-A"]);
    git(repo, &["commit", "-q", "-m", "our own settings"]);

    jigc_ok(repo, home, &["setup"], "setup");
    let installs = installs();
    assert_eq!(
        record(repo).expect("the record")["added"],
        serde_json::to_value(&installs).expect("the entries serialize"),
        "an entry one character off is another entry: jigc added all of its own"
    );

    // After the install: a new entry of each kind, and a twin of two jigc added. One
    // edit is committed and one is not — the record is read either way.
    let mut edited = settings(repo);
    for (key, mine, twin) in [
        ("allow", "Bash(cargo test:*)", "Bash(jigc:*)"),
        ("deny", "Bash(shutdown:*)", "Bash(rm -rf:*)"),
    ] {
        let entries = edited["permissions"][key].as_array_mut().expect("an array");
        entries.push(serde_json::json!(mine));
        entries.push(serde_json::json!(twin));
    }
    write(
        repo,
        SETTINGS,
        &format!("{}\n", serde_json::to_string_pretty(&edited).unwrap()),
    );
    git(repo, &["commit", "-q", "-am", "more of our own"]);
    edited["hooks"]["SessionStart"]
        .as_array_mut()
        .expect("the event's matchers")
        .push(serde_json::json!({
            "hooks": [ { "type": "command", "command": "my-own-tool --greet" } ]
        }));
    write(
        repo,
        SETTINGS,
        &format!("{}\n", serde_json::to_string_pretty(&edited).unwrap()),
    );

    jigc_ok(repo, home, &["uninstall"], "uninstall");
    assert_eq!(
        settings(repo),
        serde_json::json!({
            "permissions": {
                "allow": ["Bash(jigc :*)", "Bash(cargo test:*)", "Bash(jigc:*)"],
                "deny": ["Bash(rm -rf :*)", "Bash(shutdown:*)", "Bash(rm -rf:*)"],
            },
            "hooks": {
                "SessionStart": [
                    { "hooks": [ { "type": "command", "command": "my-own-tool --greet" } ] }
                ]
            }
        }),
        "jigc's own entries are gone — one occurrence of each — and the adopter's near \
         entry, later entries and twins are all still there"
    );
}

/// (7) **`--force` is the consent: it takes every entry identical to one jigc installs,
/// recorded or not, the adopter's included** — what the door did for every caller before
/// it kept a record, and the one-command way to clear a settings file the plain teardown
/// leaves alone.
#[test]
fn force_removes_every_entry_identical_to_one_jigc_installs() {
    for ignored in [false, true] {
        let (repo, home) = repo_with_own_settings("force", ignored);
        let (repo, home) = (repo.path(), home.path());
        jigc_ok(repo, home, &["setup"], "setup");

        let out = jigc_ok(
            repo,
            home,
            &["--format", "json", "uninstall", "--force"],
            "uninstall --force",
        );
        let after = settings(repo);
        assert_eq!(
            jigc_shaped(&after),
            0,
            "ignored={ignored}: no entry identical to one jigc installs is left: {after}"
        );
        assert_eq!(
            (listed(&after, "allow"), listed(&after, "deny")),
            (
                vec!["Bash(make:*)".to_string()],
                vec!["Bash(sudo:*)".to_string()]
            ),
            "ignored={ignored}: and every other entry is"
        );
        assert_eq!(removed_flags(&out), (true, true, true));
        assert!(
            !String::from_utf8_lossy(&out.stderr).contains("left "),
            "ignored={ignored}: nothing was left to name: {}",
            said(&out)
        );
    }

    // And after a plain teardown that left them: the same consent still clears them,
    // though the install and its record are gone.
    let (repo, home) = repo_with_own_settings("force-after", true);
    let (repo, home) = (repo.path(), home.path());
    jigc_ok(repo, home, &["setup"], "setup");
    jigc_ok(repo, home, &["uninstall"], "the plain teardown");
    assert!(jigc_shaped(&settings(repo)) > 0, "premise: it left them");
    jigc_ok(
        repo,
        home,
        &["uninstall", "--force"],
        "then with the consent",
    );
    assert_eq!(jigc_shaped(&settings(repo)), 0);
}

/// (8) **A second clone gets the record with the file it describes.** `setup` there adds
/// nothing and changes nothing; `uninstall` there removes what the first clone's install
/// added, and leaves the entries the repository had before jigc.
#[test]
fn a_second_clone_reads_the_record_that_travelled_with_the_settings_file() {
    let (origin, home) = repo_with_own_settings("origin", false);
    let before = settings(origin.path());
    jigc_ok(origin.path(), home.path(), &["setup"], "setup in the first");

    let clone = TempDir::new("clone");
    let clone_home = TempDir::new("clone-home");
    let origin_path = origin.path().to_str().expect("a utf-8 temp path");
    git(clone.path(), &["clone", "-q", origin_path, "."]);
    git(clone.path(), &["config", "user.email", "test@example.com"]);
    git(clone.path(), &["config", "user.name", "Test"]);
    let (repo, home) = (clone.path(), clone_home.path());
    let head = git(repo, &["rev-parse", "HEAD"]);
    let travelled = read(repo, RECORD);
    assert!(!travelled.is_empty(), "premise: the clone has the record");

    jigc_ok(repo, home, &["setup"], "setup in the clone");
    assert_eq!(
        read(repo, RECORD),
        travelled,
        "the clone's run added nothing, and the record still says what the first run added"
    );
    assert_eq!(git(repo, &["rev-parse", "HEAD"]), head, "no commit");
    assert_eq!(git(repo, &["status", "--porcelain"]), "");

    jigc_ok(repo, home, &["uninstall"], "uninstall in the clone");
    assert_eq!(
        settings(repo),
        before,
        "exactly what the first clone's install added is gone"
    );
}

/// (9) **The record is small, stable and jigc's own.** A re-run rewrites it byte for byte
/// and commits nothing; an entry jigc added that somebody then deleted is merged back and
/// stays claimed; and an uncommitted edit to the record is the adopter's change at an
/// install path — refused by name, like any other.
#[test]
fn the_record_is_byte_stable_across_runs_and_refuses_an_uncommitted_edit() {
    let (repo, home) = repo_with_own_settings("stable", false);
    let (repo, home) = (repo.path(), home.path());
    jigc_ok(repo, home, &["setup"], "setup");
    let first = read(repo, RECORD);
    assert!(
        first.ends_with("}\n") && first.len() < 2048,
        "one small file: {} bytes",
        first.len()
    );
    let head = git(repo, &["rev-parse", "HEAD"]);

    jigc_ok(repo, home, &["setup"], "the re-run");
    assert_eq!(read(repo, RECORD), first, "byte-identical");
    assert_eq!(git(repo, &["rev-parse", "HEAD"]), head, "and no commit");

    // An entry jigc added is deleted from the settings file and the deletion committed:
    // the next run merges it back, and it is still one jigc added.
    let mut trimmed = settings(repo);
    trimmed["permissions"]["allow"]
        .as_array_mut()
        .expect("the allow list")
        .retain(|entry| entry != "Bash(jigc:*)");
    write(
        repo,
        SETTINGS,
        &format!("{}\n", serde_json::to_string_pretty(&trimmed).unwrap()),
    );
    git(repo, &["commit", "-q", "-am", "drop the permit"]);
    jigc_ok(repo, home, &["setup"], "the run that merges it back");
    let rerecorded = record(repo).expect("the record");
    assert_eq!(
        rerecorded["added"]["allow"],
        serde_json::json!(["Bash(jigc:*)"]),
        "still claimed — this run added it"
    );
    assert_eq!(git(repo, &["status", "--porcelain"]), "", "and committed");

    // An uncommitted edit to the record is refused before anything is written.
    let committed = read(repo, RECORD);
    let edited = committed.replace("Bash(jigc:*)", "Bash(make:*)");
    assert_ne!(edited, committed, "premise: the edit changed the record");
    write(repo, RECORD, &edited);
    let out = jigc(repo, home, &["setup"]);
    assert_eq!(out.status.code(), Some(1), "{}", said(&out));
    assert!(
        said(&out).contains("setup.dirty-install-path") && said(&out).contains(RECORD),
        "by name, under the install guard's code: {}",
        said(&out)
    );
    assert_eq!(read(repo, RECORD), edited, "and the edit is where it was");
}
