use std::{env, error::Error};

use postcard::from_bytes;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::core::{
    Entry, FormSubmittedMessage, GetEntriesMessage, PluginMessage, RunCustomActionMessage,
};

pub async fn handle_plugin_messages<F, A>(
    id: &str,
    on_message: F,
    on_action: A,
    on_form_submitted: F,
) -> Result<(), Box<dyn Error>>
where
    F: Fn(GetEntriesMessage) -> Vec<Entry>,
    A: Fn(RunCustomActionMessage),
    F: Fn(FormSubmittedMessage),
{
    let args: Vec<String> = env::args().collect();
    let port = args.get(1).expect("Failed to get port");
    let connection_url = format!("127.0.0.1:{port}");
    let mut stream = TcpStream::connect(connection_url).await?;

    let mut buffer = [0u8; 1024 * 300];

    loop {
        let n = stream.read(&mut buffer).await?;

        if n == 0 {
            return Ok(());
        }

        if let Ok(message) = from_bytes::<PluginMessage>(&buffer[..n]) {
            match message {
                PluginMessage::GetEntries(get_entries_message) => {
                    if get_entries_message.plugin_id == id {
                        let entries = on_message(get_entries_message);
                        let bytes = postcard::to_allocvec(&entries)?;

                        stream.write_all(&bytes).await?;
                    }
                }
                PluginMessage::RunCustomAction(action_message) => {
                    if action_message.plugin_id == id {
                        on_action(action_message);
                    }
                }
                PluginMessage::FormSubmitted(form_submitted_message) => {
                    if form_submitted_message.plugin_id == id {
                        on_form_submitted(form_submitted_message);
                    }
                }
            }
        }
    }
}
