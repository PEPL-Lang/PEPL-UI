//! List & Data component builder — ScrollList.
//!
//! ScrollList renders a scrollable list of items using a `render` lambda
//! and a `key` function for identity. Items come from a list prop, not children.

use crate::accessibility;
use crate::prop_value::PropValue;
use crate::surface::SurfaceNode;

// ── ScrollListBuilder ─────────────────────────────────────────────────────────

/// Builder for a ScrollList component.
///
/// Required: `items` (List), `render` (Lambda), `key` (Lambda).
/// Optional: `on_reorder` (Lambda), `dividers` (bool).
pub struct ScrollListBuilder {
    items: PropValue,
    render: PropValue,
    key: PropValue,
    on_reorder: Option<PropValue>,
    dividers: Option<bool>,
}

impl ScrollListBuilder {
    /// Create a new ScrollListBuilder with required props.
    ///
    /// - `items` must be a `PropValue::List` — the data items to render.
    /// - `render` must be a `PropValue::Lambda` — called `(item, index) -> Surface`.
    /// - `key` must be a `PropValue::Lambda` — called `(item) -> string`.
    pub fn new(items: PropValue, render: PropValue, key: PropValue) -> Self {
        Self {
            items,
            render,
            key,
            on_reorder: None,
            dividers: None,
        }
    }

    /// Set the `on_reorder` callback (Lambda).
    pub fn on_reorder(mut self, on_reorder: PropValue) -> Self {
        self.on_reorder = Some(on_reorder);
        self
    }

    /// Set whether dividers are shown between items.
    pub fn dividers(mut self, dividers: bool) -> Self {
        self.dividers = Some(dividers);
        self
    }

    pub fn build(self) -> SurfaceNode {
        let mut node = SurfaceNode::new("ScrollList");
        node.set_prop("items", self.items);
        node.set_prop("render", self.render);
        node.set_prop("key", self.key);
        if let Some(on_reorder) = self.on_reorder {
            node.set_prop("on_reorder", on_reorder);
        }
        if let Some(dividers) = self.dividers {
            node.set_prop("dividers", PropValue::Bool(dividers));
        }
        accessibility::ensure_accessible(&mut node);
        node
    }
}

// ── Validation ────────────────────────────────────────────────────────────────

/// Validate a list/data component node (ScrollList).
pub fn validate_list_node(node: &SurfaceNode) -> Vec<String> {
    match node.component_type.as_str() {
        "ScrollList" => validate_scroll_list(node),
        _ => vec![format!("Unknown list component: {}", node.component_type)],
    }
}

fn validate_scroll_list(node: &SurfaceNode) -> Vec<String> {
    let mut errors = Vec::new();

    // Required: items (list)
    match node.props.get("items") {
        Some(PropValue::List(_)) => {}
        Some(other) => errors.push(format!(
            "ScrollList.items: expected list, got {}",
            other.type_name()
        )),
        None => errors.push("ScrollList.items: required prop missing".to_string()),
    }

    // Required: render (lambda)
    match node.props.get("render") {
        Some(PropValue::Lambda { .. }) => {}
        Some(other) => errors.push(format!(
            "ScrollList.render: expected lambda, got {}",
            other.type_name()
        )),
        None => errors.push("ScrollList.render: required prop missing".to_string()),
    }

    // Required: key (lambda)
    match node.props.get("key") {
        Some(PropValue::Lambda { .. }) => {}
        Some(other) => errors.push(format!(
            "ScrollList.key: expected lambda, got {}",
            other.type_name()
        )),
        None => errors.push("ScrollList.key: required prop missing".to_string()),
    }

    // Optional: on_reorder (lambda)
    if let Some(prop) = node.props.get("on_reorder") {
        if !matches!(prop, PropValue::Lambda { .. }) {
            errors.push(format!(
                "ScrollList.on_reorder: expected lambda, got {}",
                prop.type_name()
            ));
        }
    }

    // Optional: dividers (bool)
    if let Some(prop) = node.props.get("dividers") {
        if !matches!(prop, PropValue::Bool(_)) {
            errors.push(format!(
                "ScrollList.dividers: expected bool, got {}",
                prop.type_name()
            ));
        }
    }

    // No children (items rendered via render lambda)
    if !node.children.is_empty() {
        errors.push(format!(
            "ScrollList: does not accept children, but got {}",
            node.children.len()
        ));
    }

    // Optional: accessible (record)
    if let Some(prop) = node.props.get("accessible") {
        errors.extend(accessibility::validate_accessible_prop("ScrollList", prop));
    }

    // Unknown props
    for key in node.props.keys() {
        if !matches!(
            key.as_str(),
            "items" | "render" | "key" | "on_reorder" | "dividers" | "accessible"
        ) {
            errors.push(format!("ScrollList: unknown prop '{key}'"));
        }
    }

    errors
}
