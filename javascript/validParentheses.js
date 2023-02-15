// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

// An input string is valid if:
// Open brackets must be closed by the same type of brackets.
// Open brackets must be closed in the correct order.
// Every close bracket has a corresponding open bracket of the same type.


// Example 1:
// Input: s = "()"
// Output: true

// Example 2:
// Input: s = "()[]{}"
// Output: true

// Example 3:
// Input: s = "(]"
// Output: false

const isValid = function (s) {
    // iterate over the string
    // create a variable to hold current open parentheses
    // when a closed parentheses is found remove the open parentheses from the variable
    // if the variable does not have a matching parentheses return false
    // at the end if the variable is empty return true
    // else return false
    let openPar = [];
    let answer = true;
    for (let i = 0; i < s.length; i++) {
        let char = s[i];
        if (char === "(" || char === "[" || char === "{") openPar.unshift(char)
        else if (char === ")") {
            if (openPar.shift() != "(") {
                answer = false
                break;
            }
        }
        else if (char === "]") {
            if (openPar.shift() != "[") {
                answer = false
                break;
            }
        }
        else if (char === "}") {
            if (openPar.shift() != "{") {
                answer = false
                break;
            }
        }
    }

    if (openPar.length) return false
    return answer
}

console.log(isValid("()"))
console.log(isValid("()[]{}"))
console.log(isValid("(]"))
console.log(isValid("([)]")) // false
