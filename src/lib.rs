// #![deny(missing_docs)]
// #![deny(missing_debug_implementations)]
#![deny(unused_results)]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod pipeline;
mod shaders;

use egui::{pos2, vec2, Context, RawInput, Ui};
use pipeline::*;
use shaders::*;
use std::sync::Arc;
use wgpu::*;

/// All events you pass to the UI state should be
/// convertable to this type.
pub enum EventBridge {
    MouseMove { x: f32, y: f32 },
    MouseDown,
    MouseUp,
    Resize { w: f32, h: f32 },
    DpiChanged(f32),
    Ignore,
}

pub trait UiState {
    /// Place Drawing Logic Here
    fn draw(&self, ui: &mut Ui);
}

pub struct EguiRenderer<S: UiState> {
    ui_pl: Pipeline,
    raw_input: RawInput,
    ctx: Arc<Context>,
    state: S,
    start_time: std::time::Instant,
}

impl<S> EguiRenderer<S>
where
    S: UiState,
{
    /// fmt should be the same format that you render EGui to.
    pub fn new(dev: &Device, queue: &Queue, state: S, fmt: TextureFormat) -> Self {
        let mut ctx = Context::new();
        let raw_input = RawInput::default();
        let _ = ctx.begin_frame(raw_input);
        let ui_pl = Pipeline::new(dev, queue, ctx.texture(), fmt);
        Self {
            ui_pl,
            ctx,
            state,
            raw_input: RawInput::default(),
            start_time: std::time::Instant::now(),
        }
    }

    /// this should be called in a loop in immediate mode
    pub fn consume_event<'a, T>(&mut self, input: T)
    where
        T: Into<EventBridge>,
    {
        self.raw_input.time = self.start_time.elapsed().as_nanos() as f64 * 1e-9;
        match input.into() {
            EventBridge::MouseUp => self.raw_input.mouse_down = false,
            EventBridge::MouseDown => self.raw_input.mouse_down = true,
            EventBridge::MouseMove { x, y } => self.raw_input.mouse_pos = Some(pos2(x, y)),
            EventBridge::Resize { w, h } => self.raw_input.screen_size = vec2(w, h),
            EventBridge::DpiChanged(dpi) => self.raw_input.pixels_per_point = Some(dpi),
            _ => {}
        }
    }

    pub fn set_height(&mut self, h: f32) {
        self.raw_input.screen_size.y = h;
        //TODO: set texture height in buffer
    }

    pub fn set_width(&mut self, w: f32) {
        self.raw_input.screen_size.x = w;
        //TODO: set texutre width in buffer
    }

    pub fn dpi(&mut self, dpi: f32) {
        self.raw_input.pixels_per_point = Some(dpi);
    }

    /// Draws the UI using `render_pass`.
    pub fn draw_on<'a>(&'a mut self, rpass: &mut RenderPass<'a>) {
        // render the scene
        let mut ui = self.ctx.begin_frame(self.raw_input.take());
        self.state.draw(&mut ui);
        let (out, jobs) = self.ctx.end_frame();

        rpass.set_pipeline(&self.ui_pl.pl);
    }
}
