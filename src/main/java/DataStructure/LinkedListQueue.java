package DataStructure;

import edu.umd.cs.findbugs.annotations.SuppressFBWarnings;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

import java.util.Iterator;
import java.util.NoSuchElementException;

/**
 * A queue implement with Linked List
 *
 * @author ShadowySpirits
 */
public class LinkedListQueue<Item> implements Queue<Item> {

    private Node first;
    private Node last;
    private int n;

    private class Node {
        Item item;
        Node next;

        Node(Item item) {
            this.item = item;
            if (isEmpty()) {
                first = last = this;
                return;
            }
            last.next = this;
        }
    }

    @Override
    public boolean isEmpty() {
        return first == null;
    }

    @Override
    public int size() {
        return n;
    }

    @Override
    public void enqueue(Item item) {
        last = new Node(item);
        n++;
    }

    @Nullable
    @Override
    public Item dequeue() {
        if (isEmpty()) {
            return null;
        }
        Node old = first;
        first = first.next;
        if (--n == 0) {
            last = null;
        }
        return old.item;
    }

    private class ItemIterable implements Iterator<Item> {

        Node current = first;

        @Override
        public boolean hasNext() {
            return current != null;
        }

        @SuppressWarnings("Duplicates")
        @Override
        public Item next() {
            if (!hasNext()) {
                throw new NoSuchElementException();
            }
            Item item = current.item;
            current = current.next;
            return item;
        }
    }

    @SuppressFBWarnings("RCN_REDUNDANT_NULLCHECK_OF_NONNULL_VALUE")
    @NotNull
    @Override
    public Iterator<Item> iterator() {
        return new ItemIterable();
    }
}
