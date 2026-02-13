//! Tests for feedback components — Modal, Toast (U6).
//!
//! Covers construction, JSON serialization, validation (happy + error),
//! children handling (Modal), and 100-iteration determinism.

use pepl_ui::{
    validate_feedback_node, ModalBuilder, PropValue, Surface, SurfaceNode, TextBuilder,
    ToastBuilder, ToastType,
};

// ══════════════════════════════════════════════════════════════════════════════
// Modal — Construction
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn modal_basic_construction() {
    let node = ModalBuilder::new(true, PropValue::action("close_modal")).build();

    assert_eq!(node.component_type, "Modal");
    assert_eq!(node.props.get("visible"), Some(&PropValue::Bool(true)));
    assert!(matches!(
        node.props.get("on_dismiss"),
        Some(PropValue::ActionRef { .. })
    ));
    assert!(node.children.is_empty());
}

#[test]
fn modal_with_title_and_children() {
    let child = TextBuilder::new("Hello from modal").build();
    let node = ModalBuilder::new(false, PropValue::action("dismiss"))
        .title("My Modal")
        .child(child)
        .build();

    assert_eq!(
        node.props.get("title"),
        Some(&PropValue::String("My Modal".into()))
    );
    assert_eq!(node.children.len(), 1);
    assert_eq!(node.children[0].component_type, "Text");
}

#[test]
fn modal_multiple_children() {
    let node = ModalBuilder::new(true, PropValue::action("close"))
        .child(TextBuilder::new("Line 1").build())
        .child(TextBuilder::new("Line 2").build())
        .build();

    assert_eq!(node.children.len(), 2);
}

// ══════════════════════════════════════════════════════════════════════════════
// Modal — JSON
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn modal_json_round_trip() {
    let node = ModalBuilder::new(true, PropValue::action("close"))
        .title("Confirm")
        .child(TextBuilder::new("Are you sure?").build())
        .build();

    let surface = Surface::new(node);
    let json_str = surface.to_json();
    let parsed: Surface = serde_json::from_str(&json_str).unwrap();
    assert_eq!(surface, parsed);
}

// ══════════════════════════════════════════════════════════════════════════════
// Modal — Validation (happy)
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn modal_valid_minimal() {
    let node = ModalBuilder::new(true, PropValue::action("close")).build();
    let errors = validate_feedback_node(&node);
    assert!(errors.is_empty(), "unexpected errors: {errors:?}");
}

#[test]
fn modal_valid_with_all() {
    let node = ModalBuilder::new(false, PropValue::action("close"))
        .title("Title")
        .child(TextBuilder::new("content").build())
        .build();
    let errors = validate_feedback_node(&node);
    assert!(errors.is_empty(), "unexpected errors: {errors:?}");
}

// ══════════════════════════════════════════════════════════════════════════════
// Modal — Validation (errors)
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn modal_missing_visible() {
    let mut node = SurfaceNode::new("Modal");
    node.set_prop("on_dismiss", PropValue::action("close"));

    let errors = validate_feedback_node(&node);
    assert!(errors
        .iter()
        .any(|e| e.contains("visible") && e.contains("required")));
}

#[test]
fn modal_missing_on_dismiss() {
    let mut node = SurfaceNode::new("Modal");
    node.set_prop("visible", PropValue::Bool(true));

    let errors = validate_feedback_node(&node);
    assert!(errors
        .iter()
        .any(|e| e.contains("on_dismiss") && e.contains("required")));
}

#[test]
fn modal_wrong_visible_type() {
    let mut node = SurfaceNode::new("Modal");
    node.set_prop("visible", PropValue::String("yes".into()));
    node.set_prop("on_dismiss", PropValue::action("close"));

    let errors = validate_feedback_node(&node);
    assert!(errors
        .iter()
        .any(|e| e.contains("visible") && e.contains("expected bool")));
}

#[test]
fn modal_wrong_on_dismiss_type() {
    let mut node = SurfaceNode::new("Modal");
    node.set_prop("visible", PropValue::Bool(true));
    node.set_prop("on_dismiss", PropValue::String("close".into()));

    let errors = validate_feedback_node(&node);
    assert!(errors
        .iter()
        .any(|e| e.contains("on_dismiss") && e.contains("expected action")));
}

#[test]
fn modal_unknown_prop() {
    let mut node = SurfaceNode::new("Modal");
    node.set_prop("visible", PropValue::Bool(true));
    node.set_prop("on_dismiss", PropValue::action("close"));
    node.set_prop("size", PropValue::String("large".into()));

    let errors = validate_feedback_node(&node);
    assert!(errors
        .iter()
        .any(|e| e.contains("unknown prop") && e.contains("size")));
}

// ══════════════════════════════════════════════════════════════════════════════
// Toast — Construction
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn toast_basic_construction() {
    let node = ToastBuilder::new("Saved!").build();

    assert_eq!(node.component_type, "Toast");
    assert_eq!(
        node.props.get("message"),
        Some(&PropValue::String("Saved!".into()))
    );
    assert!(node.children.is_empty());
}

#[test]
fn toast_with_all_props() {
    let node = ToastBuilder::new("Error occurred")
        .duration(3000.0)
        .toast_type(ToastType::Error)
        .build();

    assert_eq!(node.props.get("duration"), Some(&PropValue::Number(3000.0)));
    assert_eq!(
        node.props.get("type"),
        Some(&PropValue::String("error".into()))
    );
}

#[test]
fn toast_all_types() {
    for (tt, expected) in [
        (ToastType::Info, "info"),
        (ToastType::Success, "success"),
        (ToastType::Warning, "warning"),
        (ToastType::Error, "error"),
    ] {
        let node = ToastBuilder::new("msg").toast_type(tt).build();
        assert_eq!(
            node.props.get("type"),
            Some(&PropValue::String(expected.into()))
        );
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Toast — JSON
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn toast_json_round_trip() {
    let node = ToastBuilder::new("Done!")
        .duration(2000.0)
        .toast_type(ToastType::Success)
        .build();

    let surface = Surface::new(node);
    let json_str = surface.to_json();
    let parsed: Surface = serde_json::from_str(&json_str).unwrap();
    assert_eq!(surface, parsed);
}

// ══════════════════════════════════════════════════════════════════════════════
// Toast — Validation (happy)
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn toast_valid_minimal() {
    let node = ToastBuilder::new("hello").build();
    let errors = validate_feedback_node(&node);
    assert!(errors.is_empty(), "unexpected errors: {errors:?}");
}

#[test]
fn toast_valid_with_all() {
    let node = ToastBuilder::new("msg")
        .duration(5000.0)
        .toast_type(ToastType::Warning)
        .build();
    let errors = validate_feedback_node(&node);
    assert!(errors.is_empty(), "unexpected errors: {errors:?}");
}

// ══════════════════════════════════════════════════════════════════════════════
// Toast — Validation (errors)
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn toast_missing_message() {
    let node = SurfaceNode::new("Toast");
    let errors = validate_feedback_node(&node);
    assert!(errors
        .iter()
        .any(|e| e.contains("message") && e.contains("required")));
}

#[test]
fn toast_wrong_message_type() {
    let mut node = SurfaceNode::new("Toast");
    node.set_prop("message", PropValue::Number(42.0));

    let errors = validate_feedback_node(&node);
    assert!(errors
        .iter()
        .any(|e| e.contains("message") && e.contains("expected string")));
}

#[test]
fn toast_wrong_duration_type() {
    let mut node = SurfaceNode::new("Toast");
    node.set_prop("message", PropValue::String("msg".into()));
    node.set_prop("duration", PropValue::String("3000".into()));

    let errors = validate_feedback_node(&node);
    assert!(errors
        .iter()
        .any(|e| e.contains("duration") && e.contains("expected number")));
}

#[test]
fn toast_invalid_type_enum() {
    let mut node = SurfaceNode::new("Toast");
    node.set_prop("message", PropValue::String("msg".into()));
    node.set_prop("type", PropValue::String("critical".into()));

    let errors = validate_feedback_node(&node);
    assert!(errors
        .iter()
        .any(|e| e.contains("type") && e.contains("expected one of")));
}

#[test]
fn toast_no_children_allowed() {
    let mut node = ToastBuilder::new("msg").build();
    node.add_child(SurfaceNode::new("Text"));

    let errors = validate_feedback_node(&node);
    assert!(errors.iter().any(|e| e.contains("children")));
}

#[test]
fn toast_unknown_prop() {
    let mut node = SurfaceNode::new("Toast");
    node.set_prop("message", PropValue::String("msg".into()));
    node.set_prop("color", PropValue::String("red".into()));

    let errors = validate_feedback_node(&node);
    assert!(errors
        .iter()
        .any(|e| e.contains("unknown prop") && e.contains("color")));
}

// ══════════════════════════════════════════════════════════════════════════════
// Determinism
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn modal_determinism_100() {
    let build = || {
        ModalBuilder::new(true, PropValue::action("close"))
            .title("Confirm Delete")
            .child(TextBuilder::new("This cannot be undone.").build())
            .build()
    };

    let reference = Surface::new(build()).to_json();
    for _ in 0..100 {
        assert_eq!(Surface::new(build()).to_json(), reference);
    }
}

#[test]
fn toast_determinism_100() {
    let build = || {
        ToastBuilder::new("Changes saved")
            .duration(3000.0)
            .toast_type(ToastType::Success)
            .build()
    };

    let reference = Surface::new(build()).to_json();
    for _ in 0..100 {
        assert_eq!(Surface::new(build()).to_json(), reference);
    }
}
