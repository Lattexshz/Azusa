use azusa::{Azusa, Color, ImageSurface, ImageType};

fn main() {
    let mut surface = ImageSurface::new(1280.0, 720.0, "sample", ImageType::Png);
    let mut azusa = Azusa::new();

    azusa.set_source_color(Color::Blue);
    azusa.clear();

    azusa.draw(&mut surface);
}