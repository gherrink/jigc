# 2. Keep the sample store in memory

Date: 2026-08-04

## Status

Accepted

## Context

We looked at putting samples in SQLite so a restart doesn't lose the window.
The service is a rollup *cache* — the caller already has a durable store, and
every sample we hold is one they can replay. Adding a database means a schema,
a migration story and a fsync on the hot ingest path.

## Decision

The store stays in memory. Durability is the caller's problem, and we say so
in the README rather than solving it twice.

## Consequences

A restart loses the current window. Anything that needs history reads it from
the caller's own store, not from us. If that ever stops being true, this
decision gets superseded rather than quietly worked around.
