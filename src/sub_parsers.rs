use crate::{color::Color, error::AnsiError};

pub(crate) fn parse_number(part: &mut impl Iterator<Item = u8>) -> (Result<u8, AnsiError>, u32) {
    part.fold((Ok(0), 0), |(total, length), char| match total {
        Ok(total) => {
            if !char.is_ascii_digit() {
                (Err(AnsiError::InvalidFormat), length + 1)
            } else if (0x30..=0x39).contains(&char) && (total < 25 || (total == 25 && char <= 0x35))
            {
                (Ok(total * 10 + (char - 0x30)), length + 1)
            } else {
                (Err(AnsiError::NumberParse), length + 1)
            }
        }
        Err(err) => (Err(err), length + 1),
    })
}

pub(crate) fn parse_color_code(part: &mut impl Iterator<Item = char>) -> Result<Color, AnsiError> {
    // match `(2|5);`
    let selector = part.next();
    if part.next() != Some(';') {
        return Err(AnsiError::InvalidFormat);
    }
    match selector {
        Some('5') => {
            let (n, length) = parse_number(&mut part.take_while(|&p| p != 'm').map(|p| p as u8));
            if length > 3 {
                return Err(AnsiError::TooLong);
            };
            match n {
                Ok(0) => Ok(Color::Black),
                Ok(1) => Ok(Color::Red),
                Ok(2) => Ok(Color::Green),
                Ok(3) => Ok(Color::Yellow),
                Ok(4) => Ok(Color::Blue),
                Ok(5) => Ok(Color::Magenta),
                Ok(6) => Ok(Color::Cyan),
                Ok(7) => Ok(Color::White),
                Ok(n) => Ok(Color::Byte(n)),
                Err(err) => Err(err),
            }
        }
        Some('2') => {
            let color: Vec<char> = part.take_while(|&p| p != 'm').take(11).collect();

            let splits: Vec<_> = color.split(|&byte| byte == ';').collect();
            if splits.len() != 3 {
                return Err(AnsiError::InvalidFormat);
            }

            let cparts: Vec<Result<u8, AnsiError>> = splits
                .into_iter()
                .map(|split| {
                    let (total, length) = parse_number(&mut split.iter().map(|&p| p as u8));
                    if length > 3 {
                        return Err(AnsiError::TooLong);
                    }
                    total
                })
                .collect();
            Ok(Color::Full(cparts[0]?, cparts[1]?, cparts[2]?))
        }
        _ => Err(AnsiError::InvalidFormat),
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
    fn full_color_from_extra_escape_part(#[case] r: u8, #[case] g: u8, #[case] b: u8) {
        let result = parse_color_code(&mut format!("2;{r};{g};{b}m").chars());
        assert_eq!(result, Ok(Color::Full(r, g, b)));
    }

    #[rstest]
    #[case("3;0;0;0m", AnsiError::InvalidFormat)]
    #[case("2;256;0;0m", AnsiError::NumberParse)]
    #[case("2;000;0000;128m", AnsiError::TooLong)]
    #[case("2;0255;0128;0001m", AnsiError::TooLong)]
    #[case("2;1;128;1;100m", AnsiError::InvalidFormat)]
    #[case("2;011;300m", AnsiError::InvalidFormat)]
    #[case("5;0112m", AnsiError::TooLong)]
    #[case("5;1;1m", AnsiError::InvalidFormat)]
    fn color_from_invalid_errors(#[case] str: &str, #[case] error_type: AnsiError) {
        let result = parse_color_code(&mut str.chars());
        assert_eq!(result, Err(error_type))
    }
}
