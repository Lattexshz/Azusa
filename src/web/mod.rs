use crate::Surface;

use web_sys::{HtmlCanvasElement,CanvasRenderingContext2d as Context};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use crate::DrawTarget;
use crate::Vec4;

pub struct WebSurface {
    canvas: HtmlCanvasElement,
    ctx: Context
}

impl WebSurface {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        let ctx: Context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        Self {
            canvas,
            ctx
        }
    }
}

impl Surface for WebSurface {
    fn draw(&mut self, ctx: Vec<DrawTarget>) {
        self.ctx.begin_path();
        for i in ctx {
            match i {
                DrawTarget::Clear(color) => {
                    let color:Vec4 = Vec4::from(color);
                    self.ctx.set_fill_style(&JsValue::from_str(&format!("rgba({},{},{},{})",color.0 as u8,color.1 as u8,color.2 as u8,color.3 as u8,)));
                    self.ctx.rect(0.0,0.0,300.0,150.0);
                    self.ctx.fill()
                }
                DrawTarget::Rectangle(color, x, y, width, height) => {
                    let color:Vec4 = Vec4::from(color);
                    self.ctx.set_fill_style(&JsValue::from_str(&format!("rgba({},{},{},{})",color.0 as u8,color.1 as u8,color.2 as u8,color.3 as u8,)));
                    self.ctx.rect(x as f64,y as f64,width as f64,height as f64);
                    self.ctx.fill()
                }
            }
        }
    }

    fn get_client_size(&self) -> (u32, u32) {
        (0,0)
    }
}