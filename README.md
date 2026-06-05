# ternary-search

**Search algorithms over ternary strategy spaces**

[![ternary](https://img.shields.io/badge/ecosystem-ternary-blue)](https://github.com/orgs/SuperInstance/repositories?q=ternary)
[![tests](https://img.shields.io/badge/tests-19-green)]()

## Overview

Search algorithms over ternary strategy spaces.

Provides binary search on thresholds, BFS/DFS on strategy graphs,
beam search, and A* with fitness heuristics.

## Architecture

- **`StrategyNode`** — core data structure
- **`StrategyGraph`** — core data structure
- **`BfsResult`** — core data structure
- **`DfsResult`** — core data structure
- **`BeamCandidate`** — core data structure
- **`AStarResult`** — core data structure
- **`Ternary`** — state enumeration

### Key Functions

- `value()`
- `new()`
- `with_signals()`
- `fitness()`
- `add_neighbor()`
- `new()`
- `add_node()`
- `add_edge()`
- `get()`
- `node_count()`
- ... and 9 more

## Why Ternary?

The balanced ternary system {-1, 0, +1} (also known as Z₃) is the mathematically optimal discrete encoding:
- **More expressive than binary**: three states capture positive, neutral, and negative
- **Natural for decisions**: accept/reject/abstain, buy/hold/sell, agree/disagree/neutral
- **Self-balancing**: the 0 state acts as a universal screen, preventing pathological lock-in
- **Z₃ cyclic dynamics**: rock-paper-scissors is the only natural coordination mechanism

## Stats

| Metric | Value |
|--------|-------|
| Lines of Rust | 574 |
| Test count | 19 |
| Public types | 7 |
| Public functions | 19 |

## Ecosystem

This crate is part of the **[SuperInstance Ternary Fleet](https://github.com/orgs/SuperInstance/repositories?q=ternary)**:

- **[ternary-core](https://github.com/SuperInstance/ternary-core)** — shared traits and Z₃ arithmetic
- **[ternary-grid](https://github.com/SuperInstance/ternary-grid)** — spatial grid with {-1, 0, +1} cells
- **[ternary-graph](https://github.com/SuperInstance/ternary-graph)** — ternary-weighted graph algorithms
- **[ternary-automata](https://github.com/SuperInstance/ternary-automata)** — three-state cellular automata
- **[ternary-compiler](https://github.com/SuperInstance/ternary-compiler)** — expression compiler and optimizer

200+ crates. 4,300+ tests. One pattern.

## Research Context

The ternary approach connects to several active research areas:
- **Ternary Neural Networks** (TNNs): weights constrained to {-1, 0, +1} for efficient inference
- **Huawei's ternary chip**: 7nm ternary silicon with 60% less power consumption
- **Active inference**: free energy minimization naturally maps to ternary action selection
- **Cyclic dominance**: RPS dynamics maintain biodiversity in spatial ecology
- **Z₃ group theory**: the only algebraic group on three elements is cyclic addition mod 3

## Usage

```toml
[dependencies]
ternary-search = "0.1.0"
```

```rust
use ternary_search;
```

## License

MIT
