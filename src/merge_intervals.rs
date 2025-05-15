// You are given an array of non-overlapping intervals intervals where intervals[i] = [starti, endi] represent the start and the end of the ith interval and intervals is sorted in ascending order by starti. You are also given an interval newInterval = [start, end] that represents the start and end of another interval.

// Insert newInterval into intervals such that intervals is still sorted in ascending order by starti and intervals still does not have any overlapping intervals (merge overlapping intervals if necessary).

// Return intervals after the insertion.

// Note that you don't need to modify intervals in-place. You can make a new array and return it.

// Example 1:

// Input: intervals = [[1,3],[6,9]], newInterval = [2,5]
// Output: [[1,5],[6,9]]
// Example 2:

// Input: intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
// Output: [[1,2],[3,10],[12,16]]
// Explanation: Because the new interval [4,8] overlaps with [3,5],[6,7],[8,10].

// Constraints:

// 0 <= intervals.length <= 104
// intervals[i].length == 2
// 0 <= starti <= endi <= 105
// intervals is sorted by starti in ascending order.
// newInterval.length == 2
// 0 <= start <= end <= 105

use std::cmp::{max, min};

pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut new_interval = new_interval;
    let mut solution: Vec<Vec<i32>> = vec![];

    for current_interval in intervals {
        if current_interval[1] < new_interval[0] {
            // current_interval starts first and not overlaps
            solution.push(current_interval);
        } else if current_interval[0] > new_interval[1] {
            // new_interval starts first and not overlaps
            solution.push(new_interval);
            new_interval = current_interval;
        } else if current_interval[1] >= new_interval[0] || current_interval[0] <= new_interval[1] {
            // there's an overlap
            new_interval[0] = min(current_interval[0], new_interval[0]);
            new_interval[1] = max(current_interval[1], new_interval[1]);
        }
    }

    solution.push(new_interval);
    solution
}
