# Azusa's architecture
The context system is used for drawing management in Azusa.  
Here we will read the actual code and explore how Azusa works!  

Let's take a look at the drawing context code right away  
```rust
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DrawTarget {
    Clear(Color),
    Rectangle(Color, u32, u32, u32, u32),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Azusa {
    ctx: Vec<DrawTarget>,
    ctx_color: Color,
}

impl Azusa {
    pub fn new() -> Self {
        Self {
            ctx: vec![],
            ctx_color: Color::Black,
        }
    }

    pub fn set_source_color(&mut self, color: Color) {
        self.ctx_color = color;
    }

    pub fn clear(&mut self) {
        self.ctx.clear();
        self.ctx.push(DrawTarget::Clear(self.ctx_color));
    }

    pub fn rectangle(&mut self, x: u32, y: u32, width: u32, height: u32) {
        self.ctx
            .push(DrawTarget::Rectangle(self.ctx_color, x, y, width, height));
    }

    pub fn draw<T: TSurface>(&self, surface: &mut T) {
        surface.draw(self.ctx.to_vec());
    }
}
```

As you can see, the contents of Azusa only contain the vector of DrawTarget and the color currently used for output.   
At the same time, we can see that the clear and rectangle methods store the DrawTarget in ctx.  
Now let's look at the draw method  
You can see that the draw method takes a surface struct that implements TSurface and passes ctx to the draw method defined in TSurface  

## What does Surface's draw method do?
Let's look at the draw method of ImageSurface  
```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImageSurface<'a> {
    width: f64,
    height: f64,
    name: &'a str,
    image_type: ImageType,
}

impl<'a> ImageSurface<'a> {
    pub fn new(width: f64, height: f64, name: &'a str, image_type: ImageType) -> Self {
        Self {
            width,
            height,
            name,
            image_type,
        }
    }

    pub fn resize(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
    }
}

impl TSurface for ImageSurface<'_> {
    fn draw(&mut self, ctx: Vec<DrawTarget>) {
        match self.image_type {
            #[cfg(feature = "png")]
            ImageType::Png => {
                for i in ctx {
                    match i {
                            DrawTarget::Clear(color) => {
                                // Clear process
                            }
                        
                            DrawTarget::Rectangle(color, x, y, width, height) => {
                                // Rectangle drawing process
                            }
                        }
                    }
                }
            ImageType::None => {}
        }
    }
}
```
Surface's draw method would get DrawTarget from the context, and the drawing process would be based on that value.  
Next, we will explain how to draw each surface  

## ImageSurface
Output to PNG file creates an array for PNG data based on the DrawTarget received from the context and outputs it  

## WindowSurface
### Windows
GDI+ is used for drawing on Windows