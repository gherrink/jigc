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
//! - **(r)** what a stage reads is a digest, and no text of the record passes through the
//!   script (the second repair plan's task `K3`): every step's composer asks for the digest
//!   and the one function that reads a stage's step refuses a line that is none; every field
//!   of the state the stages' code reads is one the tool's digest DECLARES, and what the
//!   digest gives as a count is read as one; no prompt and no return is composed from a
//!   brief, a door's derivation, a ledger row or a pending batch's subject — an agent is
//!   handed the read, the orchestrator the file; the script has a row for every word the
//!   step tool refuses with, and for no other; and a held check's kind is a kind the tool
//!   holds. The functions that are the `fix` half's own are named, each with its reason.
//!
//! - **(s)** every command a prompt names is one plain invocation of a tool (the second
//!   repair plan's task `K11`; the human's ruling of 2026-10-07 that no step of a stage may
//!   need his permission): every prompt the script can compose is composed — a table of
//!   calls of its composers, held whole against the script's own functions, driven under
//!   `node` — and held ([`prompt_faults`]): a code span that opens with `dev/` is
//!   `dev/stabilize-step`, `dev/stabilize-record` or `dev/stabilize-probe` and plain words;
//!   no span is a command of another program; nothing a shell composes with stands
//!   anywhere; and no prompt tells an agent to wait, to send a command to the background,
//!   to redirect, to pipe or to poll by its own means. What is not replaceable is a named
//!   exception with its reason, red where no prompt meets it or its line stops holding
//!   its shape; and each shape, put back into each prompt, is red. The simulation holds
//!   every prompt of every invocation it runs to the same function.
//!
//! - **(t)** what a stage needs of a return has its cell, and what still halts once an
//!   instrument is launched is a stated list (the second repair plan's task `X1`): from the
//!   stage's first held check on, and in a finishing lap from its triage on, every halt
//!   goes through `haltAfter` and names a row of the script's `HALTS_AFTER`, every row is
//!   met, no helper of that stretch halts by itself, and the workflow doc states the same
//!   rows; and — under `node` — every field a driving role's schema describes and does not
//!   require is a row of the script's `NEEDS` or of this suite's [`NOT_NEEDED`], no needed
//!   field is one a schema requires, every cell that voids is an arm of the simulation, and
//!   every cell that still halts is owed to a task by name ([`OWED`]).
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
        function(&full, "toolStep").contains(
            "readDigest(await gitStep(label, phaseTitle, prompt, STEP_SCHEMA, RUN_AGAIN, seen), act)"
        ),
        "a step that is one command of the tool goes through `gitStep`, and so through that call — read as a digest, and asked for again as its one command"
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
            && tested.contains("const gate = rerun ? null : await hold({ name: 'gate-' + label + '-a' + ctx.attempt, kind: 'gate',")
            && tested.contains("gate: pre.gate ? { commit: sha, file: pre.gate.output } : null")
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
        rule.contains("const why = about ? runRulingsFault(v.rulings, state) : null")
            && !source.contains("findingRulingsFault")
            && !source.contains("state.ledger || []")
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
        ledger.contains("state.untriaged.count")
            && ledger.contains("read: typed(RECORD_TOOL, 'untriaged --run ' + run)")
            && !ledger.contains("state.ledger")
            && !ledger.contains(".round"),
        "the rows are the state's `untriaged`, whatever round or stage they came from — as how many there are, and the read that prints them"
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
    // THE PREVIOUS RELEASE'S BINARY IS BUILT BY THE TOOL, from the commit and to the version
    // the record names (the second repair plan's `K11`): it is handed to the function that
    // holds the two builds, never to a preflight, and the tool's verdict is held to it.
    let builds = function(&full, "buildBoth");
    assert!(
        builds.contains(
            "String(previous.commit), 'bin/previous-' + short + '/jigc', previous.version,"
        ) && function(&full, "buildOf").contains("(version && f.version !== version)")
            && function(&full, "buildOf").contains("' --version ' + version"),
        "the previous release's binary is built from the record's commit, and held to the record's version"
    );
    let scope = function(&full, "scopePrompt");
    assert!(
        scope.contains("plan.previous.commit") && scope.contains("plan.fallback"),
        "the scope step is told the previous release's commit and the default scope"
    );
    let mut built = 0;
    for line in without(&full, &["selfTest", "runFix"])
        .lines()
        .filter(|line| line.contains("await buildBoth("))
    {
        built += 1;
        assert!(
            line.contains(", previousOf(state), "),
            "the builds are not handed the run's previous release: {line}"
        );
    }
    assert_eq!(
        built, 2,
        "a `test` stage and the lap that finishes a triage each build both binaries"
    );
    assert!(
        !function(&full, "preflightPrompt").contains("plan.previous")
            && !function(&full, "preflightPrompt").contains("plan.binary")
            && !function(&full, "preflightOf").contains("r.candidate")
            && !function(&full, "preflightOf").contains("r.previous"),
        "and no preflight builds or returns a binary"
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
        "items: (state.items || []).filter((i) => i.clause === c.clause)",
        "standing: at(i.standing)",
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

    // One function runs a record step: the executor first — where a batch is to be applied
    // — then THE GATE, A COMMAND THE TOOL HOLDS, under a name of the record's own that is
    // never an earlier invocation's; then the ONE commit, a git step that is handed the
    // file the tool kept the gate's output in and what the batch was composed of.
    let step = function(&full, "recordStep");
    let executor = step
        .find("await roleStep('record', ")
        .expect("a record step launches its executor");
    let gate = step
        .find("const gate = await hold({ name: rec.hold, sequence: true, kind: 'gate', flags: '--run ' + v.run,")
        .expect("a record step's gate is held by the tool, under the record's own name, in sequence");
    let commit = step
        .find("await toolStep('record:' + label, 'Record', 'record', recordCommitPrompt(v, branch, gate.output, rec.expect, stageHead))")
        .expect("a record step's commit is the tool's `record` act, handed the gate's file, what was composed and the commit the stage began on");
    assert!(
        executor < gate && gate < commit && step.contains("recordFault(r, rec.expect)"),
        "the executor, then the gate, then the commit, then the line held to what was composed"
    );
    assert!(
        step.contains("\n  if (rec.text) {\n")
            && step.contains("if (!HELD_TAKES.record.includes(gate.ends)) return { fault: ")
            && step.contains("then: recordThen(v.run, branch, 'no-gate')"),
        "no executor runs for a batch that is applied; and a gate that left no verdict commits nothing and leaves the batch applied"
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
        "applied.checks !== expect.checks",
        "applied.failed !== 0",
    ] {
        assert!(fault.contains(held), "`recordFault` holds `{held}`");
    }
    assert!(
        function(&full, "recordPrompt")
            .contains("expect: { calls: calls.length, checks: calls.filter(isCheck).length }"),
        "what a record is held to is counted off the calls the script composed"
    );

    // BOTH GATES OF A ROUND KEEP GOING — the candidate's and a record step's — so that what
    // each names red is all that is red and the two are held against each other test by
    // test. THE COMMAND IS THE STEP TOOL'S, the one of its held kind `gate`, and no code of
    // the harness spells a gate that is run: both are started by `hold`, as that kind.
    let tool = fs::read_to_string(repo_root().join(STEP_TOOL)).expect("read the step tool");
    assert!(
        tool.contains("{\"kind\": \"gate\", \"takes\": [\"run\"], \"may\": [], \"command\": \"dev/gate --keep-going\"")
            && !without(&full, &["selfTest", "closeOf"]).contains("dev/gate")
            && function(&full, "runTest").contains("kind: 'gate', flags: '--run ' + v.run,"),
        "the candidate's gate and a record step's gate are both the tool's held `gate`, which is `dev/gate --keep-going`"
    );

    // The executor's prompt spells no commit and no gate: it writes the batch to a file
    // and applies it from there, held to its hash — and nothing else.
    let prompt = function(&full, "recordPrompt");
    assert!(
        !prompt.contains("git commit")
            && !prompt.contains("git add")
            && !prompt.contains("ONE commit of the paths")
            && !prompt.contains("'3. "),
        "`recordPrompt` has the executor commit, or run a third step"
    );
    assert!(
        source.contains(
            "\nconst RECORD_RETURNS = 'YOU MAKE NO COMMIT, stage nothing, and RUN NO GATE"
        ) && prompt.contains("    RECORD_RETURNS,\n"),
        "a record step's prompt ends by saying so"
    );
    assert!(
        prompt.contains("'2. ' + typed(RECORD_TOOL, 'apply --run ' + v.run")
            && prompt.contains("' --from ' + batch.file + ' --sha256 ' + batch.sha256)")
            && function(&full, "pendingRecord")
                .contains("return { text: null, hold: recordHold(dir),"),
        "a record's calls are ONE batch, read from a file and held to its hash; and a batch that is applied already has no executor"
    );
    let executor = &definitions()["build-executor"];
    assert!(
        executor.contains("no commit of yours") && !executor.contains("its `GATE: PASS`"),
        "build-executor.md still has the record step's executor commit"
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
        finish.contains("await recordStep('pending', gs.branch, pendingRecord("),
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
    // A HELD COMMAND (the second repair plan's `K10` and `K11`): `hold-start` and
    // `hold-wait` are steps of a stage. The two acts beside them are asked for by NO STEP:
    (
        "hash",
        "no STEP is a hash: it is the one command of the tool an AGENT is told to run and read itself — the independent drive of a proposal, which has no definition, asks the tool for the candidate's hash where a prompt once spelled `shasum`",
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
        composer.contains("'1. ' + typed(STEP_TOOL, act + ' ' + flags),")
            && composer.contains("'GIT STEP — ' + what + '. ' + (asks ? ASK_RULES : STEP_RULES),")
            && composer.matches("\n    '").count() == 3,
        "`stepPrompt` is the step's opening, ONE command — rendered by the one function that renders a command — and its report: {composer}"
    );
    // ONE STEP RUNS ITS COMMAND MORE THAN ONCE — the step that asks after a held command —
    // and it is told so by rules of its own: every other step never runs its command twice.
    let asking: Vec<String> = git_prompts(&full)
        .into_iter()
        .filter(|name| function(&full, name).trim_end().ends_with(", true)\n}"))
        .collect();
    assert_eq!(
        asking,
        ["holdWaitPrompt"],
        "the steps that are told to ask again"
    );
    assert!(
        code(&full).contains("\nconst ASK_RULES = 'Run exactly the ONE command below")
            && code(&full).matches("RUN THE SAME COMMAND AGAIN").count()
                - function(&full, "selfTest")
                    .matches("RUN THE SAME COMMAND AGAIN")
                    .count()
                == 1
            && code(&full).contains("never run the command a second time"),
        "the rules of the step that asks again, and of every step that does not"
    );
    assert!(
        function(&full, "proposalPrompt").contains("typed(STEP_TOOL, 'hash --scratch ' + ctx.scratch + ' --file ' + below(ctx.scratch, built.candidate.binary))"),
        "the hash a prompt asks of the tool is the tool's `hash`, of the candidate's binary"
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
                .contains("readDigest(await gitStep(label, phaseTitle, prompt, STEP_SCHEMA, RUN_AGAIN, seen), act)")
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
        // … spelled into a role's prompt as a command: `typed(PROBE_TOOL, '<act> …`.
        if name != "probeStep" {
            for rest in body.split("typed(PROBE_TOOL, '").skip(1) {
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
            "readState(",
            "beginAttempt(",
            "settleReports(",
            "recordStep(",
            "recordPrompt(",
            "rulingsStep(",
            "launcher(",
            "reportLine(",
            "dev/gate",
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
        // ONE PROBE HOLDS A COMMAND AS A STAGE DOES — the `hold` probe's case (a), through the
        // step tool's held kind `probe`: the two prompts a stage's held command gets, and
        // nothing else of a stage's steps.
        let steps = body.matches("toolStep(").count();
        assert_eq!(
            steps,
            if name == "runProbe" { 2 } else { 0 },
            "`{name}` runs a step of the step tool"
        );
    }
    let run = function(&full, "runProbe");
    assert!(
        run.contains("heldOf(await toolStep('hold-start:a', 'Probe', 'hold-start', holdStartPrompt(v, 'a', 'probe', '--seconds ' + seconds,")
            && run.contains("await watched(() => toolStep('hold-wait:a', 'Probe', 'hold-wait', holdWaitPrompt(v, 'a',")
            && run.contains("HELD_TAKES.probe.includes(held.ends) && held.kind === 'probe' && held.name === 'a' ? 'held'"),
        "the hold probe's case (a) is started and asked after by the two steps a stage holds a command with, and its line is the tool's verdict"
    );
    // The roles a probe launches are a stage's, by the roles table: the reviewer and the
    // executor — and each is handed a probe's own prompt.
    let launched: Vec<&str> = run
        .split("roleStep('")
        .skip(1)
        .map(|rest| &rest[..rest.find('\'').expect("a role")])
        .collect();
    assert_eq!(launched, ["review", "record"], "the roles a probe launches");
    for (role, prompt) in [
        ("review", "probeReviewPrompt("),
        ("record", "probePayloadPrompt("),
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

// ---------------------------------------------------------------------------
// (r) a stage reads digests, and no text of the record passes through the script
// ---------------------------------------------------------------------------

/// The functions that are the `fix` half's own and STILL READ THE STATE DOCUMENT — rows of
/// the ledger, lists of doors — which no step hands them any more: unreachable behind the
/// stage's refusal (arm (m)), and converted by that half's repair (the second repair plan,
/// section 9, row 7). Held in both directions: a function listed here that reads nothing of
/// the document is red, and so is any other function that does.
const FIX_HALFS_OWN: &[(&str, &str)] = &[
    (
        "runFix",
        "below its hand-over to `ruleTheRun` it reads the ledger's rows, the blockers and the doors as lists; its repair reads them by key",
    ),
    (
        "fixerPrompt",
        "a fixer is handed its findings' door, clause and repro: a blocker's row has no read yet",
    ),
    (
        "areasOf",
        "the areas of a fix cycle are cut from the doors of the round's scope, as a list",
    ),
    (
        "reopenPatches",
        "a dropped round's fixes are found in the ledger's rows",
    ),
    (
        "repointPatches",
        "a part's kept fixes are found in the ledger's rows",
    ),
];

/// The names a block of the step tool's source declares: the keys of `NAME = {`, one per
/// line at the block's first indent.
fn tool_keys(tool: &str, opens: &str) -> BTreeSet<String> {
    let block = &tool[tool
        .find(opens)
        .unwrap_or_else(|| panic!("the step tool declares `{opens}`"))
        + opens.len()..];
    let block = &block[..block.find("\n}\n").expect("the block closes")];
    block
        .lines()
        .filter_map(|line| line.strip_prefix("    \""))
        .map(|rest| rest[..rest.find('"').expect("the key closes")].to_owned())
        .collect()
}

#[test]
fn r_a_stage_reads_digests_and_no_prompt_and_no_return_holds_text_of_the_record() {
    let full = harness();
    let source = code(&full);
    let tool = fs::read_to_string(repo_root().join(STEP_TOOL)).expect("read the step tool");
    let fix_halfs: Vec<&str> = FIX_HALFS_OWN.iter().map(|(name, _)| *name).collect();
    let mut cut = vec!["selfTest"];
    cut.extend(&fix_halfs);
    let ours = without(&full, &cut);

    // (1) EVERY STEP ASKS FOR ITS DIGEST: each composer of a step of the tool hands it the
    // flag, once, and one function spells the flag.
    let composers = git_prompts(&full);
    assert_eq!(
        composers.len(),
        14,
        "the composers of a step — the twelve of the acts, and the two of a held command: {composers:?}"
    );
    for name in &composers {
        assert_eq!(
            function(&full, name).matches("digestFlag(").count(),
            1,
            "`{name}` asks for its step's digest, once"
        );
    }
    assert!(
        function(&full, "digestFlag").contains("return '--digest ' + scratch")
            && source.matches("'--digest ").count() == 1,
        "one function spells the flag, with the invocation's scratch root"
    );
    assert!(
        !function(&full, "probeStep").contains("digestFlag("),
        "a probe's step is the probe tool's, which has no digest"
    );

    // (2) … AND IS READ AS ONE, OR NOT AT ALL: a stage's step goes through `readDigest`,
    // which holds the line to `readStep`, then to being printable ASCII with no backslash,
    // to naming what did not fit and the file with the rest, and to nothing struck out.
    let digest = function(&full, "readDigest");
    for held in [
        "const said = readStep(r, act)",
        "!DIGEST_RE.test(line) || line.includes('\\\\')",
        "!Array.isArray(said.unfit) || !('file' in said) || !('file_sha256' in said)",
        "if (said.unfit.length) return ",
    ] {
        assert!(digest.contains(held), "`readDigest` holds `{held}`");
    }
    assert!(
        source.contains("\nconst DIGEST_RE = /^[\\x20-\\x7e]*$/\n"),
        "a digest is printable ASCII"
    );
    assert_eq!(
        ours.matches("readStep(").count(),
        4,
        "`readStep` is declared once and called by `readDigest`, by a probe's step and by the probe that relays a line of its own — a stage's step never goes around `readDigest`"
    );
    assert!(
        !function(&full, "lostStep").contains("? r.line :")
            && function(&full, "lostStep").contains("it is not passed on"),
        "a line that is not read is not passed on either"
    );

    // (3) EVERY FIELD OF THE STATE THE STAGES' CODE READS IS ONE THE DIGEST DECLARES — the
    // tool's own table of shapes, read here — or the one thing the script adds to it: where
    // the document lies.
    let declared = tool_keys(&tool, "\nSTATE = {\n");
    assert!(
        declared.len() >= 20
            && declared.contains("never_selected")
            && declared.contains("reverify"),
        "the scan found the state's digest: {declared:?}"
    );
    let mut read: BTreeSet<String> = BTreeSet::new();
    for (at, _) in ours.match_indices("state.") {
        let before = ours[..at].chars().next_back();
        if before.is_some_and(|c| c.is_ascii_alphanumeric() || c == '_') {
            continue;
        }
        let name: String = ours[at + "state.".len()..]
            .chars()
            .take_while(|c| c.is_ascii_lowercase() || *c == '_')
            .collect();
        // `state.` at the end of a sentence of a message is no read.
        if !name.is_empty() {
            read.insert(name);
        }
    }
    assert!(
        read.len() >= 15,
        "the scan found the reads of the state: {read:?}"
    );
    let strangers: Vec<&String> = read
        .iter()
        .filter(|name| !declared.contains(*name) && *name != "document")
        .collect();
    assert_eq!(
        strangers,
        Vec::<&String>::new(),
        "the script reads a field of the state that the tool's digest does not declare"
    );
    // WHAT THE DIGEST GIVES AS A COUNT IS READ AS A COUNT: no list of rows is walked.
    for field in [
        "ledger",
        "blockers",
        "human_list",
        "doors.included",
        "doors.excluded",
    ] {
        for walk in [
            ".length",
            ".map(",
            ".filter(",
            ".find(",
            ".some(",
            ".includes(",
            ".concat(",
            ".join(",
        ] {
            assert!(
                !ours.contains(&format!("state.{field}{walk}")),
                "`state.{field}` is a count in the digest, and `{walk}` walks it as a list"
            );
        }
    }
    for gone in [
        ".facts.candidate",
        ".facts.cycles",
        "rerun.doors",
        "state.untriaged || []",
        "c.items",
    ] {
        assert!(
            !ours.contains(gone),
            "`{gone}` is a path of the state document, and of no digest"
        );
    }
    // … and the `fix` half's own functions are exactly the ones that still do.
    let walks = |body: &str| {
        [
            "state.ledger.",
            "s.ledger.",
            "ledger.filter(",
            "const row of ledger",
            "state.blockers.",
            "doors.included.concat(",
            "f.detail",
            ".facts.cycles",
        ]
        .iter()
        .any(|walk| body.contains(walk))
    };
    for (name, why) in FIX_HALFS_OWN {
        assert!(
            why.len() > 20 && walks(function(&full, name)),
            "`{name}` is listed as the `fix` half's own and reads nothing of the document"
        );
    }
    for name in functions(&full) {
        if name != "selfTest" && !fix_halfs.contains(&name.as_str()) {
            assert!(
                !walks(function(&full, &name)),
                "`{name}` walks a list of the state document, and is not listed as the `fix` half's own"
            );
        }
    }

    // (4) NO PROMPT IS COMPOSED FROM TEXT OF THE RECORD: a brief, a door's derivation, a
    // pending batch's subject and a ledger row's cells are named by no code of the stages
    // — an agent is handed the read that prints them.
    assert_eq!(
        ours.matches(".brief").count(),
        2,
        "a brief is printed by `briefLine` alone, and only where it is the script's own sentence"
    );
    assert!(
        function(&full, "briefLine")
            .contains("unit.brief != null ? 'Your brief: ' + unit.brief : ")
            && function(&full, "briefLine").contains("itemRead(ctx.run, unit.item)"),
        "else the prompt names the read of the item's row"
    );
    for gone in [
        ".derivation",
        ".subject",
        "row.repro",
        "row.door",
        "row.clause",
        "row.grade",
        ".registries",
    ] {
        assert!(
            !ours.contains(gone),
            "`{gone}` is text of the record, and no code of a stage composes with it"
        );
    }
    assert!(
        function(&full, "itemRead").contains("'item --run ' + run + ' --item ' + item")
            && function(&full, "doorsRead")
                .contains("'item-doors --run ' + run + ' --round ' + round + ' --item ' + item")
            && function(&full, "briefLine")
                .contains("typed(RECORD_TOOL, itemRead(ctx.run, unit.item))")
            && function(&full, "doorLines")
                .contains("typed(RECORD_TOOL, doorsRead(ctx.run, unit.round, unit.item))")
            && function(&full, "doorLines").contains("doorsRead(ctx.run, unit.round, unit.item)")
            && function(&full, "preflightPrompt").contains("itemRead(ctx.run, c.item)")
            && function(&full, "crossModelPrompt").contains("itemRead(ctx.run, unit.item)")
            && function(&full, "crossModelPrompt").contains("doorLines(ctx, unit)")
            && function(&full, "unitPrompt").contains("briefLine(ctx, unit)")
            && function(&full, "unitPrompt").contains("doorLines(ctx, unit)"),
        "a brief and a door list are each handed over as the read that prints them"
    );
    // A PENDING BATCH'S SUBJECT is handed to nobody any more: no executor runs for a batch
    // that is applied (the second repair plan's `K11`), so no prompt names its read.
    assert!(
        !ours.contains("pending --run") && function(&full, "pendingRecord").contains("text: null"),
        "a pending batch has no executor, and no prompt"
    );
    assert_eq!(
        ours.matches("doorList(").count(),
        3,
        "a door list is printed for the script's own unit (`doorLines`) and for a probe's reviewer, and by nothing else"
    );
    assert!(
        function(&full, "ledgerSource").contains("findings: []")
            && function(&full, "triagePrompt")
                .contains("(s.read ? ' the rows ' + s.read + ' prints"),
        "triage is handed the read of the untriaged rows, and none of them"
    );

    // (5) NO RETURN HOLDS THE DOCUMENT: what goes back of the state is what the digest
    // holds, and the file the document lies in.
    let next = function(&full, "nextOf");
    assert!(
        next.contains("returned_to_orchestrator: true, state: attached(state), named, arrived")
            && next.matches("named").count() >= 5,
        "a `next` the script does not know goes back with what the digest holds and the file — and every value of `next` with what nothing can hold"
    );
    assert!(
        !ours.contains("true, state }") && !ours.contains("human_list: state.human_list,\n"),
        "no return carries the state, or a list of it"
    );
    let named = function(&full, "namedOf");
    assert!(
        named.contains("never_selected: state.never_selected || []")
            && named.contains("regression_unknown: (state.regression_unknown || []).length")
            && named.contains("document: state.document || null")
            && named.contains("(state.uncovered || []).find((u) => u.round === within)"),
        "every return names the items no round selected, the doors of the round no item names, how many confirmed findings have their regression fact unknown, and the file"
    );
    assert!(
        function(&full, "halt").contains("named: namedOf(lastState),"),
        "a halt names them too"
    );
    for stopped in [
        "status: 'stopped', after: 'state', stage: 'test', run: v.run, round:",
        "status: 'stopped', after: 'preflight'",
        "status: 'stopped', after: 'triage'",
    ] {
        let line = function(&full, "runTest")
            .lines()
            .find(|line| line.contains(stopped))
            .unwrap_or_else(|| panic!("the `test` stage has the stop `{stopped}`"));
        assert!(
            line.contains("named: namedOf(state"),
            "a stop names them: {line}"
        );
    }
    // A step's refusal goes back as its word and the file its own line is in.
    let git_halt = function(&full, "gitHalt");
    assert!(
        git_halt.contains("stepRefusal(word, ")
            && git_halt.contains("step: stepFile(r)")
            && git_halt.contains("halt: word || r.relay ? null : r.halt || null"),
        "a refused step is its word, the script's sentence for it and the file — never the tool's prose"
    );

    // (6) THE SCRIPT HAS A ROW FOR EVERY WORD THE STEP TOOL REFUSES WITH, and for no other.
    let words = tool_keys(&tool, "\nSTATUS = {\n");
    assert!(
        words.len() >= 25,
        "the scan found the tool's words: {words:?}"
    );
    let table = &full[full
        .find("\nconst STEP_REFUSALS = {\n")
        .expect("the script's rows for a step's refusal")..];
    let table = &table[..table.find("\n}\n").expect("the table closes")];
    let rows: BTreeSet<String> = table
        .lines()
        .filter_map(|line| line.strip_prefix("  '"))
        .map(|rest| rest[..rest.find('\'').expect("the word closes")].to_owned())
        .collect();
    assert_eq!(
        rows, words,
        "the words {HARNESS} has a row for (left) are the words {STEP_TOOL} refuses with (right)"
    );
    for row in table.lines().filter(|line| line.starts_with("  '")) {
        assert!(
            row.contains("': { whose: '") && row.contains(", leaves: '"),
            "a row says whose the state is, and what leaves it: {row}"
        );
    }

    // (8) A ROUND TESTS ONE CANDIDATE, AND NO WRITER IS HANDED THE BRANCH'S TIP (the
    // orchestrator's ruling of 2026-10-07 on the core review's F3): a re-run's commit is
    // the candidate the state names for its round; the attempt's marker, the candidate's
    // gate, the results and the round's facts are all written for that one commit; and a
    // check that reads the working tree is not asked for where the tree is not the candidate.
    let tested = function(&full, "runTest");
    for held in [
        "const sha = rerun ? String((state.rounds.find((r) => r.round === rerun.round) || {}).candidate || '') : String(gs.head)",
        "const moved = sha !== String(gs.head)",
        "checks: moved ? [] : always, crossModel: crossNamed.length > 0, tested: moved }",
        "const binaries = await buildBoth(ctx, label, sha, previousOf(state), 'Preflight and scope')",
        "const unbegun = await beginAttempt(ctx, launch, sha)",
        "gate: pre.gate ? { commit: sha, file: pre.gate.output } : null, results: { commit: sha, rows: resultRows(unitStatus) }",
        "flags: '--previous ' + previousOf(state).commit + ' --candidate ' + sha + ' --list ' + REGRESSION_LIST,",
        "const facts = rerun ? null : { candidate: sha, binary: built.candidate.sha256 }",
        "const c = offTree ? null : checks.find((x) => x.check === item.item)",
    ] {
        assert!(tested.contains(held), "the `test` stage holds `{held}`");
    }
    assert_eq!(
        tested.matches("gs.head").count(),
        4,
        "the tip is read for the commit the stage began on, for a round that has no candidate yet, to tell whether the candidate is still the tree, and for what an invocation that only looks returns — and is handed to no writer"
    );

    // (7) A HELD CHECK'S KIND IS A KIND THE TOOL HOLDS, in the form the record script reads.
    let holds: BTreeSet<String> = tool
        .split("\n    {\"kind\": \"")
        .skip(1)
        .map(|rest| rest[..rest.find('"').expect("the kind closes")].to_owned())
        .collect();
    assert!(
        holds.len() >= 4,
        "the scan found the tool's held kinds: {holds:?}"
    );
    let checks = quoted_in(&full, "const HELD_CHECKS = ");
    assert!(
        !checks.is_empty() && checks.iter().all(|kind| holds.contains(kind)),
        "a held check `{checks:?}` is a held kind of the tool `{holds:?}`"
    );
    let record =
        fs::read_to_string(repo_root().join(RECORD_SCRIPT)).expect("read the record script");
    assert!(
        source.contains("\nconst HELD_PREFIX = 'held-'\n") && record.contains("\"held-\""),
        "the kind's prefix is the one the record script reads"
    );
    assert!(
        function(&full, "resultRows")
            .contains("if (unit.verdict) return { item: unit.item, verdict: unit.verdict }"),
        "a held check's result is the verdict's file, and no word"
    );
    // THE ONE WORD OF A HELD CHECK THAT LEFT NO VERDICT has one shape, and it is the record
    // script's: the harness's pattern is that script's, character for character — so no
    // row the harness composes of one is a row its batch is refused for.
    let theirs = record
        .lines()
        .find_map(|line| line.strip_prefix("HELD_VOID = re.compile(r\""))
        .and_then(|rest| rest.strip_suffix("\")"))
        .expect("the record script declares the shape of a held void's reason");
    let ours = source
        .lines()
        .find_map(|line| line.strip_prefix("const HELD_VOID_RE = /^"))
        .and_then(|rest| rest.strip_suffix("$/"))
        .expect("the harness declares the shape of a held void's reason");
    assert_eq!(
        ours, theirs,
        "the shape of the word a held check is void by (left: the harness's) is the record script's (right)"
    );
    assert!(
        function(&full, "runTest")
            .contains("{ item: h.item, status: 'void', reason: heldVoid(h) }"),
        "a held check that left no verdict is recorded void, by that word"
    );
}

// ---------------------------------------------------------------------------
// (s) every command a prompt names is one plain invocation of a tool
// ---------------------------------------------------------------------------

/// The three tools a prompt of the harness may name a command of — what the allow rules of
/// the committed settings are written from, one rule per tool, by a command's first word.
pub(crate) const PROMPT_TOOLS: [&str; 3] = [
    "dev/stabilize-step",
    "dev/stabilize-record",
    "dev/stabilize-probe",
];

/// A word a code span may not be: a program an agent would run by a shell of its own. The
/// tools of a run are reached THROUGH the step tool's acts, never typed.
const PROGRAMS: &[&str] = &[
    "git",
    "cargo",
    "shasum",
    "sha256sum",
    "sh",
    "bash",
    "zsh",
    "sleep",
    "nohup",
    "cat",
    "mkdir",
    "mktemp",
    "tar",
    "gh",
    "codex",
    "node",
    "python",
    "python3",
    "timeout",
    "watch",
    "tee",
    "xargs",
    "env",
    "cd",
    "rm",
    "cp",
    "mv",
    "tail",
    "head",
    "grep",
    "command",
];

/// A code span of several words that is no command, by what it is — the whole list: a
/// span of several words that is neither an invocation of a tool, nor a list of flags, nor
/// one of these, is red.
const NOT_COMMANDS: &[(&str, &str)] = &[(
    "left open",
    "the mark triage is told an entry carries: two words of a list, no program",
)];

/// What a prompt tells an agent to do BY ITS OWN MEANS, as a word: each is a shell the
/// agent composes, and a permission prompt nobody is watching (the human's ruling of
/// 2026-10-07). Read outside the commands of the three tools.
const OWN_MEANS: &[&str] = &[
    "wait",
    "waits",
    "waited",
    "waiting",
    "background",
    "redirect",
    "redirected",
    "redirects",
    "pipe",
    "piped",
    "pipes",
    "piping",
    "poll",
    "polls",
    "polled",
    "polling",
    "sleep",
    "sleeps",
    "nohup",
    "here-document",
    "heredoc",
    "slices",
];

/// The characters a shell composes with: none stands anywhere in a prompt.
const SHELL_SHAPES: &[(char, &str)] = &[
    ('|', "a pipe"),
    ('>', "a redirect"),
    ('<', "a redirect, or a here-document"),
    ('&', "`&&`, or a command sent to the background"),
    ('$', "a substitution"),
];

/// The report's end marker — the one text with a `<` in it that a prompt spells: the last
/// line of a report, as `dev/stabilize-record` holds it.
const END_MARKER: &str = "<!-- end of report -->";

/// The line a payload stands between, twice: the text an agent writes to a file with its
/// file tool — DATA, whatever it holds, and no instruction.
const PAYLOAD_MARKER: &str = "STABILIZE_PAYLOAD";

/// THE EXCEPTIONS — what a prompt still names that is no plain invocation of a tool, each
/// with the prompt it stands in, how its line opens (`*`: the whole prompt), why it is not
/// replaced, and what it must still hold: an exception that no prompt meets, or whose line
/// no longer holds its shape, is stale, and red.
const EXCEPTIONS: &[(&str, &str, &str, &[&str])] = &[
    (
        "proposalPrompt",
        "HOW. ",
        "the plan's row 12, NOT REPLACEABLE: an advocate's proposal is a PATCHED tree, which is no commit — and the step tool builds a commit and nothing else. The independent drive clones, patches and builds by a shell of its own, under the session's permission mode; the hash it asserts is asked of the tool, on a line of its own",
        &[
            "`git clone`",
            "`cargo build --release --locked`",
            "`mktemp -d`",
        ],
    ),
    (
        "crossModelPrompt",
        "HOW. ",
        "the cross-model pass runs a tool of another model family, which the commit holds no wrapper for and no stage needs: it is launched only for an item the invocation names, one by one, and naming it is the human's approval — its one command is a shell of the role's own",
        &[" review - ", "`mktemp -d`"],
    ),
    (
        "preflightPrompt",
        "whether the cross-model pass's tool answers",
        "the same pass's own assert, asked only where the invocation names an item for it: whether that tool answers at all",
        &["`command -v "],
    ),
    (
        "carryPrompt",
        "*",
        "the `fix` half's, which refuses to start: the re-cut of a part is ruled to be rebuilt as a revert (DECISIONS.md, 2026-10-06), and a mechanism about to go is not ported — it is the one step that is still a list of git commands",
        &["`git cherry-pick -x "],
    ),
];

/// One word of a plain invocation: bare, or in single quotes with nothing a shell reads.
fn plain_word(word: &str) -> bool {
    let bare = |text: &str, more: &str| {
        !text.is_empty()
            && text
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || "_./:=@%+,-".contains(c) || more.contains(c))
    };
    match word.strip_prefix('\'').and_then(|w| w.strip_suffix('\'')) {
        Some(quoted) => bare(quoted, " ()"),
        None => bare(word, ""),
    }
}

/// A command line as its words: split at spaces outside single quotes.
fn words_of(line: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut word = String::new();
    let mut quoted = false;
    for c in line.chars() {
        if c == '\'' {
            quoted = !quoted;
        }
        if c == ' ' && !quoted {
            words.push(std::mem::take(&mut word));
        } else {
            word.push(c);
        }
    }
    words.push(word);
    words
}

/// Whether a code span is a list of flags: it opens with `--`, and every word is plain.
fn is_flags(span: &str, words: &[&str]) -> bool {
    span.starts_with("--") && words.iter().all(|word| plain_word(word))
}

/// Why a code span that opens with a tool's path is no plain invocation of it, or `None`.
fn call_fault(span: &str) -> Option<String> {
    let words = words_of(span);
    if !PROMPT_TOOLS.contains(&words[0].as_str()) {
        return Some(format!(
            "`{span}` opens with `{}`, which is none of the three tools a prompt may name",
            words[0]
        ));
    }
    words[1..]
        .iter()
        .find(|word| !plain_word(word))
        .map(|word| format!("`{span}` is no plain invocation: `{word}` is no plain word"))
}

/// A prompt, held: what in it is not one plain invocation of a tool, or tells an agent to
/// do by its own means what a tool does — and the tools its commands are of. `name` is the
/// prompt's composer, for the exceptions; `met` collects the exceptions it met.
pub(crate) fn prompt_faults(
    name: &str,
    prompt: &str,
    met: &mut BTreeSet<&'static str>,
) -> (Vec<String>, BTreeSet<String>) {
    let mut faults = Vec::new();
    let mut tools = BTreeSet::new();
    // What is cut before anything is read: the paragraph every definition carries, a
    // payload — data — and the lines that are the named exceptions.
    let mut kept: Vec<&str> = Vec::new();
    let mut payload = false;
    let whole = EXCEPTIONS
        .iter()
        .find(|(of, opens, _, _)| *of == name && *opens == "*" && prompt.contains("re-cut a part"));
    if let Some((_, _, _, holds)) = whole {
        met.insert("carryPrompt");
        for shape in *holds {
            if !prompt.contains(shape) {
                faults.push(format!(
                    "the exception of `{name}` no longer holds `{shape}`: it is stale"
                ));
            }
        }
        return (faults, tools);
    }
    for line in prompt.lines() {
        if line == PAYLOAD_MARKER {
            payload = !payload;
            continue;
        }
        if payload || line.starts_with("**Never push to or merge into `main`") {
            continue;
        }
        let excepted = EXCEPTIONS.iter().find(|(of, opens, _, _)| {
            *of == name
                && *opens != "*"
                && (line.starts_with(opens)
                    || line
                        .split_once(". ")
                        .is_some_and(|(_, rest)| rest.starts_with(opens)))
        });
        if let Some((of, opens, _, holds)) = excepted {
            met.insert(if *opens == "HOW. " {
                *of
            } else {
                "preflightPrompt"
            });
            for shape in *holds {
                if !line.contains(shape) {
                    faults.push(format!(
                        "the exception of `{name}` (`{opens}…`) no longer holds `{shape}`: it is stale"
                    ));
                }
            }
            continue;
        }
        kept.push(line);
    }
    if payload {
        faults.push("a payload opens and does not close".to_owned());
    }
    let text = kept.join("\n").replace(END_MARKER, "");
    for (shape, what) in SHELL_SHAPES {
        if let Some(at) = text.find(*shape) {
            let from = text[..at].rfind('\n').map_or(0, |n| n + 1);
            let line = &text[from..];
            let line = &line[..line.find('\n').unwrap_or(line.len())];
            faults.push(format!(
                "`{shape}` — {what} — stands in: {}",
                &line[..line.len().min(200)]
            ));
        }
    }
    // The code spans.
    let pieces: Vec<&str> = text.split('`').collect();
    if pieces.len().is_multiple_of(2) {
        faults.push("a code span opens and does not close".to_owned());
    }
    let mut prose = String::new();
    for (n, piece) in pieces.iter().enumerate() {
        if n % 2 == 0 {
            prose.push_str(piece);
            prose.push(' ');
            continue;
        }
        let span = piece.trim();
        if span.is_empty() {
            continue;
        }
        if span.starts_with("dev/") {
            match call_fault(span) {
                Some(fault) => faults.push(fault),
                None => {
                    tools.insert(words_of(span)[0].clone());
                }
            }
            continue;
        }
        prose.push_str(span);
        prose.push(' ');
        let words: Vec<&str> = span.split_whitespace().collect();
        if words.len() == 1 {
            if PROGRAMS.contains(&words[0]) {
                faults.push(format!(
                    "`{span}` is a program an agent would run by a shell of its own"
                ));
            }
        } else if PROGRAMS.contains(&words[0]) {
            faults.push(format!(
                "`{span}` is a command of `{}`: a prompt names a command of one of the three tools, and of no other program",
                words[0]
            ));
        } else if !(is_flags(span, &words) || NOT_COMMANDS.iter().any(|(text, _)| *text == span)) {
            faults.push(format!(
                "`{span}` is a code span of several words that is no invocation of a tool, no list of flags and no text this suite lists as no command"
            ));
        }
    }
    // A tool, or any other script of `dev/`, named outside a code span.
    for word in prose.split(|c: char| c.is_whitespace() || "(),;:".contains(c)) {
        let word = word.trim_end_matches('.');
        if word.starts_with("dev/") && !PROMPT_TOOLS.contains(&word) {
            faults.push(format!(
                "`{word}` is a script of `dev/` no prompt may send an agent to"
            ));
        }
    }
    // What an agent is told to do by its own means.
    for word in prose
        .split(|c: char| !(c.is_ascii_alphanumeric() || c == '-'))
        .filter(|word| !word.is_empty())
    {
        let lower = word.to_ascii_lowercase();
        if OWN_MEANS.contains(&lower.as_str()) {
            faults.push(format!(
                "the word `{word}`: a prompt tells no agent to wait, to send a command to the background, to redirect, to pipe or to poll by its own means"
            ));
        }
    }
    (faults, tools)
}

/// Every prompt the harness can compose, as JavaScript evaluated beside its pure functions:
/// `[name of the composer, what this call of it is, the expression]`. Whatever an agent or
/// the record would supply is a plain sentinel, so what a prompt holds is the harness's.
const PROMPTS: &[(&str, &str, &str)] = &[
    (
        "stepPrompt",
        "a step",
        "stepPrompt('a step', 'state', '--run rc24')",
    ),
    (
        "stepPrompt",
        "a step that asks again",
        "stepPrompt('a held command', 'hold-wait', '--scratch /s --name n', true)",
    ),
    ("gitStatePrompt", "the first read", "gitStatePrompt(V)"),
    ("gitStatePrompt", "a look", "gitStatePrompt(V, true)"),
    ("statePrompt", "", "statePrompt(V, 'test-1')"),
    ("beginPrompt", "", "beginPrompt(CTX, SHA)"),
    (
        "checkReportsPrompt",
        "",
        "checkReportsPrompt(CTX, ['attempt', 'preflight', 'scope'])",
    ),
    ("findRoundPrompt", "", "findRoundPrompt(V, 1)"),
    (
        "openRoundPrompt",
        "",
        "openRoundPrompt(V, 1, 'fix/rc24-r1')",
    ),
    ("pushPrompt", "a branch", "pushPrompt(V, 'fix/rc24-r1')"),
    ("pushPrompt", "a record", "pushPrompt(V, 'fix/rc24', true)"),
    (
        "recordCommitPrompt",
        "",
        "recordCommitPrompt(V, 'fix/rc24', '/s/hold/record-test-r1-a1-1/output', { calls: 3, checks: 2 }, SHA)",
    ),
    ("landPrompt", "", "landPrompt(V, 1, 'fix/rc24-r1')"),
    (
        "roundCommitsPrompt",
        "",
        "roundCommitsPrompt(V, 1, 'fix/rc24-r1')",
    ),
    (
        "carryPrompt",
        "a dropped round",
        "carryPrompt(V, 1, [SHA], null)",
    ),
    (
        "carryPrompt",
        "a part",
        "carryPrompt(V, 1, [SHA], 'fix/rc24-r1-part1')",
    ),
    ("syncMainPrompt", "", "syncMainPrompt('rc24', '/s')"),
    (
        "holdStartPrompt",
        "a gate",
        "holdStartPrompt(V, 'gate-c1-a1', 'gate', '--run rc24', 'the gate')",
    ),
    (
        "holdStartPrompt",
        "a build",
        "holdStartPrompt(V, 'build-c1-a1', 'build', '--commit ' + SHA + ' --to bin/c1.a1/jigc --version 1.0.0-rc.24', 'a binary')",
    ),
    (
        "holdStartPrompt",
        "the regression set",
        "holdStartPrompt(V, 'regression-set-c1-a1', 'regression', '--previous ' + SHA + ' --candidate ' + SHA + ' --list ' + REGRESSION_LIST, 'the regression set')",
    ),
    (
        "holdStartPrompt",
        "the probe",
        "holdStartPrompt(V, 'a', 'probe', '--seconds 720', 'the hold')",
    ),
    (
        "holdWaitPrompt",
        "",
        "holdWaitPrompt(V, 'gate-c1-a1', 'the gate')",
    ),
    (
        "preflightPrompt",
        "a round's first",
        "preflightPrompt(CTX, launcher(CTX), 'preflight', { image: false, sha: SHA, label: 'c1', branch: 'fix/rc24', checks: [{ item: 'ci' }, { item: 'tarball' }], crossModel: false, tested: false })",
    ),
    (
        "preflightPrompt",
        "a candidate that is not the tree",
        "preflightPrompt(CTX, launcher(CTX), 'preflight', { image: false, sha: SHA, label: 'c1', branch: 'fix/rc24', checks: [], crossModel: false, tested: true })",
    ),
    (
        "preflightPrompt",
        "the second: the image",
        "preflightPrompt(CTX, launcher(CTX), 'preflight-second', { image: true, sha: SHA, label: 'c1', branch: 'fix/rc24', checks: [{ item: 'late' }], crossModel: false, tested: false })",
    ),
    (
        "preflightPrompt",
        "with the cross-model pass's assert",
        "preflightPrompt(CTX, launcher(CTX), 'preflight', { image: false, sha: SHA, label: 'c1', branch: 'fix/rc24', checks: [], crossModel: true, tested: false })",
    ),
    (
        "scopePrompt",
        "the first round",
        "scopePrompt(CTX, launcher(CTX), 'scope', { sha: SHA, label: 'c1', base: null, earlier: false, scope: null, previous: PREVIOUS, fallback: 'delta' })",
    ),
    (
        "scopePrompt",
        "a later round, named doors",
        "scopePrompt(Object.assign({}, CTX, { round: 2 }), launcher(CTX), 'scope', { sha: SHA, label: 'c2', base: SHA, earlier: true, scope: { doors: ['a door'] }, previous: PREVIOUS, fallback: 'everything' })",
    ),
    (
        "scopePrompt",
        "a range",
        "scopePrompt(CTX, launcher(CTX), 'scope', { sha: SHA, label: 'c1', base: null, earlier: false, scope: { range: 'abcdef1..1234567' }, previous: PREVIOUS, fallback: 'delta' })",
    ),
    (
        "unitPrompt",
        "a reviewer, by the reads",
        "unitPrompt(CTX, launcher(CTX), 'row-a-source', { as: 'source', role: 'review', task: 'the SOURCE PASS of this review row' }, { item: 'row-a', doors: 2, round: 1 }, BUILT, [])",
    ),
    (
        "unitPrompt",
        "a driver with the image, handed reports",
        "unitPrompt(CTX, launcher(CTX), 'arm-a-run', { as: 'run', role: 'drive', task: 'RUN this trial arm', image: true }, { item: 'arm-a', doors: null, round: 1 }, BUILT, [{ as: 'rehearse', report: 'completions/artifacts/rc24/r1/reports/test/arm-a-rehearse.a1.md' }, { as: 'other', report: null }])",
    ),
    (
        "unitPrompt",
        "the audit of a fix diff, as given",
        "unitPrompt(Object.assign({}, CTX, { stage: 'fix', cycle: 1 }), launcher(Object.assign({}, CTX, { stage: 'fix', cycle: 1 })), 'audit-review', { as: 'review', role: 'review', task: 'the review of this round\\'s FIX DIFF' }, { item: 'fix-diff', brief: 'a brief of the script\\'s own', doors: [{ door: 'a door', registry: 'verbs' }], range: 'abcdef1..1234567' }, BUILT, [])",
    ),
    (
        "triagePrompt",
        "the first pass, with the ledger's rows",
        "triagePrompt(CTX, launcher(CTX), 'triage-p1', [{ reporter: 'row-a-source', report: 'a/report.md', findings: ['F1 — a title · door: a door · clause: a-clause · repro: a block'] }].concat(ledgerSource('rc24', { untriaged: { count: 2 } })), 1)",
    ),
    (
        "triagePrompt",
        "a later pass",
        "triagePrompt(CTX, launcher(CTX), 'triage-p2', [{ reporter: 'verify-p1-a-key', report: null, findings: ['left open 1 — a thing'] }], 2)",
    ),
    (
        "verifyPrompt",
        "",
        "verifyPrompt(CTX, launcher(CTX), 'verify-p1-a-key', { key: 'a-key', door: 'a door', clause: 'a-clause', grade: 'breaks', repro: 'a block' }, 'what to drive again', BUILT)",
    ),
    (
        "advocatePrompt",
        "",
        "advocatePrompt(CTX, launcher(CTX), 'advocate-p1-a-key', { key: 'a-key', kind: 'contested', door: 'a door', clause: 'a-clause', repro: 'a block', statement: 'a statement' }, BUILT)",
    ),
    (
        "proposalPrompt",
        "",
        "proposalPrompt(CTX, 'proposal-p1-a-key', { key: 'a-key' }, { proposal: 'a proposal', report: 'a/report.md', driven: [{ step: 'a step', command: 'a command of the advocate', result: 'a result' }] }, BUILT)",
    ),
    (
        "crossModelPrompt",
        "",
        "crossModelPrompt(CTX, 'row-a-crossmodel', { item: 'row-a', doors: 2, round: 1 })",
    ),
    (
        "fixerPrompt",
        "",
        "fixerPrompt(Object.assign({}, CTX, { stage: 'fix', cycle: 1 }), launcher(Object.assign({}, CTX, { stage: 'fix', cycle: 1 })), 'fixer-1', { registry: 'verbs', findings: [{ key: 'a-key', door: 'a door', clause: 'a-clause', repro: 'a block', detail: 'a ruling' }] }, 'fix/rc24-r1')",
    ),
    (
        "recordPrompt",
        "a stage's record",
        "recordPrompt(V, 'the record of a stage', 'fix/rc24', '/s/record/test-r1-a1', 1, stageRecordCommands(CTX, { reporters: ['attempt'], gate: { commit: SHA, file: '/s/hold/gate-c1-a1/output' }, results: { commit: SHA, rows: [{ item: 'row-a', outcome: 'green' }] }, rows: [], patches: [], triage: [], facts: { candidate: SHA }, keys: [] }), subjectOf('rc24', 1, 'the record of the test stage')).text",
    ),
    (
        "rulingsRecordPrompt",
        "",
        "rulingsRecordPrompt(V, 1, 'fix/rc24', [{ go: true }], {}, { round: 1, stages: {} }).text",
    ),
    (
        "probeStep",
        "",
        "probeStep('its first act', 'begin', '--scratch /s --probe relay')",
    ),
    (
        "probeReviewPrompt",
        "case a",
        "probeReviewPrompt('a', { file: '/s/probe/required/binary', sha256: HASH })",
    ),
    (
        "probeReviewPrompt",
        "case b",
        "probeReviewPrompt('b', { file: '/s/probe/required/binary', sha256: HASH })",
    ),
    (
        "probePayloadPrompt",
        "",
        "probePayloadPrompt('/s', 20, probeBatch('/s', 20))",
    ),
    ("LOOK_AGAIN", "what an agent's retry is told", "LOOK_AGAIN"),
    ("RUN_AGAIN", "what a step's retry is told", "RUN_AGAIN"),
    (
        "relayAgain",
        "what a state read asked again is told",
        "relayAgain(2)",
    ),
    (
        "schemaWords",
        "what every schema says of a field",
        "schemaWords()",
    ),
];

/// What composes a prompt and is in no row of [`PROMPTS`] by its own name, each with the
/// composer that holds it.
const HELD_IN: &[(&str, &str)] = &[
    (
        "reportLine",
        "every reporter's prompt, through `launch.line`",
    ),
    ("binaryLine", "every driving agent's prompt"),
    ("branchLine", "the record's and the fixer's"),
    ("briefLine", "a unit's prompt"),
    ("doorLines", "a unit's prompt, and the cross-model pass's"),
    (
        "findingLines",
        "triage's prompt: what a reporter returned of its own findings, an agent's text",
    ),
];

/// Evaluates [`PROMPTS`] beside the harness's pure functions: per row the text, or the
/// error of an expression the script cannot evaluate.
const COMPOSER: &str = r#"
import { readFileSync } from 'node:fs'
const [script, rowsJson] = process.argv.slice(2)
const source = readFileSync(script, 'utf8').replace(/^export const meta = /m, 'const meta = ')
const pure = source.slice(0, source.indexOf('\n// ---- args: parsed, and refused before any agent ----\n'))
const rows = JSON.parse(rowsJson)
const setup = `
const SHA = 'a'.repeat(40)
const HASH = 'b'.repeat(64)
const V = { run: 'rc24', stage: 'test', scratch: '/s' }
const CTX = { run: 'rc24', round: 1, stage: 'test', attempt: 1, scratch: '/s' }
const PREVIOUS = { version: '1.0.0-rc.24', commit: SHA }
const BUILT = { candidate: { label: 'c1', sha: SHA, binary: '/s/bin/c1.a1/jigc', sha256: HASH }, previous: { version: '1.0.0-rc.24', binary: '/s/bin/previous/jigc', sha256: HASH }, image: { tag: 'jigc-trial:c1' } }
function schemaWords() {
  const said = []
  const walk = (value) => {
    if (!value || typeof value !== 'object') return
    if (typeof value.description === 'string') said.push(value.description)
    for (const inner of Object.values(value)) walk(inner)
  }
  for (const schema of [STEP_SCHEMA, CARRY_SCHEMA, PREFLIGHT_SCHEMA, SCOPE_SCHEMA, UNIT_SCHEMA, PROBE_UNIT_SCHEMA, TRIAGE_SCHEMA, VERIFY_SCHEMA, ADVOCATE_SCHEMA, PROPOSAL_SCHEMA, FIXER_SCHEMA, RECORD_SCHEMA]) walk(schema)
  return said.join('\\n')
}
`
const out = rows.map((row) => {
  try {
    const text = new Function(pure + setup + '\nreturn (' + row + ')')()
    return typeof text === 'string' ? { text } : { error: 'it composes no text: ' + JSON.stringify(text) }
  } catch (e) {
    return { error: String((e && e.message) || e) }
  }
})
console.log(JSON.stringify(out))
"#;

/// [`PROMPTS`], composed by the committed script (or by `source`, a mutant of it).
fn composed(source: Option<&str>) -> Vec<(String, String, Result<String, String>)> {
    let scratch = ScratchDir::new("stabilize-prompts");
    let driver = scratch.path().join("composer.mjs");
    fs::write(&driver, COMPOSER).expect("write the composer");
    let script = match source {
        Some(text) => {
            let path = scratch.path().join("stabilize.js");
            fs::write(&path, text).expect("write the mutant");
            path
        }
        None => repo_root().join(HARNESS),
    };
    let rows: Vec<&str> = PROMPTS.iter().map(|(_, _, row)| *row).collect();
    let out = Command::new("node")
        .arg(&driver)
        .arg(&script)
        .arg(serde_json::to_string(&rows).expect("the rows as JSON"))
        .output()
        .expect("run node");
    assert!(
        out.status.success(),
        "the composer under node: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let said: Vec<Value> = serde_json::from_slice(&out.stdout).expect("one JSON list");
    PROMPTS
        .iter()
        .zip(said)
        .map(|((name, what, _), one)| {
            let text = match one["text"].as_str() {
                Some(text) => Ok(text.to_owned()),
                None => Err(one["error"].as_str().unwrap_or("no text").to_owned()),
            };
            ((*name).to_owned(), (*what).to_owned(), text)
        })
        .collect()
}

/// Every fault of every prompt of a script, and the tools its prompts name.
fn all_faults(source: Option<&str>) -> (Vec<String>, BTreeSet<String>, BTreeSet<&'static str>) {
    let mut faults = Vec::new();
    let mut tools = BTreeSet::new();
    let mut met = BTreeSet::new();
    for (name, what, text) in composed(source) {
        let at = if what.is_empty() {
            name.clone()
        } else {
            format!("{name} ({what})")
        };
        match text {
            Err(error) => faults.push(format!("{at}: the script composes none — {error}")),
            Ok(text) => {
                let (found, named) = prompt_faults(&name, &text, &mut met);
                faults.extend(found.into_iter().map(|fault| format!("{at}: {fault}")));
                tools.extend(named);
            }
        }
    }
    (faults, tools, met)
}

#[test]
fn s_every_command_a_prompt_names_is_one_plain_invocation_of_a_tool() {
    let full = harness();
    // The table is whole: every function of the script that composes a prompt has a row —
    // or is held in one, and says in which.
    let listed: BTreeSet<&str> = PROMPTS
        .iter()
        .map(|(name, _, _)| *name)
        .chain(HELD_IN.iter().map(|(name, _)| *name))
        .collect();
    let composers: BTreeSet<String> = functions(&full)
        .into_iter()
        .filter(|name| {
            name.ends_with("Prompt")
                || name.ends_with("Line")
                || name.ends_with("Lines")
                || name == "probeStep"
        })
        .collect();
    let mut unlisted: Vec<String> = composers
        .iter()
        .filter(|name| !listed.contains(name.as_str()))
        .map(|name| format!("`{name}` composes a prompt, and this suite composes none with it"))
        .collect();
    unlisted.extend(
        HELD_IN
            .iter()
            .filter(|(name, where_)| !composers.contains(*name) || where_.is_empty())
            .map(|(name, _)| {
                format!(
                    "`{name}` is listed as part of a prompt, and the script has no such function"
                )
            }),
    );
    if !node_or_skip(&format!("no prompt of {HARNESS} was composed and read")) {
        assert_eq!(unlisted, Vec::<String>::new());
        return;
    }
    let (mut faults, tools, met) = all_faults(None);
    faults.extend(unlisted);
    assert!(
        faults.is_empty(),
        "what the prompts of {HARNESS} name that is no plain invocation of a tool, or tell an agent to do by its own means:\n{}",
        faults.join("\n")
    );
    // The tools found — what the allow rules are written from.
    println!("the tools the prompts of {HARNESS} name a command of: {tools:?}");
    assert_eq!(
        tools,
        PROMPT_TOOLS
            .iter()
            .map(|tool| (*tool).to_owned())
            .collect::<BTreeSet<_>>(),
        "the tools the prompts name commands of (left) are the three the commit holds under `dev/` (right)"
    );
    for tool in PROMPT_TOOLS {
        assert!(
            repo_root().join(tool).is_file(),
            "the commit holds `{tool}`"
        );
    }
    // Every exception is met by a prompt, and says why.
    for (name, opens, why, _) in EXCEPTIONS {
        assert!(
            why.split_whitespace().count() >= 12,
            "the exception of `{name}` (`{opens}`) carries its reason"
        );
    }
    assert_eq!(
        met,
        [
            "carryPrompt",
            "crossModelPrompt",
            "preflightPrompt",
            "proposalPrompt"
        ]
        .into_iter()
        .collect::<BTreeSet<_>>(),
        "the exceptions a prompt met: one that none meets is stale"
    );

    // EACH SHAPE, PUT BACK INTO EACH PROMPT, IS RED — and so is each word, each program
    // and each script that is no tool of the three.
    let planted: &[(&str, &str)] = &[
        (" `dev/gate --keep-going`", "none of the three tools"),
        (
            " `dev/stabilize-step state --run rc24 > /s/out.txt 2>&1`",
            "a redirect",
        ),
        (" `dev/stabilize-step state --run rc24 | tail -1`", "a pipe"),
        (
            " `dev/stabilize-step state --run rc24 && dev/stabilize-step table`",
            "`&&`",
        ),
        (
            " `dev/stabilize-step state --run rc24 ; true`",
            "no plain word",
        ),
        (
            " `dev/stabilize-step hash --file $(pwd)/x`",
            "a substitution",
        ),
        (
            " `dev/stabilize-record report --run rc24 <<'END'`",
            "a here-document",
        ),
        (
            " `dev/stabilize-step hold-wait --name n &`",
            "the background",
        ),
        (" `sleep 60`", "a command of `sleep`"),
        (" `nohup dev/stabilize-step table`", "a command of `nohup`"),
        (" `cd /s`", "a command of `cd`"),
        (
            " `FOO=1 dev/stabilize-step table`",
            "no invocation of a tool",
        ),
        (" `git status --porcelain`", "a command of `git`"),
        (" `shasum -a 256 /s/x`", "a command of `shasum`"),
        (" `cargo build --release`", "a command of `cargo`"),
        (" `dev/regression-set run`", "none of the three tools"),
        (
            " `for n in 1 2 3; do dev/stabilize-step table; done`",
            "no invocation of a tool",
        ),
        (" Run dev/gate once.", "a script of `dev/`"),
        (" It runs in the background.", "the word `background`"),
        (" You wait for it in this same turn.", "the word `wait`"),
        (
            " Its output is redirected to a file.",
            "the word `redirected`",
        ),
        (" Never piped.", "the word `piped`"),
        (" Poll the file.", "the word `Poll`"),
        (
            " Written as a single here-document.",
            "the word `here-document`",
        ),
    ];
    for (name, what, text) in composed(None) {
        let text = text.expect("composed above");
        if EXCEPTIONS
            .iter()
            .any(|(of, opens, _, _)| *of == name && *opens == "*")
            && text.contains("re-cut a part")
        {
            continue;
        }
        let first = text.lines().next().expect("a prompt has a line").to_owned();
        for (shape, names) in planted {
            let mutant = text.replacen(&first, &format!("{first}{shape}"), 1);
            let (found, _) = prompt_faults(&name, &mutant, &mut BTreeSet::new());
            assert!(
                found.iter().any(|fault| fault.contains(names)),
                "`{shape}` put into `{name}` ({what}) must be named ({names}); found: {found:#?}"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// (t) what a stage needs of a return, and what still halts once an instrument is launched
// ---------------------------------------------------------------------------

/// What a `test` stage does NOT need of a driving role's return, by the schema the role is
/// launched under: every field that schema describes and does not require, and that the
/// harness's `NEEDS` has no row for — each with why its absence costs nothing. A field that
/// is in neither table is red: a stage that comes to need a field says what its absence
/// does (a row of `NEEDS`), or this table says why it needs none.
const NOT_NEEDED: &[(&str, &str, &str)] = &[
    (
        "UNIT_SCHEMA",
        "halt",
        "read only where the status is `halted`, for the words a void's reason carries: absent, the reason says `halted` and no more",
    ),
    (
        "UNIT_SCHEMA",
        "doors_affected",
        "the `fix` stage's: the doors a fix diff can change, read from the audit of one and by no `test` stage",
    ),
    (
        "UNIT_SCHEMA",
        "report",
        "a path handed on as text: whether a report is there is the report check's word, never a return's",
    ),
    (
        "UNIT_SCHEMA",
        "summary",
        "prose for a reader: no code reads it",
    ),
    (
        "VERIFY_SCHEMA",
        "halt",
        "read only where the status is `halted`, for the words an unverified finding's reason carries",
    ),
    (
        "VERIFY_SCHEMA",
        "basis",
        "the verdict's one line, passed on into a fork's statement as it came: absent, the statement says that none was returned",
    ),
    (
        "VERIFY_SCHEMA",
        "contested",
        "A DEFAULT, AND DECLARED: absent reads as not contested, which is what most verdicts are — an omission cannot be told from it, so no fork is driven and the verdict is recorded without one",
    ),
    (
        "VERIFY_SCHEMA",
        "ran_on",
        "the container of the two hashes: what is needed of it is its `previous`, which has its own row",
    ),
    (
        "VERIFY_SCHEMA",
        "ran_on.candidate",
        "a second copy of the candidate's hash: the one the stage compares is `asserted_sha256`, and no code reads this one",
    ),
    (
        "VERIFY_SCHEMA",
        "repro",
        "the heading of the block in the verifier's report: no code reads it, the report holds the block",
    ),
    (
        "VERIFY_SCHEMA",
        "pinnable",
        "the verifier's word to a reader: no code reads it",
    ),
    (
        "VERIFY_SCHEMA",
        "left_open",
        "absent reads as nothing left open: what an agent leaves open is a list, and an empty one is the usual one",
    ),
    (
        "VERIFY_SCHEMA",
        "report",
        "a path handed on as text: whether a report is there is the report check's word, never a return's",
    ),
    (
        "ADVOCATE_SCHEMA",
        "halt",
        "read only where the status is `halted`, for the words an unverified finding's reason carries",
    ),
    (
        "ADVOCATE_SCHEMA",
        "proposal",
        "handed on to the independent drive as text: absent, that prompt says that none was returned and to read it from the advocate's report",
    ),
    (
        "ADVOCATE_SCHEMA",
        "driven",
        "handed on to the independent drive as lines: absent, that prompt lists none, and the drive works from the report",
    ),
    (
        "ADVOCATE_SCHEMA",
        "undriven",
        "the advocate's word to a reader: no code reads it",
    ),
    (
        "ADVOCATE_SCHEMA",
        "left_open",
        "absent reads as nothing left open: what an agent leaves open is a list, and an empty one is the usual one",
    ),
    (
        "ADVOCATE_SCHEMA",
        "report",
        "a path handed on as text: whether a report is there is the report check's word, never a return's",
    ),
    (
        "PROPOSAL_SCHEMA",
        "halt",
        "read only where the status is `halted`, for the words an unverified finding's reason carries",
    ),
    (
        "PROPOSAL_SCHEMA",
        "undriven",
        "the driver's word to a reader: no code reads it",
    ),
    (
        "PROPOSAL_SCHEMA",
        "left_open",
        "absent reads as nothing left open: what an agent leaves open is a list, and an empty one is the usual one",
    ),
    (
        "PROPOSAL_SCHEMA",
        "report",
        "a path handed on as text: whether a report is there is the report check's word, never a return's",
    ),
];

/// The cells of `NEEDS` whose word is still `halts`: a field whose absence halts the stage
/// after its instruments ran — each with the task it is owed to, and why this commit could
/// not turn it. A `halts` cell that is not here is red, and so is a row here whose cell is
/// gone or turned: the task that turns it adds its arm to the simulation's `ABSENT_ARMS`
/// and takes its row out of this list in the same commit.
///
/// NONE IS OWED TODAY: the two that were — a verifier's `regression` and its
/// `ran_on.previous`, with `confirmed` — were turned by the second repair plan's `X1b` into
/// the cell `unknown`, and each has its arm.
const OWED: &[(&str, &str, &str, &str)] = &[];

/// The words a cell of `NEEDS` may hold: what the absence of the field does.
const CELLS: [&str; 5] = ["void", "unverified", "no-fork", "unknown", "halts"];

/// The row of the stage's stated halts that exists only while a cell is owed.
const OWED_HALT: &str = "regression-fact";

/// Reads the harness's two tables, its roles, and — per schema named — every field the
/// schema describes and does not require: by its dotted path through objects, the halt
/// report as one field, a list's elements not descended into.
const CENSUS: &str = r#"
import { readFileSync } from 'node:fs'
const [script, namesJson] = process.argv.slice(2)
const source = readFileSync(script, 'utf8').replace(/^export const meta = /m, 'const meta = ')
const pure = source.slice(0, source.indexOf('\n// ---- args: parsed, and refused before any agent ----\n'))
const names = JSON.parse(namesJson)
const read = `
function optionalOf(schema, at) {
  const out = []
  for (const [name, inner] of Object.entries(schema.properties || {})) {
    const path = at ? at + '.' + name : name
    if (!(schema.required || []).includes(name)) out.push(path)
    if (inner !== HALT && inner.type === 'object') out.push(...optionalOf(inner, path))
  }
  return out
}
return { needs: NEEDS, halts: HALTS_AFTER, roles: ROLES, optional: { ${names.map((n) => n + ': optionalOf(' + n + ', \'\')').join(', ')} } }
`
console.log(JSON.stringify(new Function(pure + read)()))
"#;

fn census(schemas: &BTreeSet<String>) -> Value {
    let scratch = ScratchDir::new("stabilize-census");
    let driver = scratch.path().join("census.mjs");
    fs::write(&driver, CENSUS).expect("write the census");
    let out = Command::new("node")
        .arg(&driver)
        .arg(repo_root().join(HARNESS))
        .arg(serde_json::to_string(schemas).expect("the names as JSON"))
        .output()
        .expect("run node");
    assert!(
        out.status.success(),
        "the script's tables of what a stage needs of a return, and of what still halts, under node: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    serde_json::from_slice(&out.stdout).expect("one JSON object")
}

/// The schema each role is launched under, read off the script's `roleStep` calls outside
/// the probes: a role named by a literal, or — `step.role` — every role a chain names.
fn role_schemas(full: &str) -> BTreeMap<String, BTreeSet<String>> {
    let stages = without(full, &["selfTest", "runProbe"]);
    let chained: BTreeSet<String> = full
        .lines()
        .filter(|line| !line.trim_start().starts_with("//"))
        .flat_map(|line| line.split("role: '").skip(1))
        .map(|rest| rest[..rest.find('\'').expect("a role closes")].to_owned())
        .collect();
    let mut found: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for line in stages
        .lines()
        .filter(|line| !line.starts_with("function roleStep("))
    {
        for call in line.split("roleStep(").skip(1) {
            let schema = call
                .split(|c: char| !(c.is_ascii_uppercase() || c == '_'))
                .find(|word| word.ends_with("_SCHEMA"))
                .unwrap_or_else(|| panic!("a `roleStep` call names its schema on its line: {call}"))
                .to_owned();
            let roles: Vec<String> = match call.strip_prefix('\'') {
                Some(named) => vec![named[..named.find('\'').expect("a role closes")].to_owned()],
                None => {
                    assert!(
                        call.starts_with("step.role,"),
                        "a role is a literal or a chain's step: {call}"
                    );
                    chained.iter().cloned().collect()
                }
            };
            for role in roles {
                found.entry(role).or_default().insert(schema.clone());
            }
        }
    }
    found
}

/// Every call that halts a stage in `region`, as the row of the stated list it names — or
/// as a fault: a halt that names no row.
fn halts_in(region: &str, rows: &BTreeSet<String>, faults: &mut Vec<String>) -> BTreeSet<String> {
    let mut named = BTreeSet::new();
    let code = code(region);
    for (at, _) in code.match_indices("alt(") {
        let opens = code[..at]
            .rfind(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
            .map_or(0, |i| i + 1);
        let name = &code[opens..at + 3];
        let line = code[..at].lines().count();
        match name {
            "recordHalt" => {
                named.insert("record".to_owned());
            }
            "pushHalt" => {
                named.insert("push".to_owned());
            }
            _ => faults.push(format!(
                "`{name}(` (line {line} of the region): once an instrument is launched a stage halts only through `haltAfter`, on a row of HALTS_AFTER"
            )),
        }
    }
    for call in code.split("haltAfter('").skip(1) {
        let row = &call[..call.find('\'').expect("a row closes")];
        if !rows.contains(row) {
            faults.push(format!("`haltAfter('{row}'`: HALTS_AFTER has no such row"));
        }
        named.insert(row.to_owned());
    }
    named
}

/// The two stretches of the script in which an instrument, or a verifier, is launched and
/// the record is not yet read back: `runTest` from its held checks on, and the lap that
/// finishes a triage from its triage on.
fn after_launch(full: &str) -> (String, String) {
    let tested = function(full, "runTest");
    let stage = &tested[tested
        .find("\n  const heldRan = []\n")
        .expect("the stage's first instrument: its held checks")..];
    let lap = function(full, "finishTriage");
    let lap = &lap[lap.find("await triagePasses(").expect("the lap's triage")..];
    (stage.to_owned(), lap.to_owned())
}

#[test]
fn t_a_field_a_stage_needs_has_its_cell_and_what_halts_after_an_instrument_is_a_stated_list() {
    let full = harness();

    // NO SCHEMA OF A STAGE GAINED A REQUIRED FIELD FOR THIS, read off the source everywhere:
    // a return that omits a required field is a call that failed (the `required` probe).
    assert!(
        full.contains("\nconst NEEDS = {\n") && full.contains("\nconst HALTS_AFTER = {\n"),
        "the script holds what a stage needs of a return (NEEDS) and what still halts once an instrument is launched (HALTS_AFTER), each as one table"
    );
    assert_eq!(
        function(&full, "haltAfter").lines().nth(1).map(str::trim),
        Some("return halt(HALTS_AFTER[row].phase, why, more)"),
        "`haltAfter` halts at the phase its row names, and does nothing else"
    );

    // WHAT STILL HALTS ONCE AN INSTRUMENT IS LAUNCHED IS A STATED LIST, read off the source:
    // in the two stretches every halt goes through `haltAfter` and names a row — a halt
    // added later without a row is red — and the helpers they call halt nothing themselves.
    let table = &full[full.find("\nconst HALTS_AFTER = {\n").expect("the table")..];
    let table = &table[..table.find("\n}\n").expect("the table closes")];
    let rows: BTreeSet<String> = table
        .lines()
        .filter_map(|line| {
            let name = line.trim_start().strip_prefix('\'')?.split('\'').next()?;
            line.contains("': { phase: '").then(|| name.to_owned())
        })
        .collect();
    let (stage, lap) = after_launch(&full);
    let mut faults = Vec::new();
    let met = halts_in(&stage, &rows, &mut faults);
    let met_by_lap = halts_in(&lap, &rows, &mut faults);
    assert_eq!(faults, Vec::<String>::new());
    assert_eq!(
        met, rows,
        "every row of HALTS_AFTER (right) is a halt the `test` stage can meet after its first instrument (left): a row no halt names is stale"
    );
    assert!(
        met_by_lap.is_subset(&rows)
            && met_by_lap.contains("triage")
            && met_by_lap.contains("record"),
        "the finishing lap halts on rows of the same list: {met_by_lap:?}"
    );
    for helper in [
        "runUnits",
        "triagePasses",
        "driveFork",
        "settleReports",
        "preflightOf",
        "hold",
    ] {
        let body = code(function(&full, helper));
        assert!(
            !body.contains("halt(") && !body.contains("Halt("),
            "`{helper}` is called once an instrument is launched, and halts nothing itself: what it could not do it returns"
        );
    }
    // A halt put back without a row, and a row nobody wrote, are each red.
    for (planted, names) in [
        (
            "\n  if (ran.length > 99) return halt('binary', 'planted')\n",
            "`halt(`",
        ),
        (
            "\n  if (ran.length > 99) return gitHalt('git', null, 'planted')\n",
            "`gitHalt(`",
        ),
        (
            "\n  if (ran.length > 99) return haltAfter('planted', 'planted')\n",
            "no such row",
        ),
    ] {
        let mut found = Vec::new();
        halts_in(&format!("{stage}{planted}"), &rows, &mut found);
        assert!(
            found.iter().any(|fault| fault.contains(names)),
            "{planted:?} after the instruments must be named ({names}); found: {found:?}"
        );
    }

    // THE WORKFLOW DOC STATES THE SAME LIST, row for row.
    let doc = fs::read_to_string(repo_root().join("implementation/stabilization-workflow.md"))
        .expect("read the workflow doc");
    let stated = doc
        .split("**What still halts a stage once an instrument is launched**")
        .nth(1)
        .expect("the workflow doc states what still halts once an instrument is launched");
    let stated: BTreeSet<String> = stated
        .lines()
        .skip_while(|line| !line.starts_with("| `"))
        .take_while(|line| line.starts_with("| `"))
        .map(|line| {
            line["| `".len()..]
                .split('`')
                .next()
                .expect("a row")
                .to_owned()
        })
        .collect();
    assert_eq!(
        stated, rows,
        "the rows the workflow doc states (left) are the rows of HALTS_AFTER (right)"
    );

    if !node_or_skip(&format!(
        "the fields a stage of {HARNESS} needs of a return were not held to its schemas"
    )) {
        return;
    }
    // WHAT A STAGE NEEDS OF A RETURN, HELD TO THE SCHEMAS (driven under node). The census is
    // of THE ROLES THAT DRIVE — the ones handed the binary, whose return is evidence about
    // the candidate: every other role's endings are rows of the stated list above, or lie
    // before any instrument (the first preflight, the scope step), or void by a rule of
    // their own (the second preflight; the cross-model pass).
    let by_role = role_schemas(&full);
    let schemas: BTreeSet<String> = by_role.values().flatten().cloned().collect();
    let read = census(&schemas);
    let driving: BTreeSet<String> = read["roles"]
        .as_object()
        .expect("the roles")
        .iter()
        .filter(|(_, role)| role["drives"] == true)
        .map(|(name, _)| name.clone())
        .collect();
    let needs = read["needs"].as_object().expect("NEEDS");
    assert_eq!(
        needs.keys().cloned().collect::<BTreeSet<_>>(),
        driving,
        "NEEDS has a row for every role that drives (right), and for no other (left)"
    );
    let mut pairs: BTreeMap<(String, String), String> = BTreeMap::new();
    let mut faults = Vec::new();
    let mut unneeded: BTreeSet<(String, String)> = BTreeSet::new();
    for role in &driving {
        let schema = match by_role.get(role).map(|set| set.iter().collect::<Vec<_>>()) {
            Some(one) if one.len() == 1 => one[0].clone(),
            other => panic!("the role `{role}` is launched under exactly one schema: {other:?}"),
        };
        let optional: BTreeSet<String> = read["optional"][&schema]
            .as_array()
            .expect("a schema's optional fields")
            .iter()
            .map(|field| field.as_str().expect("a path").to_owned())
            .collect();
        let needed: BTreeSet<String> = needs[role]
            .as_array()
            .expect("a role's rows")
            .iter()
            .map(|row| {
                let field = row["field"]
                    .as_str()
                    .expect("a row names its field")
                    .to_owned();
                let cell = row["then"]
                    .as_str()
                    .expect("a row names its cell")
                    .to_owned();
                if !CELLS.contains(&cell.as_str()) {
                    faults.push(format!(
                        "NEEDS.{role}: `{field}` has the cell `{cell}`, which is none of {CELLS:?}"
                    ));
                }
                if row["why"]
                    .as_str()
                    .map_or(0, |why| why.split_whitespace().count())
                    < 6
                {
                    faults.push(format!(
                        "NEEDS.{role}: `{field}` does not say why the stage needs it"
                    ));
                }
                pairs.insert((role.clone(), field.clone()), cell);
                field
            })
            .collect();
        // NO SCHEMA REQUIRES A NEEDED FIELD — it is one the schema describes and leaves
        // optional, so that its absence costs one item and never three runs of an agent.
        for field in &needed {
            if !optional.contains(field) {
                faults.push(format!(
                    "NEEDS.{role}: `{field}` is no optional field of {schema} — a field a stage needs of a role is one its schema describes and does NOT require: a return that omits a required field is a call that failed"
                ));
            }
        }
        for field in &optional {
            let listed = NOT_NEEDED
                .iter()
                .any(|(of, name, _)| *of == schema && name == field);
            if listed {
                unneeded.insert((schema.clone(), field.clone()));
            }
            if listed == needed.contains(field) {
                faults.push(format!(
                    "{schema}.{field} (the role `{role}`): {}",
                    if listed {
                        "it has a row of NEEDS and a row of this suite's NOT_NEEDED: one of them is wrong"
                    } else {
                        "an optional field with NO CELL — say what its absence does to what the role was launched for (a row of NEEDS, and its arm in the simulation), or why the stage needs none (a row of NOT_NEEDED)"
                    }
                ));
            }
        }
    }
    for (schema, field, why) in NOT_NEEDED {
        if !unneeded.contains(&((*schema).to_owned(), (*field).to_owned())) {
            faults.push(format!("NOT_NEEDED: `{schema}.{field}` is no optional field of a driving role's schema: the row is stale"));
        }
        if why.split_whitespace().count() < 6 {
            faults.push(format!("NOT_NEEDED: `{schema}.{field}` carries no reason"));
        }
    }
    assert_eq!(faults, Vec::<String>::new());

    // EVERY CELL HAS ITS ARM, OR IS OWED BY NAME: a cell that voids is driven absent by the
    // simulation; a cell that still halts is a row of OWED — and the stated list holds the
    // row for it exactly as long as one is owed.
    let arms: BTreeSet<(String, String)> = super::stabilize_simulation::ABSENT_ARMS
        .iter()
        .map(|(role, field)| ((*role).to_owned(), (*field).to_owned()))
        .collect();
    let owed: BTreeSet<(String, String)> = OWED
        .iter()
        .map(|(role, field, _, _)| ((*role).to_owned(), (*field).to_owned()))
        .collect();
    let of = |halts: bool| -> BTreeSet<(String, String)> {
        pairs
            .iter()
            .filter(|(_, cell)| (cell.as_str() == "halts") == halts)
            .map(|(pair, _)| pair.clone())
            .collect()
    };
    assert_eq!(
        of(false),
        arms,
        "every (role, field) whose absence voids what the role was launched for, or leaves a fact unknown (left), is driven absent by the simulation (right: its ABSENT_ARMS)"
    );
    assert_eq!(
        of(true),
        owed,
        "every (role, field) whose absence still halts the stage (left) is owed to a task by name (right: OWED) — a cell that is turned takes its row out, and gains its arm"
    );
    for (role, field, task, why) in OWED {
        assert!(
            !task.is_empty() && why.split_whitespace().count() >= 12,
            "OWED: ({role}, {field}) names its task and why this commit could not turn it"
        );
    }
    assert_eq!(
        rows.contains(OWED_HALT),
        !owed.is_empty(),
        "the row `{OWED_HALT}` of the stated list stands exactly as long as a cell is owed"
    );
    assert_eq!(
        code(function(&full, "triagePasses"))
            .matches("faults.push(")
            .count(),
        OWED.len(),
        "the verifier's faults — what halts the stage at `{OWED_HALT}` — are the owed cells, and no other"
    );
}
