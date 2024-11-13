use std::fmt::Display;

use ansi_colours::rgb_from_ansi256;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Color {
    None,
    Byte(u8),
    Full(u8, u8, u8),
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
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
            Color::Zero => f.write_str("black"),
            Color::One => f.write_str("red"),
            Color::Two => f.write_str("green"),
            Color::Three => f.write_str("yellow"),
            Color::Four => f.write_str("blue"),
            Color::Five => f.write_str("magenta"),
            Color::Six => f.write_str("cyan"),
            Color::Seven => f.write_str("white"),
        }
    }
}
