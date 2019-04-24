package algorithm.set

import org.testng.Assert.assertEquals
import org.testng.annotations.BeforeMethod
import org.testng.annotations.Test
import java.io.File

class QuickUnionUFTest {
    private lateinit var uf: QuickUnionUF

    @BeforeMethod
    fun setUp() {
        val file = File("src/test/kotlin/algorithm/set/mediumUF.txt")
        val reader = file.bufferedReader()
        uf = QuickUnionUF(reader.readLine().toInt())
        while (true) {
            reader.readLine()?.let {
                val l = it.split(' ')
                val p = l[0].toInt()
                val q = l[1].toInt()
                if (uf.connected(p, q)) return@let
                uf.union(p, q)
            } ?: break
        }
    }

    @Test
    fun test() {
        assertEquals(3, uf.count())
    }
}