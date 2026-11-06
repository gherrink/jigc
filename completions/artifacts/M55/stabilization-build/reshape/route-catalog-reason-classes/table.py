#!/usr/bin/env python3
"""Join the classification (classes.py, keyed by the order in which each distinct text is
first written) to the projection's reason lines, and print the table. The argument is a
tree that holds `crates/cli/tests/route_catalog/` as it was at the commit the table was
read at, `c69a2f94` — a later commit's catalog has written cases for some of its rows:
  mkdir <dir> && git archive c69a2f94 crates/cli/tests/route_catalog | tar -x -C <dir>
  python3 -I <this dir>/table.py <dir> > <this dir>/../route-catalog-reason-classes.tsv"""
import sys, collections, importlib.util, os

def load(name):
    spec = importlib.util.spec_from_file_location(name, os.path.join(os.path.dirname(os.path.abspath(__file__)), name + ".py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module

cl = load("classes")
repo = sys.argv[1]
decl = load("extract").declarations(repo)
family = {}
for line in open(os.path.join(repo, "crates/cli/tests/route_catalog/projection.tsv")):
    f = line.rstrip('\n').split('\t')
    if f[0] == 'site':
        family[f[3]] = f[1]

# D-number of each distinct text, in first-declaration order (as the dump numbered them).
dnum, n = {}, 0
for x in decl:
    if x['text'] not in dnum:
        n += 1
        dnum[x['text']] = n
# Each projection line: its D-number, and every declaration of that text.
lines = collections.OrderedDict()
for x in decl:
    for kind, site, cause in x['lines']:
        key = (kind, site, cause)
        d = dnum[x['text']]
        e = lines.setdefault(key, {'d': d, 'at': [], 'why': x['why']})
        e['at'].append(f"{x['file']}:{x['line']}")

def short(site):
    return site.replace('crates/cli/src/', 'cli/').replace('crates/engine/src/', 'engine/')

out = []
for (kind, site, cause), e in lines.items():
    cls, piece, fc, ev, alt, key = cl.C[e['d']]
    fam = family.get(site, '')
    if cls == cl.NB:
        owner = cl.NAMED_TO.get(short(site).split('::')[0], 'P-F0')
    elif cls == cl.PP:
        if piece == cl.LEFT:
            owner = 'P-F1'
        elif '(a rule)' in piece:
            owner = 'a ruling (the piece changes a rule)'
        elif fc == 'doubt':
            owner = 'in doubt — P-F1 decides'
        elif piece in cl.ONE_FAMILY:
            owner = cl.NAMED_TO.get(short(site).split('::')[0], 'P-F0')
        else:
            owner = 'P-F0'
    else:
        owner = '-'
    if e['d'] in cl.BUILT:
        owner = f"{owner} — {cl.BUILT[e['d']]}"

    out.append((fam, short(site), kind, cause, e['why'], ', '.join(sorted(set(e['at']))),
                f"T{e['d']}", cls, piece or '-', fc, ev, alt or '-', key or '-', owner))

out.sort(key=lambda r: (r[0], r[1], r[2], r[3]))
hdr = ("family", "site", "line", "cause or way", "word at c69a2f94", "declared at", "text",
       "class", "piece", "F-C", "evidence", "second class or doubt", "finding key", "owner")
print('\t'.join(hdr))
for i, r in enumerate(out, 1):
    for c in r:
        assert '\t' not in c and '\n' not in c, r
    print('\t'.join(r))

cnt = collections.Counter(r[7] for r in out)
print(len(out), "lines;", dict(cnt), file=sys.stderr)
tcnt = collections.Counter(cl.C[d][0] for d in range(1, n + 1))
print(n, "texts;", dict(tcnt), file=sys.stderr)
dcl = collections.Counter(cl.C[dnum[x['text']]][0] for x in decl)
print(len(decl), "declarations;", dict(dcl), file=sys.stderr)
pc = collections.Counter(r[8] for r in out if r[7] == cl.PP)
print("pieces (lines):", dict(pc), file=sys.stderr)
kc = collections.Counter(r[12] for r in out if r[12] != '-')
print("keys (lines):", dict(kc), file=sys.stderr)
fc = collections.Counter(r[9] for r in out)
print("F-C:", dict(fc), file=sys.stderr)
nb = collections.Counter(r[0] for r in out if r[7] == cl.NB)
print("not built by family:", dict(nb), file=sys.stderr)
