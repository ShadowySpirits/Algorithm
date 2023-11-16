//! 815. Bus Routes

use std::cmp::min;
use std::collections::{HashMap, HashSet};

pub struct Solution;

struct DFS {
    routes: Vec<Vec<i32>>,
    map: HashMap<i32, HashSet<usize>>,
    cache: HashMap<i32, i32>,
    target: i32,
}

impl DFS {
    fn dfs(&mut self, current_stop: i32, visit: &mut HashSet<usize>) -> i32 {
        if current_stop == self.target {
            return 0;
        }

        if let Some(res) = self.cache.get(&current_stop) {
            return *res;
        }

        let set = self.map.get(&current_stop).unwrap().clone();

        let mut min_step = 1000000;
        for bus in set {
            if visit.contains(&bus) {
                continue;
            }

            visit.insert(bus);

            let vec = self.routes[bus].clone();
            for next_stop in vec.iter() {
                if *next_stop == current_stop {
                    continue;
                }

                min_step = min(min_step, self.dfs(*next_stop, visit) + 1)
            }

            visit.remove(&bus);
        }

        self.cache.insert(current_stop, min_step);

        min_step
    }
}

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        let mut map = HashMap::new();
        for (i, route) in routes.iter().enumerate() {
            for stop in route {
                let set = map.entry(*stop).or_insert(HashSet::new());
                set.insert(i);
            }
        }

        let mut dfs = DFS {
            routes,
            map,
            cache: HashMap::new(),
            target,
        };

        let res = dfs.dfs(source, &mut HashSet::new());

        if res == 1_000_000 {
            -1
        } else {
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // let routes = vec![vec![1, 2, 7], vec![3, 6, 7]];
        // assert_eq!(Solution::num_buses_to_destination(routes, 1, 6), 2);
        //
        // let routes = vec![vec![1, 2, 7], vec![3, 5, 7], vec![5, 9], vec![9, 6]];
        // assert_eq!(Solution::num_buses_to_destination(routes, 1, 6), 4);
        //
        // let routes = vec![vec![1, 2, 7], vec![5, 6, 7], vec![5, 9], vec![9, 6]];
        // assert_eq!(Solution::num_buses_to_destination(routes, 1, 6), 2);
        //
        // let routes = vec![vec![1, 9], vec![1, 2, 7], vec![3, 5, 7], vec![5, 9], vec![9, 6]];
        // assert_eq!(Solution::num_buses_to_destination(routes, 1, 6), 2);
        //
        // let routes = vec![vec![7, 12], vec![4, 5, 15], vec![6], vec![15, 19], vec![9, 12, 13]];
        // assert_eq!(Solution::num_buses_to_destination(routes, 15, 12), -1);

        let routes = vec![vec![4, 10, 20], vec![1, 10], vec![1, 23], vec![20, 21, 23]];
        assert_eq!(Solution::num_buses_to_destination(routes, 4, 21), 2);
    }
}