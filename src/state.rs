use std::fmt::Display;

use crate::{
    color::Color,
    error::AnsiError,
    sub_parsers::{parse_color_code, parse_number},
};

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Intensity {
    Normal,
    Bold,
    Faint,
}

impl Display for Intensity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Intensity::Normal => f.write_str(""),
            Intensity::Bold => f.write_str("bold"),
            Intensity::Faint => f.write_str("light"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Blink {
    None,
    Fast,
    Slow,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Underline {
    None,
    Single,
    Double,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Spacing {
    Proportional,
    Monospace,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum InvertColors {
    Yes,
    No,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum StrikeThrough {
    Yes,
    No,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Italics {
    Yes,
    No,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AnsiState {
    background_color: Color,
    text_color: Color,
    underline_color: Color,
    intensity: Intensity,
    italic: Italics,
    underline: Underline,
    blink: Blink,
    invert_colors: InvertColors,
    strikethrough: StrikeThrough,
    spacing: Spacing,
}

impl Default for AnsiState {
    fn default() -> Self {
        AnsiState {
            background_color: Color::None,
            text_color: Color::None,
            underline_color: Color::None,
            invert_colors: InvertColors::No,
            italic: Italics::No,
            underline: Underline::None,
            strikethrough: StrikeThrough::No,
            intensity: Intensity::Normal,
            blink: Blink::None,
            spacing: Spacing::Monospace,
        }
    }
}

impl AnsiState {
    pub(crate) fn to_style(&self) -> String {
        let mut s = String::new();
        if self.background_color != Color::None {
            s.push_str(&format!("background-color:{};", self.background_color));
        }
        if self.text_color != Color::None {
            s.push_str(&format!("color:{};", self.text_color));
        }
        if self.underline != Underline::None || self.strikethrough != StrikeThrough::No {
            if self.underline_color != Color::None {
                s.push_str(&format!("text-decoration-color:{};", self.underline_color))
            }
            let mut lines = Vec::new();
            if self.underline != Underline::None {
                lines.push("underline");
            }
            if self.strikethrough != StrikeThrough::No {
                lines.push("line-through");
            }
            s.push_str(&format!("text-decoration:{};", lines.join(" ")));
        }
        if self.underline == Underline::Double {
            s.push_str("text-decoration-style: double;")
        }
        if self.intensity != Intensity::Normal {
            s.push_str(&format!("font-weight:{};", self.intensity))
        }
        // NOTE: needs the blink animation to be defined in css
        if self.blink != Blink::None {
            let speed = match self.blink {
                Blink::None => unreachable!(),
                Blink::Fast => 0.5,
                Blink::Slow => 1.0,
            };
            s.push_str(&format!("animation: blink {} step-start infinite;", speed))
        }
        if self.spacing != Spacing::Monospace {
            s.push_str("font-family: system-ui;")
        }
        s
    }

    #[allow(unused)]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        background_color: Color,
        text_color: Color,
        underline_color: Color,
        invert: InvertColors,
        italic: Italics,
        underline: Underline,
        strikethrough: StrikeThrough,
        intensity: Intensity,
        blink: Blink,
        spacing: Spacing,
    ) -> AnsiState {
        AnsiState {
            background_color,
            text_color,
            underline_color,
            invert_colors: invert,
            italic,
            underline,
            strikethrough,
            intensity,
            blink,
            spacing,
        }
    }

    pub(crate) fn parse_ansi_code<T: Iterator<Item = char> + Clone>(
        &mut self,
        characters: &mut T,
    ) -> Result<(), AnsiError> {
        if characters.next() != Some('[') {
            return Err(AnsiError::InvalidStartBrace);
        }
        match parse_number(
            &mut characters
                .take_while(|&c| c != 'm' && c != ';')
                .map(|c| c as u8),
        )
        .0?
        {
            0 => {
                self.background_color = Color::None;
                self.text_color = Color::None;
                self.intensity = Intensity::Normal;
                self.italic = Italics::No;
                self.underline = Underline::None;
                self.blink = Blink::None;
                self.invert_colors = InvertColors::No;
                self.strikethrough = StrikeThrough::No;
            }
            1 => self.intensity = Intensity::Bold,
            2 => self.intensity = Intensity::Faint,
            3 => self.italic = Italics::Yes,
            4 => self.underline = Underline::Single,
            5 => self.blink = Blink::Slow,
            6 => self.blink = Blink::Fast,
            7 => self.invert_colors = InvertColors::Yes,
            8 => todo!("Conceal or hide"),
            9 => self.strikethrough = StrikeThrough::Yes,
            10..20 => todo!("fonts"),
            20 => todo!("Fraktur???"),
            21 => self.underline = Underline::Double,
            22 => self.intensity = Intensity::Normal,
            23 => self.italic = Italics::No,
            24 => self.underline = Underline::None,
            25 => self.blink = Blink::None,
            26 => self.spacing = Spacing::Proportional,
            27 => self.invert_colors = InvertColors::No,
            28 => todo!("reveal (undo 8)"),
            29 => self.strikethrough = StrikeThrough::No,
            30 => self.text_color = Color::Black,
            31 => self.text_color = Color::Red,
            32 => self.text_color = Color::Green,
            33 => self.text_color = Color::Yellow,
            34 => self.text_color = Color::Blue,
            35 => self.text_color = Color::Magenta,
            36 => self.text_color = Color::Cyan,
            37 => self.text_color = Color::White,
            38 => self.text_color = parse_color_code(characters)?,
            39 => self.text_color = Color::None,
            40 => self.background_color = Color::Black,
            41 => self.background_color = Color::Red,
            42 => self.background_color = Color::Green,
            43 => self.background_color = Color::Yellow,
            44 => self.background_color = Color::Blue,
            45 => self.background_color = Color::Magenta,
            46 => self.background_color = Color::Cyan,
            47 => self.background_color = Color::White,
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
            _ => return Err(AnsiError::IllegalCommand),
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_ansi_codes() {
        let mut state = AnsiState::default();
        assert_eq!(state.parse_ansi_code(&mut "[1m".chars()), Ok(()));
        assert_eq!(
            state,
            AnsiState::new(
                Color::None,
                Color::None,
                Color::None,
                InvertColors::No,
                Italics::No,
                Underline::None,
                StrikeThrough::No,
                Intensity::Bold,
                Blink::None,
                Spacing::Monospace
            ),
            "We are testing the bold code"
        );
        assert_eq!(state.parse_ansi_code(&mut "[2m".chars()), Ok(()));
        assert_eq!(
            state,
            AnsiState::new(
                Color::None,
                Color::None,
                Color::None,
                InvertColors::No,
                Italics::No,
                Underline::None,
                StrikeThrough::No,
                Intensity::Faint,
                Blink::None,
                Spacing::Monospace
            ),
            "We are testing the faint code"
        );
        assert_eq!(state.parse_ansi_code(&mut "[3m".chars()), Ok(()));
        assert_eq!(
            state,
            AnsiState::new(
                Color::None,
                Color::None,
                Color::None,
                InvertColors::No,
                Italics::Yes,
                Underline::None,
                StrikeThrough::No,
                Intensity::Faint,
                Blink::None,
                Spacing::Monospace
            ),
            "We are testing the italic code"
        );
        assert_eq!(state.parse_ansi_code(&mut "[4m".chars()), Ok(()));
        assert_eq!(
            state,
            AnsiState::new(
                Color::None,
                Color::None,
                Color::None,
                InvertColors::No,
                Italics::Yes,
                Underline::Single,
                StrikeThrough::No,
                Intensity::Faint,
                Blink::None,
                Spacing::Monospace
            ),
            "We are testing the underline code"
        );
        assert_eq!(state.parse_ansi_code(&mut "[5m".chars()), Ok(()));
        assert_eq!(
            state,
            AnsiState::new(
                Color::None,
                Color::None,
                Color::None,
                InvertColors::No,
                Italics::Yes,
                Underline::Single,
                StrikeThrough::No,
                Intensity::Faint,
                Blink::Slow,
                Spacing::Monospace
            ),
            "We are testing the slow blink code"
        );
        assert_eq!(state.parse_ansi_code(&mut "[6m".chars()), Ok(()));
        assert_eq!(
            state,
            AnsiState::new(
                Color::None,
                Color::None,
                Color::None,
                InvertColors::No,
                Italics::Yes,
                Underline::Single,
                StrikeThrough::No,
                Intensity::Faint,
                Blink::Fast,
                Spacing::Monospace
            ),
            "We are testing the fast blink code"
        );
        assert_eq!(state.parse_ansi_code(&mut "[7m".chars()), Ok(()));
        assert_eq!(
            state,
            AnsiState::new(
                Color::None,
                Color::None,
                Color::None,
                InvertColors::Yes,
                Italics::Yes,
                Underline::Single,
                StrikeThrough::No,
                Intensity::Faint,
                Blink::Fast,
                Spacing::Monospace
            ),
            "We are testing the invert code"
        );
        assert_eq!(state.parse_ansi_code(&mut "[9m".chars()), Ok(()));
        assert_eq!(
            state,
            AnsiState::new(
                Color::None,
                Color::None,
                Color::None,
                InvertColors::Yes,
                Italics::Yes,
                Underline::Single,
                StrikeThrough::Yes,
                Intensity::Faint,
                Blink::Fast,
                Spacing::Monospace
            ),
            "We are testing the strikethrough code"
        );
        assert_eq!(state.parse_ansi_code(&mut "[21m".chars()), Ok(()));
        assert_eq!(
            state,
            AnsiState::new(
                Color::None,
                Color::None,
                Color::None,
                InvertColors::Yes,
                Italics::Yes,
                Underline::Double,
                StrikeThrough::Yes,
                Intensity::Faint,
                Blink::Fast,
                Spacing::Monospace
            ),
            "We are testing the double underline code"
        );
        assert_eq!(state.parse_ansi_code(&mut "[22m".chars()), Ok(()));
        assert_eq!(
            state,
            AnsiState::new(
                Color::None,
                Color::None,
                Color::None,
                InvertColors::Yes,
                Italics::Yes,
                Underline::Double,
                StrikeThrough::Yes,
                Intensity::Normal,
                Blink::Fast,
                Spacing::Monospace
            ),
            "We are testing the reset intensity code"
        );
        assert_eq!(state.parse_ansi_code(&mut "[23m".chars()), Ok(()));
        assert_eq!(
            state,
            AnsiState::new(
                Color::None,
                Color::None,
                Color::None,
                InvertColors::Yes,
                Italics::No,
                Underline::Double,
                StrikeThrough::Yes,
                Intensity::Normal,
                Blink::Fast,
                Spacing::Monospace
            ),
            "We are testing the reset italic code"
        );
        assert_eq!(state.parse_ansi_code(&mut "[24m".chars()), Ok(()));
        assert_eq!(
            state,
            AnsiState::new(
                Color::None,
                Color::None,
                Color::None,
                InvertColors::Yes,
                Italics::No,
                Underline::None,
                StrikeThrough::Yes,
                Intensity::Normal,
                Blink::Fast,
                Spacing::Monospace
            ),
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
