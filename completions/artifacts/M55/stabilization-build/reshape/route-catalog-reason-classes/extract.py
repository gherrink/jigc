#!/usr/bin/env python3
"""List every `Why::NotConstructible(..)` / `Why::NotBuilt(..)` written in the route
catalog's command files (`crates/cli/tests/route_catalog/*.rs`), with its file:line, its
text resolved (a literal, a `const` of the file, or the literal head of a `concat!`), and
the projection lines (`projection.tsv`'s `reason`, `without-case` and `not-followed`) that
carry that text. Usage: extract.py <repo> > declarations.json"""
import re, sys, os, json


def lit_at(src, i):
    """Parse a Rust string literal starting at src[i] == '"'; return (value, end)."""
    assert src[i] == '"', src[i:i+20]
    out = []
    j = i + 1
    while True:
        c = src[j]
        if c == '\\':
            n = src[j+1]
            if n == '\n':
                j += 2
                while src[j] in ' \t\n':
                    j += 1
                continue
            out.append({'n': '\n', 't': '\t', '"': '"', '\\': '\\', "'": "'"}.get(n, '\\' + n))
            j += 2
            continue
        if c == '"':
            return ''.join(out), j + 1
        out.append(c)
        j += 1

def consts(src):
    m = {}
    for mm in re.finditer(r'const ([A-Z_0-9]+): &str =\s*"', src):
        v, _ = lit_at(src, mm.end() - 1)
        m[mm.group(1)] = v
    return m

def declarations(repo):
    d = os.path.join(repo, "crates/cli/tests/route_catalog")
    files = sorted(f for f in os.listdir(d) if f.endswith(".rs"))
    proj = []
    for line in open(os.path.join(d, "projection.tsv")):
        if line.startswith('#'):
            continue
        f = line.rstrip('\n').split('\t')
        if f[0] == 'reason':
            proj.append(dict(kind='reason', site=f[1], why=f[2], cause='', text=f[4]))
        elif f[0] in ('without-case', 'not-followed'):
            proj.append(dict(kind=f[0], site=f[1], why=f[2], cause=f[3], text=f[4]))

    rows = []
    for fn in files:
        src = open(os.path.join(d, fn)).read()
        cs = consts(src)
        for mm in re.finditer(r'Why::(NotConstructible|NotBuilt)\(\s*', src):
            why = 'not-constructible' if mm.group(1) == 'NotConstructible' else 'not-built'
            j = mm.end()
            if src[j] == '"':
                text, _ = lit_at(src, j)
                via = ''
            elif src.startswith('concat!(', j):
                k2 = src.index('"', j)
                text, _ = lit_at(src, k2)
                via = 'concat!'
            else:
                name = re.match(r'[A-Z_0-9]+', src[j:]).group(0)
                text = cs[name]
                via = name
            lineno = src.count('\n', 0, mm.start()) + 1
            # context: the nearest preceding `without_case`, `not_followed` or `Reason {`
            before = src[:mm.start()]
            k = max((before.rfind(t), t) for t in ('without_case:', 'not_followed:', 'Reason {', 'reason('))
            kind = {'without_case:': 'without-case', 'not_followed:': 'not-followed',
                    'Reason {': 'reason', 'reason(': 'reason'}[k[1]]
            cause = ''
            if kind != 'reason':
                # the cause/way: last string literal opening after the last '(' before Why
                p = before.rfind('(\n')
                q = before.rfind('&[(')
                start = max(p, q)
                seg = src[start:mm.start()]
                qi = seg.find('"')
                if qi >= 0:
                    cause, _ = lit_at(src, start + qi)
            hits = [p for p in proj if (p['text'] == text or (via == 'concat!' and p['text'].startswith(text))) and p['why'] == why]
            rows.append(dict(file=fn, line=lineno, why=why, via=via,
                             text=text, lines=[(h['kind'], h['site'], h['cause']) for h in hits]))

    return rows


if __name__ == "__main__":
    rows = declarations(sys.argv[1])
    json.dump(rows, sys.stdout, indent=1)
    print(len(rows), "declarations", file=sys.stderr)
