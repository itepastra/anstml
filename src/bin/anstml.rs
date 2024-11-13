#![recursion_limit = "512"]
use std::io::{stderr, stdin, stdout, Error, Read, Write};

use anstml::{convert, error::AnsiError};

#[derive(Debug)]
enum AnsTmlError {
    AnsiError,
    IOError,
}

impl From<std::io::Error> for AnsTmlError {
    fn from(_value: Error) -> Self {
        Self::IOError
    }
}

impl From<AnsiError> for AnsTmlError {
    fn from(_value: AnsiError) -> Self {
        Self::AnsiError
    }
}

fn main() -> Result<(), AnsTmlError> {
    let mut stdin = stdin();
    let mut ansi_buffer = Vec::new();
    let amt = stdin.read_to_end(&mut ansi_buffer)?;
    writeln!(stderr(), "read {} bytes from stdin", amt)?;

    let html = convert(&mut ansi_buffer.into_iter().map(|c| c as char))?;

    write!(stdout(), "{}", html.to_string())?;

    Ok(())
}
