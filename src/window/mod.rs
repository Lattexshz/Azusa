#[cfg(target_os = "windows")]
mod gdi;

use crate::{Color, DrawTarget, FontInfo, Surface, UString};
#[cfg(feature = "window")]
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

pub trait Backend {
    fn begin(&mut self);
    fn clear(&mut self, color: Color);
    fn fill_rectangle(
        &mut self,
        color: Color,
        border_color: Color,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    );
    fn draw_rectangle(
        &mut self,
        color: Color,
        thickness: u32,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    );
    fn draw_text(&mut self,color: Color,string: UString,info:FontInfo,x:u32,y:u32,width:u32,height:u32);
    fn end(&mut self);

    fn get_client_size(&self) -> (u32, u32);
}

#[derive(Debug)]
pub struct WindowSurface {
    backend: Box<dyn Backend>,
}

impl WindowSurface {
    pub fn new(handle: &impl HasRawWindowHandle) -> Result<Self, ()> {
        let handle = handle.raw_window_handle();
        let backend = Box::new(match handle {
            RawWindowHandle::UiKit(_) => return Err(()),
            RawWindowHandle::AppKit(_) => return Err(()),
            RawWindowHandle::Orbital(_) => return Err(()),
            RawWindowHandle::Xlib(_) => return Err(()),
            RawWindowHandle::Xcb(_) => return Err(()),
            RawWindowHandle::Wayland(_) => return Err(()),
            RawWindowHandle::Drm(_) => return Err(()),
            RawWindowHandle::Gbm(_) => return Err(()),
            #[cfg(target_os = "windows")]
            RawWindowHandle::Win32(handle) => gdi::GDIBackend::new(handle.hwnd),
            RawWindowHandle::WinRt(_) => return Err(()),
            RawWindowHandle::Web(_) => return Err(()),
            RawWindowHandle::AndroidNdk(_) => return Err(()),
            RawWindowHandle::Haiku(_) => return Err(()),
            _ => return Err(()),
        });

        Ok(Self { backend })
    }
}

impl Surface for WindowSurface {
    fn draw(&mut self, ctx: Vec<DrawTarget>) {
        self.backend.begin();
        for i in ctx {
            match i {
                DrawTarget::Clear(color) => {
                    self.backend.clear(color);
                }
                DrawTarget::FillRectangle(color, border_color, x, y, width, height) => {
                    self.backend.fill_rectangle(
                        color,
                        border_color,
                        x as f32,
                        y as f32,
                        width as f32,
                        height as f32,
                    );
                }
                DrawTarget::DrawRectangle(color, thickness, x, y, width, height) => {
                    self.backend.draw_rectangle(
                        color,
                        thickness,
                        x as f32,
                        y as f32,
                        width as f32,
                        height as f32,
                    );
                }
                DrawTarget::DrawText(color,info,x,y,width,height,string) => {
                    self.backend.draw_text(color,string,info,x,y,width,height);
                }
            }
        }
        self.backend.end();
    }

    fn get_client_size(&self) -> (u32, u32) {
        self.backend.get_client_size()
    }
}
