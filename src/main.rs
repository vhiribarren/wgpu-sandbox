use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;


fn main() {
    let event_loop = EventLoop::new();
    let _window = WindowBuilder::default().build(&event_loop).unwrap();
    event_loop.run(|event, _target, control_flow| {
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } =>  {
                *control_flow = ControlFlow::Exit;
            }
            Event::MainEventsCleared => {}
            Event::RedrawRequested(_) => {}
            _ => {}
        }
    });
}
