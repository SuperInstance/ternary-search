# ternary-search

Search algorithms over **ternary strategy spaces**. Provides binary threshold search, BFS/DFS on strategy graphs, beam search, and A* with fitness heuristics — all operating on graphs where nodes carry ternary signal vectors.

## Why It Matters

Strategy spaces in multi-agent systems are naturally ternary: each signal dimension is `+1` (positive), `0` (neutral), or `-1` (negative). Searching these spaces requires algorithms that understand the structure:

| Algorithm | Use Case |
|-----------|----------|
| Binary threshold search | Find the crossover point where a monotone evaluation changes sign |
| BFS / DFS | Exhaustive exploration of the strategy graph |
| Beam search | Bounded-width heuristic search (keep top-k) |
| A* | Optimal pathfinding with admissible heuristic |
| Shortest path | BFS-based minimum-hop distance |

## How It Works

### Strategy Graph

A `StrategyGraph` is an adjacency-list graph where each `StrategyNode` carries:
- `id`: unique identifier
- `signals: Vec<Ternary>` — the ternary state vector
- `neighbors: Vec<usize>` — adjacent node IDs

Node fitness is the sum of signal values:

```
fitness(node) = Σ signalᵢ = Σ value(tᵢ)   where tᵢ ∈ {-1, 0, +1}
```

### Binary Threshold Search

Finds the smallest integer *x* ∈ [low, high] where `eval(x) ≥ 0`:

```
while lo < hi:
    mid = lo + (hi - lo) / 2
    if eval(mid) ≥ 0: hi = mid
    else: lo = mid + 1
```

**Complexity:** O(log(hi - lo)) evaluations. **Space:** O(1).

### BFS and DFS

Standard breadth-first and depth-first traversal. BFS computes shortest-hop distances; DFS produces discovery/finish timestamps for topological analysis.

```
BFS: visit all neighbors at distance d before d+1
DFS: visit deepest unvisited neighbor first
```

**Complexity:** O(V + E) time, O(V) space.

### Beam Search

Maintains at most *B* (beam width) candidates at depth *d*. At each step:

1. Expand all candidates' neighbors → candidate pool
2. Score each: `score(child) = score(parent) + fitness(child)`
3. Keep top *B* by score

```
beam = {start}
for depth in 0..max_depth:
    candidates = { expand(c) for c in beam }
    beam = top_B(candidates, by score)
    track best
```

**Complexity:** O(B · d̄ · B) = O(B² · d̄) per depth, where d̄ is average degree. Total: O(B² · D · d̄).

**Trade-off:** Larger *B* → better solutions but exponentially more computation. *B = 1* is greedy; *B → ∞* is BFS.

### A* Search

A* finds the minimum-cost path from start to goal using:

```
f(n) = g(n) + h(n)
```

where `g(n)` is the actual cost to reach *n*, and `h(n)` is the heuristic estimate to the goal. With an **admissible** heuristic (never overestimates), A* is guaranteed to find the optimal path.

The implementation uses a `BinaryHeap` (min-heap via inverted `Ord`) with a closed set for visited nodes.

**Complexity:** O((V + E) log V) time with heap, O(V) space. Worst case degrades to Dijkstra if `h ≡ 0`.

### Shortest Path and Connectivity

Shortest path uses BFS parent tracking and backtracks from goal to start:

```
path = backtrack(goal → start via parent[])
```

`is_connected(a, b)` runs BFS from *a* and checks if *b* is reached. O(V + E).

## Quick Start

```rust
use ternary_search::{StrategyGraph, bfs, dfs, beam_search, astar, Ternary};
use ternary_types::Ternary as T;

let mut g = StrategyGraph::new();
let n0 = g.add_node(vec![T::Positive]);
let n1 = g.add_node(vec![T::Neutral]);
let n2 = g.add_node(vec![T::Negative, T::Positive]);
g.add_edge(n0, n1);
g.add_edge(n1, n2);

// BFS
let result = bfs(&g, n0);
assert_eq!(result.distances[&n2], 2);

// A*
let result = astar(&g, n0, n2,
    |_| 0,           // heuristic
    |_, _| 1,        // edge cost
).unwrap();
assert_eq!(result.path, vec![n0, n1, n2]);
```

## API

| Function | Description |
|----------|-------------|
| `binary_threshold_search(lo, hi, eval)` | Find first non-negative crossover |
| `binary_search_first_nonneg(lo, hi, pred)` | First index satisfying predicate |
| `bfs(graph, start)` | Breadth-first traversal |
| `dfs(graph, start)` | Depth-first traversal |
| `beam_search(graph, start, width, depth, score_fn)` | Beam search |
| `astar(graph, start, goal, h, edge_cost)` | A* pathfinding |
| `shortest_path(graph, start, goal)` | BFS-based shortest path |
| `is_connected(graph, a, b)` | Reachability check |

## Architecture Notes

The **γ + η = C** invariant appears in beam search: *generation* (γ) is the expansion of candidates (exploration), *entropy* (η) is the diversity of the beam (how different the *B* candidates are from each other), and *conservation* (C) is the fixed beam width *B* — the constraint that exactly *B* candidates survive each round. The truncation step enforces conservation by pruning candidates, converting generation-driven diversity into entropy-reducing selection.

## References

- **A* algorithm:** Hart, P., Nilsson, N. & Raphael, B. "A Formal Basis for the Heuristic Determination" (1968)
- **Beam search:** Bisiani, R. "Beam Search" in *Encyclopedia of AI* (1987)
- **Strategy graphs:** Marschak, J. & Radner, R. *Economic Theory of Teams* (1972)
- **BFS/DFS:** Cormen, T. et al. *Introduction to Algorithms* (2009), Chapters 22–24

## License

MIT
