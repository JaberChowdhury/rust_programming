use rand::prelude::*;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::{env, fs};

fn populate_messy_dir(dir: &Path, count: i32) -> std::io::Result<()> {
    let ext = vec![
        "txt", "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "csv", "json", "xml", "html",
        "css", "js", "ts", "rs", "py", "java", "c", "cpp", "jpg", "jpeg", "png", "gif", "svg",
        "webp", "mp3", "wav", "flac", "mp4", "mkv", "avi", "mov", "zip", "rar", "7z", "tar", "gz",
        "iso",
    ];

    let mut rng = rand::rng();

    for i in 0..count {
        let random_ext = ext.choose(&mut rng).unwrap();

        let file_name = dir.join(format!("dummy_{}.{}", i, random_ext));

        File::create(file_name)?;
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    struct FileCategory {
        name: &'static str,
        extensions: &'static [&'static str],
    }

    let categories = [
        FileCategory {
            name: "documents",
            extensions: &[
                "txt", "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "csv",
            ],
        },
        FileCategory {
            name: "data",
            extensions: &["json", "xml"],
        },
        FileCategory {
            name: "web",
            extensions: &["html", "css", "js", "ts"],
        },
        FileCategory {
            name: "code",
            extensions: &["rs", "py", "java", "c", "cpp"],
        },
        FileCategory {
            name: "images",
            extensions: &["jpg", "jpeg", "png", "gif", "svg", "webp"],
        },
        FileCategory {
            name: "audio",
            extensions: &["mp3", "wav", "flac"],
        },
        FileCategory {
            name: "video",
            extensions: &["mp4", "mkv", "avi", "mov"],
        },
        FileCategory {
            name: "archives",
            extensions: &["zip", "rar", "7z", "tar", "gz", "iso"],
        },
    ];
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <directory> [count]", args[0]);
        return Ok(());
    }

    let dir = PathBuf::from(&args[1]).join("dummy");

    let count: i32 = args.get(2).and_then(|x| x.parse().ok()).unwrap_or(20);

    populate_messy_dir(&dir, count)?;

    let entries = fs::read_dir(dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if let Some(extension) = path.extension() {
            println!("{} -> {}", path.display(), extension.to_string_lossy());
        }
    }

    Ok(())
}
