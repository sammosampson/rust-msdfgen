use freetype::*;
use euclid::*;
use crate::path::*;
use crate::font::*;
use lyon_path::builder::*;
use lyon_path::math::{Point};

pub struct FreeTypeFont {
    face: Face
}

impl FreeTypeFont {
    pub fn arial_font() -> Self {    
        let lib = Library::init().unwrap();
        Self {
            face: lib.new_face("C:/Windows/Fonts/arial.ttf", 0).unwrap()
        }
        //*face.set_char_size(40 * 64, 0, 50, 0).unwrap();
    }
}

impl Font for FreeTypeFont {
    fn get_glyph(&self, character: char) -> (u32, Vec<Contour>, Rect<f32>) {
        let mut collector = PathCollector::new();
        self.face.load_char(character as usize, freetype::face::LoadFlag::NO_SCALE).unwrap();
        let glyph = self.face.glyph();
        let outline = glyph.outline().unwrap();

        for contour in outline.contours_iter() {
            
            let start_point = contour.start();
            collector.move_to(Point::new(start_point.x as f32, start_point.y as f32));
            
            for curve in contour {
                match curve {
                    freetype::outline::Curve::Line(to_point) => {
                        collector.line_to(Point::new(to_point.x as f32, to_point.y as f32));
                    },
                    freetype::outline::Curve::Bezier2(control_point, to_point) => {
                        collector.quadratic_bezier_to(
                            Point::new(control_point.x as f32, control_point.y as f32),
                            Point::new(to_point.x as f32, to_point.y as f32));
                    },
                    freetype::outline::Curve::Bezier3(control_point_1, control_point_2, to_point) => {
                        collector.cubic_bezier_to(
                            Point::new(control_point_1.x as f32, control_point_1.y as f32),
                            Point::new(control_point_2.x as f32, control_point_2.y as f32),
                            Point::new(to_point.x as f32, to_point.y as f32));
                    },
                }
            }
            collector.close();
        }

        let metrics = glyph.metrics();

        (
            self.face.get_char_index(character as usize),
            collector.build(), 
            Rect::new(TypedPoint2D::new(0.0, 0.0), TypedSize2D::new(metrics.width as f32, metrics.height as f32))
        )
    }
}