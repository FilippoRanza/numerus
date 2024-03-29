
use std::ops::Add;


static _UPPER_UNITS: &[&'static str] =  &["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X"];
static _UPPER_TENS : &[&'static str] = &["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC", "C"];
static _UPPER_HUNDREDS : &[&'static str] = &["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM", "M"];
static _UPPER_CONVERT: &[&[&'static str]] = &[_UPPER_HUNDREDS, _UPPER_TENS, _UPPER_UNITS];

static _LOWER_UNITS: &[&'static str] =  &["", "i", "ii", "iii", "iv", "v", "vi", "vii", "viii", "ix", "x"];
static _LOWER_TENS : &[&'static str] = &["", "x", "xx", "xxx", "xl", "l", "lx", "lxx", "lxxx", "xc", "c"];
static _LOWER_HUNDREDS : &[&'static str] = &["", "c", "cc", "ccc", "cd", "d", "dc", "dcc", "dccc", "cm", "m"];
static _LOWER_CONVERT: &[&[&'static str]] = &[_LOWER_HUNDREDS, _LOWER_TENS, _LOWER_UNITS];

/// 
/// Convert the given numer into a lowercase roman 
/// numeral. This function fails if the given numer 
/// is not representable in the roman numerical system
/// (i.e. number <= 0).
///
/// An error example
/// ```
///     use numerus::int_to_roman_lower;
///     
///     /*
///         It is not possible to represent 
///         a negative numer in the roman 
///         numerical system
///     */
///     let wrong_number = -45;
///     match int_to_roman_lower(wrong_number) {
///         Some(_) => assert!(false),
///         None => assert!(true)
///     };
/// ```
/// 
/// A correct convertion
/// ```
///     use numerus::int_to_roman_lower;
/// 
///     let convertible_number = 39;
///     match int_to_roman_lower(convertible_number) {
///         Some(roman) => assert_eq!(roman, "xxxix"),
///         None => assert!(false)
///     };  
/// 
/// ``` 
pub fn int_to_roman_lower(number: i32) -> Option<String> {
    convertion_engine(number, _LOWER_CONVERT, 'm')
}



/// 
/// Convert the given numer into an uppercase roman 
/// numeral. This function fails if the given numer 
/// is not representable in the roman numerical system
/// (i.e. number <= 0).
/// 
/// An error example
/// ```
///     use numerus::int_to_roman_upper;
///     
///     /*
///       there's no representation for 0 
///       in the roman numerical system
///     */
///     let wrong_number = 0;
///     match int_to_roman_upper(wrong_number) {
///         Some(_) => assert!(false),
///         None => assert!(true)
///     };
/// ```
/// 
/// A correct convertion
/// ```
///     use numerus::int_to_roman_upper;
/// 
///     let convertible_number = 101;
///     match int_to_roman_upper(convertible_number) {
///         Some(roman) => assert_eq!(roman, "CI"),
///         None => assert!(false)
///     };
/// 
/// 
/// ``` 
pub fn int_to_roman_upper(number: i32) -> Option<String> {
    convertion_engine(number, _UPPER_CONVERT, 'M')
}


fn convertion_engine(mut number: i32, convertion: &[&[&'static str]], t: char) -> Option<String> {
    if number < 1 {
        return None;
    }

    let mut out = String::new();
    let thousands = number / 1000;
    for _ in 0..thousands {
        out.push(t);
    }
    number %= 1000;

    let mut reduce = 100;
    for conv in convertion.iter() {
        let n = number / reduce;
        out = out.add(conv[n as usize]);
        number %= reduce;
        reduce /= 10;
    }

    Some(out)
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_upper_convertion() {
        assert_eq!(int_to_roman_upper(10).unwrap(), "X");
        assert_eq!(int_to_roman_upper(15).unwrap(), "XV");
        assert_eq!(int_to_roman_upper(230).unwrap(), "CCXXX");
        assert_eq!(int_to_roman_upper(2030).unwrap(), "MMXXX");
        assert_eq!(int_to_roman_upper(1999).unwrap(), "MCMXCIX");
    }

    #[test]
    fn test_lower_convertion() {
        assert_eq!(int_to_roman_lower(10).unwrap(), "x");
        assert_eq!(int_to_roman_lower(15).unwrap(), "xv");
        assert_eq!(int_to_roman_lower(230).unwrap(), "ccxxx");
        assert_eq!(int_to_roman_lower(2030).unwrap(), "mmxxx");
        assert_eq!(int_to_roman_lower(1999).unwrap(), "mcmxcix");
    }


    #[test]
    #[should_panic]
    fn test_wrong_values() {
        int_to_roman_lower(0).unwrap();
    }


}


