import org.testng.Assert.assertFalse
import org.testng.Assert.assertTrue
import org.testng.annotations.Test

class CheckBracketMatchingTest {

    @Test
    fun testCheckBracketMatching() {
        val str1 = "[()]{}{[()()]()}"
        val str2 = "([)]"
        val str3 = "(]"
        val str4 = "(}"
        val str5 = "["
        val str6 = "]"
        val str7 = "[(123 + 1)]"

        assertTrue(CheckBracketMatching.checkBracketMatching(str1))
        assertFalse(CheckBracketMatching.checkBracketMatching(str2))
        assertFalse(CheckBracketMatching.checkBracketMatching(str3))
        assertFalse(CheckBracketMatching.checkBracketMatching(str4))
        assertFalse(CheckBracketMatching.checkBracketMatching(str5))
        assertFalse(CheckBracketMatching.checkBracketMatching(str6))
        assertTrue(CheckBracketMatching.checkBracketMatching(str7))

        CheckBracketMatching() // ensure 100% line coverage
    }
}