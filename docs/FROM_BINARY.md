# From Binary to Ternary: Search

## The Trap

Binary search is drilled into every programmer: pick a midpoint, decide left or right, discard half the space. When your search space is a *strategy space* — decisions with three or more outcomes — binary search forces you to flatten your problem into a yes/no threshold. "Is the optimal price above $50?" What if the answer is "around $50" or "need more data"? Binary search has no vocabulary for "don't know yet."

Beam search and A\* face the same problem: their heuristic functions return a single scalar. Two paths with the same heuristic score are indistinguishable, even if one is clearly riskier. Binary evaluation flattens nuance.

## Map to Three States

| Domain | −1 | 0 | +1 |
|--------|----|---|-----|
| Threshold search | below threshold | at threshold | above threshold |
| Strategy fitness | unfit | uncertain | fit |
| A\* heuristic | worse than estimate | matches estimate | better than estimate |
| BFS/DFS frontier | blocked | queued | explored |

## From Binary to Ternary

**Before: binary search on a threshold**

```rust
fn find_threshold(values: &[f64], target: f64) -> usize {
    let mut lo = 0;
    let mut hi = values.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if values[mid] < target {
            lo = mid + 1;  // yes/no, move right
        } else {
            hi = mid;      // yes/no, move left
        }
    }
    lo
}
```

This works for sorted arrays. But when your decision space has three natural outcomes — buy/hold/sell, deploy/wait/rollback, accept/review/reject — binary search can't model the middle. You end up running two binary searches.

**After: ternary search over three-valued thresholds**

```rust
// StrategyNode has a ternary value: -1 (bad), 0 (undecided), +1 (good)
// StrategyGraph search returns the best path
fn search_strategy_space(graph: &StrategyGraph) -> Vec<StrategyNode> {
    // Beam search with ternary fitness
    // 0-valued nodes aren't discarded — they're held in the beam
    // They might flip to +1 with more information
}
```

The neutral state (`0`) is the key innovation. In binary search, a node with ambiguous evidence gets assigned to whichever side minimizes worst-case error — a guess. In ternary search, it stays in the `0` beam, waiting for evidence to resolve it. This is the **0 is not nothing** principle: the neutral state is an active holding zone, not a missing value.

**Before: A\* with scalar heuristics**

```rust
// f(n) = g(n) + h(n)
// Two nodes with f=10 are treated identically
// Even if one has g=2, h=8 (optimistic) and the other g=8, h=2 (pessimistic)
```

**After: ternary-weighted A\***

```rust
// Each node carries a ternary heuristic component
// -1: heuristic overestimates (pessimistic)
//  0: heuristic is uncertain
// +1: heuristic underestimates (optimistic)
// The search can dynamically trust or discount estimates
```

## Why It Matters

Ternary search doesn't discard information. The `0` state is a pressure valve for uncertainty — ambiguity doesn't get forced into a false binary choice. Beam search with neutral fitness holds candidates that might become good; A\* with ternary heuristics models confidence in its own estimates. The result is search that matches how real decision-making works: some options are bad, some are good, and some need more information.
