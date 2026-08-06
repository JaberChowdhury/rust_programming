use std::path::PathBuf;
use tokio::fs;

async fn read_toml(path: &str) -> std::io::Result<()> {
    let content = fs::read_to_string(path).await?;
    println!("===================== 1 =====================");
    println!("--- {path} ---\n{content}");
    Ok(())
}

async fn count_lock_lines(path: &str) -> std::io::Result<()> {
    let content = fs::read_to_string(path).await?;
    let lines = content.lines().count();
    println!("===================== 2 =====================");
    println!("{path} has {lines} lines");
    Ok(())
}

async fn count_files_in_target(path: &str) -> std::io::Result<()> {
    let mut count = 0;
    let mut stack = vec![PathBuf::from(path)];

    while let Some(dir) = stack.pop() {
        let mut entries = fs::read_dir(&dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let file_type = entry.file_type().await?; // async — may need a syscall
            if file_type.is_dir() {
                stack.push(entry.path());
            } else {
                count += 1;
            }
        }
    }

    println!("===================== 3 =====================");
    println!("{path} contains {count} files total");
    Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let (r1, r2, r3) = tokio::join!(
        read_toml("Cargo.toml"),
        count_lock_lines("Cargo.lock"),
        count_files_in_target("target")
    );

    match r1 {
        Ok(val) => val,          // just discarded here since val is ()
        Err(e) => return Err(e), // bail out of main with this error
    }
    r2?;
    r3?;
    Ok(())
}
