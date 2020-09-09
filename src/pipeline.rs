use wgpu::{
    vertex_attr_array, Device, IndexFormat, InputStepMode, PipelineLayout,
    PipelineLayoutDescriptor, PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor,
    VertexBufferDescriptor, VertexStateDescriptor,
};

use crate::{default_mod, load_frag, load_vert};

pub struct Pipeline {
    pl: RenderPipeline,
    pl_layout: PipelineLayout,
    // vert_bg: BindGroup,
    // frag_bg: BindGroup,
}

impl Pipeline {
    pub fn new(dev: &Device) -> Self {
        let pl_layout = dev.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("egui-wgpu :: render_pl_layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        //TODO: when desc and state are available to be put in const
        // position again do so.
        let vertex_desc = VertexBufferDescriptor {
            attributes: &vertex_attr_array![0 => Float2, 1 => Ushort2, 2 => Uchar4],
            stride: std::mem::size_of::<Self>() as u64,
            step_mode: InputStepMode::Vertex,
        };

        let vertex_state = VertexStateDescriptor {
            index_format: IndexFormat::Uint32,
            vertex_buffers: &[vertex_desc],
        };

        let pl = dev.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("egui-wgpu :: render_pl"),
            layout: Some(&pl_layout),
            vertex_stage: default_mod(&load_vert(&dev)),
            fragment_stage: Some(default_mod(&load_frag(&dev))),
            rasterization_state: None,
            primitive_topology: PrimitiveTopology::TriangleList,
            color_states: &[],
            depth_stencil_state: None,
            vertex_state,
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        Self { pl, pl_layout }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use futures::executor::block_on;

    #[test]
    pub fn pipeline_runs() {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: None,
        }))
        .unwrap();
        let (device, _) =
            block_on(adapter.request_device(&wgpu::DeviceDescriptor::default(), None)).unwrap();

        let pl = Pipeline::new(&device);
    }
}
