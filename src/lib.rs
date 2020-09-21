mod pipeline;
mod shaders;

use bytemuck::{cast_slice, Pod, Zeroable};
use egui::{paint, pos2, vec2, Context, RawInput, Ui};
use pipeline::*;
use shaders::*;
use std::mem::size_of;
use std::sync::Arc;
use wgpu::*;

#[derive(Copy, Clone)]
#[repr(C)]
struct V {
    pub a_pos: [f32; 2],
    pub a_tc: [f32; 2],
    pub a_srgba: [u8; 4],
}

unsafe impl Zeroable for V {}
unsafe impl Pod for V {}

impl From<paint::Vertex> for V {
    fn from(v: paint::Vertex) -> Self {
        let paint::Vertex {
            pos,
            uv,
            color: paint::Color { r, g, b, a },
        } = v;
        Self {
            a_pos: [pos.x, pos.y],
            a_tc: [uv.0 as f32, uv.1 as f32],
            a_srgba: [r, g, b, a],
        }
    }
}

/// All events you pass to the UI state should be
/// convertable to this type.
#[derive(Debug, Clone, Copy)]
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

pub struct EguiRendererDescriptor<S: UiState> {
    pub state: S,
    pub fmt: TextureFormat,
    pub target_size: (f32, f32),
    pub screen_size: (f32, f32),
}

impl<S> EguiRenderer<S>
where
    S: UiState,
{
    /// fmt should be the same format that you render EGui to.
    pub fn new(dev: &Device, queue: &Queue, desc: EguiRendererDescriptor<S>) -> Self {
        let mut ctx = Context::new();
        let raw_input = RawInput::default();
        let _ = ctx.begin_frame(raw_input);

        let EguiRendererDescriptor {
            fmt,
            screen_size,
            state,
            target_size,
        } = desc;

        let ui_pl = Pipeline::new(dev, queue, ctx.texture(), fmt, screen_size, target_size);
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
    pub fn draw_on(
        &mut self,
        mut com: CommandEncoder,
        dev: &Device,
        queue: &Queue,
        frame: SwapChainFrame,
    ) {
        {
            // render the scene
            let mut ui = self.ctx.begin_frame(self.raw_input.take());
            self.state.draw(&mut ui);

            let (_, jobs) = self.ctx.end_frame();

            if self.ctx.texture().id != self.ui_pl.tex_hash {
                self.ui_pl.rebuild_texture(&queue, self.ctx.as_ref());
            }

            let buffers: Vec<_> = jobs
                .into_iter()
                .map(|(egui::Rect { min, max }, triangles)| {
                    let vert_buf = dev.create_buffer(&BufferDescriptor {
                        label: Some("egui-wgpu :: vertex_buffer "),
                        size: size_of::<V>() as u64 * triangles.vertices.len() as u64,
                        usage: BufferUsage::VERTEX | BufferUsage::COPY_DST,
                        mapped_at_creation: true,
                    });

                    let idx_buf = dev.create_buffer(&BufferDescriptor {
                        label: Some("egui-wgpu :: index_buffer "),
                        size: size_of::<u32>() as u64 * triangles.indices.len() as u64,
                        usage: BufferUsage::INDEX | BufferUsage::COPY_DST,
                        mapped_at_creation: true,
                    });

                    {
                        let mut idx = idx_buf.slice(..).get_mapped_range_mut();
                        idx.copy_from_slice(cast_slice(&triangles.indices));
                    }
                    idx_buf.unmap();

                    {
                        let mut vtx = vert_buf.slice(..).get_mapped_range_mut();
                        let verts: Vec<_> = triangles.vertices.into_iter().map(V::from).collect();
                        vtx.copy_from_slice(cast_slice(&verts));
                    }
                    vert_buf.unmap();

                    (
                        vert_buf,
                        idx_buf,
                        triangles.indices.len(),
                        (min.x, min.y, max.x, max.y),
                    )
                })
                .collect();

            let mut rpass = com.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.output.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.2,
                            g: 0.15,
                            b: 0.14,
                            a: 1.,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            rpass.set_pipeline(&self.ui_pl.pl);
            rpass.set_bind_group(0, &self.ui_pl.vert_bg, &[]);
            rpass.set_bind_group(1, &self.ui_pl.frag_bg, &[]);

            buffers.iter().for_each(|(v, i, ct, (x, y, w, h))| {
                rpass.set_viewport(*x, *y, *w, *h, 1., 0.);
                rpass.set_vertex_buffer(0, v.slice(..));
                rpass.set_index_buffer(i.slice(..));
                rpass.draw_indexed(0..*ct as u32, 0, 0..1);
            });
        }

        queue.submit(Some(com.finish()));
    }
}
