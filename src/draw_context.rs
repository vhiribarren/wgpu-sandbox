use anyhow::anyhow;
use log::debug;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    fn vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                },
            ],
        }
    }
}

pub trait Drawable {
    fn render_pipeline(&self) -> &wgpu::RenderPipeline;
    fn vertex_buffer(&self) -> &wgpu::Buffer;
    fn vertex_count(&self) -> usize;
}

pub struct DrawContext<'a> {
    _adapter: wgpu::Adapter,
    surface: wgpu::Surface,
    queue: wgpu::Queue,
    pub device: wgpu::Device,
    pub vertex_buffer_layout: wgpu::VertexBufferLayout<'a>,
    pub config: wgpu::SurfaceConfiguration,
}

impl DrawContext<'_> {
    pub async fn new<'a, 'b>(
        window_handler: &'a impl raw_window_handle::HasRawWindowHandle,
        width: u32,
        height: u32,
    ) -> anyhow::Result<DrawContext<'b>> {
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window_handler) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: Default::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .ok_or_else(|| anyhow!("Could not create WebGPU adapter"))?;
        debug!("{:?}", adapter);
        debug!("{:?}", adapter.features());
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Device Descriptor"),
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::downlevel_webgl2_defaults(),
                },
                None,
            )
            .await?;
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);
        let vertex_buffer_layout = Vertex::vertex_buffer_layout();
        Ok(DrawContext {
            _adapter: adapter,
            surface,
            device,
            queue,
            config,
            vertex_buffer_layout,
        })
    }

    pub fn render_objects(&self, drawables: &[Box<dyn Drawable>]) -> anyhow::Result<()> {
        let displayed_texture = self.surface.get_current_texture()?;
        let view = displayed_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Command Encoder"),
            });
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 1.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });
        for drawable in drawables {
            render_pass.set_pipeline(drawable.render_pipeline());
            render_pass.set_vertex_buffer(0, drawable.vertex_buffer().slice(..));
            render_pass.draw(0..(drawable.vertex_count() as u32), 0..1);
        }
        drop(render_pass);
        let command_buffers = std::iter::once(encoder.finish());
        self.queue.submit(command_buffers);
        displayed_texture.present();
        Ok(())
    }
}
