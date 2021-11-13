mod webgpu;

use std::future::Future;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

use log::{debug, info};
use winit::error::OsError;

#[cfg(target_arch = "wasm32")]
const WEBAPP_CANVAS_ID: &str = "target";

#[cfg(target_arch = "wasm32")]
fn init_log() {
    console_log::init_with_level(log::Level::Trace).unwrap();
}

#[cfg(target_arch = "wasm32")]
fn init_window<T>(event_loop: &EventLoop<T>) -> Result<Window, OsError> {
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
fn init_log() {
    use env_logger::Builder;
    Builder::new()
        .filter_module(module_path!(), log::LevelFilter::max())
        .init();
}

#[cfg(not(target_arch = "wasm32"))]
fn init_window<T>(event_loop: &EventLoop<T>) -> Result<Window, OsError> {
    WindowBuilder::default().build(event_loop)
}

async fn async_start() {
    let event_loop = EventLoop::new();
    let window = init_window(&event_loop).unwrap();
    webgpu::WebGPU::new(&window).await.unwrap();

    event_loop.run(|event, _target, control_flow| match event {
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
        Event::MainEventsCleared => {}
        Event::RedrawRequested(_) => {}
        _ => {}
    });
}

fn main() {
    init_log();
    info!("Init app");
    let start_future = async_start();
    #[cfg(not(target_arch = "wasm32"))]
    {
        pollster::block_on(start_future);
    }

    #[cfg(target_arch = "wasm32")]
    {
        wasm_bindgen_futures::spawn_local(start_future);
    }
}
