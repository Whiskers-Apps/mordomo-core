package lib

import kotlinx.serialization.Serializable

@Serializable
sealed interface FormResult

@Serializable
data class TextFormResult(
    val id: String,
    val value: String,
    val info: List<String> = emptyList()
): FormResult

@Serializable
data class NumberFormResult(
    val id: String,
    val value: Float,
    val info: List<String> = emptyList()
): FormResult

@Serializable
data class CheckFormResult(
    val id: String,
    val value: Boolean,
    val info: List<String> = emptyList()
): FormResult

@Serializable
data class PathFormResult(
    val id: String,
    val value: String,
    val info: List<String> = emptyList()
): FormResult

@Serializable
data class SelectFormResult(
    val id: String,
    val value: String,
    val info: List<String> = emptyList()
): FormResult