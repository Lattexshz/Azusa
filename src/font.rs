pub struct Font {
    
}

pub enum SurfaceFont {
    ImageSurface,
    WindowSurface(FontPlatform)
}

pub enum FontPlatform {
    Windows(winapi::shared::windef::HFONT)
}