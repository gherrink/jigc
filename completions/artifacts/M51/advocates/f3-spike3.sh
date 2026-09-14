set -e
cd /Users/maurice/projects/gherrink-jigc
out=$(dev/jigc-rig committed-singletons 2>/dev/null) || exit 1
eval "$out"
cd "$REPO"
printf 'tasks/\nindex/\nstate/\nmilestones/\nworktrees/\nlogs/\n# my own note - secrets live here\nmy-private-dir/\n' > .jigc/.gitignore
echo "BEFORE-STATUS: $(git status --porcelain | tr '\n' ';')"
"$JIGC" start "record a decision about caching" --workflow record-decision >/dev/null 2>&1 || true
T=record-a-decision-about-caching
set +e
"$JIGC" doc author adr --task "$T" --from-file - <<'EOF'
title: cache on a single node
sections:
  - id: context
    text: |
      The service reads the same rows repeatedly.
  - id: decision
    text: |
      Cache on a single node.
  - id: consequences
    text: |
      A node loss cold-starts the cache.
EOF
echo "AUTHOR EXIT=$?"
"$JIGC" doc set-field "commit:$T#header/type" --task "$T" --value docs; echo "TYPE EXIT=$?"
printf 'record the single-node cache decision' | "$JIGC" doc set-slot "commit:$T#summary" --task "$T" --from-file -; echo "SUMMARY EXIT=$?"
printf '#!/bin/sh\necho "HOOK SAYS NO" >&2\nexit 1\n' > .git/hooks/pre-commit; chmod +x .git/hooks/pre-commit
echo "=== FINALIZE ==="
"$JIGC" task finalize "$T"; echo "FINALIZE EXIT=$?"
echo "=== AFTER worktree .jigc/.gitignore ==="; cat .jigc/.gitignore
echo "=== AFTER status ==="; git status --porcelain
echo "=== HEAD ==="; git log --oneline -1
echo "=== user line anywhere in git history? ==="; git log --all -p -- .jigc/.gitignore | grep -c "my-private-dir"
