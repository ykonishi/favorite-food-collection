use std::fs;
use std::path::Path;

use favorite_food_collection::{collect_list_files, generate_favorite_food, GenerateOptions};
use tempfile::tempdir;

fn write_user_list(base: &Path, user: &str, contents: &str) {
    let user_dir = base.join(user);
    fs::create_dir_all(&user_dir).unwrap();
    fs::write(user_dir.join("list.txt"), contents).unwrap();
}

#[test]
fn collect_list_files_finds_user_lists() {
    let dir = tempdir().unwrap();
    write_user_list(dir.path(), "alice", "ラーメン\n");
    write_user_list(dir.path(), "bob", "寿司\n");

    let files = collect_list_files(dir.path()).unwrap();
    assert_eq!(files.len(), 2);
}

#[test]
fn generate_favorite_food_creates_png() {
    let dir = tempdir().unwrap();
    write_user_list(dir.path(), "alice", "コーヒー\nラーメン\n");
    write_user_list(dir.path(), "bob", "コーヒー\nからあげ\n");

    let output = dir.path().join("wordcloud.png");
    let summary = generate_favorite_food(&GenerateOptions {
        users_dir: dir.path(),
        output_path: &output,
        width: 1024,
        height: 1024,
    })
    .unwrap();

    assert_eq!(summary.file_count, 2);
    assert_eq!(summary.item_count, 4);
    assert_eq!(summary.unique_count, 3);
    assert!(output.is_file());
    assert!(fs::metadata(&output).unwrap().len() > 0);
}

#[test]
fn generate_favorite_food_errors_when_no_lists() {
    let dir = tempdir().unwrap();
    let output = dir.path().join("wordcloud.png");

    let result = generate_favorite_food(&GenerateOptions {
        users_dir: dir.path(),
        output_path: &output,
        width: 1024,
        height: 1024,
    });

    match result {
        Err(err) => assert!(err.to_string().contains("no list.txt found")),
        Ok(_) => panic!("expected error when no list.txt found"),
    }
}
