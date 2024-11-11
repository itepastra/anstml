use std::{
    io::{BufRead, Read},
    str::Bytes,
};

use html::{content::Article, inline_text::Italic};

#[derive(Debug, PartialEq)]
enum Color {
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

enum Intensity {
    Normal,
    Bold,
    Faint,
}

enum Blink {
    None,
    Fast,
    Slow,
}

enum Underline {
    None,
    Single,
    Double,
}

enum Spacing {
    Proportional,
    Monospace,
}

struct AnsiState {
    background_color: Color,
    text_color: Color,
    underline_color: Color,
    intensity: Intensity,
    italic: bool,
    underline: Underline,
    blink: Blink,
    invert_colors: bool,
    strikethrough: bool,
    spacing: Spacing,
}

impl Default for AnsiState {
    fn default() -> Self {
        AnsiState {
            background_color: Color::None,
            text_color: Color::None,
            underline_color: Color::None,
            invert_colors: false,
            italic: false,
            underline: Underline::None,
            strikethrough: false,
            intensity: Intensity::Normal,
            blink: Blink::None,
            spacing: Spacing::Proportional,
        }
    }
}

fn mk_byte_from_bytes(bytes: Vec<u8>) -> u8 {
    bytes.iter().fold(0, |b, c| b + (c - 0x30))
}

fn parse_color_code(part: &[u8]) -> Result<Color, ()> {
    if part.starts_with(b";5;") {
        let n = part
            .iter()
            .skip(3)
            .take_while(|&p| *p != b'm')
            .take(3)
            .fold(0, |total, char| total * 10 + (char - 0x30));
        match n {
            0 => Ok(Color::Zero),
            _ => todo!("waa"),
        }
    } else if part.starts_with(b";2;") {
        println!("starts with `;2;`");
        let color: Vec<u8> = part
            .iter()
            .skip(3)
            .map(|&p| p)
            .take_while(|&p| p != b'm')
            .collect();
        println!("color = {:?}", color);

        let splits: Vec<_> = color.split(|&byte| byte == b';').collect();
        println!("splits = {:?}", splits);

        let cparts: Vec<u8> = splits
            .into_iter()
            .map(|split| {
                split
                    .iter()
                    .fold(0, |total, char| total * 10 + (char - 0x30))
            })
            .collect();
        println!("cparts = {:?}", cparts);

        Ok(Color::Full(cparts[0], cparts[1], cparts[2]))
    } else {
        Err(())
    }
}

impl AnsiState {
    fn parse_ansi_code(&mut self, next_part: &str) -> Result<(), ()> {
        let bytes = next_part.as_bytes();
        match bytes[0] {
            0 => {
                self.background_color = Color::None;
                self.text_color = Color::None;
                self.intensity = Intensity::Normal;
                self.italic = false;
                self.underline = Underline::None;
                self.blink = Blink::None;
                self.invert_colors = false;
                self.strikethrough = false;
            }
            1 => self.intensity = Intensity::Bold,
            2 => self.intensity = Intensity::Faint,
            3 => self.italic = true,
            4 => self.underline = Underline::Single,
            5 => self.blink = Blink::Slow,
            6 => self.blink = Blink::Fast,
            7 => self.invert_colors = true,
            8 => todo!("Conceal or hide"),
            9 => self.strikethrough = true,
            10..20 => todo!("fonts"),
            20 => todo!("Fraktur???"),
            21 => self.underline = Underline::Double,
            22 => self.intensity = Intensity::Normal,
            23 => {
                self.italic = false;
                todo!("disable 'blackletter'")
            }
            24 => self.underline = Underline::None,
            25 => self.blink = Blink::None,
            26 => self.spacing = Spacing::Proportional,
            27 => self.invert_colors = false,
            28 => todo!("reveal (undo 8)"),
            29 => self.strikethrough = false,
            30 => self.text_color = Color::Zero,
            31 => self.text_color = Color::One,
            32 => self.text_color = Color::Two,
            33 => self.text_color = Color::Three,
            34 => self.text_color = Color::Four,
            35 => self.text_color = Color::Five,
            36 => self.text_color = Color::Six,
            37 => self.text_color = Color::Seven,
            38 => todo!("set foreground color by peeking forward"),
            39 => self.text_color = Color::None,
            40 => self.background_color = Color::Zero,
            41 => self.background_color = Color::One,
            42 => self.background_color = Color::Two,
            43 => self.background_color = Color::Three,
            44 => self.background_color = Color::Four,
            45 => self.background_color = Color::Five,
            46 => self.background_color = Color::Six,
            47 => self.background_color = Color::Seven,
            48 => todo!("set background color by peeking forward"),
            49 => self.background_color = Color::None,
            50 => self.spacing = Spacing::Monospace,
            51 => todo!("framed?"),
            52 => todo!("encircled?"),
            53 => todo!("overlined"),
            54 => todo!("neither framed or encircled"),
            55 => todo!("not overlined"),
            58 => todo!("set underline color by peeking forward"),
            59 => self.underline_color = Color::None,
            60 => todo!("Ideogram underline or right side line"),
            61 => todo!("Ideogram double underline, or double line on the right side"),
            62 => todo!("Ideogram overline or left side line"),
            63 => todo!("Ideogram double overline, or double line on the left side"),
            64 => todo!("Ideogram stress marking"),
            65 => todo!("No ideogram attributes"),
            73 => todo!("superscript"),
            74 => todo!("subscript"),
            75 => todo!("neither super nor subscript"),
            _ => return Err(()),
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    // extrema
    #[case(0, 0, 0)]
    #[case(0, 0, 255)]
    #[case(0, 255, 255)]
    #[case(0, 255, 0)]
    #[case(255, 255, 0)]
    #[case(255, 255, 255)]
    #[case(255, 0, 255)]
    #[case(255, 0, 0)]
    // correct order
    #[case(1, 2, 3)]
    #[case(10, 20, 30)]
    #[case(100, 200, 255)]
    #[case(200, 100, 50)]
    #[case(29, 99, 91)]
    // weird notation
    #[case(003, 022, 000)]
    fn test_color_from_escape(#[case] r: u8, #[case] g: u8, #[case] b: u8) {
        let result = parse_color_code(format!(";2;{r};{g};{b}m").as_bytes());
        assert_eq!(result, Ok(Color::Full(r, g, b)));
    }
}
