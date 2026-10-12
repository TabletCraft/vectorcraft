//! Tablet framebuffer types.

/// A rendered premultiplied RGBA8 frame ready for upload by a native frontend.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TabletFrame {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}

impl From<vectorcraft_render::Rendered> for TabletFrame {
    fn from(frame: vectorcraft_render::Rendered) -> Self {
        Self { width: frame.width, height: frame.height, pixels: frame.pixels }
    }
}
