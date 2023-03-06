# Drawing to Window
In the previous section, simple Azusa usage was described.
In fact, Azusa also allows you to draw in a window.  

To draw on a window in Azusa, a WindowSurface must be created.  
A RawWindowHandle is required to create a WindowSurface.  
This is independent of any particular Windowing system.   

Here is a sample code  
````rust
#![allow(clippy::single_match)]

use azusa::window::WindowSurface;
use azusa::{Azusa, Color, ImageSurface, ImageType};

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(winit::dpi::LogicalSize::new(520.0, 520.0))
        .build(&event_loop)
        .unwrap();


    // Creates a surface for a window.  
    // Pass HasRawWindowHandle and the implemented struct as arguments.
    let mut surface = WindowSurface::new(&window).unwrap();
    // Also create an ImageSurface so that it can be output to PNG
    let mut png = ImageSurface::new(0.0, 0.0, "A fantastic window", ImageType::Png);
    // Creates a drawing context
    let mut azusa = Azusa::new();

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => control_flow.set_exit(),
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                // ImageSurface does not have the ability to automatically resize surfaces,  
                // so resize manually
                png.resize((size.width - 130) as f64, (size.height - 130) as f64);
            }
            Event::WindowEvent {
                event: WindowEvent::ReceivedCharacter(c),
                ..
            } => {
                // Output to PNG when keyboard s is pressed
                if c == 's' {
                    azusa.draw(&mut png);
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
                azusa.set_source_color(Color::White);
                azusa.clear();
                azusa.set_source_color(Color::Lime);
                azusa.rectangle(5, 5, 510, 510);
                azusa.draw(&mut surface);
            }
            _ => (),
        }
    });
}
````
In this example, it means that you can draw in a window and the context can be used around.  
So you could draw in a window or output to a PNG.  
