import DataStructure.LinkedListStack

class CheckBracketMatching {

    companion object {
        private const val OPEN_PARENTHESIS = '('
        private const val CLOSE_PARENTHESIS = ')'
        private const val OPEN_BRACKET = '['
        private const val CLOSE_BRACKET = ']'
        private const val OPEN_BRACE = '{'
        private const val CLOSE_BRACE = '}'

        fun checkBracketMatching(string: String): Boolean {
            val stack = LinkedListStack<Char>()
            loop@ for (s in string) {
                when (s) {
                    OPEN_BRACE, OPEN_BRACKET, OPEN_PARENTHESIS -> stack.push(s)
                    CLOSE_PARENTHESIS -> stack.pop()?.let {
                        if (it != OPEN_PARENTHESIS) {
                            return false
                        }
                    } ?: run { return false }
                    CLOSE_BRACKET -> stack.pop()?.let {
                        if (it != OPEN_BRACKET) {
                            return false
                        }
                    } ?: run { return false }
                    CLOSE_BRACE -> stack.pop()?.let {
                        if (it != OPEN_BRACE) {
                            return false
                        }
                    } ?: run { return false }
                    else -> {
                        continue@loop
                    }
                }
            }
            if (!stack.isEmpty) {
                return false
            }
            return true
        }
    }
}
