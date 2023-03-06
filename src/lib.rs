#[cfg(feature = "window")]
pub mod window;

use std::fs::File;
use std::io::BufWriter;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    White,
    Olive,
    Yellow,
    Fuchsia,
    Silver,
    Aqua,
    Lime,
    Red,
    Gray,
    Blue,
    Green,
    Purple,
    Black,
    Navy,
    Teal,
    Maroon
}

impl From<Color> for Vec4 {
    fn from(value: Color) -> Self {
        match value {
            Color::White => Vec4(255.0,255.0,255.0,255.0),
            Color::Olive => Vec4(128.0,128.0,0.0,255.0),
            Color::Yellow => Vec4(255.0,255.0,0.0,255.0),
            Color::Fuchsia => Vec4(255.0,0.0,255.0,255.0),
            Color::Silver => Vec4(192.0,192.0,192.0,192.0),
            Color::Aqua => Vec4(0.0,255.0,255.0,255.0),
            Color::Lime => Vec4(0.0,255.0,0.0,255.0),
            Color::Red => Vec4(255.0,0.0,0.0,255.0),
            Color::Gray => Vec4(128.0,128.0,128.0,255.0),
            Color::Blue => Vec4(0.0,0.0,255.0,255.0),
            Color::Green => Vec4(0.0,128.0,0.0,255.0),
            Color::Purple => Vec4(128.0,0.0,128.0,255.0),
            Color::Black => Vec4(0.0,0.0,0.0,255.0),
            Color::Navy => Vec4(0.0,0.0,128.0,255.0),
            Color::Teal => Vec4(0.0,128.0,128.0,255.0),
            Color::Maroon => Vec4(128.0,0.0,0.0,255.0)
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

    pub fn resize(&mut self,width:f64,height:f64) {
        self.width = width;
        self.height = height;
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
                        DrawTarget::Rectangle(color,x,y,width,height) => {

                            if data.len() == 0 {
                                for _ in 0..(self.width as u32*self.height as u32) {
                                    data.push(0);
                                    data.push(0);
                                    data.push(0);
                                    data.push(0);
                                }
                            }


                            let color = Vec4::from(color);

                            let mut xc = 0;
                            let mut yc = 1;
                            let mut count = 0;
                            let mut rgba = 3;

                            for i in 0..(self.width as u32*self.height as u32)*4 {
                                count += 1;
                                xc += 1;
                                if xc/4 >= x && xc/4 <= (x+width)-1 {
                                    if yc >= y+1 && yc <= (y+height) {
                                        match rgba {
                                            0 => {
                                                data[i as usize] = color.0 as u8;
                                                rgba = 1;
                                            }
                                            1 => {
                                                data[i as usize] = color.1 as u8;
                                                rgba = 2;
                                            }
                                            2 => {
                                                data[i as usize] = color.2 as u8;;
                                                rgba = 3;
                                            }
                                            3 => {
                                                data[i as usize] = color.3 as u8;
                                                rgba = 0;
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                                
                                if xc as f64 == self.width*4.0 {
                                    yc += 1;
                                    xc = 0;
                                    rgba = 3;
                                }

                            }
                        }
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
            ctx_color: Color::Black,
        }
    }

    pub fn set_source_color(&mut self,color: Color) {
        self.ctx_color = color;
    }

    pub fn clear(&mut self) {
        self.ctx.clear();
        self.ctx.push(DrawTarget::Clear(self.ctx_color));
    }

    pub fn rectangle(&mut self,x:u32,y:u32,width:u32,height:u32) {
        self.ctx.push(DrawTarget::Rectangle(self.ctx_color,x,y,width,height));
    }

    pub fn draw<T:TSurface>(&self,surface:&mut T) {
        surface.draw(self.ctx.to_vec());
    }
}