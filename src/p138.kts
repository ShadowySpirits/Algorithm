// 138. Copy List with Random Pointer

/**
 * Example:
 * var ti = Node(5)
 * var v = ti.`val`
 * Definition for a Node.
 * class Node(var `val`: Int) {
 *     var next: Node? = null
 *     var random: Node? = null
 * }
 */

class Node(var `val`: Int) {
    var next: Node? = null
    var random: Node? = null
}

class Solution {
    fun copyRandomList(node: Node?): Node? {
        val map = HashMap<Node /* old node */, Node /* corresponding new node */>()
        val newHead = node?.let {Node(it.`val`) }
        newHead?.next = node?.next

        var oldNode = node
        var newNode = newHead

        // copy linked list
        while (oldNode != null) {
            map[oldNode] = newNode!!

            oldNode = oldNode.next
            // build new node
            val tmpNode = oldNode?.let { Node(it.`val`) }
            tmpNode?.next = oldNode?.next

            newNode.next = tmpNode
            newNode = tmpNode
        }

        // copy random
        oldNode = node
        newNode = newHead
        while (oldNode != null) {
            newNode!!.random = map[oldNode.random]
            oldNode = oldNode.next
            newNode= newNode.next
        }

        return newHead
    }
}

val head = Node(1)
val tail = Node(2)
head.next = tail
tail.random = tail

Solution().copyRandomList(head)