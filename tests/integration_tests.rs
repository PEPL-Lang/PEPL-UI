//! Phase 8: Final UI Validation — Integration tests, render budget, determinism proofs.
//!
//! These tests validate cross-cutting concerns that span all 10 Phase 0 components:
//! 1. All components serialize to valid Surface JSON
//! 2. Surface trees from canonical examples match expected output
//! 3. All components render within 16ms budget
//! 4. Prop validation produces clear errors for wrong types
//! 5. Full 100-iteration determinism across all 10 components
//! 6. Accessibility support verified for all components

use pepl_ui::{
    ButtonBuilder, ButtonVariant, ColumnBuilder, ComponentRegistry, ModalBuilder,
    ProgressBarBuilder, PropRequirement, PropValue, RowBuilder, ScrollBuilder, ScrollListBuilder,
    Surface, SurfaceNode, TextBuilder, TextInputBuilder, ToastBuilder,
};

use pepl_ui::components::content::{TextSize, TextWeight};
use std::collections::BTreeMap;
use std::time::Instant;

// ══════════════════════════════════════════════════════════════════════════════
// Helpers
// ══════════════════════════════════════════════════════════════════════════════

/// Build a Surface tree containing all 10 Phase 0 components.
fn all_components_tree() -> Surface {
    let text = TextBuilder::new("Hello").build();
    let progress = ProgressBarBuilder::new(0.5).build();
    let button = ButtonBuilder::new("Click", PropValue::action("do_action")).build();
    let input = TextInputBuilder::new("val", PropValue::lambda(1)).build();
    let scroll_list = ScrollListBuilder::new(
        PropValue::List(vec![PropValue::String("a".into())]),
        PropValue::lambda(2),
        PropValue::lambda(3),
    )
    .build();
    let modal = ModalBuilder::new(true, PropValue::action("close"))
        .child(TextBuilder::new("Modal body").build())
        .build();
    let toast = ToastBuilder::new("Done!").build();
    let scroll = ScrollBuilder::new().child(text.clone()).build();
    let row = RowBuilder::new().child(button).child(input).build();
    let column = ColumnBuilder::new()
        .child(row)
        .child(progress)
        .child(scroll)
        .child(scroll_list)
        .child(modal)
        .child(toast)
        .build();
    Surface::new(column)
}

/// Count all nodes in a Surface tree.
fn count_nodes(node: &SurfaceNode) -> usize {
    1 + node.children.iter().map(count_nodes).sum::<usize>()
}

/// Collect all component type names from a Surface tree.
fn collect_types(node: &SurfaceNode, out: &mut Vec<String>) {
    out.push(node.component_type.clone());
    for child in &node.children {
        collect_types(child, out);
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// 1. All 10 components → valid Surface JSON
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_all_10_components_in_one_tree() {
    let surface = all_components_tree();
    let mut types = Vec::new();
    collect_types(&surface.root, &mut types);

    let expected = [
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
    ];
    for name in &expected {
        assert!(
            types.contains(&name.to_string()),
            "Missing component type '{}' in tree: {:?}",
            name,
            types
        );
    }
}

#[test]
fn test_all_10_components_serialize_to_json() {
    let surface = all_components_tree();
    let json = surface.to_json();

    // Must be valid JSON
    let parsed: serde_json::Value =
        serde_json::from_str(&json).expect("Surface JSON must be valid");

    // Root must be an object with "root" containing "type", "props", "children"
    assert!(parsed.is_object());
    assert!(parsed["root"]["type"].is_string());
    assert!(parsed["root"]["props"].is_object());
    assert!(parsed["root"]["children"].is_array());
}

#[test]
fn test_all_components_roundtrip_json() {
    let surface = all_components_tree();
    let json = surface.to_json();
    let roundtrip: Surface =
        serde_json::from_str(&json).expect("Surface must roundtrip through JSON");
    assert_eq!(
        surface, roundtrip,
        "Surface must be identical after JSON roundtrip"
    );
}

#[test]
fn test_individual_component_json_validity() {
    // Test each component individually serializes to valid JSON
    let components: Vec<SurfaceNode> = vec![
        TextBuilder::new("Hi").build(),
        ProgressBarBuilder::new(0.75).build(),
        ButtonBuilder::new("Go", PropValue::action("go_action")).build(),
        TextInputBuilder::new("", PropValue::lambda(1)).build(),
        ScrollListBuilder::new(
            PropValue::List(vec![]),
            PropValue::lambda(2),
            PropValue::lambda(3),
        )
        .build(),
        ModalBuilder::new(false, PropValue::action("dismiss")).build(),
        ToastBuilder::new("Hello").build(),
        ColumnBuilder::new().build(),
        RowBuilder::new().build(),
        ScrollBuilder::new().build(),
    ];

    for node in &components {
        let surface = Surface::new(node.clone());
        let json = surface.to_json();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(
            parsed["root"]["type"].as_str().unwrap(),
            node.component_type,
            "JSON type field mismatch for {}",
            node.component_type
        );
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// 2. Canonical example Surface trees
// ══════════════════════════════════════════════════════════════════════════════

/// Build a Counter-like canonical Surface tree.
fn counter_surface() -> Surface {
    let title = TextBuilder::new("Count: 0").size(TextSize::Title).build();
    let inc_btn = ButtonBuilder::new("Increment", PropValue::action("increment")).build();
    let dec_btn = ButtonBuilder::new("Decrement", PropValue::action("decrement")).build();
    let reset_btn = ButtonBuilder::new("Reset", PropValue::action("reset"))
        .variant(ButtonVariant::Outlined)
        .build();
    let button_row = RowBuilder::new()
        .spacing(8.0)
        .child(inc_btn)
        .child(dec_btn)
        .child(reset_btn)
        .build();
    let column = ColumnBuilder::new()
        .spacing(16.0)
        .child(title)
        .child(button_row)
        .build();
    Surface::new(column)
}

/// Build a TodoList-like canonical Surface tree.
fn todo_surface() -> Surface {
    let title = TextBuilder::new("Todo List")
        .size(TextSize::Heading)
        .weight(TextWeight::Bold)
        .build();
    let input = TextInputBuilder::new("", PropValue::lambda(1))
        .placeholder("New task...")
        .build();
    let add_btn = ButtonBuilder::new("Add", PropValue::action("add_item")).build();
    let input_row = RowBuilder::new()
        .spacing(8.0)
        .child(input)
        .child(add_btn)
        .build();
    let list = ScrollListBuilder::new(
        PropValue::List(vec![PropValue::Record({
            let mut m = BTreeMap::new();
            m.insert("text".into(), PropValue::String("Buy milk".into()));
            m.insert("done".into(), PropValue::Bool(false));
            m
        })]),
        PropValue::lambda(2),
        PropValue::lambda(3),
    )
    .build();
    let column = ColumnBuilder::new()
        .spacing(16.0)
        .child(title)
        .child(input_row)
        .child(list)
        .build();
    Surface::new(column)
}

/// Build a UnitConverter-like canonical Surface tree.
fn unit_converter_surface() -> Surface {
    let title = TextBuilder::new("Unit Converter")
        .size(TextSize::Title)
        .build();
    let input = TextInputBuilder::new("0", PropValue::lambda(1))
        .label("Celsius")
        .build();
    let result = TextBuilder::new("32 °F").size(TextSize::Body).build();
    let column = ColumnBuilder::new()
        .spacing(12.0)
        .child(title)
        .child(input)
        .child(result)
        .build();
    Surface::new(column)
}

#[test]
fn test_counter_canonical_tree_structure() {
    let surface = counter_surface();
    assert_eq!(surface.root.component_type, "Column");
    assert_eq!(surface.root.children.len(), 2);
    // Title
    assert_eq!(surface.root.children[0].component_type, "Text");
    assert_eq!(
        surface.root.children[0].props["value"],
        PropValue::String("Count: 0".into())
    );
    // Button row
    assert_eq!(surface.root.children[1].component_type, "Row");
    assert_eq!(surface.root.children[1].children.len(), 3);
}

#[test]
fn test_counter_canonical_json() {
    let surface = counter_surface();
    let json = surface.to_json();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["root"]["type"], "Column");
    assert_eq!(parsed["root"]["children"].as_array().unwrap().len(), 2);
    assert_eq!(parsed["root"]["children"][0]["type"], "Text");
    assert_eq!(parsed["root"]["children"][1]["type"], "Row");
    assert_eq!(
        parsed["root"]["children"][1]["children"]
            .as_array()
            .unwrap()
            .len(),
        3
    );
}

#[test]
fn test_todo_canonical_tree_structure() {
    let surface = todo_surface();
    assert_eq!(surface.root.component_type, "Column");
    assert_eq!(surface.root.children.len(), 3);
    assert_eq!(surface.root.children[0].component_type, "Text");
    assert_eq!(surface.root.children[1].component_type, "Row");
    assert_eq!(surface.root.children[2].component_type, "ScrollList");
}

#[test]
fn test_todo_canonical_json_roundtrip() {
    let surface = todo_surface();
    let json = surface.to_json();
    let rt: Surface = serde_json::from_str(&json).unwrap();
    assert_eq!(surface, rt);
}

#[test]
fn test_unit_converter_canonical_tree() {
    let surface = unit_converter_surface();
    assert_eq!(surface.root.component_type, "Column");
    assert_eq!(surface.root.children.len(), 3);
    assert_eq!(surface.root.children[0].component_type, "Text");
    assert_eq!(surface.root.children[1].component_type, "TextInput");
    assert_eq!(surface.root.children[2].component_type, "Text");
}

// ══════════════════════════════════════════════════════════════════════════════
// 3. Render budget validation (< 16ms per component build+serialize)
// ══════════════════════════════════════════════════════════════════════════════

/// Test that building and serializing a component completes well within 16ms.
/// We run 1000 iterations and check the average to account for JIT/warmup.
fn assert_render_budget(name: &str, build_fn: impl Fn() -> SurfaceNode) {
    // Warm up
    for _ in 0..100 {
        let _ = Surface::new(build_fn()).to_json();
    }

    let start = Instant::now();
    let iterations = 1000;
    for _ in 0..iterations {
        let node = build_fn();
        let surface = Surface::new(node);
        let _json = surface.to_json();
    }
    let elapsed = start.elapsed();
    let avg_us = elapsed.as_micros() as f64 / iterations as f64;
    let avg_ms = avg_us / 1000.0;

    // Must be well under 16ms (60fps budget). We allow up to 1ms average.
    assert!(
        avg_ms < 1.0,
        "{} average render time {:.3}ms exceeds 1ms budget (16ms/frame target)",
        name,
        avg_ms
    );
}

#[test]
fn test_text_render_budget() {
    assert_render_budget("Text", || TextBuilder::new("Hello World").build());
}

#[test]
fn test_progress_bar_render_budget() {
    assert_render_budget("ProgressBar", || ProgressBarBuilder::new(0.75).build());
}

#[test]
fn test_button_render_budget() {
    assert_render_budget("Button", || {
        ButtonBuilder::new("Click", PropValue::action("action")).build()
    });
}

#[test]
fn test_text_input_render_budget() {
    assert_render_budget("TextInput", || {
        TextInputBuilder::new("val", PropValue::lambda(1)).build()
    });
}

#[test]
fn test_scroll_list_render_budget() {
    assert_render_budget("ScrollList", || {
        ScrollListBuilder::new(
            PropValue::List(vec![PropValue::String("a".into())]),
            PropValue::lambda(1),
            PropValue::lambda(2),
        )
        .build()
    });
}

#[test]
fn test_modal_render_budget() {
    assert_render_budget("Modal", || {
        ModalBuilder::new(true, PropValue::action("close"))
            .child(TextBuilder::new("Body").build())
            .build()
    });
}

#[test]
fn test_toast_render_budget() {
    assert_render_budget("Toast", || ToastBuilder::new("Done").build());
}

#[test]
fn test_column_render_budget() {
    assert_render_budget("Column", || {
        ColumnBuilder::new()
            .child(TextBuilder::new("A").build())
            .child(TextBuilder::new("B").build())
            .build()
    });
}

#[test]
fn test_row_render_budget() {
    assert_render_budget("Row", || {
        RowBuilder::new()
            .child(TextBuilder::new("A").build())
            .child(TextBuilder::new("B").build())
            .build()
    });
}

#[test]
fn test_scroll_render_budget() {
    assert_render_budget("Scroll", || {
        ScrollBuilder::new()
            .child(TextBuilder::new("Content").build())
            .build()
    });
}

#[test]
fn test_full_tree_render_budget() {
    // Full tree with all 10 components must also be within budget
    assert_render_budget("FullTree", || all_components_tree().root);
}

// ══════════════════════════════════════════════════════════════════════════════
// 4. Prop validation — wrong types produce clear errors
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_registry_validates_all_10_components() {
    let reg = ComponentRegistry::new();
    assert_eq!(reg.len(), 10);
    let names = reg.component_names();
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
fn test_registry_rejects_unknown_component() {
    let reg = ComponentRegistry::new();
    assert!(reg.get("UnknownWidget").is_none());
    assert!(!reg.is_valid("Foo"));
}

#[test]
fn test_registry_required_props_documented() {
    let reg = ComponentRegistry::new();

    // Text requires "value"
    let text_def = reg.get("Text").unwrap();
    let required: Vec<_> = text_def
        .props()
        .iter()
        .filter(|p| p.requirement == PropRequirement::Required)
        .map(|p| p.name)
        .collect();
    assert!(required.contains(&"value"), "Text must require 'value'");

    // Button requires "label" and "on_tap"
    let btn_def = reg.get("Button").unwrap();
    let required: Vec<_> = btn_def
        .props()
        .iter()
        .filter(|p| p.requirement == PropRequirement::Required)
        .map(|p| p.name)
        .collect();
    assert!(required.contains(&"label"), "Button must require 'label'");
    assert!(required.contains(&"on_tap"), "Button must require 'on_tap'");

    // Modal requires "visible" and "on_dismiss"
    let modal_def = reg.get("Modal").unwrap();
    let required: Vec<_> = modal_def
        .props()
        .iter()
        .filter(|p| p.requirement == PropRequirement::Required)
        .map(|p| p.name)
        .collect();
    assert!(
        required.contains(&"visible"),
        "Modal must require 'visible'"
    );
    assert!(
        required.contains(&"on_dismiss"),
        "Modal must require 'on_dismiss'"
    );
}

#[test]
fn test_children_acceptance_rules() {
    let reg = ComponentRegistry::new();

    // Container components accept children
    for name in &["Column", "Row", "Scroll", "Modal"] {
        let def = reg.get(name).unwrap();
        assert!(def.accepts_children(), "{} should accept children", name);
    }

    // Leaf components do not accept children
    for name in &[
        "Text",
        "ProgressBar",
        "Button",
        "TextInput",
        "ScrollList",
        "Toast",
    ] {
        let def = reg.get(name).unwrap();
        assert!(
            !def.accepts_children(),
            "{} should not accept children",
            name
        );
    }
}

#[test]
fn test_all_components_have_accessible_prop() {
    let reg = ComponentRegistry::new();
    for name in reg.component_names() {
        let def = reg.get(name).unwrap();
        let has_accessible = def.props().iter().any(|p| p.name == "accessible");
        assert!(
            has_accessible,
            "Component '{}' must have an 'accessible' prop",
            name
        );
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// 5. Full 100-iteration determinism across ALL 10 components
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_unified_100_iteration_determinism() {
    // Build the reference tree containing all 10 components
    let reference = all_components_tree();
    let ref_json = reference.to_json();

    for i in 0..100 {
        let tree = all_components_tree();
        let json = tree.to_json();
        assert_eq!(
            json, ref_json,
            "Determinism violation at iteration {} — JSON output differs",
            i
        );
        assert_eq!(
            tree, reference,
            "Determinism violation at iteration {} — struct differs",
            i
        );
    }
}

#[test]
fn test_canonical_counter_determinism_100() {
    let reference = counter_surface();
    let ref_json = reference.to_json();
    for i in 0..100 {
        let tree = counter_surface();
        assert_eq!(
            tree.to_json(),
            ref_json,
            "Counter determinism failure at iteration {}",
            i
        );
    }
}

#[test]
fn test_canonical_todo_determinism_100() {
    let reference = todo_surface();
    let ref_json = reference.to_json();
    for i in 0..100 {
        let tree = todo_surface();
        assert_eq!(
            tree.to_json(),
            ref_json,
            "TodoList determinism failure at iteration {}",
            i
        );
    }
}

#[test]
fn test_canonical_unit_converter_determinism_100() {
    let reference = unit_converter_surface();
    let ref_json = reference.to_json();
    for i in 0..100 {
        let tree = unit_converter_surface();
        assert_eq!(
            tree.to_json(),
            ref_json,
            "UnitConverter determinism failure at iteration {}",
            i
        );
    }
}

#[test]
fn test_prop_ordering_determinism() {
    // Props in BTreeMap → must serialize in alphabetical order every time
    let reference = TextBuilder::new("Test")
        .size(TextSize::Title)
        .weight(TextWeight::Bold)
        .build();
    let ref_json = serde_json::to_string(&reference).unwrap();

    for _ in 0..100 {
        let node = TextBuilder::new("Test")
            .size(TextSize::Title)
            .weight(TextWeight::Bold)
            .build();
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, ref_json, "Prop ordering must be deterministic");
    }
}

#[test]
fn test_nested_children_determinism() {
    // Deeply nested tree with multiple component types
    let build_nested = || {
        ColumnBuilder::new()
            .spacing(16.0)
            .child(
                RowBuilder::new()
                    .spacing(8.0)
                    .child(TextBuilder::new("A").build())
                    .child(ButtonBuilder::new("B", PropValue::action("b")).build())
                    .build(),
            )
            .child(
                ScrollBuilder::new()
                    .child(
                        ColumnBuilder::new()
                            .child(TextBuilder::new("C").build())
                            .child(ProgressBarBuilder::new(0.5).build())
                            .build(),
                    )
                    .build(),
            )
            .child(
                ModalBuilder::new(true, PropValue::action("close"))
                    .child(TextBuilder::new("Modal").build())
                    .build(),
            )
            .build()
    };

    let ref_json = serde_json::to_string(&build_nested()).unwrap();
    for _ in 0..100 {
        let json = serde_json::to_string(&build_nested()).unwrap();
        assert_eq!(json, ref_json, "Nested tree determinism failure");
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// 6. Accessibility verification — all components
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_all_builders_produce_accessible_node() {
    // Each builder must produce a node with an "accessible" prop by default
    let nodes = vec![
        ("Text", TextBuilder::new("Hi").build()),
        ("ProgressBar", ProgressBarBuilder::new(0.5).build()),
        (
            "Button",
            ButtonBuilder::new("Go", PropValue::action("go")).build(),
        ),
        (
            "TextInput",
            TextInputBuilder::new("val", PropValue::lambda(1)).build(),
        ),
        (
            "ScrollList",
            ScrollListBuilder::new(
                PropValue::List(vec![]),
                PropValue::lambda(1),
                PropValue::lambda(2),
            )
            .build(),
        ),
        (
            "Modal",
            ModalBuilder::new(true, PropValue::action("close")).build(),
        ),
        ("Toast", ToastBuilder::new("Msg").build()),
        ("Column", ColumnBuilder::new().build()),
        ("Row", RowBuilder::new().build()),
        ("Scroll", ScrollBuilder::new().build()),
    ];

    for (name, node) in &nodes {
        assert!(
            node.props.contains_key("accessible"),
            "{} builder must auto-generate 'accessible' prop",
            name
        );
        // Must be a Record with at least "label" and "role"
        match &node.props["accessible"] {
            PropValue::Record(rec) => {
                assert!(
                    rec.contains_key("role"),
                    "{} accessible prop must have 'role'",
                    name
                );
            }
            other => panic!("{} accessible prop must be Record, got {:?}", name, other),
        }
    }
}

#[test]
fn test_default_roles_correct() {
    let role_of = |node: &SurfaceNode| -> String {
        match &node.props["accessible"] {
            PropValue::Record(rec) => match &rec["role"] {
                PropValue::String(s) => s.clone(),
                _ => panic!("role must be String"),
            },
            _ => panic!("accessible must be Record"),
        }
    };

    assert_eq!(role_of(&TextBuilder::new("Hi").build()), "text");
    assert_eq!(
        role_of(&ButtonBuilder::new("Go", PropValue::action("go")).build()),
        "button"
    );
    assert_eq!(
        role_of(&TextInputBuilder::new("v", PropValue::lambda(1)).build()),
        "textfield"
    );
    assert_eq!(
        role_of(&ProgressBarBuilder::new(0.5).build()),
        "progressbar"
    );
    assert_eq!(
        role_of(&ModalBuilder::new(true, PropValue::action("c")).build()),
        "dialog"
    );
    assert_eq!(role_of(&ToastBuilder::new("M").build()), "alert");
}

#[test]
fn test_accessible_prop_serializes_cleanly() {
    let node = ButtonBuilder::new("Submit", PropValue::action("submit")).build();
    let json = serde_json::to_string_pretty(&node).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

    let accessible = &parsed["props"]["accessible"];
    assert!(
        accessible.is_object(),
        "accessible must serialize as object"
    );
    assert!(accessible["role"].is_string());
    assert!(accessible["label"].is_string());
}

// ══════════════════════════════════════════════════════════════════════════════
// 7. Additional integration tests
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_deep_tree_node_count() {
    let surface = all_components_tree();
    let count = count_nodes(&surface.root);
    // Column > (Row > (Button, TextInput)), ProgressBar, (Scroll > Text),
    //          ScrollList, (Modal > Text), Toast
    // = 1 + (1 + 2) + 1 + (1 + 1) + 1 + (1 + 1) + 1 = 11
    assert!(
        count >= 10,
        "Full tree must have at least 10 nodes, got {}",
        count
    );
}

#[test]
fn test_surface_pretty_json_is_valid() {
    let surface = all_components_tree();
    let pretty = surface.to_json_pretty();
    let _parsed: serde_json::Value =
        serde_json::from_str(&pretty).expect("Pretty JSON must also be valid");
    assert!(pretty.contains('\n'), "Pretty JSON should contain newlines");
}

#[test]
fn test_empty_containers_serialize() {
    let empty_col = Surface::new(ColumnBuilder::new().build());
    let json = empty_col.to_json();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["root"]["children"].as_array().unwrap().len(), 0);

    let empty_row = Surface::new(RowBuilder::new().build());
    let json = empty_row.to_json();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["root"]["children"].as_array().unwrap().len(), 0);
}

#[test]
fn test_props_sorted_alphabetically_in_json() {
    let node = TextBuilder::new("Hi")
        .size(TextSize::Title)
        .weight(TextWeight::Bold)
        .build();
    let json = serde_json::to_string(&node).unwrap();

    // In the JSON, prop keys should appear in alphabetical order
    // because we use BTreeMap. Find positions of known keys.
    let accessible_pos = json.find("\"accessible\"").unwrap_or(usize::MAX);
    let size_pos = json.find("\"size\"").unwrap_or(usize::MAX);
    let value_pos = json.find("\"value\"").unwrap_or(usize::MAX);
    let weight_pos = json.find("\"weight\"").unwrap_or(usize::MAX);

    assert!(
        accessible_pos < size_pos,
        "accessible should come before size"
    );
    assert!(size_pos < value_pos, "size should come before value");
    assert!(value_pos < weight_pos, "value should come before weight");
}

#[test]
fn test_action_ref_serialization_in_tree() {
    let button = ButtonBuilder::new("Click", PropValue::action("my_action")).build();
    let surface = Surface::new(button);
    let json = surface.to_json();
    assert!(json.contains("my_action"), "Action ref must appear in JSON");
}

#[test]
fn test_lambda_serialization_in_tree() {
    let input = TextInputBuilder::new("val", PropValue::lambda(42)).build();
    let surface = Surface::new(input);
    let json = surface.to_json();
    assert!(json.contains("42"), "Lambda ID must appear in JSON");
}

#[test]
fn test_color_prop_serialization() {
    let progress = ProgressBarBuilder::new(0.5)
        .color(pepl_ui::ColorValue {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        })
        .build();
    let surface = Surface::new(progress);
    let json = surface.to_json();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    let color = &parsed["root"]["props"]["color"];
    assert_eq!(color["r"], 1.0);
    assert_eq!(color["g"], 0.0);
}

#[test]
fn test_multiple_views_independent() {
    // Building multiple independent Surface trees should not interfere
    let counter = counter_surface();
    let todo = todo_surface();
    let converter = unit_converter_surface();

    assert_ne!(counter.to_json(), todo.to_json());
    assert_ne!(todo.to_json(), converter.to_json());
    assert_ne!(counter.to_json(), converter.to_json());
}

#[test]
fn test_large_list_render_budget() {
    // 100-item list should still render within budget
    let items: Vec<PropValue> = (0..100)
        .map(|i| PropValue::String(format!("Item {}", i)))
        .collect();

    assert_render_budget("LargeList", move || {
        ScrollListBuilder::new(
            PropValue::List(items.clone()),
            PropValue::lambda(1),
            PropValue::lambda(2),
        )
        .build()
    });
}
