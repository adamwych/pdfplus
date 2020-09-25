use font_kit;
use crate::utils;

#[derive(Debug)]
pub struct Font {
    internal: font_kit::font::Font,
}

impl Font {
    pub fn get_text_bounding_box(&self, text: &String, font_size: f64, include_max_descent: bool) -> utils::FRect {
        let mut width: f64 = 0.0;
        let mut height: f64 = 0.0;
        let mut min_y: f64 = 0.0;

        for ch in text.chars() {
            let char_bounds = self.get_character_bounding_box(ch, font_size);
            width += char_bounds.width;
            height = height.max(char_bounds.height);
            min_y = min_y.min(char_bounds.y);
        }

        if include_max_descent {
            let metrics = self.internal.metrics();
            let to_px = font_size / metrics.units_per_em as f64;
            height += metrics.descent.abs() as f64 * to_px;
        }

        utils::FRect {
            x: 0.0,
            y: min_y.abs(),
            width: width,
            height: height
        }
    }

    pub fn get_character_bounding_box(&self, character: char, font_size: f64) -> utils::FRect {
        let metrics = self.internal.metrics();
        let to_px = font_size / metrics.units_per_em as f64;

        if let Some(glyph) = self.internal.glyph_for_char(character) {
            if let Ok(bounds) = self.internal.typographic_bounds(glyph) {
                let advance = self.internal.advance(glyph).unwrap();
                let advance_x_px = advance.x() as f64 * to_px;
                let min_y_px = bounds.min_y() as f64 * to_px;

                return utils::FRect {
                    x: 0.0,
                    y: min_y_px,
                    width: advance_x_px,
                    height: bounds.height() as f64 * to_px
                }
            }
        }
        
        utils::FRect {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0
        }
    }

    pub fn wrap(font: font_kit::font::Font) -> Self {
        Self {
            internal: font
        }
    }
}