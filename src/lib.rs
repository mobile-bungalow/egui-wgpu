// #![deny(missing_docs)]
// #![deny(missing_debug_implementations)]
#![deny(unused_results)]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod pipeline;
mod shaders;

use egui::*;
use pipeline::*;
use shaders::*;
use wgpu::*;

/// Vertex struct
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
struct V {
    pub p: [f32; 2], // position
    pub u: [u16; 2], // texture coordinates
    pub c: [u8; 4],  // color in SRGB8 format
}

pub struct EguiRenderer {
    ui_pl: Pipeline,
}

impl EguiRenderer {
    pub fn new(dev: &Device, fmt: TextureFormat) -> Self {
        Self {
            ui_pl: Pipeline::new(dev, fmt),
        }
    }

    /// Draws the UI using `render_pass`.
    pub fn draw_on<'a>(&'a mut self, rpass: &'a mut RenderPass<'a>) {
        rpass.set_pipeline(self.ui_pl.inner());
    }
}
