// Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.

// You must write an algorithm with O(log n) runtime complexity.

// Example 1:
// Input: nums = [1,3,5,6], target = 5
// Output: 2

// Example 2:
// Input: nums = [1,3,5,6], target = 2
// Output: 1

// Example 3:
// Input: nums = [1,3,5,6], target = 7
// Output: 4

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let search_result = nums.binary_search(&target);
    // search_result.unwrap_or_else(|x| x) as i32
    // unwrap or else does the same thing as the match below
    match search_result {
        Ok(i) => {
            return i as i32;
        },
        Err(e) => {
            return e as i32;
        }
    }
}
