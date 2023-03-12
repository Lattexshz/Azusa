#![allow(clippy::single_match)]

use azusa::window::WindowSurface;
use azusa::{Azusa, Color, ImageSurface, ImageType, Surface};

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

    let mut surface = WindowSurface::new(&window).unwrap();
    let mut png = ImageSurface::new(0.0, 0.0, "A fantastic window", ImageType::Png);
    let mut azusa = Azusa::new();

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => control_flow.set_exit(),
            Event::WindowEvent {
                event: WindowEvent::Resized(_size),
                ..
            } => {}
            Event::WindowEvent {
                event: WindowEvent::ReceivedCharacter(c),
                ..
            } => {
                if c == 's' {
                    let (w, h) = surface.get_client_size();
                    if w != 0 && h != 0 {
                        png.resize(w as f64, h as f64);
                    }
                    azusa.draw(&mut png);
                }
            }
            Event::RedrawEventsCleared => {
                window.request_redraw();
                azusa.set_source_color(Color::Navy);
                azusa.clear();
                azusa.set_source_color(Color::Red);
                azusa.move_to(10, 10);
                azusa.fill_rectangle(520, 520);
                azusa.draw(&mut surface);
            }
            _ => (),
        }
    });
}
