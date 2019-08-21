package DataStructure

import org.testng.Assert.*
import org.testng.annotations.BeforeMethod
import org.testng.annotations.Test

class ResizingArrayDequeTest {
    private lateinit var queue: ResizingArrayDeque<Int>

    @BeforeMethod
    fun setUp() {
        queue = ResizingArrayDeque()
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
        queue.enqueue(1)
        assertEquals(queue.size(), 3)
        queue.enqueue(1)
        assertEquals(queue.size(), 4)
    }

    @Test
    fun testEnqueue() {
        queue.enqueue(1)
        assertEquals(queue.dequeue(), 1)
        queue.enqueue(1)
        assertEquals(queue.dequeue(), 1)
        queue.enqueue(1)
        assertEquals(queue.dequeue(), 1)
        queue.enqueue(1)
        queue.enqueue(2)
        queue.enqueue(3)
        queue.enqueue(4)
        assertEquals(queue.dequeue(), 1)
        assertEquals(queue.dequeue(), 2)
        assertEquals(queue.dequeue(), 3)
        assertEquals(queue.dequeue(), 4)
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
            assertEquals(item, i++)
        }
        ResizingArrayDeque<Int>().iterator().next()
    }
}
