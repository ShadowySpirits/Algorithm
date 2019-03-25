package DataStructure

import org.testng.Assert.*
import org.testng.annotations.BeforeMethod
import org.testng.annotations.Test

class ResizingArrayStackTest {

    private lateinit var stack: ResizingArrayStack<Int>

    fun function(array: IntArray) {}

    @BeforeMethod
    fun setUp() {
        stack = ResizingArrayStack()
    }

    @Test
    fun testIsEmpty() {
        assertTrue(stack.isEmpty)
        stack.push(1)
        assertFalse(stack.isEmpty)
    }

    @Test
    fun testSize() {
        assertEquals(stack.size(), 0)
        stack.push(1)
        assertEquals(stack.size(), 1)
        stack.push(1)
        assertEquals(stack.size(), 2)
    }

    @Test
    fun testPush() {
        stack.push(1)
        assertEquals(stack.pop(), Integer.valueOf(1))
        stack.push(1)
        stack.push(2)
        stack.push(3)
        assertEquals(stack.peek(), Integer.valueOf(3))
        assertEquals(stack.pop(), Integer.valueOf(3))
        assertEquals(stack.peek(), Integer.valueOf(2))
        assertEquals(stack.pop(), Integer.valueOf(2))
        assertEquals(stack.peek(), Integer.valueOf(1))
        assertEquals(stack.pop(), Integer.valueOf(1))
    }

    @Test
    fun testPop() {
        assertNull(stack.pop())
    }

    @Test
    fun testPeek() {
        assertNull(stack.peek())
    }

    @Test
    fun testIterator() {
        var i = 3
        stack.push(1)
        stack.push(2)
        stack.push(3)
        for (item in stack) {
            assertEquals(item, Integer.valueOf(i--))
        }
    }
}