use std::fmt::Display;

use ansi_colours::rgb_from_ansi256;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Color {
    None,
    Byte(u8),
    Full(u8, u8, u8),
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::None => f.write_str(""),
            Color::Byte(n) => {
                let (r, g, b) = rgb_from_ansi256(*n);
                write!(f, "#{:02X}{:02X}{:02X}", r, g, b)
            }
            Color::Full(r, g, b) => write!(f, "#{:02X}{:02X}{:02X}", r, g, b),
            Color::Black => f.write_str("black"),
            Color::Red => f.write_str("red"),
            Color::Green => f.write_str("green"),
            Color::Yellow => f.write_str("yellow"),
            Color::Blue => f.write_str("blue"),
            Color::Magenta => f.write_str("magenta"),
            Color::Cyan => f.write_str("cyan"),
            Color::White => f.write_str("white"),
        }
    }
}
