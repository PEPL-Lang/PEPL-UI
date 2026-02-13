//! Integration tests for `pepl-ui` Phase 2: Layout components (Column, Row, Scroll).

use pepl_ui::{
    Alignment, ColumnBuilder, Edges, PropValue, RowBuilder, ScrollBuilder, ScrollDirection,
    Surface, SurfaceNode, validate_layout_node,
};
use std::collections::BTreeMap;

// ── Helpers ───────────────────────────────────────────────────────────────────

fn text_node(value: &str) -> SurfaceNode {
    SurfaceNode::new("Text").with_prop("value", PropValue::String(value.into()))
}

fn button_node(label: &str) -> SurfaceNode {
    SurfaceNode::new("Button").with_prop("label", PropValue::String(label.into()))
}

// ══════════════════════════════════════════════════════════════════════════════
// ColumnBuilder tests
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_column_empty() {
    let node = ColumnBuilder::new().build();
    assert_eq!(node.component_type, "Column");
    assert_eq!(node.props.len(), 1); // accessible is auto-added
    assert!(node.children.is_empty());
}

#[test]
fn test_column_default() {
    let node = ColumnBuilder::default().build();
    assert_eq!(node.component_type, "Column");
    assert_eq!(node.props.len(), 1); // accessible is auto-added
}

#[test]
fn test_column_with_spacing() {
    let node = ColumnBuilder::new().spacing(8.0).build();
    assert_eq!(node.props["spacing"], PropValue::Number(8.0));
}

#[test]
fn test_column_spacing_zero() {
    let node = ColumnBuilder::new().spacing(0.0).build();
    assert_eq!(node.props["spacing"], PropValue::Number(0.0));
}

#[test]
fn test_column_with_align_start() {
    let node = ColumnBuilder::new().align(Alignment::Start).build();
    assert_eq!(
        node.props["align"],
        PropValue::String("start".into())
    );
}

#[test]
fn test_column_with_align_center() {
    let node = ColumnBuilder::new().align(Alignment::Center).build();
    assert_eq!(
        node.props["align"],
        PropValue::String("center".into())
    );
}

#[test]
fn test_column_with_align_end() {
    let node = ColumnBuilder::new().align(Alignment::End).build();
    assert_eq!(
        node.props["align"],
        PropValue::String("end".into())
    );
}

#[test]
fn test_column_with_align_stretch() {
    let node = ColumnBuilder::new().align(Alignment::Stretch).build();
    assert_eq!(
        node.props["align"],
        PropValue::String("stretch".into())
    );
}

#[test]
fn test_column_with_align_space_between() {
    let node = ColumnBuilder::new()
        .align(Alignment::SpaceBetween)
        .build();
    assert_eq!(
        node.props["align"],
        PropValue::String("space_between".into())
    );
}

#[test]
fn test_column_with_align_space_around() {
    let node = ColumnBuilder::new()
        .align(Alignment::SpaceAround)
        .build();
    assert_eq!(
        node.props["align"],
        PropValue::String("space_around".into())
    );
}

#[test]
fn test_column_with_padding_uniform() {
    let node = ColumnBuilder::new()
        .padding(Edges::Uniform(16.0))
        .build();
    assert_eq!(node.props["padding"], PropValue::Number(16.0));
}

#[test]
fn test_column_with_padding_sides() {
    let node = ColumnBuilder::new()
        .padding(Edges::sides(10.0, 20.0, 30.0, 40.0))
        .build();
    if let PropValue::Record(ref map) = node.props["padding"] {
        assert_eq!(map["top"], PropValue::Number(10.0));
        assert_eq!(map["bottom"], PropValue::Number(20.0));
        assert_eq!(map["start"], PropValue::Number(30.0));
        assert_eq!(map["end"], PropValue::Number(40.0));
    } else {
        panic!("Expected Record for Sides padding, got {:?}", node.props["padding"]);
    }
}

#[test]
fn test_column_with_all_props() {
    let node = ColumnBuilder::new()
        .spacing(12.0)
        .align(Alignment::Center)
        .padding(Edges::Uniform(8.0))
        .build();
    assert_eq!(node.props.len(), 4); // spacing + align + padding + accessible
    assert_eq!(node.props["align"], PropValue::String("center".into()));
    assert_eq!(node.props["padding"], PropValue::Number(8.0));
}

#[test]
fn test_column_with_single_child() {
    let node = ColumnBuilder::new()
        .child(text_node("Hello"))
        .build();
    assert_eq!(node.children.len(), 1);
    assert_eq!(node.children[0].component_type, "Text");
}

#[test]
fn test_column_with_multiple_children() {
    let node = ColumnBuilder::new()
        .child(text_node("Title"))
        .child(button_node("OK"))
        .child(text_node("Footer"))
        .build();
    assert_eq!(node.children.len(), 3);
    assert_eq!(node.children[0].component_type, "Text");
    assert_eq!(node.children[1].component_type, "Button");
    assert_eq!(node.children[2].component_type, "Text");
}

#[test]
fn test_column_with_children_vec() {
    let kids = vec![text_node("A"), text_node("B")];
    let node = ColumnBuilder::new().children(kids).build();
    assert_eq!(node.children.len(), 2);
}

#[test]
fn test_column_full_example() {
    let node = ColumnBuilder::new()
        .spacing(8.0)
        .align(Alignment::Center)
        .padding(Edges::Uniform(16.0))
        .child(text_node("Title"))
        .child(button_node("Submit"))
        .build();
    assert_eq!(node.component_type, "Column");
    assert_eq!(node.props.len(), 4); // spacing + align + padding + accessible
    assert_eq!(node.children.len(), 2);
}

// ══════════════════════════════════════════════════════════════════════════════
// RowBuilder tests
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_row_empty() {
    let node = RowBuilder::new().build();
    assert_eq!(node.component_type, "Row");
    assert_eq!(node.props.len(), 1); // accessible is auto-added
    assert!(node.children.is_empty());
}

#[test]
fn test_row_default() {
    let node = RowBuilder::default().build();
    assert_eq!(node.component_type, "Row");
}

#[test]
fn test_row_with_spacing() {
    let node = RowBuilder::new().spacing(4.0).build();
    assert_eq!(node.props["spacing"], PropValue::Number(4.0));
}

#[test]
fn test_row_with_align_center() {
    let node = RowBuilder::new().align(Alignment::Center).build();
    assert_eq!(node.props["align"], PropValue::String("center".into()));
}

#[test]
fn test_row_with_padding_uniform() {
    let node = RowBuilder::new()
        .padding(Edges::Uniform(24.0))
        .build();
    assert_eq!(node.props["padding"], PropValue::Number(24.0));
}

#[test]
fn test_row_with_padding_sides() {
    let node = RowBuilder::new()
        .padding(Edges::sides(1.0, 2.0, 3.0, 4.0))
        .build();
    if let PropValue::Record(ref map) = node.props["padding"] {
        assert_eq!(map.len(), 4);
    } else {
        panic!("Expected Record");
    }
}

#[test]
fn test_row_with_all_props() {
    let node = RowBuilder::new()
        .spacing(16.0)
        .align(Alignment::SpaceBetween)
        .padding(Edges::Uniform(4.0))
        .build();
    assert_eq!(node.props.len(), 4); // spacing + align + padding + accessible
}

#[test]
fn test_row_with_children() {
    let node = RowBuilder::new()
        .child(button_node("Cancel"))
        .child(button_node("OK"))
        .build();
    assert_eq!(node.children.len(), 2);
    assert_eq!(node.children[0].props["label"], PropValue::String("Cancel".into()));
    assert_eq!(node.children[1].props["label"], PropValue::String("OK".into()));
}

#[test]
fn test_row_with_children_vec() {
    let kids = vec![text_node("A"), text_node("B"), text_node("C")];
    let node = RowBuilder::new().children(kids).build();
    assert_eq!(node.children.len(), 3);
}

// ══════════════════════════════════════════════════════════════════════════════
// ScrollBuilder tests
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_scroll_default_direction() {
    let node = ScrollBuilder::new().build();
    assert_eq!(node.component_type, "Scroll");
    assert_eq!(
        node.props["direction"],
        PropValue::String("vertical".into())
    );
}

#[test]
fn test_scroll_default_impl() {
    let node = ScrollBuilder::default().build();
    assert_eq!(
        node.props["direction"],
        PropValue::String("vertical".into())
    );
}

#[test]
fn test_scroll_vertical() {
    let node = ScrollBuilder::new()
        .direction(ScrollDirection::Vertical)
        .build();
    assert_eq!(
        node.props["direction"],
        PropValue::String("vertical".into())
    );
}

#[test]
fn test_scroll_horizontal() {
    let node = ScrollBuilder::new()
        .direction(ScrollDirection::Horizontal)
        .build();
    assert_eq!(
        node.props["direction"],
        PropValue::String("horizontal".into())
    );
}

#[test]
fn test_scroll_both() {
    let node = ScrollBuilder::new()
        .direction(ScrollDirection::Both)
        .build();
    assert_eq!(
        node.props["direction"],
        PropValue::String("both".into())
    );
}

#[test]
fn test_scroll_with_children() {
    let node = ScrollBuilder::new()
        .child(text_node("Item 1"))
        .child(text_node("Item 2"))
        .child(text_node("Item 3"))
        .build();
    assert_eq!(node.children.len(), 3);
}

#[test]
fn test_scroll_with_children_vec() {
    let kids = vec![text_node("A"), text_node("B")];
    let node = ScrollBuilder::new().children(kids).build();
    assert_eq!(node.children.len(), 2);
}

#[test]
fn test_scroll_wrapping_layout() {
    let col = ColumnBuilder::new()
        .spacing(8.0)
        .child(text_node("Hello"))
        .build();
    let node = ScrollBuilder::new()
        .direction(ScrollDirection::Vertical)
        .child(col)
        .build();
    assert_eq!(node.children.len(), 1);
    assert_eq!(node.children[0].component_type, "Column");
}

// ══════════════════════════════════════════════════════════════════════════════
// ScrollDirection tests
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_scroll_direction_as_str() {
    assert_eq!(ScrollDirection::Vertical.as_str(), "vertical");
    assert_eq!(ScrollDirection::Horizontal.as_str(), "horizontal");
    assert_eq!(ScrollDirection::Both.as_str(), "both");
}

#[test]
fn test_scroll_direction_default() {
    let d = ScrollDirection::default();
    assert_eq!(d, ScrollDirection::Vertical);
}

#[test]
fn test_scroll_direction_clone() {
    let d = ScrollDirection::Both;
    let d2 = d.clone();
    assert_eq!(d, d2);
}

#[test]
fn test_scroll_direction_debug() {
    let d = ScrollDirection::Horizontal;
    let s = format!("{:?}", d);
    assert!(s.contains("Horizontal"));
}

// ══════════════════════════════════════════════════════════════════════════════
// Nested layout tests
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_column_in_row() {
    let col = ColumnBuilder::new()
        .spacing(4.0)
        .child(text_node("A"))
        .child(text_node("B"))
        .build();
    let row = RowBuilder::new()
        .child(col)
        .child(button_node("Action"))
        .build();
    assert_eq!(row.children.len(), 2);
    assert_eq!(row.children[0].component_type, "Column");
    assert_eq!(row.children[0].children.len(), 2);
    assert_eq!(row.children[1].component_type, "Button");
}

#[test]
fn test_row_in_column() {
    let row = RowBuilder::new()
        .spacing(8.0)
        .child(button_node("Cancel"))
        .child(button_node("OK"))
        .build();
    let col = ColumnBuilder::new()
        .spacing(16.0)
        .child(text_node("Title"))
        .child(row)
        .build();
    assert_eq!(col.children.len(), 2);
    assert_eq!(col.children[1].component_type, "Row");
    assert_eq!(col.children[1].children.len(), 2);
}

#[test]
fn test_deeply_nested_layout() {
    let inner = ColumnBuilder::new()
        .child(text_node("Deep"))
        .build();
    let middle = RowBuilder::new().child(inner).build();
    let outer = ColumnBuilder::new().child(middle).build();
    let scroll = ScrollBuilder::new().child(outer).build();

    assert_eq!(scroll.component_type, "Scroll");
    assert_eq!(scroll.children[0].component_type, "Column");
    assert_eq!(scroll.children[0].children[0].component_type, "Row");
    assert_eq!(
        scroll.children[0].children[0].children[0].component_type,
        "Column"
    );
    assert_eq!(
        scroll.children[0].children[0].children[0].children[0].component_type,
        "Text"
    );
}

#[test]
fn test_scroll_with_column_and_row() {
    let node = ScrollBuilder::new()
        .direction(ScrollDirection::Both)
        .child(
            ColumnBuilder::new()
                .spacing(8.0)
                .child(
                    RowBuilder::new()
                        .spacing(4.0)
                        .child(text_node("A"))
                        .child(text_node("B"))
                        .build(),
                )
                .child(text_node("C"))
                .build(),
        )
        .build();
    assert_eq!(node.children[0].children.len(), 2);
    assert_eq!(node.children[0].children[0].component_type, "Row");
}

#[test]
fn test_multiple_columns_in_row() {
    let col_a = ColumnBuilder::new()
        .child(text_node("A1"))
        .child(text_node("A2"))
        .build();
    let col_b = ColumnBuilder::new()
        .child(text_node("B1"))
        .child(text_node("B2"))
        .build();
    let row = RowBuilder::new()
        .spacing(12.0)
        .child(col_a)
        .child(col_b)
        .build();
    assert_eq!(row.children.len(), 2);
    assert_eq!(row.children[0].children.len(), 2);
    assert_eq!(row.children[1].children.len(), 2);
}

// ══════════════════════════════════════════════════════════════════════════════
// All alignment values tests
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_all_alignments_column() {
    let alignments = [
        (Alignment::Start, "start"),
        (Alignment::Center, "center"),
        (Alignment::End, "end"),
        (Alignment::Stretch, "stretch"),
        (Alignment::SpaceBetween, "space_between"),
        (Alignment::SpaceAround, "space_around"),
    ];
    for (align, expected) in alignments {
        let node = ColumnBuilder::new().align(align).build();
        assert_eq!(
            node.props["align"],
            PropValue::String(expected.into()),
            "Failed for {:?}",
            align
        );
    }
}

#[test]
fn test_all_alignments_row() {
    let alignments = [
        (Alignment::Start, "start"),
        (Alignment::Center, "center"),
        (Alignment::End, "end"),
        (Alignment::Stretch, "stretch"),
        (Alignment::SpaceBetween, "space_between"),
        (Alignment::SpaceAround, "space_around"),
    ];
    for (align, expected) in alignments {
        let node = RowBuilder::new().align(align).build();
        assert_eq!(
            node.props["align"],
            PropValue::String(expected.into()),
            "Failed for {:?}",
            align
        );
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Edges coercion tests
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_edges_uniform_coercion_number() {
    // Uniform(n) should produce PropValue::Number(n), matching number literal coercion.
    let node = ColumnBuilder::new()
        .padding(Edges::Uniform(0.0))
        .build();
    assert_eq!(node.props["padding"], PropValue::Number(0.0));
}

#[test]
fn test_edges_uniform_coercion_large() {
    let node = RowBuilder::new()
        .padding(Edges::Uniform(100.0))
        .build();
    assert_eq!(node.props["padding"], PropValue::Number(100.0));
}

#[test]
fn test_edges_sides_is_record() {
    let node = ColumnBuilder::new()
        .padding(Edges::sides(5.0, 10.0, 15.0, 20.0))
        .build();
    assert!(matches!(node.props["padding"], PropValue::Record(_)));
}

#[test]
fn test_edges_sides_record_keys() {
    let node = RowBuilder::new()
        .padding(Edges::sides(1.0, 2.0, 3.0, 4.0))
        .build();
    if let PropValue::Record(ref map) = node.props["padding"] {
        assert!(map.contains_key("top"));
        assert!(map.contains_key("bottom"));
        assert!(map.contains_key("start"));
        assert!(map.contains_key("end"));
    } else {
        panic!("Expected Record");
    }
}

#[test]
fn test_edges_sides_equal_values() {
    let node = ColumnBuilder::new()
        .padding(Edges::sides(8.0, 8.0, 8.0, 8.0))
        .build();
    // Even with all sides equal, Sides variant produces a Record (not coerced to Number).
    assert!(matches!(node.props["padding"], PropValue::Record(_)));
}

// ══════════════════════════════════════════════════════════════════════════════
// Validation tests
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_validate_column_valid() {
    let node = ColumnBuilder::new()
        .spacing(8.0)
        .align(Alignment::Center)
        .padding(Edges::Uniform(16.0))
        .build();
    let errors = validate_layout_node(&node);
    assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
}

#[test]
fn test_validate_row_valid() {
    let node = RowBuilder::new()
        .spacing(4.0)
        .align(Alignment::Start)
        .build();
    let errors = validate_layout_node(&node);
    assert!(errors.is_empty());
}

#[test]
fn test_validate_scroll_valid() {
    let node = ScrollBuilder::new()
        .direction(ScrollDirection::Horizontal)
        .build();
    let errors = validate_layout_node(&node);
    assert!(errors.is_empty());
}

#[test]
fn test_validate_column_invalid_spacing_type() {
    let mut node = SurfaceNode::new("Column");
    node.set_prop("spacing", PropValue::String("big".into()));
    let errors = validate_layout_node(&node);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("spacing"));
    assert!(errors[0].contains("number"));
}

#[test]
fn test_validate_column_invalid_align_value() {
    let mut node = SurfaceNode::new("Column");
    node.set_prop("align", PropValue::String("middle".into()));
    let errors = validate_layout_node(&node);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("invalid alignment"));
}

#[test]
fn test_validate_column_invalid_align_type() {
    let mut node = SurfaceNode::new("Column");
    node.set_prop("align", PropValue::Number(42.0));
    let errors = validate_layout_node(&node);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("string"));
}

#[test]
fn test_validate_column_invalid_padding_type() {
    let mut node = SurfaceNode::new("Column");
    node.set_prop("padding", PropValue::Bool(true));
    let errors = validate_layout_node(&node);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("padding"));
}

#[test]
fn test_validate_column_unknown_prop() {
    let mut node = SurfaceNode::new("Column");
    node.set_prop("color", PropValue::String("red".into()));
    let errors = validate_layout_node(&node);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("unknown prop"));
}

#[test]
fn test_validate_row_invalid_spacing() {
    let mut node = SurfaceNode::new("Row");
    node.set_prop("spacing", PropValue::Bool(false));
    let errors = validate_layout_node(&node);
    assert_eq!(errors.len(), 1);
}

#[test]
fn test_validate_scroll_invalid_direction_value() {
    let mut node = SurfaceNode::new("Scroll");
    node.set_prop("direction", PropValue::String("diagonal".into()));
    let errors = validate_layout_node(&node);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("invalid direction"));
}

#[test]
fn test_validate_scroll_invalid_direction_type() {
    let mut node = SurfaceNode::new("Scroll");
    node.set_prop("direction", PropValue::Number(1.0));
    let errors = validate_layout_node(&node);
    assert_eq!(errors.len(), 1);
}

#[test]
fn test_validate_scroll_unknown_prop() {
    let mut node = SurfaceNode::new("Scroll");
    node.set_prop("direction", PropValue::String("vertical".into()));
    node.set_prop("speed", PropValue::Number(2.0));
    let errors = validate_layout_node(&node);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("unknown prop"));
}

#[test]
fn test_validate_multiple_errors() {
    let mut node = SurfaceNode::new("Column");
    node.set_prop("spacing", PropValue::Bool(true));
    node.set_prop("align", PropValue::Number(0.0));
    node.set_prop("unknown", PropValue::Nil);
    let errors = validate_layout_node(&node);
    assert_eq!(errors.len(), 3);
}

#[test]
fn test_validate_non_layout_skipped() {
    let mut node = SurfaceNode::new("Text");
    node.set_prop("anything", PropValue::Nil);
    let errors = validate_layout_node(&node);
    assert!(errors.is_empty(), "Non-layout components should be skipped");
}

#[test]
fn test_validate_empty_column() {
    let node = SurfaceNode::new("Column");
    let errors = validate_layout_node(&node);
    assert!(errors.is_empty());
}

#[test]
fn test_validate_column_padding_record_valid() {
    let mut node = SurfaceNode::new("Row");
    let mut map = BTreeMap::new();
    map.insert("top".into(), PropValue::Number(10.0));
    map.insert("bottom".into(), PropValue::Number(10.0));
    node.set_prop("padding", PropValue::Record(map));
    let errors = validate_layout_node(&node);
    assert!(errors.is_empty()); // Record is a valid type for padding
}

// ══════════════════════════════════════════════════════════════════════════════
// JSON serialization tests for layout trees
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_column_json_roundtrip() {
    let node = ColumnBuilder::new()
        .spacing(8.0)
        .align(Alignment::Center)
        .child(text_node("Hello"))
        .build();
    let surface = Surface::new(node);
    let json = surface.to_json();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["root"]["type"], "Column");
    assert_eq!(parsed["root"]["props"]["spacing"], 8.0);
    assert_eq!(parsed["root"]["props"]["align"], "center");
    assert_eq!(parsed["root"]["children"].as_array().unwrap().len(), 1);
}

#[test]
fn test_row_json_roundtrip() {
    let node = RowBuilder::new()
        .spacing(4.0)
        .child(button_node("A"))
        .child(button_node("B"))
        .build();
    let surface = Surface::new(node);
    let json = surface.to_json();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["root"]["type"], "Row");
    assert_eq!(parsed["root"]["props"]["spacing"], 4.0);
    assert_eq!(parsed["root"]["children"].as_array().unwrap().len(), 2);
}

#[test]
fn test_scroll_json_roundtrip() {
    let node = ScrollBuilder::new()
        .direction(ScrollDirection::Both)
        .child(text_node("Content"))
        .build();
    let surface = Surface::new(node);
    let json = surface.to_json();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["root"]["type"], "Scroll");
    assert_eq!(parsed["root"]["props"]["direction"], "both");
}

#[test]
fn test_nested_layout_json() {
    let tree = ColumnBuilder::new()
        .spacing(16.0)
        .padding(Edges::Uniform(8.0))
        .child(text_node("Header"))
        .child(
            RowBuilder::new()
                .spacing(4.0)
                .child(button_node("Cancel"))
                .child(button_node("OK"))
                .build(),
        )
        .build();
    let surface = Surface::new(tree);
    let json = surface.to_json();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["root"]["type"], "Column");
    assert_eq!(parsed["root"]["children"][1]["type"], "Row");
    assert_eq!(
        parsed["root"]["children"][1]["children"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
}

#[test]
fn test_padding_sides_json() {
    let node = ColumnBuilder::new()
        .padding(Edges::sides(10.0, 20.0, 30.0, 40.0))
        .build();
    let surface = Surface::new(node);
    let json = surface.to_json();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    let padding = &parsed["root"]["props"]["padding"];
    assert_eq!(padding["top"], 10.0);
    assert_eq!(padding["bottom"], 20.0);
    assert_eq!(padding["start"], 30.0);
    assert_eq!(padding["end"], 40.0);
}

#[test]
fn test_surface_json_deserialize_roundtrip() {
    let tree = ScrollBuilder::new()
        .direction(ScrollDirection::Horizontal)
        .child(
            RowBuilder::new()
                .spacing(8.0)
                .child(text_node("A"))
                .child(text_node("B"))
                .build(),
        )
        .build();
    let surface = Surface::new(tree);
    let json = surface.to_json();
    let deserialized: Surface = serde_json::from_str(&json).unwrap();
    assert_eq!(surface, deserialized);
}

// ══════════════════════════════════════════════════════════════════════════════
// Determinism tests
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_column_json_deterministic() {
    let build = || {
        ColumnBuilder::new()
            .spacing(8.0)
            .align(Alignment::Center)
            .padding(Edges::Uniform(16.0))
            .child(text_node("Title"))
            .child(
                RowBuilder::new()
                    .spacing(4.0)
                    .child(button_node("Cancel"))
                    .child(button_node("OK"))
                    .build(),
            )
            .build()
    };

    let first = Surface::new(build()).to_json();
    for _ in 0..100 {
        let current = Surface::new(build()).to_json();
        assert_eq!(first, current, "JSON output must be deterministic");
    }
}

#[test]
fn test_scroll_json_deterministic() {
    let build = || {
        ScrollBuilder::new()
            .direction(ScrollDirection::Both)
            .child(
                ColumnBuilder::new()
                    .spacing(12.0)
                    .align(Alignment::SpaceBetween)
                    .child(text_node("A"))
                    .child(text_node("B"))
                    .child(text_node("C"))
                    .build(),
            )
            .build()
    };

    let first = Surface::new(build()).to_json();
    for _ in 0..100 {
        let current = Surface::new(build()).to_json();
        assert_eq!(first, current, "JSON output must be deterministic");
    }
}

#[test]
fn test_complex_layout_deterministic() {
    let build = || {
        ColumnBuilder::new()
            .spacing(16.0)
            .align(Alignment::Start)
            .padding(Edges::sides(10.0, 20.0, 30.0, 40.0))
            .child(text_node("Header"))
            .child(
                ScrollBuilder::new()
                    .direction(ScrollDirection::Vertical)
                    .child(
                        ColumnBuilder::new()
                            .spacing(8.0)
                            .child(
                                RowBuilder::new()
                                    .spacing(4.0)
                                    .align(Alignment::Center)
                                    .child(text_node("Left"))
                                    .child(text_node("Right"))
                                    .build(),
                            )
                            .child(button_node("Submit"))
                            .build(),
                    )
                    .build(),
            )
            .build()
    };

    let first = Surface::new(build()).to_json();
    for _ in 0..100 {
        assert_eq!(first, Surface::new(build()).to_json());
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Edge case tests
// ══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_column_overwrite_spacing() {
    // Last call wins
    let node = ColumnBuilder::new()
        .spacing(4.0)
        .spacing(8.0)
        .build();
    assert_eq!(node.props["spacing"], PropValue::Number(8.0));
}

#[test]
fn test_row_overwrite_align() {
    let node = RowBuilder::new()
        .align(Alignment::Start)
        .align(Alignment::End)
        .build();
    assert_eq!(node.props["align"], PropValue::String("end".into()));
}

#[test]
fn test_column_negative_spacing() {
    // Builder doesn't validate — validation is separate.
    let node = ColumnBuilder::new().spacing(-1.0).build();
    assert_eq!(node.props["spacing"], PropValue::Number(-1.0));
}

#[test]
fn test_column_fractional_spacing() {
    let node = ColumnBuilder::new().spacing(0.5).build();
    assert_eq!(node.props["spacing"], PropValue::Number(0.5));
}

#[test]
fn test_scroll_empty() {
    let node = ScrollBuilder::new().build();
    assert!(node.children.is_empty());
    // Scroll always has direction prop + accessible
    assert_eq!(node.props.len(), 2);
}

#[test]
fn test_column_no_children_json() {
    let surface = Surface::new(ColumnBuilder::new().build());
    let json = surface.to_json();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["root"]["children"], serde_json::json!([]));
    // accessible prop is auto-added by builder
    assert!(parsed["root"]["props"].is_object());
}

#[test]
fn test_row_large_child_count() {
    let mut builder = RowBuilder::new();
    for i in 0..50 {
        builder = builder.child(text_node(&format!("Item {i}")));
    }
    let node = builder.build();
    assert_eq!(node.children.len(), 50);
}

#[test]
fn test_column_prop_ordering_deterministic() {
    // BTreeMap guarantees alphabetical key ordering.
    let node = ColumnBuilder::new()
        .spacing(8.0)
        .padding(Edges::Uniform(16.0))
        .align(Alignment::Center)
        .build();
    let keys: Vec<&String> = node.props.keys().collect();
    assert_eq!(keys, vec!["accessible", "align", "padding", "spacing"]);
}
