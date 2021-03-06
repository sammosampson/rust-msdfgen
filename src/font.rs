use euclid::Rect;
use crate::path::*;

pub trait Font {
    fn get_glyph(&self, character: char) -> (u32, Vec<Contour>, Rect<f32>);
}
