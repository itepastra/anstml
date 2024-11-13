use crate::state::Color;

pub(crate) fn parse_number(part: &mut impl Iterator<Item = u8>) -> (Result<u8, ()>, u32) {
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

pub(crate) fn parse_color_code(part: &mut impl Iterator<Item = char>) -> Result<Color, ()> {
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
            Ok(Color::Full(cparts[0]?, cparts[1]?, cparts[2]?))
        }
        _ => Err(()),
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
}
