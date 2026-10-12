//! Platform-neutral tablet input types.

/// One native pointer sample before it is translated into VectorCraft tool input.
///
/// Coordinates are expressed in frontend logical points. Pressure is normalized
/// to `0.0..=1.0`. Richer stylus data such as tilt and azimuth will be added only
/// after the native proof of concept establishes what the platform bindings need.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TabletPointer {
    pub x: f64,
    pub y: f64,
    pub pressure: f32,
}

impl TabletPointer {
    pub fn new(x: f64, y: f64, pressure: f32) -> Self {
        Self { x, y, pressure: pressure.clamp(0.0, 1.0) }
    }
}
