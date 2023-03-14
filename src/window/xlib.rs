use std::ffi::{c_ulong, CString,c_uint};
use std::ptr::{null, null_mut};
use x11::xlib::{Colormap, Display, GC, XAllocNamedColor, XClearWindow, XColor, XCreateGC, XDefaultColormap, XFlush, XOpenDisplay, XSetBackground,XSetForeground,XGetGeometry,XDefaultScreen,XFillRectangle,XWhitePixel};
use crate::{Color, FontInfo, UString};
use crate::window::Backend;

pub struct XLibBackend {
    display: *mut Display,
    window: c_ulong,
    gc: GC
}

impl XLibBackend {
    pub fn new(window: c_ulong) -> Self {
        unsafe {
            let display = XOpenDisplay(std::ptr::null_mut());
            XClearWindow(display,window);
            let gc = XCreateGC(display,window,0,std::ptr::null_mut());

            Self {
                display,
                window,
                gc
            }
        }
    }
}

impl Backend for XLibBackend {
    fn begin(&mut self) {

    }

    fn clear(&mut self, color: Color) {
        unsafe {
            XSetForeground(self.display, self.gc, get_color_from_rgb(self.display,255,255,255));
            XSetBackground(self.display, self.gc, get_color_from_rgb(self.display,255,255,255));
            let (width,height) = get_xlib_window_size(self.display,self.window);
            XFillRectangle(self.display,self.window,self.gc,0,0,width as c_uint,height as c_uint);
        }
    }

    fn fill_rectangle(&mut self, color: Color, border_color: Color, x: f32, y: f32, width: f32, height: f32) {

    }

    fn draw_rectangle(&mut self, color: Color, thickness: u32, x: f32, y: f32, width: f32, height: f32) {

    }

    fn draw_text(&mut self, color: Color, string: UString, info: FontInfo, x: u32, y: u32, width: u32, height: u32) {

    }

    fn end(&mut self) {
        unsafe {
            XFlush(self.display);
        }
    }

    fn get_client_size(&self) -> (u32, u32) {
        (0,0)
    }
}

#[inline]
pub(crate) unsafe fn get_xlib_window_size(
    display: *mut Display,
    window: c_ulong,
) -> (c_ulong, c_ulong) {
    let mut width = 0;
    let mut height = 0;
    let mut dummy = 0;
    let mut c_int_dummy = 0;
    let mut c_uint_dummy = 0;
    XGetGeometry(
        display,
        window,
        &mut dummy,
        &mut c_int_dummy,
        &mut c_int_dummy,
        &mut width,
        &mut height,
        &mut c_uint_dummy,
        &mut c_uint_dummy,
    );

    (width.into(), height.into())
}

#[inline]
unsafe fn get_color_from_rgb(display: *mut Display,r:u32,g:u32,b:u32) -> c_ulong {
    let cmap = XDefaultColormap(display,0);
    let c0 = null_mut();
    let c1 = null_mut();
    XAllocNamedColor(display,cmap,CString::new("rgb:00/00/FF").unwrap().as_ptr(),c0,c1);
    c1.pixel
}