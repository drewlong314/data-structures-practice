// Write a function to find the longest common prefix string amongst an array of strings.

// If there is no common prefix, return an empty string "".

// Example 1:
// Input: strs = ["flower","flow","flight"]
// Output: "fl"

// Example 2:
// Input: strs = ["dog","racecar","car"]
// Output: ""
// Explanation: There is no common prefix among the input strings.

const longestCommonPrefix = function (strs) {
    // for each of the strings in the arrays check the first string to each other
    // then check the second
    // this seems like it is ineffiecient

    let shortest;
    let answer = ""
    strs.forEach(str => {
        if (!shortest) shortest = str.length
        else if (str.length < shortest) shortest = str.length
    });

    for (let i = 0; i < shortest; i++) {
        let prefix = strs[0].slice(0, i + 1)
        let endLoop = false;
        for (let j = 0; j < strs.length; j++) {
            secondIndex = j
            if (strs[j].slice(0, i + 1) != prefix) {
                endLoop = true
                break;
            }

        }
        if (endLoop) break;
        answer = prefix

    }

    return answer;
};

console.log(longestCommonPrefix(["dog", "racecar", "car"]))
