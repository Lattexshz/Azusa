use std::ffi::{c_int, c_void};
use std::ptr::{null, null_mut};
use winapi::shared::windef::{HDC, HWND};
use winapi::um::wingdi::{BitBlt, CreateCompatibleDC, DC_BRUSH, DC_PEN, GetStockObject, Rectangle, RGB, SelectObject, SetDCBrushColor, SetDCPenColor, SwapBuffers};
use winapi::um::winuser::{BeginPaint, EndPaint, GetDC, GetSystemMetrics, LPPAINTSTRUCT, PAINTSTRUCT, ReleaseDC, SendMessageA, SM_CXSCREEN, SM_CYSCREEN, WM_ERASEBKGND};
use crate::window::Backend;
use crate::{Color, Vec4};

pub struct GDIBackend {
    hwnd: HWND,
    hdc: HDC,
    ps: LPPAINTSTRUCT
}

impl GDIBackend {
    /// Create a new backend
    pub fn new(hwnd: *mut c_void) -> Self {
        unsafe {
            let i = gdiplus_sys2::GdiplusStartup(null_mut(),null_mut(),null_mut());
            println!("{}",i);
        }
        Self {
            hwnd: hwnd as HWND,
            hdc: 0 as HDC,
            ps: null_mut()
        }
    }

    /// Regenerate Target (to accommodate window resizing)
    fn update_target(&mut self) {

    }
}

impl Backend for GDIBackend {
    fn begin(&mut self) {

        unsafe {
          self.hdc = BeginPaint(self.hwnd,self.ps);
            self.hdc = CreateCompatibleDC(self.hdc);
        }
    }

    fn clear(&mut self, color: Color) {
        let color = Vec4::from(color);

        let color = RGB(color.0 as u8,color.1 as u8,color.2 as u8);
        unsafe {
            self.hdc = GetDC(self.hwnd);
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
            let color = Vec4::from(color);
            let color = RGB(color.0 as u8,color.1 as u8,color.2 as u8);

            SetDCBrushColor(self.hdc, color);
            SetDCPenColor(self.hdc, color);

            SelectObject(self.hdc, GetStockObject(DC_PEN as c_int));
            SelectObject(self.hdc, GetStockObject(DC_BRUSH as c_int));
            Rectangle(self.hdc,10,5,100,100);

            //Rectangle(self.hdc, x as i32, (x + width) as i32, y as i32, (y + height) as i32);
        }
    }

    fn end(&mut self) {
        unsafe {
            // Release hdc
            SendMessageA(self.hwnd, WM_ERASEBKGND, 1,1);
            EndPaint(self.hwnd, self.ps);
        }
    }
}
