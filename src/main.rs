use std::vec;

use crate::two_sum::two_sum;

mod two_sum;

fn main() {
    let two_sum_nums = vec![9, 12, 3, 5, 4];
    let two_sum_answer = two_sum(two_sum_nums, 9);
    print!("Two Sum Answer: {:?} ", two_sum_answer)

}
