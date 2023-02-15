use std::vec;

use palindrome_number::palindrome_number;

use crate::{
    longest_common_prefix::longest_common_prefix, roman_to_int::roman_to_int,
    search_insert_position::search_insert, two_sum::two_sum, valid_parentheses::is_valid,
};

pub mod longest_common_prefix;
pub mod palindrome_number;
pub mod roman_to_int;
pub mod search_insert_position;
mod two_sum;
pub mod valid_parentheses;

fn main() {
    let two_sum_nums = vec![9, 12, 3, 5, 4];
    let two_sum_answer = two_sum(two_sum_nums, 9);
    let palindrome_number_answer = palindrome_number(131);
    let roman_to_int_answer = roman_to_int("MCMXCIV".to_string());
    let longest_common_prefix_answer = longest_common_prefix(
        [
            "ccc".to_string(),
            "acc".to_string(),
            "ccc".to_string(),
            "cccc".to_string(),
        ]
        .to_vec(),
    );
    let valid_parentheses_answer = is_valid(String::from("]()]{}"));
    let search_insert_position_answer = search_insert(Vec::from([1, 3, 5, 6]), 7);
    println!("Two Sum Answer: {:?} ", two_sum_answer);
    println!("Palindrome Number Answer: {:?} ", palindrome_number_answer);
    println!("Roman to Int Answer: {:?} ", roman_to_int_answer);
    println!(
        "Longsest Common Prefix Answer: {:?} ",
        longest_common_prefix_answer
    );
    println!("Valid Parenthese Answer: {:?} ", valid_parentheses_answer);
    println!("Search Insert Position Answer: {:?} ", search_insert_position_answer);
}
