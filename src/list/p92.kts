package list

// 725. Split Linked List in Parts

/**
 * Example:
 * var li = ListNode(5)
 * var v = li.`val`
 * Definition for singly-linked list.
 * class ListNode(var `val`: Int) {
 *     var next: ListNode? = null
 * }
 */

class ListNode(var `val`: Int) {
    var next: ListNode? = null
}

class Solution {
    fun reverseBetween(head: ListNode?, left: Int, right: Int): ListNode? {
        // get the head and tail node that need to be reversed
        var cursor = 0
        var cur = head
        var prev: ListNode? = null
        var next: ListNode? = null

        val list = mutableListOf<ListNode>()
        while (cursor < right) {
            cursor += 1
            if (cursor < left) {
                // record previous node of specified part head
                prev = cur
            }
            if (cursor in left..right) {
                list.add(cur!!)
            }
            if (cursor == right) {
                // record next node of specified part tail
                next = cur?.next
            }
            cur = cur?.next
        }

        // reverse specified part
        // of course we can reverse list and link each other in a single loop
        // additionally, recursion allows us to achieve O(1) space complexity
        val reversed = list.reversed()
        val subHead = reversed.first()
        prev?.next = subHead
        val subTail = reversed.last()
        subTail.next = next

        // Link every node in the reversed list together
        if (reversed.size >= 2) {
            var i = 0
            // skip tail node
            while (i <= reversed.size - 2) {
                reversed[i].next = reversed[i + 1]
                i++
            }
        }

        return if (prev == null) {
            subHead
        } else head
    }
}

var head = ListNode(1)
head.next = ListNode(2)
head.next!!.next = ListNode(3)
Solution().reverseBetween(head, 2, 3)