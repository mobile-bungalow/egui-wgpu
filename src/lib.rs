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
