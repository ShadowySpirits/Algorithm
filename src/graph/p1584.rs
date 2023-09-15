//! 1584. Min Cost to Connect All Points

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub struct Solution;

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let mut map = HashMap::new();
        for (i, point) in points[..points.len() - 1].iter().enumerate() {
            for (j, another_point) in points[i + 1..].iter().enumerate() {
                let distance = (point[0] - another_point[0]).abs() + (point[1] - another_point[1]).abs();
                map.entry(i).or_insert(BinaryHeap::new()).push(Reverse((distance, i + 1 + j)));
                map.entry(i + 1 + j).or_insert(BinaryHeap::new()).push(Reverse((distance, i)));
            }
        }

        let mut res = 0;
        let mut visit = HashMap::new();
        for (point, heap) in map {
            let pair = heap.peek().unwrap().0;
            let distance = pair.0;
            let another_point = pair.1;

            if visit.contains_key(&point) {
                let last_distance = *visit.get(&point).unwrap();
                if distance < last_distance {
                    res -= distance;
                } else {
                    continue;
                }
            }

            visit.insert(point, distance);
            visit.insert(another_point, distance);
            res += distance;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::min_cost_connect_points(vec![
                vec![0, 0],
                vec![2, 2],
                vec![3, 10],
                vec![5, 2],
                vec![7, 0],
            ]),
            20
        );
        assert_eq!(
            Solution::min_cost_connect_points(vec![
                vec![3, 12],
                vec![-2, 5],
                vec![-4, 1],
                vec![0, 0],
                vec![-5, -2],
                vec![2, 4],
                vec![9, 6],
            ]),
            20
        );
        assert_eq!(
            Solution::min_cost_connect_points(vec![
                vec![0, 0],
                vec![1, 1],
                vec![1, 0],
                vec![-1, 1],
            ]),
            4
        );
        assert_eq!(
            Solution::min_cost_connect_points(vec![
                vec![-1000000, -1000000],
                vec![1000000, 1000000],
            ]),
            4000000
        );
        assert_eq!(
            Solution::min_cost_connect_points(vec![
                vec![0, 0],
                vec![2, 8],
                vec![8, 2],
                vec![4, 4],
                vec![3, 3],
                vec![0, 0],
                vec![7, 7],
                vec![9, 9],
            ]),
            39
        );
    }
}