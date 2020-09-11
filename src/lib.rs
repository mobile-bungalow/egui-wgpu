// #![deny(missing_docs)]
// #![deny(missing_debug_implementations)]
#![deny(unused_results)]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod pipeline;
mod shaders;

use egui::{
    math::clamp,
    paint::tessellator::{PaintJob, PaintJobs},
    pos2, vec2, Context, RawInput, Texture, Ui,
};
use pipeline::*;
use shaders::*;
use std::sync::Arc;
use wgpu::*;

/// Vertex struct
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
struct V {
    pub p: [f32; 2], // position
    pub u: [u16; 2], // texture coordinates
    pub c: [u8; 4],  // color in SRGB8 format
}

/// All events you pass to the UI state should be
/// convertable to this type.
pub enum EventBridge {
    MouseMove { x: f32, y: f32 },
    MouseDown,
    MouseUp,
    Resize { w: f32, h: f32 },
    DpiChanged(f32),
}

pub trait UiState {
    /// Place Drawing Logic Here
    fn draw(&self, ui: &mut Ui);
}

pub struct EguiRenderer {
    ui_pl: Pipeline,
    raw_input: RawInput,
    ctx: Arc<Context>,
    state: Box<dyn UiState>,
    start_time: std::time::Instant,
}

impl EguiRenderer {
    /// fmt should be the same format that you render EGui to.
    pub fn new(dev: &Device, state: Box<dyn UiState>, fmt: TextureFormat) -> Self {
        Self {
            ui_pl: Pipeline::new(dev, fmt),
            raw_input: RawInput::default(),
            ctx: Context::new(),
            state,
            start_time: std::time::Instant::now(),
        }
    }

    /// this should be called in a loop in immediate mode
    pub fn consume_event<T: Into<EventBridge>>(&mut self, input: T) {
        self.raw_input.time = self.start_time.elapsed().as_nanos() as f64 * 1e-9;
        match input.into() {
            EventBridge::MouseUp => self.raw_input.mouse_down = false,
            EventBridge::MouseDown => self.raw_input.mouse_down = true,
            EventBridge::MouseMove { x, y } => self.raw_input.mouse_pos = Some(pos2(x, y)),
            EventBridge::Resize { w, h } => self.raw_input.screen_size = vec2(w, h),
            EventBridge::DpiChanged(dpi) => self.raw_input.pixels_per_point = Some(dpi),
        }
    }

    pub fn set_height(&mut self, h: f32) {
        self.raw_input.screen_size.y = h;
    }

    pub fn set_width(&mut self, w: f32) {
        self.raw_input.screen_size.x = w;
    }

    pub fn dpi(&mut self, dpi: f32) {
        self.raw_input.pixels_per_point = Some(dpi);
    }

    /// Draws the UI using `render_pass`.
    pub fn draw_on<'a>(&'a mut self, rpass: &'a mut RenderPass<'a>) {
        // render the scene
        let mut ui = self.ctx.begin_frame(self.raw_input.take());
        self.state.draw(&mut ui);
        let (out, jobs) = self.ctx.end_frame();

        rpass.set_pipeline(self.ui_pl.inner());
    }
}
