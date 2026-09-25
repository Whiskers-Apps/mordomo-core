package lib

import kotlinx.serialization.Serializable

@Serializable
sealed interface Action

@Serializable
data class OpenApp(
    val path: String,
    val confirm: Boolean = false,
) : Action

@Serializable
data class OpenUrl(
    val url: String,
    val confirm: Boolean = false,
) : Action

@Serializable
data class CopyText(
    val textCopy: String,
    val confirm: Boolean = false,
) : Action

@Serializable
data class CopyImage(
    val path: String,
    val confirm: Boolean = false,
) : Action

@Serializable
data class ShowEntries(
    val entries: List<Entry>,
    val confirm: Boolean = false,
) : Action

@Serializable
data class Plugin(
    val pluginId: String,
    val action: String,
    val customInfo: List<String> = emptyList(),
    val confirm: Boolean = false,
) : Action

@Serializable
data class Form(
    val pluginId: String,
    val title: String,
    val buttonText: String,
    val inputs: List<FormInput>,
    val customInfo: List<String> = emptyList(),
    val confirm: Boolean = false,
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
    val value: String? = null,
    val selectFolder: Boolean = false,
    val fileExtensions: List<String> = emptyList(),
    val customInfo: List<String> = emptyList()
): FormInput