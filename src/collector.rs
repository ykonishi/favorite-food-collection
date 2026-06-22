use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

pub fn collect_list_files(users_dir: &Path) -> Result<Vec<PathBuf>> {
    if !users_dir.is_dir() {
        bail!("users directory not found: {}", users_dir.display());
    }

    let mut files = Vec::new();
    for entry in fs::read_dir(users_dir)
        .with_context(|| format!("failed to read directory: {}", users_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let list_path = path.join("list.txt");
            if list_path.is_file() {
                files.push(list_path);
            }
        }
    }

    files.sort();
    Ok(files)
}

pub fn parse_line(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }
    Some(trimmed.to_string())
}

pub fn read_items_from_file(path: &Path) -> Result<Vec<String>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("failed to read file: {}", path.display()))?;
    Ok(content
        .lines()
        .filter_map(parse_line)
        .collect())
}

pub fn collect_items(users_dir: &Path) -> Result<(Vec<PathBuf>, Vec<String>)> {
    let files = collect_list_files(users_dir)?;
    if files.is_empty() {
        bail!("no list.txt found under {}", users_dir.display());
    }

    let mut items = Vec::new();
    for path in &files {
        items.extend(read_items_from_file(path)?);
    }

    Ok((files, items))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_skips_empty_and_comments() {
        assert_eq!(parse_line("  コーヒー  "), Some("コーヒー".to_string()));
        assert_eq!(parse_line(""), None);
        assert_eq!(parse_line("   "), None);
        assert_eq!(parse_line("# comment"), None);
    }
}
