use std::ffi::{c_int, c_void};
use std::ptr::{null, null_mut};
use gdiplus::{color, GdiPlus, Graphics, Pen, SolidBrush};
use gdiplus::enums::SmoothingMode;
use winapi::shared::windef::{DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2, HDC, HWND, RECT};
use winapi::um::wingdi::{BitBlt, CreateCompatibleDC, DC_BRUSH, DC_PEN, GetStockObject, Rectangle, RGB, SelectObject, SetDCBrushColor, SetDCPenColor, SwapBuffers};
use winapi::um::winuser::{BeginPaint, EndPaint, GetClientRect, GetDC, GetSystemMetrics, LPPAINTSTRUCT, PAINTSTRUCT, ReleaseDC, SendMessageA, SetProcessDpiAwarenessContext, SM_CXSCREEN, SM_CYSCREEN, WM_ERASEBKGND};
use crate::window::Backend;
use crate::{Color, Vec4};

pub struct GDIBackend {
    hwnd: HWND,
    hdc: HDC,
    ps: LPPAINTSTRUCT,
    gdiplus: GdiPlus,
    graphics: Option<Graphics>
}

impl GDIBackend {
    /// Create a new backend
    pub fn new(hwnd: *mut c_void) -> Self {


        unsafe {
            SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
        }

        let gdiplus = GdiPlus::startup(None, None).unwrap();

        Self {
            hwnd: hwnd as HWND,
            hdc: 0 as HDC,
            ps: unsafe { std::mem::zeroed() },
            gdiplus,
            graphics: None
        }
    }

    /// Regenerate Target (to accommodate window resizing)
    fn update_target(&mut self) {

    }
}

impl Backend for GDIBackend {
    fn begin(&mut self) {
        unsafe {
            let mut ps = std::mem::zeroed();
            self.hdc = BeginPaint(self.hwnd, &mut ps);

            let mut rect = std::mem::zeroed();
            GetClientRect(self.hwnd, &mut rect);

            rect.left -= 1;
            rect.top -= 1;
            rect.right += 1;
            rect.bottom += 1;

            (|| -> gdiplus::Result<()> {
                let mut graphics = Graphics::from_hdc(self.hdc)?;

                graphics.set_smoothing_mode(SmoothingMode::AntiAlias)?;

                self.graphics = Some(graphics);
                Ok(())

            })().unwrap();
        }
    }

    fn clear(&mut self, color: Color) {
        let color = Vec4::from(color);
        let mut graphics = self.graphics.as_mut().unwrap();

        unsafe {
            let mut rect = std::mem::zeroed();
            GetClientRect(self.hwnd, &mut rect);

            rect.left -= 1;
            rect.top -= 1;
            rect.right += 1;
            rect.bottom += 1;
            graphics
                .with_brush(&mut SolidBrush::new(&gdiplus::Color::from((color.3 as u8,color.0 as u8,color.1 as u8,color.2 as u8))).unwrap())
                .fill_rectangle(
                    (rect.left as _, rect.top as _),
                    rect.right as _,
                    rect.bottom as _,
                ).unwrap();
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
            EndPaint(self.hwnd, self.ps);
        }
    }
}

impl Drop for GDIBackend {
    fn drop(&mut self) {
        self.gdiplus.shutdown();
    }
}
