use std::{error::Error, fs, path::PathBuf};

use dirs::cache_dir;
use serde::{Deserialize, Serialize};

use crate::plugins::{PluginInfo, get_plugins};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct Settings {
    pub width: u16,
    pub height: u16,
    pub theme: Theme,
    pub keywords: Vec<Keyword>,
    pub plugins: Vec<PluginSetting>,
    pub search_engines: Vec<SearchEngine>,
    pub default_engine: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct Theme {
    pub dark: bool,
    pub main: String,
    pub secondary: String,
    pub tertiary: String,
    pub text_main: String,
    pub text_secondary: String,
    pub text_disabled: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Keyword {
    pub plugin_id: String,
    pub keyword: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PluginSetting {
    pub plugin_id: String,
    pub setting_id: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchEngine {
    pub id: u16,
    pub keyword: String,
    pub name: String,
    pub query: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            width: 600,
            height: 400,
            theme: Theme::default(),
            keywords: vec![],
            plugins: vec![],
            search_engines: vec![
                SearchEngine {
                    id: 0,
                    keyword: String::from("ec"),
                    name: String::from("Ecosia"),
                    query: String::from("https://www.ecosia.org/search?q=%s"),
                },
                SearchEngine {
                    id: 1,
                    keyword: String::from("dd"),
                    name: String::from("DuckDuckGo"),
                    query: String::from("https://duckduckgo.com/?q=%s"),
                },
                SearchEngine {
                    id: 2,
                    keyword: String::from("g"),
                    name: String::from("Google"),
                    query: String::from("https://www.google.com/search?q=%s"),
                },
            ],
            default_engine: Some(2),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            dark: true,
            main: String::from("#141414"),
            secondary: String::from("#1F1F1F"),
            tertiary: String::from("#383838"),
            text_main: String::from("#F2F2F2"),
            text_secondary: String::from("#E5E5E5"),
            text_disabled: String::from("#9F9F9F"),
        }
    }
}

impl Settings {
    pub fn get() -> Result<Settings, Box<dyn Error>> {
        let json = fs::read(Settings::get_path()?)?;
        let settings = serde_json::from_slice(&json)?;

        Ok(settings)
    }

    pub fn save(self: &Self) -> Result<(), Box<dyn Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(Settings::get_path()?, &json)?;
        Ok(())
    }

    pub fn get_path() -> Result<PathBuf, Box<dyn Error>> {
        let mut settings_path =
            dirs::config_dir().ok_or_else(|| String::from("Failed to get config dir"))?;

        settings_path.push("mordomo");

        if !settings_path.exists() {
            fs::create_dir_all(&settings_path)?;
        }

        settings_path.push("settings.json");

        Ok(settings_path)
    }
}

impl SearchEngine {
    pub fn get_icon(&self) -> Result<PathBuf, Box<dyn Error>> {
        let path = cache_dir()
            .ok_or_else(|| "Failed to get cache dir")?
            .join("mordomo")
            .join("search-engine-icons")
            .join(self.id.to_string());

        Ok(path)
    }
}

// --------------------------------------------------------------- //

pub fn is_dark_theme() -> Result<bool, Box<dyn Error>> {
    let mut settings_path =
        dirs::config_dir().ok_or_else(|| String::from("Failed to get config dir"))?;

    settings_path.push("mordomo/settings.json");

    let json = fs::read(&settings_path)?;
    let user_settings: Settings = serde_json::from_slice(&json)?;

    Ok(user_settings.theme.dark)
}

pub fn get_plugin_setting<S: AsRef<str>>(
    plugin_id: S,
    setting_id: S,
) -> Result<String, Box<dyn Error>> {
    let plugin_id = plugin_id.as_ref();
    let setting_id = setting_id.as_ref();

    if let Some(setting) = Settings::get()?
        .plugins
        .into_iter()
        .find(|p| p.plugin_id == plugin_id && p.setting_id == setting_id)
    {
        return Ok(setting.value);
    } else {
        if let Some(plugin) = get_plugins()?.iter().find(|p| p.id == plugin_id) {
            if let Some(settings) = plugin.settings.to_owned() {
                if let Some(setting) = settings.iter().find(|s| match s {
                    crate::plugins::PluginSetting::Text(text_setting) => {
                        text_setting.id == setting_id
                    }
                    crate::plugins::PluginSetting::Number(number_setting) => {
                        number_setting.id == setting_id
                    }
                    crate::plugins::PluginSetting::Select(select_setting) => {
                        select_setting.id == setting_id
                    }
                    crate::plugins::PluginSetting::Check(check_setting) => {
                        check_setting.id == setting_id
                    }
                }) {
                    return Ok(match setting {
                        crate::plugins::PluginSetting::Text(text_setting) => {
                            text_setting.default_value.to_owned()
                        }
                        crate::plugins::PluginSetting::Number(number_setting) => {
                            number_setting.default_value.to_string()
                        }
                        crate::plugins::PluginSetting::Select(select_setting) => {
                            select_setting.default_value.to_owned()
                        }
                        crate::plugins::PluginSetting::Check(check_setting) => {
                            check_setting.default_value.to_string()
                        }
                    });
                }
            }
        }
    }

    Err("Could not find setting".into())
}

pub fn get_plugin_usize_setting<S: AsRef<str>>(
    plugin_id: S,
    setting_id: S,
) -> Result<usize, Box<dyn Error>> {
    Ok(get_plugin_setting(plugin_id.as_ref(), setting_id.as_ref())?.parse::<usize>()?)
}

pub fn get_plugin_bool_setting<S: AsRef<str>>(
    plugin_id: S,
    setting_id: S,
) -> Result<bool, Box<dyn Error>> {
    let setting = get_plugin_setting(plugin_id.as_ref(), setting_id.as_ref())?;

    match setting.as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err("Setting is not a boolean".into()),
    }
}
