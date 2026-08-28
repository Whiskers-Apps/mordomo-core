package lib

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import kotlinx.serialization.json.Json
import java.io.BufferedReader
import java.io.File
import java.io.InputStreamReader
import java.io.PrintWriter
import java.net.Socket

class PluginHandler(val pluginId: String) {
    lateinit var transmitter: PrintWriter
    lateinit var receiver: BufferedReader

    suspend fun connect() = withContext(Dispatchers.IO) {
        val socketFile = File("/tmp/mordomo.port")

        if (!socketFile.exists())
            return@withContext

        val port = socketFile.readText().toIntOrNull() ?: return@withContext

        val socket = Socket("localhost", port)

        transmitter = PrintWriter(socket.getOutputStream(), true)
        receiver = BufferedReader(InputStreamReader(socket.getInputStream()))

        transmitter.println("ack $pluginId")
    }

    suspend fun listen(onMessageReceived: suspend (PluginMessage) -> Unit) = withContext(Dispatchers.IO) {
        connect()

        while (true) {
            try {
                val message = receiver.readLine() ?: break
                val pluginMessage: PluginMessage = Json.decodeFromString(message)

                launch {
                    onMessageReceived(pluginMessage)
                }
            } catch (_: Exception) {
                break
            }
        }
    }

    suspend fun sendEntries(entries: List<Entry>) = withContext(Dispatchers.IO) {
        transmitter.println(Json.encodeToString(entries))
    }
}