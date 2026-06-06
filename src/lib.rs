//! Search algorithms over ternary strategy spaces.
//!
//! Provides binary search on thresholds, BFS/DFS on strategy graphs,
//! beam search, and A* with fitness heuristics.

#![forbid(unsafe_code)]

use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::cmp::Ordering;

/// Canonical ternary type re-exported from `ternary-types`.
pub use ternary_types::Ternary;

/// Deprecated — use [`Ternary`] directly.
#[deprecated(since = "0.2.0", note = "use ternary_types::Ternary instead")]
pub type TernaryDeprecated = Ternary;

/// Extension trait providing the [`value()`](TernaryExt::value) method.
pub trait TernaryExt {
    /// Return the numeric value of this ternary state as an `i8`.
    fn value(&self) -> i8;
}

impl TernaryExt for ternary_types::Ternary {
    fn value(&self) -> i8 {
        i8::from(*self)
    }
}

/// A node in the strategy graph.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StrategyNode {
    pub id: usize,
    pub signals: Vec<Ternary>,
    pub neighbors: Vec<usize>,
}

impl StrategyNode {
    pub fn new(id: usize) -> Self {
        StrategyNode {
            id,
            signals: Vec::new(),
            neighbors: Vec::new(),
        }
    }

    pub fn with_signals(id: usize, signals: Vec<Ternary>) -> Self {
        StrategyNode { id, signals, neighbors: Vec::new() }
    }

    pub fn fitness(&self) -> i64 {
        self.signals.iter().map(|s| s.value() as i64).sum()
    }

    pub fn add_neighbor(&mut self, neighbor_id: usize) {
        if !self.neighbors.contains(&neighbor_id) {
            self.neighbors.push(neighbor_id);
        }
    }
}

/// A graph of strategy nodes.
#[derive(Debug, Clone)]
pub struct StrategyGraph {
    nodes: HashMap<usize, StrategyNode>,
    next_id: usize,
}

impl StrategyGraph {
    pub fn new() -> Self {
        StrategyGraph { nodes: HashMap::new(), next_id: 0 }
    }

    pub fn add_node(&mut self, signals: Vec<Ternary>) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.nodes.insert(id, StrategyNode::with_signals(id, signals));
        id
    }

    pub fn add_edge(&mut self, a: usize, b: usize) {
        if let Some(node) = self.nodes.get_mut(&a) {
            node.add_neighbor(b);
        }
        if let Some(node) = self.nodes.get_mut(&b) {
            node.add_neighbor(a);
        }
    }

    pub fn get(&self, id: usize) -> Option<&StrategyNode> {
        self.nodes.get(&id)
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn nodes(&self) -> impl Iterator<Item = &StrategyNode> {
        self.nodes.values()
    }
}

impl Default for StrategyGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// Binary search on a threshold function over ternary signals.
pub fn binary_threshold_search<F>(low: i64, high: i64, eval: F) -> i64
where
    F: Fn(i64) -> i64,
{
    let mut lo = low;
    let mut hi = high;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if eval(mid) >= 0 {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

/// Binary search finding the threshold where the evaluation crosses from negative to non-negative.
pub fn binary_search_first_nonneg<F>(low: i64, high: i64, eval: F) -> Option<i64>
where
    F: Fn(i64) -> bool,
{
    let mut lo = low;
    let mut hi = high;
    let mut result = None;
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        if eval(mid) {
            result = Some(mid);
            hi = mid - 1;
        } else {
            lo = mid + 1;
        }
    }
    result
}

/// BFS result.
#[derive(Debug, Clone)]
pub struct BfsResult {
    pub visited: Vec<usize>,
    pub distances: HashMap<usize, usize>,
    pub parents: HashMap<usize, Option<usize>>,
}

/// Breadth-first search on a strategy graph.
pub fn bfs(graph: &StrategyGraph, start: usize) -> BfsResult {
    let mut visited = Vec::new();
    let mut distances = HashMap::new();
    let mut parents = HashMap::new();
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    if graph.get(start).is_none() {
        return BfsResult { visited, distances, parents };
    }

    queue.push_back(start);
    seen.insert(start);
    distances.insert(start, 0);
    parents.insert(start, None);

    while let Some(current) = queue.pop_front() {
        visited.push(current);
        if let Some(node) = graph.get(current) {
            for &neighbor in &node.neighbors {
                if !seen.contains(&neighbor) && graph.get(neighbor).is_some() {
                    seen.insert(neighbor);
                    distances.insert(neighbor, distances[&current] + 1);
                    parents.insert(neighbor, Some(current));
                    queue.push_back(neighbor);
                }
            }
        }
    }

    BfsResult { visited, distances, parents }
}

/// DFS result.
#[derive(Debug, Clone)]
pub struct DfsResult {
    pub visited: Vec<usize>,
    pub discovery_order: Vec<usize>,
    pub finish_order: Vec<usize>,
}

/// Depth-first search on a strategy graph.
pub fn dfs(graph: &StrategyGraph, start: usize) -> DfsResult {
    let mut visited = Vec::new();
    let mut discovery = Vec::new();
    let mut finish = Vec::new();
    let mut seen = HashSet::new();

    fn visit(
        graph: &StrategyGraph,
        node_id: usize,
        seen: &mut HashSet<usize>,
        visited: &mut Vec<usize>,
        discovery: &mut Vec<usize>,
        finish: &mut Vec<usize>,
    ) {
        seen.insert(node_id);
        visited.push(node_id);
        discovery.push(node_id);
        if let Some(node) = graph.get(node_id) {
            for &neighbor in &node.neighbors {
                if !seen.contains(&neighbor) && graph.get(neighbor).is_some() {
                    visit(graph, neighbor, seen, visited, discovery, finish);
                }
            }
        }
        finish.push(node_id);
    }

    if graph.get(start).is_some() {
        visit(graph, start, &mut seen, &mut visited, &mut discovery, &mut finish);
    }

    DfsResult { visited, discovery_order: discovery, finish_order: finish }
}

/// A candidate in beam search.
#[derive(Debug, Clone)]
pub struct BeamCandidate {
    pub node_id: usize,
    pub score: i64,
    pub path: Vec<usize>,
}

/// Beam search over a strategy graph.
pub fn beam_search(
    graph: &StrategyGraph,
    start: usize,
    beam_width: usize,
    max_depth: usize,
    score_fn: impl Fn(&StrategyNode) -> i64,
) -> Vec<BeamCandidate> {
    if graph.get(start).is_none() {
        return Vec::new();
    }

    let mut beam = vec![BeamCandidate {
        node_id: start,
        score: score_fn(graph.get(start).unwrap()),
        path: vec![start],
    }];

    let mut best = beam.clone();

    for _ in 0..max_depth {
        let mut candidates = Vec::new();
        for candidate in &beam {
            if let Some(node) = graph.get(candidate.node_id) {
                for &neighbor in &node.neighbors {
                    if let Some(nnode) = graph.get(neighbor) {
                        let mut new_path = candidate.path.clone();
                        new_path.push(neighbor);
                        candidates.push(BeamCandidate {
                            node_id: neighbor,
                            score: candidate.score + score_fn(nnode),
                            path: new_path,
                        });
                    }
                }
            }
        }

        if candidates.is_empty() {
            break;
        }

        // Sort descending by score, keep top beam_width
        candidates.sort_by(|a, b| b.score.cmp(&a.score));
        candidates.truncate(beam_width);
        beam = candidates;

        // Track best
        for c in &beam {
            if c.score > best.iter().map(|b| b.score).max().unwrap_or(i64::MIN) {
                best = vec![c.clone()];
            } else if c.score == best.iter().map(|b| b.score).max().unwrap_or(i64::MIN) {
                best.push(c.clone());
            }
        }
    }

    best
}

/// A* search node.
#[derive(Debug, Clone, Eq, PartialEq)]
struct AStarNode {
    node_id: usize,
    g_cost: i64,
    f_cost: i64,
    path: Vec<usize>,
}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_cost.cmp(&self.f_cost)
    }
}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A* search result.
#[derive(Debug, Clone)]
pub struct AStarResult {
    pub path: Vec<usize>,
    pub cost: i64,
    pub explored: usize,
}

/// A* search on a strategy graph with fitness heuristic.
pub fn astar(
    graph: &StrategyGraph,
    start: usize,
    goal: usize,
    heuristic: impl Fn(usize) -> i64,
    edge_cost: impl Fn(usize, usize) -> i64,
) -> Option<AStarResult> {
    if graph.get(start).is_none() || graph.get(goal).is_none() {
        return None;
    }

    let mut open = BinaryHeap::new();
    let mut g_scores: HashMap<usize, i64> = HashMap::new();
    let mut closed = HashSet::new();
    let mut explored = 0;

    g_scores.insert(start, 0);
    open.push(AStarNode {
        node_id: start,
        g_cost: 0,
        f_cost: heuristic(start),
        path: vec![start],
    });

    while let Some(current) = open.pop() {
        if current.node_id == goal {
            return Some(AStarResult {
                path: current.path,
                cost: current.g_cost,
                explored,
            });
        }

        if closed.contains(&current.node_id) {
            continue;
        }
        closed.insert(current.node_id);
        explored += 1;

        if let Some(node) = graph.get(current.node_id) {
            for &neighbor in &node.neighbors {
                if closed.contains(&neighbor) || graph.get(neighbor).is_none() {
                    continue;
                }
                let tentative_g = current.g_cost + edge_cost(current.node_id, neighbor);
                let prev_g = g_scores.get(&neighbor).copied().unwrap_or(i64::MAX);
                if tentative_g < prev_g {
                    g_scores.insert(neighbor, tentative_g);
                    let mut new_path = current.path.clone();
                    new_path.push(neighbor);
                    open.push(AStarNode {
                        node_id: neighbor,
                        g_cost: tentative_g,
                        f_cost: tentative_g + heuristic(neighbor),
                        path: new_path,
                    });
                }
            }
        }
    }

    None
}

/// Find shortest path using BFS.
pub fn shortest_path(graph: &StrategyGraph, start: usize, goal: usize) -> Option<Vec<usize>> {
    let result = bfs(graph, start);
    if !result.distances.contains_key(&goal) {
        return None;
    }
    let mut path = vec![goal];
    let mut current = goal;
    while let Some(Some(parent)) = result.parents.get(&current) {
        path.push(*parent);
        current = *parent;
    }
    path.reverse();
    Some(path)
}

/// Check if two nodes are connected.
pub fn is_connected(graph: &StrategyGraph, a: usize, b: usize) -> bool {
    bfs(graph, a).distances.contains_key(&b)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_linear_graph() -> StrategyGraph {
        let mut g = StrategyGraph::new();
        let n0 = g.add_node(vec![Ternary::Positive]);
        let n1 = g.add_node(vec![Ternary::Neutral]);
        let n2 = g.add_node(vec![Ternary::Negative]);
        let n3 = g.add_node(vec![Ternary::Positive, Ternary::Positive]);
        g.add_edge(n0, n1);
        g.add_edge(n1, n2);
        g.add_edge(n2, n3);
        g
    }

    #[test]
    fn test_graph_construction() {
        let g = make_linear_graph();
        assert_eq!(g.node_count(), 4);
    }

    #[test]
    fn test_node_fitness() {
        let node = StrategyNode::with_signals(0, vec![Ternary::Positive, Ternary::Negative, Ternary::Positive]);
        assert_eq!(node.fitness(), 1);
    }

    #[test]
    fn test_bfs_linear() {
        let g = make_linear_graph();
        let result = bfs(&g, 0);
        assert_eq!(result.visited, vec![0, 1, 2, 3]);
        assert_eq!(result.distances[&0], 0);
        assert_eq!(result.distances[&1], 1);
        assert_eq!(result.distances[&3], 3);
    }

    #[test]
    fn test_bfs_missing_node() {
        let g = make_linear_graph();
        let result = bfs(&g, 99);
        assert!(result.visited.is_empty());
    }

    #[test]
    fn test_dfs_linear() {
        let g = make_linear_graph();
        let result = dfs(&g, 0);
        assert_eq!(result.visited, vec![0, 1, 2, 3]);
        assert_eq!(result.finish_order.last(), Some(&0)); // starts first, finishes last
    }

    #[test]
    fn test_shortest_path() {
        let g = make_linear_graph();
        let path = shortest_path(&g, 0, 3).unwrap();
        assert_eq!(path, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_shortest_path_unreachable() {
        let mut g = StrategyGraph::new();
        let a = g.add_node(vec![Ternary::Positive]);
        let b = g.add_node(vec![Ternary::Negative]);
        // no edge
        assert!(shortest_path(&g, a, b).is_none());
    }

    #[test]
    fn test_is_connected() {
        let g = make_linear_graph();
        assert!(is_connected(&g, 0, 3));
        assert!(is_connected(&g, 3, 0));
    }

    #[test]
    fn test_binary_threshold_search() {
        let result = binary_threshold_search(0, 100, |x| if x >= 42 { 0 } else { -1 });
        assert_eq!(result, 42);
    }

    #[test]
    fn test_binary_search_first_nonneg() {
        let result = binary_search_first_nonneg(0, 100, |x| x >= 50);
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_binary_search_first_nonneg_none() {
        let result = binary_search_first_nonneg(0, 100, |_| false);
        assert_eq!(result, None);
    }

    #[test]
    fn test_beam_search() {
        let g = make_linear_graph();
        let results = beam_search(&g, 0, 2, 3, |n| n.fitness());
        assert!(!results.is_empty());
        assert!(results[0].path.contains(&0));
    }

    #[test]
    fn test_beam_search_empty() {
        let g = StrategyGraph::new();
        let results = beam_search(&g, 0, 2, 3, |n| n.fitness());
        assert!(results.is_empty());
    }

    #[test]
    fn test_astar() {
        let g = make_linear_graph();
        let result = astar(&g, 0, 3, |_| 0, |_, _| 1).unwrap();
        assert_eq!(result.path, vec![0, 1, 2, 3]);
        assert_eq!(result.cost, 3);
    }

    #[test]
    fn test_astar_no_path() {
        let mut g = StrategyGraph::new();
        let a = g.add_node(vec![Ternary::Positive]);
        let b = g.add_node(vec![Ternary::Negative]);
        let result = astar(&g, a, b, |_| 0, |_, _| 1);
        assert!(result.is_none());
    }

    #[test]
    fn test_astar_with_heuristic() {
        let g = make_linear_graph();
        let result = astar(&g, 0, 3, |n| (3 - n) as i64, |_, _| 1).unwrap();
        assert_eq!(result.path, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_ternary_value() {
        assert_eq!(Ternary::Positive.value(), 1);
        assert_eq!(Ternary::Negative.value(), -1);
        assert_eq!(Ternary::Neutral.value(), 0);
    }

    #[test]
    fn test_graph_add_node_returns_ids() {
        let mut g = StrategyGraph::new();
        let a = g.add_node(vec![]);
        let b = g.add_node(vec![]);
        assert_ne!(a, b);
    }

    #[test]
    fn test_cyclic_graph_bfs() {
        let mut g = StrategyGraph::new();
        let a = g.add_node(vec![Ternary::Positive]);
        let b = g.add_node(vec![Ternary::Negative]);
        let c = g.add_node(vec![Ternary::Neutral]);
        g.add_edge(a, b);
        g.add_edge(b, c);
        g.add_edge(c, a);
        let result = bfs(&g, a);
        assert_eq!(result.visited.len(), 3);
    }
}
