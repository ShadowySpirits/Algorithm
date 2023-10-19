//! 2050. Parallel Courses III

use std::cmp::max;
use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    fn dfs(node: i32, graph: &HashMap<i32, HashSet<i32>>, time: &Vec<i32>, visit: &mut HashMap<i32, i32>) -> i32 {
        let incoming_set = graph.get(&node);
        if incoming_set.is_none() {
            let result = time[node as usize];
            visit.insert(node, result);
            return result;
        }
        let incoming_set = incoming_set.unwrap();

        if let Some(result) = visit.get(&node) {
            return *result;
        }

        let mut res = i32::MIN;
        for incoming_edge in incoming_set {
            res = max(res, Solution::dfs(*incoming_edge, graph, time, visit))
        }

        res += time[node as usize];
        visit.insert(node, res);

        res
    }

    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let mut graph = HashMap::with_capacity(n as usize);

        // build graph
        for relation in relations {
            let incoming_set = graph.entry(relation[1] - 1).or_insert(HashSet::new());
            incoming_set.insert(relation[0] - 1);
        }

        let mut visit = HashMap::with_capacity(n as usize);
        for i in 0..n {
            Solution::dfs(i, &graph, &time, &mut visit);
        }

        *visit.values().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::minimum_time(1, vec![], vec![1]), 1);
        assert_eq!(Solution::minimum_time(3, vec![], vec![1, 2, 3]), 3);
        assert_eq!(Solution::minimum_time(3, vec![vec![2, 3]], vec![3, 1, 1]), 3);
        assert_eq!(Solution::minimum_time(3, vec![vec![1, 3], vec![2, 3]], vec![3, 2, 5]), 8);
        assert_eq!(Solution::minimum_time(5, vec![vec![1, 5], vec![2, 5], vec![3, 5], vec![3, 4], vec![4, 5]], vec![1, 2, 3, 4, 5]), 12);
    }
}