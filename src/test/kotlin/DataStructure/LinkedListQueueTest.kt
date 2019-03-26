package DataStructure

import org.testng.Assert.*
import org.testng.annotations.BeforeMethod
import org.testng.annotations.Test

class LinkedListQueueTest {
    private lateinit var queue: LinkedListQueue<Int>

    @BeforeMethod
    fun setUp() {
        queue = LinkedListQueue()
    }

    @Test
    fun testIsEmpty() {
        assertTrue(queue.isEmpty)
        queue.enqueue(1)
        assertFalse(queue.isEmpty)
    }

    @Test
    fun testSize() {
        assertEquals(queue.size(), 0)
        queue.enqueue(1)
        assertEquals(queue.size(), 1)
        queue.enqueue(1)
        assertEquals(queue.size(), 2)
    }

    @Test
    fun testEnqueue() {
        queue.enqueue(1)
        assertEquals(queue.dequeue(), Integer.valueOf(1))
        queue.enqueue(1)
        queue.enqueue(2)
        queue.enqueue(3)
        assertEquals(queue.dequeue(), Integer.valueOf(1))
        assertEquals(queue.dequeue(), Integer.valueOf(2))
        assertEquals(queue.dequeue(), Integer.valueOf(3))
    }

    @Test
    fun testDequeue() {
        assertNull(queue.dequeue())
    }

    @Test(expectedExceptions = [NoSuchElementException::class])
    fun testIterator() {
        var i = 1
        queue.enqueue(1)
        queue.enqueue(2)
        queue.enqueue(3)
        for (item in queue) {
            assertEquals(item, Integer.valueOf(i++))
        }
        LinkedListQueue<Int>().iterator().next()
    }
}