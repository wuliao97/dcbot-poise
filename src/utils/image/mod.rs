use std::path::Path;
use image::{self, DynamicImage, imageops::overlay, ImageResult};
use image::imageops::FilterType;
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};


pub enum BaseImageType {
    // Physical,
    // Fire,
    // Ice,
    // Lightning,
    // Wind,
    // Quantum,
    // Imaginary,
    Default,
}
impl BaseImageType {
    pub fn which_one(&self) -> String {
        let r#type = match self {
            // BaseImageType::Physical => {}
            // BaseImageType::Fire => {}
            // BaseImageType::Ice => {}
            // BaseImageType::Lightning => {}
            // BaseImageType::Wind => {}
            // BaseImageType::Quantum => {}
            // BaseImageType::Imaginary => {}
            _ => "./resources/image/material/background_1.png",
        };
        r#type.to_string()
    }
}



pub struct Drawing {
    base: DynamicImage,
    font: Font<'static>
}

impl Drawing {
    #[inline]
    #[must_use]
    pub fn new<T: AsRef<Path>>(path: T, font: Option<FontType>) -> Self {
        let base = image::open(path).unwrap();
        let font_type = match font {
            None => FontType::StarRail.which(),
            Some(font) => font.which()
        };
        let font = Font::try_from_vec(font_type).unwrap();

        Self { base, font }
    }

    #[inline]
    pub fn trans_base(&self) -> DynamicImage {
        self.base.clone()
    }

    #[inline]
    pub async fn paste(&mut self, material: DynamicImage, (x, y): (i64, i64), mask_img: Option<DynamicImage>) -> &mut Self {
        overlay(&mut self.base, &material, x, y);

        if let Some(mask) = mask_img {
            overlay(&mut self.base, &mask, 0, 0);
        };

        self
    }

    #[inline]
    pub async fn draw(
        &mut self,
        text: &str,
        (x, y): (i32, i32),
        scale: Option<Scale>,
        font: Option<&Font<'_>>) -> &mut Self {
        let scale = scale.unwrap_or(Scale { x: 25.0, y: 25.0 });
        let font = font.unwrap_or(&self.font);
        draw_text_mut(&mut self.base, image::Rgba([255u8, 255u8, 255u8, 255u8]), x, y, scale, font, text);
        self
    }

    #[inline]
    pub fn save<T: AsRef<Path>>(&self, path: T) -> ImageResult<()> {
        self.base.save(path)
    }

    #[inline]
    pub fn resize(mut self, (width, height): (u32, u32), filter_type: FilterType) -> Self {
        self.base = self.base.resize(width, height, filter_type);
        self
    }
}

pub enum FontType {
    StarRail,
    Genshin,

}
impl FontType {
    pub fn which(&self) -> Vec<u8> {
        match self {
            FontType::StarRail => Vec::from(include_bytes!("../../../resources/fonts/star-rail.ttf") as &[u8]),
            FontType::Genshin => Vec::from(include_bytes!("../../../resources/fonts/genshin.ttf"))
        }
    }
}