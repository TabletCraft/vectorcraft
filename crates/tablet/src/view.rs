//! Tablet canvas view state.

use vectorcraft_geom::{Affine, Point};

/// Platform-independent canvas navigation state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TabletView {
    pub center: Point,
    pub zoom: f64,
    /// Clockwise view rotation in radians.
    pub rotation: f64,
}

impl Default for TabletView {
    fn default() -> Self {
        Self { center: Point::ZERO, zoom: 1.0, rotation: 0.0 }
    }
}

impl TabletView {
    /// Document-to-pixel transform for a framebuffer of `width` by `height` pixels.
    pub fn render_transform(self, width: u32, height: u32) -> Affine {
        Affine::translate((f64::from(width) / 2.0, f64::from(height) / 2.0))
            * Affine::rotate(self.rotation)
            * Affine::scale(self.zoom)
            * Affine::translate(-self.center.to_vec2())
    }
}
