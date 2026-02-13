# pepl-ui

The PEPL UI component model — deterministic surface tree rendering for PEPL programs.

**Status:** Phase 3 complete (Surface types + layout + content components). See [ROADMAP.md](ROADMAP.md) for progress.

## Components

| Component | Category | Status |
|-----------|----------|--------|
| Column | Layout | ✅ Done |
| Row | Layout | ✅ Done |
| Scroll | Layout | ✅ Done |
| Text | Content | ✅ Done |
| ProgressBar | Content | ✅ Done |
| Button | Interactive | Planned (Phase 4) |
| TextInput | Interactive | Planned (Phase 4) |
| ScrollList | Collection | Planned (Phase 5) |
| Modal | Overlay | Planned (Phase 6) |
| Toast | Overlay | Planned (Phase 6) |

## Tests

180 tests:
- Surface types & component registry: 42
- Layout components (Column, Row, Scroll): 83
- Content components (Text, ProgressBar): 55

## Key Design Choices

- **Deterministic:** Render budget caps computation (no infinite loops)
- **Typed builders:** Each component has a validated builder with typed props
- **Structural validation:** `validate_layout_node` checks prop types at build time
- **Edges coercion:** Uniform padding → per-side expansion

## Build

```bash
source "$HOME/.cargo/env"
cargo build
cargo test
cargo clippy -- -D warnings
```

## Cross-Repo Coordination

Part of the PEPL project alongside [`pepl`](https://github.com/PEPL-Lang/PEPL) (compiler) and [`pepl-stdlib`](https://github.com/PEPL-Lang/PEPL-STDLIB) (standard library).

## License

MIT
