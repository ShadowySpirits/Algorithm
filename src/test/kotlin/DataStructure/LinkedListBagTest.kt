package DataStructure

import org.testng.Assert.*
import org.testng.annotations.BeforeMethod
import org.testng.annotations.Test

class LinkedListBagTest {
    private lateinit var stack: LinkedListBag<Int>

    @BeforeMethod
    fun setUp() {
        stack = LinkedListBag()
    }

    @Test
    fun testIsEmpty() {
        assertTrue(stack.isEmpty)
        stack.add(1)
        assertFalse(stack.isEmpty)
    }

    @Test
    fun testSize() {
        assertEquals(stack.size(), 0)
        stack.add(1)
        assertEquals(stack.size(), 1)
        stack.add(1)
        assertEquals(stack.size(), 2)
    }

    @Test
    fun testAdd() {
        stack.add(1)
        stack.add(2)
        stack.add(3)
    }

    @Test(expectedExceptions = [NoSuchElementException::class])
    fun testIterator() {
        var i = 3
        testAdd()
        for (item in stack) {
            assertEquals(item, Integer.valueOf(i--))
        }
        LinkedListBag<Int>().iterator().next()
    }
}