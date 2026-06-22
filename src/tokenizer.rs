use std::collections::HashMap;
use std::path::{Path, PathBuf};

use image::RgbaImage;
use wordcloud_rs::{Token, WordCloud};

const FONT_CANDIDATES: &[&str] = &[
    "/System/Library/Fonts/ヒラギノ角ゴシック W3.ttc",
    "/System/Library/Fonts/Hiragino Sans GB.ttc",
    "/Library/Fonts/Arial Unicode.ttf",
    "/System/Library/Fonts/Supplemental/Arial Unicode.ttf",
    "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
    "/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc",
    "C:\\Windows\\Fonts\\msgothic.ttc",
    "C:\\Windows\\Fonts\\YuGothM.ttc",
];

pub fn find_japanese_font() -> Option<PathBuf> {
    FONT_CANDIDATES
        .iter()
        .map(Path::new)
        .find(|path| path.is_file())
        .map(Path::to_path_buf)
}

pub fn aggregate_items(items: impl IntoIterator<Item = String>) -> Vec<(Token, f32)> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for item in items {
        *counts.entry(item).or_default() += 1;
    }

    counts
        .into_iter()
        .map(|(item, count)| (Token::Text(item), count as f32))
        .collect()
}

pub fn generate_wordcloud(
    tokens: Vec<(Token, f32)>,
    width: usize,
    height: usize,
) -> RgbaImage {
    let mut builder = WordCloud::new().dim(width, height);
    if let Some(font_path) = find_japanese_font() {
        if let Some(font_path) = font_path.to_str() {
            builder = builder.font(font_path);
        }
    }
    builder.generate(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aggregate_items_counts_occurrences() {
        let items = vec![
            "コーヒー".to_string(),
            "ラーメン".to_string(),
            "コーヒー".to_string(),
            "からあげ".to_string(),
            "コーヒー".to_string(),
        ];
        let mut tokens = aggregate_items(items);
        tokens.sort_by(|a, b| a.0.to_string().cmp(&b.0.to_string()));

        assert_eq!(tokens.len(), 3);
        assert_eq!(token_weight(&tokens, "コーヒー"), 3.0);
        assert_eq!(token_weight(&tokens, "ラーメン"), 1.0);
        assert_eq!(token_weight(&tokens, "からあげ"), 1.0);
    }

    fn token_weight(tokens: &[(Token, f32)], text: &str) -> f32 {
        tokens
            .iter()
            .find(|(token, _)| token.to_string() == text)
            .map(|(_, weight)| *weight)
            .unwrap_or(0.0)
    }
}
