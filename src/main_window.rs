/*
MIT License

Copyright (c) 2021, 2022 Vincent Hiribarren

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use instant::{Duration, Instant};
use winit::event::{DeviceEvent, ElementState, Event, MouseButton, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{CursorIcon, Window, WindowBuilder};

use crate::cameras::{Camera, PerspectiveConfig, WinitCameraAdapter};
use crate::draw_context;
use crate::scenario::{Scenario, UpdateInterval};
use log::{debug, info};
use winit::error::OsError;

const GLOBAL_LOG_FILTER: log::LevelFilter = log::LevelFilter::Info;
#[cfg(target_arch = "wasm32")]
const WEBAPP_CANVAS_ID: &str = "target";

const TARGET_DRAW_FPS: f64 = 60.0;

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

struct MouseState {
    pub is_cursor_inside: bool,
    mouse_rotation_enabled: bool,
}

impl MouseState {
    pub fn new() -> Self {
        MouseState {
            is_cursor_inside: false,
            mouse_rotation_enabled: false,
        }
    }
    pub fn left_button_action(&mut self, action: ElementState, window: &Window) {
        if !self.is_cursor_inside {
            return;
        }
        match action {
            ElementState::Pressed => {
                self.mouse_rotation_enabled = true;
                window.set_cursor_visible(false);
            }
            ElementState::Released => {
                self.mouse_rotation_enabled = false;
                window.set_cursor_visible(true);
            }
        }
    }

    pub fn resize_action(&mut self, window: &Window) {
        self.mouse_rotation_enabled = false;
        window.set_cursor_visible(true);
    }

    pub fn is_mouse_rotation_enabled(&self) -> bool {
        self.mouse_rotation_enabled
    }

    pub fn move_action(&mut self) {
        self.mouse_rotation_enabled = false;
    }
}

async fn async_main<S: Scenario + 'static>() {
    let event_loop = EventLoop::new();
    let window = create_window(&event_loop).unwrap();
    window.set_cursor_icon(CursorIcon::Grab);
    let mut draw_context = draw_context::DrawContext::new(
        &window,
        window.inner_size().width,
        window.inner_size().height,
    )
    .await
    .unwrap();
    let mut mouse_state = MouseState::new();
    let mut scenario = S::new(&draw_context);
    let scenario_start = Instant::now();
    let mut last_draw_instant = scenario_start;
    let draw_period_target = Duration::from_secs_f64(1.0 / TARGET_DRAW_FPS);
    let mut winit_camera = WinitCameraAdapter::new(Camera::from(PerspectiveConfig {
        //OrthogonalConfig {
        ..Default::default()
    }));

    event_loop.run(move |event, _target, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                debug!("Closing app");
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(physical_size),
                ..
            } => {
                mouse_state.resize_action(&window);
                draw_context.resize(physical_size.width, physical_size.height);
            }
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { ref input, .. },
                ..
            } => {
                winit_camera.keyboard_event_listener(input);
            }
            Event::WindowEvent {
                event: WindowEvent::Moved { .. },
                ..
            } => {
                dbg!("Window moved");
                mouse_state.move_action();
            }
            Event::WindowEvent {
                event: WindowEvent::CursorEntered { .. },
                ..
            } => {
                mouse_state.is_cursor_inside = true;
            }
            Event::WindowEvent {
                event: WindowEvent::CursorLeft { .. },
                ..
            } => {
                mouse_state.is_cursor_inside = false;
            }
            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => {
                // Works with WASM and browser canvas
                if button == MouseButton::Left {
                    mouse_state.left_button_action(state, &window);
                }
            }
            Event::DeviceEvent { ref event, .. } => {
                if let DeviceEvent::Button { button, state } = event {
                    // Works with MacOS
                    if *button == 0 {
                        mouse_state.left_button_action(*state, &window);
                    }
                }
                if mouse_state.is_mouse_rotation_enabled() {
                    winit_camera.mouse_event_listener(event);
                }
            }
            Event::MainEventsCleared => {
                let since_last_draw = last_draw_instant.elapsed();
                if since_last_draw >= draw_period_target {
                    window.request_redraw();
                    *control_flow = ControlFlow::Poll;
                } else {
                    *control_flow = ControlFlow::WaitUntil(
                        Instant::now() + draw_period_target - since_last_draw,
                    );
                }
            }
            Event::RedrawRequested(_) => {
                let update_delta = last_draw_instant.elapsed();
                last_draw_instant = Instant::now();
                scenario.update(
                    &draw_context,
                    &UpdateInterval {
                        scenario_start,
                        update_delta,
                    },
                );
                winit_camera.update();
                draw_context.set_projection(winit_camera.get_camera_matrix());
                draw_context.render_scene(&scenario).unwrap();
            }
            _ => {}
        }
    });
}

pub fn main_with_scenario<S: Scenario + 'static>() {
    init_log();
    info!("Init app");
    let main_future = async_main::<S>();
    #[cfg(target_arch = "wasm32")]
    {
        wasm_bindgen_futures::spawn_local(main_future);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        pollster::block_on(main_future);
    }
}
