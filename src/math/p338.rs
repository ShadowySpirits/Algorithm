//! 338. Counting Bits

struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity((n + 1) as usize);
        (0..=n).for_each(|i| {
            let binary_string = format!("{:b}", i);
            let mut count = 0;
            for c in binary_string.chars() {
                if c == '1' {
                    count += 1;
                }
            }
            result.push(count);
        });
        result
    }

    pub fn use_built_in(n: i32) -> Vec<i32> {
        (0..=n as usize).map(|x| x.count_ones() as i32).collect()
    }

    // Of course, it has magical math solution.
    // By listing the bit count of numbers ranging from 0 to 15 and
    // finding to the general term formula, we can determine the pattern:
    // count[i] = if i & 1 == 0 {
    //     count[i >> 1]
    // } else {
    //     count[i - 1] + 1
    // };
    pub fn dp(_n: i32) -> Vec<i32> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
        assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }
}