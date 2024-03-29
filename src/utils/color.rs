use std::collections::HashMap;
use std::ops::{Range, Index};
use hex as hex_utils;

#[derive(Debug, Default, Clone, Copy)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8
}

impl Color {
    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Color {
        Color {
            red: red,
            green: green,
            blue: blue,
            alpha: 255
        }
    }

    pub fn from_hex(hex: &str) -> Color {
        let mut hex_str = String::from(hex);

        // todo: support for 3 characters long color hexes (#123 -> #112233)

        if hex_str.len() == 7 && hex_str.starts_with('#') {
            hex_str = hex_str.index(Range { start: 1, end: 7 }).to_string();
        }

        if hex_str.len() == 6 {
            // todo: this can surely be optimized
            let rr = hex_utils::decode(hex_str[0..2].to_string()).ok().unwrap()[0];
            let gg = hex_utils::decode(hex_str[2..4].to_string()).ok().unwrap()[0];
            let bb = hex_utils::decode(hex_str[4..6].to_string()).ok().unwrap()[0];

            return Color {
                red: rr,
                green: gg,
                blue: bb,
                alpha: 255
            }
        }

        Color::default()
    }
}

pub fn get_predefined_colors() -> HashMap<&'static str, Color> {
    let mut map: HashMap<&str, Color> = HashMap::new();

    map.insert("black", Color::from_hex("000000"));
    map.insert("white", Color::from_hex("ffffff"));
    map.insert("red", Color::from_hex("ff0000"));
    map.insert("green", Color::from_hex("00ff00"));
    map.insert("blue", Color::from_hex("0000ff"));
    map.insert("transparent", Color { red: 0, green: 0, blue: 0, alpha: 0 });

    return map;
}

/// Attempts to transform provided string into a color.
/// Currently only supports transforming RGB hex color codes (#ff0000) and returning predefined colors.
/// If given string could not be transformed, black will be returned.
pub fn code_to_color(code: &str) -> Color {
    if code.starts_with('#') {
        return Color::from_hex(code);
    }

    if let Some(predefined_color) = get_predefined_colors().get(code) {
        return predefined_color.clone();
    }

    return Color {
        red: 0,
        green: 0,
        blue: 0,
        alpha: 255
    };
}