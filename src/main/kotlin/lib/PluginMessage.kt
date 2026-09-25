package lib

import kotlinx.serialization.Serializable

@Serializable
sealed interface PluginMessage

@Serializable
data class GetEntries(
    val searchText: String
) : PluginMessage

@Serializable
data class RunAction(
    val actionId: String,
    val customInfo: List<String> = emptyList()
) : PluginMessage

@Serializable
data class FormResults(
    val results: List<FormResult>,
    val customInfo: List<String> = emptyList()
) : PluginMessage