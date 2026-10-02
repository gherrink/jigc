//! **`jigc task validate` refuses the posture `jigc task finalize` refuses** (M52
//! Increment 3 / T6; `completions/artifacts/M52/settle-record.md` → D2.6 as amended by
//! Review amendments §4).
//!
//! # What was broken
//!
//! `cli::gate_coverage::Door::Previewed` promises *"same check, same severity, same exit
//! code"*, and `jigc task validate` is the door a driver runs to learn whether the
//! commit boundary will refuse. Through M52 Increment 2 it was silent about the one
//! pre-commit phase a caller can resolve **before** finalizing: under a merge, a rebase,
//! a half-finished pick — any operation git has left un-concluded — `task validate`
//! reported the task's content findings and exited 0 or 3, while `task finalize` refused
//! outright at exit 1. The preview whose whole job is *tell me what this finalize will
//! do* was green over a state the door it forecasts would not act in.
//!
//! # The claim, and why it is one byte-identity rather than three assertions
//!
//! The Settle's words are *"renders the identical `repo.operation-in-progress` /
//! `repo.head-detached` finding, and exits 1 exactly as `finalize` would — same check,
//! same severity, same exit"*. So the fence is **byte-identity across the three doors**
//! — `task validate`, `task finalize --dry-run` and the committing `task finalize` —
//! rather than a restatement here of what the finding should say. A test that rebuilt
//! the expected message would pass while the bytes an agent reads diverged, and the
//! identity is the contract: one producer (`cli::cli`'s posture guard), asked by the
//! preview with the **finalize** door's own `ActsOnBehalf` row, so an exemption added to
//! that row reaches the preview without an edit here.
//!
//! **This suite is also the posture member's cell in two registries.** It is what
//! `flow49_acceptance.rs`'s arm-5 `preview_cell` cites for the `posture` member of
//! `cli::gate_coverage::Tier::Previewed`, and it is where
//! `dry_run_findings_equal_set.rs` states the three-door equality for the one previewed
//! member whose check is invoked **separately at the door** rather than inside
//! `TaskArea::preview_gates` (`cli::gate_coverage::Invocation::SeparatelyAtDoor`) — that
//! suite reads the envelope off **stdout**, and a refusal's envelope rides **stderr**.
//!
//! # The axis
//!
//! Every member of [`GitState::ALL`] that can be overlaid on a built corpus — the
//! operations git can leave un-concluded plus the detached HEAD — not the merge the
//! done-criterion names. One state is an instance; the class is *any posture the commit
//! boundary refuses under*, and a preview that covered one member of it and not the
//! others would be the incomplete sweep this wave exists to stop shipping.
//!
//! **The thirteenth cell is covered elsewhere, not skipped.** `GitState::Unborn` cannot
//! be overlaid on a built corpus — `jigc setup` births HEAD on an unborn repository, so
//! the state is unreachable from any corpus that has a live task to address
//! (`GitState::overlay_refusal`) — and the one fixture in the suite set that does hold a
//! live task on an unborn HEAD is
//! `file_state_history_gate::unborn_head_keeps_the_conservative_weak_deletion_block`,
//! which asserts the preview's `repo.head-unborn` refusal there rather than leaving the
//! member to a loop that cannot build it.
//!
//! # The second preview surface (M53)
//!
//! `jigc start`'s orientation now **reports** the same posture, on the same axis, at exit
//! **0** — the M52 per-axis review's `(6, D-1)`: a repository mid-merge read
//! `findings: none` on its live task and the agent learned at `finalize`, after authoring.
//! It is the same producer asked with the same `task finalize` row, so the arm below
//! checks the cross-door identity per cell rather than restating the message here, and the
//! file's subject widens from *the preview* to *every surface that forecasts the posture
//! the commit boundary refuses under*.
//!
//! # And the refusal acts on nothing
//!
//! Each cell re-asserts the fixture's own git state after all three doors have run
//! (`git_state::assert_state`): the markers git wrote are still there, HEAD is unmoved.
//! A preview that concluded the user's merge in order to report on it would be worse
//! than the silence it replaces.

use crate::support::git_state::{self, GitState};
use crate::support::trial_corpus::{State, TrialCorpus};

use std::process::Output;

/// The three doors the identity holds across, in the order a driver meets them.
const DOORS: &[(&str, &[&str])] = &[
    ("jigc task validate", &["task", "validate"]),
    ("jigc task finalize --dry-run", &["task", "finalize"]),
    ("jigc task finalize", &["task", "finalize"]),
];

/// The argv for one door over `task`, with `--dry-run` where the row needs it.
fn argv(index: usize, args: &[&str], task: &str, format: &[&str]) -> Vec<String> {
    let mut out: Vec<String> = format.iter().map(|s| (*s).to_string()).collect();
    out.extend(args.iter().map(|s| (*s).to_string()));
    out.push(task.to_string());
    if index == 1 {
        out.push("--dry-run".to_string());
    }
    out
}

/// What a door printed, as the pair a refusal is judged on.
fn printed(out: &Output) -> (i32, String, String) {
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).to_string(),
        String::from_utf8_lossy(&out.stderr).to_string(),
    )
}

/// Drive the three doors over one corpus and return their `(exit, stdout, stderr)`.
fn three_doors(corpus: &TrialCorpus, task: &str, format: &[&str]) -> Vec<(i32, String, String)> {
    DOORS
        .iter()
        .enumerate()
        .map(|(index, (_, args))| {
            let owned = argv(index, args, task, format);
            let borrowed: Vec<&str> = owned.iter().map(String::as_str).collect();
            printed(&corpus.jigc(&borrowed))
        })
        .collect()
}

/// **The fence.** In every git state a user can leave behind, `jigc task validate`
/// refuses byte-identically to the two `finalize` doors, at the same exit code, and
/// concludes nothing.
#[test]
fn validate_refuses_every_posture_finalize_refuses_and_concludes_nothing() {
    let built = TrialCorpus::build(State::RefsPostHoc);
    let task = built
        .live_task()
        .expect("`refs-post-hoc` leaves a live task")
        .to_string();

    let mut covered = 0usize;
    for state in GitState::ALL {
        if state.overlay_refusal().is_some() {
            // Unborn cannot be overlaid on a corpus `jigc setup` has committed into
            // (module header): its cell lives in `file_state_history_gate.rs`.
            continue;
        }
        let corpus = built.copy_state();
        git_state::overlay(&corpus, *state).expect("overlay a reachable git state");

        let outs = three_doors(&corpus, &task, &[]);
        let (validate_code, validate_out, validate_err) = &outs[0];
        for ((door, _), (code, out, err)) in DOORS.iter().zip(outs.iter()) {
            assert_eq!(
                *code,
                1,
                "`{door}` must refuse in the `{}` state at exit 1; stdout:\n{out}\nstderr:\n{err}",
                state.name(),
            );
            assert_eq!(
                err,
                validate_err,
                "`{door}` and `jigc task validate` must print the SAME refusal in the \
                 `{}` state — one producer, so the preview cannot diverge from the door \
                 it forecasts",
                state.name(),
            );
            assert_eq!(
                out,
                validate_out,
                "`{door}` must print the same (empty) stdout as the preview in the `{}` \
                 state",
                state.name(),
            );
        }
        assert_eq!(
            *validate_code, 1,
            "the preview's own exit is finalize's, not a validation verdict",
        );
        assert!(
            validate_err.contains("repo."),
            "the refusal names a posture-family code in the `{}` state; got:\n{validate_err}",
            state.name(),
        );

        // Nothing was concluded: the markers git wrote are still there and HEAD has
        // not moved, after all three doors ran.
        git_state::assert_state(&corpus.repo(), &corpus.home(), *state);
        covered += 1;
    }
    assert_eq!(
        covered,
        GitState::ALL
            .iter()
            .filter(|state| state.overlay_refusal().is_none())
            .count(),
        "every overlayable git state is a cell; a skipped one is a posture nobody previewed",
    );
}

/// **The machine surface, at the state the done-criterion names.** The refusal document
/// is identical at all three doors and names the operation's own finding code and route.
///
/// The posture guard's `--format json` arm is the **flattened** `{"error": …}` one — the
/// row `cli::invocation_log::ENVELOPE_ARMS` declares for it — so this asserts the code and
/// the route inside that string rather than a `findings[]` array the door does not emit.
/// Which arm a refusal takes is that registry's call and not this suite's; what *is* this
/// suite's is that the preview takes the same one, byte for byte, as the door it forecasts.
#[test]
fn the_refusal_envelope_is_identical_at_the_three_doors() {
    let built = TrialCorpus::build(State::RefsPostHoc);
    let task = built
        .live_task()
        .expect("`refs-post-hoc` leaves a live task")
        .to_string();
    let corpus = built.copy_state();
    git_state::overlay(&corpus, GitState::Merge).expect("overlay a merge");

    let outs = three_doors(&corpus, &task, &["--format", "json"]);
    let (_, _, envelope) = &outs[0];
    let parsed: serde_json::Value = serde_json::from_str(envelope)
        .unwrap_or_else(|err| panic!("a JSON envelope ({err}); got:\n{envelope}"));
    let error = parsed["error"]
        .as_str()
        .unwrap_or_else(|| panic!("the refusal's declared arm is `error`; got:\n{parsed:#}"));
    assert!(
        error.contains("repo.operation-in-progress"),
        "the preview's refusal names the posture code; got:\n{error}",
    );
    assert!(
        error.contains("git merge --abort"),
        "and routes at the operation's own git command; got:\n{error}",
    );
    for ((door, _), (code, _, err)) in DOORS.iter().zip(outs.iter()) {
        assert_eq!(*code, 1, "`{door}` refuses at exit 1");
        assert_eq!(
            err, envelope,
            "`{door}`'s refusal envelope must be the preview's, byte for byte",
        );
    }
}

/// **The second preview surface: orientation reports the posture it does not refuse on**
/// (M53 — the pre-v1 usability batch, row 2 / the M52 per-axis review's `(6, D-1)`).
///
/// `jigc start` acts on nobody's behalf, so the door guard deliberately never fires for
/// it — and through rc.19 that meant an agent opening a session over a repository
/// mid-merge, mid-bisect or holding a conflicted cherry-pick read `findings: none` on its
/// live task and learned the truth at `finalize`, after authoring. The whole point of the
/// orientation door is that it runs at every `SessionStart`.
///
/// **What it is NOT:** a refusal. The exit stays **0**, stdout still carries the whole
/// orientation, and nothing is concluded — asserted per cell, exactly as the three doors
/// above are. What changes is that the row's own `findings` array leads with the posture,
/// on the text render **and** on the versioned envelope, under the key that array already
/// had: no new key, no `SCHEMA_VERSION` move.
///
/// **The axis is the same one**, for the same reason: one state is an instance, and a
/// report that named the merge and stayed silent about the bisect would be the incomplete
/// sweep this file's sibling arm exists to stop. Each cell also checks the cross-door
/// identity — orientation's code is the refusal's code and its route text appears verbatim
/// inside the refusal — so the surface that reports and the door that refuses cannot come
/// to say different things about one checkout.
#[test]
fn orientation_reports_every_posture_finalize_refuses_without_refusing() {
    let built = TrialCorpus::build(State::RefsPostHoc);
    let task = built
        .live_task()
        .expect("`refs-post-hoc` leaves a live task")
        .to_string();

    let mut covered = 0usize;
    for state in GitState::ALL {
        if state.overlay_refusal().is_some() {
            continue; // Unborn — see the sibling arm's comment.
        }
        let corpus = built.copy_state();
        git_state::overlay(&corpus, *state).expect("overlay a reachable git state");

        // The refusing door, for the identity this cell asserts against.
        let refusal = printed(&corpus.jigc(&["task", "validate", &task])).2;

        let (code, stdout, stderr) = printed(&corpus.jigc(&["start"]));
        assert_eq!(
            code,
            0,
            "orientation REPORTS a posture, it does not refuse on one, in the `{}` \
             state; stdout:\n{stdout}\nstderr:\n{stderr}",
            state.name(),
        );
        assert!(
            stdout.contains(&format!("Active task: {task}")),
            "the whole orientation still renders in the `{}` state; got:\n{stdout}",
            state.name(),
        );

        let (code, envelope, _) = printed(&corpus.jigc(&["--format", "json", "start"]));
        assert_eq!(code, 0, "the machine arm exits 0 too");
        let parsed: serde_json::Value = serde_json::from_str(&envelope)
            .unwrap_or_else(|err| panic!("a JSON envelope ({err}); got:\n{envelope}"));
        assert_eq!(
            parsed["state"], "active-task",
            "the posture does not change which variant orientation is",
        );
        assert_eq!(
            parsed["schema_version"], 3,
            "the posture rides the `findings` array the row already had — no key was \
             added, so no version moved",
        );
        let mut keys: Vec<&str> = parsed
            .as_object()
            .expect("an object envelope")
            .keys()
            .map(String::as_str)
            .collect();
        keys.sort_unstable();
        assert_eq!(
            keys,
            [
                "header",
                "next_steps",
                "schema_version",
                "state",
                "tasks",
                "workflows"
            ],
            "the declared top-level key set of the `active-task` arm is unchanged",
        );

        let row = &parsed["tasks"][0];
        let posture = row["findings"][0]
            .as_object()
            .unwrap_or_else(|| panic!("the row's findings lead with the posture; got:\n{row:#}"));
        let code = posture["code"].as_str().unwrap_or_default();
        let route = posture["route"].as_str().unwrap_or_default();
        assert!(
            code.starts_with("repo."),
            "the leading finding is the posture family's in the `{}` state; got `{code}`",
            state.name(),
        );
        assert_eq!(posture["severity"], "blocking");
        assert!(
            refusal.contains(code),
            "orientation's posture code is the refusing door's, in the `{}` state; \
             refusal:\n{refusal}",
            state.name(),
        );
        assert!(
            !route.is_empty() && refusal.contains(route),
            "and its route is the refusing door's, verbatim, in the `{}` state; \
             route `{route}` against refusal:\n{refusal}",
            state.name(),
        );
        assert!(
            stdout.contains(code) && stdout.contains(route),
            "both reach the TEXT render too, in the `{}` state; got:\n{stdout}",
            state.name(),
        );

        // Reporting concludes nothing, exactly as previewing does not.
        git_state::assert_state(&corpus.repo(), &corpus.home(), *state);
        covered += 1;
    }
    assert_eq!(
        covered,
        GitState::ALL
            .iter()
            .filter(|state| state.overlay_refusal().is_none())
            .count(),
        "every overlayable git state is a cell here too",
    );
}

/// **The fan-out-worktree cell** — the one checkout where the preview took a subject the
/// door's own seam does not (M53, the rc.20 per-axis review `(2, A2-2)`).
///
/// `cli::repo::posture_subject` exempts a jigc-provisioned fan-out worktree from
/// `PostureMember::HeadDetached`, because jigc detached it. That exemption belongs to the
/// acts jigc performs there **holding the worktree's handle** — the boundary's own commits
/// through `SeamSubject::dedicated` — and `jigc task finalize` is not one of them: it
/// commits in the checkout it was run in, through `SeamSubject::live`, which adjudicates
/// the member. Driven on `1.0.0-rc.20`, all three previews read clean at exit 0 while the
/// door refused at exit 1 from inside the transaction.
///
/// So the cell is two claims, and the second is what keeps the first from being a
/// regression:
///
/// 1. an **ordinary** task minted inside a provisioned worktree is refused
///    `repo.head-detached` **byte-identically** at all three doors, and reported by
///    orientation, exactly as every other member of this suite's axis is; and its route is
///    one that **runs there** — `git switch <branch>` is what the shipped bytes said, and
///    git refuses a branch another worktree holds at exit 128, which is no exit at all;
/// 2. the **sub-task** in the same worktree, at the same instant, is still silent about it
///    — its boundary is `jigc milestone finalize`, and the exemption is that door's.
#[test]
fn an_ordinary_task_in_a_fan_out_worktree_is_previewed_as_the_door_refuses_it() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    for argv in [
        vec!["milestone", "create", "Cwd wave"],
        vec!["milestone", "add-task", "cwd-wave", "Area one"],
        vec!["milestone", "provision", "cwd-wave"],
    ] {
        corpus.jigc_ok(&argv);
    }
    let worktree = corpus
        .repo()
        .join(".jigc")
        .join("worktrees")
        .join("area-one");
    assert!(
        worktree.is_dir(),
        "the sub-task worktree must be provisioned"
    );

    // An ORDINARY task, minted from inside that worktree — not a sub-task, so the door
    // that lands it is `jigc task finalize` and the checkout it commits in is this one.
    let minted = corpus.jigc_stdin_from(
        &worktree,
        &["start", "--workflow", "single-task", "Ordinary in worktree"],
        "",
    );
    assert!(
        minted.status.success(),
        "the ordinary task must mint; stderr:\n{}",
        String::from_utf8_lossy(&minted.stderr),
    );
    let task = "ordinary-in-worktree";

    // (1) The three doors, from that cwd — byte-identical refusal, identical exit.
    let mut seen: Vec<(i32, String)> = Vec::new();
    for (index, (label, args)) in DOORS.iter().enumerate() {
        let owned = argv(index, args, task, &[]);
        let borrowed: Vec<&str> = owned.iter().map(String::as_str).collect();
        let (code, _, stderr) = printed(&corpus.jigc_stdin_from(&worktree, &borrowed, ""));
        assert_eq!(
            code, 1,
            "`{label}` must refuse the posture its own seam refuses; stderr:\n{stderr}",
        );
        assert!(
            stderr.contains("blocking · repo.head-detached"),
            "`{label}` must render the door's own finding; stderr:\n{stderr}",
        );
        seen.push((code, stderr));
    }
    let (_, first) = &seen[0];
    for (_, other) in &seen[1..] {
        assert_eq!(
            first, other,
            "one producer — the three doors must print the same bytes in this cell",
        );
    }

    // …and the route is an exit that RUNS from the checkout that printed it. `git switch
    // <branch>` is the one it must no longer be: the only branch in this repository is the
    // one the main checkout holds, and git refuses it at 128.
    assert!(
        first.contains("git switch -c <new-branch>") && first.contains("`area-one`"),
        "the route must name a branch-creating exit and say which worktree this is; \
         stderr:\n{first}",
    );
    assert!(
        !first.contains("git switch <branch>"),
        "the unrunnable placeholder must be gone from this cell; stderr:\n{first}",
    );
    // (2) The fourth surface and the sub-task control, read off one orientation while HEAD
    // is still the one jigc detached — before the route below re-attaches it.
    let oriented = corpus.jigc_stdin_from(&worktree, &["--format", "json", "start"], "");
    let json = String::from_utf8_lossy(&oriented.stdout).to_string();
    let doc: serde_json::Value = serde_json::from_str(&json).expect("orientation is JSON");
    let rows = doc["tasks"].as_array().expect("an active set");
    for row in rows {
        let id = row["id"].as_str().unwrap_or_default();
        let codes: Vec<&str> = row["findings"]
            .as_array()
            .map(|set| {
                set.iter()
                    .filter_map(|finding| finding["code"].as_str())
                    .collect()
            })
            .unwrap_or_default();
        let carries = codes.contains(&"repo.head-detached");
        if id == task {
            assert!(
                carries,
                "the ordinary row must carry the posture its door refuses under; row:\n{row}",
            );
        } else {
            assert!(
                !carries,
                "sub-task `{id}`'s boundary commits this worktree from its own handle — the \
                 exemption is that door's, and the row must stay silent; row:\n{row}",
            );
        }
    }
    assert_eq!(
        rows.len(),
        2,
        "the ordinary task and the sub-task, both live"
    );

    // …and the route runs from the checkout that printed it **with its one placeholder
    // substituted** — `<new-branch>` is the agent's to name, exactly as `git switch <branch>`
    // and `--value <value>` are elsewhere, so this is the emitted argv with that span filled
    // and nothing else changed (M53, the last pre-1.0.0 batch's review, LOW 1: the comment
    // here, and `6281f768`'s commit body, both said *run verbatim*, and pasted literally the
    // `<` is a shell redirect). Running it is the half `git switch <branch>` could not do: it
    // exits 128 over a branch another worktree holds.
    let switched = std::process::Command::new("git")
        .args(["switch", "-c", "fan-out-ordinary"])
        .current_dir(&worktree)
        .output()
        .expect("run the emitted route");
    assert!(
        switched.status.success(),
        "the emitted route must run from the checkout that printed it, with its `<new-branch>` \
         placeholder substituted; git said:\n{}",
        String::from_utf8_lossy(&switched.stderr),
    );
    let refused = std::process::Command::new("git")
        .args(["switch", "main"])
        .current_dir(&worktree)
        .output()
        .expect("run the placeholder the route no longer names");
    assert!(
        !refused.status.success(),
        "the control that makes this cell a dead end: the only branch here is the one the \
         main checkout holds, and git must refuse it",
    );

    // (3) **One step further: the state that route CREATES** (M53, the last pre-1.0.0
    // batch's review, MEDIUM 1). The cell above stopped here — it ran the switch and never
    // asked what the *sub-task's* doors say afterwards. Driven, the answer was a dead end:
    // the ordinary finalize this batch unblocked lands a commit on the new branch, HEAD
    // leaves the milestone's shared base, and `blanket_base_pin_refusal`'s provisioned arm
    // then told a reader standing **in** the worktree to `cd` to the directory they were
    // already in.
    //
    // So this half drives the whole arc as an agent walks it: land the ordinary task here,
    // then both sub-task read doors, then the act their refusal names — and read back that
    // the act cost nothing. The **unprovisioned** arm of that refusal needs no cell of its
    // own: its worktree does not exist, so no caller's repository root can equal it.
    std::fs::write(worktree.join("wt-work.txt"), "the ordinary task's work\n")
        .expect("the ordinary task's own code edit");
    let added = std::process::Command::new("git")
        .args(["add", "wt-work.txt"])
        .current_dir(&worktree)
        .output()
        .expect("stage the ordinary task's work");
    assert!(added.status.success(), "the edit must stage");
    for (address, value) in [("type", "feat"), ("scope", "worktree")] {
        let wrote = corpus.jigc_stdin_from(
            &worktree,
            &[
                "doc",
                "set-field",
                &format!("commit:{task}#{address}"),
                "--task",
                task,
                "--value",
                value,
            ],
            "",
        );
        assert!(
            wrote.status.success(),
            "authoring `{address}` must succeed; stderr:\n{}",
            String::from_utf8_lossy(&wrote.stderr),
        );
    }
    for (address, prose) in [
        ("summary", "land the ordinary work"),
        ("body", "In-worktree."),
    ] {
        let wrote = corpus.jigc_stdin_from(
            &worktree,
            &[
                "doc",
                "set-slot",
                &format!("commit:{task}#{address}"),
                "--task",
                task,
                "--from-file",
                "-",
            ],
            prose,
        );
        assert!(
            wrote.status.success(),
            "authoring `{address}` must succeed; stderr:\n{}",
            String::from_utf8_lossy(&wrote.stderr),
        );
    }
    let landed = corpus.jigc_stdin_from(&worktree, &["task", "finalize", task], "");
    let (code, _, stderr) = printed(&landed);
    assert_eq!(
        code, 0,
        "the ordinary finalize this batch unblocked must land from the worktree — the whole \
         premise of the state below; stderr:\n{stderr}",
    );
    let landed_sha = git_line(&worktree, &["rev-parse", "HEAD"]);

    // Both sub-task read doors, from that cwd. `area-one` is the sub-task; its recorded
    // minting workflow is `sub-task`, which the re-entry door asserts equality on.
    let sub = "area-one";
    let mut pin: Option<String> = None;
    for argv in [
        vec!["workflow", "sub-task", "--task", sub],
        vec!["start", "--task", sub],
    ] {
        let (code, _, stderr) = printed(&corpus.jigc_stdin_from(&worktree, &argv, ""));
        assert_eq!(
            code,
            1,
            "`jigc {}` must still keep the blanket base-pin refusal here; stderr:\n{stderr}",
            argv.join(" "),
        );
        assert!(
            !offers_a_cd(&stderr),
            "the refusal must not offer a `cd` to the directory the reader is standing in — \
             a route whose first clause is a no-op; stderr:\n{stderr}",
        );
        let short = stderr
            .split("is pinned to base ")
            .nth(1)
            .and_then(|tail| tail.split_whitespace().next())
            .expect("the refusal names the pin it is off")
            .to_string();
        assert!(
            stderr.contains(&format!("git switch --detach {short}")),
            "the refusal must name the act that resolves it — re-attaching HEAD to the pin; \
             stderr:\n{stderr}",
        );
        match &pin {
            None => pin = Some(short),
            Some(first) => assert_eq!(first, &short, "one producer, one pin, both doors"),
        }
    }

    // The act, run as printed — and then the door it was printed by, which is the step the
    // cell used to stop short of.
    let pin = pin.expect("both doors named the pin");
    let reattached = std::process::Command::new("git")
        .args(["switch", "--detach", &pin])
        .current_dir(&worktree)
        .output()
        .expect("run the emitted act");
    assert!(
        reattached.status.success(),
        "the emitted act must run from the checkout that printed it; git said:\n{}",
        String::from_utf8_lossy(&reattached.stderr),
    );
    let (code, _, stderr) =
        printed(&corpus.jigc_stdin_from(&worktree, &["workflow", "sub-task", "--task", sub], ""));
    assert_eq!(
        code, 0,
        "having run the act the refusal named, the sub-task door must compose; stderr:\n{stderr}",
    );

    // …and it cost nothing: the branch the agent made is still there, and the commit it
    // landed is still reachable from it. A route that resolved the pin by throwing the
    // agent's own commit away would be a worse dead end than the no-op it replaces.
    assert_eq!(
        git_line(&worktree, &["rev-parse", "fan-out-ordinary"]),
        landed_sha,
        "the branch the route created still holds the commit the ordinary finalize landed",
    );
}

/// Does `text` offer a `cd` — the command, not the two letters?
///
/// The refusal above names its pin by short SHA (`is pinned to base d80f6cd but …`), and a
/// raw `contains("cd ")` reads every SHA ending in `cd` as a route: one run in 256 failed
/// on exactly that. A `cd` the reader could run starts a word — the start of the text, or
/// after whitespace or the backtick/quote/separator a route is printed inside — never in
/// the middle of a hex string or a path component.
fn offers_a_cd(text: &str) -> bool {
    text.match_indices("cd ").any(|(at, _)| {
        text[..at]
            .chars()
            .next_back()
            .is_none_or(|c| !(c.is_alphanumeric() || matches!(c, '_' | '-' | '.' | '/')))
    })
}

#[test]
fn the_cd_detector_reads_a_command_not_a_short_sha() {
    // The flake: a short SHA whose last two hex digits are `cd`, followed by a space.
    assert!(
        !offers_a_cd("`jigc workflow` is pinned to base d80f6cd but HEAD is 1a2b3c4"),
        "a short SHA ending in `cd` is not a `cd` route",
    );
    assert!(
        !offers_a_cd("the worktree at /tmp/jigc-abcd is gone"),
        "a path component ending in `cd` is not a `cd` route",
    );
    // The shapes the refusal's provisioned arm prints, and a bare one: all routes.
    assert!(offers_a_cd(
        "run this from that worktree — `cd /tmp/wt`, then re-run"
    ));
    assert!(offers_a_cd("then cd /tmp/wt and re-run"));
    assert!(offers_a_cd("cd /tmp/wt"));
    assert!(offers_a_cd(
        "run: `jigc milestone provision m` && cd '/tmp/wt'"
    ));
}

/// One trimmed line of `git <args>` in `cwd`, asserting git succeeded.
fn git_line(cwd: &std::path::Path, args: &[&str]) -> String {
    let out = std::process::Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} must succeed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

/// **The omitting context.** Over the same corpus with **no** operation left
/// un-concluded, the preview is untouched: it renders its validation report on stdout
/// and exits on the report's own verdict, never on a posture.
#[test]
fn a_committable_posture_leaves_the_preview_exactly_as_it_was() {
    let corpus = TrialCorpus::build(State::RefsPostHoc);
    let task = corpus.live_task().expect("a live task").to_string();

    let out = corpus.jigc(&["task", "validate", &task]);
    let (code, stdout, stderr) = printed(&out);
    assert!(
        code == 0 || code == 3,
        "a committable posture leaves the preview's own verdict; got exit {code}\n\
         stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        !stderr.contains("repo.operation-in-progress") && !stderr.contains("repo.head-detached"),
        "no posture member fires over a clean checkout; got:\n{stderr}",
    );
    assert!(
        !stdout.is_empty(),
        "the preview still renders its report on stdout",
    );

    // And orientation's own omitting context: no posture code reaches either arm.
    let (code, oriented, _) = printed(&corpus.jigc(&["start"]));
    assert_eq!(code, 0, "orientation over a clean checkout exits 0");
    assert!(
        !oriented.contains("repo.operation-in-progress")
            && !oriented.contains("repo.head-detached"),
        "no posture member is reported over a clean checkout; got:\n{oriented}",
    );
}
