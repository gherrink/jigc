# 1. Compose the parser as discrete stages

## Status

Accepted

## Context

The compile pipeline needed a deterministic, independently-testable structure rather
than one monolithic pass over the source.

## Decision

We will compose the parser/writer pipeline from discrete stages — section parsing
then canonical rendering — each a single-responsibility unit.

## Consequences

Each stage is testable in isolation and the byte-stable render path can be proven
independently of parsing.
