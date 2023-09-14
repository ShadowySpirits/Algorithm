//! 332. Reconstruct Itinerary

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub struct Solution;

impl Solution {
    // TODO: WA! Still work in progress
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut map = HashMap::new();

        for ticket in tickets {
            let heap = map.entry(ticket[0].to_string()).or_insert(BinaryHeap::new());
            heap.push(Reverse(ticket[1].to_string()));
        }

        let mut res = vec!["JFK".to_string()];
        let mut last = None;

        while !map.is_empty() {
            let departure = res.last().unwrap().to_string();
            if map.contains_key(&departure) {
                let heap = map.get_mut(&departure).unwrap();
                res.push(heap.pop().unwrap().0.to_string());

                if heap.is_empty() {
                    map.remove(&departure);
                }
            } else if last.is_none() {
                last = Some(res.pop().unwrap())
            } else {
                panic!("last is already has value!")
            }
        }

        if let Some(last) = last {
            res.push(last)
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
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

        assert_eq!(Solution::find_itinerary(vec![vec!["EZE", "TIA"], vec!["EZE", "HBA"], vec!["AXA", "TIA"], vec!["JFK", "AXA"], vec!["ANU", "JFK"], vec!["ADL", "ANU"], vec!["TIA", "AUA"], vec!["ANU", "AUA"], vec!["ADL", "EZE"], vec!["ADL", "EZE"], vec!["EZE", "ADL"], vec!["AXA", "EZE"], vec!["AUA", "AXA"], vec!["JFK", "AXA"], vec!["AXA", "AUA"], vec!["AUA", "ADL"], vec!["ANU", "EZE"], vec!["TIA", "ADL"], vec!["EZE", "ANU"], vec!["AUA", "ANU"]].into_iter().map(|v| v.iter().map(|s| s.to_string()).collect()).collect()), vec![
            "JFK",
            "NRT",
            "JFK",
            "KUL",
        ].into_iter().map(|s| s.to_string()).collect::<Vec<String>>());
    }
}