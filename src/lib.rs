// needed for the html crate
#![recursion_limit = "512"]

use error::AnsiError;
use html::{content::Article, inline_text::Span, HtmlElement};
use itertools::Itertools;
use state::AnsiState;

mod color;
mod error;
mod state;
mod sub_parsers;

pub struct Parser {
    ansi_chain: AnsiChain,
    current: AnsiState,
}

pub type AnsiChain = Vec<(AnsiState, String)>;

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
    ) -> Result<(), AnsiError> {
        let mut chain = Vec::new();
        loop {
            // get the text till the next escape code
            let part: String = characters.take_while(|&c| c != '\x1b').collect();
            if part.len() > 0 {
                chain.push((self.current.clone(), part))
            }

            // parse the escape code
            match self.current.parse_ansi_code(characters) {
                Ok(()) => {
                    println!("current post parse = {:?}", self.current);
                }
                Err(ansi_error) => {
                    if characters.next().is_some() {
                        return Err(ansi_error);
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

pub(crate) struct Formatter {}

impl Formatter {
    pub(crate) fn format_chain(chain: AnsiChain) -> Article {
        let mut art = Article::builder();
        for (state, text) in chain {
            if state != AnsiState::default() {
                let mut span = Span::builder();
                span.text(text);
                span.style(state.to_style());
                art.push(span.build());
            } else {
                art.text(text);
            }
        }
        art.build()
    }
}

pub fn convert<T: Iterator<Item = char> + Clone>(
    characters: &mut T,
) -> Result<impl HtmlElement, AnsiError> {
    let mut parser = Parser::default();
    parser.parse_ansi_text(characters)?;
    Ok(Formatter::format_chain(parser.ansi_chain))
}

#[cfg(test)]
mod tests {
    use super::*;
    use color::Color;
    use state::{Blink, Intensity, InvertColors, Italics, Spacing, StrikeThrough, Underline};
    use std::iter::zip;

    #[test]
    fn parse_text_with_ansi() {
        let text = "This is a \
            \x1b[32mpiece of text with escape codes like and \
            \x1b[33m\x1b[5m , it should parse without errors etc.\
            \x1b[0mthis should \
            \x1b[0mbe resetted again\
            \x1b[38;2;22;99;199m and this is some blue'ish";
        let mut parser = Parser::default();
        assert_eq!(parser.parse_ansi_text(&mut text.chars()), Ok(()));
        let correct = vec![
            (AnsiState::default(), "This is a ".to_string()),
            (
                AnsiState::new(
                    Color::None,
                    Color::Green,
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
                    Color::Yellow,
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

    #[test]
    fn make_html_from_chain() {
        let chain = vec![
            (AnsiState::default(), "This is default text".to_string()),
            (
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
                    Spacing::Proportional,
                ),
                "and this text is bold".to_string(),
            ),
        ];
        let correct =
            "<article>This is default text<span style=\"font-weight:bold;\">and this text is bold</span></article>";
        assert_eq!(Formatter::format_chain(chain).to_string(), correct);
    }

    #[test]
    fn convert_ansi_to_html() {
        let ansi = "I'll start with some normal text, \x1b[32and then some green \x1b[1that's also bold\x1b[2and some that's faint\x1b[0";
        let correct_html = r#""#;
    }
}
