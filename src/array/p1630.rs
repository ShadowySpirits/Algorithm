//! 1630. Arithmetic Subarrays

pub struct Solution;

impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        let mut res = vec![];

        for (l_index, r_index) in l.into_iter().zip(r) {
            let l_index = l_index as usize;
            let r_index = r_index as usize;

            if r_index < l_index + 2 {
                res.push(true);
                continue;
            }

            let mut slice = vec![];
            nums[l_index..=r_index].clone_into(&mut slice);
            slice.sort();

            let diff = slice[1] - slice[0];
            let mut is_arithmetic = true;

            for i in 2..slice.len() {
                if slice[i] - slice[i - 1] != diff {
                    is_arithmetic = false;
                    break;
                }
            }
            res.push(is_arithmetic);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![4, 6, 5, 9, 3, 7];
        let l = vec![0, 0, 2];
        let r = vec![2, 3, 5];
        let res = vec![true, false, true];
        assert_eq!(Solution::check_arithmetic_subarrays(nums, l, r), res);


        let nums = vec![4, 6, 5, 9, 3, 7];
        let l = vec![0, 0];
        let r = vec![0, 1];
        let res = vec![true, true];
        assert_eq!(Solution::check_arithmetic_subarrays(nums, l, r), res);
    }
}