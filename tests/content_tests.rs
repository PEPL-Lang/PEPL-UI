//! Tests for the `Text` and `ProgressBar` content components.
//!
//! Test categories:
//! 1. Default/minimal construction
//! 2. Each optional prop individually
//! 3. All props combined
//! 4. JSON roundtrip
//! 5. Validation (valid, invalid types, missing required, unknown props, no children)
//! 6. Edge cases (overwrite, clamping)
//! 7. 100-iteration determinism

use pepl_ui::{
    validate_content_node, ColorValue, ProgressBarBuilder, PropValue, Surface, SurfaceNode,
    TextAlign, TextBuilder, TextOverflow, TextSize, TextWeight,
};

// ═══════════════════════════════════════════════════════════════════════════════
// Text — Construction
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_text_minimal() {
    let node = TextBuilder::new("hello").build();
    assert_eq!(node.component_type, "Text");
    assert_eq!(
        node.props.get("value"),
        Some(&PropValue::String("hello".into()))
    );
    assert!(node.children.is_empty());
}

#[test]
fn test_text_with_size() {
    let node = TextBuilder::new("hi").size(TextSize::Title).build();
    assert_eq!(
        node.props.get("size"),
        Some(&PropValue::String("title".into()))
    );
}

#[test]
fn test_text_with_weight() {
    let node = TextBuilder::new("hi").weight(TextWeight::Bold).build();
    assert_eq!(
        node.props.get("weight"),
        Some(&PropValue::String("bold".into()))
    );
}

#[test]
fn test_text_with_color() {
    let color = ColorValue::new(1.0, 0.0, 0.0, 1.0);
    let node = TextBuilder::new("hi").color(color).build();
    assert_eq!(
        node.props.get("color"),
        Some(&PropValue::color(1.0, 0.0, 0.0, 1.0))
    );
}

#[test]
fn test_text_with_align() {
    let node = TextBuilder::new("hi").align(TextAlign::Center).build();
    assert_eq!(
        node.props.get("align"),
        Some(&PropValue::String("center".into()))
    );
}

#[test]
fn test_text_with_max_lines() {
    let node = TextBuilder::new("hi").max_lines(3.0).build();
    assert_eq!(node.props.get("max_lines"), Some(&PropValue::Number(3.0)));
}

#[test]
fn test_text_with_overflow() {
    let node = TextBuilder::new("hi")
        .overflow(TextOverflow::Ellipsis)
        .build();
    assert_eq!(
        node.props.get("overflow"),
        Some(&PropValue::String("ellipsis".into()))
    );
}

#[test]
fn test_text_all_props() {
    let node = TextBuilder::new("Hello!")
        .size(TextSize::Heading)
        .weight(TextWeight::Medium)
        .color(ColorValue::rgb(0.2, 0.4, 0.6))
        .align(TextAlign::End)
        .max_lines(2.0)
        .overflow(TextOverflow::Wrap)
        .build();
    assert_eq!(node.component_type, "Text");
    assert_eq!(node.props.len(), 8);
    assert_eq!(
        node.props.get("value"),
        Some(&PropValue::String("Hello!".into()))
    );
    assert_eq!(
        node.props.get("size"),
        Some(&PropValue::String("heading".into()))
    );
    assert_eq!(
        node.props.get("weight"),
        Some(&PropValue::String("medium".into()))
    );
    assert_eq!(
        node.props.get("align"),
        Some(&PropValue::String("end".into()))
    );
    assert_eq!(node.props.get("max_lines"), Some(&PropValue::Number(2.0)));
    assert_eq!(
        node.props.get("overflow"),
        Some(&PropValue::String("wrap".into()))
    );
}

#[test]
fn test_text_empty_string() {
    let node = TextBuilder::new("").build();
    assert_eq!(node.props.get("value"), Some(&PropValue::String("".into())));
}

#[test]
fn test_text_unicode() {
    let node = TextBuilder::new("Hello 🌍 世界").build();
    assert_eq!(
        node.props.get("value"),
        Some(&PropValue::String("Hello 🌍 世界".into()))
    );
}

// ── Text sizes cover all variants ─────────────────────────────────────────────

#[test]
fn test_text_size_small() {
    let node = TextBuilder::new("x").size(TextSize::Small).build();
    assert_eq!(
        node.props.get("size"),
        Some(&PropValue::String("small".into()))
    );
}

#[test]
fn test_text_size_body() {
    let node = TextBuilder::new("x").size(TextSize::Body).build();
    assert_eq!(
        node.props.get("size"),
        Some(&PropValue::String("body".into()))
    );
}

#[test]
fn test_text_size_display() {
    let node = TextBuilder::new("x").size(TextSize::Display).build();
    assert_eq!(
        node.props.get("size"),
        Some(&PropValue::String("display".into()))
    );
}

// ── Text weights cover all variants ───────────────────────────────────────────

#[test]
fn test_text_weight_normal() {
    let node = TextBuilder::new("x").weight(TextWeight::Normal).build();
    assert_eq!(
        node.props.get("weight"),
        Some(&PropValue::String("normal".into()))
    );
}

// ── Text align covers all variants ────────────────────────────────────────────

#[test]
fn test_text_align_start() {
    let node = TextBuilder::new("x").align(TextAlign::Start).build();
    assert_eq!(
        node.props.get("align"),
        Some(&PropValue::String("start".into()))
    );
}

// ── Text overflow covers all variants ─────────────────────────────────────────

#[test]
fn test_text_overflow_clip() {
    let node = TextBuilder::new("x").overflow(TextOverflow::Clip).build();
    assert_eq!(
        node.props.get("overflow"),
        Some(&PropValue::String("clip".into()))
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// Text — JSON Roundtrip
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_text_json_roundtrip() {
    let node = TextBuilder::new("Test")
        .size(TextSize::Title)
        .weight(TextWeight::Bold)
        .build();
    let surface = Surface { root: node };
    let json = surface.to_json();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["root"]["type"], "Text");
    assert_eq!(parsed["root"]["props"]["value"], "Test");
    assert_eq!(parsed["root"]["props"]["size"], "title");
    assert_eq!(parsed["root"]["props"]["weight"], "bold");
}

#[test]
fn test_text_json_deterministic() {
    let build = || {
        TextBuilder::new("det")
            .size(TextSize::Body)
            .color(ColorValue::rgb(1.0, 0.0, 0.0))
            .build()
    };
    let json1 = Surface { root: build() }.to_json();
    let json2 = Surface { root: build() }.to_json();
    assert_eq!(json1, json2);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Text — Validation
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_text_valid() {
    let node = TextBuilder::new("valid").build();
    assert!(validate_content_node(&node).is_empty());
}

#[test]
fn test_text_valid_all_props() {
    let node = TextBuilder::new("hi")
        .size(TextSize::Small)
        .weight(TextWeight::Normal)
        .color(ColorValue::rgb(0.0, 0.0, 0.0))
        .align(TextAlign::Center)
        .max_lines(1.0)
        .overflow(TextOverflow::Clip)
        .build();
    assert!(validate_content_node(&node).is_empty());
}

#[test]
fn test_text_missing_value() {
    let node = SurfaceNode::new("Text");
    let errors = validate_content_node(&node);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("required prop missing"));
}

#[test]
fn test_text_wrong_value_type() {
    let mut node = SurfaceNode::new("Text");
    node.set_prop("value", PropValue::Number(42.0));
    let errors = validate_content_node(&node);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("expected string"));
}

#[test]
fn test_text_invalid_size() {
    let mut node = TextBuilder::new("hi").build();
    node.set_prop("size", PropValue::String("huge".into()));
    let errors = validate_content_node(&node);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("size"));
}

#[test]
fn test_text_invalid_weight() {
    let mut node = TextBuilder::new("hi").build();
    node.set_prop("weight", PropValue::Number(700.0));
    let errors = validate_content_node(&node);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("weight"));
}

#[test]
fn test_text_invalid_color() {
    let mut node = TextBuilder::new("hi").build();
    node.set_prop("color", PropValue::String("red".into()));
    let errors = validate_content_node(&node);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("color"));
}

#[test]
fn test_text_invalid_align() {
    let mut node = TextBuilder::new("hi").build();
    node.set_prop("align", PropValue::String("left".into()));
    let errors = validate_content_node(&node);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("align"));
}

#[test]
fn test_text_invalid_max_lines() {
    let mut node = TextBuilder::new("hi").build();
    node.set_prop("max_lines", PropValue::String("3".into()));
    let errors = validate_content_node(&node);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("max_lines"));
}

#[test]
fn test_text_invalid_overflow() {
    let mut node = TextBuilder::new("hi").build();
    node.set_prop("overflow", PropValue::String("scroll".into()));
    let errors = validate_content_node(&node);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("overflow"));
}

#[test]
fn test_text_unknown_prop() {
    let mut node = TextBuilder::new("hi").build();
    node.set_prop("font_family", PropValue::String("Arial".into()));
    let errors = validate_content_node(&node);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("unknown prop"));
}

#[test]
fn test_text_no_children_allowed() {
    let mut node = TextBuilder::new("hi").build();
    node.children.push(SurfaceNode::new("Text"));
    let errors = validate_content_node(&node);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("does not accept children"));
}

#[test]
fn test_text_multiple_errors() {
    let mut node = SurfaceNode::new("Text");
    node.set_prop("size", PropValue::Number(12.0));
    node.set_prop("unknown", PropValue::Bool(true));
    // missing value + invalid size + unknown prop = 3 errors
    let errors = validate_content_node(&node);
    assert_eq!(errors.len(), 3);
}

// ═══════════════════════════════════════════════════════════════════════════════
// ProgressBar — Construction
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_progress_bar_minimal() {
    let node = ProgressBarBuilder::new(0.5).build();
    assert_eq!(node.component_type, "ProgressBar");
    assert_eq!(node.props.get("value"), Some(&PropValue::Number(0.5)));
    assert!(node.children.is_empty());
}

#[test]
fn test_progress_bar_zero() {
    let node = ProgressBarBuilder::new(0.0).build();
    assert_eq!(node.props.get("value"), Some(&PropValue::Number(0.0)));
}

#[test]
fn test_progress_bar_one() {
    let node = ProgressBarBuilder::new(1.0).build();
    assert_eq!(node.props.get("value"), Some(&PropValue::Number(1.0)));
}

#[test]
fn test_progress_bar_clamp_above() {
    let node = ProgressBarBuilder::new(1.5).build();
    assert_eq!(node.props.get("value"), Some(&PropValue::Number(1.0)));
}

#[test]
fn test_progress_bar_clamp_below() {
    let node = ProgressBarBuilder::new(-0.5).build();
    assert_eq!(node.props.get("value"), Some(&PropValue::Number(0.0)));
}

#[test]
fn test_progress_bar_with_color() {
    let node = ProgressBarBuilder::new(0.5)
        .color(ColorValue::rgb(0.0, 1.0, 0.0))
        .build();
    assert_eq!(
        node.props.get("color"),
        Some(&PropValue::color(0.0, 1.0, 0.0, 1.0))
    );
}

#[test]
fn test_progress_bar_with_background() {
    let node = ProgressBarBuilder::new(0.5)
        .background(ColorValue::new(0.9, 0.9, 0.9, 0.5))
        .build();
    assert_eq!(
        node.props.get("background"),
        Some(&PropValue::color(0.9, 0.9, 0.9, 0.5))
    );
}

#[test]
fn test_progress_bar_with_height() {
    let node = ProgressBarBuilder::new(0.5).height(8.0).build();
    assert_eq!(node.props.get("height"), Some(&PropValue::Number(8.0)));
}

#[test]
fn test_progress_bar_all_props() {
    let node = ProgressBarBuilder::new(0.75)
        .color(ColorValue::rgb(0.0, 0.5, 1.0))
        .background(ColorValue::rgb(0.9, 0.9, 0.9))
        .height(12.0)
        .build();
    assert_eq!(node.component_type, "ProgressBar");
    assert_eq!(node.props.len(), 5);
    assert_eq!(node.props.get("value"), Some(&PropValue::Number(0.75)));
    assert_eq!(node.props.get("height"), Some(&PropValue::Number(12.0)));
}

// ═══════════════════════════════════════════════════════════════════════════════
// ProgressBar — JSON Roundtrip
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_progress_bar_json_roundtrip() {
    let node = ProgressBarBuilder::new(0.3).height(4.0).build();
    let surface = Surface { root: node };
    let json = surface.to_json();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["root"]["type"], "ProgressBar");
    assert_eq!(parsed["root"]["props"]["value"], 0.3);
    assert_eq!(parsed["root"]["props"]["height"], 4.0);
}

#[test]
fn test_progress_bar_json_deterministic() {
    let build = || {
        ProgressBarBuilder::new(0.42)
            .color(ColorValue::rgb(0.0, 1.0, 0.0))
            .build()
    };
    let json1 = Surface { root: build() }.to_json();
    let json2 = Surface { root: build() }.to_json();
    assert_eq!(json1, json2);
}

// ═══════════════════════════════════════════════════════════════════════════════
// ProgressBar — Validation
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_progress_bar_valid() {
    let node = ProgressBarBuilder::new(0.5).build();
    assert!(validate_content_node(&node).is_empty());
}

#[test]
fn test_progress_bar_valid_all_props() {
    let node = ProgressBarBuilder::new(0.5)
        .color(ColorValue::rgb(1.0, 0.0, 0.0))
        .background(ColorValue::rgb(0.9, 0.9, 0.9))
        .height(6.0)
        .build();
    assert!(validate_content_node(&node).is_empty());
}

#[test]
fn test_progress_bar_missing_value() {
    let node = SurfaceNode::new("ProgressBar");
    let errors = validate_content_node(&node);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("required prop missing"));
}

#[test]
fn test_progress_bar_wrong_value_type() {
    let mut node = SurfaceNode::new("ProgressBar");
    node.set_prop("value", PropValue::String("50%".into()));
    let errors = validate_content_node(&node);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("expected number"));
}

#[test]
fn test_progress_bar_invalid_color() {
    let mut node = ProgressBarBuilder::new(0.5).build();
    node.set_prop("color", PropValue::String("green".into()));
    let errors = validate_content_node(&node);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("color"));
}

#[test]
fn test_progress_bar_invalid_background() {
    let mut node = ProgressBarBuilder::new(0.5).build();
    node.set_prop("background", PropValue::Number(0.0));
    let errors = validate_content_node(&node);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("background"));
}

#[test]
fn test_progress_bar_invalid_height() {
    let mut node = ProgressBarBuilder::new(0.5).build();
    node.set_prop("height", PropValue::String("10px".into()));
    let errors = validate_content_node(&node);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("height"));
}

#[test]
fn test_progress_bar_unknown_prop() {
    let mut node = ProgressBarBuilder::new(0.5).build();
    node.set_prop("width", PropValue::Number(100.0));
    let errors = validate_content_node(&node);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("unknown prop"));
}

#[test]
fn test_progress_bar_no_children_allowed() {
    let mut node = ProgressBarBuilder::new(0.5).build();
    node.children.push(SurfaceNode::new("Text"));
    let errors = validate_content_node(&node);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("does not accept children"));
}

#[test]
fn test_progress_bar_multiple_errors() {
    let mut node = SurfaceNode::new("ProgressBar");
    node.set_prop("color", PropValue::String("red".into()));
    node.set_prop("unknown", PropValue::Bool(true));
    // missing value + invalid color + unknown prop = 3 errors
    let errors = validate_content_node(&node);
    assert_eq!(errors.len(), 3);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Unknown Component
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_unknown_component() {
    let node = SurfaceNode::new("FooBar");
    let errors = validate_content_node(&node);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("Unknown content component"));
}

// ═══════════════════════════════════════════════════════════════════════════════
// Determinism — 100 iterations
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_text_determinism_100() {
    let build = || {
        TextBuilder::new("Determinism")
            .size(TextSize::Title)
            .weight(TextWeight::Bold)
            .color(ColorValue::rgb(0.1, 0.2, 0.3))
            .align(TextAlign::Center)
            .max_lines(5.0)
            .overflow(TextOverflow::Ellipsis)
            .build()
    };
    let baseline = Surface { root: build() }.to_json();
    for _ in 0..100 {
        assert_eq!(Surface { root: build() }.to_json(), baseline);
    }
}

#[test]
fn test_progress_bar_determinism_100() {
    let build = || {
        ProgressBarBuilder::new(0.42)
            .color(ColorValue::rgb(0.0, 0.8, 0.2))
            .background(ColorValue::new(0.9, 0.9, 0.9, 0.5))
            .height(10.0)
            .build()
    };
    let baseline = Surface { root: build() }.to_json();
    for _ in 0..100 {
        assert_eq!(Surface { root: build() }.to_json(), baseline);
    }
}
