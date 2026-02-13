//! Tests for Phase 7: Accessibility
//!
//! Covers:
//! - 7.1 Accessibility Primitives (accessible() function, auto-labels, attach to all 10 components)
//! - 7.2 Semantic Roles (default roles, role overrides, validation)

use pepl_ui::accessibility::{
    AccessibilityInfo, LiveRegion, SemanticRole, auto_accessible, default_role,
    ensure_accessible, validate_accessible_prop,
};
use pepl_ui::components::content::validate_content_node;
use pepl_ui::components::feedback::validate_feedback_node;
use pepl_ui::components::interactive::validate_interactive_node;
use pepl_ui::components::layout::validate_layout_node;
use pepl_ui::components::list::validate_list_node;
use pepl_ui::PropValue;
use pepl_ui::SurfaceNode;
use pepl_ui::{
    ButtonBuilder, ColumnBuilder, ModalBuilder, ProgressBarBuilder, RowBuilder, ScrollBuilder,
    ScrollListBuilder, TextBuilder, TextInputBuilder, ToastBuilder,
};
use std::collections::BTreeMap;

// ══════════════════════════════════════════════════════════════════════════════
// SemanticRole
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn semantic_role_as_str() {
    assert_eq!(SemanticRole::Button.as_str(), "button");
    assert_eq!(SemanticRole::TextField.as_str(), "textfield");
    assert_eq!(SemanticRole::ProgressBar.as_str(), "progressbar");
    assert_eq!(SemanticRole::Heading.as_str(), "heading");
    assert_eq!(SemanticRole::Image.as_str(), "image");
    assert_eq!(SemanticRole::Link.as_str(), "link");
    assert_eq!(SemanticRole::Checkbox.as_str(), "checkbox");
    assert_eq!(SemanticRole::Slider.as_str(), "slider");
    assert_eq!(SemanticRole::List.as_str(), "list");
    assert_eq!(SemanticRole::Dialog.as_str(), "dialog");
    assert_eq!(SemanticRole::Alert.as_str(), "alert");
    assert_eq!(SemanticRole::Group.as_str(), "group");
    assert_eq!(SemanticRole::Region.as_str(), "region");
    assert_eq!(SemanticRole::Text.as_str(), "text");
    assert_eq!(SemanticRole::None.as_str(), "none");
}

#[test]
fn semantic_role_from_str_roundtrip() {
    for role_str in SemanticRole::valid_values() {
        let role = SemanticRole::parse(role_str).unwrap();
        assert_eq!(role.as_str(), *role_str);
    }
}

#[test]
fn semantic_role_from_str_invalid() {
    assert_eq!(SemanticRole::parse("unknown"), None);
    assert_eq!(SemanticRole::parse(""), None);
    assert_eq!(SemanticRole::parse("BUTTON"), None); // case-sensitive
}

// ══════════════════════════════════════════════════════════════════════════════
// LiveRegion
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn live_region_as_str() {
    assert_eq!(LiveRegion::Polite.as_str(), "polite");
    assert_eq!(LiveRegion::Assertive.as_str(), "assertive");
}

#[test]
fn live_region_from_str() {
    assert_eq!(LiveRegion::parse("polite"), Some(LiveRegion::Polite));
    assert_eq!(LiveRegion::parse("assertive"), Some(LiveRegion::Assertive));
    assert_eq!(LiveRegion::parse("off"), None);
}

// ══════════════════════════════════════════════════════════════════════════════
// AccessibilityInfo
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn accessibility_info_new_minimal() {
    let info = AccessibilityInfo::new("Submit");
    assert_eq!(info.label, "Submit");
    assert!(info.hint.is_none());
    assert!(info.role.is_none());
    assert!(info.value.is_none());
    assert!(info.live_region.is_none());
}

#[test]
fn accessibility_info_builder_chain() {
    let info = AccessibilityInfo::new("Submit")
        .hint("Double tap to submit form")
        .role(SemanticRole::Button)
        .value("active")
        .live_region(LiveRegion::Polite);

    assert_eq!(info.label, "Submit");
    assert_eq!(info.hint.as_deref(), Some("Double tap to submit form"));
    assert_eq!(info.role, Some(SemanticRole::Button));
    assert_eq!(info.value.as_deref(), Some("active"));
    assert_eq!(info.live_region, Some(LiveRegion::Polite));
}

#[test]
fn accessibility_info_to_prop_value_minimal() {
    let info = AccessibilityInfo::new("Click me");
    let prop = info.to_prop_value();

    match &prop {
        PropValue::Record(fields) => {
            assert_eq!(fields.len(), 1);
            assert_eq!(fields["label"], PropValue::String("Click me".to_string()));
        }
        _ => panic!("Expected Record, got {:?}", prop),
    }
}

#[test]
fn accessibility_info_to_prop_value_full() {
    let info = AccessibilityInfo::new("Progress")
        .hint("Shows download progress")
        .role(SemanticRole::ProgressBar)
        .value("75%")
        .live_region(LiveRegion::Polite);
    let prop = info.to_prop_value();

    match &prop {
        PropValue::Record(fields) => {
            assert_eq!(fields.len(), 5);
            assert_eq!(fields["label"], PropValue::String("Progress".to_string()));
            assert_eq!(
                fields["hint"],
                PropValue::String("Shows download progress".to_string())
            );
            assert_eq!(fields["role"], PropValue::String("progressbar".to_string()));
            assert_eq!(fields["value"], PropValue::String("75%".to_string()));
            assert_eq!(
                fields["live_region"],
                PropValue::String("polite".to_string())
            );
        }
        _ => panic!("Expected Record"),
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Default Roles
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn default_role_all_10_components() {
    assert_eq!(default_role("Button"), SemanticRole::Button);
    assert_eq!(default_role("TextInput"), SemanticRole::TextField);
    assert_eq!(default_role("Text"), SemanticRole::Text);
    assert_eq!(default_role("ProgressBar"), SemanticRole::ProgressBar);
    assert_eq!(default_role("Column"), SemanticRole::Group);
    assert_eq!(default_role("Row"), SemanticRole::Group);
    assert_eq!(default_role("Scroll"), SemanticRole::Region);
    assert_eq!(default_role("ScrollList"), SemanticRole::List);
    assert_eq!(default_role("Modal"), SemanticRole::Dialog);
    assert_eq!(default_role("Toast"), SemanticRole::Alert);
}

#[test]
fn default_role_unknown_component() {
    assert_eq!(default_role("FooBar"), SemanticRole::None);
}

// ══════════════════════════════════════════════════════════════════════════════
// Auto-Generated Accessibility
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn auto_accessible_button_label() {
    let mut props = BTreeMap::new();
    props.insert("label".to_string(), PropValue::String("Save".to_string()));
    let info = auto_accessible("Button", &props);
    assert_eq!(info.label, "Save");
    assert_eq!(info.role, Some(SemanticRole::Button));
}

#[test]
fn auto_accessible_button_no_label() {
    let props = BTreeMap::new();
    let info = auto_accessible("Button", &props);
    assert_eq!(info.label, "Button");
}

#[test]
fn auto_accessible_text_input_label() {
    let mut props = BTreeMap::new();
    props.insert("label".to_string(), PropValue::String("Email".to_string()));
    let info = auto_accessible("TextInput", &props);
    assert_eq!(info.label, "Email");
    assert_eq!(info.role, Some(SemanticRole::TextField));
}

#[test]
fn auto_accessible_text_input_placeholder_fallback() {
    let mut props = BTreeMap::new();
    props.insert(
        "placeholder".to_string(),
        PropValue::String("Enter email".to_string()),
    );
    let info = auto_accessible("TextInput", &props);
    assert_eq!(info.label, "Enter email");
}

#[test]
fn auto_accessible_text_input_no_label_or_placeholder() {
    let props = BTreeMap::new();
    let info = auto_accessible("TextInput", &props);
    assert_eq!(info.label, "Text input");
}

#[test]
fn auto_accessible_text_value() {
    let mut props = BTreeMap::new();
    props.insert(
        "value".to_string(),
        PropValue::String("Hello, world!".to_string()),
    );
    let info = auto_accessible("Text", &props);
    assert_eq!(info.label, "Hello, world!");
    assert_eq!(info.role, Some(SemanticRole::Text));
}

#[test]
fn auto_accessible_text_long_value_truncated() {
    let mut props = BTreeMap::new();
    let long_text = "a".repeat(200);
    props.insert("value".to_string(), PropValue::String(long_text));
    let info = auto_accessible("Text", &props);
    assert!(info.label.len() <= 104); // 100 chars + "…" (3 bytes UTF-8)
    assert!(info.label.ends_with('…'));
}

#[test]
fn auto_accessible_progress_bar() {
    let mut props = BTreeMap::new();
    props.insert("value".to_string(), PropValue::Number(0.75));
    let info = auto_accessible("ProgressBar", &props);
    assert_eq!(info.label, "75% complete");
    assert_eq!(info.role, Some(SemanticRole::ProgressBar));
    assert_eq!(info.value.as_deref(), Some("75%"));
}

#[test]
fn auto_accessible_progress_bar_no_value() {
    let props = BTreeMap::new();
    let info = auto_accessible("ProgressBar", &props);
    assert_eq!(info.label, "Progress bar");
}

#[test]
fn auto_accessible_modal_title() {
    let mut props = BTreeMap::new();
    props.insert(
        "title".to_string(),
        PropValue::String("Settings".to_string()),
    );
    let info = auto_accessible("Modal", &props);
    assert_eq!(info.label, "Settings");
    assert_eq!(info.role, Some(SemanticRole::Dialog));
}

#[test]
fn auto_accessible_modal_no_title() {
    let props = BTreeMap::new();
    let info = auto_accessible("Modal", &props);
    assert_eq!(info.label, "Dialog");
}

#[test]
fn auto_accessible_toast_message() {
    let mut props = BTreeMap::new();
    props.insert(
        "message".to_string(),
        PropValue::String("Saved!".to_string()),
    );
    let info = auto_accessible("Toast", &props);
    assert_eq!(info.label, "Saved!");
    assert_eq!(info.role, Some(SemanticRole::Alert));
    assert_eq!(info.live_region, Some(LiveRegion::Assertive));
}

#[test]
fn auto_accessible_toast_no_message() {
    let props = BTreeMap::new();
    let info = auto_accessible("Toast", &props);
    assert_eq!(info.label, "Notification");
}

#[test]
fn auto_accessible_column() {
    let props = BTreeMap::new();
    let info = auto_accessible("Column", &props);
    assert_eq!(info.label, "Column");
    assert_eq!(info.role, Some(SemanticRole::Group));
}

#[test]
fn auto_accessible_row() {
    let props = BTreeMap::new();
    let info = auto_accessible("Row", &props);
    assert_eq!(info.label, "Row");
    assert_eq!(info.role, Some(SemanticRole::Group));
}

#[test]
fn auto_accessible_scroll() {
    let props = BTreeMap::new();
    let info = auto_accessible("Scroll", &props);
    assert_eq!(info.label, "Scroll");
    assert_eq!(info.role, Some(SemanticRole::Region));
}

#[test]
fn auto_accessible_scroll_list() {
    let props = BTreeMap::new();
    let info = auto_accessible("ScrollList", &props);
    assert_eq!(info.label, "List");
    assert_eq!(info.role, Some(SemanticRole::List));
}

// ══════════════════════════════════════════════════════════════════════════════
// Validation
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn validate_accessible_valid_minimal() {
    let mut fields = BTreeMap::new();
    fields.insert("label".to_string(), PropValue::String("OK".to_string()));
    let prop = PropValue::Record(fields);
    let errors = validate_accessible_prop("Button", &prop);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn validate_accessible_valid_full() {
    let mut fields = BTreeMap::new();
    fields.insert("label".to_string(), PropValue::String("Save".to_string()));
    fields.insert("hint".to_string(), PropValue::String("Saves data".to_string()));
    fields.insert("role".to_string(), PropValue::String("button".to_string()));
    fields.insert("value".to_string(), PropValue::String("active".to_string()));
    fields.insert("live_region".to_string(), PropValue::String("polite".to_string()));
    let prop = PropValue::Record(fields);
    let errors = validate_accessible_prop("Button", &prop);
    assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
}

#[test]
fn validate_accessible_not_record() {
    let prop = PropValue::String("bad".to_string());
    let errors = validate_accessible_prop("Button", &prop);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("expected record"));
}

#[test]
fn validate_accessible_missing_label() {
    let fields = BTreeMap::new();
    let prop = PropValue::Record(fields);
    let errors = validate_accessible_prop("Text", &prop);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("label: required field missing"));
}

#[test]
fn validate_accessible_wrong_label_type() {
    let mut fields = BTreeMap::new();
    fields.insert("label".to_string(), PropValue::Number(42.0));
    let prop = PropValue::Record(fields);
    let errors = validate_accessible_prop("Button", &prop);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("label: expected string"));
}

#[test]
fn validate_accessible_wrong_hint_type() {
    let mut fields = BTreeMap::new();
    fields.insert("label".to_string(), PropValue::String("OK".to_string()));
    fields.insert("hint".to_string(), PropValue::Bool(true));
    let prop = PropValue::Record(fields);
    let errors = validate_accessible_prop("Button", &prop);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("hint: expected string"));
}

#[test]
fn validate_accessible_invalid_role() {
    let mut fields = BTreeMap::new();
    fields.insert("label".to_string(), PropValue::String("OK".to_string()));
    fields.insert("role".to_string(), PropValue::String("widget".to_string()));
    let prop = PropValue::Record(fields);
    let errors = validate_accessible_prop("Button", &prop);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("unknown role"));
}

#[test]
fn validate_accessible_invalid_live_region() {
    let mut fields = BTreeMap::new();
    fields.insert("label".to_string(), PropValue::String("OK".to_string()));
    fields.insert("live_region".to_string(), PropValue::String("off".to_string()));
    let prop = PropValue::Record(fields);
    let errors = validate_accessible_prop("Toast", &prop);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("expected 'polite' or 'assertive'"));
}

#[test]
fn validate_accessible_unknown_field() {
    let mut fields = BTreeMap::new();
    fields.insert("label".to_string(), PropValue::String("OK".to_string()));
    fields.insert("foo".to_string(), PropValue::String("bar".to_string()));
    let prop = PropValue::Record(fields);
    let errors = validate_accessible_prop("Button", &prop);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("unknown field 'foo'"));
}

// ══════════════════════════════════════════════════════════════════════════════
// ensure_accessible
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn ensure_accessible_adds_default() {
    let mut node = SurfaceNode::new("Button");
    node.set_prop("label", PropValue::String("Save".to_string()));
    assert!(!node.props.contains_key("accessible"));

    ensure_accessible(&mut node);
    assert!(node.props.contains_key("accessible"));

    // Should have auto-generated label from Button.label
    match &node.props["accessible"] {
        PropValue::Record(fields) => {
            assert_eq!(fields["label"], PropValue::String("Save".to_string()));
            assert_eq!(fields["role"], PropValue::String("button".to_string()));
        }
        _ => panic!("Expected Record"),
    }
}

#[test]
fn ensure_accessible_does_not_overwrite() {
    let custom = AccessibilityInfo::new("Custom label")
        .role(SemanticRole::Link)
        .to_prop_value();

    let mut node = SurfaceNode::new("Button");
    node.set_prop("label", PropValue::String("Save".to_string()));
    node.set_prop("accessible", custom);

    ensure_accessible(&mut node);

    // Should keep custom, not overwrite
    match &node.props["accessible"] {
        PropValue::Record(fields) => {
            assert_eq!(
                fields["label"],
                PropValue::String("Custom label".to_string())
            );
            assert_eq!(fields["role"], PropValue::String("link".to_string()));
        }
        _ => panic!("Expected Record"),
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Builder Integration — All 10 Components
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn button_builder_has_accessible() {
    let node = ButtonBuilder::new("Click", PropValue::action("submit")).build();
    assert!(node.props.contains_key("accessible"));
    let errors = validate_interactive_node(&node);
    assert!(errors.is_empty(), "Validation failed: {:?}", errors);
}

#[test]
fn button_builder_accessible_has_correct_defaults() {
    let node = ButtonBuilder::new("Save Changes", PropValue::action("save")).build();
    match &node.props["accessible"] {
        PropValue::Record(fields) => {
            assert_eq!(
                fields["label"],
                PropValue::String("Save Changes".to_string())
            );
            assert_eq!(fields["role"], PropValue::String("button".to_string()));
        }
        _ => panic!("Expected Record"),
    }
}

#[test]
fn text_input_builder_has_accessible() {
    let node = TextInputBuilder::new("hello", PropValue::lambda(1))
        .label("Name")
        .build();
    assert!(node.props.contains_key("accessible"));
    let errors = validate_interactive_node(&node);
    assert!(errors.is_empty(), "Validation failed: {:?}", errors);

    match &node.props["accessible"] {
        PropValue::Record(fields) => {
            assert_eq!(fields["label"], PropValue::String("Name".to_string()));
            assert_eq!(fields["role"], PropValue::String("textfield".to_string()));
        }
        _ => panic!("Expected Record"),
    }
}

#[test]
fn text_input_builder_placeholder_fallback() {
    let node = TextInputBuilder::new("", PropValue::lambda(2))
        .placeholder("Enter name")
        .build();
    match &node.props["accessible"] {
        PropValue::Record(fields) => {
            assert_eq!(
                fields["label"],
                PropValue::String("Enter name".to_string())
            );
        }
        _ => panic!("Expected Record"),
    }
}

#[test]
fn text_builder_has_accessible() {
    let node = TextBuilder::new("Hello, world!").build();
    assert!(node.props.contains_key("accessible"));
    let errors = validate_content_node(&node);
    assert!(errors.is_empty(), "Validation failed: {:?}", errors);

    match &node.props["accessible"] {
        PropValue::Record(fields) => {
            assert_eq!(
                fields["label"],
                PropValue::String("Hello, world!".to_string())
            );
            assert_eq!(fields["role"], PropValue::String("text".to_string()));
        }
        _ => panic!("Expected Record"),
    }
}

#[test]
fn progress_bar_builder_has_accessible() {
    let node = ProgressBarBuilder::new(0.5).build();
    assert!(node.props.contains_key("accessible"));
    let errors = validate_content_node(&node);
    assert!(errors.is_empty(), "Validation failed: {:?}", errors);

    match &node.props["accessible"] {
        PropValue::Record(fields) => {
            assert_eq!(
                fields["label"],
                PropValue::String("50% complete".to_string())
            );
            assert_eq!(
                fields["role"],
                PropValue::String("progressbar".to_string())
            );
            assert_eq!(fields["value"], PropValue::String("50%".to_string()));
        }
        _ => panic!("Expected Record"),
    }
}

#[test]
fn column_builder_has_accessible() {
    let node = ColumnBuilder::new()
        .child(TextBuilder::new("hi").build())
        .build();
    assert!(node.props.contains_key("accessible"));
    let errors = validate_layout_node(&node);
    assert!(errors.is_empty(), "Validation failed: {:?}", errors);

    match &node.props["accessible"] {
        PropValue::Record(fields) => {
            assert_eq!(fields["role"], PropValue::String("group".to_string()));
        }
        _ => panic!("Expected Record"),
    }
}

#[test]
fn row_builder_has_accessible() {
    let node = RowBuilder::new().build();
    assert!(node.props.contains_key("accessible"));
    let errors = validate_layout_node(&node);
    assert!(errors.is_empty(), "Validation failed: {:?}", errors);
}

#[test]
fn scroll_builder_has_accessible() {
    let node = ScrollBuilder::new().build();
    assert!(node.props.contains_key("accessible"));
    let errors = validate_layout_node(&node);
    assert!(errors.is_empty(), "Validation failed: {:?}", errors);

    match &node.props["accessible"] {
        PropValue::Record(fields) => {
            assert_eq!(fields["role"], PropValue::String("region".to_string()));
        }
        _ => panic!("Expected Record"),
    }
}

#[test]
fn scroll_list_builder_has_accessible() {
    let node = ScrollListBuilder::new(
        PropValue::List(vec![]),
        PropValue::lambda(3),
        PropValue::lambda(4),
    )
    .build();
    assert!(node.props.contains_key("accessible"));
    let errors = validate_list_node(&node);
    assert!(errors.is_empty(), "Validation failed: {:?}", errors);

    match &node.props["accessible"] {
        PropValue::Record(fields) => {
            assert_eq!(fields["role"], PropValue::String("list".to_string()));
        }
        _ => panic!("Expected Record"),
    }
}

#[test]
fn modal_builder_has_accessible() {
    let node = ModalBuilder::new(true, PropValue::action("dismiss"))
        .title("Settings")
        .build();
    assert!(node.props.contains_key("accessible"));
    let errors = validate_feedback_node(&node);
    assert!(errors.is_empty(), "Validation failed: {:?}", errors);

    match &node.props["accessible"] {
        PropValue::Record(fields) => {
            assert_eq!(
                fields["label"],
                PropValue::String("Settings".to_string())
            );
            assert_eq!(fields["role"], PropValue::String("dialog".to_string()));
        }
        _ => panic!("Expected Record"),
    }
}

#[test]
fn toast_builder_has_accessible() {
    let node = ToastBuilder::new("Changes saved!").build();
    assert!(node.props.contains_key("accessible"));
    let errors = validate_feedback_node(&node);
    assert!(errors.is_empty(), "Validation failed: {:?}", errors);

    match &node.props["accessible"] {
        PropValue::Record(fields) => {
            assert_eq!(
                fields["label"],
                PropValue::String("Changes saved!".to_string())
            );
            assert_eq!(fields["role"], PropValue::String("alert".to_string()));
            assert_eq!(
                fields["live_region"],
                PropValue::String("assertive".to_string())
            );
        }
        _ => panic!("Expected Record"),
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Determinism — same inputs always produce same accessible output
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn deterministic_accessible_output() {
    let build_button = || {
        ButtonBuilder::new("Tap", PropValue::action("go"))
            .variant(pepl_ui::ButtonVariant::Outlined)
            .disabled(true)
            .build()
    };

    let a = build_button();
    let b = build_button();

    let a_acc = &a.props["accessible"];
    let b_acc = &b.props["accessible"];
    assert_eq!(a_acc, b_acc, "Accessible output must be deterministic");
}

#[test]
fn deterministic_progress_bar_accessible() {
    let a = ProgressBarBuilder::new(0.333).build();
    let b = ProgressBarBuilder::new(0.333).build();
    assert_eq!(a.props["accessible"], b.props["accessible"]);
}

// ══════════════════════════════════════════════════════════════════════════════
// Registry — all 10 components include accessible prop
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn registry_all_components_have_accessible_prop() {
    use pepl_ui::ComponentRegistry;

    let registry = ComponentRegistry::new();
    for name in registry.component_names() {
        let def = registry.get(name).unwrap();
        let has_accessible = def.props().iter().any(|p| p.name == "accessible");
        assert!(
            has_accessible,
            "Component '{name}' missing 'accessible' prop in registry"
        );
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Validation — built nodes pass their category validators
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn all_builders_pass_validation() {
    // Interactive
    let button = ButtonBuilder::new("OK", PropValue::action("ok")).build();
    assert!(validate_interactive_node(&button).is_empty());

    let text_input =
        TextInputBuilder::new("val", PropValue::lambda(2)).build();
    assert!(validate_interactive_node(&text_input).is_empty());

    // Content
    let text = TextBuilder::new("hi").build();
    assert!(validate_content_node(&text).is_empty());

    let progress = ProgressBarBuilder::new(0.5).build();
    assert!(validate_content_node(&progress).is_empty());

    // Layout
    let col = ColumnBuilder::new().build();
    assert!(validate_layout_node(&col).is_empty());

    let row = RowBuilder::new().build();
    assert!(validate_layout_node(&row).is_empty());

    let scroll = ScrollBuilder::new().build();
    assert!(validate_layout_node(&scroll).is_empty());

    // List
    let list = ScrollListBuilder::new(
        PropValue::List(vec![]),
        PropValue::lambda(5),
        PropValue::lambda(6),
    )
    .build();
    assert!(validate_list_node(&list).is_empty());

    // Feedback
    let modal = ModalBuilder::new(false, PropValue::action("close")).build();
    assert!(validate_feedback_node(&modal).is_empty());

    let toast = ToastBuilder::new("Done").build();
    assert!(validate_feedback_node(&toast).is_empty());
}

// ══════════════════════════════════════════════════════════════════════════════
// Edge cases
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn empty_label_is_valid() {
    let info = AccessibilityInfo::new("");
    let prop = info.to_prop_value();
    // Validation should pass (empty label is semantically odd but structurally valid)
    let errors = validate_accessible_prop("Text", &prop);
    assert!(errors.is_empty());
}

#[test]
fn validate_multiple_errors() {
    let mut fields = BTreeMap::new();
    // Missing label, wrong hint type, invalid role, unknown field
    fields.insert("hint".to_string(), PropValue::Number(42.0));
    fields.insert("role".to_string(), PropValue::String("banana".to_string()));
    fields.insert("foo".to_string(), PropValue::Nil);
    let prop = PropValue::Record(fields);
    let errors = validate_accessible_prop("Button", &prop);
    assert!(errors.len() >= 3, "Expected 3+ errors, got: {:?}", errors);
}

#[test]
fn all_valid_roles_pass_validation() {
    for role_str in SemanticRole::valid_values() {
        let mut fields = BTreeMap::new();
        fields.insert("label".to_string(), PropValue::String("test".to_string()));
        fields.insert(
            "role".to_string(),
            PropValue::String(role_str.to_string()),
        );
        let prop = PropValue::Record(fields);
        let errors = validate_accessible_prop("Button", &prop);
        assert!(
            errors.is_empty(),
            "Role '{role_str}' should be valid: {:?}",
            errors
        );
    }
}
