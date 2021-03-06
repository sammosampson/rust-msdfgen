use rust_msdfgen::*;
use rust_msdfgen::font::*;
use rust_msdfgen::freetype::*;
use euclid::*;
use lyon_path::math::{Point};

const SDF_DIMENSION: u32 = 32;

fn main() {
    let font = FreeTypeFont::arial_font();
    let mut font_atlas = FontAtlas::build(SDF_DIMENSION, &font);
    font_atlas.layout_string("Hello!");
}

struct FontAtlasGlyph {
    id: u32,
    /// Where it actually is in the atlas texture
    uv: Rect<f32>,
    /// The font-space rectangle covered by the uv rectangle
    font_units: Rect<f32>,
}

struct FontAtlas<'font, TFont> where TFont: Font {
    font: &'font TFont,
    character_dimensions: u32,
    texture: Vec<Vec<(f32, f32, f32)>>
}

impl<'font, TFont> FontAtlas<'font, TFont>  where TFont: Font {
    fn build(character_dimensions: u32, font: &'font TFont) -> Self {
        let allocated_size = character_dimensions * 16;

        Self {
            font,
            character_dimensions,
            texture: vec!()
        }
    }

    fn layout_string(&mut self, s: &str) {
        for c in s.chars() {
            let information = self.character_information(c);
        }
    }

    fn character_information(&mut self, c: char) -> FontAtlasGlyph {
        const INIT_UV_BORDER: f32 = 0.2;
        const UV_BORDER: f32 = 0.1;
        
        let uv = Rect::new(
            Point::new(INIT_UV_BORDER, INIT_UV_BORDER),
            TypedSize2D::new(1.0 - 2.0 * INIT_UV_BORDER, 1.0 - 2.0 * INIT_UV_BORDER),
        );


        let (glyph_id, contours, font_units) = self.font.get_glyph(c);

        //let (contours, transform) = rescale_contours(contours, font_units, uv);
        let contours = recolor_contours(contours, Angle::degrees(3.0), 1);
        self.texture = compute_msdf(&contours, self.character_dimensions as usize);
        
        FontAtlasGlyph {
            id: glyph_id,
            uv,
            font_units
        }
    }
}