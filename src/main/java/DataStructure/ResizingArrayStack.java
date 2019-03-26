package DataStructure;

import edu.umd.cs.findbugs.annotations.SuppressFBWarnings;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

import java.util.Iterator;
import java.util.NoSuchElementException;

/**
 * A stack implement with resizing array
 *
 * @author ShadowySpirits
 */
public class ResizingArrayStack<Item> implements Stack<Item> {

    @SuppressWarnings("unchecked")
    private Item[] a = (Item[]) new Object[1];
    private int n;

    @Override
    public boolean isEmpty() {
        return n == 0;
    }

    @Override
    public int size() {
        return n;
    }

    private void resize(int len) {
        @SuppressWarnings("unchecked")
        Item[] temp = (Item[]) new Object[len];
//        for (int i = 0; i < a.length; i++) {
//            temp[i] = a[i];
//        }
        // can use System.arraycopy() to instead manual array copy.
        System.arraycopy(a, 0, temp, 0, n);
        a = temp;
    }

    @Override
    public void push(Item item) {
        if (n == a.length) {
            resize(n * 2);
        }
        a[n++] = item;
    }

    @Nullable
    @Override
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
    @Override
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
            if (!hasNext()) {
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

