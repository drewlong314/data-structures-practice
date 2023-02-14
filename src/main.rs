use std::vec;

use palindrome_number::palindrome_number;

use crate::{two_sum::two_sum, roman_to_int::roman_to_int};

pub mod palindrome_number;
mod two_sum;
pub mod roman_to_int;

fn main() {
    let two_sum_nums = vec![9, 12, 3, 5, 4];
    let two_sum_answer = two_sum(two_sum_nums, 9);
    let palindrome_number_answer = palindrome_number(131);
    let roman_to_int_answer = roman_to_int("MCMXCIV".to_string());
    print!("Two Sum Answer: {:?} ", two_sum_answer);
    print!("Palindrome Number Answer: {:?} ", palindrome_number_answer);
    print!("Roman to Int Answer: {:?} ", roman_to_int_answer);
}
