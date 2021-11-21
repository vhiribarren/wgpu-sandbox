mod draw_context;

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

use log::{debug, info};
use winit::error::OsError;

const GLOBAL_LOG_FILTER: log::LevelFilter = log::LevelFilter::Info;
#[cfg(target_arch = "wasm32")]
const WEBAPP_CANVAS_ID: &str = "target";

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
            draw_context.render().unwrap();
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
