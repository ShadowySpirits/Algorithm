//! 815. Bus Routes

use std::cmp::min;
use std::collections::{HashMap, HashSet};

pub struct Solution;

struct DFS {
    routes: Vec<HashSet<i32>>,
    map: HashMap<i32, HashSet<usize>>,
    target: i32,
}

impl DFS {
    fn dfs(&self, current_stop: i32, visit: &mut HashSet<usize>, cache: &mut HashMap<i32, i32>) -> i32 {
        if let Some(res) = cache.get(&current_stop) {
            return *res;
        }

        let set = self.map.get(&current_stop).unwrap();

        let mut min_step = 1_000_000;
        for bus in set {
            let route = &self.routes[*bus];
            if route.contains(&self.target) {
                min_step = 1;
                break;
            }

            if visit.contains(bus) {
                continue;
            }

            visit.insert(*bus);

            for next_stop in route.iter() {
                if *next_stop == current_stop {
                    continue;
                }

                min_step = min(min_step, self.dfs(*next_stop, visit, cache) + 1)
            }

            visit.remove(&bus);
        }

        cache.insert(current_stop, min_step);

        min_step
    }
}

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {
            return 0;
        }

        let mut map = HashMap::new();
        let mut new_routes = vec![HashSet::new(); routes.len()];

        for (i, route) in routes.iter().enumerate() {
            for stop in route {
                new_routes[i].insert(*stop);
                let set = map.entry(*stop).or_insert(HashSet::new());
                set.insert(i);
            }
        }

        let dfs = DFS {
            routes: new_routes,
            map,
            target,
        };

        let res = dfs.dfs(source, &mut HashSet::new(), &mut HashMap::new());

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
        let routes = vec![vec![1, 2, 7], vec![3, 6, 7]];
        assert_eq!(Solution::num_buses_to_destination(routes, 1, 6), 2);

        let routes = vec![vec![1, 2, 7], vec![3, 5, 7], vec![5, 9], vec![9, 6]];
        assert_eq!(Solution::num_buses_to_destination(routes, 1, 6), 4);

        let routes = vec![vec![1, 2, 7], vec![5, 6, 7], vec![5, 9], vec![9, 6]];
        assert_eq!(Solution::num_buses_to_destination(routes, 1, 6), 2);

        let routes = vec![vec![1, 9], vec![1, 2, 7], vec![3, 5, 7], vec![5, 9], vec![9, 6]];
        assert_eq!(Solution::num_buses_to_destination(routes, 1, 6), 2);

        let routes = vec![vec![7, 12], vec![4, 5, 15], vec![6], vec![15, 19], vec![9, 12, 13]];
        assert_eq!(Solution::num_buses_to_destination(routes, 15, 12), -1);

        let routes = vec![vec![4, 10, 20], vec![1, 10], vec![1, 23], vec![20, 21, 23]];
        assert_eq!(Solution::num_buses_to_destination(routes, 4, 21), 2);
    }
}