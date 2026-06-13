# decisions-log

## Entries

### decided-task is a creates-task:true dev-task variant binding allows-create:[decisions-log] + an author-decision step  {#decided-task-is-a-creates-tasktrue-dev-task}

Decision-recording needs a task plus a promoting finalize, which the existing increment workflow (creates-task:false orchestration shell) cannot provide and dev-task lacks (no allows-create, transient commit only). So the lightweight path is a dev-task-shaped variant that adds one author-decision step bound to the decisions-log singleton via allows-create — reusing the idempotent create-log and per-entry add-item/set-slot machinery rather than inventing new verbs. (Seeded out-of-band edit for the M17 reconciliation absorb check.)

<!-- fields -->
- date: 2026-06-13

### decided-task is selectable:false (off-router), invoked via --workflow  {#decided-task-is-selectablefalse-off-router}

The selectable catalog deliberately presents exactly one model-free default (dev-task) until the router-flip workflow is built (CLAUDE.md MVP scope: flip default-workflow to a router only when >=2 selectable work-workflows exist). Making decided-task selectable now would add a second catalog entry and break that scope-honesty invariant (guarded by methodology_increment_off_router) without the router to disambiguate. So decided-task follows the planning/completion/record-dogfood authoring-spine pattern: selectable:false, reached via --workflow decided-task.

<!-- fields -->
- date: 2026-06-13
