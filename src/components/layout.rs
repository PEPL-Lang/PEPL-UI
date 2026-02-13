//! Layout component builders — Column, Row, Scroll.
//!
//! These builders produce [`SurfaceNode`] trees with correct prop types
//! and validated structure. They are convenience wrappers used by the
//! evaluator when constructing Surface trees from PEPL UI blocks.
//!
//! # Components
//!
//! | Component | Props | Children |
//! |-----------|-------|----------|
//! | `Column` | `spacing?: number`, `align?: alignment`, `padding?: edges` | Yes |
//! | `Row` | `spacing?: number`, `align?: alignment`, `padding?: edges` | Yes |
//! | `Scroll` | `direction?: "vertical"\|"horizontal"\|"both"` | Yes |

use crate::accessibility;
use crate::prop_value::PropValue;
use crate::surface::SurfaceNode;
use crate::types::{Alignment, Edges};
use serde_json;

// ── Column ────────────────────────────────────────────────────────────────────

/// Builder for the `Column` layout component (vertical stack).
///
/// ```ignore
/// Column { spacing: 8, align: Center, padding: 16 } {
///     Text { value: "Hello" }
///     Button { label: "OK", on_tap: submit }
/// }
/// ```
pub struct ColumnBuilder {
    spacing: Option<f64>,
    align: Option<Alignment>,
    padding: Option<Edges>,
    children: Vec<SurfaceNode>,
}

impl ColumnBuilder {
    pub fn new() -> Self {
        Self {
            spacing: None,
            align: None,
            padding: None,
            children: Vec::new(),
        }
    }

    pub fn spacing(mut self, spacing: f64) -> Self {
        self.spacing = Some(spacing);
        self
    }

    pub fn align(mut self, align: Alignment) -> Self {
        self.align = Some(align);
        self
    }

    pub fn padding(mut self, padding: Edges) -> Self {
        self.padding = Some(padding);
        self
    }

    pub fn child(mut self, child: SurfaceNode) -> Self {
        self.children.push(child);
        self
    }

    pub fn children(mut self, children: Vec<SurfaceNode>) -> Self {
        self.children = children;
        self
    }

    pub fn build(self) -> SurfaceNode {
        let mut node = SurfaceNode::new("Column");

        if let Some(spacing) = self.spacing {
            node.set_prop("spacing", PropValue::Number(spacing));
        }
        if let Some(align) = self.align {
            node.set_prop("align", alignment_to_prop(align));
        }
        if let Some(padding) = self.padding {
            node.set_prop("padding", edges_to_prop(padding));
        }

        node.children = self.children;
        accessibility::ensure_accessible(&mut node);
        node
    }
}

impl Default for ColumnBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ── Row ───────────────────────────────────────────────────────────────────────

/// Builder for the `Row` layout component (horizontal stack).
///
/// Same prop signature as Column but lays out children horizontally.
pub struct RowBuilder {
    spacing: Option<f64>,
    align: Option<Alignment>,
    padding: Option<Edges>,
    children: Vec<SurfaceNode>,
}

impl RowBuilder {
    pub fn new() -> Self {
        Self {
            spacing: None,
            align: None,
            padding: None,
            children: Vec::new(),
        }
    }

    pub fn spacing(mut self, spacing: f64) -> Self {
        self.spacing = Some(spacing);
        self
    }

    pub fn align(mut self, align: Alignment) -> Self {
        self.align = Some(align);
        self
    }

    pub fn padding(mut self, padding: Edges) -> Self {
        self.padding = Some(padding);
        self
    }

    pub fn child(mut self, child: SurfaceNode) -> Self {
        self.children.push(child);
        self
    }

    pub fn children(mut self, children: Vec<SurfaceNode>) -> Self {
        self.children = children;
        self
    }

    pub fn build(self) -> SurfaceNode {
        let mut node = SurfaceNode::new("Row");

        if let Some(spacing) = self.spacing {
            node.set_prop("spacing", PropValue::Number(spacing));
        }
        if let Some(align) = self.align {
            node.set_prop("align", alignment_to_prop(align));
        }
        if let Some(padding) = self.padding {
            node.set_prop("padding", edges_to_prop(padding));
        }

        node.children = self.children;
        accessibility::ensure_accessible(&mut node);
        node
    }
}

impl Default for RowBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ── Scroll ────────────────────────────────────────────────────────────────────

/// Scroll direction for the Scroll component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScrollDirection {
    #[default]
    Vertical,
    Horizontal,
    Both,
}

impl ScrollDirection {
    /// Returns the string value used in the Surface tree.
    pub fn as_str(&self) -> &'static str {
        match self {
            ScrollDirection::Vertical => "vertical",
            ScrollDirection::Horizontal => "horizontal",
            ScrollDirection::Both => "both",
        }
    }
}

/// Builder for the `Scroll` layout component (scrollable container).
///
/// Default direction is `"vertical"`.
pub struct ScrollBuilder {
    direction: ScrollDirection,
    children: Vec<SurfaceNode>,
}

impl ScrollBuilder {
    pub fn new() -> Self {
        Self {
            direction: ScrollDirection::default(),
            children: Vec::new(),
        }
    }

    pub fn direction(mut self, direction: ScrollDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn child(mut self, child: SurfaceNode) -> Self {
        self.children.push(child);
        self
    }

    pub fn children(mut self, children: Vec<SurfaceNode>) -> Self {
        self.children = children;
        self
    }

    pub fn build(self) -> SurfaceNode {
        let mut node = SurfaceNode::new("Scroll");
        node.set_prop(
            "direction",
            PropValue::String(self.direction.as_str().to_string()),
        );
        node.children = self.children;
        accessibility::ensure_accessible(&mut node);
        node
    }
}

impl Default for ScrollBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Convert an `Alignment` enum to a `PropValue` for the Surface tree.
fn alignment_to_prop(align: Alignment) -> PropValue {
    let s = match align {
        Alignment::Start => "start",
        Alignment::Center => "center",
        Alignment::End => "end",
        Alignment::Stretch => "stretch",
        Alignment::SpaceBetween => "space_between",
        Alignment::SpaceAround => "space_around",
    };
    PropValue::String(s.to_string())
}

/// Convert an `Edges` value to a `PropValue` for the Surface tree.
///
/// - `Uniform(n)` → `PropValue::Number(n)` (number literal coercion)
/// - `Sides { top, bottom, start, end }` → `PropValue::Record { top, bottom, start, end }`
fn edges_to_prop(edges: Edges) -> PropValue {
    match edges {
        Edges::Uniform(n) => PropValue::Number(n),
        Edges::Sides { .. } => {
            let s = serde_json::to_value(&edges).expect("Edges serialization should never fail");
            serde_json::from_value(s).expect("Edges deserialization should never fail")
        }
    }
}

/// Validate that a component node has valid prop types.
///
/// Returns a list of validation errors. Empty means valid.
pub fn validate_layout_node(node: &SurfaceNode) -> Vec<String> {
    let mut errors = Vec::new();

    match node.component_type.as_str() {
        "Column" | "Row" => {
            for (key, val) in &node.props {
                match key.as_str() {
                    "spacing" => {
                        if !matches!(val, PropValue::Number(_)) {
                            errors.push(format!(
                                "{}: 'spacing' must be a number, got {}",
                                node.component_type,
                                val.type_name()
                            ));
                        }
                    }
                    "align" => {
                        if let PropValue::String(s) = val {
                            let valid = [
                                "start",
                                "center",
                                "end",
                                "stretch",
                                "space_between",
                                "space_around",
                            ];
                            if !valid.contains(&s.as_str()) {
                                errors.push(format!(
                                    "{}: invalid alignment '{s}'",
                                    node.component_type
                                ));
                            }
                        } else {
                            errors.push(format!(
                                "{}: 'align' must be a string, got {}",
                                node.component_type,
                                val.type_name()
                            ));
                        }
                    }
                    "padding" => {
                        // Number (Uniform coercion) or Record (Sides)
                        if !matches!(val, PropValue::Number(_) | PropValue::Record(_)) {
                            errors.push(format!(
                                "{}: 'padding' must be a number or record, got {}",
                                node.component_type,
                                val.type_name()
                            ));
                        }
                    }
                    "accessible" => {
                        errors.extend(accessibility::validate_accessible_prop(
                            &node.component_type,
                            val,
                        ));
                    }
                    other => {
                        errors.push(format!("{}: unknown prop '{other}'", node.component_type));
                    }
                }
            }
        }
        "Scroll" => {
            for (key, val) in &node.props {
                match key.as_str() {
                    "direction" => {
                        if let PropValue::String(s) = val {
                            let valid = ["vertical", "horizontal", "both"];
                            if !valid.contains(&s.as_str()) {
                                errors.push(format!("Scroll: invalid direction '{s}'"));
                            }
                        } else {
                            errors.push(format!(
                                "Scroll: 'direction' must be a string, got {}",
                                val.type_name()
                            ));
                        }
                    }
                    "accessible" => {
                        errors.extend(accessibility::validate_accessible_prop("Scroll", val));
                    }
                    other => {
                        errors.push(format!("Scroll: unknown prop '{other}'"));
                    }
                }
            }
        }
        _ => {} // Not a layout component — skip validation
    }

    errors
}
