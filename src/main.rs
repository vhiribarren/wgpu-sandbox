mod draw_context;
mod triangle;

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

use crate::draw_context::Drawable;
use log::{debug, info};
use winit::error::OsError;

const GLOBAL_LOG_FILTER: log::LevelFilter = log::LevelFilter::Info;
#[cfg(target_arch = "wasm32")]
const WEBAPP_CANVAS_ID: &str = "target";

const DEFAULT_SHADER: &str = include_str!("./shaders/default.wgsl");
const DEFAULT_SHADER_MAIN_FRG: &str = "frg_main";
const DEFAULT_SHADER_MAIN_VTX: &str = "vtx_main";

fn init_log() {
    let mut builder = fern::Dispatch::new();
    let level_formatter;
    #[cfg(target_arch = "wasm32")]
    {
        level_formatter = |level| level;
        builder = builder.chain(fern::Output::call(console_log::log));
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use fern::colors::{Color, ColoredLevelConfig};
        let colors = ColoredLevelConfig::new()
            .info(Color::Blue)
            .debug(Color::Green);
        level_formatter = move |level| colors.color(level);
        builder = builder.chain(std::io::stdout());
    }
    builder
        .level(GLOBAL_LOG_FILTER)
        .level_for(module_path!(), log::LevelFilter::Debug)
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}:{}] {}",
                chrono::Local::now().format("[%H:%M:%S]"),
                level_formatter(record.level()),
                record.target(),
                record.line().unwrap_or_default(),
                message
            ))
        })
        .apply()
        .unwrap();
}

fn create_window<T>(event_loop: &EventLoop<T>) -> Result<Window, OsError> {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        use winit::platform::web::WindowBuilderExtWebSys;
        let dom_window = web_sys::window().unwrap();
        let dom_document = dom_window.document().unwrap();
        let dom_canvas = dom_document.get_element_by_id(WEBAPP_CANVAS_ID).unwrap();
        let canvas = dom_canvas.dyn_into::<web_sys::HtmlCanvasElement>().ok();
        WindowBuilder::default()
            .with_canvas(canvas)
            .build(event_loop)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        WindowBuilder::default().build(event_loop)
    }
}

async fn async_main() {
    let event_loop = EventLoop::new();
    let window = create_window(&event_loop).unwrap();
    dbg!(window.inner_size());
    let draw_context = draw_context::DrawContext::new(
        &window,
        window.inner_size().width,
        window.inner_size().height,
    )
    .await
    .unwrap();
    let triangle = {
        let default_shader_module =
            draw_context
                .device
                .create_shader_module(&wgpu::ShaderModuleDescriptor {
                    label: Some("Fragment Shader"),
                    source: wgpu::ShaderSource::Wgsl(DEFAULT_SHADER.into()),
                });
        let vertex_state = wgpu::VertexState {
            module: &default_shader_module,
            entry_point: DEFAULT_SHADER_MAIN_VTX,
            buffers: &[draw_context.vertex_buffer_layout.clone()],
        };
        let fragment_state = wgpu::FragmentState {
            module: &default_shader_module,
            entry_point: DEFAULT_SHADER_MAIN_FRG,
            targets: &[wgpu::ColorTargetState {
                format: draw_context.config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            }],
        };
        triangle::Triangle::init(&draw_context, vertex_state, fragment_state)
    };
    let objects: Vec<Box<dyn Drawable>> = vec![Box::new(triangle)];
    event_loop.run(move |event, _target, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            debug!("Closing app");
            *control_flow = ControlFlow::Exit;
        }
        Event::WindowEvent {
            event: WindowEvent::Resized(_),
            ..
        } => {
            debug!("Window resized");
        }
        Event::MainEventsCleared => {
            //window.request_redraw();
        }
        Event::RedrawRequested(_) => {
            draw_context.render_objects(objects.as_slice()).unwrap();
        }
        _ => {}
    });
}

fn main() {
    init_log();
    info!("Init app");
    let main_future = async_main();
    #[cfg(target_arch = "wasm32")]
    {
        wasm_bindgen_futures::spawn_local(main_future);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        pollster::block_on(main_future);
    }
}
