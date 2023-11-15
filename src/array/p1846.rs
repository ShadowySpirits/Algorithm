//! 1846. Maximum Element After Decreasing and Rearranging

pub struct Solution;

impl Solution {
    // Obviously, the maximum value we are looking for will not exceed the length of the array.
    // Thus we sort the array and then traverse the array in this way:
    // In each iteration, we check if the value is greater than the current maximum value.
    // If so we increment the maximum value by 1.
    pub fn maximum_element_after_decrementing_and_rearranging(arr: Vec<i32>) -> i32 {
        let mut arr = arr;
        arr.sort();

        let mut res = 0;

        for num in arr.into_iter() {
            if num > res {
                res += 1;
            }
        }

        res
    }

    // We can also solve this problem without sorting, which can solve the problem
    // in time complex O(n).
    pub fn without_sorting(arr: Vec<i32>) -> i32 {
        let len = arr.len();
        let mut count_vec = vec![0; len];

        // Because the maximum possible answer is the length of the array,
        // we can use the array as a counter.
        for num in arr.into_iter() {
            count_vec[len.min(num as usize) - 1] += 1;
        }

        // And then we traverse the array in reverse order to find how many number will miss.
        let mut miss = 0;
        let mut credit = 0;
        for count in count_vec.into_iter().rev() {
            if count > 0 {
                // Because we can only decrement the number, We accumulate credits
                // that can only be used on the next mismatch.
                credit += count - 1;
            } else if credit > 0 {
                credit -= 1;
            } else {
                miss += 1;
            }
        }

        len as i32 - miss
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::maximum_element_after_decrementing_and_rearranging(vec![2, 2, 1, 2, 1]),
            2
        );
        assert_eq!(
            Solution::maximum_element_after_decrementing_and_rearranging(vec![3, 2, 1, 2, 1]),
            3
        );
        assert_eq!(
            Solution::maximum_element_after_decrementing_and_rearranging(vec![100, 1, 1000]),
            3
        );
        assert_eq!(
            Solution::maximum_element_after_decrementing_and_rearranging(vec![1, 2, 3, 4, 5]),
            5
        );
        assert_eq!(
            Solution::maximum_element_after_decrementing_and_rearranging(vec![73, 98, 9]),
            3
        );
    }
}