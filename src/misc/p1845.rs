//! 1845. Seat Reservation Manager

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct SeatManager {
    heap: BinaryHeap<Reverse<i32>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SeatManager {

    fn new(n: i32) -> Self {
        SeatManager {
            heap: (1..=n).map(Reverse).collect()
        }
    }

    fn reserve(&mut self) -> i32 {
        self.heap.pop().unwrap().0
    }

    fn unreserve(&mut self, seat_number: i32) {
        self.heap.push(Reverse(seat_number));
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Your SeatManager object will be instantiated and called as such:
     * let obj = SeatManager::new(n);
     * let ret_1: i32 = obj.reserve();
     * obj.unreserve(seatNumber);
     */
    #[test]
    fn test() {
        let mut obj = SeatManager::new(5);
        assert_eq!(obj.reserve(), 1);
        assert_eq!(obj.reserve(), 2);
        obj.unreserve(2);
        assert_eq!(obj.reserve(), 2);
        assert_eq!(obj.reserve(), 3);
        assert_eq!(obj.reserve(), 4);
        assert_eq!(obj.reserve(), 5);
        obj.unreserve(5);
        assert_eq!(obj.reserve(), 5);
    }
}