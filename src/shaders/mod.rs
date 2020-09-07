use wgpu::{include_spirv, Device, ShaderModule};

pub fn load_vert(dev: &Device) -> ShaderModule {
    let src = include_spirv!("vert.sprv");
    dev.create_shader_module(src)
}

pub fn load_frag(dev: &Device) -> ShaderModule {
    let src = include_spirv!("frag.sprv");
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
        load_vert(&device);
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
        load_frag(&device);
    }
}
