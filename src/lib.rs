//! PEPL UI Component Model
//!
//! 10 Phase 0 components: platform-abstract, accessibility-first, deterministic rendering.
//! Components produce [`Surface`] trees — the host's View Layer renders them.
//!
//! # Architecture
//!
//! ```text
//! PEPL view function → evaluator builds SurfaceNode tree → Surface → JSON → host renders
//! ```
//!
//! # Phase 0 Components
//!
//! | Category | Components |
//! |----------|-----------|
//! | Layout | Column, Row, Scroll |
//! | Content | Text, ProgressBar |
//! | Interactive | Button, TextInput |
//! | List & Data | ScrollList |
//! | Feedback | Modal, Toast |

pub mod components;
mod prop_value;
mod registry;
mod surface;
mod types;

pub use components::content::{
    ProgressBarBuilder, TextAlign, TextBuilder, TextOverflow, TextSize, TextWeight,
    validate_content_node,
};
pub use components::interactive::{
    ButtonBuilder, ButtonVariant, KeyboardType, TextInputBuilder, validate_interactive_node,
};
pub use components::layout::{
    ColumnBuilder, RowBuilder, ScrollBuilder, ScrollDirection, validate_layout_node,
};
pub use prop_value::PropValue;
pub use registry::{ComponentDef, ComponentRegistry, PropDef, PropRequirement};
pub use surface::{Surface, SurfaceNode};
pub use types::{Alignment, BorderStyle, ColorValue, Dimension, Edges, ShadowStyle};
