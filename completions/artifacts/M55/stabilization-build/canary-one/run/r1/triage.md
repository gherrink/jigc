# canary-one — round 1, the triage record

One row per finding this round triaged: whether its door is inside the round's test set (computed from `scope.md`, never supplied), triage's grade, the verifier's verdict, whether it is a regression, and the disposition its ledger row carried when the round recorded it. Written by `dev/stabilize-record` and never by hand.

| key | inside | triage | verdict | regression | found with | fork |
|---|---|---|---|---|---|---|
| `canary-seeded-claim` | inside | breaks | refuted | - | open | - |
| `r1-doc-list-unreadable-entry-fails-listing` | inside | unclear | refuted | - | open | - |
| `r1-doc-list-staged-arm-unreadable-entry` | inside | unclear | refuted | - | open | - |
| `r1-read-verb-writes-invocation-log-unnamed` | inside | no-break | - | - | open | - |
| `r1-invocation-log-write-follows-link` | inside | needs-bound | - | - | open | - |
| `r1-doc-list-served-from-no-checkout-home` | inside | no-break | - | - | open | - |
| `r1-setup-installs-outside-work-tree` | outside: excluded | unclear | confirmed | no | open | contested: robust-now, differs |
| `r1-read-verb-fence-one-form-of-doc-list` | inside | no-break | - | - | open | - |
| `r1-doc-list-non-utf8-path-drops-orphan-rows` | inside | no-break | - | - | open | - |
| `r1-validate-non-utf8-path-drops-blocking-orphan` | outside: excluded | unclear | refuted | - | open | - |
| `r1-served-from-note-nested-worktree-path` | inside | no-break | - | - | open | - |
| `r1-task-read-before-setup-answers-no-task` | inside | no-break | - | - | open | - |
| `r1-doc-list-prints-id-doc-show-refuses` | inside | unclear | refuted | - | open | - |
| `r1-doc-list-reads-through-link-out-of-repo` | inside | no-break | - | - | open | - |
| `r1-doc-list-fsmonitor-cookie-mtime` | inside | no-break | - | - | open | - |
| `r1-non-utf8-path-other-orphan-walk-consumers` | outside: unlisted | unclear | refuted | - | open | - |
| `r1-doc-list-cells-driven-on-one-platform` | inside | no-break | - | - | open | - |
| `r1-finalize-may-commit-redirected-log-record` | outside: excluded | unclear | refuted | - | open | - |
| `r1-uninstall-log-append-through-live-link` | outside: excluded | unclear | refuted | - | open | - |
| `r1-run-session-refuses-scripted-arm-without-credential` | outside: unlisted | no-break | - | - | open | - |
| `r1-arm-control-does-not-reach-changed-path` | inside | no-break | - | - | open | - |
| `r1-walk-arm-runs-in-caller-dir-outside-container` | outside: unlisted | no-break | - | - | open | - |
| `r1-observe-exits-1-on-scripted-arm-evidence` | outside: unlisted | no-break | - | - | open | - |
| `r1-config-set-leaves-manifest-untracked` | outside: excluded | no-break | - | - | open | - |
| `r1-provenance-names-model-no-session-used` | outside: unlisted | no-break | - | - | open | - |
| `r1-control-pass-counts-failed-read` | outside: unlisted | no-break | - | - | open | - |
| `r1-run-session-exits-0-when-arm-fails` | outside: unlisted | no-break | - | - | open | - |
| `r1-arm-evidence-git-dir-touched-after-run` | outside: unlisted | no-break | - | - | open | - |
| `r1-scope-registry-is-the-scope-steps-choice` | outside: unlisted | no-break | - | - | open | - |
| `r1-scope-other-registries-unread-two-changed` | outside: unlisted | unclear | refuted | - | open | - |
| `r1-scope-47-doors-reached-and-excluded` | outside: unlisted | no-break | - | - | open | - |
| `r1-scope-control-arm-drives-excluded-doors` | outside: unlisted | no-break | - | - | open | - |
| `r1-scope-never-selected-lists-row-before-test` | outside: unlisted | no-break | - | - | open | - |
| `r1-scope-change-sits-on-committed-arm-only` | inside | no-break | - | - | open | - |
| `r1-doc-read-surface-silent-on-unreadable-instance` | inside | no-break | - | - | open | - |
| `r1-validate-clean-over-store-doc-list-cannot-enumerate` | outside: excluded | no-break | - | - | open | - |
| `r1-doc-read-surface-stale-index-line-citations` | outside: unlisted | no-break | - | - | open | - |
| `r1-doc-list-unreadable-entry-linux-not-driven` | inside | needs-bound | - | - | open | - |
| `r1-route-clause-reading-refusal-without-route` | inside | needs-bound | - | - | open | - |
| `r1-doc-list-unreadable-entry-undriven-shapes-and-consumers` | inside | unclear | refuted | - | open | - |
| `r1-staged-note-route-exits-1-under-unreadable-staged-entry` | inside | needs-bound | - | - | open | - |
| `r1-staging-area-writers-not-enumerated` | inside | unclear | refuted | - | open | - |
| `r1-doc-list-staged-arm-linux-and-root-not-driven` | inside | needs-bound | - | - | open | - |
| `r1-staged-doc-ids-other-callers-not-driven` | outside: unlisted | unclear | refuted | - | open | - |
| `r1-doc-list-staged-arm-pin-limits` | inside | no-break | - | - | open | - |
| `r1-setup-outside-work-tree-exit-pinned-as-parity` | outside: excluded | no-break | - | - | open | - |
| `r1-setup-in-submodule-writes-superproject-hooks` | outside: excluded | needs-bound | - | - | open | - |
| `r1-superproject-hook-effect-not-established` | outside: excluded | unclear | refuted | - | open | - |
| `r1-setup-install-hook-route-other-causes-unexamined` | outside: excluded | unclear | confirmed | yes | open | - |
| `r1-setup-separate-git-dir-not-driven-by-verifier` | outside: excluded | no-break | - | - | open | - |
| `r1-finalize-destroys-untracked-file-where-home-is-no-work-tree` | outside: excluded | needs-bound | - | - | open | - |
| `r1-second-finalize-blocked-where-home-is-no-work-tree` | outside: excluded | needs-bound | - | - | open | - |
| `r1-validate-home-vacated-for-homes-never-occupied` | outside: excluded | needs-bound | - | - | open | - |
| `r1-milestone-create-fails-where-home-is-no-work-tree` | outside: excluded | no-break | - | - | open | - |
| `r1-uninstall-refuses-and-force-half-removes-no-work-tree-home` | outside: excluded | needs-bound | - | - | open | - |
| `r1-unmanage-bare-error-or-false-no-file-no-work-tree-home` | outside: excluded | needs-bound | - | - | open | - |
| `r1-finalize-ack-names-main-checkout-where-none-exists` | outside: excluded | no-break | - | - | open | - |
| `r1-setup-typed-beside-bare-git-dir-installs-at-exit-0` | outside: excluded | needs-bound | - | - | open | - |
| `r1-decisions-pending-d-row-true-one-doc-deep` | outside: unlisted | no-break | - | - | open | - |
| `r1-start-resolves-home-five-times` | outside: excluded | no-break | - | - | open | - |
| `r1-advocate-report-underclaims-doc-list-cell-behind-bare` | outside: unlisted | no-break | - | - | open | - |
| `r1-proposal-hook-inert-in-no-work-tree-layouts` | outside: unlisted | no-break | - | - | open | - |
| `r1-proposal-bare-worktrees-share-one-hooks-directory` | outside: unlisted | no-break | - | - | open | - |
| `r1-proposal-submodule-linked-worktree-home-is-a-choice` | outside: unlisted | no-break | - | - | open | - |
| `r1-proposal-leaves-source-comments-stating-old-rule` | outside: unlisted | no-break | - | - | open | - |
| `r1-proposal-strands-open-task-under-old-home` | outside: unlisted | no-break | - | - | open | - |
| `r1-doc-list-no-committed-docs-where-home-is-no-work-tree` | inside | no-break | - | - | open | - |
| `r1-advocate-call-site-count-not-reproducible` | outside: unlisted | no-break | - | - | open | - |
| `r1-working-product-reading-false-green-of-validate` | outside: excluded | needs-bound | - | - | open | - |
| `r1-non-utf8-tracked-path-supported-or-planted` | outside: excluded | needs-bound | - | - | open | - |
| `r1-validate-unreadable-orphan-variant-not-driven` | outside: excluded | unclear | refuted | - | open | - |
| `r1-doc-show-lowercase-address-prints-path-no-file-has` | outside: excluded | unclear | confirmed | no | open | - |
| `r1-validate-calls-untracked-file-committed` | outside: excluded | no-break | - | - | open | - |
| `r1-adoption-route-for-non-doc-id-name-not-run` | outside: excluded | unclear | confirmed | no | open | - |
| `r1-unregistered-row-prints-address-doc-show-refuses-design` | inside | no-break | - | - | open | - |
| `r1-doc-list-prints-id-coverage-claim-half-true` | inside | no-break | - | - | open | - |
| `r1-root-knob-lands-over-undecodable-git-listing` | outside: excluded | needs-bound | - | - | open | - |
| `r1-validate-clean-over-docs-stranded-by-root-knob` | outside: excluded | no-break | - | - | open | - |
| `r1-doc-show-stranded-doc-not-found-route` | outside: excluded | unclear | refuted | - | open | - |
| `r1-orphaned-doc-route-after-strand-not-run` | outside: excluded | unclear | refuted | - | open | - |
| `r1-orphan-walk-consumers-previous-release-not-driven` | outside: unlisted | unclear | refuted | - | open | - |
| `r1-non-utf8-path-reach-not-driven` | outside: unlisted | needs-bound | - | - | open | - |
| `r1-git-capture-other-callers-not-enumerated` | outside: unlisted | unclear | refuted | - | open | - |
| `r1-log-writer-follows-link-at-log-path` | inside | needs-bound | - | - | open | - |
| `r1-finalize-log-link-into-path-finalize-stages` | outside: excluded | needs-bound | - | - | open | - |
| `r1-log-link-other-committing-doors-not-driven` | outside: unlisted | needs-bound | - | - | open | - |
| `r1-uninstall-help-says-touches-nothing-outside` | outside: excluded | no-break | - | - | open | - |
| `r1-uninstall-force-warning-overstates-link-loss` | outside: excluded | no-break | - | - | open | - |
| `r1-regression-list-holds-no-row-for-two-ruled-differences` | outside: unlisted | no-break | - | - | open | - |
| `r1-promote-clobber-route-commits-staged-blob` | outside: excluded | unclear | refuted | - | open | - |
| `r1-p3-ingest-route-fails-beside-unreadable-entry` | outside: excluded | needs-bound | - | - | open | - |
| `r1-p3-not-staged-route-task-less-read-refuses` | outside: excluded | unclear | confirmed | no | open | - |
| `r1-p3-not-found-over-unreadable-file-routes-in-a-loop` | outside: excluded | needs-bound | - | - | open | - |
| `r1-p3-unreadable-entry-stops-commit-boundaries-no-route` | outside: unlisted | no-break | - | - | open | - |
| `r1-p3-doc-list-drops-unreadable-stamped-orphan-silently` | inside | needs-bound | - | - | open | - |
| `r1-p3-validate-two-postures-over-unreadable-entry` | outside: excluded | no-break | - | - | open | - |
| `r1-p3-unreadable-entry-class-door-list-traced-by-hand` | outside: unlisted | unclear | refuted | - | open | - |
| `r1-p3-refusing-doors-leave-own-cache-files-at-exit-1` | outside: unlisted | no-break | - | - | open | - |
| `r1-p3-finalize-left-out-hint-over-unreadable-file` | outside: excluded | no-break | - | - | open | - |
| `r1-p3-unreadable-entry-undriven-platform-shapes-and-doors` | outside: unlisted | unclear | refuted | - | open | - |
| `r1-p3-reporter-slip-helper-scripts-by-file-tool` | outside: unlisted | no-break | - | - | open | - |
| `r1-p3-umask-masking-owner-read-unreadable-staged-doc` | inside | needs-bound | - | - | open | - |
| `r1-p3-doc-list-tracked-dangling-link-raw-os-error` | inside | unclear | refuted | - | open | - |
| `r1-p3-copy-in-doors-raw-os-error-unreadable-committed-doc` | outside: unlisted | no-break | - | - | open | - |
| `r1-p3-milestone-merged-area-writer-follows-a-link` | outside: unlisted | unclear | refuted | - | open | - |
| `r1-p3-staging-area-writers-undriven-and-unread-remainder` | inside | unclear | refuted | - | open | - |
| `r1-p3-reporter-slip-driver-by-file-tool-report-patched` | outside: unlisted | no-break | - | - | open | - |
| `r1-p3-staged-prose-refusal-read-route-says-not-staged` | outside: unlisted | needs-bound | - | - | open | - |
| `r1-p3-task-finalize-raw-os-error-unreadable-staged-file` | outside: excluded | needs-bound | - | - | open | - |
| `r1-p3-forced-doors-name-no-staged-docs-under-dangling-link` | outside: unlisted | no-break | - | - | open | - |
| `r1-p3-staged-doc-ids-orientation-and-milestone-callers` | outside: unlisted | unclear | refuted | - | open | - |
| `r1-p3-discard-uninstall-plants-linux-and-root-not-driven` | outside: unlisted | unclear | - | - | open | - |
| `r1-p3-superproject-with-own-install-shared-hook-not-driven` | outside: unlisted | unclear | refuted | - | open | - |
| `r1-p3-setup-in-submodule-no-backstop-install-elsewhere` | outside: excluded | needs-bound | - | - | open | - |
| `r1-p3-validate-in-submodule-runs-at-git-dir` | outside: excluded | needs-bound | - | - | open | - |
| `r1-p3-hook-in-another-repository-layouts-not-enumerated` | outside: excluded | unclear | refuted | - | open | - |
| `r1-p3-superproject-hook-block-unpinned` | outside: excluded | no-break | - | - | open | - |
| `r1-p3-setup-writability-route-printed-for-other-causes` | outside: excluded | breaks | confirmed | no | open | - |
| `r1-p3-uninstall-writability-route-not-driven` | outside: excluded | unclear | confirmed | no | open | - |
| `r1-p3-display-hook-path-other-callers-not-driven` | outside: unlisted | unclear | refuted | - | open | - |
| `r1-p3-setup-hook-refusal-after-install-files-written` | outside: excluded | no-break | - | - | open | - |
| `r1-p3-previous-release-wrote-through-hook-link` | outside: excluded | no-break | - | - | open | - |
| `r1-p3-setup-install-hook-undriven-producers-and-causes` | outside: excluded | unclear | confirmed | yes | open | - |
| `r1-p3-validate-clean-over-unreadable-orphan-reading-and-layout` | outside: excluded | needs-bound | - | - | open | - |
| `r1-p3-validate-clean-over-orphan-absent-from-work-tree` | outside: excluded | unclear | refuted | - | open | - |
| `r1-p3-validate-unreadable-orphan-platforms-and-variants` | outside: excluded | unclear | refuted | - | open | - |
| `r1-p3-migrate-case-variant-path-says-git-holds-no-copy` | outside: excluded | breaks | confirmed | no | open | - |
| `r1-p3-validate-says-no-address-reaches-file-one-does` | outside: excluded | no-break | - | - | open | - |
| `r1-p3-migration-destination-is-source-on-case-folding-volume` | outside: excluded | unclear | refuted | - | open | - |
| `r1-p3-doc-show-lowercase-address-case-sensitive-volume` | outside: excluded | unclear | refuted | - | open | - |
| `r1-p3-lowercase-address-other-doors-not-driven` | outside: unlisted | unclear | confirmed | no | open | - |
| `r1-p3-doc-author-nothing-persisted-yet-task-bound` | outside: excluded | unclear | refuted | - | open | - |
| `r1-p3-refused-doc-rename-leaves-staged-foreign-copy` | outside: excluded | unclear | refuted | - | open | - |
| `r1-p3-rename-bare-git-mv-error-no-code-no-route` | outside: excluded | unclear | refuted | - | open | - |
| `r1-p3-finalize-approve-advisory-for-file-it-retires` | outside: excluded | no-break | - | - | open | - |
| `r1-p3-adoption-of-staged-source-no-deletion-row` | outside: excluded | unclear | refuted | - | open | - |
| `r1-p3-adoption-route-title-upper-undriven-cells` | outside: excluded | unclear | refuted | - | open | - |
| `r1-p3-adoption-report-table-names-rig-not-driven` | outside: unlisted | no-break | - | - | open | - |
| `r1-p3-doc-show-stranded-generic-route-non-utf8-layout` | outside: excluded | needs-bound | - | - | open | - |
| `r1-p3-start-workflow-bare-refusal-non-utf8-index-entry` | outside: excluded | needs-bound | - | - | open | - |
| `r1-p3-doc-show-task-no-committed-copy-of-stranded-doc` | outside: excluded | unclear | confirmed | no | open | - |
| `r1-p3-second-doc-of-one-identity-beside-stranded-doc` | outside: unlisted | unclear | refuted | - | open | - |
| `r1-p3-doc-list-omits-stranded-doc` | inside | unclear | refuted | - | open | - |
| `r1-p3-start-orientation-host-temp-path-findings-unknown` | outside: excluded | unclear | refuted | - | open | - |
| `r1-p3-config-set-then-git-reset-strands-relocated-docs` | outside: excluded | no-break | - | - | open | - |
| `r1-p3-orphaned-doc-route-unmanage-without-path` | outside: excluded | no-break | - | - | open | - |
| `r1-p3-doc-show-offers-unmanage-after-unmanage` | outside: excluded | unclear | refuted | - | open | - |
| `r1-p3-unregistered-doc-route-after-unmanage-not-run` | outside: excluded | unclear | refuted | - | open | - |
| `r1-p3-finalize-after-orphaned-doc-route-arms-not-driven` | outside: excluded | unclear | refuted | - | open | - |
| `r1-p3-orphaned-doc-placement-arm-and-json-not-driven` | outside: excluded | unclear | refuted | - | open | - |
| `r1-p3-reporter-slip-count-corrected-by-shell` | outside: unlisted | no-break | - | - | open | - |
| `r1-p3-root-knob-doors-non-utf8-name-no-lost-files-reading` | outside: unlisted | needs-bound | - | - | open | - |
| `r1-p3-validate-clean-after-placement-root-strand-non-utf8` | outside: excluded | needs-bound | - | - | open | - |
| `r1-p3-git-capture-callers-enumerated-by-neighbouring-report` | outside: unlisted | no-break | - | - | open | - |
| `r1-p3-orphan-walk-aftermath-on-previous-release` | outside: unlisted | no-break | - | - | open | - |
| `r1-p3-task-finalize-refuses-staged-non-utf8-text-file` | outside: excluded | unclear | refuted | - | open | - |
| `r1-p3-rename-mention-advisory-silent-undecodable-listing` | outside: excluded | unclear | refuted | - | open | - |
| `r1-p3-uninstall-force-names-nothing-undecodable-listing` | outside: excluded | needs-bound | - | - | open | - |
| `r1-p3-amend-refusal-route-spells-c-quoted-name-as-path` | outside: excluded | unclear | confirmed | no | open | - |
| `r1-p3-milestone-doors-bare-refusal-non-utf8-index-entry` | outside: unlisted | needs-bound | - | - | open | - |
| `r1-p3-post-commit-capture-sites-host-that-holds-the-name` | outside: unlisted | unclear | refuted | - | open | - |
| `r1-p3-git-capture-commit-message-sites-not-driven` | outside: unlisted | unclear | refuted | - | open | - |
| `r1-p3-finalize-text-left-out-list-no-kind` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-unparseable-adoption-route-refuses-untracked-source` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-not-staged-route-mode-000-repro-b-not-redriven` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-not-staged-block-other-unservable-states-not-enumerated` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-precommit-hook-silent-pass-over-unreadable-entry` | outside: unlisted | needs-bound | - | - | open | - |
| `r1-p4-nothing-staged-route-git-add-fails-unreadable-file` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-doc-create-over-unreadable-home-bare-refusal` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-relocate-parks-untracked-entry-at-destination` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-validate-unreadable-entry-masks-out-of-band-rename` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-unreadable-entry-class-linux-root-remaining-cells-not-driven` | outside: unlisted | unclear | - | - | open | - |
| `r1-p4-named-pipe-at-managed-home-holds-reading-doors` | outside: unlisted | needs-bound | - | - | open | - |
| `r1-p4-migrate-corpus-omits-unreadable-doc-silently` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-validate-unreadable-entry-masks-schema-version-finding` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-relocate-blocked-unreadable-doc-no-route` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-not-found-not-staged-routes-loop-at-resume-door` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-reconciliation-rename-route-jigc-rename-not-found` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-doc-list-directory-named-md-ends-listing` | inside | no-break | - | - | open | - |
| `r1-p4-unreadable-entry-refusals-do-not-name-the-file` | outside: unlisted | no-break | - | - | open | - |
| `r1-p4-doc-list-refusal-prints-absolute-path` | inside | no-break | - | - | open | - |
| `r1-p4-doc-list-dangling-link-reach-by-jigc-verb-not-driven` | inside | unclear | - | - | open | - |
| `r1-p4-doc-list-dangling-link-undriven-cells` | inside | unclear | - | - | open | - |
| `r1-p4-reading-refusal-without-route-under-working-product` | inside | needs-bound | - | - | open | - |
| `r1-p4-doc-list-dangling-link-block-pinning-note` | inside | no-break | - | - | open | - |
| `r1-p4-milestone-merged-writer-planted-link-no-declared-bound` | outside: excluded | needs-bound | - | - | open | - |
| `r1-p4-milestone-finalize-drops-authored-doc-under-planted-link` | outside: excluded | needs-bound | - | - | open | - |
| `r1-p4-milestone-finalize-refusal-writes-through-planted-link` | outside: excluded | needs-bound | - | - | open | - |
| `r1-p4-milestone-merged-link-plant-undriven-cells` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-following-writes-class-not-walked` | outside: unlisted | unclear | - | - | open | - |
| `r1-p4-staging-area-writers-root-and-linux-not-driven` | inside | unclear | - | - | open | - |
| `r1-p4-finalize-refuses-tracked-link-home-previous-release-landed` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-finalize-approve-read-only-destination-raw-os-error` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-unadopted-instance-route-prints-absolute-path` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-tracked-path-under-task-staging-area-git-writer` | outside: unlisted | unclear | - | - | open | - |
| `r1-p4-staging-area-writers-undriven-doors` | inside | unclear | - | - | open | - |
| `r1-p4-staging-area-enumeration-bound-and-other-readers` | inside | unclear | - | - | open | - |
| `r1-p4-orientation-nothing-staged-under-dangling-link` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-milestone-discard-force-unnamed-staged-doc-under-link` | outside: excluded | needs-bound | - | - | open | - |
| `r1-p4-milestone-discard-json-ack-names-nothing` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-milestone-staged-prose-route-not-staged-unreadable-doc` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-milestone-finalize-missing-provenance-route-no-command` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-orientation-absolute-path-findings-unknown-unreadable-doc` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-staged-doc-ids-other-plants-linux-root-not-driven` | outside: unlisted | unclear | - | - | open | - |
| `r1-p4-staged-doc-ids-plant-reach-not-driven` | outside: unlisted | unclear | - | - | open | - |
| `r1-p4-erratum-milestone-door-cell-count-in-report` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-uninstall-submodule-refusal-first-route-does-not-clear` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-uninstall-force-in-submodule-cuts-superproject-hook-block` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-validate-upgrade-silent-about-missing-hook-block` | outside: unlisted | no-break | - | - | open | - |
| `r1-p4-uninstall-force-leaves-settings-json-in-submodule-git-dir` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-superproject-submodule-undriven-orders` | outside: unlisted | unclear | - | - | open | - |
| `r1-p4-shared-hook-jigc-line-names-last-setup-binary` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-setup-ack-local-to-this-checkout-of-shared-hook` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-setup-separate-git-dir-commits-into-enclosing-repository` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-hook-rename-block-absent-in-linked-worktree` | outside: unlisted | unclear | - | - | open | - |
| `r1-p4-hook-git-dir-export-validate-children-not-examined` | outside: unlisted | unclear | - | - | open | - |
| `r1-p4-setup-separate-git-dir-refusal-route-not-driven` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-setup-bare-repository-pointer-installs-outside-work-tree` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-two-installs-one-hook-other-layouts-not-driven` | outside: unlisted | unclear | - | - | open | - |
| `r1-p4-hook-layouts-undriven-cells-and-previous-release` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-uninstall-writability-sentence-other-causes-not-driven` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-setup-writability-route-names-directory-not-parent` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-setup-over-compiled-program-hook-not-driven` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-setup-writability-route-undriven-causes` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-uninstall-six-other-teardown-refusals-same-route-shape` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-uninstall-refusal-leaves-teardown-half-done` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-uninstall-writes-through-outward-hook-link` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-uninstall-ignores-installed-hook-after-hookspath-change` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-uninstall-removes-hook-link-leaves-block-at-links-end` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-uninstall-writability-refusal-undriven-causes` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-uninstall-warning-outward-link-outside-file-not-driven` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-uninstall-kept-hook-sentence-not-tested` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-setup-after-warned-delete-dangling-hook-link-refuses-once` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-uninstall-force-linked-hook-block-pinning-note` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-display-hook-path-undriven-states` | outside: unlisted | unclear | - | - | open | - |
| `r1-p4-setup-make-executable-failure-not-driven-on-candidate` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-setup-hook-linked-to-dev-null-route-names-dev-null` | outside: excluded | breaks | - | - | open | - |
| `r1-p4-setup-writability-route-directory-or-unreadable-hook` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-setup-install-hook-refusal-after-install-files-written` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-setup-splices-into-hook-once-readable` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-setup-install-hook-undriven-sites` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-validation-doc-silent-on-where-stamp-is-read` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-validate-green-line-says-committed-with-unstaged-deletion` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-validate-names-only-on-disk-orphan` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-doc-list-over-orphan-absent-from-work-tree-not-driven` | inside | unclear | - | - | open | - |
| `r1-p4-orphan-absent-verdict-basis-note` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-orphan-absent-file-posture-unpinned` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-orphan-absent-class-instance-unbounded` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-validate-unreadable-orphan-ownership-root-linux-not-driven` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-reading-rely-on-and-exit-0-false-green` | outside: excluded | needs-bound | - | - | open | - |
| `r1-p4-validate-unreadable-docs-root-names-repository-root` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-validate-two-postures-for-unreadable-entry` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-doc-list-over-unreadable-orphan-not-driven` | inside | unclear | - | - | open | - |
| `r1-p4-migration-of-upper-case-file-stage-failed-route-refuses` | outside: excluded | breaks | - | - | open | - |
| `r1-p4-finalize-rollback-conflict-false-sentence-leaves-rewrite` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-conflict-block-route-after-failed-migration-not-driven` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-migration-review-hold-names-two-spellings-of-one-file` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-ingest-case-rename-route-not-driven` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-migration-source-equals-destination-undriven-states` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-doc-list-prints-id-no-door-takes-seen-again` | inside | unclear | - | - | open | - |
| `r1-p4-two-case-spellings-side-by-side-not-driven` | outside: unlisted | unclear | - | - | open | - |
| `r1-p4-doc-show-lowercase-ignorecase-mismatch-and-linux-not-driven` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-doc-show-not-found-route-task-read-not-driven` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-address-doors-case-sensitive-volume-not-driven` | outside: unlisted | unclear | - | - | open | - |
| `r1-p4-finalize-exit-0-commits-without-doc-lowercase-address` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-finalize-base-mismatch-after-ingest-case-rename` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-copy-in-records-baseline-under-spelling-no-file-has` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-task-validate-two-spellings-clean-before-failing-finalize` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-validate-ingest-say-no-address-reaches-file` | outside: unlisted | no-break | - | - | open | - |
| `r1-p4-rename-lowercase-address-refusal-wording-and-ordering` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-unmanage-address-operand-no-op-exit-0` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-lowercase-address-six-doors-not-driven` | outside: unlisted | unclear | - | - | open | - |
| `r1-p4-identity-change-route-doc-rename-exits-1-over-foreign-file` | outside: excluded | breaks | - | - | open | - |
| `r1-p4-non-reparseable-route-names-neither-broken-thing` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-refused-doc-author-leaves-file-state-lock` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-provenance-row-of-rolled-back-doc-survives` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-doc-author-nothing-persisted-undriven-cells` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-nothing-was-persisted-sentence-unpinned` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-verifier-process-note-scratch-scripts-by-file-tool` | outside: unlisted | no-break | - | - | open | - |
| `r1-p4-doc-list-task-reports-foreign-file-managed` | inside | unclear | - | - | open | - |
| `r1-p4-task-validate-one-file-two-readings-after-refused-rename` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-task-discard-says-no-commit-holds-copy-of-head-blob` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-refused-doc-rename-leaves-copy-in-and-binding` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-refused-doc-rename-conformant-foreign-file-not-driven` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-refused-doc-rename-undriven-volume-and-previous-release` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-rename-moves-and-rewrites-unadopted-file` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-rename-frame-names-identity-listing-does-not-hold` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-rename-cause-line-does-not-name-case-difference` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-rename-refusal-undriven-volume-and-previous-release` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-rename-repro-r-1-never-run-whole` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-source-untracked-route-cures-one-of-two-reasons` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-approve-ack-does-not-name-removal-staged-only-source` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-composed-step-says-nothing-to-recover-from` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-unadopted-instance-advisory-miscalls-file` | outside: unlisted | no-break | - | - | open | - |
| `r1-p4-staged-source-edited-after-git-add-not-driven` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-unreachable-blob-lifetime-not-driven` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-adoption-chain-upper-previous-release-not-driven` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-reading-git-object-reachable-staged-only-adoption` | outside: excluded | needs-bound | - | - | open | - |
| `r1-p4-staged-source-adoption-no-deletion-row-seen-again` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-setup-install-commit-carries-settings-entries-json` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-adoption-route-upper-other-platform-not-driven` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-adoption-route-upper-blocks-pinnability-note` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-not-staged-over-strand-verdict-reading-note` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-not-staged-route-create-or-author-gate-blocked` | outside: excluded | breaks | - | - | open | - |
| `r1-p4-doc-create-over-strand-second-doc-seen-again` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-set-slot-over-strand-bare-refusal-line` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-reading-incorrect-thing-written-second-doc` | outside: unlisted | needs-bound | - | - | open | - |
| `r1-p4-unregistered-doc-never-adopted-after-reset-strand` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-unregistered-doc-route-prints-absolute-path` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-reconciliation-rename-routes-unmanage-of-staged-path` | outside: unlisted | unclear | - | - | open | - |
| `r1-p4-fork-doc-list-row-for-knob-stranded-doc` | inside | no-break | - | - | open | - |
| `r1-p4-config-set-root-knob-moves-nothing-non-utf8-entry` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-validate-silent-about-strand-under-non-utf8-entry` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-task-validate-bare-line-under-non-utf8-index-entry` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-orientation-run-directives-not-driven-non-utf8-entry` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-start-json-findings-unavailable-per-invocation-path` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-printed-path-fence-no-row-for-materialize-index` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-start-orientation-block-pinnability-note` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-doc-list-shows-unmanaged-doc-managed-after-repair` | inside | no-break | - | - | open | - |
| `r1-p4-unregistered-doc-says-never-adopted-after-unmanage` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-unregistered-doc-route-names-moved-away-path` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-doc-show-stale-unmanage-offer-placement-and-json` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-finalize-after-unmanage-arms-of-show-route-not-driven` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-doc-show-stale-offer-block-pinning-note` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-migrate-help-says-ingest-advisory-routes-migrate` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-migration-commit-lands-pending-manifest` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-live-task-finalize-after-migrate-route-not-driven` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-unregistered-doc-route-undriven-cells` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-unregistered-doc-route-placement-fallback-json-not-driven` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-unregistered-doc-block-pinnability-note` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-unregistered-doc-route-emitted-command-unpinned` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-ref-resolves-route-create-arm-gate-blocked` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-unmanage-silent-about-open-task-reference` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-finalize-ref-resolves-over-stranded-target-names-no-strand` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-finalize-design-stage-sentence-against-own-rollback-table` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-finalize-after-orphaned-doc-route-remaining-arms` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-ingest-after-hand-move-keeps-prior-file-state-keys` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-unmanage-stranded-placement-doc-edges-not-exercised` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-finalize-after-placement-strand-arms-not-driven` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-task-validate-clean-before-finalize-refuses-non-utf8-file` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-migration-finalize-capture-helper-callers-not-enumerated` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-migration-approve-commits-staged-file-not-the-migrations` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-handed-block-control-does-not-hold-empty-commit` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-finalize-lands-when-git-treats-file-as-binary` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-finalize-non-utf8-text-file-undriven-shapes` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-rename-mention-scan-other-quoted-names-not-driven` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-rename-empty-advisory-reads-like-undecodable-listing` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-line-oriented-git-listing-consumers-not-traced` | outside: unlisted | unclear | - | - | open | - |
| `r1-p4-rename-mention-block-pinning-note` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-amend-refusal-route-preview-door-not-driven` | outside: unlisted | unclear | - | - | open | - |
| `r1-p4-amend-refusal-message-and-locus-c-quoted-spelling` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-amend-refusal-route-undriven-name-shapes` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-amend-refusal-route-non-regression-weight-note` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-task-finalize-exits-1-after-commit-planted-hook` | outside: excluded | needs-bound | - | - | open | - |
| `r1-p4-milestone-finalize-exits-1-after-commit-planted-hook` | outside: excluded | needs-bound | - | - | open | - |
| `r1-p4-finalize-doors-refuse-staged-non-utf8-name` | outside: unlisted | unclear | - | - | open | - |
| `r1-p4-amend-arm-no-file-state-record-when-note-fires` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-landed-summary-c-quoted-name-and-no-hash-record` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-milestone-finalize-removes-worktree-with-untracked-file` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-left-out-list-names-one-entry-after-landing` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-post-commit-capture-real-host-previous-release-not-driven` | outside: unlisted | unclear | - | - | open | - |
| `r1-p4-amend-dry-run-bare-refusal-undecodable-head-subject` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-amend-arm-drops-co-author-trailer-undecodable-message` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-task-finalize-exits-1-after-commit-latin1-log-output` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-milestone-finalize-exits-1-after-commits-latin1-log-output` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-milestone-provision-failed-says-0-landed-worktree-registered` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-task-amend-ack-no-subject-line-undecodable-subject` | outside: excluded | no-break | - | - | open | - |
| `r1-p4-amend-empty-commit-frame-says-fix-the-hooks-complaint` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-milestone-aggregate-subject-site-not-reached` | outside: excluded | unclear | - | - | open | - |
| `r1-p4-commit-encoding-non-utf8-not-driven` | outside: unlisted | unclear | - | - | open | - |
| `r1-p4-reading-ordinary-git-configuration-log-output-encoding` | outside: unlisted | needs-bound | - | - | open | - |
| `r1-p4-prescribed-plant-commit-tree-reencodes-message` | outside: unlisted | no-break | - | - | open | - |
