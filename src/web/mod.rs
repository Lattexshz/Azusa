use crate::Surface;

use web_sys::{HtmlCanvasElement,WebGlRenderingContext as GL};
use wasm_bindgen::JsCast;
use crate::DrawTarget;

pub struct WebSurface {
    canvas: HtmlCanvasElement,
    gl: GL
}

impl WebSurface {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        let gl: GL = canvas
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        Self {
            canvas,
            gl
        }
    }
}

impl Surface for WebSurface {
    fn draw(&mut self, ctx: Vec<DrawTarget>) {
        for i in ctx {
            match i {
                DrawTarget::Clear(color) => {

                }
                DrawTarget::Rectangle(color, x, y, width, height) => {

                }
            }
        }
    }

    fn get_client_size(&self) -> (u32, u32) {
        (0,0)
    }
}