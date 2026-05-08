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
    FormSubmitted(FormSubmittedMessage),
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormSubmittedMessage {
    pub plugin_id: String,
    pub results: Vec<FormResult>,
    pub custom_info: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FormResult {
    TextFormResult(TextFormResult),
    NumberFormResult(NumberFormResult),
    CheckFormResult(CheckFormResult),
    PathFormResult(PathFormResult),
    SelectFormResult(SelectFormResult),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextFormResult {
    pub id: String,
    pub value: String,
    pub custom_info: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumberFormResult {
    pub id: String,
    pub value: usize,
    pub custom_info: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckFormResult {
    pub id: String,
    pub value: bool,
    pub custom_info: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathFormResult {
    pub id: String,
    pub value: Option<PathBuf>,
    pub custom_info: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectFormResult {
    pub id: String,
    pub value: String,
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

    Form(Form),
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Form {
    pub plugin_id: String,
    pub title: String,
    pub positive_button_text: String,
    pub entries: Vec<FormEntry>,
    pub custom_info: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FormEntry {
    TextEntry(TextEntry),
    NumberEntry(NumberEntry),
    CheckEntry(CheckEntry),
    SelectEntry(SelectEntry),
    PathEntry(PathEntry),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextEntry {
    pub id: String,
    pub title: String,
    pub description: String,
    pub value: String,
    pub custom_info: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NumberEntry {
    pub id: String,
    pub title: String,
    pub description: String,
    pub value: usize,
    pub custom_info: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SelectEntry {
    pub id: String,
    pub title: String,
    pub description: String,
    pub value: String,
    pub options: Vec<SelectEntryOption>,
    pub custom_info: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SelectEntryOption {
    pub id: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheckEntry {
    pub id: String,
    pub title: String,
    pub description: String,
    pub value: bool,
    pub custom_info: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PathEntry {
    pub id: String,
    pub title: String,
    pub description: String,
    pub value: Option<PathBuf>,
    pub select_folder: bool,
    pub file_extensions: Option<Vec<String>>,
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

impl OpenURL {
    pub fn new<S: AsRef<str>>(url: S) -> Self {
        Self {
            url: url.as_ref().to_string(),
        }
    }
}

impl CopyText {
    pub fn new<S: AsRef<str>>(text: S) -> Self {
        Self {
            text: text.as_ref().to_string(),
        }
    }
}

impl CopyImage {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            image_path: path.as_ref().to_path_buf(),
        }
    }
}

impl ShowEntries {
    pub fn new(entries: &[Entry]) -> Self {
        Self {
            entries: entries.to_vec(),
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

    pub fn add_custom_info<S: AsRef<str>>(&mut self, info: S) -> Self {
        self.custom_info = if let Some(mut custom_info) = self.custom_info.clone() {
            custom_info.push(info.as_ref().to_string());
            Some(custom_info)
        } else {
            Some(vec![info.as_ref().to_string()])
        };

        self.to_owned()
    }
}

impl Form {
    pub fn new<S: AsRef<str>>(plugin_id: S, title: S, positive_button_text: S) -> Self {
        Self {
            plugin_id: plugin_id.as_ref().to_string(),
            title: title.as_ref().to_string(),
            positive_button_text: positive_button_text.as_ref().to_string(),
            entries: Vec::new(),
            custom_info: None,
        }
    }

    pub fn add_entry(&mut self, entry: FormEntry) -> Self {
        self.entries.push(entry);
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
}

impl TextEntry {
    pub fn new<S: AsRef<str>>(id: S, title: S, description: S, value: S) -> Self {
        Self {
            id: id.as_ref().to_string(),
            title: title.as_ref().to_string(),
            description: description.as_ref().to_string(),
            value: value.as_ref().to_string(),
            custom_info: None,
        }
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
}

impl NumberEntry {
    pub fn new<S: AsRef<str>>(id: S, title: S, description: S, value: usize) -> Self {
        Self {
            id: id.as_ref().to_string(),
            title: title.as_ref().to_string(),
            description: description.as_ref().to_string(),
            value,
            custom_info: None,
        }
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
}

impl SelectEntry {
    pub fn new<S: AsRef<str>>(id: S, title: S, description: S, value: S) -> Self {
        Self {
            id: id.as_ref().to_string(),
            title: title.as_ref().to_string(),
            description: description.as_ref().to_string(),
            value: value.as_ref().to_string(),
            options: Vec::new(),
            custom_info: None,
        }
    }

    pub fn add_option(&mut self, option: SelectEntryOption) -> Self {
        self.options.push(option);
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
}

impl SelectEntryOption {
    pub fn new<S: AsRef<str>>(id: S, text: S) -> Self {
        Self {
            id: id.as_ref().to_string(),
            text: text.as_ref().to_string(),
        }
    }
}

impl CheckEntry {
    pub fn new<S: AsRef<str>>(id: S, title: S, description: S, value: bool) -> Self {
        Self {
            id: id.as_ref().to_string(),
            title: title.as_ref().to_string(),
            description: description.as_ref().to_string(),
            value,
            custom_info: None,
        }
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
}

impl PathEntry {
    pub fn new<S: AsRef<str>>(id: S, title: S, description: S) -> Self {
        Self {
            id: id.as_ref().to_string(),
            title: title.as_ref().to_string(),
            description: description.as_ref().to_string(),
            value: None,
            select_folder: false,
            file_extensions: None,
            custom_info: None,
        }
    }

    pub fn set_value<P: AsRef<Path>>(&mut self, path: P) -> Self {
        self.value = Some(path.as_ref().to_path_buf());
        self.clone()
    }

    pub fn set_select_folder(&mut self, select_folder: bool) -> Self {
        self.select_folder = select_folder;
        self.clone()
    }

    pub fn add_file_extension<S: AsRef<str>>(&mut self, extension: S) -> Self {
        let extension = extension.as_ref().to_string();
        self.file_extensions = if let Some(mut extensions) = self.file_extensions.to_owned() {
            extensions.push(extension);
            Some(extensions)
        } else {
            Some(vec![extension])
        };
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
}
