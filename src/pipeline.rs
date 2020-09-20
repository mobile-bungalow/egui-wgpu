use std::{mem::size_of, num::NonZeroU64};
use wgpu::*;

use crate::{default_mod, load_frag, load_vert};

pub struct Pipeline {
    pub pl: RenderPipeline,
    pub vert_bg: BindGroup,
    pub frag_bg: BindGroup,
    pub egui_tex: Texture,
    pub vert_uniform_buf: Buffer,
    pub tex_hash: u64,
}

impl Pipeline {
    pub fn new(
        dev: &Device,
        q: &Queue,
        tex: &egui::paint::Texture,
        fmt: TextureFormat,
        screen_dims: (f32, f32),
        target_dims: (f32, f32),
    ) -> Self {
        // TODO: put these in const position with an updated version of the
        // layout macro
        let vert_layout = dev.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("egui-wgpu :: vert_bind_group_layout"),
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

        let vert_uniform_buf = dev.create_buffer(&BufferDescriptor {
            label: Some("egui-wgpu :: vertex_uniform_buffer"),
            size: size_of::<[f32; 4]>() as u64,
            usage: BufferUsage::UNIFORM,
            mapped_at_creation: true,
        });

        vert_uniform_buf
            .slice(..)
            .get_mapped_range_mut()
            .copy_from_slice(bytemuck::cast_slice(&[
                screen_dims.0,
                screen_dims.1,
                target_dims.0,
                target_dims.1,
            ]));
        vert_uniform_buf.unmap();

        let vert_bg = dev.create_bind_group(&BindGroupDescriptor {
            label: Some("egui-wgpu :: vert_bind_group"),
            layout: &vert_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: BindingResource::Buffer(vert_uniform_buf.slice(..)),
            }],
        });

        let frag_layout = dev.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("egui-wgpu :: frag_bind_group_layout"),
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

        let egui_tex = dev.create_texture(&TextureDescriptor {
            label: Some("egui-wgpu :: main_texture"),
            size: wgpu::Extent3d {
                height: tex.height as u32,
                width: tex.width as u32,
                depth: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Uint,
            usage: TextureUsage::SAMPLED | TextureUsage::COPY_DST,
        });

        let egui_sampler = dev.create_sampler(&SamplerDescriptor {
            label: Some("egui-wgpu :: main_sampler"),
            ..Default::default()
        });

        let frag_bg = dev.create_bind_group(&BindGroupDescriptor {
            label: Some("egui-wgpu :: frag_bind_group"),
            layout: &frag_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Sampler(&egui_sampler),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&egui_tex.create_view(
                        &TextureViewDescriptor {
                            label: Some("egui-wgpu :: main_texture_view"),
                            dimension: Some(TextureViewDimension::D2),
                            ..Default::default()
                        },
                    )),
                },
            ],
        });

        let pl_layout = dev.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("egui-wgpu :: render_pl_layout"),
            bind_group_layouts: &[&vert_layout, &frag_layout],
            push_constant_ranges: &[],
        });

        let pixels = tex.pixels.iter().fold(Vec::<u8>::new(), |mut vec, byte| {
            vec.extend(&[*byte; 4]);
            vec
        });

        q.write_texture(
            wgpu::TextureCopyView {
                texture: &egui_tex,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &pixels,
            wgpu::TextureDataLayout {
                offset: 0,
                bytes_per_row: tex.width as u32 * 4,
                rows_per_image: tex.height as u32,
            },
            wgpu::Extent3d {
                width: tex.width as u32,
                height: tex.height as u32,
                depth: 1,
            },
        );

        //TODO: when desc and state are available to be put in const
        // position again do so.
        let vertex_desc = VertexBufferDescriptor {
            attributes: &vertex_attr_array![0 => Float2, 1 => Float2, 2 => Uchar4],
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

        Self {
            pl,
            frag_bg,
            vert_bg,
            vert_uniform_buf,
            egui_tex,
            tex_hash: tex.id,
        }
    }

    pub fn rebuild_texture(&mut self, queue: &Queue, tex: &egui::paint::Texture) {
        //queue.write_texture(
        //    wgpu::TextureCopyView {
        //        texture: &self.egui_tex,
        //        mip_level: 0,
        //        origin: wgpu::Origin3d::ZERO,
        //    },
        //    &tex.pixels,
        //    wgpu::TextureDataLayout {
        //        offset: 0,
        //        bytes_per_row: tex.width as u32 * 4,
        //        rows_per_image: tex.height as u32,
        //    },
        //    wgpu::Extent3d {
        //        width: tex.width as u32,
        //        height: tex.height as u32,
        //        depth: 1,
        //    },
        //);
    }
}
