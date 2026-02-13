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

pub mod accessibility;
pub mod components;
mod prop_value;
mod registry;
mod surface;
mod types;

pub use components::content::{
    validate_content_node, ProgressBarBuilder, TextAlign, TextBuilder, TextOverflow, TextSize,
    TextWeight,
};
pub use components::feedback::{validate_feedback_node, ModalBuilder, ToastBuilder, ToastType};
pub use components::interactive::{
    validate_interactive_node, ButtonBuilder, ButtonVariant, KeyboardType, TextInputBuilder,
};
pub use components::layout::{
    validate_layout_node, ColumnBuilder, RowBuilder, ScrollBuilder, ScrollDirection,
};
pub use components::list::{validate_list_node, ScrollListBuilder};
pub use prop_value::PropValue;
pub use registry::{ComponentDef, ComponentRegistry, PropDef, PropRequirement};
pub use surface::{Surface, SurfaceNode};
pub use types::{Alignment, BorderStyle, ColorValue, Dimension, Edges, ShadowStyle};

// Accessibility
pub use accessibility::{
    auto_accessible, default_role, ensure_accessible, validate_accessible_prop, AccessibilityInfo,
    LiveRegion, SemanticRole,
};
