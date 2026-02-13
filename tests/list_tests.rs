//! Tests for ScrollList component (U5).
//!
//! Covers construction, JSON serialization, validation (happy + error),
//! and 100-iteration determinism.

use pepl_ui::{validate_list_node, PropValue, ScrollListBuilder, Surface, SurfaceNode};

// ══════════════════════════════════════════════════════════════════════════════
// Construction
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn scroll_list_basic_construction() {
    let node = ScrollListBuilder::new(
        PropValue::List(vec![
            PropValue::String("a".into()),
            PropValue::String("b".into()),
        ]),
        PropValue::lambda(1),
        PropValue::lambda(1),
    )
    .build();

    assert_eq!(node.component_type, "ScrollList");
    assert!(matches!(node.props.get("items"), Some(PropValue::List(_))));
    assert!(matches!(
        node.props.get("render"),
        Some(PropValue::Lambda { .. })
    ));
    assert!(matches!(
        node.props.get("key"),
        Some(PropValue::Lambda { .. })
    ));
    assert!(node.children.is_empty());
}

#[test]
fn scroll_list_with_all_props() {
    let node = ScrollListBuilder::new(
        PropValue::List(vec![PropValue::Number(1.0)]),
        PropValue::lambda(1),
        PropValue::lambda(1),
    )
    .on_reorder(PropValue::lambda(1))
    .dividers(true)
    .build();

    assert!(matches!(
        node.props.get("on_reorder"),
        Some(PropValue::Lambda { .. })
    ));
    assert_eq!(node.props.get("dividers"), Some(&PropValue::Bool(true)));
}

#[test]
fn scroll_list_empty_items() {
    let node = ScrollListBuilder::new(
        PropValue::List(vec![]),
        PropValue::lambda(1),
        PropValue::lambda(1),
    )
    .build();

    if let Some(PropValue::List(items)) = node.props.get("items") {
        assert!(items.is_empty());
    } else {
        panic!("expected list");
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// JSON serialization
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn scroll_list_json_round_trip() {
    let node = ScrollListBuilder::new(
        PropValue::List(vec![PropValue::String("item1".into())]),
        PropValue::lambda(1),
        PropValue::lambda(1),
    )
    .dividers(true)
    .build();

    let surface = Surface::new(node);
    let json_str = surface.to_json();
    let parsed: Surface = serde_json::from_str(&json_str).unwrap();
    assert_eq!(surface, parsed);
}

// ══════════════════════════════════════════════════════════════════════════════
// Validation (happy path)
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn scroll_list_valid() {
    let node = ScrollListBuilder::new(
        PropValue::List(vec![]),
        PropValue::lambda(1),
        PropValue::lambda(1),
    )
    .build();

    let errors = validate_list_node(&node);
    assert!(errors.is_empty(), "unexpected errors: {errors:?}");
}

#[test]
fn scroll_list_valid_with_all_optional() {
    let node = ScrollListBuilder::new(
        PropValue::List(vec![PropValue::Number(1.0)]),
        PropValue::lambda(1),
        PropValue::lambda(1),
    )
    .on_reorder(PropValue::lambda(1))
    .dividers(false)
    .build();

    let errors = validate_list_node(&node);
    assert!(errors.is_empty(), "unexpected errors: {errors:?}");
}

// ══════════════════════════════════════════════════════════════════════════════
// Validation (error cases)
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn scroll_list_missing_items() {
    let mut node = SurfaceNode::new("ScrollList");
    node.set_prop("render", PropValue::lambda(1));
    node.set_prop("key", PropValue::lambda(1));

    let errors = validate_list_node(&node);
    assert!(errors.iter().any(|e| e.contains("items") && e.contains("required")));
}

#[test]
fn scroll_list_missing_render() {
    let mut node = SurfaceNode::new("ScrollList");
    node.set_prop("items", PropValue::List(vec![]));
    node.set_prop("key", PropValue::lambda(1));

    let errors = validate_list_node(&node);
    assert!(errors.iter().any(|e| e.contains("render") && e.contains("required")));
}

#[test]
fn scroll_list_missing_key() {
    let mut node = SurfaceNode::new("ScrollList");
    node.set_prop("items", PropValue::List(vec![]));
    node.set_prop("render", PropValue::lambda(1));

    let errors = validate_list_node(&node);
    assert!(errors.iter().any(|e| e.contains("key") && e.contains("required")));
}

#[test]
fn scroll_list_wrong_items_type() {
    let mut node = SurfaceNode::new("ScrollList");
    node.set_prop("items", PropValue::String("not a list".into()));
    node.set_prop("render", PropValue::lambda(1));
    node.set_prop("key", PropValue::lambda(1));

    let errors = validate_list_node(&node);
    assert!(errors.iter().any(|e| e.contains("items") && e.contains("expected list")));
}

#[test]
fn scroll_list_wrong_render_type() {
    let mut node = SurfaceNode::new("ScrollList");
    node.set_prop("items", PropValue::List(vec![]));
    node.set_prop("render", PropValue::String("not lambda".into()));
    node.set_prop("key", PropValue::lambda(1));

    let errors = validate_list_node(&node);
    assert!(errors.iter().any(|e| e.contains("render") && e.contains("expected lambda")));
}

#[test]
fn scroll_list_wrong_on_reorder_type() {
    let mut node = SurfaceNode::new("ScrollList");
    node.set_prop("items", PropValue::List(vec![]));
    node.set_prop("render", PropValue::lambda(1));
    node.set_prop("key", PropValue::lambda(1));
    node.set_prop("on_reorder", PropValue::Number(42.0));

    let errors = validate_list_node(&node);
    assert!(errors.iter().any(|e| e.contains("on_reorder") && e.contains("expected lambda")));
}

#[test]
fn scroll_list_no_children_allowed() {
    let mut node = ScrollListBuilder::new(
        PropValue::List(vec![]),
        PropValue::lambda(1),
        PropValue::lambda(1),
    )
    .build();
    node.add_child(SurfaceNode::new("Text"));

    let errors = validate_list_node(&node);
    assert!(errors.iter().any(|e| e.contains("children")));
}

#[test]
fn scroll_list_unknown_prop() {
    let mut node = SurfaceNode::new("ScrollList");
    node.set_prop("items", PropValue::List(vec![]));
    node.set_prop("render", PropValue::lambda(1));
    node.set_prop("key", PropValue::lambda(1));
    node.set_prop("foo", PropValue::Bool(true));

    let errors = validate_list_node(&node);
    assert!(errors.iter().any(|e| e.contains("unknown prop") && e.contains("foo")));
}

// ══════════════════════════════════════════════════════════════════════════════
// Determinism
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn scroll_list_determinism_100() {
    let build = || {
        ScrollListBuilder::new(
            PropValue::List(vec![
                PropValue::String("alpha".into()),
                PropValue::String("beta".into()),
                PropValue::String("gamma".into()),
            ]),
            PropValue::lambda(1),
            PropValue::lambda(1),
        )
        .dividers(true)
        .on_reorder(PropValue::lambda(1))
        .build()
    };

    let reference = Surface::new(build()).to_json();
    for _ in 0..100 {
        assert_eq!(Surface::new(build()).to_json(), reference);
    }
}
