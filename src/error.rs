#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AnsiError {
    NumberParse,
    InvalidStartBrace,
    InvalidFormat,
    TooLong,
    IllegalCommand,
}
