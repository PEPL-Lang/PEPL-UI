//! Accessibility primitives for PEPL UI components.
//!
//! Every component has built-in accessibility support. The `accessible()` function
//! creates an [`AccessibilityInfo`] that maps to platform accessibility APIs
//! (VoiceOver, TalkBack, ARIA, etc.).
//!
//! # Default Accessibility
//!
//! Components auto-generate sensible defaults:
//! - Button label → accessible label, role "button"
//! - TextInput label/placeholder → accessible label, role "textfield"
//! - Text value → accessible label, role "text"
//! - ProgressBar → "{value}% complete", role "progressbar"
//! - Modal title → accessible label, role "dialog"
//! - Toast message → accessible label, role "alert"
//!
//! Developers can override defaults via the `accessible` prop:
//! ```pepl
//! Button {
//!     label: "Add Water",
//!     on_tap: add_water(250),
//!     accessible: accessible(
//!         label: "Add 250 milliliters of water",
//!         hint: "Double tap to add water to today's intake",
//!     ),
//! }
//! ```

use crate::prop_value::PropValue;
use std::collections::BTreeMap;

// ── Semantic Role ────────────────────────────────────────────────────────────

/// Semantic role for accessibility.
///
/// Maps to platform equivalents:
/// - iOS: `UIAccessibilityTraits`
/// - Android: `AccessibilityNodeInfo.setRoleDescription`
/// - Web: ARIA `role` attribute
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticRole {
    Button,
    TextField,
    ProgressBar,
    Heading,
    Image,
    Link,
    Checkbox,
    Slider,
    List,
    Dialog,
    Alert,
    Group,
    Region,
    Text,
    None,
}

impl SemanticRole {
    /// String representation matching the spec enum values.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Button => "button",
            Self::TextField => "textfield",
            Self::ProgressBar => "progressbar",
            Self::Heading => "heading",
            Self::Image => "image",
            Self::Link => "link",
            Self::Checkbox => "checkbox",
            Self::Slider => "slider",
            Self::List => "list",
            Self::Dialog => "dialog",
            Self::Alert => "alert",
            Self::Group => "group",
            Self::Region => "region",
            Self::Text => "text",
            Self::None => "none",
        }
    }

    /// Parse a role string. Returns `None` for unrecognized values.
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "button" => Some(Self::Button),
            "textfield" => Some(Self::TextField),
            "progressbar" => Some(Self::ProgressBar),
            "heading" => Some(Self::Heading),
            "image" => Some(Self::Image),
            "link" => Some(Self::Link),
            "checkbox" => Some(Self::Checkbox),
            "slider" => Some(Self::Slider),
            "list" => Some(Self::List),
            "dialog" => Some(Self::Dialog),
            "alert" => Some(Self::Alert),
            "group" => Some(Self::Group),
            "region" => Some(Self::Region),
            "text" => Some(Self::Text),
            "none" => Some(Self::None),
            _ => None,
        }
    }

    /// All valid role string values (for validation).
    pub fn valid_values() -> &'static [&'static str] {
        &[
            "button",
            "textfield",
            "progressbar",
            "heading",
            "image",
            "link",
            "checkbox",
            "slider",
            "list",
            "dialog",
            "alert",
            "group",
            "region",
            "text",
            "none",
        ]
    }
}

// ── Live Region ──────────────────────────────────────────────────────────────

/// Live region behavior for dynamic content updates.
///
/// - `Polite`: Announces updates when the user is idle.
/// - `Assertive`: Interrupts current speech to announce updates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiveRegion {
    Polite,
    Assertive,
}

impl LiveRegion {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Polite => "polite",
            Self::Assertive => "assertive",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "polite" => Some(Self::Polite),
            "assertive" => Some(Self::Assertive),
            _ => None,
        }
    }
}

// ── AccessibilityInfo ────────────────────────────────────────────────────────

/// Accessibility attributes for a UI component.
///
/// Created via the `accessible()` function in PEPL source:
/// ```pepl
/// accessible(
///     label: "Add 250ml water",
///     hint: "Double tap to add water",
///     role: "button",
///     value: "250",
///     live_region: "polite",
/// )
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct AccessibilityInfo {
    /// Screen reader label (required). Describes the element's purpose.
    pub label: String,

    /// Additional context or instructions (optional).
    pub hint: Option<String>,

    /// Semantic role (optional — defaults per component type).
    pub role: Option<SemanticRole>,

    /// Current value for screen readers (optional — e.g., progress percentage).
    pub value: Option<String>,

    /// Live region behavior for dynamic updates (optional).
    pub live_region: Option<LiveRegion>,
}

impl AccessibilityInfo {
    /// Create a new AccessibilityInfo with a label (required).
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            hint: None,
            role: None,
            value: None,
            live_region: None,
        }
    }

    /// Set the hint.
    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    /// Set the semantic role.
    pub fn role(mut self, role: SemanticRole) -> Self {
        self.role = Some(role);
        self
    }

    /// Set the current value.
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Set the live region behavior.
    pub fn live_region(mut self, live_region: LiveRegion) -> Self {
        self.live_region = Some(live_region);
        self
    }

    /// Convert to a `PropValue::Record` for insertion into `SurfaceNode.props`.
    pub fn to_prop_value(&self) -> PropValue {
        let mut fields = BTreeMap::new();
        fields.insert("label".to_string(), PropValue::String(self.label.clone()));
        if let Some(ref hint) = self.hint {
            fields.insert("hint".to_string(), PropValue::String(hint.clone()));
        }
        if let Some(role) = self.role {
            fields.insert(
                "role".to_string(),
                PropValue::String(role.as_str().to_string()),
            );
        }
        if let Some(ref value) = self.value {
            fields.insert("value".to_string(), PropValue::String(value.clone()));
        }
        if let Some(live_region) = self.live_region {
            fields.insert(
                "live_region".to_string(),
                PropValue::String(live_region.as_str().to_string()),
            );
        }
        PropValue::Record(fields)
    }
}

// ── Default Role Mapping ─────────────────────────────────────────────────────

/// Returns the default semantic role for a component type.
///
/// | Component    | Default Role   |
/// |-------------|----------------|
/// | Button      | button         |
/// | TextInput   | textfield      |
/// | Text        | text           |
/// | ProgressBar | progressbar    |
/// | Column      | group          |
/// | Row         | group          |
/// | Scroll      | region         |
/// | ScrollList  | list           |
/// | Modal       | dialog         |
/// | Toast       | alert          |
pub fn default_role(component_type: &str) -> SemanticRole {
    match component_type {
        "Button" => SemanticRole::Button,
        "TextInput" => SemanticRole::TextField,
        "Text" => SemanticRole::Text,
        "ProgressBar" => SemanticRole::ProgressBar,
        "Column" => SemanticRole::Group,
        "Row" => SemanticRole::Group,
        "Scroll" => SemanticRole::Region,
        "ScrollList" => SemanticRole::List,
        "Modal" => SemanticRole::Dialog,
        "Toast" => SemanticRole::Alert,
        _ => SemanticRole::None,
    }
}

// ── Auto-Generated Accessibility ─────────────────────────────────────────────

/// Generate default accessibility info from component type and existing props.
///
/// Auto-labeling rules:
/// - Button: `label` prop → accessible label
/// - TextInput: `label` prop, else `placeholder`, else "Text input"
/// - Text: `value` prop (truncated to 100 chars)
/// - ProgressBar: "{value}% complete"
/// - Modal: `title` prop, else "Dialog"
/// - Toast: `message` prop
/// - Column, Row, Scroll, ScrollList: component type name (generic)
pub fn auto_accessible(
    component_type: &str,
    props: &BTreeMap<String, PropValue>,
) -> AccessibilityInfo {
    let role = default_role(component_type);
    let label = auto_label(component_type, props);

    let mut info = AccessibilityInfo::new(label).role(role);

    // Add value for ProgressBar
    if component_type == "ProgressBar" {
        if let Some(PropValue::Number(v)) = props.get("value") {
            let pct = (v * 100.0).round() as i64;
            info = info.value(format!("{pct}%"));
        }
    }

    // Add live_region for Toast (assertive — interrupts to announce)
    if component_type == "Toast" {
        info = info.live_region(LiveRegion::Assertive);
    }

    info
}

/// Extract an auto-generated label from component props.
fn auto_label(component_type: &str, props: &BTreeMap<String, PropValue>) -> String {
    match component_type {
        "Button" => extract_string_prop(props, "label").unwrap_or_else(|| "Button".to_string()),

        "TextInput" => extract_string_prop(props, "label")
            .or_else(|| extract_string_prop(props, "placeholder"))
            .unwrap_or_else(|| "Text input".to_string()),

        "Text" => {
            let value = extract_string_prop(props, "value").unwrap_or_else(|| "Text".to_string());
            // Truncate long text for accessibility labels
            if value.len() > 100 {
                format!("{}…", &value[..100])
            } else {
                value
            }
        }

        "ProgressBar" => {
            if let Some(PropValue::Number(v)) = props.get("value") {
                let pct = (v * 100.0).round() as i64;
                format!("{pct}% complete")
            } else {
                "Progress bar".to_string()
            }
        }

        "Modal" => extract_string_prop(props, "title").unwrap_or_else(|| "Dialog".to_string()),

        "Toast" => {
            extract_string_prop(props, "message").unwrap_or_else(|| "Notification".to_string())
        }

        "ScrollList" => "List".to_string(),

        // Layout containers: generic labels
        _ => component_type.to_string(),
    }
}

/// Extract a string prop value.
fn extract_string_prop(props: &BTreeMap<String, PropValue>, key: &str) -> Option<String> {
    match props.get(key) {
        Some(PropValue::String(s)) => Some(s.clone()),
        _ => None,
    }
}

// ── Validation ───────────────────────────────────────────────────────────────

/// Validate an `accessible` prop value.
///
/// The `accessible` prop must be a `PropValue::Record` with:
/// - `label`: string (required)
/// - `hint`: string (optional)
/// - `role`: string enum (optional) — one of the valid semantic roles
/// - `value`: string (optional)
/// - `live_region`: string enum (optional) — "polite" or "assertive"
pub fn validate_accessible_prop(component_name: &str, prop: &PropValue) -> Vec<String> {
    let mut errors = Vec::new();

    let fields = match prop {
        PropValue::Record(fields) => fields,
        _ => {
            errors.push(format!(
                "{component_name}.accessible: expected record, got {}",
                prop.type_name()
            ));
            return errors;
        }
    };

    // Required: label (string)
    match fields.get("label") {
        Some(PropValue::String(_)) => {}
        Some(other) => errors.push(format!(
            "{component_name}.accessible.label: expected string, got {}",
            other.type_name()
        )),
        None => errors.push(format!(
            "{component_name}.accessible.label: required field missing"
        )),
    }

    // Optional: hint (string)
    if let Some(val) = fields.get("hint") {
        if !matches!(val, PropValue::String(_)) {
            errors.push(format!(
                "{component_name}.accessible.hint: expected string, got {}",
                val.type_name()
            ));
        }
    }

    // Optional: role (string enum)
    if let Some(val) = fields.get("role") {
        match val {
            PropValue::String(s) if SemanticRole::parse(s).is_some() => {}
            PropValue::String(s) => errors.push(format!(
                "{component_name}.accessible.role: unknown role '{s}', expected one of {:?}",
                SemanticRole::valid_values()
            )),
            other => errors.push(format!(
                "{component_name}.accessible.role: expected string, got {}",
                other.type_name()
            )),
        }
    }

    // Optional: value (string)
    if let Some(val) = fields.get("value") {
        if !matches!(val, PropValue::String(_)) {
            errors.push(format!(
                "{component_name}.accessible.value: expected string, got {}",
                val.type_name()
            ));
        }
    }

    // Optional: live_region (string enum)
    if let Some(val) = fields.get("live_region") {
        match val {
            PropValue::String(s) if LiveRegion::parse(s).is_some() => {}
            PropValue::String(s) => errors.push(format!(
                "{component_name}.accessible.live_region: expected 'polite' or 'assertive', got '{s}'"
            )),
            other => errors.push(format!(
                "{component_name}.accessible.live_region: expected string, got {}",
                other.type_name()
            )),
        }
    }

    // Unknown fields
    for key in fields.keys() {
        if !matches!(
            key.as_str(),
            "label" | "hint" | "role" | "value" | "live_region"
        ) {
            errors.push(format!(
                "{component_name}.accessible: unknown field '{key}'"
            ));
        }
    }

    errors
}

/// Apply default accessibility to a SurfaceNode if not already present.
///
/// If the node already has an `"accessible"` prop, this is a no-op.
/// Otherwise, auto-generates defaults based on component type and existing props.
pub fn ensure_accessible(node: &mut crate::surface::SurfaceNode) {
    if node.props.contains_key("accessible") {
        return;
    }
    let info = auto_accessible(&node.component_type, &node.props);
    node.set_prop("accessible", info.to_prop_value());
}
