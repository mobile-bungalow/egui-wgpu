use egui_wgpu::{EguiRenderer, EventBridge, UiState};
use wgpu::TextureFormat;
use winit::{
    dpi::PhysicalSize, //    window::Window,
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

const FMT: TextureFormat = TextureFormat::Rgba8UnormSrgb;
struct EventWrapper<'a>(Event<'a, ()>);

impl<'a> Into<EventBridge> for EventWrapper<'a> {
    fn into(self) -> EventBridge {
        match self.0 {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(PhysicalSize { width, height }) => EventBridge::Resize {
                    w: width as f32,
                    h: height as f32,
                },
                WindowEvent::CursorMoved { position: p, .. } => EventBridge::MouseMove {
                    x: p.x as f32,
                    y: p.y as f32,
                },
                WindowEvent::MouseInput { state, .. } => match state {
                    ElementState::Pressed => EventBridge::MouseDown,
                    ElementState::Released => EventBridge::MouseUp,
                },
                WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                    EventBridge::DpiChanged(scale_factor as f32)
                }
                _ => EventBridge::Ignore,
            },
            _ => EventBridge::Ignore,
        }
    }
}

struct UI;

impl UiState for UI {
    fn draw(&self, ui: &mut egui::Ui) {
        ui.add(egui::Label::new("Egui on WGPU").text_style(egui::TextStyle::Heading));
        ui.separator();
        ui.label("Oh Yes!");
        if ui.button("Quit").clicked {
            std::process::exit(0);
        }
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
    let (device, _queue) = futures::executor::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
            shader_validation: true,
        },
        None,
    ))
    .expect("Failed to create device");

    let ui_state = UI;
    let egui_renderer = EguiRenderer::new(&device, ui_state, FMT);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(_size),
                ..
            } => {}
            Event::RedrawRequested(_) => {}
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });
}
