package lib

import kotlinx.serialization.Serializable


@Serializable
data class Settings(
    /// Map<PluginId, Map<SettingId, Value>>
    val pluginsSettings: Map<String, Map<String, String>> = emptyMap(),
    val searchKeyword: String? = "s",
    val defaultSearchEngine: Int? = null,
    val searchEngines: List<SearchEngine> = listOf(
        SearchEngine(
            id = 1,
            name = "DuckDuckGo (No AI)",
            query = "https://noai.duckduckgo.com/?q=%s",
            keyword = "!d"
        ),
        SearchEngine(
            id = 2,
            name = "Google",
            query = "https://www.google.com/search?q=%s",
            keyword = "!g"
        ),
        SearchEngine(
            id = 3,
            name = "Ecosia",
            query = "https://www.ecosia.org/search?q=%s",
            keyword = "!e"
        ),
        SearchEngine(
            id = 4,
            name = "Startpage",
            query = "https://www.startpage.com/sp/search?query=%s",
            keyword = "!s"
        )
    ),
    val theme: Theme = Theme(),
    val hideFooter: Boolean = false,
    val drawBorder: Boolean = false,
)

@Serializable
data class Theme(
    val dark: Boolean = false,
    val main: String = "#F6F6F6",
    val secondary: String = "#EBEBEB",
    val tertiary: String = "#D8D8D8",
    val textMain: String = "#000000",
    val textSecondary: String = "#1A1A1A",
    val textDisabled: String = "#606060",
    val accent: String = "#9A7100",
    val onAccent: String = "#FFFFFF",
    val danger: String = "#9A0000",
    val onDanger: String = "#FFFFFF",
)

@Serializable
data class SearchEngine(
    val id: Int,
    val name: String,
    val query: String,
    val keyword: String?,
)