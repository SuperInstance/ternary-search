# ternary-search: Graph search algorithms over ternary strategy spaces

BFS, DFS, A*, beam search, shortest path, and binary threshold search over graphs where nodes carry ternary signals and edges connect strategy states.

## Why This Exists

When strategy spaces are large and structured as graphs (states connected by transitions), you need search algorithms to find good paths, optimal solutions, or just explore the space efficiently. This crate implements classic graph algorithms on a `StrategyGraph` where each node stores a vector of ternary signals and edges connect to neighbor nodes. The ternary signals provide a natural fitness function (sum of signal values) that drives heuristic search.

## Core Concepts

- **Ternary** — A value: `Positive` (+1), `Negative` (−1), or `Neutral` (0).
- **StrategyNode** — A graph node with an id, a vector of ternary signals, and a list of neighbor ids. Fitness is the sum of signal values.
- **StrategyGraph** — An adjacency-list graph of `StrategyNode` values. Nodes are stored in a `HashMap<usize, StrategyNode>`. Edges are bidirectional.
- **BFS (breadth-first search)** — Explores nodes layer by layer. Returns visited order, distances from start, and parent pointers. Used for shortest path in unweighted graphs.
- **DFS (depth-first search)** — Explores as deep as possible before backtracking. Returns visited order, discovery order, and finish order (useful for topological analysis).
- **Beam search** — A bounded best-first search. At each depth, keeps only the top `beam_width` candidates by cumulative score. Explores greedily but with limited memory.
- **A\*** — Optimal pathfinding using `f(n) = g(n) + h(n)` where g is the actual cost from start and h is a user-provided heuristic. Returns shortest path and cost.
- **Binary threshold search** — Finds the threshold value where an evaluation function transitions from negative to non-negative.

## Quick Start

```toml
# Cargo.toml
[dependencies]
ternary-search = "0.1"
```

```rust
use ternary_search::*;

// Build a strategy graph
let mut graph = StrategyGraph::new();
let a = graph.add_node(vec![Ternary::Positive]);
let b = graph.add_node(vec![Ternary::Neutral]);
let c = graph.add_node(vec![Ternary::Negative]);
let d = graph.add_node(vec![Ternary::Positive, Ternary::Positive]);
graph.add_edge(a, b);
graph.add_edge(b, c);
graph.add_edge(c, d);

// BFS: explore from node a
let bfs_result = bfs(&graph, a);
assert_eq!(bfs_result.visited, vec![a, b, c, d]);
assert_eq!(bfs_result.distances[&d], 3);

// Shortest path
let path = shortest_path(&graph, a, d).unwrap();
assert_eq!(path, vec![a, b, c, d]);

// A* with a simple heuristic
let result = astar(&graph, a, d, |node_id| (d - node_id) as i64, |_, _| 1).unwrap();
assert_eq!(result.path, vec![a, b, c, d]);
assert_eq!(result.cost, 3);

// Beam search
let best = beam_search(&graph, a, 2, 3, |n| n.fitness());
assert!(!best.is_empty());

// Binary threshold search
let threshold = binary_threshold_search(0, 100, |x| if x >= 42 { 0 } else { -1 });
assert_eq!(threshold, 42);
```

## API Overview

| Type / Function | What it is |
|---|---|
| `Ternary` | Enum: `Positive`, `Negative`, `Neutral` |
| `StrategyNode` | Graph node with signals and neighbors |
| `StrategyGraph` | Adjacency-list graph indexed by node id |
| `bfs` | Breadth-first search; returns visited, distances, parents |
| `dfs` | Depth-first search; returns visited, discovery, finish orders |
| `shortest_path` | BFS-based unweighted shortest path |
| `is_connected` | Check if two nodes are reachable |
| `beam_search` | Bounded best-first search by cumulative fitness |
| `astar` | A* pathfinding with heuristic and edge cost functions |
| `binary_threshold_search` | Find threshold where evaluation crosses zero |
| `binary_search_first_nonneg` | Find first value where predicate is true |

## How It Works

**Graph construction.** Nodes are assigned sequential IDs starting from 0. `add_node` returns the new node's id. `add_edge` adds bidirectional neighbor links (both directions). Duplicate edges are ignored.

**BFS.** Uses `VecDeque` as a queue. Starts from the given node, visits neighbors level by level, and records distance and parent for each visited node. Returns empty results if the start node doesn't exist.

**DFS.** Uses recursive traversal. Records three orderings: visited (preorder), discovery (same as visited), and finish (postorder—when all descendants have been explored).

**Beam search.** Starts with the initial node as the only candidate. At each depth, expands all candidates by visiting their neighbors, scores the new candidates by cumulative fitness (sum of node fitness values along the path), sorts descending, and keeps only the top `beam_width`. Tracks the overall best across all depths.

**A\*.** Uses a `BinaryHeap` (max-heap with inverted ordering to simulate min-heap on f-cost). Maintains a closed set and g-scores. For each neighbor, computes tentative g = current.g + edge_cost. If better than the stored g, updates and pushes to the open set. Returns when the goal is popped.

**Binary search.** Two variants: `binary_threshold_search` finds the threshold where `eval(x) >= 0`, and `binary_search_first_nonneg` finds the first x where `predicate(x)` is true. Both use standard binary search over the [low, high] integer range.

## Known Limitations

- **Beam search is not optimal.** Beam search prunes candidates at each level, so it can miss the globally best path. It's a heuristic approximation, not a guarantee.
- **No edge weights in BFS/DFS.** BFS assumes uniform edge cost (1). For weighted graphs, use A* with an appropriate edge cost function.
- **A* requires admissible heuristic.** If the heuristic overestimates the true cost, A* may return a non-optimal path. Correctness is the caller's responsibility.
- **Graph is bidirectional only.** `add_edge` creates edges in both directions. There's no directed graph mode.
- **Beam search tracks best across all depths, but may duplicate.** The `best` list can contain multiple entries with the same score. No deduplication by path.

## Use Cases

- **Strategy exploration.** Build a graph of strategy states (each state is a ternary signal vector). Use beam search to find high-fitness strategies without exploring the full exponential space.
- **Path planning with ternary costs.** Nodes represent locations with ternary terrain ratings (good/neutral/bad). A* finds the lowest-cost path.
- **Threshold calibration.** Use `binary_threshold_search` to find the minimum signal strength where a ternary strategy becomes profitable.

## Ecosystem Context

Feeds into `ternary-scoring` (search results become scoring candidates) and `ternary-validation` (validate paths found by search). `ternary-grammar` expressions may define the fitness functions used by beam search. No direct dependencies on other ternary crates.

## License

MIT

## See Also
- **ternary-planning** — related
- **ternary-optimization** — related
- **ternary-gradient** — related
- **ternary-compass** — related
- **ternary-constraint** — related

