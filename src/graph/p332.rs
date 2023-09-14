//! 332. Reconstruct Itinerary

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub struct Solution;

impl Solution {
    // Using DFS and a Min-heap to find the smallest lexicographical path in a directed graph.
    // Because all the tickets must be used, the graph is a Eulerian path.
    // Thus we can use DFS that find the vertex with zero out-degree.
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        // The graph is represented by a HashMap, where the key is the source airport
        // and the value is a Min-heap of destination airports.
        let mut graph = HashMap::new();

        for ticket in tickets {
            let heap = graph.entry(ticket[0].to_string()).or_insert(BinaryHeap::new());
            heap.push(Reverse(ticket[1].to_string()));
        }

        // the stack to store the path temporarily.
        let mut stack = Vec::new();
        stack.push("JFK".to_string());

        // the stack to push in the vertex with zero out-degree.
        let mut res = Vec::new();

        // DFS
        // The loop will end when the stack is empty which means all of the ticks has been used.
        while !stack.is_empty() {
            // Find the vertex with zero out-degree,
            // or in other words, destination airport in the loop.
            while let Some(heap) = graph.get_mut(stack.last().unwrap()) {
                if heap.is_empty() {
                    break;
                }
                stack.push(heap.pop().unwrap().0);
            }
            // Push the destination airport into the result stack.
            res.push(stack.pop().unwrap());
        }

        // Because we push the destination airport each time we find it,
        // we need to reverse it to get the result.
        res.reverse();
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // We push JFK into stack first, then push ATL but not LHR according to lexicographical order.
        // So that we find the first vertex with zero out-degree, which is ATL.
        // We pop it from stack and push it into result.
        //
        // Then we push LHR and JFK into stack, and find the next vertex with zero out-degree, which is JFK.
        // The we pop JFK, LHR, JFK one by one from stack and push it into result.
        //
        // Finally, we get the result: [ATL, JFK, LHR, JFK]. Reverse it and we get the answer.
        assert_eq!(Solution::find_itinerary(vec![
            vec!["JFK", "ATL"],
            vec!["LHR", "JFK"],
            vec!["JFK", "LHR"],
        ].into_iter().map(|v| v.iter().map(|s| s.to_string()).collect()).collect()), vec![
            "JFK",
            "LHR",
            "JFK",
            "ATL",
        ].into_iter().map(|s| s.to_string()).collect::<Vec<String>>());

        assert_eq!(Solution::find_itinerary(vec![
            vec!["MUC", "LHR"],
            vec!["JFK", "MUC"],
            vec!["SFO", "SJC"],
            vec!["LHR", "SFO"],
        ].into_iter().map(|v| v.iter().map(|s| s.to_string()).collect()).collect()), vec![
            "JFK",
            "MUC",
            "LHR",
            "SFO",
            "SJC",
        ].into_iter().map(|s| s.to_string()).collect::<Vec<String>>());

        assert_eq!(Solution::find_itinerary(vec![
            vec!["JFK", "SFO"],
            vec!["JFK", "ATL"],
            vec!["SFO", "ATL"],
            vec!["ATL", "JFK"],
            vec!["ATL", "SFO"],
        ].into_iter().map(|v| v.iter().map(|s| s.to_string()).collect()).collect()), vec![
            "JFK",
            "ATL",
            "JFK",
            "SFO",
            "ATL",
            "SFO",
        ].into_iter().map(|s| s.to_string()).collect::<Vec<String>>());
    }
}