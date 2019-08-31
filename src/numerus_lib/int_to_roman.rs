
use std::ops::Add;


static _UPPER_UNITS: &[&'static str] =  &["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X"];
static _UPPER_TENS : &[&'static str] = &["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC", "C"];
static _UPPER_HUNDREDS : &[&'static str] = &["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM", "M"];
static _UPPER_CONVERT: &[&[&'static str]] = &[_UPPER_HUNDREDS, _UPPER_TENS, _UPPER_UNITS];

static _LOWER_UNITS: &[&'static str] =  &["", "i", "ii", "iii", "iv", "v", "vi", "vii", "viii", "ix", "x"];
static _LOWER_TENS : &[&'static str] = &["", "x", "xx", "xxx", "xl", "l", "lx", "lxx", "lxxx", "xc", "c"];
static _LOWER_HUNDREDS : &[&'static str] = &["", "c", "cc", "ccc", "cd", "d", "dc", "dcc", "dccc", "cm", "m"];
static _LOWER_CONVERT: &[&[&'static str]] = &[_LOWER_HUNDREDS, _LOWER_TENS, _LOWER_UNITS];


pub fn int_to_roman_lower(number: i32) -> Option<String> {
    convertion_engine(number, _LOWER_CONVERT, 'm')
}

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


