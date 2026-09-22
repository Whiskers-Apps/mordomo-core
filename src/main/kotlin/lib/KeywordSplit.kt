package lib

class KeywordSplit {
    var keyword: String? = null
    var searchText: String? = null
    var fullText: String = ""

    constructor(text: String) {
        this.fullText = text

        val textParts = text.split(" ")

        this.keyword = if (textParts.size > 1)
            textParts[0]
        else
            null

        this.searchText = if (keyword != null)
            textParts.filterIndexed { index, _ -> index > 0 }.joinToString(" ")
        else
            null
    }
}
