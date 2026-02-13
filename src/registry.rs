use std::collections::BTreeMap;

/// Whether a prop is required or optional.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropRequirement {
    Required,
    Optional,
}

/// Expected prop type for validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropType {
    String,
    Number,
    Bool,
    Color,
    Action,
    Lambda,
    List,
    Record,
    /// One of a fixed set of string values (e.g., `"filled"|"outlined"|"text"`).
    StringEnum(&'static [&'static str]),
    /// Dimension type (Px, Auto, Fill, Percent).
    Dimension,
    /// Edges type (Uniform or Sides).
    Edges,
    /// Alignment enum.
    Alignment,
}

/// Definition of a single prop on a component.
#[derive(Debug, Clone)]
pub struct PropDef {
    pub name: &'static str,
    pub requirement: PropRequirement,
    pub prop_type: PropType,
}

impl PropDef {
    pub const fn required(name: &'static str, prop_type: PropType) -> Self {
        Self {
            name,
            requirement: PropRequirement::Required,
            prop_type,
        }
    }

    pub const fn optional(name: &'static str, prop_type: PropType) -> Self {
        Self {
            name,
            requirement: PropRequirement::Optional,
            prop_type,
        }
    }
}

/// Definition of a PEPL UI component.
///
/// Each of the 10 Phase 0 components has a static definition specifying
/// its name, props, and whether it accepts children.
pub trait ComponentDef {
    /// Component type name (e.g., "Column", "Text", "Button").
    fn name(&self) -> &'static str;

    /// Whether this component accepts children.
    fn accepts_children(&self) -> bool;

    /// Prop definitions (required and optional).
    fn props(&self) -> &[PropDef];
}

/// Registry of all Phase 0 components.
///
/// Provides lookup by name and validation of component usage.
pub struct ComponentRegistry {
    components: BTreeMap<&'static str, Box<dyn ComponentDef>>,
}

impl ComponentRegistry {
    /// Create a registry with all 10 Phase 0 components registered.
    pub fn new() -> Self {
        let mut components: BTreeMap<&'static str, Box<dyn ComponentDef>> = BTreeMap::new();

        // Layout
        components.insert("Column", Box::new(ColumnDef));
        components.insert("Row", Box::new(RowDef));
        components.insert("Scroll", Box::new(ScrollDef));

        // Content
        components.insert("Text", Box::new(TextDef));
        components.insert("ProgressBar", Box::new(ProgressBarDef));

        // Interactive
        components.insert("Button", Box::new(ButtonDef));
        components.insert("TextInput", Box::new(TextInputDef));

        // List & Data
        components.insert("ScrollList", Box::new(ScrollListDef));

        // Feedback & Overlay
        components.insert("Modal", Box::new(ModalDef));
        components.insert("Toast", Box::new(ToastDef));

        Self { components }
    }

    /// Look up a component by name. Returns `None` for unknown components (E402).
    pub fn get(&self, name: &str) -> Option<&dyn ComponentDef> {
        self.components.get(name).map(|b| b.as_ref())
    }

    /// Check if a component name is valid.
    pub fn is_valid(&self, name: &str) -> bool {
        self.components.contains_key(name)
    }

    /// Get all registered component names (sorted, deterministic).
    pub fn component_names(&self) -> Vec<&'static str> {
        self.components.keys().copied().collect()
    }

    /// Total number of registered components.
    pub fn len(&self) -> usize {
        self.components.len()
    }

    /// Whether the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Layout components
// ══════════════════════════════════════════════════════════════════════════════

struct ColumnDef;
impl ComponentDef for ColumnDef {
    fn name(&self) -> &'static str { "Column" }
    fn accepts_children(&self) -> bool { true }
    fn props(&self) -> &[PropDef] {
        static PROPS: &[PropDef] = &[
            PropDef { name: "spacing", requirement: PropRequirement::Optional, prop_type: PropType::Number },
            PropDef { name: "align", requirement: PropRequirement::Optional, prop_type: PropType::Alignment },
            PropDef { name: "padding", requirement: PropRequirement::Optional, prop_type: PropType::Edges },
            PropDef { name: "accessible", requirement: PropRequirement::Optional, prop_type: PropType::Record },
        ];
        PROPS
    }
}

struct RowDef;
impl ComponentDef for RowDef {
    fn name(&self) -> &'static str { "Row" }
    fn accepts_children(&self) -> bool { true }
    fn props(&self) -> &[PropDef] {
        static PROPS: &[PropDef] = &[
            PropDef { name: "spacing", requirement: PropRequirement::Optional, prop_type: PropType::Number },
            PropDef { name: "align", requirement: PropRequirement::Optional, prop_type: PropType::Alignment },
            PropDef { name: "padding", requirement: PropRequirement::Optional, prop_type: PropType::Edges },
            PropDef { name: "accessible", requirement: PropRequirement::Optional, prop_type: PropType::Record },
        ];
        PROPS
    }
}

struct ScrollDef;
impl ComponentDef for ScrollDef {
    fn name(&self) -> &'static str { "Scroll" }
    fn accepts_children(&self) -> bool { true }
    fn props(&self) -> &[PropDef] {
        static PROPS: &[PropDef] = &[
            PropDef {
                name: "direction",
                requirement: PropRequirement::Optional,
                prop_type: PropType::StringEnum(&["vertical", "horizontal", "both"]),
            },
            PropDef { name: "accessible", requirement: PropRequirement::Optional, prop_type: PropType::Record },
        ];
        PROPS
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Content components
// ══════════════════════════════════════════════════════════════════════════════

struct TextDef;
impl ComponentDef for TextDef {
    fn name(&self) -> &'static str { "Text" }
    fn accepts_children(&self) -> bool { false }
    fn props(&self) -> &[PropDef] {
        static PROPS: &[PropDef] = &[
            PropDef { name: "value", requirement: PropRequirement::Required, prop_type: PropType::String },
            PropDef { name: "size", requirement: PropRequirement::Optional, prop_type: PropType::StringEnum(&["small", "body", "title", "heading", "display"]) },
            PropDef { name: "weight", requirement: PropRequirement::Optional, prop_type: PropType::StringEnum(&["normal", "medium", "bold"]) },
            PropDef { name: "color", requirement: PropRequirement::Optional, prop_type: PropType::Color },
            PropDef { name: "align", requirement: PropRequirement::Optional, prop_type: PropType::StringEnum(&["start", "center", "end"]) },
            PropDef { name: "max_lines", requirement: PropRequirement::Optional, prop_type: PropType::Number },
            PropDef { name: "overflow", requirement: PropRequirement::Optional, prop_type: PropType::StringEnum(&["clip", "ellipsis", "wrap"]) },
            PropDef { name: "accessible", requirement: PropRequirement::Optional, prop_type: PropType::Record },
        ];
        PROPS
    }
}

struct ProgressBarDef;
impl ComponentDef for ProgressBarDef {
    fn name(&self) -> &'static str { "ProgressBar" }
    fn accepts_children(&self) -> bool { false }
    fn props(&self) -> &[PropDef] {
        static PROPS: &[PropDef] = &[
            PropDef { name: "value", requirement: PropRequirement::Required, prop_type: PropType::Number },
            PropDef { name: "color", requirement: PropRequirement::Optional, prop_type: PropType::Color },
            PropDef { name: "background", requirement: PropRequirement::Optional, prop_type: PropType::Color },
            PropDef { name: "height", requirement: PropRequirement::Optional, prop_type: PropType::Number },
            PropDef { name: "accessible", requirement: PropRequirement::Optional, prop_type: PropType::Record },
        ];
        PROPS
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Interactive components
// ══════════════════════════════════════════════════════════════════════════════

struct ButtonDef;
impl ComponentDef for ButtonDef {
    fn name(&self) -> &'static str { "Button" }
    fn accepts_children(&self) -> bool { false }
    fn props(&self) -> &[PropDef] {
        static PROPS: &[PropDef] = &[
            PropDef { name: "label", requirement: PropRequirement::Required, prop_type: PropType::String },
            PropDef { name: "on_tap", requirement: PropRequirement::Required, prop_type: PropType::Action },
            PropDef { name: "variant", requirement: PropRequirement::Optional, prop_type: PropType::StringEnum(&["filled", "outlined", "text"]) },
            PropDef { name: "icon", requirement: PropRequirement::Optional, prop_type: PropType::String },
            PropDef { name: "disabled", requirement: PropRequirement::Optional, prop_type: PropType::Bool },
            PropDef { name: "loading", requirement: PropRequirement::Optional, prop_type: PropType::Bool },
            PropDef { name: "accessible", requirement: PropRequirement::Optional, prop_type: PropType::Record },
        ];
        PROPS
    }
}

struct TextInputDef;
impl ComponentDef for TextInputDef {
    fn name(&self) -> &'static str { "TextInput" }
    fn accepts_children(&self) -> bool { false }
    fn props(&self) -> &[PropDef] {
        static PROPS: &[PropDef] = &[
            PropDef { name: "value", requirement: PropRequirement::Required, prop_type: PropType::String },
            PropDef { name: "on_change", requirement: PropRequirement::Required, prop_type: PropType::Lambda },
            PropDef { name: "placeholder", requirement: PropRequirement::Optional, prop_type: PropType::String },
            PropDef { name: "label", requirement: PropRequirement::Optional, prop_type: PropType::String },
            PropDef { name: "keyboard", requirement: PropRequirement::Optional, prop_type: PropType::StringEnum(&["text", "number", "email", "phone", "url"]) },
            PropDef { name: "max_length", requirement: PropRequirement::Optional, prop_type: PropType::Number },
            PropDef { name: "multiline", requirement: PropRequirement::Optional, prop_type: PropType::Bool },
            PropDef { name: "accessible", requirement: PropRequirement::Optional, prop_type: PropType::Record },
        ];
        PROPS
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// List & Data components
// ══════════════════════════════════════════════════════════════════════════════

struct ScrollListDef;
impl ComponentDef for ScrollListDef {
    fn name(&self) -> &'static str { "ScrollList" }
    fn accepts_children(&self) -> bool { false }
    fn props(&self) -> &[PropDef] {
        static PROPS: &[PropDef] = &[
            PropDef { name: "items", requirement: PropRequirement::Required, prop_type: PropType::List },
            PropDef { name: "render", requirement: PropRequirement::Required, prop_type: PropType::Lambda },
            PropDef { name: "key", requirement: PropRequirement::Required, prop_type: PropType::Lambda },
            PropDef { name: "on_reorder", requirement: PropRequirement::Optional, prop_type: PropType::Lambda },
            PropDef { name: "dividers", requirement: PropRequirement::Optional, prop_type: PropType::Bool },
            PropDef { name: "accessible", requirement: PropRequirement::Optional, prop_type: PropType::Record },
        ];
        PROPS
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Feedback & Overlay components
// ══════════════════════════════════════════════════════════════════════════════

struct ModalDef;
impl ComponentDef for ModalDef {
    fn name(&self) -> &'static str { "Modal" }
    fn accepts_children(&self) -> bool { true }
    fn props(&self) -> &[PropDef] {
        static PROPS: &[PropDef] = &[
            PropDef { name: "visible", requirement: PropRequirement::Required, prop_type: PropType::Bool },
            PropDef { name: "on_dismiss", requirement: PropRequirement::Required, prop_type: PropType::Action },
            PropDef { name: "title", requirement: PropRequirement::Optional, prop_type: PropType::String },
            PropDef { name: "accessible", requirement: PropRequirement::Optional, prop_type: PropType::Record },
        ];
        PROPS
    }
}

struct ToastDef;
impl ComponentDef for ToastDef {
    fn name(&self) -> &'static str { "Toast" }
    fn accepts_children(&self) -> bool { false }
    fn props(&self) -> &[PropDef] {
        static PROPS: &[PropDef] = &[
            PropDef { name: "message", requirement: PropRequirement::Required, prop_type: PropType::String },
            PropDef { name: "duration", requirement: PropRequirement::Optional, prop_type: PropType::Number },
            PropDef { name: "type", requirement: PropRequirement::Optional, prop_type: PropType::StringEnum(&["info", "success", "warning", "error"]) },
            PropDef { name: "accessible", requirement: PropRequirement::Optional, prop_type: PropType::Record },
        ];
        PROPS
    }
}
