package algorithm.sort.array

fun mergeSort(a: Array<Int>) {
    mergeSortInternally(a, 0, a.size - 1)
}

private fun mergeSortInternally(a: Array<Int>, f: Int, e: Int) {
    if (f == e) return
    val h = f + (e - f) / 2
    mergeSortInternally(a, f, h)
    mergeSortInternally(a, h + 1, e)
    merge(a, f, h, e)
}

private fun merge(a: Array<Int>, f: Int, h: Int, e: Int) {
    val size = e - f + 1
    var indexA = f
    var indexB = h + 1
    System.arraycopy(Array(size) {
        if (indexA > h) {
            return@Array a[indexB++]
        }

        if (indexB > e) {
            return@Array a[indexA++]
        }

        if (a[indexA] < a[indexB]) {
            return@Array a[indexA++]
        }
        return@Array a[indexB++]
    }, 0, a, f, size)
}