package lib

import kotlinx.serialization.Serializable

@Serializable
sealed interface Action

@Serializable
data class OpenApp(
    val text: String,
    val path: String
) : Action

@Serializable
data class OpenUrl(
    val text: String,
    val url: String
) : Action

@Serializable
data class CopyText(
    val text: String,
    val textCopy: String
) : Action

@Serializable
data class CopyImage(
    val text: String,
    val path: String
) : Action

@Serializable
data class ShowEntries(
    val text: String,
    val entries: List<Entry>
) : Action

@Serializable
data class Plugin(
    val text: String = "",
    val pluginId: String,
    val action: String,
    val customInfo: List<String> = emptyList()
) : Action

@Serializable
data class Form(
    val pluginId: String,
    val title: String,
    val buttonText: String,
    val inputs: List<FormInput>,
    val customInfo: List<String> = emptyList()
): Action

@Serializable
sealed interface FormInput

@Serializable
data class TextInput(
    val id: String,
    val title: String,
    val description: String,
    val value: String,
    val customInfo: List<String> = emptyList()
): FormInput

@Serializable
data class NumberInput(
    val id: String,
    val title: String,
    val description: String,
    val value: Int,
    val customInfo: List<String> = emptyList()
): FormInput

@Serializable
data class SelectInput(
    val id: String,
    val title: String,
    val description: String,
    val value: String,
    val options: List<SelectOption>,
    val customInfo: List<String> = emptyList()
): FormInput

@Serializable
data class SelectOption(
    val id: String,
    val text: String,
)

@Serializable
data class CheckInput(
    val id: String,
    val title: String,
    val description: String,
    val value: Boolean,
    val customInfo: List<String> = emptyList()
): FormInput

@Serializable
data class PathInput(
    val id: String,
    val title: String,
    val description: String,
    val value: Boolean,
    val selectFolder: Boolean,
    val fileExtensions: List<String>,
    val customInfo: List<String> = emptyList()
): FormInput