use crate::Surface;

pub struct FontInfo {}

pub struct Font {
    sfont: FontKind,
    info: FontInfo
}

impl Font {
    pub fn new<T: Surface(surface: &T,info: FontInfo) -> Self {
        Self {
            sfont: surface.get_font_kind(),
            info
        }
    }
}

pub enum FontKind {
    ImageSurface,
    WindowSurface(FontPlatform)
}

pub enum FontPlatform {
    Windows(winapi::shared::windef::HFONT)
}