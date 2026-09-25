package lib

import kotlinx.serialization.Serializable

@Serializable
sealed interface FormResult

@Serializable
data class TextFormResult(
    val id: String,
    val value: String,
    val customInfo: List<String> = emptyList()
): FormResult

@Serializable
data class NumberFormResult(
    val id: String,
    val value: Int,
    val customInfo: List<String> = emptyList()
): FormResult

@Serializable
data class CheckFormResult(
    val id: String,
    val value: Boolean,
    val customInfo: List<String> = emptyList()
): FormResult

@Serializable
data class PathFormResult(
    val id: String,
    val value: String?,
    val customInfo: List<String> = emptyList()
): FormResult

@Serializable
data class SelectFormResult(
    val id: String,
    val value: String,
    val customInfo: List<String> = emptyList()
): FormResult

fun List<FormResult>.getTextResult(id: String): TextFormResult? {
    for(result in this) {
        when(result) {
            is TextFormResult -> { if(result.id == id) return result }
            else -> continue
        }
    }

    return null
}

fun List<FormResult>.getNumberResult(id: String): NumberFormResult? {
    for(result in this) {
        when(result) {
            is NumberFormResult -> { if(result.id == id) return result }
            else -> continue
        }
    }

    return null
}


fun List<FormResult>.getCheckResult(id: String): CheckFormResult? {
    for(result in this) {
        when(result) {
            is CheckFormResult -> { if(result.id == id) return result }
            else -> continue
        }
    }

    return null
}

fun List<FormResult>.getSelectResult(id: String): SelectFormResult? {
    for(result in this) {
        when(result) {
            is SelectFormResult -> { if(result.id == id) return result }
            else -> continue
        }
    }

    return null
}

fun List<FormResult>.getPathResult(id: String): PathFormResult? {
    for(result in this) {
        when(result) {
            is PathFormResult -> { if(result.id == id) return result }
            else -> continue
        }
    }

    return null
}