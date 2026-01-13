#![allow(dead_code)]

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

        let text_blocks: Vec<&str> = text.split(" ").collect();
        let text_blocks: Vec<String> = text_blocks.iter().map(|block| block.to_string()).collect();

        let has_keyword = text_blocks.len() > 1;

        if has_keyword {
            text = String::new();

            for (index, block) in text_blocks.into_iter().enumerate() {
                if index == 0 {
                    keyword = block;
                    continue;
                }

                text = format!("{text}{block}");
            }
        }

        Self {
            has_keyword,
            keyword,
            text,
        }
    }
}
