use serde::{Deserialize, Serialize};

/// Dimension type for width, height, etc.
///
/// Number literal coercion: `width: 100` → `Px(100.0)`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum Dimension {
    /// Fixed pixel value.
    Px(f64),
    /// Automatic sizing based on content.
    Auto,
    /// Fill available space.
    Fill,
    /// Percentage of parent (0.0–100.0).
    Percent(f64),
}

impl Dimension {
    /// Coerce a number to `Px`.
    pub fn from_number(n: f64) -> Self {
        Dimension::Px(n)
    }
}

/// Edge insets (padding, margin, etc.).
///
/// Number literal coercion: `padding: 16` → `Uniform(16.0)`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Edges {
    /// All four sides equal.
    Uniform(f64),
    /// Individual sides.
    Sides {
        top: f64,
        bottom: f64,
        start: f64,
        end: f64,
    },
}

impl Edges {
    /// Coerce a number to `Uniform`.
    pub fn from_number(n: f64) -> Self {
        Edges::Uniform(n)
    }

    /// Create explicit sides.
    pub fn sides(top: f64, bottom: f64, start: f64, end: f64) -> Self {
        Edges::Sides {
            top,
            bottom,
            start,
            end,
        }
    }
}

/// Alignment for layout components (Column, Row).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Alignment {
    Start,
    Center,
    End,
    Stretch,
    SpaceBetween,
    SpaceAround,
}

/// Border style definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BorderStyle {
    /// Border width in pixels.
    pub width: f64,
    /// Border color as RGBA.
    pub color: ColorValue,
    /// Border line style (default: "solid").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

/// Shadow style definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShadowStyle {
    /// Horizontal offset in pixels.
    pub offset_x: f64,
    /// Vertical offset in pixels.
    pub offset_y: f64,
    /// Blur radius in pixels.
    pub blur: f64,
    /// Shadow color as RGBA.
    pub color: ColorValue,
}

/// RGBA color value (each component 0.0–1.0).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColorValue {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl ColorValue {
    /// Create a new color.
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
    }

    /// Opaque color (alpha = 1.0).
    pub fn rgb(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b, a: 1.0 }
    }
}
