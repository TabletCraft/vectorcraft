//! Tablet-facing application session.

use vectorcraft_engine::Session;
use vectorcraft_render::Renderer;

use crate::view::TabletView;

/// Owns the VectorCraft state needed by a native tablet frontend.
///
/// Platform bindings should talk to this facade instead of reaching directly
/// into engine, tool, or renderer internals. The proof-of-concept will grow this
/// type with document creation, pointer dispatch, commands, and rendering.
pub struct TabletSession {
    pub(crate) session: Session,
    pub(crate) renderer: Renderer,
    pub view: TabletView,
}

impl TabletSession {
    pub fn from_session(session: Session) -> Self {
        Self { session, renderer: Renderer::new(), view: TabletView::default() }
    }

    pub fn session(&self) -> &Session {
        &self.session
    }

    pub fn session_mut(&mut self) -> &mut Session {
        &mut self.session
    }

    pub fn renderer(&mut self) -> &mut Renderer {
        &mut self.renderer
    }
}
