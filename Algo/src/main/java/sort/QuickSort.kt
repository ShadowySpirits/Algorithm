package sort

private const val CUT_OFF = 15

fun <T : Comparable<T>> quickSort(a: Array<T>) {
    quickSortInternally(a, 0, a.size - 1)
}

private fun <T : Comparable<T>> quickSortInternally(a: Array<T>, f: Int, e: Int) {
    if (e - f <= CUT_OFF) {
        insertionSort(a, f, e)
        return
    }
    val pivot = partition(a, f, e)
    quickSortInternally(a, f, pivot - 1)
    quickSortInternally(a, pivot + 1, e)
}

private fun <T : Comparable<T>> partition(a: Array<T>, f: Int, e: Int): Int {
    val mid = (f + e) / 2
    if (a[mid] < a[f]) {
        exchange(a, mid, f)
    }
    if (a[e] < a[mid]) {
        exchange(a, mid, e)
    }
    if (a[e] < a[f]) {
        exchange(a, e, f)
    }
    exchange(a, mid, e)
    var i = f
    for (j in f until e) {
        if (a[j] < a[e]) {
            exchange(a, i, j)
            i++
        }
    }
    exchange(a, i, e)
    return i
}

private fun <T : Comparable<T>> exchange(a: Array<T>, i: Int, j: Int) {
    val t = a[i]
    a[i] = a[j]
    a[j] = t
}
