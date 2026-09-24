package lib

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.isActive
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import kotlinx.serialization.json.Json
import java.io.BufferedReader
import java.io.File
import java.io.InputStreamReader
import java.io.PrintWriter
import java.net.Socket
import java.nio.file.FileSystems
import java.nio.file.Path
import java.nio.file.StandardWatchEventKinds

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

    suspend fun listen(onMessageReceived: suspend (PluginMessage, Map<String, String>) -> Unit) =
        withContext(Dispatchers.IO) {
            connect()

            val pluginSettings: MutableMap<String, String> = mutableMapOf()

            launch {
                val watchService = FileSystems.getDefault().newWatchService()
                val configDir = File(System.getProperty("user.home")).resolve(".config/mordomo")
                val path = File(configDir, "settings.json")

                val jsonContent = path.readText()
                val settings: Settings = Json.decodeFromString(jsonContent)

                settings.pluginsSettings[pluginId]?.forEach { (settingId, settingsValue) ->
                    pluginSettings[settingId] = settingsValue
                }

                try {
                    configDir.toPath().register(watchService, StandardWatchEventKinds.ENTRY_MODIFY)
                } catch (e: Exception) {
                    println("Failed to register watch service. $e")
                    return@launch
                }

                while (isActive) {
                    val watchKey = try {
                        watchService.take()
                    } catch (e: Exception) {
                        println("Failed to watch settings file. $e")
                        break
                    }

                    for (event in watchKey.pollEvents()) {
                        val changedFile = event.context() as? Path ?: continue
                        if (changedFile.toString() != "settings.json") continue

                        val jsonContent = path.readText()
                        val settings: Settings = Json.decodeFromString(jsonContent)

                        settings.pluginsSettings[pluginId]?.forEach { (settingId, settingsValue) ->
                            pluginSettings[settingId] = settingsValue
                        }
                    }

                    watchKey.reset()
                }
            }

            while (true) {
                try {
                    val message = receiver.readLine() ?: break
                    val pluginMessage: PluginMessage = Json.decodeFromString(message)

                    launch {
                        onMessageReceived(pluginMessage, pluginSettings)
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