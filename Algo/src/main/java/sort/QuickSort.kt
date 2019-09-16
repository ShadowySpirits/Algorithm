package sort

fun <T : Comparable<T>> quickSort(a: Array<T>) {
    quickSortInternally(a, 0, a.size - 1)
}

private fun <T : Comparable<T>> quickSortInternally(a: Array<T>, f: Int, e: Int) {
    if (f >= e) return
    val pivot = partition(a, f, e)
    quickSortInternally(a, f, pivot - 1)
    quickSortInternally(a, pivot + 1, e)
}

private fun <T : Comparable<T>> partition(a: Array<T>, f: Int, e: Int): Int {
    var i = f
    for (j in f..e) {
        if (a[j] < a[e]) {
            val t = a[i]
            a[i] = a[j]
            a[j] = t
            i++
        }
    }
    val t = a[i]
    a[i] = a[e]
    a[e] = t
    return i
}
