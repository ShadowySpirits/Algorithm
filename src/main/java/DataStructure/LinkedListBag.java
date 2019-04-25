package DataStructure;

import edu.umd.cs.findbugs.annotations.SuppressFBWarnings;
import org.jetbrains.annotations.NotNull;

import java.util.Iterator;
import java.util.NoSuchElementException;

/**
 * A bag implement with Linked List
 *
 * @author ShadowySpirits
 */
public class LinkedListBag<Item> implements Bag<Item> {

    private Node top;
    private int n;

    private class Node {

        Item item;
        Node next;

        Node(Item item) {
            this.item = item;
            next = top;
        }
    }

    @Override
    public boolean isEmpty() {
        return top == null;
    }

    @Override
    public int size() {
        return n;
    }

    @Override
    public void add(Item item) {
        top = new Node(item);
        n++;
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

    @SuppressFBWarnings("RCN_REDUNDANT_NULLCHECK_OF_NONNULL_VALUE")
    @NotNull
    @Override
    public Iterator<Item> iterator() {
        return new ItemIterator();
    }
}