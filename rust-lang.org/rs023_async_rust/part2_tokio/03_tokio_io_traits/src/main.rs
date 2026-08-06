// CONCEPT: Async file I/O with Tokio.

use tokio::fs::{self, File};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let test_file = "test_io.txt";
    let copy_file = "test_io_copy.txt";

    // Write file
    tokio::fs::write(test_file, b"Hello Async World!").await?;
    println!("Written to {}", test_file);

    // Read file
    let contents = tokio::fs::read_to_string(test_file).await?;
    println!("Read from file: {}", contents);

    // BufWriter
    let mut f = File::create(copy_file).await?;
    let mut writer = BufWriter::new(&mut f);
    writer.write_all(b"Buffered write").await?;
    writer.flush().await?;

    // Copy
    let mut input = File::open(test_file).await?;
    let mut output = File::create("copied.txt").await?;
    tokio::io::copy(&mut input, &mut output).await?;
    println!("File copied");

    // Cleanup
    fs::remove_file(test_file).await?;
    fs::remove_file(copy_file).await?;
    fs::remove_file("copied.txt").await?;

    Ok(())
}
// EXERCISE:
// 1. Use BufReader to read lines.
// 2. Append to a file instead of overwriting.
