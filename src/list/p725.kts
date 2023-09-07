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
    fun splitListToParts(head: ListNode?, k: Int): Array<ListNode?> {
        val res = arrayOfNulls<ListNode?>(k)

        // length of total linked list
        var len = 0
        var tmpNode = head
        while (tmpNode != null) {
            len++
            tmpNode = tmpNode.next
        }

        // length of parted linked list
        var partLen = len / k
        // count of longer part
        var longerPart = 0
        if (len < k) {
            partLen += 1
        } else {
            longerPart = len % k
        }

        tmpNode = head
        for (i in 0 until k) {
            res[i] = tmpNode

            var diff = 0
            if (longerPart > 0) {
                longerPart -= 1
                diff = 1
            }

            // calculate last node of res[i]
            var last = tmpNode
            for (j in 0 until partLen + diff - 1) {
                tmpNode = tmpNode?.next
                last = tmpNode
            }

            // assign tmpNode to the first node of next part
            tmpNode = last?.next
            // disconnect each part
            last?.next = null
        }

        return res
    }
}

var head = ListNode(1)
head.next = ListNode(2)
head.next!!.next = ListNode(3)
Solution().splitListToParts(head, 2)
Solution().splitListToParts(head, 5)