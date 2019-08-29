package algorithm.sort.array

fun <T : Comparable<T>> bubbleSort(a: Array<T>) {
    if (a.size <= 1) return
    var exchangeBorder = a.size - 1
    for (i in a.indices) {
        var flag = false
        for (j in 0 until exchangeBorder) {
            if (a[j] > a[j + 1]) {
                flag = true
                exchangeBorder = j
                val t = a[j]
                a[j] = a[j + 1]
                a[j + 1] = t
            }
        }
        if (!flag) break
    }
}
