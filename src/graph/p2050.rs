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

    // dfs algorithm
    pub fn minimum_time_dfs(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let mut graph = HashMap::with_capacity(n as usize);

        // build graph: Map<node, incoming_edge_set>
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

    // kahn algorithm
    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let mut graph = HashMap::with_capacity(n as usize);
        let mut indegree_vec: Vec<i32> = vec![0; n as usize];

        // build graph: Map<node, outgoing_edge_set>
        // This is the main difference between with dfs algorithm
        for relation in relations {
            let node = relation[0] - 1;
            let next_node = relation[1] - 1;
            let outgoing_set = graph.entry(node).or_insert(HashSet::new());
            outgoing_set.insert(next_node);
            indegree_vec[next_node as usize] += 1;
        }

        let mut zero_indegree_node_stack = vec![];
        for (node, indegree) in indegree_vec.iter().enumerate() {
            if *indegree == 0 {
                zero_indegree_node_stack.push(node);
            }
        }

        let mut res = vec![0; n as usize];
        while let Some(node) = zero_indegree_node_stack.pop() {
            // Add node itself time: max_path + current_node_value.
            res[node] += time[node];

            if let Some(outgoing_set) = graph.get(&(node as i32)) {
                for edge in outgoing_set {
                    let edge = *edge as usize;
                    // Select the max time path: max(current_path, previous_max_path).
                    res[edge] = max(res[node], res[edge]);

                    // Remove current edge.
                    // If all edge is removed, we can add the next node to the stack.
                    indegree_vec[edge] -= 1;
                    if indegree_vec[edge] == 0 {
                        zero_indegree_node_stack.push(edge);
                    }
                }
            }
        }

        *res.iter().max().unwrap()
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