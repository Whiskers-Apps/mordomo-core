package lib

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
data class PluginManifest(
    val id: String,
    val name: String,
    val description: String,
    val settings: List<PluginSetting> = emptyList()
)

@Serializable
sealed interface PluginSetting

@Serializable
@SerialName("text")
data class TextSetting(
    val id: String,
    val title: String,
    val description: String,
    val value: String
) : PluginSetting

@Serializable
@SerialName("number")
data class NumberSetting(
    val id: String,
    val title: String,
    val description: String,
    val value: Float
) : PluginSetting


@Serializable
@SerialName("check")
data class CheckSetting(
    val id: String,
    val title: String,
    val description: String,
    val value: Boolean
) : PluginSetting


@Serializable
@SerialName("select")
data class SelectSetting(
    val id: String,
    val title: String,
    val description: String,
    val defaultOptionId: String,
    val options: List<Option> = emptyList()
) : PluginSetting {

    @Serializable
    data class Option(
        val id: String,
        val text: String
    )
}