package lib

import kotlinx.serialization.Serializable

@Serializable
data class Entry(
    val image: String? = null,
    val title: String,
    val description: String? = null,
    val action: Action? = null,
)