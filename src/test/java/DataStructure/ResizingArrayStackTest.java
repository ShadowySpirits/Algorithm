package DataStructure;

import edu.umd.cs.findbugs.annotations.SuppressFBWarnings;
import org.testng.annotations.BeforeMethod;
import org.testng.annotations.Test;

import static org.testng.Assert.*;

@SuppressFBWarnings("PRMC_POSSIBLY_REDUNDANT_METHOD_CALLS")
public class ResizingArrayStackTest {

    private ResizingArrayStack<Integer> stack;

    @BeforeMethod
    public void setUp() {
        stack = new ResizingArrayStack<>();
    }

    @Test
    public void testIsEmpty() {
        assertTrue(stack.isEmpty());
        stack.push(1);
        assertFalse(stack.isEmpty());
    }

    @Test
    public void testSize() {
        assertEquals(stack.size(), 0);
        stack.push(1);
        assertEquals(stack.size(), 1);
        stack.push(1);
        assertEquals(stack.size(), 2);
    }

    @Test
    public void testPush() {
        stack.push(1);
        assertEquals(stack.pop(), Integer.valueOf(1));
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assertEquals(stack.peek(), Integer.valueOf(3));
        assertEquals(stack.pop(), Integer.valueOf(3));
        assertEquals(stack.peek(), Integer.valueOf(2));
        assertEquals(stack.pop(), Integer.valueOf(2));
        assertEquals(stack.peek(), Integer.valueOf(1));
        assertEquals(stack.pop(), Integer.valueOf(1));
    }

    @Test
    public void testPop() {
        assertNull(stack.pop());
    }

    @Test
    public void testPeek() {
        assertNull(stack.peek());
    }

    @Test
    public void testIterator() {
        int i = 3;
        stack.push(1);
        stack.push(2);
        stack.push(3);
        for (Integer item : stack) {
            assertEquals(item, Integer.valueOf(i--));
        }
    }
}