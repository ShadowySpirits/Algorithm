//! 2642. Design Graph With Shortest Path Calculator

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

struct Graph {
    node_count: usize,
    edge_map: HashMap<i32 /*Node*/, Vec<(i32 /*Node*/, i32 /*Weight*/)> /*EdgeVec*/>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Graph {
    fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        Self {
            node_count: n as usize,
            edge_map: edges.into_iter()
                .fold(HashMap::with_capacity(n as usize), |mut map, edge| {
                    let from = edge[0];
                    let to = edge[1];
                    let weight = edge[2];
                    map.entry(from).or_insert(vec![]).push((to, weight));
                    map
                }),
        }
    }

    fn add_edge(&mut self, edge: Vec<i32>) {
        let from = edge[0];
        let to = edge[1];
        let weight = edge[2];
        self.edge_map.entry(from).or_insert(vec![]).push((to, weight));
    }

    // Dijkstra
    fn shortest_path(&self, source: i32, target: i32) -> i32 {
        let mut distance_vec = vec![i32::MAX; self.node_count];
        distance_vec[source as usize] = 0;

        let mut heap = BinaryHeap::with_capacity(self.node_count);
        heap.push((Reverse(0), source));

        while let Some((Reverse(distance), node)) = heap.pop() {
            // We skip dealing with the already visited node, which has calculated the shortest path.
            if distance > distance_vec[node as usize] {
                continue;
            }

            // We start dealing with the target node, which means we have calculated
            // all shorter paths. In other words, we can not find a shorter path that
            // starts with the target node and ends in the target node.
            if node == target {
                return distance;
            }

            if let Some(edge_vec) = self.edge_map.get(&node) {
                for (to, weight) in edge_vec {
                    let to = *to as usize;
                    // Notice: the point is that we should accumulate the distance.
                    let new_distance = distance + weight;
                    if new_distance < distance_vec[to] {
                        distance_vec[to] = new_distance;
                        heap.push((Reverse(new_distance), to as i32))
                    }
                }
            }
        }

        let res = distance_vec[target as usize];
        if res == i32::MAX {
            -1
        } else {
            res
        }
    }
}

/**
 * Your Graph object will be instantiated and called as such:
 * let obj = Graph::new(n, edges);
 * obj.add_edge(edge);
 * let ret_2: i32 = obj.shortest_path(node1, node2);
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 3;
        let edges = vec![vec![0, 1, 100], vec![1, 2, 100]];
        let mut graph = Graph::new(n, edges);
        graph.add_edge(vec![0, 2, 500]);

        let distance: i32 = graph.shortest_path(0, 1);
        assert_eq!(distance, 100);

        let distance: i32 = graph.shortest_path(1, 2);
        assert_eq!(distance, 100);

        let distance: i32 = graph.shortest_path(0, 2);
        assert_eq!(distance, 200);

        let distance: i32 = graph.shortest_path(2, 0);
        assert_eq!(distance, -1);


        let n = 4;
        let edges = vec![vec![0, 1, 5], vec![1, 2, 3], vec![1, 3, 8], vec![2, 3, 7]];
        let graph = Graph::new(n, edges);

        let distance: i32 = graph.shortest_path(0, 3);
        assert_eq!(distance, 13);
    }
}