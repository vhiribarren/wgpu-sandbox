use env_logger::Builder;
use log::LevelFilter;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use log::debug;

fn main() {
    Builder::new().filter_module(module_path!() , LevelFilter::max()).init();
    let event_loop = EventLoop::new();
    let _window = WindowBuilder::default().build(&event_loop).unwrap();
    event_loop.run(|event, _target, control_flow| {
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } =>  {
                debug!("Closing app");
                *control_flow = ControlFlow::Exit;
            }
            Event::MainEventsCleared => {}
            Event::RedrawRequested(_) => {}
            _ => {}
        }
    });
}
