package DataStructure;

import org.jetbrains.annotations.Contract;
import org.jetbrains.annotations.Nullable;

/**
 * A LIFO(last-in, first-out) data  structure
 *
 * @author ShadowySpirits
 */
public interface Stack<Item> extends Iterable<Item> {

    @Contract(pure = true)
    boolean isEmpty();

    @Contract(pure = true)
    int size();

    void push(Item item);

    @Nullable
    Item pop();

    @Nullable
    Item peek();
}
