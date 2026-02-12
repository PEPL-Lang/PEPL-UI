use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A property value in the Surface tree.
///
/// Matches the JSON representation used in the host WASM contract.
/// Uses `BTreeMap` for record props to guarantee deterministic serialization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropValue {
    /// String value (e.g., `label: "Click me"`).
    String(String),

    /// Numeric value (e.g., `spacing: 8`).
    Number(f64),

    /// Boolean value (e.g., `disabled: true`).
    Bool(bool),

    /// Null / absent value.
    Nil,

    /// RGBA color as `{ r, g, b, a }` — each 0.0–1.0.
    Color {
        r: f64,
        g: f64,
        b: f64,
        a: f64,
    },

    /// Action reference (e.g., `on_tap: "increment"`).
    /// Serialized as `{ "__action": "action_name" }` or
    /// `{ "__action": "action_name", "__args": [...] }`.
    ActionRef {
        #[serde(rename = "__action")]
        action: String,
        #[serde(rename = "__args", skip_serializing_if = "Option::is_none")]
        args: Option<Vec<PropValue>>,
    },

    /// Lambda / callback reference (e.g., `on_change: (s) -> set value = s`).
    /// Serialized as `{ "__lambda": id }`. The host resolves the lambda at dispatch time.
    Lambda {
        #[serde(rename = "__lambda")]
        lambda_id: u32,
    },

    /// Ordered list of values.
    List(Vec<PropValue>),

    /// Named fields. Uses `BTreeMap` for deterministic ordering.
    Record(BTreeMap<String, PropValue>),
}

// ── Constructors ──────────────────────────────────────────────────────────────

impl PropValue {
    /// Create an action reference without arguments.
    pub fn action(name: impl Into<String>) -> Self {
        PropValue::ActionRef {
            action: name.into(),
            args: None,
        }
    }

    /// Create an action reference with arguments.
    pub fn action_with_args(name: impl Into<String>, args: Vec<PropValue>) -> Self {
        PropValue::ActionRef {
            action: name.into(),
            args: Some(args),
        }
    }

    /// Create a lambda reference.
    pub fn lambda(id: u32) -> Self {
        PropValue::Lambda { lambda_id: id }
    }

    /// Create a color value.
    pub fn color(r: f64, g: f64, b: f64, a: f64) -> Self {
        PropValue::Color { r, g, b, a }
    }

    /// Returns the type name for error messages.
    pub fn type_name(&self) -> &'static str {
        match self {
            PropValue::String(_) => "string",
            PropValue::Number(_) => "number",
            PropValue::Bool(_) => "bool",
            PropValue::Nil => "nil",
            PropValue::Color { .. } => "color",
            PropValue::ActionRef { .. } => "action",
            PropValue::Lambda { .. } => "lambda",
            PropValue::List(_) => "list",
            PropValue::Record(_) => "record",
        }
    }
}

// ── From impls ────────────────────────────────────────────────────────────────

impl From<&str> for PropValue {
    fn from(s: &str) -> Self {
        PropValue::String(s.to_string())
    }
}

impl From<String> for PropValue {
    fn from(s: String) -> Self {
        PropValue::String(s)
    }
}

impl From<f64> for PropValue {
    fn from(n: f64) -> Self {
        PropValue::Number(n)
    }
}

impl From<i64> for PropValue {
    fn from(n: i64) -> Self {
        PropValue::Number(n as f64)
    }
}

impl From<bool> for PropValue {
    fn from(b: bool) -> Self {
        PropValue::Bool(b)
    }
}
