use wgpu::{
    include_spirv, // BindGroupLayoutDescriptor,
    Device,
    ProgrammableStageDescriptor,
    ShaderModule,
};

//const DESC: BindGroupLayoutDescriptor = BindGroupLayoutDescriptor {
//    label: Some("egui-wgpu :: bind_group_descriptor"),
//};

/// creates a shader stage with entry point main from a shader module.
pub fn default_mod<'a>(module: &'a ShaderModule) -> ProgrammableStageDescriptor<'a> {
    ProgrammableStageDescriptor {
        module,
        entry_point: "main",
    }
}

pub fn load_vert(dev: &Device) -> ShaderModule {
    let src = include_spirv!("vert.spv");
    dev.create_shader_module(src)
}

pub fn load_frag(dev: &Device) -> ShaderModule {
    let src = include_spirv!("frag.spv");
    dev.create_shader_module(src)
}

#[cfg(test)]
mod test {
    use super::{load_frag, load_vert};
    use futures::executor::block_on;

    #[test]
    fn loading_vert_does_not_panic() {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: None,
        }))
        .unwrap();
        let (device, _) =
            block_on(adapter.request_device(&wgpu::DeviceDescriptor::default(), None)).unwrap();
        let _ = load_vert(&device);
    }

    #[test]
    fn loading_frag_does_not_panic() {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: None,
        }))
        .unwrap();
        let (device, _) =
            block_on(adapter.request_device(&wgpu::DeviceDescriptor::default(), None)).unwrap();
        let _ = load_frag(&device);
    }
}
