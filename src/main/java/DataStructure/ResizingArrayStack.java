package DataStructure;

import edu.umd.cs.findbugs.annotations.SuppressFBWarnings;
import org.jetbrains.annotations.Contract;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

import java.util.Iterator;
import java.util.NoSuchElementException;

/**
 * A LIFO(last-in, first-out) data  structure implement by resizing array
 *
 * @author SSpirits
 */
public class ResizingArrayStack<Item> implements Iterable<Item> {
    @NotNull
    private Item[] a = (Item[]) new Object[1];
    private int n = 0;

    @Contract(pure = true)
    public boolean isEmpty() {
        return n == 0;
    }

    @Contract(pure = true)
    public int size() {
        return n;
    }

    private void resize(int len) {
        Item[] temp = (Item[]) new Object[len];
//        for (int i = 0; i < a.length; i++) {
//            temp[i] = a[i];
//        }
        // can use System.arraycopy() to instead manual array copy.
        System.arraycopy(a, 0, temp, 0, a.length);
        a = temp;
    }

    public void push(Item item) {
        if (n == a.length) {
            resize(n * 2);
        }
        a[n++] = item;
    }

    @Nullable
    public Item pop() {
        if (n <= 0) {
            return null;
        }
        Item item = a[--n];
        a[n] = null;
        if (n > 0 && n < a.length / 4) {
            resize(a.length / 2);
        }
        return item;
    }

    @Nullable
    public Item peek() {
        if (n <= 0) {
            return null;
        }
        return a[n - 1];
    }

    private class ItemIterator implements Iterator<Item> {

        private int i = n;

        @Override
        public boolean hasNext() {
            return i > 0;
        }

        @Override
        public Item next() {
            if (i <= 0) {
                throw new NoSuchElementException();
            }
            return a[--i];
        }
    }

    @SuppressFBWarnings("RCN_REDUNDANT_NULLCHECK_OF_NONNULL_VALUE")
    @NotNull
    @Override
    public Iterator<Item> iterator() {
        return new ItemIterator();
    }
}

