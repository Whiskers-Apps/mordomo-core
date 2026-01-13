use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IpcMessage {
    Main(MainMessage),
    Plugin(PluginMessage),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MainMessage {
    Show,
    Entries(Vec<Entry>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub text: String,
    pub subtext: Option<String>,
    pub icon_path: Option<PathBuf>,
    pub custom_info: Option<Vec<String>>,
    pub action: Option<Action>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginMessage {
    GetEntries(GetEntriesMessage),
    RunCustomAction(RunCustomActionMessage),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEntriesMessage {
    pub plugin_id: String,
    pub search_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunCustomActionMessage {
    pub plugin_id: String,
    pub action: String,
    pub custom_info: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Action {
    /// Opens an app given it's desktop file
    OpenApp(OpenApp),

    /// Open a file/folder
    OpenFile(OpenFile),

    /// Open URL in default browser
    OpenURL(OpenURL),

    /// Copy text to clipboard
    CopyText(CopyText),

    /// Copy image to clipboard
    CopyImage(CopyImage),

    /// Useful for menus
    ShowEntries(ShowEntries),

    /// Extension
    RunOnPlugin(RunOnPlugin),

    /// This is only for the app
    Core,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenApp {
    pub path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenFile {
    pub path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenURL {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CopyText {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CopyImage {
    pub image_path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShowEntries {
    pub entries: Vec<Entry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RunOnPlugin {
    pub plugin_id: String,
    pub action: String,
    pub custom_info: Option<Vec<String>>,
}

// ----------------------------------------------------------------------------------- //

impl Entry {
    pub fn new<S: AsRef<str>>(text: S) -> Self {
        Self {
            text: text.as_ref().to_owned(),
            subtext: None,
            icon_path: None,
            custom_info: None,
            action: None,
        }
    }

    pub fn set_subtext<S: AsRef<str>>(&mut self, text: S) -> Self {
        self.subtext = Some(text.as_ref().to_string());
        self.clone()
    }

    pub fn set_icon_path<P: AsRef<Path>>(&mut self, path: P) -> Self {
        self.icon_path = Some(path.as_ref().to_path_buf());
        self.clone()
    }

    pub fn add_custom_info<S: AsRef<str>>(&mut self, info: S) -> Self {
        let info = info.as_ref().to_string();

        self.custom_info = if let Some(mut custom_info) = self.custom_info.to_owned() {
            custom_info.push(info);
            Some(custom_info)
        } else {
            Some(vec![info])
        };

        self.clone()
    }

    pub fn set_action(&mut self, action: Action) -> Self {
        self.action = Some(action);
        self.clone()
    }
}

impl OpenApp {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }
}

impl OpenFile {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }
}

impl RunOnPlugin {
    pub fn new<S: AsRef<str>>(plugin_id: S, action: S) -> Self {
        Self {
            plugin_id: plugin_id.as_ref().to_string(),
            action: action.as_ref().to_string(),
            custom_info: None,
        }
    }

    pub fn add_custom_info<S: AsRef<String>>(&mut self, info: S) -> Self {
        self.custom_info = if let Some(mut custom_info) = self.custom_info.clone() {
            custom_info.push(info.as_ref().to_string());
            Some(custom_info)
        } else {
            Some(vec![info.as_ref().to_string()])
        };

        self.to_owned()
    }
}
