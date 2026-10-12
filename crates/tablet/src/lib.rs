//! Native tablet frontend boundary for VectorCraft.
//!
//! This crate is intentionally thin. Platform frontends (iPadOS and Android)
//! should translate native input and view state here, while document editing,
//! tools, history, and rendering remain owned by the existing VectorCraft crates.
//!
//! The first proof of concept is deliberately small: establish this crate as the
//! stable home for the tablet-facing API before adding platform bindings.
#![forbid(unsafe_code)]

pub mod input;
pub mod render;
pub mod session;
pub mod view;

pub use input::TabletPointer;
pub use render::TabletFrame;
pub use session::TabletSession;
pub use view::TabletView;
