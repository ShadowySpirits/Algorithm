package stack;

import edu.umd.cs.findbugs.annotations.SuppressFBWarnings;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

import java.util.Iterator;
import java.util.NoSuchElementException;

/**
 * A stack implement with Linked List
 *
 * @author ShadowySpirits
 */
public class LinkedListStack<Item> implements Stack<Item> {

    private Node top;
    private int n;

    @Override
    public boolean isEmpty() {
        return top == null;
    }

    @Override
    public int size() {
        return n;
    }

    @Override
    public void push(Item item) {
        top = new Node(item);
        n++;
    }

    @Nullable
    @Override
    public Item pop() {
        if (top == null) {
            return null;
        }
        Node old = top;
        top = old.next;
        n--;
        return old.item;
    }

    @Nullable
    @Override
    public Item peek() {
        if (top == null) {
            return null;
        }
        return top.item;
    }

    @SuppressFBWarnings("RCN_REDUNDANT_NULLCHECK_OF_NONNULL_VALUE")
    @NotNull
    @Override
    public Iterator<Item> iterator() {
        return new ItemIterator();
    }

    private class Node {

        Item item;
        Node next;

        Node(Item item) {
            this.item = item;
            next = top;
        }
    }

    private class ItemIterator implements Iterator<Item> {

        private Node current = top;

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
}
