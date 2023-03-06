#[cfg(feature = "window")]
pub mod window;

use std::fs::File;
use std::io::BufWriter;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    Blue
}

impl From<Color> for Vec4 {
    fn from(value: Color) -> Self {
        match value {
            Color::Blue => Vec4(0.0,0.0,255.0,255.0)
        }
    }
}

struct Vec4(f64,f64,f64,f64);

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DrawTarget {
    Clear(Color),
    Rectangle(Color,u32,u32,u32,u32)
}

pub trait TSurface {
    fn draw(&mut self,ctx:Vec<DrawTarget>);
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImageType {
    #[cfg(feature = "png")]
    Png,
    None
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImageSurface<'a> {
    width: f64,
    height: f64,
    name: &'a str,
    image_type: ImageType
}

impl<'a> ImageSurface<'a> {
    pub fn new(width:f64,height:f64,name:&'a str,image_type: ImageType) -> Self {
        Self {
            width,
            height,
            name,
            image_type
        }
    }
}


impl TSurface for ImageSurface<'_> {
    fn draw(&mut self,ctx: Vec<DrawTarget>) {
        match self.image_type {
            #[cfg(feature = "png")]
            ImageType::Png => {

                // Initialize png encoder

                let path = format!("{}.png",self.name);
                let file = File::create(path).unwrap();
                let w = &mut BufWriter::new(file);

                let mut encoder = png::Encoder::new(w, self.width as u32, self.height as u32); // Width is 2 pixels and height is 1.
                encoder.set_color(png::ColorType::Rgba);
                encoder.set_depth(png::BitDepth::Eight);
                // Adding text chunks to the header
                encoder
                    .add_text_chunk(
                        "Testing tEXt".to_string(),
                        "This is a tEXt chunk that will appear before the IDAT chunks.".to_string(),
                    )
                    .unwrap();
                encoder
                    .add_ztxt_chunk(
                        "Testing zTXt".to_string(),
                        "This is a zTXt chunk that is compressed in the png file.".to_string(),
                    )
                    .unwrap();
                encoder
                    .add_itxt_chunk(
                        "Testing iTXt".to_string(),
                        "iTXt chunks support all of UTF8. Example: हिंदी.".to_string(),
                    )
                    .unwrap();

                let mut writer = encoder.write_header().unwrap();

                let mut data:Vec<u8> = vec![];

                for i in ctx {
                    match i {
                        DrawTarget::Clear(color) => {
                            let color = Vec4::from(color);
                            for _ in 0..(self.width as u32*self.height as u32) {
                                data.push(color.0 as u8);
                                data.push(color.1 as u8);
                                data.push(color.2 as u8);
                                data.push(color.3 as u8);
                            }
                        }
                        DrawTarget::Rectangle(_, _, _, _) => {}
                    }
                }

                writer.write_image_data(&data).unwrap();
            }
            ImageType::None => {}
        }

    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Azusa {
    ctx: Vec<DrawTarget>,
    ctx_color: Color
}

impl Azusa {
    pub fn new() -> Self {
        Self {
            ctx: vec![],
        }
    }

    pub fn set_source_color(&mut self,color: Color) {
        self.color = color;
    }

    pub fn clear(&mut self) {
        self.ctx.clear();
        self.ctx.push(DrawTarget::Clear(self.color));
    }

    pub fn rectangle(&mut self,x:u32,y:u32,width:u32,height:u32) {
        self.ctx.push(DrawTarget::Rectangle(self.color,x,y,width,height));
    }

    pub fn draw<T:TSurface>(&self,surface:&mut T) {
        surface.draw(self.ctx.to_vec());
    }
}