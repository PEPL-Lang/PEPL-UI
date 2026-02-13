# pepl-ui

The PEPL UI component model — deterministic surface tree rendering for PEPL programs.

**Status:** Phase 6 complete (all 10 Phase 0 components implemented). See [ROADMAP.md](ROADMAP.md) for progress.

## Components

| Component | Category | Status |
|-----------|----------|--------|
| Column | Layout | ✅ Done |
| Row | Layout | ✅ Done |
| Scroll | Layout | ✅ Done |
| Text | Content | ✅ Done |
| ProgressBar | Content | ✅ Done |
| Button | Interactive | ✅ Done |
| TextInput | Interactive | ✅ Done |
| ScrollList | Collection | ✅ Done |
| Modal | Overlay | ✅ Done |
| Toast | Overlay | ✅ Done |

## Tests

278 tests:
- Surface types & component registry: 42
- Layout components (Column, Row, Scroll): 83
- Content components (Text, ProgressBar): 55
- Interactive components (Button, TextInput): 56
- Collection components (ScrollList): 15
- Feedback components (Modal, Toast): 25
- Doc-tests: 2

## Key Design Choices

- **Deterministic:** Render budget caps computation (no infinite loops)
- **Typed builders:** Each component has a validated builder with typed props
- **Structural validation:** `validate_layout_node`, `validate_content_node`, `validate_interactive_node`, `validate_list_node`, `validate_feedback_node` check prop types at build time
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
