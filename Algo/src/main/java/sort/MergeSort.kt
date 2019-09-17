package sort

private const val CUT_OFF = 15

fun <T : Comparable<T>> mergeSort(a: Array<T>) {
    val aux = a.clone()
    mergeSortInternally(aux, a, 0, a.size - 1)
}

private fun <T : Comparable<T>> mergeSortInternally(a: Array<T>, aux: Array<T>, f: Int, e: Int) {
    if (e - f <= CUT_OFF) {
        insertionSort(aux, f, e)
        return
    }
    val h = f + (e - f) / 2
    mergeSortInternally(aux, a, f, h)
    mergeSortInternally(aux, a, h + 1, e)
    if (a[h] > a[h + 1]) {
        merge(a, aux, f, h, e)
        return
    }
    System.arraycopy(a, f, aux, f, e - f + 1)
}

private fun <T : Comparable<T>> merge(a: Array<T>, aux: Array<T>, f: Int, h: Int, e: Int) {
    var indexA = f
    var indexB = h + 1
    for (i in f..e) {
        when {
            indexA > h -> aux[i] = a[indexB++]
            indexB > e -> aux[i] = a[indexA++]
            a[indexA] < a[indexB] -> aux[i] = a[indexA++]
            else -> aux[i] = a[indexB++]
        }
    }
}
