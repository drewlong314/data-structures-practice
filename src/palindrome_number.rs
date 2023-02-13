// Given an integer x, return true if x is a palindrome, and false otherwise.

// Example 1:
// Input: x = 121
// Output: true
// Explanation: 121 reads as 121 from left to right and from right to left.

// Example 2:
// Input: x = -121
// Output: false
// Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.

// Example 3:
// Input: x = 10
// Output: false
// Explanation: Reads 01 from right to left. Therefore it is not a palindrome.

pub fn palindrome_number(x: i32) -> bool {
    let x = x.to_string().into_bytes();
    let mut start = 0;
    let mut end = x.len() - 1;

    loop {
        if start == end || start > end {
            break;
        }

        if x[start] != x[end] {
            return false;
        }

        start += 1;
        end -= 1;
    }

    return true;

    // x.to_string() == x.to_string().chars().rev().collect::<String>()
}
