import org.testng.Assert.assertEquals
import org.testng.Assert.expectThrows
import org.testng.annotations.Test


class Decimal2BinaryTest {

    @Test
    fun testDecimal2Binary() {
        assertEquals(Decimal2Binary.dec2bin(50), "110010")
        assertEquals(Decimal2Binary.dec2bin("50"), "110010")
        expectThrows(NumberFormatException::class.java) { Decimal2Binary.dec2bin("a") }

        Decimal2Binary() // ensure 100% line coverage
    }
}