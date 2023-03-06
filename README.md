# Azusa
## Rusty 2D graphic library

# Concept
## Context-based drawing management
Azusa uses a drawing context called Azusa to manage drawing.

# Versatility
Azusa allows you to write your own drawing methods by implementing a trait called Surface in your own structure.  
(For example, you can use it to draw a picture consisting only of * and spaces.)  

# Example
Sample of drawing in a window
```rust
#![allow(clippy::single_match)]

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use azusa::{Azusa, Color, ImageSurface};
use azusa::window::WindowSurface;

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0))
        .build(&event_loop)
        .unwrap();

    let mut surface = WindowSurface::new(&window).unwrap();
    let mut azusa = Azusa::new();

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => control_flow.set_exit(),
            Event::MainEventsCleared => {
                window.request_redraw();
                azusa.clear(Color::Blue);
                azusa.draw(&mut surface);
            }
            _ => (),
        }
    });
}
```

# Inspirations
Azusa's interface is influenced by Cairo.  
Based on it, it is further lightened and made more portable using raw-window-handle  

# Origin of the name
The name of this repository is taken from the Azusa River(Matsumoto City,Nagano prefecture) in Japan.

# LICENSE
"Azusa" is licensed under [MIT LICENSE]()
