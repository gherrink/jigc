#!/usr/bin/env bash
# selftest.sh — verify the harness's analysis layer with ZERO API spend.
#
# The runner (run-eval.sh) needs Docker + a real agent; the *analysis* (analyze.py +
# eval.py) is pure and is what must stay correct as the workflow layer evolves. This
# exercises both over the committed fixtures and asserts the headline metrics, so a
# future change to the parsers is caught without spending a cent. CI-safe.
set -euo pipefail
cd "$(dirname "$0")"

echo "== analyze.py fixture assertions =="
python3 analyze.py --selftest

echo "== eval.py end-to-end over the fixtures =="
R="$(mktemp -d)"
trap 'rm -rf "$R"' EXIT
i=0
for f in fixtures/engaged-single-task.stream.json \
         fixtures/bypassed-quick-fix.stream.json \
         fixtures/no-engagement.stream.json; do
  mkdir -p "$R/rep$i"; cp "$f" "$R/rep$i/transcript.jsonl"; i=$((i+1))
done
printf 'no findings — the committed store validates clean\n' > "$R/rep0/jigc-validate.after.txt"

python3 eval.py "$R" --expected-workflow single-task > "$R/out.json"
python3 - "$R/out.json" <<'PY'
import json, sys
r = json.load(open(sys.argv[1]))
def check(key, want):
    got = r[key]
    assert got == want, f"{key} = {got!r}, want {want!r}"
    print(f"  ok {key} = {got}")
check("n_runs", 3)
check("engage_rate", 0.667)            # 2/3 ran jigc
check("select_rate_on_expected", 0.5)  # 1 of 2 selectors picked single-task
check("complete_rate", 0.333)          # 1/3 reached finalize
check("outcomes", {"clean": 1, "unknown": 2})
print("eval.py OK")
PY

echo "ALL SELFTESTS PASSED"
