# ROADMAP — pepl-ui (UI Component Model)

> 10 Phase 0 components: platform-abstract, accessibility-first, deterministic rendering.
> Components produce Surface trees — the host's View Layer renders them.
> See `ORCHESTRATION.md` for cross-repo sequencing.

---

## Phase 1: Project Scaffolding & Surface Tree Types

### 1.1 Cargo Project Setup
- [ ] Create Cargo library crate `pepl-ui`
- [ ] Configure dependencies: `serde`, `serde_json`
- [ ] Define `Surface` type (the abstract UI tree)
- [ ] Define `SurfaceNode` (type, props, children)
- [ ] Define `PropValue` enum (String, Number, Bool, Color, ActionRef, Lambda, List, Record, Nil)
- [ ] Define serialization to JSON (matching the host WASM contract view tree format)
- [ ] Workspace-level `cargo build` succeeds

### 1.2 Component Registry
- [ ] Define `ComponentDef` trait (name, required props, optional props, accepts children)
- [ ] Create component registry with all 10 Phase 0 components
- [ ] Component name validation: unknown names → error E402
- [ ] Unit tests for registry lookup and E402 error

### 1.3 Shared Types
- [ ] Define `dimension` type: `Px(number) | Auto | Fill | Percent(number)`
- [ ] Define `edges` type: `Uniform(number) | Sides(top, bottom, start, end)`
- [ ] Define `alignment` type: `Start | Center | End | Stretch | SpaceBetween | SpaceAround`
- [ ] Define `border_style` type: `{ width, color, style? }`
- [ ] Define `shadow_style` type: `{ offset_x, offset_y, blur, color }`
- [ ] Number literal coercion: `width: 100` → `Px(100)`, `padding: 16` → `Uniform(16)`
- [ ] Unit tests for type coercions

---

## Phase 2: Layout Components

### 2.1 `Column` Component
- [ ] Props: `spacing?: number`, `align?: alignment`, `padding?: edges`
- [ ] Accepts children
- [ ] Serializes to Surface node with correct prop types
- [ ] Unit tests (required props, optional props, children)

### 2.2 `Row` Component
- [ ] Props: `spacing?: number`, `align?: alignment`, `padding?: edges`
- [ ] Accepts children
- [ ] Unit tests

### 2.3 `Scroll` Component
- [ ] Props: `direction?: "vertical" | "horizontal" | "both"`
- [ ] Accepts children
- [ ] Default direction: "vertical"
- [ ] Unit tests

### 2.4 Layout Tests
- [ ] Test nested Column/Row combinations
- [ ] Test all alignment values
- [ ] Test edges coercion (number → Uniform, explicit Sides)
- [ ] 100-iteration determinism test

---

## Phase 3: Content Components

### 3.1 `Text` Component
- [ ] Props: `value: string` (required), `size?: "small"|"body"|"title"|"heading"|"display"`, `weight?: "normal"|"medium"|"bold"`, `color?: color`, `align?: "start"|"center"|"end"`, `max_lines?: number`, `overflow?: "clip"|"ellipsis"|"wrap"`
- [ ] No children
- [ ] Unit tests for all prop variants

### 3.2 `ProgressBar` Component
- [ ] Props: `value: number` (required, 0.0–1.0), `color?: color`, `background?: color`, `height?: number`
- [ ] Values outside 0.0–1.0 are clamped
- [ ] No children
- [ ] Unit tests (within range, clamping, color props)

### 3.3 Content Tests
- [ ] 100-iteration determinism test

---

## Phase 4: Interactive Components

### 4.1 `Button` Component
- [ ] Props: `label: string` (required), `on_tap: () -> nil` (required), `variant?: "filled"|"outlined"|"text"`, `icon?: string`, `disabled?: bool`, `loading?: bool`
- [ ] Action reference handling: `on_tap: action_name`, `on_tap: action_name(arg)`
- [ ] No children
- [ ] Unit tests (action binding, variants, disabled state)

### 4.2 `TextInput` Component
- [ ] Props: `value: string` (required), `on_change: (string) -> nil` (required), `placeholder?: string`, `label?: string`, `keyboard?: "text"|"number"|"email"|"phone"|"url"`, `max_length?: number`, `multiline?: bool`
- [ ] Action reference / lambda callback handling for on_change
- [ ] No children
- [ ] Unit tests

### 4.3 Interactive Tests
- [ ] Test action reference serialization in props
- [ ] Test lambda callback serialization
- [ ] 100-iteration determinism test

---

## Phase 5: List & Data Components

### 5.1 `ScrollList` Component
- [ ] Props: `items: list<T>` (required), `render: (T, number) -> Surface` (required), `key: (T) -> string` (required), `on_reorder?: (list<T>) -> nil`, `dividers?: bool`
- [ ] Accepts render function (lambda) for item rendering
- [ ] Unit tests (item rendering, key function, dividers)

### 5.2 List Tests
- [ ] Test ScrollList with various item types
- [ ] 100-iteration determinism test

---

## Phase 6: Feedback & Overlay Components

### 6.1 `Modal` Component
- [ ] Props: `visible: bool` (required), `on_dismiss: () -> nil` (required), `title?: string`
- [ ] Accepts children via second brace block: `Modal { props } { children }`
- [ ] Unit tests (visible toggling, children serialization)

### 6.2 `Toast` Component
- [ ] Props: `message: string` (required), `duration?: number`, `type?: "info"|"success"|"warning"|"error"`
- [ ] No children
- [ ] Unit tests

### 6.3 Feedback Tests
- [ ] Test Modal with children
- [ ] 100-iteration determinism test

---

## Phase 7: Accessibility

### 7.1 Accessibility Primitives
- [ ] Define `accessible()` function: `label: string`, `hint?: string`, `role?: string`, `value?: string`, `live_region?: "polite"|"assertive"`
- [ ] Attach accessibility attributes to all 10 Phase 0 components
- [ ] Default accessibility: auto-generate labels from content where possible (e.g., Button label → accessible label)
- [ ] Unit tests for accessibility attribute serialization

### 7.2 Accessibility Roles
- [ ] Map each component to default semantic role: Button→"button", TextInput→"textfield", etc.
- [ ] Allow role override via `accessible()` prop
- [ ] Unit tests for role mapping

---

## Phase 8: Final Validation

### 8.1 Integration Tests
- [ ] All 10 components serialize to valid Surface JSON
- [ ] Surface tree from canonical examples matches expected output
- [ ] All components render in < 16ms budget
- [ ] Component prop validation: wrong types produce clear errors

### 8.2 Final Checks
- [ ] Full 100-iteration determinism test across all components
- [ ] `cargo clippy -- -D warnings` clean
- [ ] `cargo fmt --check` clean
- [ ] README.md with component reference and architecture overview
- [ ] Every component has built-in accessibility support verified
