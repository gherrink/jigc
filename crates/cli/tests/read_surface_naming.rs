//! M49 Increment 11, T7 — **the pack names the two read surfaces it never named**
//! (B1; `design/doc-read-surface.md` → the four read surfaces;
//! `design/surface-contract.md` → law 2, nothing hides).
//!
//! Measured across both shipped packs at `f0cda4b`: `jigc doc show` named **35×**,
//! `jigc doc schema` **0×**, `jigc doc list` **0×**. The producer of that asymmetry
//! is named and is jigc's own: M48's read-back fence
//! (`cli::pack` → `assert_staged_read_back_stated`) routes at *"name the staged
//! read-back … `jigc doc show <addr> --task {{task.id}}`"*, so every soliciting step
//! learned exactly the one verb the fence named. Two of the four shipped read
//! surfaces were reachable only by an agent that already knew they existed — the
//! discoverability lens that has landed six consecutive trials.
//!
//! **The owe-set is the fence's own**, not a second derivation: both call
//! [`cli::pack::solicits_managed_doc_write`] over the same
//! [`cli::pack::doc_write_command_ids`] set, so the sweep and the fence cannot
//! disagree about which steps are in. Every member is then **disposed** against each
//! of the two surfaces — named, or OUT on a *structural* signal the step already
//! renders, never a hand-maintained exclusion list:
//!
//!   * **`jigc doc schema <T>`** — the shape read. Owed by a soliciting step, OUT
//!     when the step carries `{{schema:<T>}}`: that projection *renders* the schema
//!     inline, so the read it stands for is already answered in place. (This is the
//!     guard against the failure mode of this task — blanket-sprinkling the verb
//!     across 31 steps would have been noise, and 14 of them genuinely owe nothing.)
//!   * **`jigc doc list <T> --task <id>`** — the index read. Owed by a soliciting
//!     step whose staged read-back address carries an unresolved `<slug>` placeholder,
//!     OUT when that address is printed in full (`commit:{{task.id}}`,
//!     `changelog:changelog`, …). At 17 of 31 members the read-back sentence the M48
//!     fence forced onto the step is **unrunnable as printed** — the agent is told to
//!     read back an address the step never says how to obtain — and `doc list --task`,
//!     shipped at M48 for exactly that question, was named nowhere.
//!
//! Both dispositions are derived from the step's own bytes, so a step added later is
//! swept by construction. [`the_derivation_is_not_vacuous`] fences the derivation
//! itself: every partition must have members, so a predicate that silently starts
//! matching nothing reddens instead of passing.
//!
//! **Asserted on the COMPOSED bytes through the real binary**
//! ([`every_composed_workflow_names_the_read_surfaces_its_steps_owe`]): each workflow
//! of both packs is previewed with `jigc workflow <id> --preview` and must carry the
//! statements its own included steps owe. Step ids resolve first-wins across the pack
//! precedence order, the way composition resolves them, so a step shadowed by a
//! same-id sibling is not asserted against output that never renders it.
//!
//! The **installed guide artifact** — `.claude/skills/jigc/SKILL.md`, whose bytes are
//! `QUICKSTART.md` + `MIGRATING.md` through `setup::guide_body` — is swept in
//! [`the_installed_guide_artifact_names_both_read_surfaces`], read off disk after a
//! real `jigc setup` rather than from the source constants.

use crate::support;

use std::collections::{BTreeMap, BTreeSet};

use cli::pack::EmbeddedPack;
use engine::packsource::{PackResourceKind, PackSource};

use support::trial_corpus::{State, TrialCorpus};

/// The Claude Code profile's declared guide target — the same premise
/// `adapter_artifact.rs` states; the profile is the authority and a change there
/// reddens both.
const GUIDE_PATH: &str = ".claude/skills/jigc/SKILL.md";

/// The two embedded packs in precedence order (dev wins), built the CWD-free way —
/// never `make_pack()`, which resolves against the process CWD.
fn embedded_packs() -> Vec<(&'static str, EmbeddedPack)> {
    vec![
        ("dev", EmbeddedPack::new()),
        ("methodology", EmbeddedPack::methodology()),
    ]
}

/// One member of the derived owe-set: a step that solicits a managed-doc write,
/// carrying the two structural signals its dispositions key on.
struct Member {
    pack: &'static str,
    id: String,
    body: String,
    /// The doctype of the step's staged read-back address (`adr` in
    /// `jigc doc show adr:<slug> --task …`) — the doctype it solicits a write of.
    doctype: String,
    /// The read-back address carries an unresolved `<…>` placeholder, so the address
    /// to read by is a question the step leaves open.
    address_open: bool,
    /// The step renders `{{schema:<T>}}` — the schema is already in front of the
    /// agent, inline.
    renders_schema: bool,
}

impl Member {
    fn label(&self) -> String {
        format!("{}:{}", self.pack, self.id)
    }

    /// The statement this member owes for the shape read, if any.
    fn owed_schema_read(&self) -> Option<String> {
        (!self.renders_schema).then(|| format!("jigc doc schema {}", self.doctype))
    }

    /// The statement this member owes for the index read, if any.
    fn owed_index_read(&self) -> Option<String> {
        self.address_open
            .then(|| format!("jigc doc list {} --task", self.doctype))
    }
}

/// The staged read-back command line a soliciting step prints, as
/// `(doctype, address-tail)` — parsed off the shipped line shape
/// `jigc doc show <type>:<tail> --task {{task.id}}`.
fn read_back_addresses(body: &str) -> Vec<(String, String)> {
    body.lines()
        .filter_map(|line| {
            let rest = line.trim().strip_prefix("jigc doc show ")?;
            let address = rest.split_whitespace().next()?;
            let (ty, tail) = address.split_once(':')?;
            Some((ty.to_owned(), tail.to_owned()))
        })
        .collect()
}

/// The owe-set, derived with **the read-back fence's own predicate** over both packs'
/// whole step trees.
fn owe_set() -> Vec<Member> {
    let mut out = Vec::new();
    for (pack_name, pack) in embedded_packs() {
        let write_refs = cli::pack::doc_write_command_ids(&pack);
        for id in pack.list(PackResourceKind::Steps) {
            let bytes = pack
                .read(PackResourceKind::Steps, &id)
                .expect("a listed step reads back");
            let source = String::from_utf8(bytes).expect("pack resources are UTF-8");
            let def = engine::compose::load_step_def(id.as_str(), source.as_bytes())
                .expect("a shipped step loads");
            if !cli::pack::solicits_managed_doc_write(&def.body, &write_refs) {
                continue;
            }
            let addresses = read_back_addresses(&def.body);
            assert_eq!(
                addresses.len(),
                1,
                "step `{pack_name}:{}` solicits a managed-doc write, so the M48 fence \
                 makes it print exactly one `jigc doc show <addr> --task <id>` line — \
                 this derivation keys both dispositions on that line and found {}. \
                 Teach the parser the new shape rather than letting a member fall out \
                 of the sweep silently.",
                id.as_str(),
                addresses.len(),
            );
            let (doctype, tail) = addresses.into_iter().next().expect("checked above");
            out.push(Member {
                pack: pack_name,
                id: id.as_str().to_owned(),
                doctype,
                address_open: tail.contains('<'),
                renders_schema: !cli::pack::schema_refs(&def.body).is_empty(),
                body: def.body,
            });
        }
    }
    out
}

/// The derivation's own fence. Every partition it splits on must be populated: a
/// predicate that quietly stops matching, or a signal that becomes universal, would
/// otherwise turn the whole sweep into a green over nothing (`implementation/pinning.md`
/// §1 — a sweep that can pass vacuously pins nothing).
#[test]
fn the_derivation_is_not_vacuous() {
    let members = owe_set();
    assert!(
        members.len() >= 20,
        "the read-back fence's owe-set spans both packs' author steps — {} members is \
         a derivation that broke, not a pack that shrank",
        members.len(),
    );
    let mut partitions: BTreeMap<&str, Vec<String>> = BTreeMap::new();
    for member in &members {
        let bucket = match (member.renders_schema, member.address_open) {
            (false, _) => "owes-schema-read",
            (true, _) => "out-renders-schema-inline",
        };
        partitions.entry(bucket).or_default().push(member.label());
        let bucket = if member.address_open {
            "owes-index-read"
        } else {
            "out-address-printed-in-full"
        };
        partitions.entry(bucket).or_default().push(member.label());
    }
    for bucket in [
        "owes-schema-read",
        "out-renders-schema-inline",
        "owes-index-read",
        "out-address-printed-in-full",
    ] {
        assert!(
            partitions.get(bucket).is_some_and(|m| !m.is_empty()),
            "partition `{bucket}` is empty — the disposition it stands for is no longer \
             being exercised; partitions: {partitions:?}",
        );
    }
}

/// **Every member disposed, both surfaces.** A step that solicits a managed-doc write
/// names the shape read unless it renders the schema inline, and names the index read
/// unless the address it tells the agent to read back is already printed in full.
#[test]
fn every_soliciting_step_disposes_both_read_surfaces() {
    let mut gaps = Vec::new();
    for member in owe_set() {
        for (surface, owed) in [
            ("the shape read", member.owed_schema_read()),
            ("the index read", member.owed_index_read()),
        ] {
            let Some(statement) = owed else { continue };
            if !member.body.contains(&statement) {
                gaps.push(format!(
                    "step `{}` owes {surface} and never says `{statement}`",
                    member.label(),
                ));
            }
        }
    }
    assert!(
        gaps.is_empty(),
        "a step solicits a managed-doc write but leaves a read surface unnamed — the \
         agent is told how to write and how to read the result back, and never how to \
         learn the shape it must fill (`jigc doc schema <T>`) or how to obtain the \
         address the read-back takes (`jigc doc list <T> --task <id>`). Two of the four \
         shipped read surfaces are then reachable only by an agent that already knew \
         they existed (design/doc-read-surface.md → the four read surfaces):\n  {}",
        gaps.join("\n  "),
    );
}

/// The composed half — **the emitted bytes an agent actually reads**. Every workflow
/// of both packs is previewed through the real binary, and must carry the statements
/// its own included steps owe.
#[test]
fn every_composed_workflow_names_the_read_surfaces_its_steps_owe() {
    // Step ids resolve **pack-locally**, the way composition resolves them
    // (`design/multi-pack.md` → Pack-local body-reference resolution: a loser-pack
    // workflow composes its own steps, never the precedence-winner's divergent ones).
    // Both packs ship an `implement` step and only dev's solicits a write; keyed by id
    // alone, a methodology workflow would be asserted against text it never renders.
    let mut resolved: BTreeMap<(&str, String), Member> = BTreeMap::new();
    for member in owe_set() {
        resolved.insert((member.pack, member.id.clone()), member);
    }

    let corpus = TrialCorpus::build(State::Fresh);
    let mut gaps = Vec::new();
    let mut asserted = 0usize;
    for (pack_name, pack) in embedded_packs() {
        for id in pack.list(PackResourceKind::Workflows) {
            let bytes = pack
                .read(PackResourceKind::Workflows, &id)
                .expect("a listed workflow reads back");
            let source = String::from_utf8(bytes).expect("pack resources are UTF-8");
            let owed: BTreeSet<String> = included_step_ids(&source)
                .iter()
                .filter_map(|step| resolve_step(&resolved, pack_name, step))
                .flat_map(|member| {
                    member
                        .owed_schema_read()
                        .into_iter()
                        .chain(member.owed_index_read())
                })
                .collect();
            if owed.is_empty() {
                continue;
            }
            let composed = corpus.jigc_ok(&["workflow", id.as_str(), "--preview"]);
            for statement in owed {
                asserted += 1;
                if !composed.contains(&statement) {
                    gaps.push(format!(
                        "the composed `{pack_name}:{}` preview never says `{statement}`",
                        id.as_str(),
                    ));
                }
            }
        }
    }
    assert!(asserted > 0, "the composed sweep asserted nothing");
    assert!(
        gaps.is_empty(),
        "the COMPOSED bytes — what an agent reads — must carry the read surfaces the \
         workflow's own steps owe:\n  {}",
        gaps.join("\n  "),
    );
}

/// Resolve an included step id the way composition does: the workflow's **own** pack
/// first, then the remaining packs in precedence order. A step no pack's owe-set holds
/// is `None` — it solicits nothing and owes nothing.
fn resolve_step<'a>(
    resolved: &'a BTreeMap<(&'static str, String), Member>,
    workflow_pack: &'static str,
    step: &str,
) -> Option<&'a Member> {
    if owning_pack_has_step(workflow_pack, step) {
        return resolved.get(&(workflow_pack, step.to_owned()));
    }
    embedded_packs()
        .into_iter()
        .find_map(|(name, _)| resolved.get(&(name, step.to_owned())))
}

/// Whether `pack` itself ships the step — the pack-local half of the resolution above,
/// read from the pack rather than inferred from the owe-set (a step the pack ships but
/// that solicits nothing must still shadow a same-id sibling).
fn owning_pack_has_step(pack: &str, step: &str) -> bool {
    embedded_packs()
        .into_iter()
        .find(|(name, _)| *name == pack)
        .is_some_and(|(_, source)| {
            source
                .read(
                    PackResourceKind::Steps,
                    &engine::packsource::ResourceId::from(step),
                )
                .is_ok()
        })
}

/// Every `{{ include: step:<id> }}` of a workflow body, in order.
fn included_step_ids(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = body;
    while let Some(open) = rest.find("{{") {
        rest = &rest[open + 2..];
        let Some(close) = rest.find("}}") else { break };
        let inner = rest[..close].trim();
        if let Some(tail) = inner.strip_prefix("include:")
            && let Some(step) = tail.trim().strip_prefix("step:")
        {
            out.push(step.trim().to_owned());
        }
        rest = &rest[close + 2..];
    }
    out
}

/// The **installed** guide artifact — the version-stamped file `jigc setup` writes,
/// read off disk rather than from the source constants — names both surfaces. An
/// adopter's only version-matched copy of the guides said `jigc doc` exactly once
/// (`doc author`, in the migration notes) and named neither read.
#[test]
fn the_installed_guide_artifact_names_both_read_surfaces() {
    let corpus = TrialCorpus::build(State::Fresh);
    let installed = support::trial_corpus::read(&corpus.repo(), GUIDE_PATH);
    for statement in ["jigc doc schema", "jigc doc list"] {
        assert!(
            installed.contains(statement),
            "the installed guide artifact (`{GUIDE_PATH}`) never names `{statement}` — \
             it is the adopter's only version-matched copy of the guides, so a read \
             surface it omits is a surface an adopter meets only by accident; got:\n{installed}",
        );
    }
}
