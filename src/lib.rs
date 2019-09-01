//!
//! # Numerus
//! 
//! Convert integers to roman numerals and vice versa
//! 
//! 
//! ## Examples
//! 
//! Convert from integer to an uppercase roman numeral.
//! ```
//!     use numerus::int_to_roman_upper;
//! 
//!     let a = 14;
//!     assert_eq!(int_to_roman_upper(a).unwrap(), "XIV");
//! ```
//! 
//! Convert from integer to a lowercase roman numeral.
//! ```
//!     use numerus::int_to_roman_lower;
//! 
//!     let a = 789;
//!     assert_eq!(int_to_roman_lower(a).unwrap(), "dcclxxxix");
//! ```
//! 
//! 
//! Convert from a roman numeral to an integer
//! ```
//!     use numerus::roman_to_int;
//! 
//!     let year = "MCMXCVIII";
//!     assert_eq!(roman_to_int(year).unwrap(), 1998);
//! ```
//! 
//! See functions documentation for more information 
//! about error handling
//! 
pub mod numerus_lib;

pub use numerus_lib::int_to_roman::{
    int_to_roman_upper,
    int_to_roman_lower
};

pub use numerus_lib::roman_to_int::roman_to_int;

