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

mod prop_value;
mod registry;
mod surface;
mod types;

pub use prop_value::PropValue;
pub use registry::{ComponentDef, ComponentRegistry, PropDef, PropRequirement};
pub use surface::{Surface, SurfaceNode};
pub use types::{Alignment, BorderStyle, ColorValue, Dimension, Edges, ShadowStyle};
