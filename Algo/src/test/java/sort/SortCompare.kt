package sort

import org.testng.annotations.BeforeTest
import org.testng.annotations.Test
import java.util.*

class SortCompare {
    private lateinit var a: Array<Int>
    private lateinit var b: Array<Int>
    private lateinit var c: Array<Int>
    private lateinit var d: Array<Int>
    private lateinit var e: Array<Int>
    private lateinit var f: Array<Int>

    companion object {
        private const val ARRAY_COUNT = 10000
    }

    private fun getSortList(): Array<Int> {
        val ra = Random()
        return Array(ARRAY_COUNT) {
            return@Array ra.nextInt(ARRAY_COUNT * 10)
        }
    }

    @BeforeTest
    fun setUp() {
        a = getSortList()
        b = a.copyOf()
        c = a.copyOf()
        d = a.copyOf()
        e = a.copyOf()
        f = a.copyOf()
    }

    @Test
    fun testBubbleSort() {
        bubbleSort(a)
    }

    @Test
    fun testInsertSort() {
        insertionSort(b)
    }

    @Test
    fun testShellSort() {
        shellSort(c)
    }

    @Test
    fun testSelectionSort() {
        selectionSort(d)
    }

    @Test
    fun testMergeSort() {
        mergeSort(e)
    }

    @Test
    fun testQuickSort() {
        quickSort(f)
    }
}
