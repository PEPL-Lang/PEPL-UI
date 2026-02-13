//! Content component builders — Text, ProgressBar.
//!
//! These are leaf components with no children. They render visible content
//! for PEPL UI views.

use crate::prop_value::PropValue;
use crate::surface::SurfaceNode;
use crate::types::ColorValue;

// ── Text Size Enum ────────────────────────────────────────────────────────────

/// Predefined text sizes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextSize {
    Small,
    Body,
    Title,
    Heading,
    Display,
}

impl TextSize {
    fn as_str(self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Body => "body",
            Self::Title => "title",
            Self::Heading => "heading",
            Self::Display => "display",
        }
    }
}

// ── Text Weight Enum ──────────────────────────────────────────────────────────

/// Font weight options.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextWeight {
    Normal,
    Medium,
    Bold,
}

impl TextWeight {
    fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Medium => "medium",
            Self::Bold => "bold",
        }
    }
}

// ── Text Align Enum ───────────────────────────────────────────────────────────

/// Text alignment options.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextAlign {
    Start,
    Center,
    End,
}

impl TextAlign {
    fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
        }
    }
}

// ── Text Overflow Enum ────────────────────────────────────────────────────────

/// Text overflow behaviour.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextOverflow {
    Clip,
    Ellipsis,
    Wrap,
}

impl TextOverflow {
    fn as_str(self) -> &'static str {
        match self {
            Self::Clip => "clip",
            Self::Ellipsis => "ellipsis",
            Self::Wrap => "wrap",
        }
    }
}

// ── TextBuilder ───────────────────────────────────────────────────────────────

/// Builder for the `Text` component.
///
/// `Text` is a leaf component (no children) that displays a string value
/// with optional styling props.
///
/// # Example
/// ```
/// use pepl_ui::TextBuilder;
/// use pepl_ui::components::content::{TextSize, TextWeight};
///
/// let node = TextBuilder::new("Hello, PEPL!")
///     .size(TextSize::Title)
///     .weight(TextWeight::Bold)
///     .build();
///
/// assert_eq!(node.component_type, "Text");
/// ```
pub struct TextBuilder {
    value: String,
    size: Option<TextSize>,
    weight: Option<TextWeight>,
    color: Option<ColorValue>,
    align: Option<TextAlign>,
    max_lines: Option<f64>,
    overflow: Option<TextOverflow>,
}

impl TextBuilder {
    /// Create a new `TextBuilder` with the required `value` prop.
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            size: None,
            weight: None,
            color: None,
            align: None,
            max_lines: None,
            overflow: None,
        }
    }

    /// Set the text size preset.
    pub fn size(mut self, size: TextSize) -> Self {
        self.size = Some(size);
        self
    }

    /// Set the font weight.
    pub fn weight(mut self, weight: TextWeight) -> Self {
        self.weight = Some(weight);
        self
    }

    /// Set the text color.
    pub fn color(mut self, color: ColorValue) -> Self {
        self.color = Some(color);
        self
    }

    /// Set the text alignment.
    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = Some(align);
        self
    }

    /// Set maximum number of lines (clipped/ellipsized after).
    pub fn max_lines(mut self, max_lines: f64) -> Self {
        self.max_lines = Some(max_lines);
        self
    }

    /// Set overflow behaviour.
    pub fn overflow(mut self, overflow: TextOverflow) -> Self {
        self.overflow = Some(overflow);
        self
    }

    /// Build the `SurfaceNode`.
    pub fn build(self) -> SurfaceNode {
        let mut node = SurfaceNode::new("Text");
        node.set_prop("value", PropValue::String(self.value));
        if let Some(size) = self.size {
            node.set_prop("size", PropValue::String(size.as_str().to_string()));
        }
        if let Some(weight) = self.weight {
            node.set_prop("weight", PropValue::String(weight.as_str().to_string()));
        }
        if let Some(color) = self.color {
            node.set_prop("color", PropValue::color(color.r, color.g, color.b, color.a));
        }
        if let Some(align) = self.align {
            node.set_prop("align", PropValue::String(align.as_str().to_string()));
        }
        if let Some(max_lines) = self.max_lines {
            node.set_prop("max_lines", PropValue::Number(max_lines));
        }
        if let Some(overflow) = self.overflow {
            node.set_prop("overflow", PropValue::String(overflow.as_str().to_string()));
        }
        node
    }
}

// ── ProgressBarBuilder ────────────────────────────────────────────────────────

/// Builder for the `ProgressBar` component.
///
/// `ProgressBar` is a leaf component (no children) that displays a
/// horizontal progress indicator. The `value` prop is clamped to 0.0–1.0.
///
/// # Example
/// ```
/// use pepl_ui::ProgressBarBuilder;
///
/// let node = ProgressBarBuilder::new(0.75).build();
/// assert_eq!(node.component_type, "ProgressBar");
/// ```
pub struct ProgressBarBuilder {
    value: f64,
    color: Option<ColorValue>,
    background: Option<ColorValue>,
    height: Option<f64>,
}

impl ProgressBarBuilder {
    /// Create a new `ProgressBarBuilder` with the required `value` prop.
    ///
    /// Values outside 0.0–1.0 are clamped.
    pub fn new(value: f64) -> Self {
        Self {
            value: value.clamp(0.0, 1.0),
            color: None,
            background: None,
            height: None,
        }
    }

    /// Set the fill color.
    pub fn color(mut self, color: ColorValue) -> Self {
        self.color = Some(color);
        self
    }

    /// Set the background (track) color.
    pub fn background(mut self, background: ColorValue) -> Self {
        self.background = Some(background);
        self
    }

    /// Set the bar height in logical pixels.
    pub fn height(mut self, height: f64) -> Self {
        self.height = Some(height);
        self
    }

    /// Build the `SurfaceNode`.
    pub fn build(self) -> SurfaceNode {
        let mut node = SurfaceNode::new("ProgressBar");
        node.set_prop("value", PropValue::Number(self.value));
        if let Some(color) = self.color {
            node.set_prop("color", PropValue::color(color.r, color.g, color.b, color.a));
        }
        if let Some(background) = self.background {
            node.set_prop("background", PropValue::color(background.r, background.g, background.b, background.a));
        }
        if let Some(height) = self.height {
            node.set_prop("height", PropValue::Number(height));
        }
        node
    }
}

// ── Validation ────────────────────────────────────────────────────────────────

/// Validates a content component node's props.
///
/// Returns a list of human-readable error strings. An empty list means
/// the node is valid.
pub fn validate_content_node(node: &SurfaceNode) -> Vec<String> {
    match node.component_type.as_str() {
        "Text" => validate_text(node),
        "ProgressBar" => validate_progress_bar(node),
        _ => vec![format!("Unknown content component: {}", node.component_type)],
    }
}

fn validate_text(node: &SurfaceNode) -> Vec<String> {
    let mut errors = Vec::new();

    // Required: value must be a string
    match node.props.get("value") {
        Some(PropValue::String(_)) => {}
        Some(other) => errors.push(format!(
            "Text.value: expected string, got {}",
            other.type_name()
        )),
        None => errors.push("Text.value: required prop missing".to_string()),
    }

    // Optional: size must be one of the allowed values
    if let Some(prop) = node.props.get("size") {
        match prop {
            PropValue::String(s) if matches!(s.as_str(), "small" | "body" | "title" | "heading" | "display") => {}
            _ => errors.push(format!(
                "Text.size: expected one of [small, body, title, heading, display], got {:?}",
                prop
            )),
        }
    }

    // Optional: weight
    if let Some(prop) = node.props.get("weight") {
        match prop {
            PropValue::String(s) if matches!(s.as_str(), "normal" | "medium" | "bold") => {}
            _ => errors.push(format!(
                "Text.weight: expected one of [normal, medium, bold], got {:?}",
                prop
            )),
        }
    }

    // Optional: color
    if let Some(prop) = node.props.get("color") {
        if !matches!(prop, PropValue::Color { .. }) {
            errors.push(format!(
                "Text.color: expected color, got {}",
                prop.type_name()
            ));
        }
    }

    // Optional: align
    if let Some(prop) = node.props.get("align") {
        match prop {
            PropValue::String(s) if matches!(s.as_str(), "start" | "center" | "end") => {}
            _ => errors.push(format!(
                "Text.align: expected one of [start, center, end], got {:?}",
                prop
            )),
        }
    }

    // Optional: max_lines
    if let Some(prop) = node.props.get("max_lines") {
        if !matches!(prop, PropValue::Number(_)) {
            errors.push(format!(
                "Text.max_lines: expected number, got {}",
                prop.type_name()
            ));
        }
    }

    // Optional: overflow
    if let Some(prop) = node.props.get("overflow") {
        match prop {
            PropValue::String(s) if matches!(s.as_str(), "clip" | "ellipsis" | "wrap") => {}
            _ => errors.push(format!(
                "Text.overflow: expected one of [clip, ellipsis, wrap], got {:?}",
                prop
            )),
        }
    }

    // No children allowed
    if !node.children.is_empty() {
        errors.push(format!(
            "Text: does not accept children, but got {}",
            node.children.len()
        ));
    }

    // Check for unknown props
    for key in node.props.keys() {
        if !matches!(
            key.as_str(),
            "value" | "size" | "weight" | "color" | "align" | "max_lines" | "overflow"
        ) {
            errors.push(format!("Text: unknown prop '{key}'"));
        }
    }

    errors
}

fn validate_progress_bar(node: &SurfaceNode) -> Vec<String> {
    let mut errors = Vec::new();

    // Required: value must be a number
    match node.props.get("value") {
        Some(PropValue::Number(_)) => {}
        Some(other) => errors.push(format!(
            "ProgressBar.value: expected number, got {}",
            other.type_name()
        )),
        None => errors.push("ProgressBar.value: required prop missing".to_string()),
    }

    // Optional: color
    if let Some(prop) = node.props.get("color") {
        if !matches!(prop, PropValue::Color { .. }) {
            errors.push(format!(
                "ProgressBar.color: expected color, got {}",
                prop.type_name()
            ));
        }
    }

    // Optional: background
    if let Some(prop) = node.props.get("background") {
        if !matches!(prop, PropValue::Color { .. }) {
            errors.push(format!(
                "ProgressBar.background: expected color, got {}",
                prop.type_name()
            ));
        }
    }

    // Optional: height
    if let Some(prop) = node.props.get("height") {
        if !matches!(prop, PropValue::Number(_)) {
            errors.push(format!(
                "ProgressBar.height: expected number, got {}",
                prop.type_name()
            ));
        }
    }

    // No children allowed
    if !node.children.is_empty() {
        errors.push(format!(
            "ProgressBar: does not accept children, but got {}",
            node.children.len()
        ));
    }

    // Check for unknown props
    for key in node.props.keys() {
        if !matches!(key.as_str(), "value" | "color" | "background" | "height") {
            errors.push(format!("ProgressBar: unknown prop '{key}'"));
        }
    }

    errors
}
