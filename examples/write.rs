use window_clipboard::Clipboard;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() -> Result<(), impl std::error::Error> {
    let event_loop = EventLoop::new()?;

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .build(&event_loop)?;

    let mut clipboard =
        Clipboard::connect(&window).expect("Connect to clipboard");

    clipboard
        .write(String::from("Hello, world!"))
        .expect("Write to clipboard");

    event_loop.run(move |event, target| match event {
        Event::AboutToWait => {}
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            window_id,
        } if window_id == window.id() => target.exit(),
        _ => target.set_control_flow(ControlFlow::Wait),
    })
}
