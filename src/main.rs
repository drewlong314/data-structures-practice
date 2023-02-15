use std::vec;

use palindrome_number::palindrome_number;

use crate::{two_sum::two_sum, roman_to_int::roman_to_int, longest_common_prefix::longest_common_prefix};

pub mod palindrome_number;
mod two_sum;
pub mod roman_to_int;
pub mod longest_common_prefix;
pub mod valid_parentheses;

fn main() {
    let two_sum_nums = vec![9, 12, 3, 5, 4];
    let two_sum_answer = two_sum(two_sum_nums, 9);
    let palindrome_number_answer = palindrome_number(131);
    let roman_to_int_answer = roman_to_int("MCMXCIV".to_string());
    let longest_common_prefix_answer= longest_common_prefix(["ccc".to_string(),"acc".to_string(),"ccc".to_string(),"cccc".to_string()].to_vec());
    print!("Two Sum Answer: {:?} ", two_sum_answer);
    print!("Palindrome Number Answer: {:?} ", palindrome_number_answer);
    print!("Roman to Int Answer: {:?} ", roman_to_int_answer);
    print!("Longsest Common Prefix Answer: {:?} ", longest_common_prefix_answer);
}
