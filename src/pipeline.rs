use std::{mem::size_of, num::NonZeroU64};
use wgpu::{
    vertex_attr_array, BindGroupDescriptor, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Device, IndexFormat, InputStepMode, PipelineLayout,
    PipelineLayoutDescriptor, PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor,
    ShaderStage, TextureComponentType, TextureFormat, TextureViewDimension, VertexBufferDescriptor,
    VertexStateDescriptor,
};

use crate::{default_mod, load_frag, load_vert};

pub struct Pipeline {
    pl: RenderPipeline,
    // vert_bg: BindGroup,
    // frag_bg: BindGroup
}

impl Pipeline {
    pub fn new(dev: &Device, fmt: TextureFormat) -> Self {
        // TODO: put these in const position with an updated version of the
        // layout macro
        let vert_layout = dev.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("egui-wgpu :: vert_bgl"),
            entries: &[BindGroupLayoutEntry {
                visibility: ShaderStage::VERTEX,
                binding: 0,
                count: None,
                ty: BindingType::UniformBuffer {
                    dynamic: false,
                    min_binding_size: NonZeroU64::new(size_of::<[f32; 4]>() as u64),
                },
            }],
        });

        let frag_layout = dev.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("egui-wgpu :: frag_bgl"),
            entries: &[
                BindGroupLayoutEntry {
                    visibility: ShaderStage::FRAGMENT,
                    binding: 0,
                    count: None,
                    ty: BindingType::Sampler { comparison: false },
                },
                BindGroupLayoutEntry {
                    visibility: ShaderStage::FRAGMENT,
                    binding: 1,
                    count: None,
                    ty: BindingType::SampledTexture {
                        dimension: TextureViewDimension::D2,
                        component_type: TextureComponentType::Uint,
                        multisampled: false,
                    },
                },
            ],
        });

        let pl_layout = dev.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("egui-wgpu :: render_pl_layout"),
            bind_group_layouts: &[&vert_layout, &frag_layout],
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
            color_states: &[fmt.into()],
            depth_stencil_state: None,
            vertex_state,
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        Self { pl }
    }

    pub fn inner(&self) -> &RenderPipeline {
        &self.pl
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

        let _ = Pipeline::new(&device, TextureFormat::Rgba8UnormSrgb);
    }
}
