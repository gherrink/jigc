//! M49 Increment 11, T8 — **the permitted stdin form gets named against
//! `--from-file -`** (S-1; `design/surface-contract.md` → law 2, nothing hides).
//!
//! Measured across both shipped packs at `82d424c`: `--from-file -` appears **42×** in
//! authored pack text while `heredoc`, `<<EOF` and `<<'EOF'` appear **0×** there. A
//! blind worker under the default permission set reached for
//! `cat payload | jigc doc author … --from-file -` and for three write-a-scratch-file
//! idioms, drew eleven denials, and halted
//! (`completions/artifacts/RC-1.0-final/findings-verification.md` → S-1).
//!
//! **jigc already emits the permitted form at one producer, and the count above does not
//! say otherwise.** The `{{schema:<T>}}` payload-skeleton renderer
//! (`engine::compose` → the authoring-payload projection) composes
//! `jigc doc author <T> --from-file - … <<'EOF' … EOF` — so the 0× is a fact about
//! *authored pack text*, not about the composed surface. That producer is a
//! demonstration and is disposed **out** of the owe-set by the same predicate that
//! disposes the ones this task adds ([`is_demonstration`]), and the two sets are
//! disjoint: no step carrying a literal or catalog stdin site also renders a
//! `{{schema:<T>}}` skeleton, which is exactly why the 17 members below reached a trial
//! with the form unnamed.
//!
//! **The fact taught is about the harness, never about jigc.** jigc reads stdin either
//! way; what the four-command probe settled is that an agent harness which *statically
//! analyses* a shell command before running it can refuse the `cat … | jigc` pipeline
//! ("contains shell syntax (pipeline) that cannot be statically analyzed") while
//! permitting the heredoc attached **directly** to the `jigc` command. A step claiming
//! jigc rejects the pipeline would be a fresh law-1 lie, so the shipped sentence says
//! the harness can refuse it and that jigc itself accepts it.
//!
//! **The 42 was a lead; this suite derives the site set.** A site is a *command line*
//! that reads a payload from stdin, and it has two producers, only one of which the
//! grep found:
//!
//!   * a **literal** `jigc … --from-file - …` line in the step body (the 42), and
//!   * a **catalog** `{{cli.<id>}}` ref whose command-ref carries `--from-file -` —
//!     rendered into the composed bytes as a `Run:` line. `packs/methodology`'s
//!     `author-commit` step has **no** literal site and two catalog ones, so a
//!     grep-shaped owe-set would have missed a member outright.
//!
//! The ref predicate is [`cli::pack::cli_refs`], the fences' own, so the sweep and the
//! pack cannot disagree about what a `{{cli.<id>}}` ref is.
//!
//! Every site is then **disposed**: named by its step's own statement, or OUT because
//! the line *is* the naming (the demonstration line carries `<<'EOF'` itself).
//! [`the_site_set_is_not_vacuous`] fences the derivation — it asserts the shape of the
//! shipped packs, so a predicate that silently stops matching reddens instead of
//! passing over nothing.
//!
//! Three arms carry the claim, in rising strength:
//!
//!   1. [`every_site_bearing_step_names_the_permitted_form`] — the source sweep: every
//!      member states the sentence and demonstrates the form, **above** its own sites.
//!   2. [`every_composed_workflow_names_the_form_above_its_sites`] — **the emitted
//!      bytes**, through the real binary: each workflow of both packs is previewed and
//!      every site in the composed text must have the statement above it. This arm sees
//!      the catalog-rendered `Run:` sites the source arm cannot.
//!   3. [`the_named_heredoc_form_runs_verbatim_through_a_real_shell`] — the emitted
//!      demonstration is **extracted from a real `jigc start` and executed by `sh`,
//!      byte for byte**, and the write it claims to make is read back. A form the packs
//!      name and no test runs would be law 1's next entry, not its repair.
//!
//! [`every_member_states_the_form_in_the_same_words`] holds the sentence itself to one
//! wording across all 17 members: one fact stated 17 ways is how it stays true in one
//! file and drifts in the next, and a per-step paraphrase is the likeliest way this
//! repair rots.
//!
//! **A heredoc-opening line is a demonstration, not a solicit** — a distinction three
//! sibling suites now share, because they count or execute the composed writes and a
//! demonstration is neither a second instruction nor bytes to pipe prose into
//! (`commit_solicit_axis.rs` → `emitted_writes`, `flow21_measured_run.rs` →
//! `extract_cmd`). The distinction is structural: the line carries its own payload and
//! its closing `EOF`.

use crate::support;

use std::collections::BTreeMap;
use std::process::Command;

use cli::pack::EmbeddedPack;
use engine::packsource::{PackResourceKind, PackSource};

use support::trial_corpus::{State, TrialCorpus};

/// The named facts the statement must carry, in [`normalize`]d form: the permitted
/// form, the denied one, and **whose** refusal it is. The third is load-bearing —
/// without it the sentence reads as a claim about jigc, which accepts both.
const STATEMENT_TOKENS: [&str; 3] = [
    "attach it as a heredoc directly to the `jigc` command",
    "never as a `cat payload | jigc",
    "an agent harness that statically analyses shell commands can refuse to run",
];

/// The one-line anchor the ordering checks key on — the statement's opening clause,
/// which is authored unwrapped so a line index exists for it.
const STATEMENT_ANCHOR: &str = "reads its payload from stdin";

/// The heredoc the demonstration opens (quoted, so the payload is literal) and the
/// line that closes it.
const HEREDOC_OPEN: &str = " <<'EOF'";
const HEREDOC_CLOSE: &str = "EOF";

/// The two embedded packs in precedence order, built the CWD-free way.
fn embedded_packs() -> Vec<(&'static str, EmbeddedPack)> {
    vec![
        ("dev", EmbeddedPack::new()),
        ("methodology", EmbeddedPack::methodology()),
    ]
}

/// Whitespace runs collapsed, ASCII case folded — the comparison view the pack's own
/// named-fact fence uses, so a fact that wraps across a hard-wrapped line still matches.
fn normalize(text: &str) -> String {
    let mut out = String::new();
    let mut pending = false;
    for ch in text.chars() {
        if ch.is_whitespace() {
            pending = true;
            continue;
        }
        if pending && !out.is_empty() {
            out.push(' ');
        }
        pending = false;
        out.push(ch.to_ascii_lowercase());
    }
    out
}

/// Whether a rendered or authored line is a **command line that reads stdin** — a
/// `jigc` invocation (bare, or inside a composed `Run:` line) carrying `--from-file -`.
/// Prose that merely mentions the flag is not a site; the statement's own first line
/// mentions it and must not count as one.
fn is_site_line(line: &str) -> bool {
    let trimmed = line.trim();
    let command = trimmed
        .strip_prefix("Run: `")
        .map(|rest| rest.trim_end_matches('`'))
        .unwrap_or(trimmed);
    command.starts_with("jigc ") && command.contains("--from-file -")
}

/// Whether a site line is the **demonstration** — the line that names the permitted
/// form by carrying it. These are the disposed-out half of the site set: the naming
/// cannot itself be owed a naming.
fn is_demonstration(line: &str) -> bool {
    line.trim_end().ends_with(HEREDOC_OPEN)
}

/// One site-bearing step: the member unit of the owe-set.
struct Member {
    pack: &'static str,
    id: String,
    body: String,
    /// Literal `jigc … --from-file - …` lines in the body, excluding the
    /// demonstration — the sites this step owes a naming for, with their line index.
    literal_sites: Vec<(usize, String)>,
    /// `{{cli.<id>}}` refs whose command-ref reads stdin — rendered as `Run:` sites in
    /// the composed bytes, invisible to a grep over the step source.
    catalog_sites: Vec<String>,
}

impl Member {
    fn label(&self) -> String {
        format!("{}:{}", self.pack, self.id)
    }
}

/// The catalog ids of `owner` whose command-ref reads its payload from stdin
/// (`--from-file` followed by the `-` literal).
fn stdin_command_ids(owner: &dyn PackSource) -> std::collections::BTreeSet<String> {
    let Some(catalog) = owner
        .read(
            PackResourceKind::Config,
            &engine::packsource::ResourceId::from("commands"),
        )
        .ok()
        .and_then(|bytes| engine::compose::load_command_catalog(&bytes).ok())
    else {
        return std::collections::BTreeSet::new();
    };
    catalog
        .commands
        .iter()
        .filter(|(_, command)| {
            let literals: Vec<&str> = command
                .args
                .iter()
                .filter_map(|arg| match arg {
                    engine::compose::CommandArg::Literal { literal } => Some(literal.as_str()),
                    _ => None,
                })
                .collect();
            literals.windows(2).any(|pair| pair == ["--from-file", "-"])
        })
        .map(|(id, _)| id.clone())
        .collect()
}

/// The site set, derived from both packs' whole step trees.
fn site_set() -> Vec<Member> {
    let mut out = Vec::new();
    for (pack_name, pack) in embedded_packs() {
        let stdin_refs = stdin_command_ids(&pack);
        for id in pack.list(PackResourceKind::Steps) {
            let bytes = pack
                .read(PackResourceKind::Steps, &id)
                .expect("a listed step reads back");
            let source = String::from_utf8(bytes).expect("pack resources are UTF-8");
            let def = engine::compose::load_step_def(id.as_str(), source.as_bytes())
                .expect("a shipped step loads");
            let literal_sites: Vec<(usize, String)> = def
                .body
                .lines()
                .enumerate()
                .filter(|(_, line)| is_site_line(line) && !is_demonstration(line))
                .map(|(index, line)| (index, line.trim().to_owned()))
                .collect();
            let catalog_sites: Vec<String> = cli::pack::cli_refs(&def.body)
                .into_iter()
                .filter(|reference| stdin_refs.contains(reference))
                .collect();
            if literal_sites.is_empty() && catalog_sites.is_empty() {
                continue;
            }
            out.push(Member {
                pack: pack_name,
                id: id.as_str().to_owned(),
                literal_sites,
                catalog_sites,
                body: def.body,
            });
        }
    }
    out
}

/// The demonstration block a body carries: the `<<'EOF'` line, its payload, and the
/// closing `EOF`, as `(line index, command, payload)`.
fn demonstration_blocks(text: &str) -> Vec<(usize, String, String)> {
    let lines: Vec<&str> = text.lines().collect();
    let mut out = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        if !is_site_line(line) || !is_demonstration(line) {
            continue;
        }
        let command = line
            .trim()
            .strip_suffix(HEREDOC_OPEN.trim_start())
            .expect("a demonstration ends with the heredoc opener")
            .trim_end()
            .to_owned();
        let mut payload = Vec::new();
        let mut closed = false;
        for follower in lines.iter().skip(index + 1) {
            if follower.trim_end() == HEREDOC_CLOSE {
                closed = true;
                break;
            }
            payload.push(*follower);
        }
        assert!(
            closed && !payload.is_empty(),
            "the demonstration at line {index} opens a heredoc that never closes on a \
             lone `EOF` line, or closes it with no payload — as printed it would hang \
             the agent that copies it:\n{line}",
        );
        out.push((index, command, payload.join("\n")));
    }
    out
}

/// The derivation's own fence — it asserts the **shape of the shipped packs**, not the
/// fix, so it holds before and after and reddens when a predicate stops matching
/// (`implementation/pinning.md` §1 — a sweep that can pass vacuously pins nothing).
#[test]
fn the_site_set_is_not_vacuous() {
    let members = site_set();
    let literal: usize = members.iter().map(|m| m.literal_sites.len()).sum();
    let catalog: usize = members.iter().map(|m| m.catalog_sites.len()).sum();
    assert!(
        members.len() >= 16 && literal >= 40,
        "the shipped packs carry ~42 literal stdin sites across ~17 steps — {} members \
         / {literal} literal sites is a derivation that broke, not a pack that shrank",
        members.len(),
    );
    assert!(
        catalog >= 1,
        "no member carries a catalog stdin site — the `{{{{cli.<id>}}}}` half of the \
         derivation has stopped matching, and with it the only member \
         (`methodology:author-commit`) that a grep over `--from-file -` never sees",
    );
    assert!(
        members
            .iter()
            .any(|member| member.literal_sites.is_empty() && !member.catalog_sites.is_empty()),
        "the catalog-only partition is empty — every member is reachable by grep again, \
         so the derivation is no longer buying anything over the reported 42",
    );
    let overlapping: Vec<String> = members
        .iter()
        .filter(|member| !cli::pack::schema_refs(&member.body).is_empty())
        .map(Member::label)
        .collect();
    assert!(
        overlapping.is_empty(),
        "this suite's header states that the site-bearing steps and the
         `{{{{schema:<T>}}}}` steps are disjoint — which is why one set reached a trial \
         with the form unnamed while the other has emitted a heredoc since M43. That is \
         no longer true of {overlapping:?}: revise the header, and decide whether the \
         projection's own demonstration now discharges those members' statement",
    );
}

/// **The source sweep.** Every site-bearing step states the sentence and demonstrates
/// the form, and the demonstration sits **above** the step's own sites — a naming an
/// agent reaches only after it has already run the site is not a naming.
#[test]
fn every_site_bearing_step_names_the_permitted_form() {
    let mut gaps = Vec::new();
    for member in site_set() {
        let body = normalize(&member.body);
        for token in STATEMENT_TOKENS {
            if !body.contains(token) {
                gaps.push(format!(
                    "step `{}` carries {} stdin site(s) and never says \"{token}\"",
                    member.label(),
                    member.literal_sites.len() + member.catalog_sites.len(),
                ));
            }
        }
        let blocks = demonstration_blocks(&member.body);
        let Some((demo_index, command, _)) = blocks.first() else {
            gaps.push(format!(
                "step `{}` never demonstrates the permitted form — no \
                 `jigc … --from-file - …{HEREDOC_OPEN}` line",
                member.label(),
            ));
            continue;
        };
        assert!(
            command.contains("--from-file -") && command.starts_with("jigc doc "),
            "the demonstration in `{}` must be a real `jigc doc` stdin write, got: {command}",
            member.label(),
        );
        let anchor = member
            .body
            .lines()
            .position(|line| line.contains(STATEMENT_ANCHOR));
        match anchor {
            None => gaps.push(format!(
                "step `{}` never carries the statement anchor \"{STATEMENT_ANCHOR}\" on a \
                 single line — the ordering checks key on it, so a rewrap must be made \
                 visible rather than silently unchecking the placement",
                member.label(),
            )),
            Some(anchor_index) => {
                if let Some((site_index, site)) = member.literal_sites.first()
                    && (*site_index < anchor_index || *site_index < *demo_index)
                {
                    gaps.push(format!(
                        "step `{}` names the permitted form BELOW its first site — the \
                         agent meets `{site}` before it is told how to feed it",
                        member.label(),
                    ));
                }
            }
        }
    }
    assert!(
        gaps.is_empty(),
        "a step prints `--from-file -` and never names a form that can supply it. The \
         permitted form is the heredoc attached directly to the `jigc` command; the \
         `cat … | jigc` pipeline an agent reaches for first is what a statically \
         analysing harness refuses (RC-1.0-final S-1: eleven denials, then the worker \
         halted). design/surface-contract.md → law 2, nothing hides:\n  {}",
        gaps.join("\n  "),
    );
}

/// **The emitted bytes.** Every workflow of both packs, composed through the real
/// binary: each site in the composed text must have the statement above it. This arm
/// sees what the source arm cannot — the catalog-rendered `Run:` sites, and whatever
/// step ordering composition actually produces.
#[test]
fn every_composed_workflow_names_the_form_above_its_sites() {
    let corpus = TrialCorpus::build(State::Fresh);
    let mut doors = support::composed::DoorFixtures::new(&corpus);
    let mut gaps = Vec::new();
    let mut asserted = 0usize;
    for (pack_name, pack) in embedded_packs() {
        for id in pack.list(PackResourceKind::Workflows) {
            let bytes = pack
                .read(PackResourceKind::Workflows, &id)
                .expect("a listed workflow reads back");
            let def = engine::compose::load_workflow_def(&bytes)
                .unwrap_or_else(|f| panic!("`{}` must load: {}", id.as_str(), f.message));
            let composed = doors.compose(id.as_str(), &def);
            let lines: Vec<&str> = composed.lines().collect();
            let sites: Vec<(usize, &str)> = lines
                .iter()
                .enumerate()
                .filter(|(_, line)| is_site_line(line) && !is_demonstration(line))
                .map(|(index, line)| (index, *line))
                .collect();
            if sites.is_empty() {
                continue;
            }
            let anchors: Vec<usize> = lines
                .iter()
                .enumerate()
                .filter(|(_, line)| line.contains(STATEMENT_ANCHOR))
                .map(|(index, _)| index)
                .collect();
            for (index, site) in sites {
                asserted += 1;
                if !anchors.iter().any(|anchor| *anchor < index) {
                    gaps.push(format!(
                        "the composed `{pack_name}:{}` prints `{}` with no naming of the \
                         permitted stdin form above it",
                        id.as_str(),
                        site.trim(),
                    ));
                }
            }
        }
    }
    assert!(asserted > 0, "the composed sweep asserted nothing");
    assert!(
        gaps.is_empty(),
        "the COMPOSED bytes — what an agent actually reads — must name a form that can \
         feed `--from-file -` before the site that needs it:\n  {}",
        gaps.join("\n  "),
    );
}

/// **The named form is run, not just printed.** The demonstration is extracted from a
/// real `jigc start` composition and handed to `sh` **verbatim** — the emitted bytes are
/// the contract, and a form the packs teach that no test executes is a claim, not a
/// fact. Only fully-resolved demonstrations are executable (one carrying `<slug>` is a
/// placeholder the agent fills), and the executable set is derived rather than named.
#[test]
fn the_named_heredoc_form_runs_verbatim_through_a_real_shell() {
    let corpus = TrialCorpus::build(State::Fresh);
    let composed = corpus.jigc_ok(&["start", "--workflow", "single-task", "name the stdin form"]);
    let task = composed
        .lines()
        .find_map(|line| line.trim().strip_prefix("task minted: "))
        .expect("start prints the id it minted")
        .to_owned();

    let bin_dir = std::path::Path::new(env!("CARGO_BIN_EXE_jigc"))
        .parent()
        .expect("the test binary's jigc has a parent directory")
        .to_owned();
    let path = format!(
        "{}:{}",
        bin_dir.display(),
        std::env::var("PATH").unwrap_or_default()
    );

    let mut ran = 0usize;
    for (_, command, payload) in demonstration_blocks(&composed) {
        if command.contains('<') || !command.starts_with("jigc doc set-slot ") {
            // Two exclusions, both structural. An **address** placeholder (`<slug>`,
            // `<group-addr>`) is a question the step leaves to the agent, so the line
            // is printed to be completed rather than copied. And a **batch** payload
            // (`jigc doc author …`, the `{{schema:<T>}}` projection's own skeleton
            // among them) has a *grammar* to satisfy, so a printed template is not
            // runnable bytes. A slot payload has none: inside `<<'EOF'` every byte is
            // literal, so the demonstration runs exactly as printed — placeholder
            // prose included, which is what lands and what is read back.
            continue;
        }
        let script = format!("{command}{HEREDOC_OPEN}\n{payload}\n{HEREDOC_CLOSE}\n");
        let out = Command::new("sh")
            .arg("-c")
            .arg(&script)
            .current_dir(corpus.repo())
            .env("HOME", corpus.home())
            .env("PATH", &path)
            .env_remove("JIGC_PACK_DIR")
            .output()
            .expect("spawn sh");
        assert!(
            out.status.success(),
            "the composed demonstration does not run as printed ({}):\n{script}\n\
             --- stdout ---\n{}\n--- stderr ---\n{}",
            out.status,
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
        let address = command
            .split_whitespace()
            .nth(3)
            .expect("`jigc doc set-slot <address> …`");
        let shown = corpus.jigc_ok(&["doc", "show", address, "--task", &task]);
        assert!(
            shown.contains(payload.trim()),
            "the demonstration ran but its payload never reached `{address}` — the form \
             the packs name must actually perform the write they name it for; got:\n{shown}",
        );
        ran += 1;
    }
    assert!(
        ran > 0,
        "no composed demonstration was fully resolved, so the permitted form was \
         printed and never executed — the arm that makes this a fact rather than a \
         claim asserted nothing",
    );
}

/// Both packs' statements are the **same** sentence. A per-step paraphrase is how a
/// fact stays true in one file and drifts in the next, and this one rides 17 steps.
#[test]
fn every_member_states_the_form_in_the_same_words() {
    let mut wordings: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for member in site_set() {
        let normalized = normalize(&member.body);
        let Some(start) = normalized.find(&normalize(STATEMENT_ANCHOR)) else {
            continue;
        };
        let sentence = normalized[start..]
            .split(':')
            .next()
            .expect("a split yields one part")
            .to_owned();
        wordings.entry(sentence).or_default().push(member.label());
    }
    assert_eq!(
        wordings.len(),
        1,
        "the permitted-form sentence is worded {} different ways across the packs — one \
         fact, one wording: {wordings:#?}",
        wordings.len(),
    );
}
