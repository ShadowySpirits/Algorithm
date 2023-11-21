//! 2391. Minimum Amount of Time to Collect Garbage

pub struct Solution;

impl Solution {
    pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
        let mut last_m = 0;
        let mut last_p = 0;
        let mut last_g = 0;

        for (index, garbage) in garbage.iter().enumerate() {
            if garbage.contains("M") {
                last_m = index;
            }
            if garbage.contains("P") {
                last_p = index;
            }
            if garbage.contains("G") {
                last_g = index;
            }
        }

        let mut cost_m = 0;
        let mut cost_p = 0;
        let mut cost_g = 0;

        let mut travel = travel;
        let mut extend_travel = vec![0];
        extend_travel.append(&mut travel);

        for (index, garbage) in garbage.iter().enumerate() {
            if index <= last_m {
                cost_m += extend_travel[index];
            }
            if index <= last_p {
                cost_p += extend_travel[index];
            }
            if index <= last_g {
                cost_g += extend_travel[index];
            }

            for garbage_type in garbage.chars() {
                match garbage_type {
                    'M' => cost_m += 1,
                    'P' => cost_p += 1,
                    'G' => cost_g += 1,
                    _ => {}
                };
            }
        }

        cost_m + cost_p + cost_g
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::garbage_collection(
            vec!["G".to_string(), "P".to_string(), "GP".to_string(), "GG".to_string()],
            vec![2, 4, 3],
        ), 21);
        assert_eq!(Solution::garbage_collection(
            vec!["MMM".to_string(), "PGM".to_string(), "GP".to_string()],
            vec![3, 10],
        ), 37);
    }
}