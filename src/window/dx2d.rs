use crate::{Color, Vec4};
#[cfg(target_os = "windows")]
#[cfg(feature= "window")]
use windows::{
    core::*, Foundation::Numerics::*, Win32::Foundation::*, Win32::Graphics::Direct2D::Common::*,
    Win32::Graphics::Direct2D::*, Win32::UI::WindowsAndMessaging::*,
};
use crate::window::Backend;

pub struct WindowsBackend {
    handle: HWND,

    width: u32,
    height: u32,

    factory: ID2D1Factory1,
    target: ID2D1HwndRenderTarget,
}

impl WindowsBackend {
    /// Create a new backend
    pub fn new(hwnd: HWND) -> Result<Self> {
        let factory = create_factory()?;
        let (target, width, height) = create_target(hwnd, &factory);
        Ok(Self {
            handle: hwnd,
            width,
            height,
            factory,
            target,
        })
    }

    /// Regenerate Target (to accommodate window resizing)
    fn update_target(&mut self) {
        let mut rect = RECT::default();

        unsafe {
            GetClientRect(self.handle, &mut rect);
        }

        let d2d_rect = D2D_SIZE_U {
            width: (rect.right - rect.left) as u32,
            height: (rect.bottom - rect.top) as u32,
        };

        // Re-create Target only when the window size changes
        if self.width != d2d_rect.width || self.height != d2d_rect.height {
            self.width = d2d_rect.width;
            self.height = d2d_rect.height;

            let render_properties = D2D1_RENDER_TARGET_PROPERTIES::default();

            let hwnd_render_properties = D2D1_HWND_RENDER_TARGET_PROPERTIES {
                hwnd: self.handle,
                pixelSize: d2d_rect,
                presentOptions: D2D1_PRESENT_OPTIONS_NONE,
            };

            self.target = unsafe {
                self.factory
                    .CreateHwndRenderTarget(&render_properties, &hwnd_render_properties)
                    .unwrap()
            };
        }
    }
}


fn create_target(hwnd: HWND, factory: &ID2D1Factory1) -> (ID2D1HwndRenderTarget, u32, u32) {
    let mut rect = RECT::default();

    unsafe {
        GetClientRect(hwnd, &mut rect);
    }

    let d2d_rect = D2D_SIZE_U {
        width: (rect.right - rect.left) as u32,
        height: (rect.bottom - rect.top) as u32,
    };

    let render_properties = D2D1_RENDER_TARGET_PROPERTIES::default();

    let hwnd_render_properties = D2D1_HWND_RENDER_TARGET_PROPERTIES {
        hwnd,
        pixelSize: d2d_rect,
        presentOptions: D2D1_PRESENT_OPTIONS_NONE,
    };

    let target = unsafe {
        factory
            .CreateHwndRenderTarget(&render_properties, &hwnd_render_properties)
            .unwrap()
    };
    (
        target,
        (rect.right - rect.left) as u32,
        (rect.bottom - rect.top) as u32,
    )
}

fn create_factory() -> Result<ID2D1Factory1> {
    let mut options = D2D1_FACTORY_OPTIONS::default();

    if cfg!(debug_assertions) {
        options.debugLevel = D2D1_DEBUG_LEVEL_INFORMATION;
    }

    let mut result = None;

    unsafe {
        D2D1CreateFactory(
            D2D1_FACTORY_TYPE_SINGLE_THREADED,
            &ID2D1Factory1::IID,
            &options,
            std::mem::transmute(&mut result),
        )
            .map(|()| result.unwrap())
    }
}

impl Backend for WindowsBackend {
    fn begin(&mut self) {
        unsafe {
            self.update_target();
            self.target.BeginDraw();
        }
    }

    fn clear(&mut self, color: Color) {
        let color = Vec4::from(color);
        unsafe {
            println!("Clear");
            self.target.Clear(&D2D1_COLOR_F { r: color.0 as f32, g: color.1 as f32, b: color.2 as f32, a: color.3 as f32 });
        }
    }

    fn end(&mut self) {
        unsafe {
            self.target
                .EndDraw(std::ptr::null_mut(), std::ptr::null_mut())
                .unwrap();
        }
    }
}