use azusa::{Azusa, Color, ImageSurface, ImageType};

fn main() {
    let mut surface = ImageSurface::new(100.0, 100.0, "sample", ImageType::Png);
    let mut azusa = Azusa::new();

    azusa.set_source_color(Color::Blue);
    azusa.clear();
    azusa.set_source_color(Color::Navy);
    azusa.rectangle(5, 5, 90, 90);

    azusa.draw(&mut surface);
}
