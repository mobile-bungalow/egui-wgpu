use egui_wgpu::{EguiRenderer, EguiRendererDescriptor, EventBridge, UiState};
use wgpu::TextureFormat;
use winit::{
    dpi::PhysicalSize, //    window::Window,
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

const FMT: TextureFormat = TextureFormat::Bgra8UnormSrgb;
struct EventWrapper<'a, 'b>(&'b Event<'a, ()>);

impl<'a, 'b> Into<EventBridge> for EventWrapper<'a, 'b> {
    fn into(self) -> EventBridge {
        match self.0 {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(PhysicalSize { width, height }) => EventBridge::Resize {
                    w: *width as f32,
                    h: *height as f32,
                },
                WindowEvent::CursorMoved { position: p, .. } => EventBridge::MouseMove {
                    x: p.x as f32,
                    y: p.y as f32,
                },
                WindowEvent::MouseWheel {
                    delta: winit::event::MouseScrollDelta::LineDelta(x, y),
                    ..
                } => EventBridge::Scroll { x: *x, y: *y },
                WindowEvent::MouseWheel {
                    delta: winit::event::MouseScrollDelta::PixelDelta(pos),
                    ..
                } => EventBridge::Scroll {
                    x: pos.x as f32,
                    y: pos.y as f32,
                },
                WindowEvent::MouseInput { state, .. } => match state {
                    ElementState::Pressed => EventBridge::MouseDown,
                    ElementState::Released => EventBridge::MouseUp,
                },
                WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                    EventBridge::DpiChanged(*scale_factor as f32)
                }
                _ => EventBridge::Ignore,
            },
            _ => EventBridge::Ignore,
        }
    }
}

#[derive(Copy, Clone)]
struct UI;

impl UiState for UI {
    fn draw(&self, ui: &mut egui::Ui) {
        egui::Window::new("Debug")
            .default_size(egui::vec2(200.0, 100.0))
            .show(ui.ctx(), |ui| {
                ui.add(egui::Label::new("Egui on WGPU").text_style(egui::TextStyle::Heading));
                ui.separator();
                ui.label("Oh Yes!");
                if ui.button("Quit").clicked {
                    std::process::exit(0);
                }
            });
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();

    let size = window.inner_size();
    let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
    let surface = unsafe { instance.create_surface(&window) };
    let adapter =
        futures::executor::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::Default,
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
        }))
        .expect("Failed to find an appropiate adapter");

    // Create the logical device and command queue
    let (device, queue) = futures::executor::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
            shader_validation: true,
        },
        None,
    ))
    .expect("Failed to create device");

    let ui_state = UI;
    let mut egui_renderer = EguiRenderer::new(
        &device,
        &queue,
        EguiRendererDescriptor {
            fmt: FMT,
            state: ui_state,
            screen_size: window.inner_size().into(),
            target_size: window.inner_size().into(),
        },
    );

    let mut sc_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: FMT,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Mailbox,
    };

    let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);

    event_loop.run(move |event, _, control_flow| {
        egui_renderer.consume_event(EventWrapper(&event));
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                sc_desc.width = size.width;
                sc_desc.height = size.height;
                swap_chain = device.create_swap_chain(&surface, &sc_desc);
            }
            // This is lazy and bad do not cargo cult it.
            Event::MainEventsCleared => {
                let frame = swap_chain.get_current_frame().expect("Swap Chain Failed");
                let encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("egui-wgpu :: ui encoder"),
                });
                egui_renderer.draw_on(encoder, &device, &queue, frame);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });
}
