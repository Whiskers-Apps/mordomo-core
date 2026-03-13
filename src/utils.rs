#![allow(dead_code)]

use std::{
    env,
    error::Error,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
};

#[derive(Debug, Clone)]
pub struct KeywordSplit {
    pub has_keyword: bool,
    pub keyword: String,
    pub text: String,
}

impl KeywordSplit {
    pub fn from<T: AsRef<str>>(text: T) -> Self {
        let mut text = text.as_ref().to_string();
        let mut keyword = String::new();

        let text_blocks: Vec<&str> = text.trim_start().split(" ").collect();
        let text_blocks: Vec<String> = text_blocks.iter().map(|block| block.to_string()).collect();

        let has_keyword = text_blocks.len() > 1;

        if has_keyword {
            text = String::new();

            for (index, block) in text_blocks.into_iter().enumerate() {
                if index == 0 {
                    keyword = block;
                    continue;
                }

                text = format!("{text} {block}");
            }
        }

        Self {
            has_keyword,
            keyword,
            text,
        }
    }
}

pub fn get_icon(name: &str) -> Result<PathBuf, Box<dyn Error>> {
    let mut path = env::current_exe()?;
    path = path.parent().unwrap().to_path_buf();
    path.push(format!("src/icons/{name}"));

    Ok(path)
}

pub fn send_notification<S: AsRef<str>>(title: S, text: S) -> Result<(), Box<dyn Error>> {
    Command::new("notify-send")
        .args([
            "-i",
            "mordomo",
            "-a",
            "Mordomo",
            title.as_ref(),
            text.as_ref(),
        ])
        .spawn()?;

    Ok(())
}

pub fn copy_text<S: AsRef<str>>(text: S) -> Result<(), Box<dyn Error>> {
    let text = text.as_ref().to_owned();

    thread::spawn(move || {
        let text_blocks: Vec<&str> = text.split(" ").collect();

        Command::new("wl-copy")
            .args(text_blocks)
            .spawn()
            .expect("Error copying to clipboard");
    });

    Ok(())
}

pub fn copy_image<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    let path = path.as_ref().to_owned();

    thread::spawn(move || {
        Command::new("cat")
            .arg(format!("'{}'", path.display()))
            .args(["|", "wl-copy", "-t", "image/png"])
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()
            .expect("Error copying image")
    });

    Ok(())
}
