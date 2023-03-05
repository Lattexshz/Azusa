#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    Blue
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DrawTarget {
    Clear(Color),
    Rectangle(u32,u32,u32,u32)
}

pub trait TSurface {
    fn draw(&self,ctx:Vec<DrawTarget>);
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImageSurface<'a> {
    width: f64,
    height: f64,
    name: &'a str
}

impl<'a> ImageSurface<'a> {
    pub fn new(width:f64,height:f64,name:&'a str) -> Self {
        Self {
            width,
            height,
            name,
        }
    }
}


impl TSurface for ImageSurface<'_> {
    fn draw(&self,ctx: Vec<DrawTarget>) {
        for i in ctx {
            match i {
                DrawTarget::Clear(_) => {}
                DrawTarget::Rectangle(_, _, _, _) => {}
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Azusa {
    ctx: Vec<DrawTarget>
}

impl Azusa {
    pub fn new() -> Self {
        Self {
            ctx: vec![],
        }
    }

    pub fn clear(&mut self) {
        self.ctx.clear();
    }

    pub fn rectangle(&mut self,x:u32,y:u32,width:u32,height:u32) {
        self.ctx.push(DrawTarget::Rectangle(x,y,width,height));
    }

    pub fn len(&self) -> usize {
        self.ctx.len()
    }

    pub fn draw<T:TSurface>(&self,surface:T) {
        surface.draw(self.ctx.to_vec());
    }
}