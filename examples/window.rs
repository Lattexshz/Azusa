#![allow(clippy::single_match)]

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use winit::dpi::LogicalSize;
use azusa::{Azusa, Color, ImageSurface, ImageType};
use azusa::window::WindowSurface;

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(winit::dpi::LogicalSize::new(520.0,520.0))
        .build(&event_loop)
        .unwrap();

    let mut surface = WindowSurface::new(&window).unwrap();
    let mut png = ImageSurface::new(0.0,0.0,"A fantastic window",ImageType::Png);
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
            }=> {
                png.resize((size.width-130) as f64, (size.height-130) as f64);
            }
            Event::WindowEvent {
                event: WindowEvent::ReceivedCharacter(c),
                ..
            } => {
                if c == 's' {
                    azusa.draw(&mut png);
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
                azusa.set_source_color(Color::Fuchsia);
                azusa.clear();
                azusa.set_source_color(Color::Navy);
                azusa.rectangle(20,20,150,150);
                azusa.draw(&mut surface);
            }
            _ => (),
        }
    });
}