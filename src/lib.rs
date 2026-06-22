mod collector;
mod tokenizer;

use std::fs;
use std::path::Path;

use anyhow::{bail, Context, Result};
use image::ImageFormat;

pub use collector::{collect_items, collect_list_files, parse_line, read_items_from_file};
pub use tokenizer::{aggregate_items, generate_wordcloud};

pub struct GenerateOptions<'a> {
    pub users_dir: &'a Path,
    pub output_path: &'a Path,
    pub width: usize,
    pub height: usize,
}

pub struct GenerateSummary {
    pub file_count: usize,
    pub item_count: usize,
    pub unique_count: usize,
    pub output_path: String,
}

pub fn validate_dimensions(width: usize, height: usize) -> Result<()> {
    let bits = usize::BITS as usize;
    if width % bits != 0 || height % bits != 0 {
        bail!(
            "width and height must be multiples of {} (got {}x{})",
            bits,
            width,
            height
        );
    }
    Ok(())
}

pub fn generate_favorite_food(options: &GenerateOptions<'_>) -> Result<GenerateSummary> {
    validate_dimensions(options.width, options.height)?;

    let (files, items) = collect_items(options.users_dir)?;
    if items.is_empty() {
        bail!("no items found");
    }

    let item_count = items.len();
    let tokens = aggregate_items(items);
    let unique_count = tokens.len();

    let image = generate_wordcloud(tokens, options.width, options.height);

    if let Some(parent) = options.output_path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create directory: {}", parent.display()))?;
        }
    }

    image
        .save_with_format(options.output_path, ImageFormat::Png)
        .with_context(|| format!("failed to save wordcloud: {}", options.output_path.display()))?;

    Ok(GenerateSummary {
        file_count: files.len(),
        item_count,
        unique_count,
        output_path: options.output_path.display().to_string(),
    })
}
