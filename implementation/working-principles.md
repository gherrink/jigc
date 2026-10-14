# Working principles

Rules of judgment the human has set that hold across every workflow — design, dev, increment, milestone, stabilization — and that no single workflow doc owns. Each is one section: the rule, why it holds, how it is applied, and the `DECISIONS.md` entry that set it. A rule that belongs to one workflow lives in that workflow's doc, never here; a rule here is pointed at, never restated.

## When a doc and the robust behaviour disagree, the behaviour wins and the doc changes

**The rule.** Where a question is *which reading of the docs does the product (or a test of it) answer to* — a doc is silent, says two things, or says something that a sibling command or the project's invariants contradict — the answer is the behaviour that fits the project in general best: the robust, future-proof one. If that makes a sentence of a design doc wrong, the sentence is recorded as an inconsistency and changed, together with the product. Readings are not chosen so that every sentence already written stays true.

**Why.** The design docs were written one milestone at a time, and a sentence often records a local fix: true when written, narrower than the rule it sits beside. Choosing the reading that keeps it true freezes the local fix into a rule and loses the project's general invariants — writes are transactional, a refusal changes nothing, one command behaves as its siblings do ([VISION.md](../VISION.md), [CLAUDE.md](../CLAUDE.md) → Architectural invariants).

**How it is applied.** A question of reading put to the human leads with what the product *should* do by the project's general principles — the invariants, the exit rule, how the sibling commands behave — names the doc sentence that would change and the product change that follows (a finding, routed like any other), and only then lists the readings. A test or table built before the ruling follows the stricter reading and marks the cell, as it does today.

*Set 2026-10-14* ([DECISIONS.md](../DECISIONS.md) → *2026-10-14 — When a doc and the robust behaviour disagree, the behaviour wins*); the case that raised it: `jigc setup` refusing after its first write — the design stages the files it wrote, and the rule chose a rollback that leaves the repository as it was.
