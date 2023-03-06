#[cfg(target_os = "windows")]
mod dx2d;

#[cfg(feature = "window")]
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

use crate::{Color, DrawTarget, TSurface};

#[cfg(feature = "window")]
#[cfg(target_os = "windows")]
use windows::Win32::Foundation::HWND;

pub trait Backend {
    fn begin(&mut self);
    fn clear(&mut self, color: Color);
    fn rectangle(&mut self, color: Color, x: f32, y: f32, width: f32, height: f32);
    fn end(&mut self);
}

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
            RawWindowHandle::Win32(handle) => {
                dx2d::WindowsBackend::new(HWND(handle.hwnd as isize)).unwrap()
            }
            RawWindowHandle::WinRt(_) => return Err(()),
            RawWindowHandle::Web(_) => return Err(()),
            RawWindowHandle::AndroidNdk(_) => return Err(()),
            RawWindowHandle::Haiku(_) => return Err(()),
            _ => return Err(()),
        });

        Ok(Self { backend })
    }
}

impl TSurface for WindowSurface {
    fn draw(&mut self, ctx: Vec<DrawTarget>) {
        self.backend.begin();
        for i in ctx {
            match i {
                DrawTarget::Clear(color) => {
                    self.backend.clear(color);
                }
                DrawTarget::Rectangle(color, x, y, width, height) => {
                    self.backend
                        .rectangle(color, x as f32, y as f32, width as f32, height as f32);
                }
            }
        }
        self.backend.end();
    }
}
