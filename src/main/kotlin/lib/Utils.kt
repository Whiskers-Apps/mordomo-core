package lib

import java.io.File

fun getJarPath(): File {
    return File(object {}.javaClass.protectionDomain.codeSource.location.toURI()).parentFile
}