use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement};
use yew::{html, Component, Context, Html, NodeRef};

use azusa::web::WebSurface;
use azusa::Azusa;
use azusa::Color;

pub struct App {
    node_ref: NodeRef
}

impl App {
    fn render_gl(&self,mut azusa: Azusa,surface:&mut WebSurface) {
        azusa.set_source_color(Color::Navy);
        azusa.clear();

        azusa.draw(surface);
    }
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <canvas ref={self.node_ref.clone()} />
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }

        let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();
        let mut surface = WebSurface::new(canvas);
        let mut azusa = Azusa::new();
        self.render_gl(azusa,&mut surface);
    }
}

#[wasm_bindgen(start)]
fn main() {
    yew::Renderer::<App>::new().render();
}

