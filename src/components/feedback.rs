//! Feedback component builders — Modal, Toast.
//!
//! Modal is a container component (accepts children via second brace block).
//! Toast is a leaf notification component.

use crate::prop_value::PropValue;
use crate::surface::SurfaceNode;

// ── Toast Type Enum ───────────────────────────────────────────────────────────

/// Visual style for a Toast notification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastType {
    Info,
    Success,
    Warning,
    Error,
}

impl ToastType {
    fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

// ── ModalBuilder ──────────────────────────────────────────────────────────────

/// Builder for a Modal component.
///
/// Required: `visible` (Bool), `on_dismiss` (ActionRef).
/// Optional: `title` (String).
/// Accepts children (content inside the modal).
pub struct ModalBuilder {
    visible: bool,
    on_dismiss: PropValue,
    title: Option<String>,
    children: Vec<SurfaceNode>,
}

impl ModalBuilder {
    /// Create a new ModalBuilder with required props.
    ///
    /// `on_dismiss` must be a `PropValue::ActionRef` — use `PropValue::action()`.
    pub fn new(visible: bool, on_dismiss: PropValue) -> Self {
        Self {
            visible,
            on_dismiss,
            title: None,
            children: Vec::new(),
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Add a child node to the modal's content.
    pub fn child(mut self, child: SurfaceNode) -> Self {
        self.children.push(child);
        self
    }

    pub fn build(self) -> SurfaceNode {
        let mut node = SurfaceNode::new("Modal");
        node.set_prop("visible", PropValue::Bool(self.visible));
        node.set_prop("on_dismiss", self.on_dismiss);
        if let Some(title) = self.title {
            node.set_prop("title", PropValue::String(title));
        }
        for child in self.children {
            node.add_child(child);
        }
        node
    }
}

// ── ToastBuilder ──────────────────────────────────────────────────────────────

/// Builder for a Toast component.
///
/// Required: `message` (String).
/// Optional: `duration` (Number), `toast_type` (string enum).
pub struct ToastBuilder {
    message: String,
    duration: Option<f64>,
    toast_type: Option<ToastType>,
}

impl ToastBuilder {
    /// Create a new ToastBuilder with the required message.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            duration: None,
            toast_type: None,
        }
    }

    /// Set the duration in milliseconds.
    pub fn duration(mut self, duration: f64) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Set the toast type (info, success, warning, error).
    pub fn toast_type(mut self, toast_type: ToastType) -> Self {
        self.toast_type = Some(toast_type);
        self
    }

    pub fn build(self) -> SurfaceNode {
        let mut node = SurfaceNode::new("Toast");
        node.set_prop("message", PropValue::String(self.message));
        if let Some(duration) = self.duration {
            node.set_prop("duration", PropValue::Number(duration));
        }
        if let Some(toast_type) = self.toast_type {
            node.set_prop("type", PropValue::String(toast_type.as_str().to_string()));
        }
        node
    }
}

// ── Validation ────────────────────────────────────────────────────────────────

/// Validate a feedback component node (Modal or Toast).
pub fn validate_feedback_node(node: &SurfaceNode) -> Vec<String> {
    match node.component_type.as_str() {
        "Modal" => validate_modal(node),
        "Toast" => validate_toast(node),
        _ => vec![format!(
            "Unknown feedback component: {}",
            node.component_type
        )],
    }
}

fn validate_modal(node: &SurfaceNode) -> Vec<String> {
    let mut errors = Vec::new();

    // Required: visible (bool)
    match node.props.get("visible") {
        Some(PropValue::Bool(_)) => {}
        Some(other) => errors.push(format!(
            "Modal.visible: expected bool, got {}",
            other.type_name()
        )),
        None => errors.push("Modal.visible: required prop missing".to_string()),
    }

    // Required: on_dismiss (action)
    match node.props.get("on_dismiss") {
        Some(PropValue::ActionRef { .. }) => {}
        Some(other) => errors.push(format!(
            "Modal.on_dismiss: expected action, got {}",
            other.type_name()
        )),
        None => errors.push("Modal.on_dismiss: required prop missing".to_string()),
    }

    // Optional: title (string)
    if let Some(prop) = node.props.get("title") {
        if !matches!(prop, PropValue::String(_)) {
            errors.push(format!(
                "Modal.title: expected string, got {}",
                prop.type_name()
            ));
        }
    }

    // Children are allowed (Modal is a container)

    // Unknown props
    for key in node.props.keys() {
        if !matches!(key.as_str(), "visible" | "on_dismiss" | "title") {
            errors.push(format!("Modal: unknown prop '{key}'"));
        }
    }

    errors
}

fn validate_toast(node: &SurfaceNode) -> Vec<String> {
    let mut errors = Vec::new();

    // Required: message (string)
    match node.props.get("message") {
        Some(PropValue::String(_)) => {}
        Some(other) => errors.push(format!(
            "Toast.message: expected string, got {}",
            other.type_name()
        )),
        None => errors.push("Toast.message: required prop missing".to_string()),
    }

    // Optional: duration (number)
    if let Some(prop) = node.props.get("duration") {
        if !matches!(prop, PropValue::Number(_)) {
            errors.push(format!(
                "Toast.duration: expected number, got {}",
                prop.type_name()
            ));
        }
    }

    // Optional: type (string enum)
    if let Some(prop) = node.props.get("type") {
        match prop {
            PropValue::String(s)
                if matches!(s.as_str(), "info" | "success" | "warning" | "error") => {}
            _ => errors.push(format!(
                "Toast.type: expected one of [info, success, warning, error], got {:?}",
                prop
            )),
        }
    }

    // No children
    if !node.children.is_empty() {
        errors.push(format!(
            "Toast: does not accept children, but got {}",
            node.children.len()
        ));
    }

    // Unknown props
    for key in node.props.keys() {
        if !matches!(key.as_str(), "message" | "duration" | "type") {
            errors.push(format!("Toast: unknown prop '{key}'"));
        }
    }

    errors
}
