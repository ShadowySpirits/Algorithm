import stack.LinkedListStack

class Decimal2Binary {
    companion object {
        fun dec2bin(arg: Int): String {
            var dec = arg
            val stack = LinkedListStack<Int>()
            while (dec > 0) {
                stack.push(dec % 2)
                dec /= 2
            }
            var str = ""
            for (item in stack) {
                str += item
            }
            return str
        }

        fun dec2bin(arg: String): String {
            var dec = arg.toInt()
            val stack = LinkedListStack<Int>()
            while (dec > 0) {
                stack.push(dec % 2)
                dec /= 2
            }
            var str = ""
            for (item in stack) {
                str += item
            }
            return str
        }
    }
}
