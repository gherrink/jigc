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
//!   its definition binds on — and the binary line reaches EXACTLY the roles whose
//!   definition binds on it, each of which returns the hash it asserted (`M7`).
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
//! - **(o)** the two roles that have no definition carry the paragraph every definition
//!   carries — *Never push to or merge into `main`…* — byte for byte as the definitions
//!   have it (`L7`).
//! - **(p)** what an agent's ending decides has one road each (`M5`, `M9`, `M6`): an attempt
//!   of the `test` stage is begun on record before any agent of it is launched; a reporter
//!   leaves the launched list only on the word of the report check; and the preflight's
//!   definition takes the candidate the finishing lap asks it for. What a stage then DOES is
//!   held where it is run ([`stabilize_simulation`](super::stabilize_simulation)).
//!
//! - **(q)** a runtime probe works on no run (the second repair plan's task `K0`): an
//!   invocation that names one is refused beside anything of a stage, and reaches no code of
//!   a stage — the script's last statement sends it to `runProbe` and nowhere else; every
//!   step of a probe is a command of `dev/stabilize-probe` — each act that tool has is asked
//!   for, and no other — on the roles a stage launches; no code of a probe names a run, a
//!   round, a branch, the record script, the step tool or a gate; the harness and the tool
//!   have the same cases; and the one schema that requires the hash is the probe's.
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
const PROBE_TOOL: &str = "dev/stabilize-probe";
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
/// opens `GIT STEP — `. `probeStep` is no such function: it composes a step of a runtime
/// probe — the same prompt, with the probe tool where the step tool stands — and arm (q)
/// holds what it is handed.
fn git_prompts(full: &str) -> Vec<String> {
    functions(full)
        .into_iter()
        .filter(|name| name != "stepPrompt" && name != "selfTest" && name != "probeStep")
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
    // branch prefix of the branch model, and none is assembled from a round's suffix. (The
    // paragraph every definition carries names the branch types an agent may push, as
    // prose: arm (o) holds that line to the definitions' own bytes.)
    let elsewhere: String = without(&full, &["branchName", "selfTest"])
        .lines()
        .filter(|line| !line.starts_with("const NEVER = \""))
        .map(|line| format!("{line}\n"))
        .collect();
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

    // THE BINARY LINE REACHES EXACTLY THE ROLES WHOSE DEFINITION BINDS ON IT (the harness
    // review's `M7`: the reviewers' definition carried the binary paragraph and the role was
    // never sent the line, so a reviewer verified its findings on a build of the working
    // tree). Both sets are derived, and they are equal.
    let binary = &labels["binary"];
    let binding: BTreeSet<&str> = roles
        .iter()
        .filter(|role| {
            definitions
                .get(&role.agent_type)
                .is_some_and(|text| text.contains(&format!("`{binary}`")))
        })
        .map(|role| role.name.as_str())
        .collect();
    let handed: BTreeSet<&str> = roles
        .iter()
        .filter(|role| role.labels.iter().any(|key| key == "binary"))
        .map(|role| role.name.as_str())
        .collect();
    assert!(
        binding.len() >= 4,
        "the scan found the roles whose definition binds on `{binary}`: {binding:?}"
    );
    assert_eq!(
        binding, handed,
        "the roles whose definition binds on `{binary}` (left) are the roles the script sends it to (right)"
    );
    // And a role that is handed the binary returns the hash it asserted: `drives` says so
    // for exactly those roles — and for the one prompt-only role whose prompt spells the
    // hash check out.
    let table = &full[full.find("\nconst ROLES = {\n").expect("the roles table")..];
    let table = &table[..table.find("\n}\n").expect("the table closes")];
    for role in &roles {
        let line = table
            .lines()
            .find(|line| line.trim_start().starts_with(&format!("{}: ", role.name)))
            .expect("the role's row");
        let drives = line.contains("drives: true");
        let should = handed.contains(role.name.as_str()) || role.name == "proposal";
        assert_eq!(
            drives, should,
            "`{}` is handed the binary, and is held to the hash it asserts, or neither: {line}",
            role.name
        );
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
        source.matches("roleStep('record', ").count()
            - function(&full, "runProbe")
                .matches("roleStep('record', 'probe:payload:e' + n, 'Probe', probePayloadPrompt(")
                .count(),
        1,
        "the record's executor is launched in one place — and by the `payload` probe, whose \
         prompt applies nothing (arm (q))"
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

    // BOTH GATES OF A ROUND KEEP GOING — the candidate's, which the preflight runs, and a
    // record step's — so that what each names red is all that is red and the two are held
    // against each other test by test. One constant spells the command, and nothing else
    // in the harness spells a gate that is run.
    assert!(
        source.contains("\nconst GATE = 'dev/gate --keep-going'\n")
            && function(&full, "preflightPrompt")
                .contains("`' + GATE + ' > ' + plan.gate + ' 2>&1`")
            && function(&full, "gateStep").contains("`' + GATE + ' > ' + file + ' 2>&1`")
            && !source.contains("`dev/gate > "),
        "the candidate's gate and a record step's gate are not both `dev/gate --keep-going`"
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
// (o) the roles that have no definition carry the Never paragraph
// ---------------------------------------------------------------------------

#[test]
fn o_the_roles_with_no_definition_carry_the_never_paragraph_as_every_definition_has_it() {
    let full = harness();
    // The paragraph, as the definitions have it: one line, the same bytes in every one
    // (`release_pipeline_fence` holds that) — read from them, never spelled here.
    let opening = "**Never push to or merge into `main`";
    let paragraphs: BTreeSet<String> = definitions()
        .values()
        .map(|text| {
            text.lines()
                .find(|line| line.starts_with(opening))
                .expect("every definition carries the paragraph")
                .to_owned()
        })
        .collect();
    assert_eq!(
        paragraphs.len(),
        1,
        "the paragraph is one text across the definitions"
    );
    let paragraph = paragraphs.into_iter().next().expect("one");
    assert_eq!(
        full.lines()
            .find(|line| line.starts_with("const NEVER = "))
            .expect("the script holds the paragraph"),
        format!("const NEVER = \"{paragraph}\""),
        "the script's copy of the paragraph is the definitions', byte for byte"
    );
    // The roles with no definition are derived from the roles table; each one's prompt
    // function hands the paragraph over, as a line of its own.
    let prompt_only: Vec<String> = roles(&full)
        .into_iter()
        .filter(|role| role.agent_type == "general-purpose")
        .map(|role| role.name)
        .collect();
    assert_eq!(
        prompt_only,
        ["proposal", "crossModel"],
        "the roles whose prompt is their whole contract"
    );
    for role in &prompt_only {
        let prompt = function(&full, &format!("{role}Prompt"));
        assert!(
            prompt.contains("\n    NEVER,\n  ].join('\\n')"),
            "`{role}Prompt` does not end with the paragraph"
        );
    }
    assert_eq!(
        without(&full, &["selfTest"]).matches("NEVER").count(),
        3,
        "the paragraph is declared once and handed to those two prompts"
    );
}

// ---------------------------------------------------------------------------
// (p) what an agent's ending decides has one road each
// ---------------------------------------------------------------------------

#[test]
fn p_an_attempt_begins_on_record_and_a_reporter_leaves_the_list_only_on_the_checks_word() {
    let full = harness();
    let source = code(&full);
    let tested = function(&full, "runTest");

    // AN ATTEMPT OF THE `test` STAGE IS BEGUN ON RECORD BEFORE ANY AGENT OF IT IS LAUNCHED:
    // once where the stage finishes a triage, once where it runs — a re-run included — and
    // each before the first agent of that road.
    assert_eq!(
        tested.matches("await beginAttempt(").count(),
        2,
        "the finishing lap, and the stage itself"
    );
    let lap = tested
        .find("\n  if (at.triage) {\n")
        .expect("the finishing lap");
    let begun = tested[lap..]
        .find("await beginAttempt(ctx, lap, tested)")
        .expect("the lap begins its attempt");
    let finishes = tested[lap..]
        .find("await finishTriage(")
        .expect("the lap's triage");
    assert!(begun < finishes, "the lap is begun before its triage");
    let main = tested
        .find("await beginAttempt(ctx, launch, sha)")
        .expect("the stage begins its attempt");
    for launching in [
        "preflightOf(ctx",
        "roleStep(",
        "runUnits(",
        "triagePasses(ctx, launch, built, sources",
    ] {
        let first = tested[lap + finishes..]
            .find(launching)
            .unwrap_or_else(|| panic!("the stage launches `{launching}`"));
        assert!(
            main < lap + finishes + first,
            "`{launching}` is launched before the attempt is on record"
        );
    }
    let begin = function(&full, "beginAttempt");
    assert!(
        begin.contains("launch.add([ATTEMPT])")
            && begin.contains("toolStep('begin', 'State', 'begin', beginPrompt(ctx, sha))")
            && begin.contains("begun.status !== 'begun'"),
        "the marker is a launched reporter, written by the tool's `begin` act, and a step that did not begin it halts"
    );

    // A REPORTER LEAVES THE LAUNCHED LIST ONLY ON THE WORD OF THE REPORT CHECK: one function
    // asks, and takes off exactly what the check names as missing.
    let settle = function(&full, "settleReports");
    assert!(
        settle.contains("checkReportsPrompt(ctx, launch.names)")
            && settle.contains("for (const name of read.missing || []) launch.drop(name)"),
        "`settleReports` asks the check over every name handed out, and drops what it names"
    );
    assert_eq!(
        without(&full, &["selfTest", "launcher"])
            .matches(".drop(")
            .count(),
        1,
        "nothing else takes a reporter off the list"
    );
    assert_eq!(
        source.matches("checkReportsPrompt(").count()
            - function(&full, "selfTest")
                .matches("checkReportsPrompt(")
                .count(),
        2,
        "the report check is asked for in one place"
    );
    // After the instruments, and after every triage pass.
    assert!(
        tested.contains("await settleReports(ctx, launch, 'check-reports', 'Instruments')")
            && function(&full, "triagePasses")
                .contains("await settleReports(ctx, launch, 'check-reports:p' + pass, 'Triage')"),
        "the reports are checked after the instruments and after every triage pass"
    );

    // THE CANDIDATE A FINISHING LAP ASKS THE PREFLIGHT FOR IS THE ONE THE ROUND TESTED, and
    // the preflight's definition takes it (`M6`).
    assert!(
        tested.contains(
            "await finishTriage(ctx, lap, state, { sha: tested, label, tested: true }, loopBranch)"
        ) && function(&full, "preflightPrompt").contains("THE CANDIDATE IS NOT `HEAD` HERE"),
        "the finishing lap asks for the commit the round tested, and its prompt says that it is not HEAD"
    );
    let preflight = &definitions()["stabilize-preflight"];
    assert!(
        preflight.contains("`git merge-base --is-ancestor <sha> HEAD` exits 0")
            && preflight.contains("no step that reads the working tree may be listed beside it")
            && !preflight.contains("the tree is not the candidate"),
        "stabilize-preflight.md still asserts that every candidate is HEAD"
    );
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

/// The acts of the tool that NO STAGE ASKS FOR, each with its reason — the difference
/// between the acts the script composes and the acts the tool has (the orchestrator's
/// ruling of 2026-10-07 on the second repair plan's `K2`; the convention is the rig's
/// `RIG_ONLY_STATES`).
const NO_STAGES: &[(&str, &str)] = &[
    (
        "discard",
        "taking an applied batch back is the orchestrator's decision and a subagent's act, never a stage's",
    ),
    (
        "table",
        "the tool's table of acts and arrival states, printed for a reader and for the tool's own suite",
    ),
    // A HELD COMMAND (the second repair plan's `K10`): built in the tool, and asked for by
    // no stage UNTIL `K11` — which makes the gates, the regression set and the two builds
    // tool steps, and takes the first three of these rows out again (a listed act the
    // script asks for is red, below).
    (
        "hold-start",
        "no stage asks for it YET: the harness starts a long command by a tool step once the second repair plan's K11 has landed, and that task removes this row",
    ),
    (
        "hold-wait",
        "no stage asks for it YET: the harness waits for a held command by a tool step once the second repair plan's K11 has landed, and that task removes this row",
    ),
    (
        "hash",
        "no stage asks for it YET: a file's hash is asked of the tool, not of `shasum`, once the second repair plan's K11 has landed, and that task removes this row",
    ),
    (
        "build",
        "the command of the held kind `build`, which `hold-start --kind build` runs: it takes as long as cargo does, and no prompt of any stage names it",
    ),
];

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
    // The acts the script asks for are acts of the tool — and the tool's other acts are
    // [`NO_STAGES`], each with its reason. Held in both directions: an act of the tool that
    // is neither asked for nor listed is red, and so is a listed act the script asks for.
    let asked_for: BTreeSet<String> = asked.keys().cloned().collect();
    let listed: BTreeSet<String> = NO_STAGES
        .iter()
        .map(|(act, why)| {
            assert!(
                why.split_whitespace().count() >= 5,
                "`{act}` carries its reason: {why:?}"
            );
            (*act).to_owned()
        })
        .collect();
    assert_eq!(listed.len(), NO_STAGES.len(), "no act is listed twice");
    assert_eq!(
        asked_for.intersection(&listed).collect::<Vec<_>>(),
        Vec::<&String>::new(),
        "an act the script asks for is listed as one no stage asks for: the list is stale"
    );
    assert_eq!(
        asked_for.union(&listed).cloned().collect::<BTreeSet<_>>(),
        acts,
        "the acts the script asks for and the ones no stage asks for (left) are the acts {STEP_TOOL} has (right)"
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
            } else if line.contains(
                "const raw = await gitStep('probe:' + label, 'Probe', probeStep(what, act, flags), STEP_SCHEMA)",
            ) {
                "a probe's step"
            } else {
                "another"
            }
        })
        .collect();
    assert_eq!(
        direct,
        ["toolStep", "a probe's step", "the part's re-cut"],
        "who calls `gitStep`"
    );
}

// ---------------------------------------------------------------------------
// (q) a runtime probe works on no run
// ---------------------------------------------------------------------------

/// The acts `dev/stabilize-probe` has, read from its own parser.
fn probe_acts() -> BTreeSet<String> {
    let tool = fs::read_to_string(repo_root().join(PROBE_TOOL)).expect("read the probe tool");
    tool.lines()
        .filter_map(|line| line.strip_prefix("    act(\""))
        .map(|rest| rest[..rest.find('"').expect("the act's name closes")].to_owned())
        .collect()
}

/// The numbers of the one-line list or tuple a source declares as `declaration`.
fn numbers_in(source: &str, declaration: &str) -> Vec<u64> {
    let line = source
        .lines()
        .find(|line| line.starts_with(declaration))
        .unwrap_or_else(|| panic!("`{declaration}` is declared"));
    line[declaration.len()..]
        .split(|c: char| !c.is_ascii_digit())
        .filter(|digits| !digits.is_empty())
        .map(|digits| digits.parse().expect("a number"))
        .collect()
}

#[test]
fn q_a_runtime_probe_works_on_no_run_and_its_steps_are_the_probe_tools() {
    let full = harness();
    let source = code(&full);
    assert!(
        source.contains(&format!("\nconst PROBE_TOOL = '{PROBE_TOOL}'\n")),
        "the script names the probe tool once"
    );
    assert_eq!(
        source.matches(PROBE_TOOL).count()
            - function(&full, "selfTest").matches(PROBE_TOOL).count(),
        1,
        "and spells it nowhere else: every command of it is composed from that name"
    );

    // An invocation that names a probe is refused beside anything of a stage, before the
    // stage's own arguments are read; and it reaches no stage: the script's last statement
    // sends it to `runProbe`.
    let args = function(&full, "validateArgs");
    let probed = args
        .find("\n  if (a.probe != null) return probeFault(a)\n")
        .expect("a probe's arguments are judged by `probeFault`");
    assert!(
        args.find("\n  if (a.selfTest) return null\n")
            .expect("the self-test")
            < probed
            && probed < args.find("STAGES.includes(a.stage)").expect("the stage"),
        "a probe is told from a stage before anything of a stage is read"
    );
    let fault = function(&full, "probeFault");
    assert!(
        fault.contains("!['probe', 'scratch', 'seconds'].includes(name)")
            && fault.contains("if (scratchFault(a.scratch)) return scratchFault(a.scratch)")
            && fault.contains("a.probe !== 'hold'"),
        "a probe takes its scratch root — and `seconds`, for `hold` — and nothing else: {fault}"
    );
    assert!(
        source.trim_end().ends_with(
            "\nreturn await (v.probe ? runProbe() : v.stage === 'test' ? runTest() : runFix())"
        ),
        "the script's last statement sends a probe to `runProbe`, and nothing else there"
    );
    assert_eq!(
        source.matches("runProbe(").count(),
        2,
        "`runProbe` is declared, and called by that statement alone"
    );

    // EVERY STEP OF A PROBE IS A COMMAND OF THE PROBE TOOL. The acts it asks for — through
    // its one step function, or spelled into a role's prompt — are the acts the tool has.
    let acts = probe_acts();
    assert!(
        acts.len() >= 6,
        "the scan found the probe tool's acts: {acts:?}"
    );
    let probe_functions: Vec<String> = functions(&full)
        .into_iter()
        .filter(|name| {
            name.starts_with("probe")
                || name == "runProbe"
                || name == "watched"
                || name == "faultOf"
        })
        .collect();
    assert!(
        probe_functions.len() >= 10,
        "the scan found the probe's functions: {probe_functions:?}"
    );
    let mut asked = BTreeSet::new();
    for name in &probe_functions {
        let body = function(&full, name);
        // … by a git step: `probeAct('<label>', '<act>', …`.
        for rest in body.split("probeAct('").skip(1) {
            let act = rest.split('\'').nth(2).expect("a step names its act");
            asked.insert(act.to_owned());
        }
        // … spelled into a role's prompt as a command: `PROBE_TOOL + ' <act> `, in backticks.
        if name != "probeStep" {
            for rest in body.split("`' + PROBE_TOOL + ' ").skip(1) {
                asked.insert(rest[..rest.find(' ').expect("an act and its flags")].to_owned());
            }
        }
    }
    assert_eq!(
        asked, acts,
        "the acts a probe asks for (left) are the acts {PROBE_TOOL} has (right)"
    );
    let step = function(&full, "probeStep");
    assert!(
        step.contains(
            "return stepPrompt('a PROBE of the stabilization harness, of no run — ' + what, act, flags).replace('1. `' + STEP_TOOL + ' ', '1. `' + PROBE_TOOL + ' ')"
        ),
        "a probe's step is the harness's own step prompt, with the probe tool where the step \
         tool stands: {step}"
    );
    assert!(
        function(&full, "probeAct").contains("return { raw, read: readStep(raw, act) }"),
        "and its line is read as every step's line is, as the act that was asked for"
    );

    // NO CODE OF A PROBE NAMES A RUN, A ROUND OR A BRANCH, and none reaches what a stage
    // does with side effects: the record script, the step tool's acts, a gate, a report.
    for name in &probe_functions {
        let body = code(function(&full, name));
        for foreign in [
            "branchName(",
            "loopBranch",
            "branchLine(",
            "LABELS.branch",
            "LABELS.report",
            "runDir(",
            "v.run",
            "v.stage",
            "toolStep(",
            "readState(",
            "beginAttempt(",
            "settleReports(",
            "recordStep(",
            "recordPrompt(",
            "rulingsStep(",
            "launcher(",
            "reportLine(",
            "' + GATE",
            "GATE + '",
            "stabilize-record apply",
            "git ",
        ] {
            assert!(
                !body.contains(foreign),
                "`{name}` is a probe's, and has `{foreign}`"
            );
        }
        if name != "probeStep" {
            assert!(
                !body.contains("STEP_TOOL") && !body.contains("stepPrompt("),
                "`{name}` composes a command of the step tool"
            );
        }
    }
    // The roles a probe launches are a stage's, by the roles table: the reviewer, the
    // executor and the preflight — and each is handed a probe's own prompt.
    let run = function(&full, "runProbe");
    let launched: Vec<&str> = run
        .split("roleStep('")
        .skip(1)
        .map(|rest| &rest[..rest.find('\'').expect("a role")])
        .collect();
    assert_eq!(
        launched,
        ["review", "record", "preflight"],
        "the roles a probe launches"
    );
    for (role, prompt) in [
        ("review", "probeReviewPrompt("),
        ("record", "probePayloadPrompt("),
        ("preflight", "probeHoldPrompt("),
    ] {
        let call = run
            .lines()
            .find(|line| line.contains(&format!("roleStep('{role}', 'probe:")))
            .expect("the role's call");
        assert!(
            call.contains(prompt) && call.contains("await watched(() => "),
            "the `{role}` of a probe is handed a probe's prompt, and its tries are watched: {call}"
        );
    }
    // A PROBE'S AGENTS ARE MEANT TO FAIL: what a watched call exhausted is the probe's
    // answer, and the breaker is put back so that the judgement still runs.
    let watched = function(&full, "watched");
    assert!(
        watched.contains("\n  triesSeen = []\n")
            && watched.contains("\n  triesSeen = null\n")
            && watched.contains("\n  exhaustedLabels = []\n  breakerTripped = false\n"),
        "`watched` counts the tries of one call, and puts the breaker back: {watched}"
    );
    assert_eq!(
        source.matches("triesSeen = ").count(),
        3,
        "the tries are watched in `watched`, and nowhere in a stage"
    );
    assert_eq!(
        function(&full, "agentR")
            .matches("if (triesSeen) triesSeen.push(")
            .count(),
        2,
        "a try is counted where it returns or comes back with nothing, and where it throws"
    );

    // THE ONE SCHEMA THAT REQUIRES THE HASH IS THE PROBE'S: no stage's return gained a
    // required field by this — what a `required` buys is what the probe is there to see.
    assert!(
        source.contains(
            "\nconst PROBE_UNIT_SCHEMA = Object.assign({}, UNIT_SCHEMA, { required: UNIT_SCHEMA.required.concat(['asserted_sha256']) })\n"
        ),
        "the probe's schema is a unit's, with the hash required"
    );
    assert_eq!(
        source.matches("PROBE_UNIT_SCHEMA").count()
            - function(&full, "selfTest")
                .matches("PROBE_UNIT_SCHEMA")
                .count(),
        2,
        "and it is used by the `required` probe's reviewers alone"
    );
    let unit = &source[source
        .find("\nconst UNIT_SCHEMA = {\n")
        .expect("a unit's schema")..];
    assert!(
        unit[..unit.find("\n}\n").expect("it closes")]
            .contains("\n  required: ['status', 'findings'],\n"),
        "a stage's unit still requires its status and its findings, and no hash"
    );

    // THE HARNESS AND THE TOOL HAVE THE SAME CASES.
    let tool = fs::read_to_string(repo_root().join(PROBE_TOOL)).expect("read the probe tool");
    assert_eq!(
        quoted_in(&full, "const PROBES = "),
        tool.lines()
            .find(|line| line.starts_with("PROBES = ("))
            .expect("the tool's probes")
            .split('"')
            .skip(1)
            .step_by(2)
            .map(str::to_owned)
            .collect::<Vec<_>>(),
        "the probes"
    );
    let sizes = numbers_in(&tool, "SIZES = ");
    assert_eq!(
        numbers_in(&full, "const PROBE_ROWS = "),
        sizes.iter().step_by(2).copied().collect::<Vec<_>>(),
        "the ledger rows of the relay probe's runs"
    );
    assert_eq!(
        numbers_in(&full, "const PROBE_RELAYS = "),
        numbers_in(&tool, "RELAYS = "),
        "how often each run's line is relayed"
    );
    assert_eq!(
        numbers_in(&full, "const PROBE_ENTRIES = "),
        numbers_in(&tool, "ENTRIES = "),
        "the entries of the payload probe's batches"
    );
    let slice = numbers_in(&full, "const PROBE_SLICE = ")[0];
    assert!(
        slice < 120 && slice < numbers_in(&tool, "SLICE_BELOW = ")[0],
        "a read of a hold waits a slice the tool takes, under a shell tool's default timeout: {slice}"
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
    // An invocation that names a probe is refused beside anything of a stage, with no
    // scratch root, and under a name no probe has — and no agent is launched for it.
    for args in [
        r#"{"probe": "everything", "scratch": "/tmp/scratch-1"}"#.to_owned(),
        r#"{"probe": "relay"}"#.to_owned(),
        r#"{"probe": "relay", "scratch": "relative/dir"}"#.to_owned(),
        r#"{"probe": "relay", "scratch": "/tmp/scratch-1", "stage": "test"}"#.to_owned(),
        format!(r#"{{"probe": "payload", "stage": "test", {run}}}"#),
        r#"{"probe": "hold", "scratch": "/tmp/scratch-1", "run": "rc24-tier1"}"#.to_owned(),
        r#"{"probe": "hold", "scratch": "/tmp/scratch-1", "seconds": 0}"#.to_owned(),
        r#"{"probe": "relay", "scratch": "/tmp/scratch-1", "seconds": 60}"#.to_owned(),
        format!(r#"{{"stage": "test", {run}, "seconds": 60}}"#),
    ] {
        let seen = invoke(&driver, &args);
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
