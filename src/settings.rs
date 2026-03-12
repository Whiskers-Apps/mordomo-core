use std::{error::Error, fs};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct Settings {
    pub width: u16,
    pub height: u16,
    pub theme: Theme,
    pub keywords: Vec<Keyword>,
    pub search_engines: Vec<SearchEngine>,
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
            ],
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

// --------------------------------------------------------------- //

pub fn is_dark_theme() -> Result<bool, Box<dyn Error>> {
    let mut settings_path =
        dirs::config_dir().ok_or_else(|| String::from("Failed to get config dir"))?;

    settings_path.push("mordomo/settings.json");

    let json = fs::read(&settings_path)?;
    let user_settings: Settings = serde_json::from_slice(&json)?;

    Ok(user_settings.theme.dark)
}
