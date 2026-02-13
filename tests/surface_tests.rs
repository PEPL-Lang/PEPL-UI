//! Integration tests for `pepl-ui` Phase 1: Surface tree types, component registry, shared types.

use pepl_ui::{
    Alignment, BorderStyle, ColorValue, ComponentRegistry, Dimension, Edges, PropRequirement,
    PropValue, ShadowStyle, Surface, SurfaceNode,
};
use std::collections::BTreeMap;

// ── Helpers ───────────────────────────────────────────────────────────────────

fn registry() -> ComponentRegistry {
    ComponentRegistry::new()
}

// ══════════════════════════════════════════════════════════════════════════════
// SurfaceNode construction tests
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_surface_node_new() {
    let node = SurfaceNode::new("Text");
    assert_eq!(node.component_type, "Text");
    assert!(node.props.is_empty());
    assert!(node.children.is_empty());
}

#[test]
fn test_surface_node_builder() {
    let node = SurfaceNode::new("Text")
        .with_prop("value", PropValue::String("Hello".into()))
        .with_prop("size", PropValue::String("title".into()));
    assert_eq!(node.props.len(), 2);
    assert_eq!(node.props["value"], PropValue::String("Hello".into()));
}

#[test]
fn test_surface_node_with_child() {
    let child = SurfaceNode::new("Text").with_prop("value", PropValue::String("Hi".into()));
    let parent = SurfaceNode::new("Column").with_child(child);
    assert_eq!(parent.children.len(), 1);
    assert_eq!(parent.children[0].component_type, "Text");
}

#[test]
fn test_surface_node_mutable_set_prop() {
    let mut node = SurfaceNode::new("Button");
    node.set_prop("label", PropValue::String("OK".into()));
    assert_eq!(node.props["label"], PropValue::String("OK".into()));
}

#[test]
fn test_surface_node_mutable_add_child() {
    let mut parent = SurfaceNode::new("Row");
    parent.add_child(SurfaceNode::new("Text"));
    parent.add_child(SurfaceNode::new("Button"));
    assert_eq!(parent.children.len(), 2);
}

// ══════════════════════════════════════════════════════════════════════════════
// Surface JSON serialization tests
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_surface_to_json_simple() {
    let surface = Surface::new(
        SurfaceNode::new("Text").with_prop("value", PropValue::String("Hello".into())),
    );
    let json = surface.to_json();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["root"]["type"], "Text");
    assert_eq!(parsed["root"]["props"]["value"], "Hello");
    assert_eq!(parsed["root"]["children"], serde_json::json!([]));
}

#[test]
fn test_surface_to_json_nested() {
    let surface = Surface::new(
        SurfaceNode::new("Column")
            .with_prop("spacing", PropValue::Number(8.0))
            .with_child(
                SurfaceNode::new("Text").with_prop("value", PropValue::String("Title".into())),
            )
            .with_child(
                SurfaceNode::new("Button")
                    .with_prop("label", PropValue::String("Click".into()))
                    .with_prop("on_tap", PropValue::action("increment")),
            ),
    );
    let json = surface.to_json();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["root"]["type"], "Column");
    assert_eq!(parsed["root"]["props"]["spacing"], 8.0);
    assert_eq!(parsed["root"]["children"].as_array().unwrap().len(), 2);
    assert_eq!(parsed["root"]["children"][0]["type"], "Text");
    assert_eq!(parsed["root"]["children"][1]["type"], "Button");
    assert_eq!(
        parsed["root"]["children"][1]["props"]["on_tap"]["__action"],
        "increment"
    );
}

#[test]
fn test_surface_roundtrip_json() {
    let surface = Surface::new(
        SurfaceNode::new("Row")
            .with_prop("spacing", PropValue::Number(4.0))
            .with_child(SurfaceNode::new("Text").with_prop("value", "A".into())),
    );
    let json = surface.to_json();
    let deserialized: Surface = serde_json::from_str(&json).unwrap();
    assert_eq!(surface, deserialized);
}

// ══════════════════════════════════════════════════════════════════════════════
// PropValue tests
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_prop_value_type_names() {
    assert_eq!(PropValue::String("x".into()).type_name(), "string");
    assert_eq!(PropValue::Number(1.0).type_name(), "number");
    assert_eq!(PropValue::Bool(true).type_name(), "bool");
    assert_eq!(PropValue::Nil.type_name(), "nil");
    assert_eq!(PropValue::color(1.0, 0.0, 0.0, 1.0).type_name(), "color");
    assert_eq!(PropValue::action("foo").type_name(), "action");
    assert_eq!(PropValue::lambda(1).type_name(), "lambda");
    assert_eq!(PropValue::List(vec![]).type_name(), "list");
    assert_eq!(PropValue::Record(BTreeMap::new()).type_name(), "record");
}

#[test]
fn test_prop_value_action_ref_json() {
    let action = PropValue::action("increment");
    let json = serde_json::to_string(&action).unwrap();
    assert!(json.contains("\"__action\":\"increment\""));
    assert!(!json.contains("__args"));
}

#[test]
fn test_prop_value_action_ref_with_args_json() {
    let action = PropValue::action_with_args("set_count", vec![PropValue::Number(5.0)]);
    let json = serde_json::to_string(&action).unwrap();
    assert!(json.contains("\"__action\":\"set_count\""));
    assert!(json.contains("\"__args\":[5.0]"));
}

#[test]
fn test_prop_value_lambda_json() {
    let lambda = PropValue::lambda(42);
    let json = serde_json::to_string(&lambda).unwrap();
    assert!(json.contains("\"__lambda\":42"));
}

#[test]
fn test_prop_value_color_json() {
    let color = PropValue::color(1.0, 0.5, 0.0, 1.0);
    let json = serde_json::to_string(&color).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["r"], 1.0);
    assert_eq!(parsed["g"], 0.5);
    assert_eq!(parsed["b"], 0.0);
    assert_eq!(parsed["a"], 1.0);
}

#[test]
fn test_prop_value_from_str() {
    let v: PropValue = "hello".into();
    assert_eq!(v, PropValue::String("hello".into()));
}

#[test]
fn test_prop_value_from_f64() {
    let v: PropValue = 3.15.into();
    assert_eq!(v, PropValue::Number(3.15));
}

#[test]
fn test_prop_value_from_bool() {
    let v: PropValue = true.into();
    assert_eq!(v, PropValue::Bool(true));
}

// ══════════════════════════════════════════════════════════════════════════════
// Shared types tests
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_dimension_from_number() {
    assert_eq!(Dimension::from_number(100.0), Dimension::Px(100.0));
}

#[test]
fn test_dimension_variants() {
    assert_eq!(Dimension::Auto, Dimension::Auto);
    assert_eq!(Dimension::Fill, Dimension::Fill);
    assert_eq!(Dimension::Percent(50.0), Dimension::Percent(50.0));
    assert_ne!(Dimension::Px(10.0), Dimension::Px(20.0));
}

#[test]
fn test_dimension_json_serialization() {
    let px = serde_json::to_string(&Dimension::Px(100.0)).unwrap();
    assert!(px.contains("\"Px\""));
    assert!(px.contains("100"));

    let auto = serde_json::to_string(&Dimension::Auto).unwrap();
    assert!(auto.contains("\"Auto\""));
}

#[test]
fn test_edges_from_number() {
    assert_eq!(Edges::from_number(16.0), Edges::Uniform(16.0));
}

#[test]
fn test_edges_sides() {
    let edges = Edges::sides(10.0, 20.0, 5.0, 5.0);
    match edges {
        Edges::Sides {
            top,
            bottom,
            start,
            end,
        } => {
            assert_eq!(top, 10.0);
            assert_eq!(bottom, 20.0);
            assert_eq!(start, 5.0);
            assert_eq!(end, 5.0);
        }
        _ => panic!("expected Sides"),
    }
}

#[test]
fn test_edges_json_serialization() {
    let uniform = serde_json::to_string(&Edges::Uniform(16.0)).unwrap();
    assert_eq!(uniform, "16.0");

    let sides = serde_json::to_string(&Edges::sides(1.0, 2.0, 3.0, 4.0)).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&sides).unwrap();
    assert_eq!(parsed["top"], 1.0);
    assert_eq!(parsed["bottom"], 2.0);
}

#[test]
fn test_alignment_all_values() {
    let values = [
        Alignment::Start,
        Alignment::Center,
        Alignment::End,
        Alignment::Stretch,
        Alignment::SpaceBetween,
        Alignment::SpaceAround,
    ];
    for val in &values {
        let json = serde_json::to_string(val).unwrap();
        let roundtrip: Alignment = serde_json::from_str(&json).unwrap();
        assert_eq!(*val, roundtrip);
    }
}

#[test]
fn test_alignment_json_snake_case() {
    let json = serde_json::to_string(&Alignment::SpaceBetween).unwrap();
    assert_eq!(json, "\"space_between\"");
}

#[test]
fn test_border_style() {
    let border = BorderStyle {
        width: 2.0,
        color: ColorValue::rgb(0.0, 0.0, 0.0),
        style: None,
    };
    let json = serde_json::to_string(&border).unwrap();
    assert!(!json.contains("style")); // skip_serializing_if None
    assert!(json.contains("\"width\":2.0"));
}

#[test]
fn test_shadow_style() {
    let shadow = ShadowStyle {
        offset_x: 0.0,
        offset_y: 2.0,
        blur: 4.0,
        color: ColorValue::new(0.0, 0.0, 0.0, 0.25),
    };
    let json = serde_json::to_string(&shadow).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["blur"], 4.0);
    assert_eq!(parsed["color"]["a"], 0.25);
}

#[test]
fn test_color_value_rgb() {
    let c = ColorValue::rgb(1.0, 0.0, 0.0);
    assert_eq!(c.a, 1.0);
    assert_eq!(c.r, 1.0);
}

// ══════════════════════════════════════════════════════════════════════════════
// Component registry tests
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_registry_has_10_components() {
    assert_eq!(registry().len(), 10);
}

#[test]
fn test_registry_all_component_names() {
    let names = registry().component_names();
    assert_eq!(
        names,
        vec![
            "Button",
            "Column",
            "Modal",
            "ProgressBar",
            "Row",
            "Scroll",
            "ScrollList",
            "Text",
            "TextInput",
            "Toast",
        ]
    );
}

#[test]
fn test_registry_lookup_valid() {
    let reg = registry();
    for name in &[
        "Column",
        "Row",
        "Scroll",
        "Text",
        "ProgressBar",
        "Button",
        "TextInput",
        "ScrollList",
        "Modal",
        "Toast",
    ] {
        assert!(reg.is_valid(name), "{name} should be valid");
        let def = reg.get(name).unwrap();
        assert_eq!(def.name(), *name);
    }
}

#[test]
fn test_registry_lookup_invalid_e402() {
    let reg = registry();
    assert!(!reg.is_valid("NonExistent"));
    assert!(reg.get("NonExistent").is_none()); // E402 — unknown component
    assert!(!reg.is_valid("column")); // case-sensitive
    assert!(!reg.is_valid(""));
}

#[test]
fn test_layout_components_accept_children() {
    let reg = registry();
    assert!(reg.get("Column").unwrap().accepts_children());
    assert!(reg.get("Row").unwrap().accepts_children());
    assert!(reg.get("Scroll").unwrap().accepts_children());
    assert!(reg.get("Modal").unwrap().accepts_children());
}

#[test]
fn test_leaf_components_no_children() {
    let reg = registry();
    assert!(!reg.get("Text").unwrap().accepts_children());
    assert!(!reg.get("ProgressBar").unwrap().accepts_children());
    assert!(!reg.get("Button").unwrap().accepts_children());
    assert!(!reg.get("TextInput").unwrap().accepts_children());
    assert!(!reg.get("ScrollList").unwrap().accepts_children());
    assert!(!reg.get("Toast").unwrap().accepts_children());
}

#[test]
fn test_button_required_props() {
    let reg = registry();
    let button = reg.get("Button").unwrap();
    let props = button.props();
    let required: Vec<&str> = props
        .iter()
        .filter(|p| p.requirement == PropRequirement::Required)
        .map(|p| p.name)
        .collect();
    assert!(required.contains(&"label"));
    assert!(required.contains(&"on_tap"));
}

#[test]
fn test_text_required_props() {
    let reg = registry();
    let text = reg.get("Text").unwrap();
    let props = text.props();
    let required: Vec<&str> = props
        .iter()
        .filter(|p| p.requirement == PropRequirement::Required)
        .map(|p| p.name)
        .collect();
    assert_eq!(required, vec!["value"]);
}

#[test]
fn test_text_input_props() {
    let reg = registry();
    let input = reg.get("TextInput").unwrap();
    let props = input.props();
    let names: Vec<&str> = props.iter().map(|p| p.name).collect();
    assert!(names.contains(&"value"));
    assert!(names.contains(&"on_change"));
    assert!(names.contains(&"placeholder"));
    assert!(names.contains(&"keyboard"));
}

#[test]
fn test_scroll_list_required_props() {
    let reg = registry();
    let sl = reg.get("ScrollList").unwrap();
    let required: Vec<&str> = sl
        .props()
        .iter()
        .filter(|p| p.requirement == PropRequirement::Required)
        .map(|p| p.name)
        .collect();
    assert!(required.contains(&"items"));
    assert!(required.contains(&"render"));
    assert!(required.contains(&"key"));
}

#[test]
fn test_modal_required_props() {
    let reg = registry();
    let modal = reg.get("Modal").unwrap();
    let required: Vec<&str> = modal
        .props()
        .iter()
        .filter(|p| p.requirement == PropRequirement::Required)
        .map(|p| p.name)
        .collect();
    assert!(required.contains(&"visible"));
    assert!(required.contains(&"on_dismiss"));
}

#[test]
fn test_toast_props() {
    let reg = registry();
    let toast = reg.get("Toast").unwrap();
    let required: Vec<&str> = toast
        .props()
        .iter()
        .filter(|p| p.requirement == PropRequirement::Required)
        .map(|p| p.name)
        .collect();
    assert_eq!(required, vec!["message"]);
}

// ══════════════════════════════════════════════════════════════════════════════
// Surface schema freeze test
// ══════════════════════════════════════════════════════════════════════════════

/// This test locks down the Surface JSON schema. If anyone changes SurfaceNode
/// fields, prop serialization order, or JSON structure, this test breaks
/// immediately — protecting golden references (M3) and WASM validation (M4).
#[test]
fn test_surface_schema_freeze() {
    let surface = Surface::new(
        SurfaceNode::new("Column")
            .with_prop("spacing", PropValue::Number(8.0))
            .with_child(
                SurfaceNode::new("Text")
                    .with_prop("value", PropValue::String("Hello World".into()))
                    .with_prop("size", PropValue::String("title".into())),
            )
            .with_child(
                SurfaceNode::new("Button")
                    .with_prop("label", PropValue::String("Click Me".into()))
                    .with_prop("on_tap", PropValue::action("handle_click")),
            )
            .with_child(
                SurfaceNode::new("ProgressBar").with_prop("value", PropValue::Number(0.75)),
            ),
    );

    // The exact JSON output. BTreeMap guarantees alphabetical key ordering.
    // This string is the frozen schema — if it changes, something broke.
    let expected = r#"{"root":{"type":"Column","props":{"spacing":8.0},"children":[{"type":"Text","props":{"size":"title","value":"Hello World"},"children":[]},{"type":"Button","props":{"label":"Click Me","on_tap":{"__action":"handle_click"}},"children":[]},{"type":"ProgressBar","props":{"value":0.75},"children":[]}]}}"#;

    assert_eq!(
        surface.to_json(),
        expected,
        "Surface JSON schema has changed! This breaks golden references. \
         If this change is intentional, update this test AND all golden snapshots."
    );
}

// ══════════════════════════════════════════════════════════════════════════════
// Determinism test
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_surface_determinism_100_iterations() {
    let build_surface = || {
        Surface::new(
            SurfaceNode::new("Column")
                .with_prop("spacing", PropValue::Number(16.0))
                .with_child(
                    SurfaceNode::new("Row")
                        .with_prop("align", PropValue::String("center".into()))
                        .with_child(
                            SurfaceNode::new("Text")
                                .with_prop("value", PropValue::String("Count: 42".into()))
                                .with_prop("weight", PropValue::String("bold".into())),
                        )
                        .with_child(
                            SurfaceNode::new("Button")
                                .with_prop("label", PropValue::String("+1".into()))
                                .with_prop("on_tap", PropValue::action("increment")),
                        ),
                )
                .with_child(
                    SurfaceNode::new("TextInput")
                        .with_prop("value", PropValue::String("".into()))
                        .with_prop("on_change", PropValue::lambda(1))
                        .with_prop("placeholder", PropValue::String("Type here".into())),
                )
                .with_child(
                    SurfaceNode::new("Modal")
                        .with_prop("visible", PropValue::Bool(false))
                        .with_prop("on_dismiss", PropValue::action("close_modal"))
                        .with_child(
                            SurfaceNode::new("Text")
                                .with_prop("value", PropValue::String("Modal content".into())),
                        ),
                ),
        )
    };

    let reference = build_surface().to_json();
    for i in 0..100 {
        assert_eq!(
            build_surface().to_json(),
            reference,
            "Surface JSON diverged at iteration {i}"
        );
    }
}

#[test]
fn test_registry_determinism_100_iterations() {
    let reference = registry().component_names();
    for i in 0..100 {
        assert_eq!(
            registry().component_names(),
            reference,
            "Registry order diverged at iteration {i}"
        );
    }
}
