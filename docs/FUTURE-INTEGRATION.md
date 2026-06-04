# Future Integration: ternary-search

## Current State
Provides search algorithms over ternary strategy spaces: binary search on thresholds, BFS/DFS on strategy graphs (`StrategyNode` with ternary signals), beam search, and A* with fitness heuristics. `StrategyNode` has an ID, signal vector, neighbor list, and fitness score.

## Integration Opportunities

### With ternary-cell (Optimal Room Transitions)
When an agent moves between rooms in PLATO, it needs to find the optimal path through the room graph. ternary-search's A* algorithm operates on `StrategyNode` graphs where each node is a room and edges are transitions. The fitness heuristic estimates room suitability for the agent's current task. Beam search finds near-optimal room sequences without exhaustive search — critical when the room graph has hundreds of nodes.

### With ternary-topology (Landscape Navigation)
ternary-topology classifies landscape features (Peak, Valley, Saddle, Plateau). ternary-search navigates this landscape: hill-climbing toward peaks, escaping valleys, crossing saddles. A* with topology-aware heuristics uses landscape classification to guide search — saddle points are waypoints, peaks are destinations, valleys are obstacles.

### With ternary-scheduling (Task Search)
ternary-scheduling needs to find optimal task orderings. ternary-search's beam search explores the combinatorial space of task sequences, pruning low-fitness orderings. Each node is a partial schedule, signals represent remaining tasks, and fitness combines deadline adherence with priority weighting.

## Potential in Mature Systems
In room-as-codespace, ternary-search becomes the room discovery engine. When PLATO needs to find which room can handle a user query, it searches the room registry using beam search — each node is a room, fitness is estimated relevance, and the beam width controls exploration breadth. For the ESP32, ternary-search operates on the 81-entry lookup table as a tiny strategy graph, finding the best response via BFS from the current state.

## Cross-Pollination Ideas
- **ternary-transfer**: Transfer learning across rooms — search finds which source rooms' knowledge transfers best to the target room, using domain gap as the fitness function.
- **ternary-curriculum**: Curriculum design as search — find the optimal lesson sequence by searching the curriculum graph with fitness = learning efficiency.
- **ternary-kalman**: Kalman-filtered search — use state estimates to predict which nodes are worth expanding, reducing search effort.

## Dependencies for Next Steps
- Define `RoomGraph` as `StrategyNode` graph in PLATO room registry
- Add fitness heuristics based on room capability matching
- Implement beam search for room discovery with configurable beam width
- Benchmark A* on ESP32 for real-time strategy lookup
