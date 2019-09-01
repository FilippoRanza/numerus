
extern crate numerus;

#[cfg(test)]
mod test {
    
    use numerus::{
        int_to_roman_upper,
        int_to_roman_lower,
        roman_to_int
    };

    #[test]
    fn test_full_convetion_uppercase() {
        for i in 1..10000 {
            let tmp = int_to_roman_upper(i).unwrap();
            assert_eq!(roman_to_int(&tmp).unwrap(), i);
        }
    }


    #[test]
    fn test_full_convetion_lowercase() {
        for i in 1..10000 {
            let tmp = int_to_roman_lower(i).unwrap();
            assert_eq!(roman_to_int(&tmp).unwrap(), i);
        }
    }

}