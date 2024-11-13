use crate::sub_parsers::{parse_color_code, parse_number};
use itertools::Itertools;

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

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Intensity {
    Normal,
    Bold,
    Faint,
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
pub(crate) struct AnsiState {
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
            spacing: Spacing::Proportional,
        }
    }
}

impl AnsiState {
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
    ) -> Result<(), ()> {
        if characters.next() != Some('[') {
            return Err(());
        }
        match parse_number(
            &mut characters
                .take_while_ref(|&c| c.is_ascii_digit())
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

    #[test]
    fn parse_ansi_codes() {
        let mut state = AnsiState::default();
        assert_eq!(state.parse_ansi_code(&mut "[1".chars()), Ok(()));
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
                Spacing::Proportional
            ),
            "We are testing the bold code"
        );
        assert_eq!(state.parse_ansi_code(&mut "[2".chars()), Ok(()));
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
                Spacing::Proportional
            ),
            "We are testing the faint code"
        );
        assert_eq!(state.parse_ansi_code(&mut "[3".chars()), Ok(()));
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
                Spacing::Proportional
            ),
            "We are testing the italic code"
        );
        assert_eq!(state.parse_ansi_code(&mut "[4".chars()), Ok(()));
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
                Spacing::Proportional
            ),
            "We are testing the underline code"
        );
        assert_eq!(state.parse_ansi_code(&mut "[5".chars()), Ok(()));
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
                Spacing::Proportional
            ),
            "We are testing the slow blink code"
        );
        assert_eq!(state.parse_ansi_code(&mut "[6".chars()), Ok(()));
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
                Spacing::Proportional
            ),
            "We are testing the fast blink code"
        );
        assert_eq!(state.parse_ansi_code(&mut "[7".chars()), Ok(()));
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
                Spacing::Proportional
            ),
            "We are testing the invert code"
        );
        assert_eq!(state.parse_ansi_code(&mut "[9".chars()), Ok(()));
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
                Spacing::Proportional
            ),
            "We are testing the strikethrough code"
        );
        assert_eq!(state.parse_ansi_code(&mut "[21".chars()), Ok(()));
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
                Spacing::Proportional
            ),
            "We are testing the double underline code"
        );
        assert_eq!(state.parse_ansi_code(&mut "[22".chars()), Ok(()));
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
                Spacing::Proportional
            ),
            "We are testing the reset intensity code"
        );
        assert_eq!(state.parse_ansi_code(&mut "[23".chars()), Ok(()));
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
                Spacing::Proportional
            ),
            "We are testing the reset italic code"
        );
        assert_eq!(state.parse_ansi_code(&mut "[24".chars()), Ok(()));
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
                Spacing::Proportional
            ),
            "We are testing the reset underline code"
        );
        // TODO: test the rest of the codes
        println!("testing code 0");
        assert_eq!(state.parse_ansi_code(&mut "[0".chars()), Ok(()));
        assert_eq!(
            state,
            AnsiState::default(),
            "We are testing the reset all code"
        );
    }
}
