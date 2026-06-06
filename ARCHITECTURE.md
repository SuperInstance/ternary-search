# Architecture — ternary-search

> *Internal design and data flow.*

## Overview

This crate implements ternary {-1, 0, +1} semantics for the `search` domain.
It is one of ~280 ternary crates in the SuperInstance fleet, all sharing Z₃ arithmetic
from [ternary-core](https://github.com/SuperInstance/ternary-core).

## Core Types

- **`StrategyNode`**
- **`StrategyGraph`**
- **`BfsResult`**
- **`DfsResult`**
- **`BeamCandidate`**
- **`AStarResult`**

## Key Functions

- `value()`
- `new()`
- `with_signals()`
- `fitness()`
- `add_neighbor()`
- `new()`
- `add_node()`
- `add_edge()`

## Ternary Mapping

| Value | Meaning |
|-------|---------|
| +1 | Found / match |
| 0  | Unknown / continue |
| -1 | Not found / prune |

## Source Structure

1 Rust source file(s) in `src/`.
Language: Rust

## Cross-Repo References

- [ternary-core](https://github.com/SuperInstance/ternary-core) — shared Z₃ traits
- [ternary-types](https://github.com/SuperInstance/ternary-types) — type-level encodings
- [Full SuperInstance fleet](https://github.com/orgs/SuperInstance/repositories?q=ternary)
