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

const twoSum = function (nums, target) {
    // iterate over the nums array
    // iterate over the array again
    // add the number from the first iteration to the number from the second iteration
    // if they equal the target then return the current indexes

    let firstIndex = 0;
    let secondIndex = 1;

    while (firstIndex <= nums.length - 2) {
        secondIndex = firstIndex + 1
        while (secondIndex < nums.length) {
            if (nums[firstIndex] + nums[secondIndex] === target) return [firstIndex, secondIndex]

            secondIndex++
        }
        firstIndex++
    }

};
