package algorithm.sort.array

import org.testng.annotations.BeforeMethod
import org.testng.annotations.Test
import java.util.*

class SortTest {
    var a = emptyArray<Int>()

    companion object {
        private const val ARRAY_COUNT = 10000
    }

    private fun getSortList(): Array<Int> {
        val ra = Random()
        return Array(ARRAY_COUNT) {
            return@Array ra.nextInt(ARRAY_COUNT * 10)
        }
    }

    @BeforeMethod
    fun setUp() {
        a = getSortList()
    }

    @Test
    fun testBubbleSort() {
        val b = a.copyOf()
        bubbleSort(a)
        b.sort()
        assert(a.contentEquals(b))
    }

    @Test
    fun testInsertSort() {
        val b = a.copyOf()
        insertionSort(a)
        b.sort()
        assert(a.contentEquals(b))
    }

    @Test
    fun testShellSort() {
        val b = a.copyOf()
        shellSort(a)
        b.sort()
        assert(a.contentEquals(b))
    }

    @Test
    fun testSelectionSort() {
        val b = a.copyOf()
        selectionSort(a)
        b.sort()
        assert(a.contentEquals(b))
    }
}
