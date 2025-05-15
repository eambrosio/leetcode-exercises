// You are given an integer array digits, where each element is a digit. The array may contain duplicates.

// You need to find all the unique integers that follow the given requirements:

// The integer consists of the concatenation of three elements from digits in any arbitrary order.
// The integer does not have leading zeros.
// The integer is even.
// For example, if the given digits were [1, 2, 3], integers 132 and 312 follow the requirements.

// Return a sorted array of the unique integers.

// Example 1:

// Input: digits = [2,1,3,0]
// Output: [102,120,130,132,210,230,302,310,312,320]
// Explanation: All the possible integers that follow the requirements are in the output array.
// Notice that there are no odd integers or integers with leading zeros.
// Example 2:

// Input: digits = [2,2,8,8,2]
// Output: [222,228,282,288,822,828,882]
// Explanation: The same digit can be used as many times as it appears in digits.
// In this example, the digit 8 is used twice each time in 288, 828, and 882.
// Example 3:

// Input: digits = [3,7,5]
// Output: []
// Explanation: No even integers can be formed using the given digits.

// Constraints:

// 3 <= digits.length <= 100
// 0 <= digits[i] <= 9

use std::collections::{HashMap, HashSet};

// [2,2,8,8,2]
pub fn three_digit_even_numbers(digits: Vec<i32>) -> Vec<i32> {
    let mut results = vec![];
    // initialize a vector with numbers from 0..9
    let mut counter = [0; 10];

    for num in digits {
        // increase by `1` all numbers included in digits. we set the frequency based on the input.
        counter[num as usize] += 1;
    }

    // this for loop starts at 1 because we don't want leading zeros
    for i in 1..10 {
        if counter[i] == 0 {
            continue;
        }

        // mark the number to not used again in the next loop
        counter[i] -= 1;

        // this 2nd loop starts at 0
        for j in 0..10 {
            if counter[j] == 0 {
                continue;
            }
            // mark the number to not used again in the next loop
            counter[j] -= 1;
            for k in (0..10).step_by(2) {
                if counter[k] > 0 {
                    results.push((k + 10 * j + 100 * i) as i32);
                }
            }
            counter[j] += 1;
        }
        counter[i] += 1;
    }

    results
}
