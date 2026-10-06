//! **The stabilization harness, held to what its rulings ask of it by a scan of its own
//! source** ([DECISIONS.md](../DECISIONS.md) → *2026-10-05 — The stabilization
//! workflow, as ruled*, and the entries of 2026-10-06, *The stabilization harness, as built*
//! and *The decision table's holes, closed*).
//!
//! `.claude/workflows/stabilize.js` runs under a runtime this suite does not have: it cannot
//! launch an agent. What it can do is read the script, and — where `node` is installed —
//! run the script's own self-test, which launches none. So the arms are of two kinds.
//!
//! **Read off the source, everywhere:**
//!
//! - **(a)** every agent type the script names resolves to a definition under
//!   `.claude/agents/`, or is `general-purpose`; and no call takes its type from anywhere
//!   but a literal or the roles table.
//! - **(b)** every `build-git` call — the one the git steps go through, and the close's sync
//!   step the script returns — carries `model: GIT_MODEL`, which is Sonnet.
//! - **(c)** a branch name is minted in exactly one function: no other code of the script
//!   spells a branch prefix — and neither does `dev/stabilize-step`, which is handed every
//!   name it works on.
//! - **(l)** every git step is ONE command of `dev/stabilize-step`, composed by one function
//!   and read back by one: each act the tool has is asked for by exactly one prompt
//!   function, and each call names the act its prompt composes. The one step that is still
//!   a list of commands is the re-cut of a part. What an act does is held where it is run
//!   ([`dev_stabilize_step`](super::dev_stabilize_step)).
//! - **(m)** a stage that is not fit for use refuses to start: the `fix` stage is named in
//!   the script's `NOT_FIT` for as long as [DECISIONS.md](../DECISIONS.md) has no heading
//!   that records its half as repaired and re-reviewed (the human's ruling of 2026-10-06 on
//!   the order of the repair); the refusal is asked before anything that could launch an
//!   agent; and it lets through only an invocation that records what the human ruled about
//!   the run.
//! - **(d)** each label the definitions' contracts bind on is spelled once in the script,
//!   and identically in every definition that binds on it; and a role is sent only labels
//!   its definition binds on.
//! - **(e)** a paragraph the definitions repeat for a stabilization run is byte-identical
//!   wherever it stands — the set derived from the definitions, never listed.
//! - **(f)** the human's rulings enter in one place: the declared-bounds list and the three
//!   dispositions that are the human's are written by the one step that records the
//!   `rulings` argument, and no other prompt carries those commands.
//! - **(g)** the cross-model pass is launched only for an item the invocation names:
//!   everything the script says about that tool lies in the `crossModel*` functions, each
//!   call of one sits where the naming is tested, and no definition the script launches
//!   assumes the tool.
//! - **(h)** the script's self-test and everything it calls launch no agent, and the script
//!   uses nothing the runtime forbids.
//! - **(j)** what `dev/stabilize-record` decides from reaches it, and what it decides is
//!   passed on: a stage records ONE RESULT PER ITEM it ran and nothing about a clause —
//!   a clause's status is the record script's to derive — and its results before anything
//!   of the ledger; a re-run is the position's (`rerun`), never the invocation's to decide,
//!   begins no round and writes no fact of one; a
//!   stage's dispositions are written before its triage, so that a finding found again
//!   after its fix is found *with* that fix; the script has a sentence for every word the
//!   record script refuses a stage with; and `next` is relayed, never recomputed.
//! - **(k)** what the decisions of 2026-10-06 on the table's holes gave this script to do
//!   has one road each: the human's go, one more re-run of a clause and the raised bound
//!   are written by the rulings step and by nothing else, and an invocation that carries
//!   them starts nothing; an unfinished triage is finished where the position says so, by
//!   a function that runs no instrument, and every triage is handed the ledger's rows that
//!   still await one; the previous release and the default scope reach a prompt from the
//!   run's recorded facts, never from prose; `close` goes back with each clause's evidence;
//!   and the verifier's definition says what an unverified finding means.
//!
//! - **(n)** a record is accepted on its commit step's own line, and on nothing an agent says
//!   of it: one function runs a record step — the executor, which applies the batch and
//!   runs the gate and commits nothing, then the ONE commit as a git step — and holds the
//!   line to the commit, the gate check and one result per check the script composed; and
//!   an invocation that finds a batch applied and not committed finishes it before it reads
//!   the run's state, and starts nothing.
//!
//! **Driven, where `node` is on `PATH`** (it is on this project's development machines and
//! on GitHub's hosted runners; where it is not, the arm fails under CI and passes anywhere
//! else — the gate gains no dependency — and the gate's own summary names it as a test
//! that passed without running: [`node_or_skip`]):
//!
//! - **(i)** the script parses; its self-test passes without launching an agent; every
//!   malformed invocation is refused before one is launched; and no value of `crossModel`
//!   turns the cross-model pass on for every item.
//!
//! The sync step's log list is held to `milestone-build.js`'s by
//! [`merge_logs_fence`](super::merge_logs_fence), arm (k) — the home of that parity.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;

use crate::support::root_walk;
use crate::support::scratch::ScratchDir;

use super::dev_stabilize_step::node_or_skip;

const HARNESS: &str = ".claude/workflows/stabilize.js";
const RECORD_SCRIPT: &str = "dev/stabilize-record";
const STEP_TOOL: &str = "dev/stabilize-step";
const DEFINITIONS: &str = ".claude/agents";

/// The four lines the definitions task named as what its contracts bind on. The script may
/// send more labelled lines than these; it may not send fewer.
const RULED_LABELS: [&str; 4] = ["REPORT:", "BINARY:", "AREA:", "RECORD STEP"];

/// The three dispositions that are the human's to give (ruling 4).
const HUMAN_DISPOSITIONS: [&str; 3] = ["admitted", "bound", "later"];

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/cli has a repo root two levels up")
        .to_path_buf()
}

fn harness() -> String {
    fs::read_to_string(repo_root().join(HARNESS)).expect("read the stabilization harness")
}

/// Every definition, as `(name, text)`.
fn definitions() -> BTreeMap<String, String> {
    root_walk::files_in(&repo_root().join(DEFINITIONS), root_walk::ext("md"))
        .into_iter()
        .map(|path| {
            let name = path
                .file_stem()
                .expect("a definition has a name")
                .to_string_lossy()
                .into_owned();
            let text = fs::read_to_string(&path).expect("read a definition");
            (name, text)
        })
        .collect()
}

/// The script without its comment lines: what runs.
fn code(source: &str) -> String {
    source
        .lines()
        .filter(|line| !line.trim_start().starts_with("//"))
        .map(|line| format!("{line}\n"))
        .collect()
}

/// The top-level function `name`, from its `function` line to the `}` that closes it.
fn function<'a>(source: &'a str, name: &str) -> &'a str {
    let plain = format!("\nfunction {name}(");
    let asynchronous = format!("\nasync function {name}(");
    let at = source
        .find(&plain)
        .or_else(|| source.find(&asynchronous))
        .unwrap_or_else(|| panic!("the script has a top-level function `{name}`"));
    let body = &source[at + 1..];
    &body[..body.find("\n}\n").expect("the function closes") + 2]
}

/// The names of the script's top-level functions.
fn functions(source: &str) -> Vec<String> {
    source
        .lines()
        .filter_map(|line| {
            let rest = line
                .strip_prefix("function ")
                .or_else(|| line.strip_prefix("async function "))?;
            Some(rest[..rest.find('(')?].to_owned())
        })
        .collect()
}

/// The script's code with the top-level functions `names` cut out of it.
fn without(source: &str, names: &[&str]) -> String {
    let mut rest = source.to_owned();
    for name in names {
        let body = function(&rest, name).to_owned();
        rest = rest.replacen(&body, "", 1);
    }
    code(&rest)
}

/// The quoted values of the entries of the one-line object or list literal `const NAME = …`.
fn quoted_in(source: &str, declaration: &str) -> Vec<String> {
    let line = source
        .lines()
        .find(|line| line.starts_with(declaration))
        .unwrap_or_else(|| panic!("the script declares `{declaration}`"));
    line[declaration.len()..]
        .split('\'')
        .skip(1)
        .step_by(2)
        .map(str::to_owned)
        .collect()
}

/// A role of the script's roles table: the agent type it is launched as, and the labels
/// of the lines its prompt carries, by their key in `LABELS`.
struct Role {
    name: String,
    agent_type: String,
    labels: Vec<String>,
}

fn roles(source: &str) -> Vec<Role> {
    let at = source.find("\nconst ROLES = {\n").expect("the roles table");
    let table = &source[at..];
    let table = &table[..table.find("\n}\n").expect("the table closes")];
    table
        .lines()
        .filter(|line| line.contains("agentType: '"))
        .map(|line| {
            let name = line.trim_start();
            let name = name[..name.find(':').expect("a role has a name")].to_owned();
            let agent_type = line.split("agentType: '").nth(1).expect("an agent type");
            let agent_type = agent_type[..agent_type.find('\'').expect("it closes")].to_owned();
            let labels = line
                .split("labels: [")
                .nth(1)
                .expect("a role lists its labels");
            let labels = labels[..labels.find(']').expect("the list closes")]
                .split('\'')
                .skip(1)
                .step_by(2)
                .map(str::to_owned)
                .collect();
            Role {
                name,
                agent_type,
                labels,
            }
        })
        .collect()
}

/// The script's `LABELS`, as `key -> the line's label`.
fn labels(source: &str) -> BTreeMap<String, String> {
    let line = source
        .lines()
        .find(|line| line.starts_with("const LABELS = {"))
        .expect("the script declares its labels");
    let body = &line[line.find('{').expect("an object") + 1..line.rfind('}').expect("closed")];
    body.split(',')
        .map(|entry| {
            let (key, value) = entry.split_once(':').expect("a key and a value");
            let value = value.trim().trim_matches('\'');
            (key.trim().to_owned(), value.to_owned())
        })
        .collect()
}

// ---------------------------------------------------------------------------
// (a) every agent type resolves
// ---------------------------------------------------------------------------

#[test]
fn a_every_agent_type_the_script_names_resolves_to_a_definition() {
    let source = code(&harness());
    let defined: BTreeSet<String> = definitions().into_keys().collect();
    let mut named = BTreeSet::new();
    for (n, line) in source.lines().enumerate() {
        for rest in line.split("agentType: ").skip(1) {
            if let Some(literal) = rest.strip_prefix('\'') {
                named.insert(literal[..literal.find('\'').expect("it closes")].to_owned());
            } else {
                assert!(
                    rest.starts_with("ROLES[role].agentType"),
                    "line {}: an agent type that is neither a literal nor the roles table's: {line}",
                    n + 1
                );
            }
        }
    }
    assert!(
        named.len() >= 9,
        "the scan found the script's agent types: {named:?}"
    );
    let unresolved: Vec<&String> = named
        .iter()
        .filter(|name| *name != "general-purpose" && !defined.contains(*name))
        .collect();
    assert!(
        unresolved.is_empty(),
        "agent type(s) with no definition under {DEFINITIONS}/: {unresolved:?}"
    );
    // The roles table is where every non-git call takes its type from, so it is the census.
    let listed: BTreeSet<String> = roles(&harness())
        .into_iter()
        .map(|role| role.agent_type)
        .collect();
    let mut expected = listed.clone();
    expected.insert("build-git".to_owned());
    assert_eq!(
        named, expected,
        "the agent types the script names are its roles' and the git steps'"
    );
}

// ---------------------------------------------------------------------------
// (b) every git step runs on Sonnet
// ---------------------------------------------------------------------------

#[test]
fn b_every_build_git_call_runs_on_sonnet() {
    let full = harness();
    let source = code(&full);
    assert!(
        source.contains("\nconst GIT_MODEL = 'sonnet'\n"),
        "the script pins its git steps to Sonnet"
    );
    let calls: Vec<&str> = source
        .lines()
        .filter(|line| line.contains("'build-git'"))
        .collect();
    assert_eq!(
        calls.len(),
        2,
        "the git steps' one call, and the sync step the close returns: {calls:#?}"
    );
    for call in calls {
        assert!(
            call.contains("agentType: 'build-git'") && call.contains("model: GIT_MODEL"),
            "a build-git step without `model: GIT_MODEL`: {call}"
        );
    }
    // Every git step goes through that one call: no prompt of a git step reaches an agent
    // by another road.
    let git_prompts = git_prompts(&full);
    assert!(
        git_prompts.len() >= 10,
        "the scan found the git steps' prompts: {git_prompts:?}"
    );
    for name in &git_prompts {
        let call = format!("{name}(");
        for line in without(&full, &["selfTest", name]).lines() {
            if line.contains(&call) {
                assert!(
                    line.contains("toolStep(")
                        || line.contains("gitStep(")
                        || line.contains("sync: {"),
                    "`{name}` is a git step's prompt, and this line sends it another way: {line}"
                );
            }
        }
    }
    assert!(
        function(&full, "toolStep")
            .contains("readStep(await gitStep(label, phaseTitle, prompt, STEP_SCHEMA), act)"),
        "a step that is one command of the tool goes through `gitStep`, and so through that call"
    );
    assert_eq!(
        source.matches("agentR(").count(),
        3,
        "agentR is declared once and called by `gitStep` and `roleStep`, and by nothing else"
    );
    assert_eq!(
        source.matches("await agent(").count(),
        1,
        "the runtime's `agent` is called in one place, `agentR`"
    );
}

/// The functions that compose a git step's prompt: through `stepPrompt`, or as a list that
/// opens `GIT STEP — `.
fn git_prompts(full: &str) -> Vec<String> {
    functions(full)
        .into_iter()
        .filter(|name| name != "stepPrompt" && name != "selfTest")
        .filter(|name| {
            let body = function(full, name);
            body.contains("stepPrompt(") || body.contains("'GIT STEP — ")
        })
        .collect()
}

// ---------------------------------------------------------------------------
// (c) one function mints a branch name
// ---------------------------------------------------------------------------

#[test]
fn c_a_branch_name_is_minted_in_exactly_one_function() {
    let full = harness();
    let minting = function(&full, "branchName");
    assert_eq!(
        minting.matches("'fix/'").count(),
        1,
        "`branchName` spells the branch prefix, once"
    );
    // Everywhere else — the self-test's expected values apart — no string opens with a
    // branch prefix of the branch model, and none is assembled from a round's suffix.
    let elsewhere = without(&full, &["branchName", "selfTest"]);
    for prefix in ["fix/", "work/", "milestone/", "stabilize/"] {
        for quote in ['\'', '"', '`'] {
            let opening = format!("{quote}{prefix}");
            assert!(
                !elsewhere.contains(&opening),
                "a string outside `branchName` opens with the branch prefix `{prefix}`"
            );
        }
    }
    for suffix in ["'-r'", "'-part'"] {
        assert_eq!(
            code(&full).matches(suffix).count(),
            1,
            "the suffix {suffix} is spelled once, in `branchName`"
        );
        assert!(minting.contains(suffix), "and `branchName` is where");
    }

    // The tool that does the git acts mints none either: every branch it works on is an
    // argument the script composed. No string of its code opens with a prefix of the branch
    // model, or is a round's or a part's suffix.
    let tool = fs::read_to_string(repo_root().join(STEP_TOOL)).expect("read the step tool");
    let tool: String = tool
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .map(|line| format!("{line}\n"))
        .collect();
    assert!(
        tool.contains("\ndef push(branch, facts):\n"),
        "the scan reads the tool's code"
    );
    for prefix in ["fix/", "work/", "milestone/", "stabilize/", "-r", "-part"] {
        for quote in ['\'', '"'] {
            assert!(
                !tool.contains(&format!("{quote}{prefix}")),
                "a string of {STEP_TOOL} opens with `{prefix}`: a branch name is the harness's to mint"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// (d) the labels
// ---------------------------------------------------------------------------

/// How often `label` stands in `text`, and how often it stands there as code: `` `label` ``.
fn spellings(text: &str, label: &str) -> (usize, usize) {
    // A label is found whatever its case and whatever follows its word: `Report:`, and
    // `REPORT :`, are spellings of `REPORT:`.
    let word = label.trim_end_matches(':').to_lowercase();
    let lower = text.to_lowercase();
    let mut loose = 0;
    let mut from = 0;
    while let Some(at) = lower[from..].find(&word) {
        let start = from + at;
        let end = start + word.len();
        let before = lower[..start].chars().next_back();
        let after = lower[end..].trim_start_matches(' ').chars().next();
        let labelled = label.ends_with(':') && after == Some(':');
        let whole = !before.is_some_and(char::is_alphanumeric);
        // Only an upper-case word, or a word with its colon, is the label; `a report:` in
        // prose is neither.
        let upper = text[start..end] == label.trim_end_matches(':')[..];
        if whole && (upper || (labelled && text[start..end].chars().any(char::is_uppercase))) {
            loose += 1;
        }
        from = end;
    }
    (loose, text.matches(&format!("`{label}`")).count())
}

#[test]
fn d_each_label_is_spelled_as_every_definition_that_binds_on_it_spells_it() {
    let full = harness();
    let labels = labels(&full);
    let definitions = definitions();
    let sent: BTreeSet<&str> = labels.values().map(String::as_str).collect();
    for ruled in RULED_LABELS {
        assert!(sent.contains(ruled), "the script sends `{ruled}`");
    }

    // In the script a label is spelled once, where it is declared: every prompt takes it
    // from there. (The self-test spells its expected lines out, which is its job.)
    let elsewhere = without(&full, &["selfTest"]);
    for label in labels.values() {
        assert_eq!(
            elsewhere.matches(&format!("'{label}")).count(),
            1,
            "`{label}` opens one string of the script, in `LABELS`"
        );
    }

    // In a definition a label is always the same bytes, as code.
    for (key, label) in &labels {
        let binding: Vec<&String> = definitions
            .iter()
            .filter(|(_, text)| text.contains(&format!("`{label}`")))
            .map(|(name, _)| name)
            .collect();
        assert!(
            !binding.is_empty(),
            "no definition binds on `{label}` (the script's `{key}`)"
        );
        for (name, text) in &definitions {
            let (loose, exact) = spellings(text, label);
            assert_eq!(
                loose, exact,
                "{name}.md spells the label `{label}` {loose} time(s), {exact} of them exactly"
            );
        }
    }

    // And a role is sent only labels its definition binds on.
    let roles = roles(&full);
    assert!(roles.len() >= 10, "the scan found the roles table");
    for role in &roles {
        let Some(definition) = definitions.get(&role.agent_type) else {
            assert!(
                role.agent_type == "general-purpose" && role.labels.is_empty(),
                "`{}` has no definition, so no label can bind: {:?}",
                role.name,
                role.labels
            );
            continue;
        };
        for key in &role.labels {
            let label = labels.get(key).unwrap_or_else(|| {
                panic!(
                    "`{}` is sent the label `{key}`, which `LABELS` lacks",
                    role.name
                )
            });
            assert!(
                definition.contains(&format!("`{label}`")),
                "the role `{}` is sent `{label}`, and {}.md does not bind on it",
                role.name,
                role.agent_type
            );
        }
    }
}

// ---------------------------------------------------------------------------
// (e) the repeated paragraphs
// ---------------------------------------------------------------------------

/// The bold sentence a paragraph opens with, when it opens with one.
fn lead(paragraph: &str) -> Option<&str> {
    let rest = paragraph.strip_prefix("**")?;
    Some(&rest[..rest.find("**")?])
}

#[test]
fn e_a_paragraph_the_definitions_repeat_is_byte_identical_wherever_it_stands() {
    // A definition is read alone, so what several of them owe a stabilization run is one
    // paragraph repeated in each. The set is derived: every paragraph that opens with a
    // bold sentence about a stabilization run and stands in more than one definition.
    let definitions = definitions();
    let mut copies: BTreeMap<String, BTreeMap<String, Vec<String>>> = BTreeMap::new();
    for (name, text) in &definitions {
        for paragraph in text.lines() {
            if let Some(lead) = lead(paragraph).filter(|lead| lead.contains("stabilization run")) {
                copies
                    .entry(lead.to_owned())
                    .or_default()
                    .entry(paragraph.to_owned())
                    .or_default()
                    .push(name.clone());
            }
        }
    }
    let repeated: Vec<(&String, &BTreeMap<String, Vec<String>>)> = copies
        .iter()
        .filter(|(_, texts)| texts.values().map(Vec::len).sum::<usize>() > 1)
        .collect();
    let leads: Vec<&str> = repeated.iter().map(|(lead, _)| lead.as_str()).collect();
    for expected in [
        "In a stabilization run your report is a file, and it reaches the repository only through `dev/stabilize-record`.",
        "In a stabilization run you drive the binary you are handed, never one you build.",
        "What a finding carries in a stabilization run",
    ] {
        assert!(
            leads.contains(&expected),
            "the scan found the repeated paragraph `{expected}`: {leads:#?}"
        );
    }
    for (lead, texts) in repeated {
        assert_eq!(
            texts.len(),
            1,
            "the paragraph `{lead}` differs between definitions: {:#?}",
            texts.values().collect::<Vec<_>>()
        );
    }
}

// ---------------------------------------------------------------------------
// (f) the human's rulings enter in one place
// ---------------------------------------------------------------------------

#[test]
fn f_the_humans_rulings_are_written_by_one_step_and_no_other() {
    let full = harness();
    let step = function(&full, "rulingsRecordPrompt");
    assert!(
        step.contains("call('bound-set --run ") && step.contains("call('ledger-set --run "),
        "the rulings step writes the declared-bounds list and the dispositions"
    );
    assert!(
        step.contains("disposition: r.ruling"),
        "and the disposition it writes is the ruling it was handed"
    );

    // No other prompt carries the bounds list's writer.
    assert!(
        !without(&full, &["rulingsRecordPrompt", "selfTest"]).contains("bound-set"),
        "`bound-set` stands in the rulings step and nowhere else"
    );
    // The three dispositions are spelled once, in the vocabulary a ruling is checked
    // against; no other code of the script can write one, because none spells one.
    assert_eq!(
        quoted_in(&full, "const HUMAN_RULINGS = "),
        HUMAN_DISPOSITIONS,
        "the script's vocabulary of rulings is ruling 4's three"
    );
    // The vocabulary is read where the argument is checked, and by nothing that writes.
    let readers: Vec<String> = functions(&full)
        .into_iter()
        .filter(|name| function(&full, name).contains("HUMAN_RULINGS"))
        .collect();
    assert_eq!(
        readers,
        ["validateRulings", "selfTest"],
        "the functions that read the vocabulary of rulings"
    );
    // Outside the step, a disposition a prompt carries is a literal of the script's own —
    // never a value handed in.
    let elsewhere = without(&full, &["rulingsRecordPrompt", "selfTest"]);
    for rest in elsewhere.split("disposition: ").skip(1) {
        assert!(
            rest.starts_with('\''),
            "a disposition composed from a value, outside the rulings step: `disposition: {}`",
            &rest[..rest.len().min(40)]
        );
    }
    // The other writer of a disposition — a stage's record — writes what a fixer or a
    // carry-over established: `fixed` and `open`.
    let others: BTreeSet<String> = without(&full, &["rulingsRecordPrompt", "selfTest"])
        .split("disposition: '")
        .skip(1)
        .map(|rest| rest[..rest.find('\'').expect("it closes")].to_owned())
        .collect();
    assert_eq!(
        others,
        BTreeSet::from(["fixed".to_owned(), "open".to_owned()]),
        "the dispositions any other code of the script composes"
    );
    // And the step is reached from one place: where the `rulings` argument is.
    let callers: Vec<String> = without(&full, &["rulingsRecordPrompt", "selfTest"])
        .lines()
        .filter(|line| line.contains("rulingsRecordPrompt("))
        .map(str::to_owned)
        .collect();
    assert_eq!(callers.len(), 1, "one caller: {callers:#?}");
    assert!(
        callers[0].contains("v.rulings"),
        "which hands it the argument: {}",
        callers[0]
    );
}

// ---------------------------------------------------------------------------
// (g) the cross-model pass, only for an item the invocation names
// ---------------------------------------------------------------------------

/// The tool of the cross-model pass, lower-cased — asked of the script, never spelled here
/// twice.
fn cross_model_tool(full: &str) -> String {
    let body = function(full, "crossModelTool");
    let name = body.split('\'').nth(1).expect("the tool's name");
    assert!(!name.is_empty() && name.chars().all(|c| c.is_ascii_lowercase()));
    name.to_owned()
}

#[test]
fn g_the_cross_model_pass_is_launched_only_for_an_item_the_invocation_names() {
    let full = harness();
    let tool = cross_model_tool(&full);
    let own: Vec<String> = functions(&full)
        .into_iter()
        .filter(|name| name.starts_with("crossModel"))
        .collect();
    assert!(own.len() >= 3, "the cross-model functions: {own:?}");
    let own: Vec<&str> = own.iter().map(String::as_str).collect();

    // Whatever the script says about the tool, it says in those functions.
    let elsewhere = without(&full, &own);
    assert!(
        !elsewhere.to_lowercase().contains(&tool),
        "the tool of the cross-model pass is named outside the `crossModel*` functions"
    );
    // The description every permission dialog shows names it not at all.
    assert!(
        !full[..full.find("\n// ---- constants").expect("the constants")]
            .lines()
            .filter(|line| !line.starts_with("//"))
            .any(|line| line.to_lowercase().contains(&tool))
    );

    // Each of them is called only where the naming is tested: a call sits on a line that
    // asks whether this step, or this plan, is a cross-model one.
    let mut calls = 0;
    for line in without(&full, &[&own[..], &["selfTest"]].concat()).lines() {
        for name in &own {
            if line.contains(&format!("{name}(")) {
                calls += 1;
                assert!(
                    line.contains("crossModel ? "),
                    "`{name}` is called on a line that does not test the opt-in: {line}"
                );
            }
        }
    }
    assert!(
        calls >= 2,
        "the prompt and the preflight's question are called"
    );

    // A chain has the step only under a strict `true`, which a caller derives from the
    // item's own name — and the argument is a list of items, with no value for all of them.
    let chain = function(&full, "chainOf");
    assert!(
        chain.contains("!s.crossModel || crossModel === true"),
        "`chainOf` keeps a cross-model step under a strict true only"
    );
    let source = code(&full);
    let chains: Vec<&str> = source
        .lines()
        .filter(|line| line.contains("chainOf(") && !line.starts_with("function "))
        .collect();
    let selftest = function(&full, "selfTest");
    let mut built = 0;
    for line in chains.iter().filter(|line| !selftest.contains(**line)) {
        for call in line.split("chainOf(").skip(1) {
            // The call's arguments: up to the parenthesis that closes it.
            let mut depth = 1;
            let end = call
                .char_indices()
                .find(|(_, c)| {
                    depth += i32::from(*c == '(') - i32::from(*c == ')');
                    depth == 0
                })
                .map_or(call.len(), |(at, _)| at);
            let arguments = &call[..end];
            built += 1;
            assert!(
                arguments.ends_with(", true")
                    || arguments.ends_with(", false")
                    || arguments.ends_with("&& crossNamed.includes(i.item)"),
                "a chain is built for one item, by whether the invocation named it: `chainOf({arguments})`"
            );
        }
    }
    assert!(
        built >= 3,
        "the scan found the chains the test stage builds"
    );
    let validation = function(&full, "validateArgs");
    assert!(
        validation.contains(
            "Array.isArray(a.crossModel) && a.crossModel.length > 0 && a.crossModel.every(isSlug)"
        ),
        "`crossModel` is a non-empty list of item ids, and nothing else"
    );

    // And no definition the script launches assumes the tool: without a named item, a
    // machine that lacks it runs the whole workflow.
    let definitions = definitions();
    for role in roles(&full) {
        if let Some(text) = definitions.get(&role.agent_type) {
            assert!(
                !text.to_lowercase().contains(&tool),
                "{}.md names the tool of the cross-model pass",
                role.agent_type
            );
        }
    }
    assert!(!definitions["build-git"].to_lowercase().contains(&tool));
}

// ---------------------------------------------------------------------------
// (h) the self-test launches nothing, and the runtime's limits
// ---------------------------------------------------------------------------

#[test]
fn h_the_self_test_launches_no_agent_and_the_script_keeps_the_runtimes_limits() {
    let full = harness();
    // Everything above the args is declarations and pure functions: the self-test calls
    // only what stands there, and nothing there can launch or wait for an agent.
    let pure = &full[..full
        .find("\n// ---- args: parsed, and refused before any agent ----\n")
        .expect("the script's first statement that runs")];
    let pure = code(pure);
    for launching in [
        "await ",
        "agent(",
        "agentR(",
        "gitStep(",
        "roleStep(",
        "parallel(",
        "pipeline(",
        "workflow(",
    ] {
        assert!(
            !pure.contains(launching),
            "`{launching}` stands above the script's first statement, among what the self-test may call"
        );
    }
    let after = &full[full.find("\n// ---- args: parsed").expect("as above")..];
    let first: Vec<String> = code(after)
        .lines()
        .filter(|line| line.starts_with("if (") || line.starts_with("return "))
        .take(3)
        .map(str::to_owned)
        .collect();
    assert_eq!(
        first,
        [
            "if (typeof args === 'string') {",
            "if (refusal) {",
            "if (parsedArgs.selfTest) return selfTest()",
        ],
        "the args are refused, and the self-test returned, before anything else runs"
    );

    // What the runtime forbids a script, because it would break a resume — and what it
    // requires of one.
    let source = code(&full);
    for forbidden in [
        "Date.now(",
        "Math.random(",
        "new Date(",
        "require(",
        "import ",
    ] {
        assert!(!source.contains(forbidden), "the script uses `{forbidden}`");
    }
    assert!(
        source
            .trim_start()
            .starts_with("export const meta = {\n  name: 'stabilize',\n"),
        "the script opens with its `meta`, a literal, under the name it is invoked by"
    );
    let meta = &source[..source.find("\n}\n").expect("meta closes")];
    assert!(
        !meta.contains('+') && !meta.contains('`') && !meta.contains("${"),
        "`meta` is a pure literal"
    );
}

// ---------------------------------------------------------------------------
// (j) what the record script decides from, and what it decides
// ---------------------------------------------------------------------------

#[test]
fn j_what_the_record_script_decides_from_reaches_it_and_what_it_decides_is_passed_on() {
    let full = harness();
    let source = code(&full);

    // EVIDENCE BELONGS TO A TEST-SET ITEM, and a clause's status is derived by the record
    // script: this script composes one result per item it ran — in one function, called by
    // the `test` stage and by nothing else — and no call of it names a clause's status.
    assert!(
        !source.contains("clause-set") && !source.contains("clauseRows"),
        "no code of the script writes a clause's row: its status is the record script's to derive"
    );
    assert_eq!(
        source.matches("resultRows(").count()
            - function(&full, "selfTest").matches("resultRows(").count(),
        2,
        "what an item did is composed in one function, with one caller: the `test` stage's record"
    );
    assert!(
        function(&full, "runTest")
            .contains("results: { commit: sha, rows: resultRows(unitStatus) }")
            && function(&full, "stageRecordCommands").contains("'result-set --run '"),
        "the `test` stage's record holds one result per item it ran"
    );
    // The results come before anything of the ledger: the record script takes a re-run
    // only while its state asks for one, and a finding the re-run found ends that.
    let composed = function(&full, "stageRecordCommands");
    let placed = |call: &str| {
        composed
            .find(call)
            .unwrap_or_else(|| panic!("`{call}` is no call of a stage's record"))
    };
    assert!(
        placed("'result-set --run '") < placed("'ledger-add --run '")
            && placed("'result-set --run '") < placed("'triage-set --run '")
            && placed("'result-set --run '") < placed("'round-set --run '"),
        "a stage's results are written before its ledger rows, its triage and the round's facts"
    );
    // A RE-RUN IS THE POSITION'S: the state asks for it and names its clause, its round and
    // its items; the invocation's `clause` only has to agree. It begins no round, resolves
    // no scope, and writes no fact of the round it runs inside.
    let tested = function(&full, "runTest");
    assert!(
        tested.contains("\n  const rerun = at.rerun || null\n")
            && tested
                .contains("if (rerun && v.clause !== rerun.clause) return { status: 'refused'")
            && tested.contains("if (!rerun && v.clause != null) return { status: 'refused'"),
        "a re-run is read from the position; a `clause` the state does not ask for, and a re-run answered without it, are refused"
    );
    assert_eq!(
        source.matches("v.clause").count(),
        3,
        "the invocation's `clause` is compared with the position's, and decides nothing else"
    );
    assert!(
        tested.contains(
            "const facts = rerun ? null : { candidate: sha, binary: built.candidate.sha256 }"
        ) && tested.contains("if (!rerun) steps.push(() => roleStep('scope', 'scope',")
            && tested.contains("gate: rerun ? null : { commit: sha, file: candidateGate }")
            && !source.contains("alone"),
        "a re-run writes no fact of the round, resolves no scope and records no candidate's gate"
    );

    // A round's triage records the disposition a row carried when the finding was found.
    // A fix cycle's audit finds it after the cycle's fix, so the fix is on the row first.
    let record = function(&full, "stageRecordCommands");
    let at = |payload: &str| {
        record
            .find(payload)
            .unwrap_or_else(|| panic!("a stage's record composes `{payload}`"))
    };
    assert!(
        at("call('ledger-add --run ") < at("call('ledger-set --run ")
            && at("call('ledger-set --run ") < at("call('triage-set --run "),
        "a stage's record writes its rows, then its dispositions, then its triage"
    );

    // Every word the record script refuses a stage with has its sentence here — and the
    // script has a sentence for no word that script lacks.
    let script =
        fs::read_to_string(repo_root().join(RECORD_SCRIPT)).expect("read the record script");
    let position = &script[script
        .find("\ndef position_of(")
        .expect("the record script computes the position")..];
    let position = &position[1..];
    let position = &position[..position.find("\ndef ").expect("the function ends")];
    let refused: BTreeSet<String> = position
        .split("refused(\"")
        .skip(1)
        .map(|rest| rest[..rest.find('"').expect("the word closes")].to_owned())
        .collect();
    assert!(
        refused.len() >= 6,
        "the scan found the words a stage is refused with: {refused:?}"
    );
    let table = &full[full
        .find("\nconst REFUSALS = {\n")
        .expect("the script's sentences for a refusal")..];
    let table = &table[..table.find("\n}\n").expect("the table closes")];
    let sentences: BTreeSet<String> = table
        .lines()
        .filter_map(|line| line.strip_prefix("  '"))
        .map(|rest| rest[..rest.find('\'').expect("the word closes")].to_owned())
        .collect();
    assert_eq!(
        sentences, refused,
        "the words {HARNESS} has a sentence for (left) are the words {RECORD_SCRIPT} refuses a stage with (right)"
    );
    assert!(
        function(&full, "refusalOf").contains("REFUSALS[at.refused]"),
        "and a refusal is said in the sentence of its own word"
    );

    // `next` is relayed as the state document gives it. The values this script acts on
    // are the three it was built with; every other value goes back with the state.
    assert_eq!(
        quoted_in(&full, "const KNOWN_NEXT = "),
        ["fix", "rule", "close"],
        "the values of `next` the script acts on"
    );
    assert_eq!(
        without(&full, &["selfTest"]).matches("next: '").count(),
        0,
        "no code of the script composes a value of `next`"
    );
}

// ---------------------------------------------------------------------------
// (k) the table's holes, closed: one road each
// ---------------------------------------------------------------------------

#[test]
fn k_what_the_human_rules_about_the_run_an_unfinished_triage_and_the_openings_facts_each_have_one_road()
 {
    let full = harness();
    let source = code(&full);
    let test_stage = function(&full, "runTest");
    let fix_stage = function(&full, "runFix");

    // The human's go after a stop, one more re-run of a clause, one more triage of a
    // finding, one more attempt of a stage, and either bound raised are facts of the record
    // — and the rulings step writes them, and nothing else does.
    let step = function(&full, "rulingsRecordPrompt");
    for written in [
        "value: { go: true }",
        "value: { granted: r.rerun }",
        "value: { reverify: r.reverify }",
        "value: { again: r.again }",
        "value: { rounds: r.rounds }",
        "value: { cycles: r.cycles }",
        "'round-set --run '",
        "'run-set --run '",
    ] {
        assert!(
            step.contains(written),
            "the rulings step writes `{written}`"
        );
    }
    let elsewhere = without(&full, &["rulingsRecordPrompt", "selfTest"]);
    for fact in [
        "go: true",
        "granted:",
        "reverify: r",
        "again: r",
        "run-set --run",
    ] {
        assert!(
            !elsewhere.contains(fact),
            "`{fact}` stands outside the rulings step: a go, a grant or a raised bound has a second writer"
        );
    }

    // An invocation that carries them records them and starts nothing: each stage hands
    // over before it reads its position, and what it hands over to launches no instrument,
    // no fixer and no landing.
    // The `test` stage hands over every invocation that carries rulings — the ones about
    // the run, and the ones on findings and bounds, which it records and does nothing with;
    // the `fix` stage hands over the ones about the run, and records the others itself.
    for (name, stage, hand_over) in [
        (
            "runTest",
            test_stage,
            "\n  if (v.rulings) return await ruleTheRun(state, gs.branch)\n",
        ),
        (
            "runFix",
            fix_stage,
            "\n  if (v.rulings && v.rulings.every(runRuling)) return await ruleTheRun(state, gs.branch)\n",
        ),
    ] {
        let at = stage.find(hand_over).unwrap_or_else(|| {
            panic!("`{name}` hands a ruling about the run over, and returns what comes back")
        });
        let position = stage
            .find("state.position")
            .expect("a stage reads its position");
        assert!(
            at < position,
            "`{name}` hands a ruling about the run over before it reads its position"
        );
    }
    let rule = function(&full, "ruleTheRun");
    assert!(
        rule.contains("runRulingsFault(v.rulings, state)")
            && rule.contains("findingRulingsFault(v.rulings, state)")
            && rule.contains("checkedOut !== loopBranch")
            && rule.matches("await rulingsStep(").count() == 1,
        "a ruling is asked of the state, on the loop branch, then recorded by the one step"
    );
    for launching in [
        "runUnits(",
        "preflightOf(",
        "triagePasses(",
        "fixerPrompt(",
        "landPrompt(",
        "finishTriage(",
    ] {
        assert!(
            !rule.contains(launching),
            "`ruleTheRun` starts something: `{launching}`"
        );
    }

    // An unfinished triage is finished where the record script's position says so — and
    // by a function that grades, verifies and records, and runs no instrument.
    let finish = function(&full, "finishTriage");
    assert!(
        finish.contains("ledgerSource(v.run, state)")
            && finish.contains("await triagePasses(")
            && finish.contains("stageRecordCommands(")
            && finish.contains("await readState()"),
        "`finishTriage` hands the rows that await triage to the triage passes, records them, and reads the state back"
    );
    for running in [
        "runUnits(",
        "scopePrompt(",
        "fixerPrompt(",
        "resultRows(",
        "unitPrompt(",
    ] {
        assert!(
            !finish.contains(running),
            "`finishTriage` runs an instrument, a scope or a fixer: `{running}`"
        );
    }
    assert!(
        test_stage.contains("\n  if (at.triage) {\n")
            && fix_stage.contains("\n    if (at.triage) {\n"),
        "each stage finishes a triage exactly where its position says `triage`"
    );
    assert_eq!(
        source.matches("await finishTriage(").count(),
        2,
        "one call in each stage"
    );
    for stage in [test_stage, fix_stage] {
        let asked = stage.find("at.triage").expect("as above");
        let called = stage.find("await finishTriage(").expect("as above");
        assert!(
            asked < called && !stage[asked..called].contains("runUnits("),
            "between the position's word and the call no instrument runs"
        );
    }
    // And every triage — a stage's own, and a finishing one — is handed the rows of the
    // ledger whose triage nobody finished: seeded at the opening, or left without a verdict.
    assert_eq!(
        source.matches("await triagePasses(").count(),
        3,
        "the test stage's triage, a fix cycle's, and the finishing one"
    );
    assert_eq!(
        source.matches("ledgerSource(v.run, state)").count(),
        3,
        "and each of the three is handed the ledger's rows"
    );
    assert!(
        test_stage.contains("for (const source of ledgerSource(v.run, state)) sources.push(source)\n  const tri = await triagePasses(ctx, launch, built, sources, [])"),
        "the test stage's triage"
    );
    assert!(
        fix_stage.contains("const sources = fixerSources.concat(ledgerSource(v.run, state))"),
        "a fix cycle's triage"
    );
    let ledger = function(&full, "ledgerSource");
    assert!(
        ledger.contains("(state.untriaged || []).map(") && !ledger.contains(".round"),
        "the rows are the state's `untriaged`, whatever round or stage they came from"
    );

    // The previous release and the default scope are the run's recorded facts: no prompt
    // sends an agent to read either out of the opening record's prose.
    assert!(
        !without(&full, &["selfTest"]).contains("opening record names"),
        "a prompt still says that the opening record names something"
    );
    let previous = function(&full, "previousOf");
    assert!(
        previous.contains("facts.previous") && previous.contains("facts['previous-commit']"),
        "the previous release is read off the state's facts"
    );
    let preflight = function(&full, "preflightPrompt");
    assert!(
        preflight.contains("plan.previous.version") && preflight.contains("plan.previous.commit"),
        "the preflight is told the previous release's version and commit"
    );
    let scope = function(&full, "scopePrompt");
    assert!(
        scope.contains("plan.previous.commit") && scope.contains("plan.fallback"),
        "the scope step is told the previous release's commit and the default scope"
    );
    let mut preflights = 0;
    for line in source
        .lines()
        .filter(|line| line.contains("preflightOf(ctx") && !line.starts_with("async function "))
    {
        preflights += 1;
        assert!(
            line.contains("previous: previousOf(state)"),
            "a preflight is not handed the run's previous release: {line}"
        );
    }
    assert!(preflights >= 4, "the scan found the preflight calls");
    assert!(
        function(&full, "preflightOf").contains("r.previous.version !== plan.previous.version"),
        "and the binary it returns is held to that release"
    );
    assert!(
        source.contains("scope: v.scope, previous: previousOf(state), fallback: state.facts.scope"),
        "the default scope is the state's fact"
    );

    // Closing is the human's to confirm with the evidence in front of them: per clause,
    // how far behind the candidate its last evidence is.
    assert!(
        function(&full, "nextOf").contains("close: closeOf(), evidence: evidenceOf(state)"),
        "`close` goes back with each clause's evidence"
    );
    let evidence = function(&full, "evidenceOf");
    for field in [
        "behind: c.behind",
        "round: c.round",
        "items: (c.items || [])",
        "standing: i.standing",
    ] {
        assert!(evidence.contains(field), "the evidence carries `{field}`");
    }

    // And the verifier's definition says what the record script computes: a finding it
    // left without a verdict leaves the round's triage unfinished.
    let verifier = &definitions()["finding-verifier"];
    assert!(
        verifier.contains("the round's triage is not finished")
            && !verifier.contains("hands to the human"),
        "finding-verifier.md still says that an unverified finding goes to the human"
    );
    let triage = &definitions()["finding-triage"];
    assert!(
        triage.contains("`ledger`"),
        "finding-triage.md says what the source `ledger` is"
    );
}

// ---------------------------------------------------------------------------
// (n) the record step: accepted on the commit step's line, and the executor commits nothing
// ---------------------------------------------------------------------------

#[test]
fn n_a_record_is_accepted_on_its_commit_steps_own_line_and_the_executor_commits_nothing() {
    let full = harness();
    let source = code(&full);

    // One function runs a record step: the executor first, then the ONE commit, a git step
    // that is handed what the batch was composed of.
    let step = function(&full, "recordStep");
    let executor = step
        .find("await roleStep('record', ")
        .expect("a record step launches its executor");
    let commit = step
        .find("await toolStep('record:' + label, 'Record', 'record', recordCommitPrompt(v, branch, rec.gate, rec.expect))")
        .expect("a record step's commit is the tool's `record` act, handed the gate's file and what was composed");
    assert!(
        executor < commit && step.contains("recordFault(r, rec.expect)"),
        "the executor, then the commit, then the line held to what was composed"
    );
    assert_eq!(
        source.matches("roleStep('record', ").count(),
        1,
        "the record's executor is launched in one place"
    );
    assert_eq!(
        source.matches("recordCommitPrompt(").count()
            - function(&full, "selfTest")
                .matches("recordCommitPrompt(")
                .count(),
        2,
        "and its commit is asked for in one place"
    );

    // What is accepted: the commit, the gate check that holds, and one result per check
    // composed, each of which holds.
    let fault = function(&full, "recordFault");
    for held in [
        "r.status !== 'recorded'",
        "SHA_RE.test(String(r.commit || ''))",
        "r.gate.ok !== true",
        "applied.calls !== expect.calls",
        "applied.checks.length !== expect.checks",
        "c.ok !== true",
    ] {
        assert!(fault.contains(held), "`recordFault` holds `{held}`");
    }
    assert!(
        function(&full, "recordPrompt")
            .contains("expect: { calls: calls.length, checks: calls.filter(isCheck).length }"),
        "what a record is held to is counted off the calls the script composed"
    );

    // The executor's prompt spells no commit: it applies the batch and runs the gate.
    for name in ["recordPrompt", "pendingRecordPrompt", "gateStep"] {
        let body = function(&full, name);
        assert!(
            !body.contains("git commit")
                && !body.contains("git add")
                && !body.contains("ONE commit of the paths"),
            "`{name}` has the executor commit"
        );
    }
    assert!(
        source.contains("\nconst RECORD_RETURNS = 'YOU MAKE NO COMMIT, and stage nothing: ")
            && function(&full, "recordPrompt").contains("    RECORD_RETURNS,\n")
            && function(&full, "pendingRecordPrompt").contains("    RECORD_RETURNS,\n"),
        "both prompts of a record step end by saying so"
    );
    assert!(
        function(&full, "recordPrompt").contains("'2. `dev/stabilize-record apply --run '")
            && !function(&full, "pendingRecordPrompt").contains("stabilize-record apply"),
        "a record's calls are ONE batch, and a batch that is applied already is not applied again"
    );
    let executor = &definitions()["build-executor"];
    assert!(
        executor.contains("no commit of yours")
            && executor.contains("a red gate is not your halt here")
            && !executor.contains("its `GATE: PASS`"),
        "build-executor.md still has the record step's executor commit, or halt on a red gate"
    );

    // An invocation that finds a batch applied and not committed finishes it first — before
    // it reads the run's state — and what finishes it starts nothing.
    for name in ["runTest", "runFix"] {
        let stage = function(&full, name);
        let found = stage
            .find("\n  if (gs.pending) return await finishRecord(gs)\n")
            .unwrap_or_else(|| panic!("`{name}` finishes a pending batch"));
        let read = stage.find("readState()").expect("a stage reads the state");
        assert!(
            found < read,
            "`{name}` finishes a pending batch before it reads the state"
        );
    }
    let finish = function(&full, "finishRecord");
    assert!(
        finish.contains("await recordStep('pending', gs.branch, pendingRecordPrompt("),
        "a pending batch is finished by a record step that applies nothing"
    );
    for launching in [
        "runUnits(",
        "preflightOf(",
        "triagePasses(",
        "rulingsStep(",
        "scopePrompt(",
        "landPrompt(",
        "finishTriage(",
    ] {
        assert!(
            !finish.contains(launching),
            "`finishRecord` starts something: `{launching}`"
        );
    }
}

// ---------------------------------------------------------------------------
// (l) every git step is one command of the tool
// ---------------------------------------------------------------------------

/// The acts `dev/stabilize-step` has, read from its own parser.
fn tool_acts() -> BTreeSet<String> {
    let tool = fs::read_to_string(repo_root().join(STEP_TOOL)).expect("read the step tool");
    tool.lines()
        .filter_map(|line| line.strip_prefix("    act(\""))
        .map(|rest| rest[..rest.find('"').expect("the act's name closes")].to_owned())
        .collect()
}

#[test]
fn l_every_git_step_is_one_command_of_the_tool_but_the_recut_of_a_part() {
    let full = harness();
    assert!(
        code(&full).contains(&format!("\nconst STEP_TOOL = '{STEP_TOOL}'\n")),
        "the script names the tool once"
    );
    let acts = tool_acts();
    assert!(acts.len() >= 10, "the scan found the tool's acts: {acts:?}");

    // One function composes every such prompt: the command, and "relay its line".
    let composer = function(&full, "stepPrompt");
    assert!(
        composer.contains("'1. `' + STEP_TOOL + ' ' + act + ' ' + flags + '`',")
            && composer.contains("'GIT STEP — ' + what + '. ' + STEP_RULES,")
            && composer.matches("\n    '").count() == 3,
        "`stepPrompt` is the step's opening, ONE command and its report: {composer}"
    );

    // Each act is asked for by exactly one prompt function, and each prompt function asks
    // for exactly one act.
    let prompts = git_prompts(&full);
    let mut asked: BTreeMap<String, String> = BTreeMap::new();
    for name in &prompts {
        let body = function(&full, name);
        let composed: Vec<&String> = acts
            .iter()
            .filter(|act| body.contains(&format!("', '{act}', ")))
            .collect();
        assert_eq!(
            composed.len(),
            1,
            "`{name}` composes one command of the tool: {composed:?}"
        );
        assert_eq!(
            body.matches("stepPrompt(").count(),
            1,
            "`{name}` calls `stepPrompt` once"
        );
        let taken = asked.insert(composed[0].clone(), name.clone());
        assert!(
            taken.is_none(),
            "the act `{}` is composed by `{name}` and by `{}`",
            composed[0],
            taken.unwrap_or_default()
        );
    }
    assert_eq!(
        asked.keys().cloned().collect::<BTreeSet<_>>(),
        acts,
        "the acts the script asks for (left) are the acts {STEP_TOOL} has (right)"
    );

    // A step that is one command lists no git command of its own. The one function that
    // still does is the re-cut of a part — ruled to be rebuilt as a revert, so its list is
    // kept as it was and not ported.
    let listing: Vec<&String> = prompts
        .iter()
        .filter(|name| function(&full, name).contains("`git "))
        .collect();
    assert_eq!(
        listing,
        ["carryPrompt"],
        "the prompt functions that still list git commands"
    );
    let recut = function(&full, "carryPrompt");
    assert!(
        recut.contains("\n  if (!part) return stepPrompt(")
            && recut.contains("'GIT STEP — re-cut a part of round '"),
        "`carryPrompt` is the tool's act for a dropped round, and the list for a part"
    );
    assert_eq!(
        code(&full).matches("'GIT STEP — ").count()
            - function(&full, "selfTest").matches("'GIT STEP — ").count(),
        2,
        "a git step's opening is spelled by `stepPrompt` and by the part's re-cut, and by no other"
    );

    // And each call names the act its prompt composes, so the line it reads back is that
    // act's: a call that named another would halt on every line.
    let mut calls = 0;
    for line in without(&full, &["selfTest", "toolStep"]).lines() {
        let Some(rest) = line.split("await toolStep(").nth(1) else {
            continue;
        };
        calls += 1;
        let prompt = prompts
            .iter()
            .find(|name| rest.contains(&format!("{name}(")))
            .unwrap_or_else(|| panic!("a tool step sends a git step's prompt: {line}"));
        let act = asked
            .iter()
            .find(|(_, name)| *name == prompt)
            .map(|(act, _)| act)
            .expect("every prompt function composes an act");
        assert!(
            rest.contains(&format!(", '{act}', {prompt}(")),
            "this call sends `{prompt}`, which composes `{act}`, and reads another act's line: {line}"
        );
    }
    assert!(calls >= 15, "the scan found the tool steps: {calls}");
    // The part's re-cut is the one step that is not read through the tool.
    let direct: Vec<&str> = code(&full)
        .lines()
        .filter(|line| line.contains("await gitStep("))
        .collect::<Vec<_>>()
        .into_iter()
        .map(|line| {
            if line.contains("carryPrompt(v, round, take, part), CARRY_SCHEMA)") {
                "the part's re-cut"
            } else if line
                .contains("readStep(await gitStep(label, phaseTitle, prompt, STEP_SCHEMA), act)")
            {
                "toolStep"
            } else {
                "another"
            }
        })
        .collect();
    assert_eq!(
        direct,
        ["toolStep", "the part's re-cut"],
        "who calls `gitStep`"
    );
}

// ---------------------------------------------------------------------------
// (m) a stage that is not fit for use refuses to start
// ---------------------------------------------------------------------------

/// The words a `DECISIONS.md` heading records a half's re-review with — the record that
/// lifts a stage's refusal.
fn re_review_heading(stage: &str) -> String {
    format!("the `{stage}` half of the stabilization workflow, repaired and re-reviewed")
}

/// The stages the script's `NOT_FIT` names.
fn not_fit(full: &str) -> Vec<String> {
    let table = &full[full
        .find("\nconst NOT_FIT = {\n")
        .expect("the script's table of the stages that refuse to start")..];
    table["\nconst NOT_FIT = {\n".len()..]
        .lines()
        .take_while(|line| *line != "}")
        .filter_map(|line| line.strip_prefix("  "))
        .filter_map(|line| line.split_once(": '").map(|(stage, _)| stage.to_owned()))
        .collect()
}

#[test]
fn m_a_stage_that_is_not_fit_for_use_refuses_to_start() {
    let full = harness();
    let source = code(&full);
    let refusing = not_fit(&full);

    // WHICH stages refuse is held to the record: the `fix` stage, until DECISIONS.md has a
    // heading that records its half as repaired and re-reviewed. Lifting the refusal is the
    // one edit that deletes the stage's line from `NOT_FIT`, made by the commit that writes
    // that heading — and no edit to this arm.
    let decisions =
        fs::read_to_string(repo_root().join("DECISIONS.md")).expect("read DECISIONS.md");
    let stage = "fix";
    let words = re_review_heading(stage);
    let recorded = decisions
        .lines()
        .any(|line| line.starts_with("## ") && line.contains(&words));
    let refuses = refusing.iter().any(|named| named == stage);
    assert!(
        refuses || recorded,
        "the `{stage}` stage no longer refuses to start, and DECISIONS.md has no heading \
         with the words `{words}`: the refusal is lifted by the commit that records that \
         re-review, and by no other"
    );
    assert!(
        !(refuses && recorded),
        "DECISIONS.md records `{words}`, and the `{stage}` stage still refuses to start: \
         delete its line from `NOT_FIT` in {HARNESS}"
    );
    for stage in &refusing {
        assert!(
            quoted_in(&full, "const STAGES = ").contains(stage),
            "`NOT_FIT` names `{stage}`, which is no stage"
        );
    }

    // The refusal is asked of every invocation, after the arguments are refused and the
    // self-test returned and BEFORE anything that could launch an agent is declared or run.
    let asked = "\nif (parsedArgs.selfTest) return selfTest()\nconst unfit = notFit(parsedArgs)\nif (unfit) {\n  return { status: 'refused', ";
    let at = source
        .find(asked)
        .expect("the stage's fitness is asked right after the self-test's return");
    for launching in [
        "await ",
        "agent(",
        "agentR(",
        "gitStep(",
        "toolStep(",
        "roleStep(",
    ] {
        assert!(
            !source[..at + asked.len()].contains(launching),
            "`{launching}` stands before the refusal of a stage that is not fit for use"
        );
    }
    assert_eq!(
        source.matches("notFit(parsedArgs)").count(),
        1,
        "and it is asked once, of the arguments as parsed"
    );

    // What it lets through: a stage the table does not name — and, of a stage it names,
    // only an invocation whose every ruling is about the run, which starts nothing.
    let asks = function(&full, "notFit");
    assert!(
        asks.contains("\n  if (!NOT_FIT[a.stage]) return null\n  if (a.rulings != null && a.rulings.every(runRuling)) return null\n  return '"),
        "`notFit` passes a stage that is fit, and an invocation that only records what the human ruled about the run: {asks}"
    );
    assert!(
        asks.contains("BUILD_RECORD") && asks.contains("Nothing was run"),
        "and its refusal points at the build's record and says that nothing ran"
    );
    let record = quoted_in(&full, "const BUILD_RECORD = ");
    assert_eq!(record.len(), 1, "the build's record is one path");
    assert!(
        repo_root().join(&record[0]).is_file(),
        "the build's record the refusal points at exists: {}",
        record[0]
    );

    // Driven, where `node` is installed: every invocation of a stage that is not fit is
    // refused with that word, and no agent is launched for it.
    if !node_or_skip(&format!(
        "the refusal of a stage that is not fit for use was read off {HARNESS} and not run"
    )) {
        return;
    }
    let scratch = ScratchDir::new("stabilize-not-fit");
    let driver = scratch.path().join("driver.mjs");
    fs::write(&driver, DRIVER).expect("write the driver");
    for stage in &refusing {
        let mut invocations = vec![
            String::new(),
            r#", "model": "sonnet""#.to_owned(),
            r#", "rulings": [{"key": "f-1", "ruling": "later"}]"#.to_owned(),
            r#", "rulings": [{"bound": "b", "reach": "x", "where": "y", "pin": "unpinned"}]"#
                .to_owned(),
        ];
        if stage == "fix" {
            for more in [
                r#", "exit": "drop""#,
                r#", "exit": {"part": ["abcdef1"]}"#,
                r#", "raise": {"cycles": 4}"#,
                r#", "stopAfter": "state""#,
                r#", "rulings": [{"key": "f-1", "ruling": "admitted"}], "stopAfter": "rulings""#,
            ] {
                invocations.push(more.to_owned());
            }
        }
        for more in invocations {
            let args = format!(
                r#"{{"stage": "{stage}", "run": "rc24-tier1", "scratch": "/tmp/scratch-1"{more}}}"#
            );
            let seen = invoke(&driver, &args);
            assert_eq!(seen["status"], "refused", "args {args}: {seen}");
            assert_eq!(
                seen["not_fit"]["stage"],
                stage.as_str(),
                "args {args}: {seen}"
            );
            assert_eq!(
                seen["not_fit"]["record"],
                record[0].as_str(),
                "args {args}: {seen}"
            );
            assert!(
                seen["message"]
                    .as_str()
                    .is_some_and(|message| message.contains("NOT FIT FOR USE")
                        && message.contains(&record[0])
                        && message.contains("Nothing was run")),
                "args {args}: {seen}"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// (i) driven under node, where there is one
// ---------------------------------------------------------------------------

/// The script's body as an async function, the runtime's globals handed in — and an
/// `agent` that fails the run if anything launches one.
const DRIVER: &str = r#"
import { readFileSync } from 'node:fs'
const [script, argsJson] = process.argv.slice(2)
const source = readFileSync(script, 'utf8').replace(/^export const meta = /m, 'const meta = ')
const AsyncFunction = Object.getPrototypeOf(async function () {}).constructor
let launched = 0
const launch = async () => { launched += 1; throw new Error('an agent was launched') }
const body = new AsyncFunction('agent', 'parallel', 'pipeline', 'log', 'phase', 'args', 'budget', 'workflow', source)
const args = argsJson === 'undefined' ? undefined : JSON.parse(argsJson)
const result = await body(launch, launch, launch, () => {}, () => {}, args, { total: null }, launch)
console.log(JSON.stringify({ result, launched }))
"#;

fn invoke(driver: &Path, args: &str) -> Value {
    let out = Command::new("node")
        .arg(driver)
        .arg(repo_root().join(HARNESS))
        .arg(args)
        .output()
        .expect("run node");
    assert!(
        out.status.success(),
        "the script does not parse, or threw, under args {args}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let seen: Value =
        serde_json::from_slice(&out.stdout).expect("the driver prints one JSON value");
    assert_eq!(
        seen["launched"], 0,
        "an agent was launched under args {args}"
    );
    seen["result"].clone()
}

#[test]
fn i_where_node_is_installed_the_script_parses_and_its_self_test_passes() {
    if !node_or_skip(&format!(
        "{HARNESS} was scanned and not run: its self-test and its refusals are unchecked"
    )) {
        return;
    }
    let scratch = ScratchDir::new("stabilize-harness");
    let driver = scratch.path().join("driver.mjs");
    fs::write(&driver, DRIVER).expect("write the driver");

    let seen = invoke(&driver, r#"{"selfTest": true}"#);
    assert_eq!(
        seen["status"], "self-test-passed",
        "the script's self-test: {seen}"
    );
    assert!(
        seen["checks"].as_u64().is_some_and(|n| n >= 280),
        "the self-test ran its checks: {seen}"
    );

    // Every malformed invocation is refused, and no agent is launched for it.
    let run = r#""run": "rc24-tier1", "scratch": "/tmp/scratch-1""#;
    let malformed = [
        "undefined".to_owned(),
        "{}".to_owned(),
        r#""test""#.to_owned(),
        format!(r#"{{"stage": "both", {run}}}"#),
        r#"{"stage": "test", "run": "RC 24", "scratch": "/tmp/s"}"#.to_owned(),
        r#"{"stage": "test", "run": "rc24", "scratch": "relative/dir"}"#.to_owned(),
        r#"{"stage": "test", "run": "rc24", "scratch": "/tmp/it's here"}"#.to_owned(),
        r#"{"stage": "test", "run": "rc24"}"#.to_owned(),
        format!(
            r#"{{"stage": "test", {run}, "rulings": [{{"key": "f-1", "ruling": "later"}}], "stopAfter": "state"}}"#
        ),
        format!(r#"{{"stage": "test", {run}, "rulings": [{{"key": "f-1", "ruling": "fixed"}}]}}"#),
        format!(r#"{{"stage": "fix", {run}, "rulings": [{{"key": "f-1", "ruling": "fixed"}}]}}"#),
        format!(r#"{{"stage": "fix", {run}, "exit": "continue"}}"#),
        format!(r#"{{"stage": "fix", {run}, "raise": {{"cycles": 3}}}}"#),
        format!(r#"{{"stage": "test", {run}, "stopAfer": "state"}}"#),
        format!(r#"{{"stage": "test", {run}, "clause": "no-lost-files", "scope": "everything"}}"#),
        format!(
            r#"{{"stage": "test", {run}, "clause": "no-lost-files", "crossModel": ["row-3"]}}"#
        ),
        format!(r#"{{"stage": "test", {run}, "rulings": [{{"go": false}}]}}"#),
        format!(
            r#"{{"stage": "test", {run}, "rulings": [{{"go": true}}], "scope": "everything"}}"#
        ),
        format!(
            r#"{{"stage": "fix", {run}, "rulings": [{{"go": true}}, {{"key": "f-1", "ruling": "later"}}]}}"#
        ),
        format!(r#"{{"stage": "fix", {run}, "rulings": [{{"rounds": 0}}]}}"#),
        format!(r#"{{"stage": "fix", {run}, "rulings": [{{"rerun": "No lost files"}}]}}"#),
    ];
    for args in &malformed {
        let seen = invoke(&driver, args);
        assert_eq!(seen["status"], "refused", "args {args}: {seen}");
    }
    // No value of `crossModel` turns the cross-model pass on for every item.
    for every in [
        "true",
        "false",
        r#""all""#,
        r#""*""#,
        r#""every""#,
        "1",
        "[]",
        "{}",
        r#"["*"]"#,
        r#"["all rows"]"#,
        r#"{"all": true}"#,
        "[true]",
    ] {
        let args = format!(r#"{{"stage": "test", {run}, "crossModel": {every}}}"#);
        let seen = invoke(&driver, &args);
        assert_eq!(seen["status"], "refused", "args {args}: {seen}");
    }
}
