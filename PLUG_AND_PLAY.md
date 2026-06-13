# PLUG_AND_PLAY — Search

> Search algorithms using ternary state spaces and strategy trees

## 🚀 Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
ternary-search = { git = "https://github.com/SuperInstance/ternary-search" }
```

Use in your code:

```rust
use ternary_search::StrategyTree;

let mut tree = StrategyTree::new(3);
tree.expand();
let best = tree.search();
```

## 📚 Available Documentation

| Document | Description |
|----------|-------------|
| `docs/FROM_BINARY.md` | Understanding ternary concepts as a binary programmer |
| `docs/MIGRATION.md` | Version migration guide |
| `docs/FUTURE-INTEGRATION.md` | Planned features and roadmap |

## 🔗 Integration

This crate is part of the [SuperInstance ternary fleet](https://github.com/SuperInstance). It uses the canonical `Ternary` type from `ternary-types` for cross-crate compatibility.

## 📄 License

MIT
