#[macro_use]
extern crate log;

use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufWriter;

#[cfg(feature = "window")]
pub mod window;

#[cfg(feature = "web")]
pub mod web;

pub mod font;

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
    Maroon,
}

impl From<Color> for Vec4 {
    fn from(value: Color) -> Self {
        match value {
            Color::White => Vec4(255.0, 255.0, 255.0, 255.0),
            Color::Olive => Vec4(128.0, 128.0, 0.0, 255.0),
            Color::Yellow => Vec4(255.0, 255.0, 0.0, 255.0),
            Color::Fuchsia => Vec4(255.0, 0.0, 255.0, 255.0),
            Color::Silver => Vec4(192.0, 192.0, 192.0, 192.0),
            Color::Aqua => Vec4(0.0, 255.0, 255.0, 255.0),
            Color::Lime => Vec4(0.0, 255.0, 0.0, 255.0),
            Color::Red => Vec4(255.0, 0.0, 0.0, 255.0),
            Color::Gray => Vec4(128.0, 128.0, 128.0, 255.0),
            Color::Blue => Vec4(0.0, 0.0, 255.0, 255.0),
            Color::Green => Vec4(0.0, 128.0, 0.0, 255.0),
            Color::Purple => Vec4(128.0, 0.0, 128.0, 255.0),
            Color::Black => Vec4(0.0, 0.0, 0.0, 255.0),
            Color::Navy => Vec4(0.0, 0.0, 128.0, 255.0),
            Color::Teal => Vec4(0.0, 128.0, 128.0, 255.0),
            Color::Maroon => Vec4(128.0, 0.0, 0.0, 255.0),
        }
    }
}

#[derive(Clone,PartialEq,Debug)]
pub struct UString {
    data: Vec<u16>
}

impl UString {
    pub fn new(string: &str) -> Self {
        Self {
            data: string.encode_utf16().chain(std::iter::once(0)).collect::<Vec<u16>>()
        }
    }
}

impl From<String> for UString {
    fn from(value: String) -> Self {
        UString::new(&value)
    }
}

impl Display for UString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",String::from_utf16(self.data.as_slice()).unwrap())
    }
}

#[derive(Clone,Debug,PartialEq)]
pub struct FontInfo(pub(crate) u32,pub(crate) bool,pub(crate) bool);

impl FontInfo {
    pub fn new(px:u32,is_italic: bool,is_under_line: bool) -> Self {
        Self {
            0:px,
            1: is_italic,
            2: is_under_line
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Vec4(f64, f64, f64, f64);

#[derive(Clone, Debug, PartialEq)]
pub enum DrawTarget {
    /// Clear(Color)
    Clear(Color),
    /// FillRectangle(Color,BorderColor,x,y,width,height
    FillRectangle(Color, Color, u32, u32, u32, u32),
    /// DrawRectangle(Color,x,y,width,height,thickness)
    DrawRectangle(Color, u32, u32, u32, u32, u32),
    /// DrawText(Color,x,y,width,height,Text)
    DrawText(Color,FontInfo,u32,u32,u32,u32,UString)
}

pub trait Surface {
    fn draw(&mut self, ctx: Vec<DrawTarget>);
    /// Get surface size
    fn get_client_size(&self) -> (u32, u32);
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImageType {
    #[cfg(feature = "png")]
    Png,
    None,
}

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

impl Surface for ImageSurface<'_> {
    fn draw(&mut self, ctx: Vec<DrawTarget>) {
        match self.image_type {
            #[cfg(feature = "png")]
            ImageType::Png => {
                // Initialize png encoder

                let mut png = immo::png::Png::new(self.width as u32, self.height as u32);

                let path = format!("{}.png", self.name);
                let file = File::create(path).unwrap();
                let w = &mut BufWriter::new(file);

                let mut encoder = png::Encoder::new(w, self.width as u32, self.height as u32); // Width is 2 pixels and height is 1.
                encoder.set_color(png::ColorType::Rgba);
                encoder.set_depth(png::BitDepth::Eight);

                let mut writer = encoder.write_header().unwrap();

                for i in ctx {
                    match i {
                        DrawTarget::Clear(color) => {
                            let color = Vec4::from(color);
                            png.clear((color.0 as u8, color.1 as u8, color.2 as u8, color.3 as u8));
                        }
                        DrawTarget::FillRectangle(color, border_color, x, y, width, height) => {
                            let color = Vec4::from(color);
                            let border_color = Vec4::from(border_color);

                            match png.draw_rectangle(
                                x,
                                y,
                                width,
                                height,
                                1,
                                (
                                    border_color.0 as u8,
                                    border_color.1 as u8,
                                    border_color.2 as u8,
                                    border_color.3 as u8,
                                ),
                            ) {
                                Ok(_) => {}
                                Err(e) => {
                                    error!("{}", e);
                                }
                            }

                            match png.fill_rectangle(
                                x + 1,
                                y + 1,
                                width - 2,
                                height - 2,
                                (color.0 as u8, color.1 as u8, color.2 as u8, color.3 as u8),
                            ) {
                                Ok(_) => {}
                                Err(e) => {
                                    error!("{}", e);
                                }
                            }
                        }

                        DrawTarget::DrawRectangle(color, thickness, x, y, width, height) => {
                            let color = Vec4::from(color);
                            match png.draw_rectangle(
                                x,
                                y,
                                width,
                                height,
                                thickness,
                                (color.0 as u8, color.1 as u8, color.2 as u8, color.3 as u8),
                            ) {
                                Ok(_) => {}
                                Err(e) => {
                                    error!("{}", e);
                                }
                            }
                        }
                        DrawTarget::DrawText(color,info,x,y,width,height,string) => {

                        }
                    }
                }

                writer.write_image_data(png.as_slice()).unwrap();
            }
            ImageType::None => {}
        }
    }

    fn get_client_size(&self) -> (u32, u32) {
        (self.width as u32, self.height as u32)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Azusa {
    ctx: Vec<DrawTarget>,
    ctx_color: Color,
    ctx_border_color: Color,

    ctx_x: u32,
    ctx_y: u32,
}

impl Azusa {
    pub fn new() -> Self {
        info!("Azusa context has been created");
        Self {
            ctx: vec![],
            ctx_color: Color::Black,
            ctx_border_color: Color::Black,
            ctx_x: 0,
            ctx_y: 0,
        }
    }

    /// Retrieves the contents of a context
    pub fn get_ctx(&self) -> &[DrawTarget] {
        &self.ctx
    }

    /// Specifies the color to use for the fill
    pub fn set_source_color(&mut self, color: Color) {
        self.ctx_color = color;
    }

    /// Specify border color
    pub fn set_border_color(&mut self, color: Color) {
        self.ctx_border_color = color;
    }

    /// Fills a surface with a specific color and clears the contents of the context.
    pub fn clear(&mut self) {
        self.ctx.clear();
        self.ctx.push(DrawTarget::Clear(self.ctx_color));
    }

    pub fn move_to(&mut self, x: u32, y: u32) {
        self.ctx_x = x;
        self.ctx_y = y;
    }

    /// Reserves the context to fill rectangle
    pub fn fill_rectangle(&mut self, width: u32, height: u32) {
        self.ctx.push(DrawTarget::FillRectangle(
            self.ctx_color,
            self.ctx_border_color,
            self.ctx_x,
            self.ctx_y,
            width,
            height,
        ));
    }

    /// Reserves the context to draw rectangle
    pub fn draw_rectangle(&mut self, thickness: u32, width: u32, height: u32) {
        self.ctx.push(DrawTarget::DrawRectangle(
            self.ctx_color,
            self.ctx_x,
            self.ctx_y,
            thickness,
            width,
            height,
        ));
    }

    /// Reserves the context to write text
    pub fn draw_text(&mut self,width:u32,height:u32,string: UString,info: FontInfo) {
        self.ctx.push(DrawTarget::DrawText(self.ctx_color,info,self.ctx_x,self.ctx_y,width,height,string));
    }

    /// Writes to the surface passed as argument
    pub fn draw<T: Surface>(&self, surface: &mut T) {
        surface.draw(self.ctx.to_vec());
    }
}
