

pub fn roman_to_int(number: &str) -> Option<i32> {

    let mut out: i32 = 0;
    let mut prev = 0;
    for conv in number.chars().map(|c| convert(c)) {
        match conv {
            Some(v) => {
                if v > prev {
                    out -= prev;
                }
                else {
                    out += prev
                }
                prev = v;
            },
            None => return None
        };
    }
    out += prev;
    Some(out)
}


fn convert(token: char) -> Option<i32> {
    match token {
        'M' => Some(1000),
        'D' => Some(500),
        'C' => Some(100),
        'L' => Some(50),
        'X' => Some(10),
        'V' => Some(5),
        'I' => Some(1),
        _ => None
    }
}




#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_roman_to_int() {
        assert_eq!(roman_to_int("IX").unwrap(), 9);
        assert_eq!(roman_to_int("X").unwrap(), 10);
        assert_eq!(roman_to_int("XV").unwrap(), 15);
        assert_eq!(roman_to_int("CCXXX").unwrap(), 230);
        assert_eq!(roman_to_int("MMXXX").unwrap(), 2030);
        assert_eq!(roman_to_int("MCMXCIX").unwrap(), 1999);
    }

    #[test]
    #[should_panic]
    fn test_error_conv() {
        roman_to_int("XXXK").unwrap();
    }


}
