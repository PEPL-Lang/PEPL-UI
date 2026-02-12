use crate::prop_value::PropValue;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// The complete abstract UI tree produced by evaluating a PEPL `view` function.
///
/// A `Surface` is the top-level container that wraps the root [`SurfaceNode`].
/// The host serializes this to JSON and renders it via its View Layer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Surface {
    /// The root node of the UI tree.
    pub root: SurfaceNode,
}

/// A single node in the abstract UI tree.
///
/// Matches the JSON schema from `host-integration.md`:
/// ```json
/// {
///   "type": "Column",
///   "props": { "spacing": 8 },
///   "children": [ ... ]
/// }
/// ```
///
/// Props use [`BTreeMap`] for deterministic serialization order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SurfaceNode {
    /// Component type name (e.g., "Column", "Text", "Button").
    #[serde(rename = "type")]
    pub component_type: String,

    /// Component properties. Uses `BTreeMap` for deterministic key ordering.
    pub props: BTreeMap<String, PropValue>,

    /// Child nodes (empty for leaf components like Text, Button).
    pub children: Vec<SurfaceNode>,
}

// ── Constructors ──────────────────────────────────────────────────────────────

impl Surface {
    /// Create a new Surface wrapping a root node.
    pub fn new(root: SurfaceNode) -> Self {
        Self { root }
    }

    /// Serialize this Surface to JSON (deterministic output).
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Surface serialization should never fail")
    }

    /// Serialize this Surface to pretty-printed JSON.
    pub fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(self).expect("Surface serialization should never fail")
    }
}

impl SurfaceNode {
    /// Create a new node with the given component type and no props or children.
    pub fn new(component_type: impl Into<String>) -> Self {
        Self {
            component_type: component_type.into(),
            props: BTreeMap::new(),
            children: Vec::new(),
        }
    }

    /// Builder: add a prop.
    pub fn with_prop(mut self, key: impl Into<String>, value: PropValue) -> Self {
        self.props.insert(key.into(), value);
        self
    }

    /// Builder: add a child node.
    pub fn with_child(mut self, child: SurfaceNode) -> Self {
        self.children.push(child);
        self
    }

    /// Builder: set children.
    pub fn with_children(mut self, children: Vec<SurfaceNode>) -> Self {
        self.children = children;
        self
    }

    /// Add a prop (mutable).
    pub fn set_prop(&mut self, key: impl Into<String>, value: PropValue) {
        self.props.insert(key.into(), value);
    }

    /// Add a child (mutable).
    pub fn add_child(&mut self, child: SurfaceNode) {
        self.children.push(child);
    }
}
