use std::{error::Error, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PluginInfo {
    pub id: String,
    pub name: String,
    pub description: String,

    #[serde(default)]
    pub settings: Option<Vec<PluginSetting>>,

    #[serde(default)]
    pub dir: Option<PathBuf>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PluginSetting {
    Text(TextSetting),
    Number(NumberSetting),
    Select(SelectSetting),
    Check(CheckSetting),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextSetting {
    pub id: String,
    pub default_value: String,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NumberSetting {
    pub id: String,
    pub default_value: usize,
    pub min: usize,
    pub max: usize,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SelectSetting {
    pub id: String,
    pub default_value: String,
    pub options: Vec<SelectOption>,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SelectOption {
    pub id: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheckSetting {
    pub id: String,
    pub default_value: bool,
    pub title: String,
    pub description: Option<String>,
}

pub fn get_plugins() -> Result<Vec<PluginInfo>, Box<dyn Error>> {
    let mut dir = dirs::data_local_dir().ok_or_else(|| "Failed to get local dir")?;
    dir.push("mordomo/plugins");

    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }

    let plugins_dirs: Vec<PathBuf> = fs::read_dir(&dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_dir())
        .map(|entry| entry.path())
        .collect();

    let plugins: Vec<PluginInfo> = plugins_dirs
        .iter()
        .filter_map(|dir| {
            let info_path = dir.join("info.json");
            let bytes = fs::read(&info_path).ok()?;
            let mut info = serde_json::from_slice::<PluginInfo>(&bytes).ok()?;

            info.dir = Some(dir.to_owned());

            Some(info)
        })
        .collect();

    Ok(plugins)
}
