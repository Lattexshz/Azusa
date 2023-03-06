# Azusa basic
This section describes the basic usage of Azusa.  
To draw with Azusa, you need a drawing context called "Azusa" and a structure called "surface" that specifies the drawing destination.  

Here is a sample code
```rust
use azusa::{Azusa, Color, ImageSurface, ImageType};

fn main() {
    // Create a surface (here the width and height are 100px and the image format is specified as PNG).
    let mut surface = ImageSurface::new(100.0, 100.0, "sample", ImageType::Png);
    // Create a drawing context
    let mut azusa = Azusa::new();

    // Specifies the color to be used when drawing
    azusa.set_source_color(Color::Blue);
    // Fills the surface  
    // (as will be explained later, the clear method is not only responsible for filling the surface, but also for resetting any drawing scheduled for the context)
    azusa.clear();
    azusa.set_source_color(Color::Navy);
    
    // Reserves the drawing of a rectangle
    // Usage: rectangle(x,y,width,height)
    azusa.rectangle(5, 5, 90, 90);

    // Performs the drawing scheduled for the context
    azusa.draw(&mut surface);
}
```

It is drawn like this
![output](https://raw.githubusercontent.com/Lattexshz/Azusa/main/sample.png)