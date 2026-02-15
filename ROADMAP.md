# ROADMAP — pepl-ui (UI Component Model)

> 10 Phase 0 components: platform-abstract, accessibility-first, deterministic rendering.
> Components produce Surface trees — the host's View Layer renders them.
> See `ORCHESTRATION.md` in the [`.github`](https://github.com/PEPL-Lang/.github) repo for cross-repo sequencing.

---

## Phase 1: Project Scaffolding & Surface Tree Types

### 1.1 Cargo Project Setup
- [x] Create Cargo library crate `pepl-ui`
- [x] Configure dependencies: `serde`, `serde_json`
- [x] Define `Surface` type (the abstract UI tree)
- [x] Define `SurfaceNode` (type, props, children)
- [x] Define `PropValue` enum (String, Number, Bool, Color, ActionRef, Lambda, List, Record, Nil)
- [x] Define serialization to JSON (matching the host WASM contract view tree format)
- [x] Workspace-level `cargo build` succeeds

### 1.2 Component Registry
- [x] Define `ComponentDef` trait (name, required props, optional props, accepts children)
- [x] Create component registry with all 10 Phase 0 components
- [x] Component name validation: unknown names → error E402
- [x] Unit tests for registry lookup and E402 error

### 1.3 Shared Types
- [x] Define `dimension` type: `Px(number) | Auto | Fill | Percent(number)`
- [x] Define `edges` type: `Uniform(number) | Sides(top, bottom, start, end)`
- [x] Define `alignment` type: `Start | Center | End | Stretch | SpaceBetween | SpaceAround`
- [x] Define `border_style` type: `{ width, color, style? }`
- [x] Define `shadow_style` type: `{ offset_x, offset_y, blur, color }`
- [x] Number literal coercion: `width: 100` → `Px(100)`, `padding: 16` → `Uniform(16)`
- [x] Unit tests for type coercions
- [x] Surface schema freeze test: serialize a representative Surface tree to JSON, assert byte-for-byte stability

> **API FREEZE:** After Phase 1, the `Surface`, `SurfaceNode`, and `PropValue` types are **frozen**. All subsequent phases add component definitions — they do not change the Surface tree structure. This stability is critical because golden references (M3) and WASM validation (M4) depend on identical serialization.

---

## Phase 2: Layout Components

### 2.1 `Column` Component
- [x] Props: `spacing?: number`, `align?: alignment`, `padding?: edges`
- [x] Accepts children
- [x] Serializes to Surface node with correct prop types
- [x] Unit tests (required props, optional props, children)

### 2.2 `Row` Component
- [x] Props: `spacing?: number`, `align?: alignment`, `padding?: edges`
- [x] Accepts children
- [x] Unit tests

### 2.3 `Scroll` Component
- [x] Props: `direction?: "vertical" | "horizontal" | "both"`
- [x] Accepts children
- [x] Default direction: "vertical"
- [x] Unit tests

### 2.4 Layout Tests
- [x] Test nested Column/Row combinations
- [x] Test all alignment values
- [x] Test edges coercion (number → Uniform, explicit Sides)
- [x] 100-iteration determinism test

---

## Phase 3: Content Components

### 3.1 `Text` Component
- [x] Props: `value: string` (required), `size?: "small"|"body"|"title"|"heading"|"display"`, `weight?: "normal"|"medium"|"bold"`, `color?: color`, `align?: "start"|"center"|"end"`, `max_lines?: number`, `overflow?: "clip"|"ellipsis"|"wrap"`
- [x] No children
- [x] Type-safe enums: TextSize, TextWeight, TextAlign, TextOverflow
- [x] TextBuilder with fluent API
- [x] Validation: required prop check, enum value check, type check, unknown prop check, no-children check
- [x] Unit tests for all prop variants
- [x] JSON roundtrip + determinism tests

### 3.2 `ProgressBar` Component
- [x] Props: `value: number` (required, 0.0–1.0), `color?: color`, `background?: color`, `height?: number`
- [x] Values outside 0.0–1.0 are clamped
- [x] No children
- [x] ProgressBarBuilder with fluent API
- [x] Validation: required prop check, type check, unknown prop check, no-children check
- [x] Unit tests (within range, clamping, color props)

### 3.3 Content Tests
- [x] 55 content component tests
- [x] 100-iteration determinism tests for both Text and ProgressBar
- [x] Validation tests (missing required, wrong types, unknown props, multiple errors)
- [x] `cargo clippy` clean

---

## Phase 4: Interactive Components

### 4.1 `Button` Component
- [x] Props: `label: string` (required), `on_tap: () -> nil` (required), `variant?: "filled"|"outlined"|"text"`, `icon?: string`, `disabled?: bool`, `loading?: bool`
- [x] Action reference handling: `on_tap: action_name`, `on_tap: action_name(arg)`
- [x] No children
- [x] Unit tests (action binding, variants, disabled state)

### 4.2 `TextInput` Component
- [x] Props: `value: string` (required), `on_change: (string) -> nil` (required), `placeholder?: string`, `label?: string`, `keyboard?: "text"|"number"|"email"|"phone"|"url"`, `max_length?: number`, `multiline?: bool`
- [x] Action reference / lambda callback handling for on_change
- [x] No children
- [x] Unit tests

### 4.3 Interactive Tests
- [x] Test action reference serialization in props
- [x] Test lambda callback serialization
- [x] 100-iteration determinism test

---

## Phase 5: List & Data Components

### 5.1 `ScrollList` Component
- [x] Props: `items: list<T>` (required), `render: (T, number) -> Surface` (required), `key: (T) -> string` (required), `on_reorder?: (list<T>) -> nil`, `dividers?: bool`
- [x] Accepts render function (lambda) for item rendering
- [x] Unit tests (item rendering, key function, dividers)

### 5.2 List Tests
- [x] Test ScrollList with various item types (15 tests: construction, props, JSON round-trip, validation, determinism)
- [x] 100-iteration determinism test

---

## Phase 6: Feedback & Overlay Components

### 6.1 `Modal` Component
- [x] Props: `visible: bool` (required), `on_dismiss: () -> nil` (required), `title?: string`
- [x] Accepts children via second brace block: `Modal { props } { children }`
- [x] Unit tests (visible toggling, children serialization)

### 6.2 `Toast` Component
- [x] Props: `message: string` (required), `duration?: number`, `type?: "info"|"success"|"warning"|"error"`
- [x] No children
- [x] Unit tests

### 6.3 Feedback Tests
- [x] Test Modal with children (25 tests: construction, round-trip, validation, determinism)
- [x] 100-iteration determinism test

---

## Phase 7: Accessibility

### 7.1 Accessibility Primitives
- [x] Define `accessible()` function: `label: string`, `hint?: string`, `role?: string`, `value?: string`, `live_region?: "polite"|"assertive"`
- [x] Attach accessibility attributes to all 10 Phase 0 components
- [x] Default accessibility: auto-generate labels from content where possible (e.g., Button label → accessible label)
- [x] Unit tests for accessibility attribute serialization

### 7.2 Accessibility Roles
- [x] Map each component to default semantic role: Button→"button", TextInput→"textfield", etc.
- [x] Allow role override via `accessible()` prop
- [x] Unit tests for role mapping

---

## Phase 8: Final Validation

### 8.1 Integration Tests
- [x] All 10 components serialize to valid Surface JSON
- [x] Surface tree from canonical examples matches expected output
- [x] All components render in < 16ms budget
- [x] Component prop validation: wrong types produce clear errors

### 8.2 Final Checks
- [x] Full 100-iteration determinism test across all components
- [x] `cargo clippy -- -D warnings` clean
- [x] `cargo fmt --check` clean
- [x] README.md with component reference and architecture overview
- [x] Every component has built-in accessibility support verified
