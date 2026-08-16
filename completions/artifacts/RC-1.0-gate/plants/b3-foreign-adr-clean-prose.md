# 2. Keep the sample store in memory

Date: 2026-08-04

## Status

Accepted

## Context

We looked at putting samples in SQLite so a restart doesn't lose the window.
The service is a bounded rollup buffer: it holds a recent window and nothing
more, and every sample in it arrived over the ingest path inside that window.
A database means a schema, a migration story and a fsync on the hot ingest
path — paid on every sample, for a window we have already decided we can lose.

## Decision

The store stays in memory. A restart starts a fresh window; a caller that
needs the old one sends it again.

## Consequences

A restart loses the current window. Anything that needs history keeps it
itself, not here. If that ever stops being true, this decision gets superseded
rather than quietly worked around.
