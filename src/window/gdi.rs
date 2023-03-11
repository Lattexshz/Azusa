use crate::window::Backend;
use crate::{Color, Vec4};


use std::ffi::{c_int, c_void};

use winapi::shared::windef::{
    DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2, HBITMAP, HDC, HGDIOBJ, HWND, RECT,
};
use winapi::um::wingdi::{
    BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC,
    DeleteObject, GetStockObject, Rectangle, SelectObject, SetDCBrushColor,
    SetDCPenColor, DC_BRUSH, DC_PEN, RGB, SRCCOPY,
};
use winapi::um::winuser::{
    GetClientRect, GetDC, ReleaseDC,
    SetProcessDpiAwarenessContext, LPPAINTSTRUCT,
};

pub struct GDIBackend {
    hwnd: HWND,
    hdc: HDC,
    dc: HDC,

    bitmap: HBITMAP,
    obmp: HGDIOBJ,

    rect: RECT,
}

impl GDIBackend {
    /// Create a new backend
    pub fn new(hwnd: *mut c_void) -> Self {
        unsafe {
            SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
        }

        Self {
            hwnd: hwnd as HWND,
            hdc: 0 as HDC,
            dc: 0 as HDC,
            bitmap: 0 as HBITMAP,
            obmp: 0 as HGDIOBJ,
            rect: RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            },
        }
    }

    /// Regenerate Target (to accommodate window resizing)
    #[inline]
    fn set_color(&mut self, color: Vec4) {
        unsafe {
            SetDCBrushColor(self.hdc, RGB(color.0 as u8, color.1 as u8, color.2 as u8));
            SetDCPenColor(self.hdc, RGB(color.0 as u8, color.1 as u8, color.2 as u8));

            SelectObject(self.hdc, GetStockObject(DC_PEN as c_int));
            SelectObject(self.hdc, GetStockObject(DC_BRUSH as c_int));
        }
    }
}

impl Backend for GDIBackend {
    fn begin(&mut self) {
        unsafe {
            GetClientRect(self.hwnd, &mut self.rect);

            self.dc = GetDC(self.hwnd);
            self.hdc = CreateCompatibleDC(self.dc);
            self.bitmap = CreateCompatibleBitmap(self.dc, self.rect.right, self.rect.bottom);
            self.obmp = SelectObject(self.hdc, self.bitmap as HGDIOBJ);
        }
    }

    fn clear(&mut self, color: Color) {
        let color = Vec4::from(color);

        unsafe {
            self.set_color(color);
            Rectangle(
                self.hdc,
                self.rect.left,
                self.rect.top,
                self.rect.right,
                self.rect.bottom,
            );
        }
    }

    fn rectangle(&mut self, color: Color, x: f32, y: f32, width: f32, height: f32) {
        let color = Vec4::from(color);
        unsafe {
            let rect = RECT {
                left: x as i32,
                right: width as i32,
                top: y as i32,
                bottom: height as i32,
            };

            self.set_color(color);
            Rectangle(self.hdc, rect.left, rect.top, rect.right, rect.bottom);
        }
    }

    fn end(&mut self) {
        unsafe {
            BitBlt(
                self.dc,
                self.rect.left,
                self.rect.top,
                self.rect.right,
                self.rect.bottom,
                self.hdc,
                0,
                0,
                SRCCOPY,
            );
            SelectObject(self.hdc, self.obmp);

            DeleteDC(self.hdc);
            DeleteObject(self.bitmap as HGDIOBJ);

            ReleaseDC(self.hwnd, self.dc);
        }
    }
}
