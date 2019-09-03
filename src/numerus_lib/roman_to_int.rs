
///
/// Convert given roman numeral into an integer.
/// This function accepts, at the same time, both
/// uppercase and lowercase character.
/// 
/// The function fails if any invalid character 
/// is meet.
/// 
/// An error example
/// ```
///     use numerus::roman_to_int;
/// 
///     /*
///         K is not a symbol in the
///         roman numerical system
///     */
///     let wrong_roman = "XCK";
///     match roman_to_int(wrong_roman) {
///         Some(_) => assert!(false),
///         None => assert!(true)
///     }
/// ```
/// 
/// A correct convertion
/// ```
///     use numerus::roman_to_int;
///     
///     let wrong_roman = "MCMLXIX";
///     match roman_to_int(wrong_roman) {
///         Some(arabic) => assert_eq!(1969, arabic),
///         None => assert!(false)
///     }
/// 
/// ```
/// 
pub fn roman_to_int(number: &str) -> Option<i32> {

    let mut out: i32 = 0;
    let mut prev = 0;
    for res in number.chars().map(convert) {
        let v = res?;
        if v > prev {
            out -= prev;
        }
        else {
            out += prev
        }
        prev = v;
    
    }
    out += prev;
    Some(out)
}


fn convert(token: char) -> Option<i32> {
    match token {
        'm' => Some(1000),
        'M' => Some(1000),
        'd' => Some(500),
        'D' => Some(500),
        'c' => Some(100),
        'C' => Some(100),
        'l' => Some(50),
        'L' => Some(50),
        'x' => Some(10),
        'X' => Some(10),
        'v' => Some(5),
        'V' => Some(5),
        'i' => Some(1),
        'I' => Some(1),

        _ => None
    }
}




#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_roman_to_int_uppercase() {
        assert_eq!(roman_to_int("IX").unwrap(), 9);
        assert_eq!(roman_to_int("X").unwrap(), 10);
        assert_eq!(roman_to_int("XV").unwrap(), 15);
        assert_eq!(roman_to_int("CCXXX").unwrap(), 230);
        assert_eq!(roman_to_int("MMXXX").unwrap(), 2030);
        assert_eq!(roman_to_int("MCMXCIX").unwrap(), 1999);
    }

    #[test]
    fn test_roman_to_int_lowercase() {
        assert_eq!(roman_to_int("ix").unwrap(), 9);
        assert_eq!(roman_to_int("x").unwrap(), 10);
        assert_eq!(roman_to_int("xv").unwrap(), 15);
        assert_eq!(roman_to_int("ccxxx").unwrap(), 230);
        assert_eq!(roman_to_int("mmxxx").unwrap(), 2030);
        assert_eq!(roman_to_int("mcmxcix").unwrap(), 1999);
    }


    #[test]
    #[should_panic]
    fn test_error_conv() {
        roman_to_int("XXXK").unwrap();
    }


}
