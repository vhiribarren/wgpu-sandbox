use anyhow::anyhow;
use log::info;
use wgpu::RequestAdapterOptions;

pub struct WebGPU {
    adapter: wgpu::Adapter,
}

impl WebGPU {
    pub async fn new(
        window_handler: &impl raw_window_handle::HasRawWindowHandle,
    ) -> anyhow::Result<WebGPU> {
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window_handler) };
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: Default::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .ok_or(anyhow!("Could not create WebGPU adapter"))?;
        info!("{:?}", adapter);
        Ok(WebGPU { adapter })
    }
}
