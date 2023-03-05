use azusa::{Azusa, Color, ImageSurface, ImageType};

fn main() {
    let surface = ImageSurface::new(1280.0,720.0,"sample",ImageType::Png);
    let mut azusa = Azusa::new();

    azusa.clear(Color::Blue);

    azusa.draw(surface);
}