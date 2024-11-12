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

#[derive(Debug, PartialEq)]
enum Intensity {
    Normal,
    Bold,
    Faint,
}

#[derive(Debug, PartialEq)]
enum Blink {
    None,
    Fast,
    Slow,
}

#[derive(Debug, PartialEq)]
enum Underline {
    None,
    Single,
    Double,
}

#[derive(Debug, PartialEq)]
enum Spacing {
    Proportional,
    Monospace,
}

#[derive(Debug, PartialEq)]
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

fn parse_number(part: &mut impl Iterator<Item = u8>) -> (Result<u8, ()>, u32) {
    part.fold((Ok(0), 0), |(total, length), char| match total {
        Ok(total) => {
            if (char >= 0x30 && char < 0x3a) && (total < 25 || (total == 25 && char <= 0x35)) {
                (Ok(total * 10 + (char - 0x30)), length + 1)
            } else {
                (Err(()), length + 1)
            }
        }
        Err(()) => (Err(()), length + 1),
    })
}

fn parse_color_code(part: &mut impl Iterator<Item = char>) -> Result<Color, ()> {
    // match `;(2|5);`
    if part.next() != Some(';') {
        return Err(());
    }
    let selector = part.next();
    if part.next() != Some(';') {
        return Err(());
    }
    match selector {
        Some('5') => {
            let (n, length) = parse_number(&mut part.take_while(|&p| p != 'm').map(|p| p as u8));
            if length > 3 {
                return Err(());
            };
            match n {
                Ok(0) => Ok(Color::Zero),
                Err(()) => Err(()),
                _ => todo!("waa"),
            }
        }
        Some('2') => {
            let color: Vec<char> = part.take_while(|&p| p != 'm').take(11).collect();

            let splits: Vec<_> = color.split(|&byte| byte == ';').collect();
            if splits.len() != 3 {
                return Err(());
            }

            let cparts: Vec<Result<u8, ()>> = splits
                .into_iter()
                .map(|split| {
                    let (total, length) = parse_number(&mut split.iter().map(|&p| p as u8));
                    if length > 3 {
                        return Err(());
                    }
                    total
                })
                .collect();
            println!("cparts = {:?}", cparts);

            Ok(Color::Full(cparts[0]?, cparts[1]?, cparts[2]?))
        }
        _ => Err(()),
    }
}

impl AnsiState {
    fn parse_ansi_code(&mut self, characters: &mut impl Iterator<Item = char>) -> Result<(), ()> {
        if characters.next() != Some('[') {
            return Err(());
        }
        match parse_number(
            &mut characters
                .take_while(|&c| (c as u8) >= 0x30 && (c as u8) < 0x3a)
                .map(|c| c as u8),
        )
        .0?
        {
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
            23 => self.italic = false,
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
            38 => self.text_color = parse_color_code(characters)?,
            39 => self.text_color = Color::None,
            40 => self.background_color = Color::Zero,
            41 => self.background_color = Color::One,
            42 => self.background_color = Color::Two,
            43 => self.background_color = Color::Three,
            44 => self.background_color = Color::Four,
            45 => self.background_color = Color::Five,
            46 => self.background_color = Color::Six,
            47 => self.background_color = Color::Seven,
            48 => self.background_color = parse_color_code(characters)?,
            49 => self.background_color = Color::None,
            50 => self.spacing = Spacing::Monospace,
            51 => todo!("framed?"),
            52 => todo!("encircled?"),
            53 => todo!("overlined"),
            54 => todo!("neither framed or encircled"),
            55 => todo!("not overlined"),
            58 => self.underline_color = parse_color_code(characters)?,
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
    fn full_color_from_extra_escape_part(#[case] r: u8, #[case] g: u8, #[case] b: u8) {
        let result = parse_color_code(&mut format!(";2;{r};{g};{b}m").chars());
        assert_eq!(result, Ok(Color::Full(r, g, b)));
    }

    #[rstest]
    #[case(";3;0;0;0m")]
    #[case(";2;256;0;0m")]
    #[case(";2;000;0000;128m")]
    #[case(";2;0255;0128;0001m")]
    #[case(";2;1;128;1;100m")]
    #[case(";2;011;300m")]
    #[case(";5;0112m")]
    #[case(";5;1;1m")]
    fn color_from_invalid_errors(#[case] str: &str) {
        let result = parse_color_code(&mut str.chars());
        assert_eq!(result, Err(()))
    }

    #[test]
    fn parse_ansi_codes() {
        let mut state = AnsiState::default();
        assert_eq!(state.parse_ansi_code(&mut "[1m".chars()), Ok(()));
        assert_eq!(
            state,
            AnsiState {
                background_color: Color::None,
                text_color: Color::None,
                underline_color: Color::None,
                intensity: Intensity::Bold,
                italic: false,
                underline: Underline::None,
                blink: Blink::None,
                invert_colors: false,
                strikethrough: false,
                spacing: Spacing::Proportional
            },
            "We are testing the bold code"
        );
        assert_eq!(state.parse_ansi_code(&mut "[2m".chars()), Ok(()));
        assert_eq!(
            state,
            AnsiState {
                background_color: Color::None,
                text_color: Color::None,
                underline_color: Color::None,
                intensity: Intensity::Faint,
                italic: false,
                underline: Underline::None,
                blink: Blink::None,
                invert_colors: false,
                strikethrough: false,
                spacing: Spacing::Proportional
            },
            "We are testing the faint code"
        );
        assert_eq!(state.parse_ansi_code(&mut "[3m".chars()), Ok(()));
        assert_eq!(
            state,
            AnsiState {
                background_color: Color::None,
                text_color: Color::None,
                underline_color: Color::None,
                intensity: Intensity::Faint,
                italic: true,
                underline: Underline::None,
                blink: Blink::None,
                invert_colors: false,
                strikethrough: false,
                spacing: Spacing::Proportional
            },
            "We are testing the italic code"
        );
        assert_eq!(state.parse_ansi_code(&mut "[4m".chars()), Ok(()));
        assert_eq!(
            state,
            AnsiState {
                background_color: Color::None,
                text_color: Color::None,
                underline_color: Color::None,
                intensity: Intensity::Faint,
                italic: true,
                underline: Underline::Single,
                blink: Blink::None,
                invert_colors: false,
                strikethrough: false,
                spacing: Spacing::Proportional
            },
            "We are testing the underline code"
        );
        assert_eq!(state.parse_ansi_code(&mut "[5m".chars()), Ok(()));
        assert_eq!(
            state,
            AnsiState {
                background_color: Color::None,
                text_color: Color::None,
                underline_color: Color::None,
                intensity: Intensity::Faint,
                italic: true,
                underline: Underline::Single,
                blink: Blink::Slow,
                invert_colors: false,
                strikethrough: false,
                spacing: Spacing::Proportional
            },
            "We are testing the slow blink code"
        );
        assert_eq!(state.parse_ansi_code(&mut "[6m".chars()), Ok(()));
        assert_eq!(
            state,
            AnsiState {
                background_color: Color::None,
                text_color: Color::None,
                underline_color: Color::None,
                intensity: Intensity::Faint,
                italic: true,
                underline: Underline::Single,
                blink: Blink::Fast,
                invert_colors: false,
                strikethrough: false,
                spacing: Spacing::Proportional
            },
            "We are testing the fast blink code"
        );
        assert_eq!(state.parse_ansi_code(&mut "[7m".chars()), Ok(()));
        assert_eq!(
            state,
            AnsiState {
                background_color: Color::None,
                text_color: Color::None,
                underline_color: Color::None,
                intensity: Intensity::Faint,
                italic: true,
                underline: Underline::Single,
                blink: Blink::Fast,
                invert_colors: true,
                strikethrough: false,
                spacing: Spacing::Proportional
            },
            "We are testing the invert code"
        );
        assert_eq!(state.parse_ansi_code(&mut "[9m".chars()), Ok(()));
        assert_eq!(
            state,
            AnsiState {
                background_color: Color::None,
                text_color: Color::None,
                underline_color: Color::None,
                intensity: Intensity::Faint,
                italic: true,
                underline: Underline::Single,
                blink: Blink::Fast,
                invert_colors: true,
                strikethrough: true,
                spacing: Spacing::Proportional
            },
            "We are testing the strikethrough code"
        );
        assert_eq!(state.parse_ansi_code(&mut "[21m".chars()), Ok(()));
        assert_eq!(
            state,
            AnsiState {
                background_color: Color::None,
                text_color: Color::None,
                underline_color: Color::None,
                intensity: Intensity::Faint,
                italic: true,
                underline: Underline::Double,
                blink: Blink::Fast,
                invert_colors: true,
                strikethrough: true,
                spacing: Spacing::Proportional
            },
            "We are testing the double underline code"
        );
        assert_eq!(state.parse_ansi_code(&mut "[22m".chars()), Ok(()));
        assert_eq!(
            state,
            AnsiState {
                background_color: Color::None,
                text_color: Color::None,
                underline_color: Color::None,
                intensity: Intensity::Normal,
                italic: true,
                underline: Underline::Double,
                blink: Blink::Fast,
                invert_colors: true,
                strikethrough: true,
                spacing: Spacing::Proportional
            },
            "We are testing the reset intensity code"
        );
        assert_eq!(state.parse_ansi_code(&mut "[23m".chars()), Ok(()));
        assert_eq!(
            state,
            AnsiState {
                background_color: Color::None,
                text_color: Color::None,
                underline_color: Color::None,
                intensity: Intensity::Normal,
                italic: false,
                underline: Underline::Double,
                blink: Blink::Fast,
                invert_colors: true,
                strikethrough: true,
                spacing: Spacing::Proportional
            },
            "We are testing the reset italic code"
        );
        assert_eq!(state.parse_ansi_code(&mut "[24m".chars()), Ok(()));
        assert_eq!(
            state,
            AnsiState {
                background_color: Color::None,
                text_color: Color::None,
                underline_color: Color::None,
                intensity: Intensity::Normal,
                italic: false,
                underline: Underline::None,
                blink: Blink::Fast,
                invert_colors: true,
                strikethrough: true,
                spacing: Spacing::Proportional
            },
            "We are testing the reset underline code"
        );
        // TODO: test the rest of the codes
        println!("testing code 0");
        assert_eq!(state.parse_ansi_code(&mut "[0m".chars()), Ok(()));
        assert_eq!(
            state,
            AnsiState::default(),
            "We are testing the reset all code"
        );
    }
}
