use std::ffi::{c_int, c_void};
use winapi::shared::windef::{HDC, HWND};
use winapi::um::wingdi::{DC_BRUSH, DC_PEN, GetStockObject, Rectangle, RGB, SelectObject, SetDCBrushColor, SetDCPenColor};
use winapi::um::winuser::{GetDC, GetSystemMetrics, ReleaseDC, SM_CXSCREEN, SM_CYSCREEN};
use crate::window::Backend;
use crate::{Color, Vec4};

pub struct GDIBackend {
    hwnd: HWND,
    hdc: HDC,
}

impl GDIBackend {
    /// Create a new backend
    pub fn new(hwnd: *mut c_void) -> Self {
        Self {
            hwnd: hwnd as HWND,
            hdc: 0 as HDC
        }
    }

    /// Regenerate Target (to accommodate window resizing)
    fn update_target(&mut self) {

    }
}

impl Backend for GDIBackend {
    fn begin(&mut self) {
        unsafe {
            self.hdc = GetDC(self.hwnd);
        }
    }

    fn clear(&mut self, color: Color) {
        let color = Vec4::from(color);

        let color = RGB(color.0 as u8,color.1 as u8,color.2 as u8);
        unsafe {
            SetDCBrushColor(self.hdc, color);
            SetDCPenColor(self.hdc, color);

            SelectObject(self.hdc, GetStockObject(DC_PEN as c_int));
            SelectObject(self.hdc, GetStockObject(DC_BRUSH as c_int));

            Rectangle(
                self.hdc,
                0,
                0,
                GetSystemMetrics(SM_CXSCREEN) as c_int,
                GetSystemMetrics(SM_CYSCREEN) as c_int,
            );


        }
    }

    fn rectangle(&mut self, color: Color, x: f32, y: f32, width: f32, height: f32) {
        unsafe {
            // let color = Vec4::from(color);
            //
            // let hdc = GetDC(self.handle);
            // Rectangle(hdc, x as i32, (x + width) as i32, y as i32, (y + height) as i32);
        }
    }

    fn end(&mut self) {
        unsafe {
            // Release hdc
            ReleaseDC(self.hwnd, self.hdc);
        }
    }
}
