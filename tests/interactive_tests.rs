//! Tests for interactive components — Button and TextInput.
//!
//! Covers construction (builder), JSON round-trip, validation, and
//! determinism. Follows the same pattern as content_tests.rs.

use pepl_ui::{
    ButtonBuilder, ButtonVariant, KeyboardType, PropValue, Surface, SurfaceNode,
    TextInputBuilder, validate_interactive_node,
};

// ══════════════════════════════════════════════════════════════════════════════
// Button — Construction
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn button_minimal() {
    let node = ButtonBuilder::new("Save", PropValue::action("save")).build();
    assert_eq!(node.component_type, "Button");
    assert_eq!(node.props.get("label"), Some(&PropValue::String("Save".into())));
    assert!(matches!(node.props.get("on_tap"), Some(PropValue::ActionRef { .. })));
    assert!(node.children.is_empty());
}

#[test]
fn button_with_variant_filled() {
    let node = ButtonBuilder::new("OK", PropValue::action("ok"))
        .variant(ButtonVariant::Filled)
        .build();
    assert_eq!(
        node.props.get("variant"),
        Some(&PropValue::String("filled".into()))
    );
}

#[test]
fn button_with_variant_outlined() {
    let node = ButtonBuilder::new("Cancel", PropValue::action("cancel"))
        .variant(ButtonVariant::Outlined)
        .build();
    assert_eq!(
        node.props.get("variant"),
        Some(&PropValue::String("outlined".into()))
    );
}

#[test]
fn button_with_variant_text() {
    let node = ButtonBuilder::new("Link", PropValue::action("go"))
        .variant(ButtonVariant::Text)
        .build();
    assert_eq!(
        node.props.get("variant"),
        Some(&PropValue::String("text".into()))
    );
}

#[test]
fn button_with_icon() {
    let node = ButtonBuilder::new("Delete", PropValue::action("del"))
        .icon("trash")
        .build();
    assert_eq!(
        node.props.get("icon"),
        Some(&PropValue::String("trash".into()))
    );
}

#[test]
fn button_disabled() {
    let node = ButtonBuilder::new("Submit", PropValue::action("submit"))
        .disabled(true)
        .build();
    assert_eq!(node.props.get("disabled"), Some(&PropValue::Bool(true)));
}

#[test]
fn button_loading() {
    let node = ButtonBuilder::new("Submit", PropValue::action("submit"))
        .loading(true)
        .build();
    assert_eq!(node.props.get("loading"), Some(&PropValue::Bool(true)));
}

#[test]
fn button_all_props() {
    let node = ButtonBuilder::new("Action", PropValue::action_with_args("do_thing", vec![PropValue::Number(42.0)]))
        .variant(ButtonVariant::Outlined)
        .icon("star")
        .disabled(false)
        .loading(true)
        .build();

    assert_eq!(node.component_type, "Button");
    assert_eq!(node.props.get("label"), Some(&PropValue::String("Action".into())));
    assert!(matches!(node.props.get("on_tap"), Some(PropValue::ActionRef { .. })));
    assert_eq!(node.props.get("variant"), Some(&PropValue::String("outlined".into())));
    assert_eq!(node.props.get("icon"), Some(&PropValue::String("star".into())));
    assert_eq!(node.props.get("disabled"), Some(&PropValue::Bool(false)));
    assert_eq!(node.props.get("loading"), Some(&PropValue::Bool(true)));
    assert!(node.children.is_empty());
}

#[test]
fn button_action_with_args() {
    let node = ButtonBuilder::new("Add", PropValue::action_with_args("add_item", vec![
        PropValue::String("foo".into()),
        PropValue::Number(1.0),
    ])).build();

    match node.props.get("on_tap") {
        Some(PropValue::ActionRef { action, args }) => {
            assert_eq!(action, "add_item");
            let args = args.as_ref().unwrap();
            assert_eq!(args.len(), 2);
            assert_eq!(args[0], PropValue::String("foo".into()));
            assert_eq!(args[1], PropValue::Number(1.0));
        }
        other => panic!("Expected ActionRef, got {:?}", other),
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Button — JSON round-trip
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn button_json_roundtrip() {
    let node = ButtonBuilder::new("Go", PropValue::action("navigate"))
        .variant(ButtonVariant::Filled)
        .disabled(false)
        .build();
    let surface = Surface::new(node);
    let json = surface.to_json();
    let parsed: Surface = serde_json::from_str(&json).unwrap();
    assert_eq!(surface, parsed);
}

#[test]
fn button_json_roundtrip_with_args() {
    let node = ButtonBuilder::new("X", PropValue::action_with_args("remove", vec![PropValue::Number(99.0)]))
        .build();
    let surface = Surface::new(node);
    let json = surface.to_json();
    let parsed: Surface = serde_json::from_str(&json).unwrap();
    assert_eq!(surface, parsed);
}

// ══════════════════════════════════════════════════════════════════════════════
// Button — Validation
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn button_valid_minimal() {
    let node = ButtonBuilder::new("OK", PropValue::action("ok")).build();
    assert!(validate_interactive_node(&node).is_empty());
}

#[test]
fn button_valid_all_props() {
    let node = ButtonBuilder::new("Go", PropValue::action("go"))
        .variant(ButtonVariant::Text)
        .icon("arrow")
        .disabled(true)
        .loading(false)
        .build();
    assert!(validate_interactive_node(&node).is_empty());
}

#[test]
fn button_missing_label() {
    let mut node = SurfaceNode::new("Button");
    node.set_prop("on_tap", PropValue::action("tap"));
    let errors = validate_interactive_node(&node);
    assert!(errors.iter().any(|e| e.contains("label") && e.contains("required")));
}

#[test]
fn button_missing_on_tap() {
    let mut node = SurfaceNode::new("Button");
    node.set_prop("label", PropValue::String("Click".into()));
    let errors = validate_interactive_node(&node);
    assert!(errors.iter().any(|e| e.contains("on_tap") && e.contains("required")));
}

#[test]
fn button_wrong_label_type() {
    let mut node = SurfaceNode::new("Button");
    node.set_prop("label", PropValue::Number(42.0));
    node.set_prop("on_tap", PropValue::action("x"));
    let errors = validate_interactive_node(&node);
    assert!(errors.iter().any(|e| e.contains("label") && e.contains("string")));
}

#[test]
fn button_wrong_on_tap_type() {
    let mut node = SurfaceNode::new("Button");
    node.set_prop("label", PropValue::String("A".into()));
    node.set_prop("on_tap", PropValue::String("not_an_action".into()));
    let errors = validate_interactive_node(&node);
    assert!(errors.iter().any(|e| e.contains("on_tap") && e.contains("action")));
}

#[test]
fn button_invalid_variant() {
    let mut node = SurfaceNode::new("Button");
    node.set_prop("label", PropValue::String("B".into()));
    node.set_prop("on_tap", PropValue::action("b"));
    node.set_prop("variant", PropValue::String("neon".into()));
    let errors = validate_interactive_node(&node);
    assert!(errors.iter().any(|e| e.contains("variant")));
}

#[test]
fn button_wrong_icon_type() {
    let mut node = SurfaceNode::new("Button");
    node.set_prop("label", PropValue::String("C".into()));
    node.set_prop("on_tap", PropValue::action("c"));
    node.set_prop("icon", PropValue::Number(123.0));
    let errors = validate_interactive_node(&node);
    assert!(errors.iter().any(|e| e.contains("icon") && e.contains("string")));
}

#[test]
fn button_wrong_disabled_type() {
    let mut node = SurfaceNode::new("Button");
    node.set_prop("label", PropValue::String("D".into()));
    node.set_prop("on_tap", PropValue::action("d"));
    node.set_prop("disabled", PropValue::String("yes".into()));
    let errors = validate_interactive_node(&node);
    assert!(errors.iter().any(|e| e.contains("disabled") && e.contains("bool")));
}

#[test]
fn button_wrong_loading_type() {
    let mut node = SurfaceNode::new("Button");
    node.set_prop("label", PropValue::String("E".into()));
    node.set_prop("on_tap", PropValue::action("e"));
    node.set_prop("loading", PropValue::Number(1.0));
    let errors = validate_interactive_node(&node);
    assert!(errors.iter().any(|e| e.contains("loading") && e.contains("bool")));
}

#[test]
fn button_unknown_prop() {
    let mut node = SurfaceNode::new("Button");
    node.set_prop("label", PropValue::String("F".into()));
    node.set_prop("on_tap", PropValue::action("f"));
    node.set_prop("color", PropValue::color(1.0, 0.0, 0.0, 1.0));
    let errors = validate_interactive_node(&node);
    assert!(errors.iter().any(|e| e.contains("unknown prop") && e.contains("color")));
}

#[test]
fn button_no_children() {
    let mut node = ButtonBuilder::new("G", PropValue::action("g")).build();
    node.children.push(SurfaceNode::new("Text"));
    let errors = validate_interactive_node(&node);
    assert!(errors.iter().any(|e| e.contains("children")));
}

#[test]
fn button_multiple_errors() {
    let node = SurfaceNode::new("Button"); // missing label AND on_tap
    let errors = validate_interactive_node(&node);
    assert!(errors.len() >= 2);
}

// ══════════════════════════════════════════════════════════════════════════════
// TextInput — Construction
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn text_input_minimal() {
    let node = TextInputBuilder::new("hello", PropValue::lambda(1)).build();
    assert_eq!(node.component_type, "TextInput");
    assert_eq!(node.props.get("value"), Some(&PropValue::String("hello".into())));
    assert!(matches!(node.props.get("on_change"), Some(PropValue::Lambda { .. })));
    assert!(node.children.is_empty());
}

#[test]
fn text_input_with_placeholder() {
    let node = TextInputBuilder::new("", PropValue::lambda(2))
        .placeholder("Type here...")
        .build();
    assert_eq!(
        node.props.get("placeholder"),
        Some(&PropValue::String("Type here...".into()))
    );
}

#[test]
fn text_input_with_label() {
    let node = TextInputBuilder::new("", PropValue::lambda(3))
        .label("Username")
        .build();
    assert_eq!(
        node.props.get("label"),
        Some(&PropValue::String("Username".into()))
    );
}

#[test]
fn text_input_keyboard_text() {
    let node = TextInputBuilder::new("", PropValue::lambda(4))
        .keyboard(KeyboardType::Text)
        .build();
    assert_eq!(
        node.props.get("keyboard"),
        Some(&PropValue::String("text".into()))
    );
}

#[test]
fn text_input_keyboard_number() {
    let node = TextInputBuilder::new("", PropValue::lambda(5))
        .keyboard(KeyboardType::Number)
        .build();
    assert_eq!(
        node.props.get("keyboard"),
        Some(&PropValue::String("number".into()))
    );
}

#[test]
fn text_input_keyboard_email() {
    let node = TextInputBuilder::new("", PropValue::lambda(6))
        .keyboard(KeyboardType::Email)
        .build();
    assert_eq!(
        node.props.get("keyboard"),
        Some(&PropValue::String("email".into()))
    );
}

#[test]
fn text_input_keyboard_phone() {
    let node = TextInputBuilder::new("", PropValue::lambda(7))
        .keyboard(KeyboardType::Phone)
        .build();
    assert_eq!(
        node.props.get("keyboard"),
        Some(&PropValue::String("phone".into()))
    );
}

#[test]
fn text_input_keyboard_url() {
    let node = TextInputBuilder::new("", PropValue::lambda(8))
        .keyboard(KeyboardType::Url)
        .build();
    assert_eq!(
        node.props.get("keyboard"),
        Some(&PropValue::String("url".into()))
    );
}

#[test]
fn text_input_max_length() {
    let node = TextInputBuilder::new("", PropValue::lambda(9))
        .max_length(100.0)
        .build();
    assert_eq!(node.props.get("max_length"), Some(&PropValue::Number(100.0)));
}

#[test]
fn text_input_multiline() {
    let node = TextInputBuilder::new("", PropValue::lambda(10))
        .multiline(true)
        .build();
    assert_eq!(node.props.get("multiline"), Some(&PropValue::Bool(true)));
}

#[test]
fn text_input_all_props() {
    let node = TextInputBuilder::new("initial", PropValue::lambda(11))
        .placeholder("Enter text")
        .label("Notes")
        .keyboard(KeyboardType::Text)
        .max_length(500.0)
        .multiline(true)
        .build();

    assert_eq!(node.component_type, "TextInput");
    assert_eq!(node.props.get("value"), Some(&PropValue::String("initial".into())));
    assert!(matches!(node.props.get("on_change"), Some(PropValue::Lambda { lambda_id: 11 })));
    assert_eq!(node.props.get("placeholder"), Some(&PropValue::String("Enter text".into())));
    assert_eq!(node.props.get("label"), Some(&PropValue::String("Notes".into())));
    assert_eq!(node.props.get("keyboard"), Some(&PropValue::String("text".into())));
    assert_eq!(node.props.get("max_length"), Some(&PropValue::Number(500.0)));
    assert_eq!(node.props.get("multiline"), Some(&PropValue::Bool(true)));
    assert!(node.children.is_empty());
}

// ══════════════════════════════════════════════════════════════════════════════
// TextInput — JSON round-trip
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn text_input_json_roundtrip() {
    let node = TextInputBuilder::new("value", PropValue::lambda(20))
        .placeholder("...")
        .keyboard(KeyboardType::Email)
        .build();
    let surface = Surface::new(node);
    let json = surface.to_json();
    let parsed: Surface = serde_json::from_str(&json).unwrap();
    assert_eq!(surface, parsed);
}

#[test]
fn text_input_json_roundtrip_all() {
    let node = TextInputBuilder::new("abc", PropValue::lambda(21))
        .placeholder("p")
        .label("L")
        .keyboard(KeyboardType::Url)
        .max_length(50.0)
        .multiline(false)
        .build();
    let surface = Surface::new(node);
    let json = surface.to_json();
    let parsed: Surface = serde_json::from_str(&json).unwrap();
    assert_eq!(surface, parsed);
}

// ══════════════════════════════════════════════════════════════════════════════
// TextInput — Validation
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn text_input_valid_minimal() {
    let node = TextInputBuilder::new("x", PropValue::lambda(30)).build();
    assert!(validate_interactive_node(&node).is_empty());
}

#[test]
fn text_input_valid_all_props() {
    let node = TextInputBuilder::new("y", PropValue::lambda(31))
        .placeholder("p")
        .label("l")
        .keyboard(KeyboardType::Phone)
        .max_length(10.0)
        .multiline(true)
        .build();
    assert!(validate_interactive_node(&node).is_empty());
}

#[test]
fn text_input_missing_value() {
    let mut node = SurfaceNode::new("TextInput");
    node.set_prop("on_change", PropValue::lambda(32));
    let errors = validate_interactive_node(&node);
    assert!(errors.iter().any(|e| e.contains("value") && e.contains("required")));
}

#[test]
fn text_input_missing_on_change() {
    let mut node = SurfaceNode::new("TextInput");
    node.set_prop("value", PropValue::String("v".into()));
    let errors = validate_interactive_node(&node);
    assert!(errors.iter().any(|e| e.contains("on_change") && e.contains("required")));
}

#[test]
fn text_input_wrong_value_type() {
    let mut node = SurfaceNode::new("TextInput");
    node.set_prop("value", PropValue::Number(42.0));
    node.set_prop("on_change", PropValue::lambda(33));
    let errors = validate_interactive_node(&node);
    assert!(errors.iter().any(|e| e.contains("value") && e.contains("string")));
}

#[test]
fn text_input_wrong_on_change_type() {
    let mut node = SurfaceNode::new("TextInput");
    node.set_prop("value", PropValue::String("v".into()));
    node.set_prop("on_change", PropValue::action("not_lambda"));
    let errors = validate_interactive_node(&node);
    assert!(errors.iter().any(|e| e.contains("on_change") && e.contains("lambda")));
}

#[test]
fn text_input_invalid_keyboard() {
    let mut node = SurfaceNode::new("TextInput");
    node.set_prop("value", PropValue::String("v".into()));
    node.set_prop("on_change", PropValue::lambda(34));
    node.set_prop("keyboard", PropValue::String("emoji".into()));
    let errors = validate_interactive_node(&node);
    assert!(errors.iter().any(|e| e.contains("keyboard")));
}

#[test]
fn text_input_wrong_placeholder_type() {
    let mut node = SurfaceNode::new("TextInput");
    node.set_prop("value", PropValue::String("v".into()));
    node.set_prop("on_change", PropValue::lambda(35));
    node.set_prop("placeholder", PropValue::Number(0.0));
    let errors = validate_interactive_node(&node);
    assert!(errors.iter().any(|e| e.contains("placeholder") && e.contains("string")));
}

#[test]
fn text_input_wrong_label_type() {
    let mut node = SurfaceNode::new("TextInput");
    node.set_prop("value", PropValue::String("v".into()));
    node.set_prop("on_change", PropValue::lambda(36));
    node.set_prop("label", PropValue::Bool(true));
    let errors = validate_interactive_node(&node);
    assert!(errors.iter().any(|e| e.contains("label") && e.contains("string")));
}

#[test]
fn text_input_wrong_max_length_type() {
    let mut node = SurfaceNode::new("TextInput");
    node.set_prop("value", PropValue::String("v".into()));
    node.set_prop("on_change", PropValue::lambda(37));
    node.set_prop("max_length", PropValue::String("100".into()));
    let errors = validate_interactive_node(&node);
    assert!(errors.iter().any(|e| e.contains("max_length") && e.contains("number")));
}

#[test]
fn text_input_wrong_multiline_type() {
    let mut node = SurfaceNode::new("TextInput");
    node.set_prop("value", PropValue::String("v".into()));
    node.set_prop("on_change", PropValue::lambda(38));
    node.set_prop("multiline", PropValue::String("yes".into()));
    let errors = validate_interactive_node(&node);
    assert!(errors.iter().any(|e| e.contains("multiline") && e.contains("bool")));
}

#[test]
fn text_input_unknown_prop() {
    let mut node = SurfaceNode::new("TextInput");
    node.set_prop("value", PropValue::String("v".into()));
    node.set_prop("on_change", PropValue::lambda(39));
    node.set_prop("autocomplete", PropValue::Bool(true));
    let errors = validate_interactive_node(&node);
    assert!(errors.iter().any(|e| e.contains("unknown prop") && e.contains("autocomplete")));
}

#[test]
fn text_input_no_children() {
    let mut node = TextInputBuilder::new("v", PropValue::lambda(40)).build();
    node.children.push(SurfaceNode::new("Text"));
    let errors = validate_interactive_node(&node);
    assert!(errors.iter().any(|e| e.contains("children")));
}

#[test]
fn text_input_multiple_errors() {
    let node = SurfaceNode::new("TextInput"); // missing value AND on_change
    let errors = validate_interactive_node(&node);
    assert!(errors.len() >= 2);
}

// ══════════════════════════════════════════════════════════════════════════════
// Action reference serialization
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn action_ref_serialization_no_args() {
    let node = ButtonBuilder::new("Tap", PropValue::action("my_action")).build();
    let surface = Surface::new(node);
    let json = surface.to_json();
    assert!(json.contains("\"__action\":\"my_action\""));
    // args should not appear when None
    assert!(!json.contains("__args"));
}

#[test]
fn action_ref_serialization_with_args() {
    let node = ButtonBuilder::new("Tap", PropValue::action_with_args("act", vec![
        PropValue::String("hello".into()),
        PropValue::Number(3.14),
    ])).build();
    let surface = Surface::new(node);
    let json = surface.to_json();
    assert!(json.contains("\"__action\":\"act\""));
    assert!(json.contains("\"__args\""));
    assert!(json.contains("hello"));
    assert!(json.contains("3.14"));
}

#[test]
fn lambda_callback_serialization() {
    let node = TextInputBuilder::new("val", PropValue::lambda(42)).build();
    let surface = Surface::new(node);
    let json = surface.to_json();
    assert!(json.contains("\"__lambda\":42"));
}

// ══════════════════════════════════════════════════════════════════════════════
// Unknown component type
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn unknown_interactive_component() {
    let node = SurfaceNode::new("Slider");
    let errors = validate_interactive_node(&node);
    assert!(errors.iter().any(|e| e.contains("Unknown interactive component")));
}

// ══════════════════════════════════════════════════════════════════════════════
// Determinism
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn interactive_determinism_100_iterations() {
    let build_button = || {
        ButtonBuilder::new("Deterministic", PropValue::action_with_args("act", vec![PropValue::Number(1.0)]))
            .variant(ButtonVariant::Filled)
            .icon("check")
            .disabled(false)
            .loading(true)
            .build()
    };
    let build_input = || {
        TextInputBuilder::new("det", PropValue::lambda(99))
            .placeholder("p")
            .label("l")
            .keyboard(KeyboardType::Email)
            .max_length(100.0)
            .multiline(true)
            .build()
    };

    let button_ref = Surface::new(build_button()).to_json();
    let input_ref = Surface::new(build_input()).to_json();

    for _ in 0..100 {
        assert_eq!(Surface::new(build_button()).to_json(), button_ref);
        assert_eq!(Surface::new(build_input()).to_json(), input_ref);
    }
}
