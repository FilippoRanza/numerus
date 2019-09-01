# numerus
[![Build Status](https://travis-ci.com/FilippoRanza/numerus.svg?branch=master)](https://travis-ci.com/FilippoRanza/numerus) [![crates.io](https://img.shields.io/crates/v/numerus.svg)](https://crates.io/crates/numerus)

Convert integers to roman numerals and vice versa


## Examples

Convert from int to roman numeral.
```rust
    use numerus::int_to_roman_upper;
    let a = 14;
    assert_eq!(int_to_roman_upper(a).unwrap(), "XIV");
```

Convert a roman numeral to int
```rust
    use numerus::roman_to_int;
    let year = "MCMXCVIII";
    assert_eq!(roman_to_int(year).unwrap(), 1998);
```


For more examples see the [documentaion](https://docs.rs/numerus/0.1.0/numerus/)
