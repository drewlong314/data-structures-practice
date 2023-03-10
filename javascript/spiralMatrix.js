// Given an m x n matrix, return all elements of the matrix in spiral order.

// Example 1:
// Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
// Output: [1,2,3,6,9,8,7,4,5]

// Example 2:
// Input: matrix = [[1,2,3,4],[5,6,7,8],[9,10,11,12]]
// Output: [1,2,3,4,8,12,11,10,9,5,6,7]

const spiralOrder = function (matrix) {
    let top = 0;
    let bottom = matrix.length - 1;
    let left = 0;
    let right = matrix[0].length - 1;
    let output = [];

    while (left <= right && top <= bottom) {
        for (let i = left; i <= right; i++) {
            output.push(matrix[top][i])
        }
        top++;

        for (let i = top; i <= bottom; i++) {
            output.push(matrix[i][right])
        }
        right--;

        for (let i = right; i >= left && top <= bottom; i--) {
            output.push(matrix[bottom][i])
        }
        bottom--;

        for (let i = bottom; i >= top && left <= right; i--) {
            output.push(matrix[i][left])
        }
        left++;
    }

    return output;
};

console.log(spiralOrder([[1, 2, 3], [4, 5, 6], [7, 8, 9]]))
console.log(spiralOrder([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]))
console.log(spiralOrder([[3], [2]]))
// [[1,2,3,4],[5,6,7,8],[9,10,11,12],[13,14,15,16]]
// [1,2,3,4,8,12,16,15,14,13,9,10,11]
// [1,2,3,4,8,12,16,15,14,13,9,5,6,7,11,10]
