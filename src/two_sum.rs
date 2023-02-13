// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
// You may assume that each input would have exactly one solution, and you may not use the same element twice.
// You can return the answer in any order.

// Example: 1
// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].

// Example 2:
// Input: nums = [3,2,4], target = 6
// Output: [1,2]

// Example 3:
// Input: nums = [3,3], target = 6
// Output: [0,1]

use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // Hashmap

    let mut hash: HashMap<i32, usize> = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&complement_index) = hash.get(&complement) {
            return vec![complement_index as i32, i as i32];
        }
        hash.insert(*num, i);
    }

    // Easier to read code
    // pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    //     let mut answer_map: HashMap<i32, i32> = HashMap::new();
    //     for (pos, n) in nums.iter().enumerate() {
    //         let answer: i32 = target - n;
    //         if answer_map.contains_key(&n){
    //             return vec![pos as i32, answer_map.get(&n).unwrap().clone()]
    //         } else {
    //             answer_map.insert(answer.clone(), pos as i32);
    //         }
    //     }
    //     Vec::new()
    // }

    // Brute Force
    // for (i, num1) in nums.iter().enumerate() {
    //     for (j, num2) in nums.iter().skip(i + 1).enumerate() {
    //         if num1 + num2 == target {
    //             return vec![i as i32, (j + 1 + i) as i32];
    //         }
    //     }
    // }
    return vec![];
}
