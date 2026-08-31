//! The **item-block shape space**, manufactured — the one generator behind the axis
//! `{slotless, single-slot, multi-slot} × {nested, ¬nested}` (M49 Increment 1, T6; the
//! composite acceptance's arm 1, Increment 12).
//!
//! # This is a MANUFACTURED SHAPE SPACE, not a registry enumeration
//!
//! M45 and M47 established the axis-iterating pattern as *enumerate the code-side
//! registry*. **This axis deliberately departs from it, and the departure is the
//! point.** Across both embedded packs the shipped item blocks populate **two** of these
//! six shapes with the ingredients the cells need: no shipped block is multi-slot
//! **and** nested, and no shipped **multi-slot** block carries a settable field at all
//! (`roadmap.milestones` is the only multi-slot block either pack ships, and its three
//! leaves are an id-source `title` and two slots). A suite that looped the registry here
//! would drive two cells, go green, and report an axis it never reached — the failure
//! mode the complete-fix contract exists to prevent, arriving *through* the mechanism the
//! contract prescribes (`completions/artifacts/M49/settle-record.md` → *Acceptance — one
//! correction to how the axis is built*; `implementation/pinning.md` §1).
//!
//! So the shapes are **built**: [`shape_schema`] emits one doctype schema per shape, and
//! a [`crate::support::trial_corpus::FixturePack`] carries it into a real corpus. The
//! generator is the axis — a shape is a `(slots, nested)` pair, never a hand-written
//! constant — so no cell can be quietly dropped by editing one fixture.
//!
//! The claim about the shipped registry is **fenced rather than asserted**, in
//! `item_region_shape_space::the_registry_cannot_supply_this_axis`, against the loaded
//! schemas of both embedded packs.
//!
//! # Two consumers, one generator
//!
//! `item_region_shape_space.rs` drives the eighteen `shape × op` cells over the **staged**
//! copy; `flow50_acceptance.rs`'s arm 1 drives the same shapes to a **commit** and reads
//! them back through the pinned `doc show` contract. The second needs a persisted home,
//! which is the only axis [`shape_schema_at`] adds — and [`shape_schema`] is exactly
//! `shape_schema_at(shape, None)`, so the eighteen-cell suite's fixture bytes are
//! unchanged by the lift.

/// One shape of item block: how many prose slots its template declares, and whether it
/// nests a repeatable. The pair *is* the axis — a shape is computed, never a constant,
/// so a cell cannot be dropped by editing a fixture.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Shape {
    pub slots: usize,
    pub nested: bool,
}

/// The six shapes: `{slotless, single-slot, multi-slot} × {nested, ¬nested}`.
pub const SHAPES: [Shape; 6] = [
    Shape {
        slots: 0,
        nested: false,
    },
    Shape {
        slots: 0,
        nested: true,
    },
    Shape {
        slots: 1,
        nested: false,
    },
    Shape {
        slots: 1,
        nested: true,
    },
    Shape {
        slots: 2,
        nested: false,
    },
    Shape {
        slots: 2,
        nested: true,
    },
];

impl Shape {
    /// The shape's name, as the fixture-pack labels and assertion messages spell it.
    pub fn label(self) -> String {
        let slots = match self.slots {
            0 => "slotless",
            1 => "single-slot",
            _ => "multi-slot",
        };
        let nesting = if self.nested { "nested" } else { "flat" };
        format!("{slots}-{nesting}")
    }

    /// The declared slot ids of this shape's item template, in document order.
    pub fn slot_ids(self) -> &'static [&'static str] {
        &["statement", "proves"][..self.slots]
    }
}

/// The doctype schema for one shape, generated from the `(slots, nested)` pair, with no
/// home — the **transient** form.
///
/// It is written over the dev pack's `changelog` slot: a fixture pack **replaces** a
/// schema rather than registering a new doctype, so nothing outside the schema file has
/// to be manufactured too. Every shape carries the same optional `status` field — the
/// leaf every cell writes — so the only thing varying across the space is the item
/// template's own shape.
pub fn shape_schema(shape: Shape) -> String {
    shape_schema_at(shape, None)
}

/// [`shape_schema`] with an optional `location:` home — the persisted form, which is what
/// a cell driving the shape through `jigc task finalize` to a **committed** file needs.
///
/// `None` reproduces [`shape_schema`] byte for byte.
pub fn shape_schema_at(shape: Shape, location: Option<&str>) -> String {
    let mut yaml = String::from(
        "type: changelog
id-from: title
",
    );
    if let Some(location) = location {
        yaml.push_str(&format!("location: {location}\n"));
    }
    yaml.push_str(
        "description: A manufactured findings log for the item-region shape space.
usage: the item-region boundary needs item-block shapes the shipped packs do not declare.
sections:
  - id: findings
    repeatable:
      id-from: label
      block:
        - { id: label, type: string }
        - { id: status, type: string, optional: true }
",
    );
    for slot in shape.slot_ids() {
        yaml.push_str(&format!(
            "        - {{ id: {slot}, slot: {{ hint: \"The {slot}.\" }} }}\n"
        ));
    }
    if shape.nested {
        yaml.push_str(
            "        - id: notes
          repeatable:
            id-from: label
            block:
              - { id: label, type: string }
              - { id: detail, slot: { hint: \"The note.\" } }
",
        );
    }
    yaml
}

/// The workflow whose create-gate admits the fixture doctype — `creates-task: true`, so
/// `jigc start --workflow` mints the task every write addresses.
pub const FIXTURE_WORKFLOW: &str = "\
---
when: record a finding in the findings log
description: Author the findings log.
usage: a finding needs recording in the findings log.
creates-task: true
allows-create: [{type: changelog, as: findings}]
---
{{ include: step:finalize }}
";
