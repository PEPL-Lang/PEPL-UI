//! Interactive component builders — Button, TextInput.
//!
//! These are leaf components with no children. They handle user interactions
//! via action references (`on_tap`) or lambda callbacks (`on_change`).

use crate::prop_value::PropValue;
use crate::surface::SurfaceNode;

// ── Button Variant Enum ───────────────────────────────────────────────────────

/// Visual style for a Button.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Filled,
    Outlined,
    Text,
}

impl ButtonVariant {
    fn as_str(self) -> &'static str {
        match self {
            Self::Filled => "filled",
            Self::Outlined => "outlined",
            Self::Text => "text",
        }
    }
}

// ── Keyboard Type Enum ────────────────────────────────────────────────────────

/// Virtual keyboard type for a TextInput.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardType {
    Text,
    Number,
    Email,
    Phone,
    Url,
}

impl KeyboardType {
    fn as_str(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Number => "number",
            Self::Email => "email",
            Self::Phone => "phone",
            Self::Url => "url",
        }
    }
}

// ── ButtonBuilder ─────────────────────────────────────────────────────────────

/// Builder for a Button component.
///
/// Required: `label` (String), `on_tap` (ActionRef).
/// Optional: `variant`, `icon`, `disabled`, `loading`.
pub struct ButtonBuilder {
    label: String,
    on_tap: PropValue,
    variant: Option<ButtonVariant>,
    icon: Option<String>,
    disabled: Option<bool>,
    loading: Option<bool>,
}

impl ButtonBuilder {
    /// Create a new ButtonBuilder with required props.
    ///
    /// `on_tap` must be a `PropValue::ActionRef` — use `PropValue::action()` or
    /// `PropValue::action_with_args()`.
    pub fn new(label: impl Into<String>, on_tap: PropValue) -> Self {
        Self {
            label: label.into(),
            on_tap,
            variant: None,
            icon: None,
            disabled: None,
            loading: None,
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = Some(variant);
        self
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = Some(disabled);
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = Some(loading);
        self
    }

    pub fn build(self) -> SurfaceNode {
        let mut node = SurfaceNode::new("Button");
        node.set_prop("label", PropValue::String(self.label));
        node.set_prop("on_tap", self.on_tap);
        if let Some(variant) = self.variant {
            node.set_prop("variant", PropValue::String(variant.as_str().to_string()));
        }
        if let Some(icon) = self.icon {
            node.set_prop("icon", PropValue::String(icon));
        }
        if let Some(disabled) = self.disabled {
            node.set_prop("disabled", PropValue::Bool(disabled));
        }
        if let Some(loading) = self.loading {
            node.set_prop("loading", PropValue::Bool(loading));
        }
        node
    }
}

// ── TextInputBuilder ──────────────────────────────────────────────────────────

/// Builder for a TextInput component.
///
/// Required: `value` (String), `on_change` (Lambda).
/// Optional: `placeholder`, `label`, `keyboard`, `max_length`, `multiline`.
pub struct TextInputBuilder {
    value: String,
    on_change: PropValue,
    placeholder: Option<String>,
    label: Option<String>,
    keyboard: Option<KeyboardType>,
    max_length: Option<f64>,
    multiline: Option<bool>,
}

impl TextInputBuilder {
    /// Create a new TextInputBuilder with required props.
    ///
    /// `on_change` must be a `PropValue::Lambda` — use `PropValue::lambda(id)`.
    pub fn new(value: impl Into<String>, on_change: PropValue) -> Self {
        Self {
            value: value.into(),
            on_change,
            placeholder: None,
            label: None,
            keyboard: None,
            max_length: None,
            multiline: None,
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn keyboard(mut self, keyboard: KeyboardType) -> Self {
        self.keyboard = Some(keyboard);
        self
    }

    pub fn max_length(mut self, max_length: f64) -> Self {
        self.max_length = Some(max_length);
        self
    }

    pub fn multiline(mut self, multiline: bool) -> Self {
        self.multiline = Some(multiline);
        self
    }

    pub fn build(self) -> SurfaceNode {
        let mut node = SurfaceNode::new("TextInput");
        node.set_prop("value", PropValue::String(self.value));
        node.set_prop("on_change", self.on_change);
        if let Some(placeholder) = self.placeholder {
            node.set_prop("placeholder", PropValue::String(placeholder));
        }
        if let Some(label) = self.label {
            node.set_prop("label", PropValue::String(label));
        }
        if let Some(keyboard) = self.keyboard {
            node.set_prop("keyboard", PropValue::String(keyboard.as_str().to_string()));
        }
        if let Some(max_length) = self.max_length {
            node.set_prop("max_length", PropValue::Number(max_length));
        }
        if let Some(multiline) = self.multiline {
            node.set_prop("multiline", PropValue::Bool(multiline));
        }
        node
    }
}

// ── Validation ────────────────────────────────────────────────────────────────

/// Validate an interactive component node (Button or TextInput).
pub fn validate_interactive_node(node: &SurfaceNode) -> Vec<String> {
    match node.component_type.as_str() {
        "Button" => validate_button(node),
        "TextInput" => validate_text_input(node),
        _ => vec![format!(
            "Unknown interactive component: {}",
            node.component_type
        )],
    }
}

fn validate_button(node: &SurfaceNode) -> Vec<String> {
    let mut errors = Vec::new();

    // Required: label (string)
    match node.props.get("label") {
        Some(PropValue::String(_)) => {}
        Some(other) => errors.push(format!(
            "Button.label: expected string, got {}",
            other.type_name()
        )),
        None => errors.push("Button.label: required prop missing".to_string()),
    }

    // Required: on_tap (action)
    match node.props.get("on_tap") {
        Some(PropValue::ActionRef { .. }) => {}
        Some(other) => errors.push(format!(
            "Button.on_tap: expected action, got {}",
            other.type_name()
        )),
        None => errors.push("Button.on_tap: required prop missing".to_string()),
    }

    // Optional: variant (string enum)
    if let Some(prop) = node.props.get("variant") {
        match prop {
            PropValue::String(s)
                if matches!(s.as_str(), "filled" | "outlined" | "text") => {}
            _ => errors.push(format!(
                "Button.variant: expected one of [filled, outlined, text], got {:?}",
                prop
            )),
        }
    }

    // Optional: icon (string)
    if let Some(prop) = node.props.get("icon") {
        if !matches!(prop, PropValue::String(_)) {
            errors.push(format!(
                "Button.icon: expected string, got {}",
                prop.type_name()
            ));
        }
    }

    // Optional: disabled (bool)
    if let Some(prop) = node.props.get("disabled") {
        if !matches!(prop, PropValue::Bool(_)) {
            errors.push(format!(
                "Button.disabled: expected bool, got {}",
                prop.type_name()
            ));
        }
    }

    // Optional: loading (bool)
    if let Some(prop) = node.props.get("loading") {
        if !matches!(prop, PropValue::Bool(_)) {
            errors.push(format!(
                "Button.loading: expected bool, got {}",
                prop.type_name()
            ));
        }
    }

    // No children
    if !node.children.is_empty() {
        errors.push(format!(
            "Button: does not accept children, but got {}",
            node.children.len()
        ));
    }

    // Unknown props
    for key in node.props.keys() {
        if !matches!(
            key.as_str(),
            "label" | "on_tap" | "variant" | "icon" | "disabled" | "loading"
        ) {
            errors.push(format!("Button: unknown prop '{key}'"));
        }
    }

    errors
}

fn validate_text_input(node: &SurfaceNode) -> Vec<String> {
    let mut errors = Vec::new();

    // Required: value (string)
    match node.props.get("value") {
        Some(PropValue::String(_)) => {}
        Some(other) => errors.push(format!(
            "TextInput.value: expected string, got {}",
            other.type_name()
        )),
        None => errors.push("TextInput.value: required prop missing".to_string()),
    }

    // Required: on_change (lambda)
    match node.props.get("on_change") {
        Some(PropValue::Lambda { .. }) => {}
        Some(other) => errors.push(format!(
            "TextInput.on_change: expected lambda, got {}",
            other.type_name()
        )),
        None => errors.push("TextInput.on_change: required prop missing".to_string()),
    }

    // Optional: placeholder (string)
    if let Some(prop) = node.props.get("placeholder") {
        if !matches!(prop, PropValue::String(_)) {
            errors.push(format!(
                "TextInput.placeholder: expected string, got {}",
                prop.type_name()
            ));
        }
    }

    // Optional: label (string)
    if let Some(prop) = node.props.get("label") {
        if !matches!(prop, PropValue::String(_)) {
            errors.push(format!(
                "TextInput.label: expected string, got {}",
                prop.type_name()
            ));
        }
    }

    // Optional: keyboard (string enum)
    if let Some(prop) = node.props.get("keyboard") {
        match prop {
            PropValue::String(s)
                if matches!(s.as_str(), "text" | "number" | "email" | "phone" | "url") => {}
            _ => errors.push(format!(
                "TextInput.keyboard: expected one of [text, number, email, phone, url], got {:?}",
                prop
            )),
        }
    }

    // Optional: max_length (number)
    if let Some(prop) = node.props.get("max_length") {
        if !matches!(prop, PropValue::Number(_)) {
            errors.push(format!(
                "TextInput.max_length: expected number, got {}",
                prop.type_name()
            ));
        }
    }

    // Optional: multiline (bool)
    if let Some(prop) = node.props.get("multiline") {
        if !matches!(prop, PropValue::Bool(_)) {
            errors.push(format!(
                "TextInput.multiline: expected bool, got {}",
                prop.type_name()
            ));
        }
    }

    // No children
    if !node.children.is_empty() {
        errors.push(format!(
            "TextInput: does not accept children, but got {}",
            node.children.len()
        ));
    }

    // Unknown props
    for key in node.props.keys() {
        if !matches!(
            key.as_str(),
            "value" | "on_change" | "placeholder" | "label" | "keyboard" | "max_length"
                | "multiline"
        ) {
            errors.push(format!("TextInput: unknown prop '{key}'"));
        }
    }

    errors
}
