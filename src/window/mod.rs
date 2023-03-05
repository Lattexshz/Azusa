use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use crate::{DrawTarget, TSurface};

pub struct WindowSurface<T:HasRawWindowHandle> {
    handle: T
}

impl<T:HasRawWindowHandle> WindowSurface<T> {
    pub fn new(handle: T) -> Self {
        Self {
            handle
        }
    }
}

impl<T:HasRawWindowHandle> TSurface for WindowSurface<T> {
    fn draw(&self, ctx: Vec<DrawTarget>) {
        let raw_window_handle = self.handle.raw_window_handle();

        match raw_window_handle {
            RawWindowHandle::UiKit(_) => {}
            RawWindowHandle::AppKit(_) => {}
            RawWindowHandle::Orbital(_) => {}
            RawWindowHandle::Xlib(_) => {}
            RawWindowHandle::Xcb(_) => {}
            RawWindowHandle::Wayland(_) => {}
            RawWindowHandle::Drm(_) => {}
            RawWindowHandle::Gbm(_) => {}
            RawWindowHandle::Win32(handle) => {
                for i in ctx {
                    match i {
                        DrawTarget::Clear(_) => {}
                        DrawTarget::Rectangle(_, _, _, _) => {}
                    }
                }
            }
            RawWindowHandle::WinRt(_) => {}
            RawWindowHandle::Web(_) => {}
            RawWindowHandle::AndroidNdk(_) => {}
            RawWindowHandle::Haiku(_) => {}
            _ => {}
        }
    }
}