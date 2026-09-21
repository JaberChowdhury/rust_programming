use rand::prelude::*;
use std::fs;
use std::fs::File;
use std::path::*;

pub fn populate_messy_dir(dir: &str, count: i32) -> std::io::Result<()> {
    let ext = vec![
        "txt", "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "csv", "json", "xml", "html",
        "css", "js", "ts", "rs", "py", "java", "c", "cpp", "jpg", "jpeg", "png", "gif", "svg",
        "webp", "mp3", "wav", "flac", "mp4", "mkv", "avi", "mov", "zip", "rar", "7z", "tar", "gz",
        "iso",
    ];

    fs::create_dir_all(Path::new(dir).join("dummy"))?;

    let mut rng = rand::rng();

    for i in 0..count {
        let random_ext = ext.choose(&mut rng).unwrap();

        let file_name = Path::new(dir).join(format!("dummy_{}.{}", i, random_ext));

        File::create(file_name)?;
    }

    Ok(())
}
