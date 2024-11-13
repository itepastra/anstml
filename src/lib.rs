use itertools::Itertools;

use state::AnsiState;

mod state;
mod sub_parsers;

pub struct Parser {
    ansi_chain: Vec<(AnsiState, String)>,
    current: AnsiState,
}

impl Default for Parser {
    fn default() -> Self {
        Parser {
            ansi_chain: Vec::new(),
            current: AnsiState::default(),
        }
    }
}

impl Parser {
    pub fn parse_ansi_text<T: Iterator<Item = char> + Clone>(
        &mut self,
        characters: &mut T,
    ) -> Result<(), ()> {
        let mut chain = Vec::new();
        loop {
            // get the text till the next escape code
            let part: String = characters.take_while(|&c| c != '\x1b').collect();
            if part.len() > 0 {
                chain.push((self.current.clone(), part))
            }

            // parse the escape code
            match self.current.parse_ansi_code(characters) {
                Ok(()) => {}
                Err(()) => {
                    if characters.next().is_some() {
                        return Err(());
                    } else {
                        break;
                    }
                }
            }
        }
        // find and fix duplicates
        self.ansi_chain = chain
            .into_iter()
            .coalesce(|(left_state, left_string), (right_state, right_string)| {
                if left_state == right_state {
                    Ok((left_state, left_string + &right_string))
                } else {
                    Err(((left_state, left_string), (right_state, right_string)))
                }
            })
            .collect();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use state::{
        Blink, Color, Intensity, InvertColors, Italics, Spacing, StrikeThrough, Underline,
    };
    use std::iter::zip;

    #[test]
    fn parse_text_with_ansi() {
        let text = "This is a \
            \x1b[32piece of text with escape codes like and \
            \x1b[33\x1b[5 , it should parse without errors etc.\
            \x1b[0this should \
            \x1b[0be resetted again\
            \x1b[38;2;22;99;199m and this is some blue'ish";
        let mut parser = Parser::default();
        assert_eq!(parser.parse_ansi_text(&mut text.chars()), Ok(()));
        let correct = vec![
            (AnsiState::default(), "This is a ".to_string()),
            (
                AnsiState::new(
                    Color::None,
                    Color::Two,
                    Color::None,
                    InvertColors::No,
                    Italics::No,
                    Underline::None,
                    StrikeThrough::No,
                    Intensity::Normal,
                    Blink::None,
                    Spacing::Proportional,
                ),
                "piece of text with escape codes like and ".to_string(),
            ),
            (
                AnsiState::new(
                    Color::None,
                    Color::Three,
                    Color::None,
                    InvertColors::No,
                    Italics::No,
                    Underline::None,
                    StrikeThrough::No,
                    Intensity::Normal,
                    Blink::Slow,
                    Spacing::Proportional,
                ),
                " , it should parse without errors etc.".to_string(),
            ),
            (
                AnsiState::default(),
                "this should be resetted again".to_string(),
            ),
            (
                AnsiState::new(
                    Color::None,
                    Color::Full(22, 99, 199),
                    Color::None,
                    InvertColors::No,
                    Italics::No,
                    Underline::None,
                    StrikeThrough::No,
                    Intensity::Normal,
                    Blink::None,
                    Spacing::Proportional,
                ),
                " and this is some blue'ish".to_string(),
            ),
        ];
        assert_eq!(parser.ansi_chain.len(), correct.len());
        for (correct, actual) in zip(parser.ansi_chain, correct) {
            assert_eq!(actual, correct)
        }
    }
}
