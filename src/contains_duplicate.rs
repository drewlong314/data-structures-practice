use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut num_set = HashSet::<i32>::new();
    for num in nums {
        if !num_set.insert(num) {
            return true;
        }
    }
    return false;
}
